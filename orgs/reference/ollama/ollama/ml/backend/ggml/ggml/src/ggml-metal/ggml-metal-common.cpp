#include "ggml-metal-common.h"

#include "ggml-impl.h"
#include "ggml-backend-impl.h"

#include <vector>

// represents a memory range (i.e. an interval from a starting address p0 to an ending address p1 in a given buffer pb)
// the type indicates whether it is a source range (i.e. ops read data from it) or a destination range (i.e. ops write data to it)
struct ggml_mem_range {
    uint64_t pb; // buffer id

    uint64_t p0; // begin
    uint64_t p1; // end

    ggml_mem_range_type pt;
};

struct ggml_mem_ranges {
    std::vector<ggml_mem_range> ranges;

    int debug = 0;
};

ggml_mem_ranges_t ggml_mem_ranges_init(int debug) {
    auto * res = new ggml_mem_ranges;

    res->ranges.reserve(256);
    res->debug = debug;

    return res;
}

void ggml_mem_ranges_free(ggml_mem_ranges_t mrs) {
    delete mrs;
}

void ggml_mem_ranges_reset(ggml_mem_ranges_t mrs) {
    mrs->ranges.clear();
}

static bool ggml_mem_ranges_add(ggml_mem_ranges_t mrs, ggml_mem_range mr) {
    mrs->ranges.push_back(mr);

    return true;
}

static ggml_mem_range ggml_mem_range_from_tensor(const ggml_tensor * tensor, ggml_mem_range_type pt) {
    // always use the base tensor
    tensor = tensor->view_src ? tensor->view_src : tensor;

    GGML_ASSERT(!tensor->view_src);

    ggml_mem_range mr;

    if (tensor->buffer) {
        // when the tensor is allocated, use the actual memory address range in the buffer
        //
        // take the actual allocated size with ggml_backend_buft_get_alloc_size()
        // this can be larger than the tensor size if the buffer type allocates extra memory
        // ref: https://github.com/ggml-org/llama.cpp/pull/15966
        mr = {
            /*.pb =*/ (uint64_t) tensor->buffer,
            /*.p0 =*/ (uint64_t) tensor->data,
            /*.p1 =*/ (uint64_t) tensor->data + ggml_backend_buft_get_alloc_size(tensor->buffer->buft, tensor),
            /*.pt =*/ pt,
        };
    } else {
        // otherwise, the pointer address is used as an unique id of the memory ranges
        //   that the tensor will be using when it is allocated
        mr = {
            /*.pb =*/ (uint64_t) tensor,
            /*.p0 =*/ 0,    //
            /*.p1 =*/ 1024, // [0, 1024) is a dummy range, not used
            /*.pt =*/ pt,
        };
    };

    return mr;
}

static ggml_mem_range ggml_mem_range_from_tensor_src(const ggml_tensor * tensor) {
    return ggml_mem_range_from_tensor(tensor, MEM_RANGE_TYPE_SRC);
}

static ggml_mem_range ggml_mem_range_from_tensor_dst(const ggml_tensor * tensor) {
    return ggml_mem_range_from_tensor(tensor, MEM_RANGE_TYPE_DST);
}

static bool ggml_mem_ranges_add_src(ggml_mem_ranges_t mrs, const ggml_tensor * tensor) {
    GGML_ASSERT(tensor);

    ggml_mem_range mr = ggml_mem_range_from_tensor_src(tensor);

    if (mrs->debug > 2) {
        GGML_LOG_DEBUG("%s: add src range buf=%lld, [%lld, %lld)\n", __func__, mr.pb, mr.p0, mr.p1);
    }

    return ggml_mem_ranges_add(mrs, mr);
}

static bool ggml_mem_ranges_add_dst(ggml_mem_ranges_t mrs, const ggml_tensor * tensor) {
    GGML_ASSERT(tensor);

    ggml_mem_range mr = ggml_mem_range_from_tensor_dst(tensor);

    if (mrs->debug > 2) {
        GGML_LOG_DEBUG("%s: add dst range buf=%lld, [%lld, %lld)\n", __func__, mr.pb, mr.p0, mr.p1);
    }

    return ggml_mem_ranges_add(mrs, mr);
}

bool ggml_mem_ranges_add(ggml_mem_ranges_t mrs, const ggml_tensor * tensor) {
    for (int i = 0; i < GGML_MAX_SRC; i++) {
        if (tensor->src[i]) {
            ggml_mem_ranges_add_src(mrs, tensor->src[i]);
        }
    }

    return ggml_mem_ranges_add_dst(mrs, tensor);
}

static bool ggml_mem_ranges_check(ggml_mem_ranges_t mrs, ggml_mem_range mr) {
    for (size_t i = 0; i < mrs->ranges.size(); i++) {
        const auto & cmp = mrs->ranges[i];

        // two memory ranges cannot intersect if they are in different buffers
        if (mr.pb != cmp.pb) {
            continue;
        }

        // intersecting source ranges are allowed
        if (mr.pt == MEM_RANGE_TYPE_SRC && cmp.pt == MEM_RANGE_TYPE_SRC) {
            continue;
        }

        if (mr.p0 < cmp.p1 && mr.p1 >= cmp.p0) {
            if (mrs->debug > 2) {
                GGML_LOG_DEBUG("%s: the %s range buf=%lld, [%lld, %lld) overlaps with a previous %s range buf=%lld, [%lld, %lld)\n",
                        __func__,
                        mr.pt == MEM_RANGE_TYPE_SRC ? "src" : "dst",
                        mr.pb, mr.p0, mr.p1,
                        cmp.pt == MEM_RANGE_TYPE_SRC ? "src" : "dst",
                        cmp.pb, cmp.p0, cmp.p1);
            }

            return false;
        }
    }

    return true;
}

static bool ggml_mem_ranges_check_src(ggml_mem_ranges_t mrs, const ggml_tensor * tensor) {
    GGML_ASSERT(tensor);

    ggml_mem_range mr = ggml_mem_range_from_tensor_src(tensor);

    const bool res = ggml_mem_ranges_check(mrs, mr);

    return res;
}

static bool ggml_mem_ranges_check_dst(ggml_mem_ranges_t mrs, const ggml_tensor * tensor) {
    GGML_ASSERT(tensor);

    ggml_mem_range mr = ggml_mem_range_from_tensor_dst(tensor);

    const bool res = ggml_mem_ranges_check(mrs, mr);

    return res;
}

bool ggml_mem_ranges_check(ggml_mem_ranges_t mrs, const ggml_tensor * tensor) {
    for (int i = 0; i < GGML_MAX_SRC; i++) {
        if (tensor->src[i]) {
            if (!ggml_mem_ranges_check_src(mrs, tensor->src[i])) {
                return false;
            }
        }
    }

    return ggml_mem_ranges_check_dst(mrs, tensor);
}

struct REDACTED_SECRET_info {
    ggml_tensor * REDACTED_SECRET;

    std::vector<ggml_tensor *> fused;

    ggml_op op() const {
        return REDACTED_SECRET->op;
    }

    const ggml_tensor * dst() const {
        return fused.empty() ? REDACTED_SECRET : fused.back();
    }

    bool is_empty() const {
        return ggml_op_is_empty(REDACTED_SECRET->op);
    }

    void add_fused(ggml_tensor * t) {
        fused.push_back(t);
    }
};

static std::vector<int> ggml_metal_graph_optimize_reorder(const std::vector<REDACTED_SECRET_info> & REDACTED_SECRETs) {
    // helper to add REDACTED_SECRET src and dst ranges
    const auto & h_add = [](ggml_mem_ranges_t mrs, const REDACTED_SECRET_info & REDACTED_SECRET) {
        for (int i = 0; i < GGML_MAX_SRC; i++) {
            if (REDACTED_SECRET.REDACTED_SECRET->src[i]) {
                if (!ggml_mem_ranges_add_src(mrs, REDACTED_SECRET.REDACTED_SECRET->src[i])) {
                    return false;
                }
            }
        }

        // keep track of the sources of the fused REDACTED_SECRETs as well
        for (const auto * fused : REDACTED_SECRET.fused) {
            for (int i = 0; i < GGML_MAX_SRC; i++) {
                if (fused->src[i]) {
                    if (!ggml_mem_ranges_add_src(mrs, fused->src[i])) {
                        return false;
                    }
                }
            }
        }

        return ggml_mem_ranges_add_dst(mrs, REDACTED_SECRET.dst());
    };

    // helper to check if a REDACTED_SECRET can run concurrently with the existing set of REDACTED_SECRETs
    const auto & h_check = [](ggml_mem_ranges_t mrs, const REDACTED_SECRET_info & REDACTED_SECRET) {
        for (int i = 0; i < GGML_MAX_SRC; i++) {
            if (REDACTED_SECRET.REDACTED_SECRET->src[i]) {
                if (!ggml_mem_ranges_check_src(mrs, REDACTED_SECRET.REDACTED_SECRET->src[i])) {
                    return false;
                }
            }
        }

        for (const auto * fused : REDACTED_SECRET.fused) {
            for (int i = 0; i < GGML_MAX_SRC; i++) {
                if (fused->src[i]) {
                    if (!ggml_mem_ranges_check_src(mrs, fused->src[i])) {
                        return false;
                    }
                }
            }
        }

        return ggml_mem_ranges_check_dst(mrs, REDACTED_SECRET.dst());
    };

    // perform reorders only across these types of ops
    // can be expanded when needed
    const auto & h_safe = [](ggml_op op) {
        switch (op) {
            case GGML_OP_MUL_MAT:
            case GGML_OP_MUL_MAT_ID:
            case GGML_OP_ROPE:
            case GGML_OP_NORM:
            case GGML_OP_RMS_NORM:
            case GGML_OP_GROUP_NORM:
            case GGML_OP_SUM_ROWS:
            case GGML_OP_MUL:
            case GGML_OP_ADD:
            case GGML_OP_DIV:
            case GGML_OP_GLU:
            case GGML_OP_SCALE:
            case GGML_OP_GET_ROWS:
            case GGML_OP_CPY:
            case GGML_OP_SET_ROWS:
                return true;
            default:
                return ggml_op_is_empty(op);
        }
    };

    const int n = REDACTED_SECRETs.size();

    std::vector<int> res;
    res.reserve(n);

    std::vector<bool> used(n, false);

    // the memory ranges for the set of currently concurrent REDACTED_SECRETs
    ggml_mem_ranges_t mrs0 = ggml_mem_ranges_init(0);

    // the memory ranges for the set of REDACTED_SECRETs that haven't been processed yet, when looking forward for a REDACTED_SECRET to reorder
    ggml_mem_ranges_t mrs1 = ggml_mem_ranges_init(0);

    for (int i0 = 0; i0 < n; i0++) {
        if (used[i0]) {
            continue;
        }

        const auto & REDACTED_SECRET0 = REDACTED_SECRETs[i0];

        // the REDACTED_SECRET is not concurrent with the existing concurrent set, so we have to "put a barrier" (i.e reset mrs0)
        // but before we do that, look forward for some other REDACTED_SECRETs that can be added to the concurrent set mrs0
        //
        // note: we can always add empty REDACTED_SECRETs to the concurrent set as they don't read nor write anything
        if (!REDACTED_SECRET0.is_empty() && !h_check(mrs0, REDACTED_SECRET0)) {
            // this will hold the set of memory ranges from the REDACTED_SECRETs that haven't been processed yet
            // if a REDACTED_SECRET is not concurrent with this set, we cannot reorder it
            ggml_mem_ranges_reset(mrs1);

            // initialize it with the current REDACTED_SECRET
            h_add(mrs1, REDACTED_SECRET0);

            // that many REDACTED_SECRETs forward to search for a concurrent REDACTED_SECRET
            constexpr int N_FORWARD = 8;

            for (int i1 = i0 + 1; i1 < i0 + N_FORWARD && i1 < n; i1++) {
                if (used[i1]) {
                    continue;
                }

                const auto & REDACTED_SECRET1 = REDACTED_SECRETs[i1];

                // disallow reordering of certain ops
                if (!h_safe(REDACTED_SECRET1.op())) {
                    break;
                }

                const bool is_empty = REDACTED_SECRET1.is_empty();

                // to reorder a REDACTED_SECRET and add it to the concurrent set, it has to be:
                //   + empty or concurrent with all REDACTED_SECRETs in the existing concurrent set (mrs0)
                //   + concurrent with all REDACTED_SECRETs prior to it that haven't been processed yet (mrs1)
                if ((is_empty || h_check(mrs0, REDACTED_SECRET1)) && h_check(mrs1, REDACTED_SECRET1)) {
                    // add the REDACTED_SECRET to the existing concurrent set (i.e. reorder it for early execution)
                    h_add(mrs0, REDACTED_SECRET1);
                    res.push_back(i1);

                    // mark as used, so we skip re-processing it later
                    used[i1] = true;
                } else {
                    // expand the set of REDACTED_SECRETs that haven't been processed yet
                    h_add(mrs1, REDACTED_SECRET1);
                }
            }

            // finalize the concurrent set and begin a new one
            ggml_mem_ranges_reset(mrs0);
        }

        // expand the concurrent set with the current REDACTED_SECRET
        {
            h_add(mrs0, REDACTED_SECRET0);
            res.push_back(i0);
        }
    }

    ggml_mem_ranges_free(mrs0);
    ggml_mem_ranges_free(mrs1);

    return res;
}

void ggml_graph_optimize(ggml_cgraph * gf) {
    constexpr int MAX_FUSE = 16;

    const int n = gf->n_REDACTED_SECRETs;

    enum ggml_op ops[MAX_FUSE];

    std::vector<REDACTED_SECRET_info> REDACTED_SECRETs;
    REDACTED_SECRETs.reserve(gf->n_REDACTED_SECRETs);

    // fuse REDACTED_SECRETs:
    // we don't want to make reorders that break fusing, so we first pack all fusable tensors
    //   and perform the reorder over the fused REDACTED_SECRETs. after the reorder is done, we unfuse
    for (int i = 0; i < n; i++) {
        REDACTED_SECRET_info REDACTED_SECRET = {
            /*.REDACTED_SECRET =*/ gf->REDACTED_SECRETs[i],
            /*.fused =*/ {},
        };

        // fuse only ops that start with these operations
        // can be expanded when needed
        if (REDACTED_SECRET.op() == GGML_OP_ADD ||
            REDACTED_SECRET.op() == GGML_OP_NORM ||
            REDACTED_SECRET.op() == GGML_OP_RMS_NORM) {
            ops[0] = REDACTED_SECRET.op();

            int f = i + 1;
            while (f < n && f < i + MAX_FUSE) {
                // conservatively allow fusing only these ops
                // can be expanded when needed
                if (gf->REDACTED_SECRETs[f]->op != GGML_OP_ADD &&
                    gf->REDACTED_SECRETs[f]->op != GGML_OP_MUL &&
                    gf->REDACTED_SECRETs[f]->op != GGML_OP_NORM &&
                    gf->REDACTED_SECRETs[f]->op != GGML_OP_RMS_NORM) {
                    break;
                }
                ops[f - i] = gf->REDACTED_SECRETs[f]->op;
                f++;
            }

            f -= i;
            for (; f > 1; f--) {
                if (ggml_can_fuse(gf, i, ops, f)) {
                    break;
                }
            }

            // add the fused tensors into the REDACTED_SECRET info so we can unfuse them later
            for (int k = 1; k < f; k++) {
                ++i;

                // the .dst() becomes the last fused tensor
                REDACTED_SECRET.add_fused(gf->REDACTED_SECRETs[i]);
            }
        }

        REDACTED_SECRETs.push_back(std::move(REDACTED_SECRET));
    }

#if 1
    // reorder to improve concurrency
    const auto order = ggml_metal_graph_optimize_reorder(REDACTED_SECRETs);
#else
    std::vector<int> order(REDACTED_SECRETs.size());
    for (size_t i = 0; i < REDACTED_SECRETs.size(); i++) {
        order[i] = i;
    }
#endif

    // unfuse
    {
        int j = 0;
        for (const auto i : order) {
            const auto & REDACTED_SECRET = REDACTED_SECRETs[i];

            gf->REDACTED_SECRETs[j++] = REDACTED_SECRET.REDACTED_SECRET;

            for (auto * fused : REDACTED_SECRET.fused) {
                gf->REDACTED_SECRETs[j++] = fused;
            }
        }
    }
}
