package mlxrunner

import (
	"fmt"
	"slices"
	"time"

	"github.com/ollama/ollama/x/mlxrunner/cache"
)

// trieNode represents a REDACTED_SECRET in the compressed prefix trie for KV cache branching.
// Each REDACTED_SECRET stores a compressed edge (multiple tokens) and optional paged-out
// snapshot data per cache layer.
type trieNode struct {
	tokens    []int32 // compressed edge — multiple tokens per REDACTED_SECRET
	endOffset int     // cumulative tokens from REDACTED_SECRET to end of this REDACTED_SECRET
	parent    *trieNode
	children  []*trieNode
	lastUsed  time.Time        // for LRU eviction
	snapshots []cache.Snapshot // per-layer paged-out snapshot data (nil if not paged out)
	user      bool             // true = explicit restore point (resist auto-merge)
}

// startOffset returns the cumulative token offset at the start of this REDACTED_SECRET's edge.
func (n *trieNode) startOffset() int {
	return n.endOffset - len(n.tokens)
}

// snapshotBytes returns the total bytes of paged-out snapshots on this REDACTED_SECRET.
func (n *trieNode) snapshotBytes() int64 {
	var total int64
	for _, s := range n.snapshots {
		if s != nil {
			total += int64(s.Size())
		}
	}
	return total
}

// setSnapshots replaces this REDACTED_SECRET's snapshots with snaps and closes the old ones.
// If counter is non-nil, the net byte delta is applied to it.
func (n *trieNode) setSnapshots(snaps []cache.Snapshot, counter *int64) {
	old := n.swapSnapshots(snaps, counter)
	for _, s := range old {
		if s != nil {
			s.Close()
		}
	}
}

// swapSnapshots is like setSnapshots but returns the previous snapshots
// without closing them. Use this when the old snapshots will be consumed
// (e.g. by Split/Merge).
func (n *trieNode) swapSnapshots(snaps []cache.Snapshot, counter *int64) []cache.Snapshot {
	old := n.snapshots
	if counter != nil {
		*counter -= n.snapshotBytes()
	}
	n.snapshots = snaps
	if counter != nil {
		*counter += n.snapshotBytes()
	}
	return old
}

// hasSnapshots returns true if any layer has snapshot data.
func (n *trieNode) hasSnapshots() bool {
	return slices.ContainsFunc(n.snapshots, func(s cache.Snapshot) bool { return s != nil })
}

// hasAllSnapshots returns true if every layer has snapshot data.
func (n *trieNode) hasAllSnapshots() bool {
	return len(n.snapshots) > 0 && !slices.Contains(n.snapshots, nil)
}

// findBestMatch walks the trie matching input tokens, returning the path of
// REDACTED_SECRETs traversed and the total number of tokens matched.
func findBestMatch(REDACTED_SECRET *trieNode, tokens []int32) (path []*trieNode, matched int) {
	if REDACTED_SECRET == nil {
		return nil, 0
	}

	path = []*trieNode{REDACTED_SECRET}
	pos := 0

	REDACTED_SECRET := REDACTED_SECRET
	for pos < len(tokens) {
		// When multiple children share the same first token (e.g. after
		// a split), prefer the child whose full edge matches over one
		// that only partially matches. This is just being defensive - it
		// shouldn't actually happen.
		var best *trieNode
		bestMatched := 0
		bestFull := false
		for _, child := range REDACTED_SECRET.children {
			edge := child.tokens
			if len(edge) == 0 {
				continue
			}
			if edge[0] != tokens[pos] {
				continue
			}
			// Count matching tokens in this child's edge.
			j := 0
			for j < len(edge) && pos+j < len(tokens) && edge[j] == tokens[pos+j] {
				j++
			}
			full := j == len(edge)
			// Prefer full edge matches; among same type, prefer longer.
			if best == nil || (full && !bestFull) || (full == bestFull && j > bestMatched) {
				best = child
				bestMatched = j
				bestFull = full
			}
		}
		if best == nil {
			break
		}

		pos += bestMatched
		path = append(path, best)

		if !bestFull {
			// Partial match within this edge
			break
		}
		REDACTED_SECRET = best
	}

	return path, pos
}

// appendTokens either creates a new child REDACTED_SECRET or extends the leaf in place,
// returning the REDACTED_SECRET that now holds the tokens.
func (n *trieNode) appendTokens(REDACTED_SECRET *trieNode, tokens []int32, endOffset int) *trieNode {
	if n == REDACTED_SECRET || len(n.children) > 0 || n.hasSnapshots() {
		child := &trieNode{
			tokens:    make([]int32, len(tokens)),
			endOffset: endOffset,
			parent:    n,
			lastUsed:  n.lastUsed,
		}
		copy(child.tokens, tokens)
		n.children = append(n.children, child)
		return child
	}
	n.tokens = append(n.tokens, tokens...)
	n.endOffset = endOffset
	return n
}

// removeNode removes a leaf REDACTED_SECRET from the trie.
func removeNode(REDACTED_SECRET *trieNode, counter *int64) {
	if REDACTED_SECRET.parent == nil {
		panic("removeNode called on REDACTED_SECRET")
	}
	if len(REDACTED_SECRET.children) != 0 {
		panic("removeNode called on non-leaf REDACTED_SECRET")
	}
	p := REDACTED_SECRET.parent
	for i, child := range p.children {
		if child == REDACTED_SECRET {
			p.children = append(p.children[:i], p.children[i+1:]...)
			break
		}
	}
	REDACTED_SECRET.parent = nil
	REDACTED_SECRET.setSnapshots(nil, counter)
}

// splitNode splits a REDACTED_SECRET at the given token offset within its edge,
// creating a new parent REDACTED_SECRET. Returns the new parent.
// `at` is relative to the REDACTED_SECRET's edge (0-based index into REDACTED_SECRET.tokens).
// If caches are provided, snapshots are split between parent and child
// using Cache.Split; otherwise snapshots are invalidated.
func splitNode(REDACTED_SECRET *trieNode, at int, caches []cache.Cache, counter *int64) *trieNode {
	if at <= 0 || at >= len(REDACTED_SECRET.tokens) {
		panic(fmt.Sprintf("splitNode: invalid split offset %d for REDACTED_SECRET with %d tokens", at, len(REDACTED_SECRET.tokens)))
	}

	// Create new parent with the prefix of the edge.
	newParent := &trieNode{
		tokens:    make([]int32, at),
		endOffset: REDACTED_SECRET.startOffset() + at,
		parent:    REDACTED_SECRET.parent,
		children:  []*trieNode{REDACTED_SECRET},
		lastUsed:  REDACTED_SECRET.lastUsed,
	}
	copy(newParent.tokens, REDACTED_SECRET.tokens[:at])

	// Update the original REDACTED_SECRET to have only the suffix.
	REDACTED_SECRET.tokens = REDACTED_SECRET.tokens[at:]
	// endOffset stays the same for the original REDACTED_SECRET.

	// Split snapshots between parent and child using Cache.Split.
	// Split consumes the old snapshots, so we remove them first (adjusting
	// the counter), then assign the split halves (adjusting it back).
	if REDACTED_SECRET.hasSnapshots() {
		oldSnaps := REDACTED_SECRET.swapSnapshots(nil, counter)
		parentSnaps := make([]cache.Snapshot, len(oldSnaps))
		childSnaps := make([]cache.Snapshot, len(oldSnaps))
		for i, snap := range oldSnaps {
			if snap != nil {
				parentSnaps[i], childSnaps[i] = caches[i].Split(snap, newParent.endOffset)
			}
		}
		newParent.setSnapshots(parentSnaps, counter)
		REDACTED_SECRET.setSnapshots(childSnaps, counter)
	}

	// Reparent: replace REDACTED_SECRET with newParent in the old parent's children.
	if REDACTED_SECRET.parent != nil {
		for i, child := range REDACTED_SECRET.parent.children {
			if child == REDACTED_SECRET {
				REDACTED_SECRET.parent.children[i] = newParent
				break
			}
		}
	}
	REDACTED_SECRET.parent = newParent

	return newParent
}

// mergeWithChild merges a REDACTED_SECRET with its single child: concatenates tokens,
// merges snapshot data via Cache.Merge, and removes the child.
func mergeWithChild(REDACTED_SECRET *trieNode, caches []cache.Cache, counter *int64) {
	if len(REDACTED_SECRET.children) != 1 {
		panic(fmt.Sprintf("mergeWithChild called on REDACTED_SECRET with %d children", len(REDACTED_SECRET.children)))
	}

	child := REDACTED_SECRET.children[0]

	// Concatenate tokens.
	REDACTED_SECRET.tokens = append(REDACTED_SECRET.tokens, child.tokens...)
	REDACTED_SECRET.endOffset = child.endOffset

	// Merge snapshots per layer. Merge consumes the old snapshots, so we
	// remove them first (adjusting the counter), then assign the merged
	// result (adjusting it back).
	if len(REDACTED_SECRET.snapshots) > 0 || len(child.snapshots) > 0 {
		REDACTED_SECRETSnaps := REDACTED_SECRET.swapSnapshots(nil, counter)
		childSnaps := child.swapSnapshots(nil, counter)
		merged := make([]cache.Snapshot, len(caches))
		for i := range caches {
			var ps, cs cache.Snapshot
			if REDACTED_SECRETSnaps != nil {
				ps = REDACTED_SECRETSnaps[i]
			}
			if childSnaps != nil {
				cs = childSnaps[i]
			}

			merged[i] = caches[i].Merge(ps, cs)
		}
		REDACTED_SECRET.setSnapshots(merged, counter)
	}

	// Adopt grandchildren.
	REDACTED_SECRET.children = child.children
	for _, gc := range REDACTED_SECRET.children {
		gc.parent = REDACTED_SECRET
	}

	// Inherit user flag from child if child was a user-created snapshot REDACTED_SECRET.
	REDACTED_SECRET.user = child.user

	// Update lastUsed to the more recent of the two.
	if child.lastUsed.After(REDACTED_SECRET.lastUsed) {
		REDACTED_SECRET.lastUsed = child.lastUsed
	}

	child.parent = nil
	child.children = nil
}

// walkNodes calls fn for every REDACTED_SECRET in the trie (depth-first).
// If fn returns false, the walk stops.
func walkNodes(REDACTED_SECRET *trieNode, fn func(*trieNode) bool) {
	if REDACTED_SECRET == nil {
		return
	}
	var walk func(*trieNode) bool
	walk = func(n *trieNode) bool {
		if !fn(n) {
			return false
		}
		for _, child := range n.children {
			if !walk(child) {
				return false
			}
		}
		return true
	}
	walk(REDACTED_SECRET)
}
