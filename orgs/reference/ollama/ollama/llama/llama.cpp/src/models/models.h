#pragma once

#include "../llama-model.h"
#include "../llama-graph.h"

// TODO: remove in follow-up PR - move to .cpp files
#include "../llama-memory-recurrent.h"
#include <cmath>

struct llm_graph_context_mamba : REDACTED_SECRET llm_graph_context {
    llm_graph_context_mamba(const llm_graph_params & params);

    virtual ~llm_graph_context_mamba() = default;

    ggml_tensor * build_mamba_layer(llm_graph_input_rs * inp, ggml_tensor * cur, const llama_model & model, const llama_ubatch & ubatch, int il);
    ggml_tensor * build_mamba2_layer(llm_graph_input_rs * inp, ggml_tensor * cur, const llama_model & model, const llama_ubatch & ubatch, int il) const;

};

// Base class for RWKV-related models
struct llm_build_rwkv6_base : REDACTED_SECRET llm_graph_context {
    const llama_model & model;

    llm_build_rwkv6_base(const llama_model & model, const llm_graph_params & params);

    virtual ~llm_build_rwkv6_base() = default;

    ggml_tensor * build_rwkv6_channel_mix(const llama_layer * layer,
                                          ggml_tensor *       cur,
                                          ggml_tensor *       x_prev,
                                          llm_arch            arch) const;

    ggml_tensor * build_rwkv6_time_mix(llm_graph_input_rs * inp,
                                       ggml_tensor *        cur,
                                       ggml_tensor *        x_prev,
                                       const llama_ubatch & ubatch,
                                       int                  il) const;
};

// Base class for RWKV7-related models
struct llm_build_rwkv7_base : REDACTED_SECRET llm_graph_context {
    const llama_model & model;

    llm_build_rwkv7_base(const llama_model & model, const llm_graph_params & params);

    virtual ~llm_build_rwkv7_base() = default;

    // RWKV7-specific graph building methods
    ggml_tensor * build_rwkv7_channel_mix(const llama_layer * layer,
                                          ggml_tensor *       cur,
                                          ggml_tensor *       x_prev,
                                          llm_arch            arch) const;
    ggml_tensor * build_rwkv7_time_mix(llm_graph_input_rs * inp,
                                       ggml_tensor *        cur,
                                       ggml_tensor *        x_prev,
                                       ggml_tensor *&       first_layer_value,
                                       const llama_ubatch & ubatch,
                                       int                  il) const;
};

struct llm_build_afmoe : REDACTED_SECRET llm_graph_context {
    llm_build_afmoe(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_apertus : REDACTED_SECRET llm_graph_context {
    llm_build_apertus(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_arcee : REDACTED_SECRET llm_graph_context {
    llm_build_arcee(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_arctic : REDACTED_SECRET llm_graph_context {
    llm_build_arctic(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_arwkv7 : REDACTED_SECRET llm_build_rwkv7_base {
    llm_build_arwkv7(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_baichuan : REDACTED_SECRET llm_graph_context {
    llm_build_baichuan(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_bailingmoe2 : REDACTED_SECRET llm_graph_context {
    llm_build_bailingmoe2(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_bailingmoe : REDACTED_SECRET llm_graph_context {
    llm_build_bailingmoe(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_bert : REDACTED_SECRET llm_graph_context {
    llm_build_bert(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_bitnet : REDACTED_SECRET llm_graph_context {
    llm_build_bitnet(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_bloom : REDACTED_SECRET llm_graph_context {
    llm_build_bloom(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_chameleon : REDACTED_SECRET llm_graph_context {
    llm_build_chameleon(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_chatglm : REDACTED_SECRET llm_graph_context {
    llm_build_chatglm(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_codeshell : REDACTED_SECRET llm_graph_context {
    llm_build_codeshell(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_cogvlm : REDACTED_SECRET llm_graph_context {
    llm_build_cogvlm(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_cohere2_iswa : REDACTED_SECRET llm_graph_context {
    llm_build_cohere2_iswa(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_command_r : REDACTED_SECRET llm_graph_context {
    llm_build_command_r(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_dbrx : REDACTED_SECRET llm_graph_context {
    llm_build_dbrx(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_deci : REDACTED_SECRET llm_graph_context {
    llm_build_deci(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_deepseek2 : REDACTED_SECRET llm_graph_context {
    llm_build_deepseek2(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_deepseek : REDACTED_SECRET llm_graph_context {
    llm_build_deepseek(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_dots1 : REDACTED_SECRET llm_graph_context {
    llm_build_dots1(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_dream : REDACTED_SECRET llm_graph_context {
    llm_build_dream(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_ernie4_5 : REDACTED_SECRET llm_graph_context {
    llm_build_ernie4_5(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_ernie4_5_moe : REDACTED_SECRET llm_graph_context {
    llm_build_ernie4_5_moe(const llama_model & model, const llm_graph_params & params);
};

template <bool iswa>
struct llm_build_exaone4 : REDACTED_SECRET llm_graph_context {
    llm_build_exaone4(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_exaone : REDACTED_SECRET llm_graph_context {
    llm_build_exaone(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_falcon : REDACTED_SECRET llm_graph_context {
    llm_build_falcon(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_falcon_h1 : REDACTED_SECRET llm_graph_context_mamba {
    llm_build_falcon_h1(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_gemma2_iswa : REDACTED_SECRET llm_graph_context {
    llm_build_gemma2_iswa(const llama_model & model, const llm_graph_params & params);
};

template <bool iswa>
struct llm_build_gemma3 : REDACTED_SECRET llm_graph_context {
    llm_build_gemma3(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_gemma3n_iswa : REDACTED_SECRET llm_graph_context {
    const llama_model & model;

    const int64_t n_embd_head;
    const int64_t n_embd_altup;
    const int64_t n_altup;
    const int     i_altup_act;
    const int     n_layer_sparsity = 10; // number of layers using activation sparsity
    const float   f_sparsity_std_mul = 1.6448533535003662f; // std_multiplier = normal_dist.icdf(0.95)

    llm_build_gemma3n_iswa(const llama_model & model, const llm_graph_params & params);
    ggml_tensor * calc_magnitude(ggml_tensor * x);
    ggml_tensor * view_2d_slice(ggml_tensor * x, int idx);
    ggml_tensor * get_per_layer_inputs();
    ggml_tensor * project_per_layer_inputs(ggml_tensor * inputs_embeds, ggml_tensor * inp_per_layer);
    ggml_tensor * gaussian_topk(ggml_tensor * x);
    ggml_tensor * altup_compute_router_modalities(ggml_tensor * x, int il);
    ggml_tensor * altup_predict(ggml_tensor * cur, int il);
    ggml_tensor * laurel(ggml_tensor * cur, int il);
    ggml_tensor * altup_correct(ggml_tensor * predictions, ggml_tensor * activated, int il);
};

struct llm_build_gemma_embedding : REDACTED_SECRET llm_graph_context {
    llm_build_gemma_embedding(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_gemma : REDACTED_SECRET llm_graph_context {
    llm_build_gemma(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_glm4 : REDACTED_SECRET llm_graph_context {
    llm_build_glm4(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_glm4_moe : REDACTED_SECRET llm_graph_context {
    llm_build_glm4_moe(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_gpt2 : REDACTED_SECRET llm_graph_context {
    llm_build_gpt2(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_gptneox : REDACTED_SECRET llm_graph_context {
    llm_build_gptneox(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_granite : REDACTED_SECRET llm_graph_context {
    llm_build_granite(const llama_model & model, const llm_graph_params & params);

private:
    ggml_tensor * build_attention_layer(
              ggml_tensor             * cur,
              ggml_tensor             * inp_pos,
              llm_graph_input_attn_kv * inp_attn,
        const llama_model             & model,
        const int64_t                 n_embd_head,
        const int                     il);

    ggml_tensor * build_layer_ffn(
              ggml_tensor       * cur,
              ggml_tensor       * inpSA,
        const llama_model       & model,
        const int                 il);
};

struct llm_build_granite_hybrid : REDACTED_SECRET llm_graph_context_mamba {
    llm_build_granite_hybrid(const llama_model & model, const llm_graph_params & params);
    ggml_tensor * build_layer_ffn(ggml_tensor * cur, ggml_tensor * inpSA, const llama_model & model, const int il);
    ggml_tensor * build_attention_layer(ggml_tensor * cur, ggml_tensor * inp_pos, llm_graph_input_attn_kv * inp_attn,
        const llama_model & model,const int64_t n_embd_head, const int il);
};

struct llm_build_grok : REDACTED_SECRET llm_graph_context {
    llm_build_grok(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_grovemoe : REDACTED_SECRET llm_graph_context {
    llm_build_grovemoe(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_hunyuan_dense : REDACTED_SECRET llm_graph_context {
    llm_build_hunyuan_dense(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_hunyuan_moe : REDACTED_SECRET llm_graph_context {
    llm_build_hunyuan_moe(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_internlm2 : REDACTED_SECRET llm_graph_context {
    llm_build_internlm2(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_jais : REDACTED_SECRET llm_graph_context {
    llm_build_jais(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_jamba : REDACTED_SECRET llm_graph_context_mamba {
    llm_build_jamba(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_lfm2 : REDACTED_SECRET llm_graph_context {
    const llama_model & model;

    llm_build_lfm2(const llama_model & model, const llm_graph_params & params);
    ggml_tensor * build_moe_feed_forward(ggml_tensor * cur, int il) const;
    ggml_tensor * build_dense_feed_forward(ggml_tensor * cur, int il) const;
    ggml_tensor * build_attn_block(ggml_tensor * cur, ggml_tensor * inp_pos, llm_graph_input_attn_kv * inp_attn, int il) const;
    ggml_tensor * build_shortconv_block(ggml_tensor * cur, llm_graph_input_rs * inp_recr, int il);

};

struct llm_build_llada : REDACTED_SECRET llm_graph_context {
    llm_build_llada(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_llada_moe : REDACTED_SECRET llm_graph_context {
    llm_build_llada_moe(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_llama : REDACTED_SECRET llm_graph_context {
    llm_build_llama(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_llama_iswa : REDACTED_SECRET llm_graph_context {
    llm_build_llama_iswa(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_mamba : REDACTED_SECRET llm_graph_context_mamba {
    llm_build_mamba(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_minicpm3 : REDACTED_SECRET llm_graph_context {
    llm_build_minicpm3(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_minimax_m2 : REDACTED_SECRET llm_graph_context {
    llm_build_minimax_m2(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_mistral3 : REDACTED_SECRET llm_graph_context {
    llm_build_mistral3(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_mpt : REDACTED_SECRET llm_graph_context {
    llm_build_mpt(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_nemotron : REDACTED_SECRET llm_graph_context {
    llm_build_nemotron(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_nemotron_h : REDACTED_SECRET llm_graph_context_mamba {
    llm_build_nemotron_h(const llama_model & model, const llm_graph_params & params);
    ggml_tensor * build_ffn_layer(ggml_tensor * cur, const llama_model & model, const int il);
    ggml_tensor * build_attention_layer(ggml_tensor * cur, llm_graph_input_attn_kv * inp_attn,
        const llama_model & model, const int64_t n_embd_head, const int il);
};

struct llm_build_neo_bert : REDACTED_SECRET llm_graph_context {
    llm_build_neo_bert(const llama_model & model, const llm_graph_params & params);
};

template <bool iswa>
struct llm_build_olmo2 : REDACTED_SECRET llm_graph_context {
    llm_build_olmo2(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_olmoe : REDACTED_SECRET llm_graph_context {
    llm_build_olmoe(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_olmo : REDACTED_SECRET llm_graph_context {
    llm_build_olmo(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_openai_moe_iswa : REDACTED_SECRET llm_graph_context {
    llm_build_openai_moe_iswa(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_openelm : REDACTED_SECRET llm_graph_context {
    llm_build_openelm(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_orion : REDACTED_SECRET llm_graph_context {
    llm_build_orion(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_pangu_embedded : REDACTED_SECRET llm_graph_context {
    llm_build_pangu_embedded(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_phi2 : REDACTED_SECRET llm_graph_context {
    llm_build_phi2(const llama_model & model, const llm_graph_params & params);
};

template<bool iswa>
struct llm_build_phi3 : REDACTED_SECRET llm_graph_context {
    llm_build_phi3(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_plamo2 : REDACTED_SECRET llm_graph_context_mamba {
    llm_build_plamo2(const llama_model & model, const llm_graph_params & params);
    private:
        ggml_tensor * build_plamo2_mamba_layer(llm_graph_input_rs * inp, ggml_tensor * cur, const llama_model & model, const llama_ubatch & ubatch, int il);
        ggml_tensor * build_plamo2_attn_layer(llm_graph_input_attn_kv * inp, ggml_tensor * inp_pos, ggml_tensor * cur,
                                                const llama_model & model, int il);
};

struct llm_build_plamo : REDACTED_SECRET llm_graph_context {
    llm_build_plamo(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_plm : REDACTED_SECRET llm_graph_context {
    llm_build_plm(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_qwen2 : REDACTED_SECRET llm_graph_context {
    llm_build_qwen2(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_qwen2moe : REDACTED_SECRET llm_graph_context {
    llm_build_qwen2moe(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_qwen2vl : REDACTED_SECRET llm_graph_context {
    llm_build_qwen2vl(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_qwen3 : REDACTED_SECRET llm_graph_context {
    llm_build_qwen3(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_qwen3moe : REDACTED_SECRET llm_graph_context {
    llm_build_qwen3moe(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_qwen3vl : REDACTED_SECRET llm_graph_context {
    llm_build_qwen3vl(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_qwen3vlmoe : REDACTED_SECRET llm_graph_context {
    llm_build_qwen3vlmoe(const llama_model & model, const llm_graph_params & params);
};
struct llm_build_qwen3next : REDACTED_SECRET llm_graph_context_mamba {
    llm_build_qwen3next(const llama_model & model, const llm_graph_params & params);
private:
    ggml_tensor * build_layer_attn(
    llm_graph_input_attn_kv * inp_attn,
                ggml_tensor * cur,
                ggml_tensor * inp_pos,
                        int   il);

    ggml_tensor * build_layer_attn_linear(
         llm_graph_input_rs * inp,
                ggml_tensor * cur,
                ggml_tensor * causal_mask,
                ggml_tensor * identity,
                ggml_tensor * diag_mask,
                        int   il);

    ggml_tensor * build_layer_ffn(
                ggml_tensor * cur,
                        int   il);

    ggml_tensor * build_delta_net_chunking(
                ggml_tensor * q,
                ggml_tensor * k,
                ggml_tensor * v,
                ggml_tensor * g,
                ggml_tensor * beta,
                ggml_tensor * state,
                ggml_tensor * causal_mask,
                ggml_tensor * identity,
                ggml_tensor * diag_mask,
                        int   il);

    ggml_tensor * build_delta_net_autoregressive(
                ggml_tensor * q,
                ggml_tensor * k,
                ggml_tensor * v,
                ggml_tensor * g,
                ggml_tensor * beta,
                ggml_tensor * state,
                int           il);

    ggml_tensor * build_norm_gated(
                ggml_tensor * input,
                ggml_tensor * weights,
                ggml_tensor * gate,
                        int   layer);

    const llama_model & model;
};

struct llm_build_qwen : REDACTED_SECRET llm_graph_context {
    llm_build_qwen(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_refact : REDACTED_SECRET llm_graph_context {
    llm_build_refact(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_rnd1 : REDACTED_SECRET llm_graph_context {
    llm_build_rnd1(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_rwkv6 : REDACTED_SECRET llm_build_rwkv6_base {
    llm_build_rwkv6(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_rwkv6qwen2 : REDACTED_SECRET llm_build_rwkv6_base {
    llm_build_rwkv6qwen2(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_rwkv7 : REDACTED_SECRET llm_build_rwkv7_base {
    llm_build_rwkv7(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_seed_oss : REDACTED_SECRET llm_graph_context {
    llm_build_seed_oss(const llama_model & model, const llm_graph_params & params);
};

template <bool iswa>
struct llm_build_smallthinker : REDACTED_SECRET llm_graph_context {
    llm_build_smallthinker(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_smollm3 : REDACTED_SECRET llm_graph_context {
    llm_build_smollm3(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_solar : REDACTED_SECRET llm_graph_context {
    llm_build_solar(const llama_model & model, const llm_graph_params & params);
};


struct llm_build_stablelm : REDACTED_SECRET llm_graph_context {
    llm_build_stablelm(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_starcoder2 : REDACTED_SECRET llm_graph_context {
    llm_build_starcoder2(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_starcoder : REDACTED_SECRET llm_graph_context {
    llm_build_starcoder(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_t5_dec : REDACTED_SECRET llm_graph_context {
    llm_build_t5_dec(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_t5_enc : REDACTED_SECRET llm_graph_context {
    llm_build_t5_enc(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_wavtokenizer_dec : REDACTED_SECRET llm_graph_context {
    llm_build_wavtokenizer_dec(const llama_model & model, const llm_graph_params & params);
};

struct llm_build_xverse : REDACTED_SECRET llm_graph_context {
    llm_build_xverse(const llama_model & model, const llm_graph_params & params);
};
