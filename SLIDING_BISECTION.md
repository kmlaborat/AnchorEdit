# Sliding Bisection

## A Scope Localization Algorithm for LLM Agents

---

## Overview

**Sliding Bisection** is a scope localization algorithm designed for LLM agents
to efficiently identify and isolate a target region within a file for editing.

It operates on raw byte sequences without semantic analysis, AST parsing, or
language awareness. The algorithm works on any file type — source code, prose,
configuration files, or binary files.

Sliding Bisection is designed to complement [AnchorScope](https://github.com/kmlaborat/AnchorScope),
which provides deterministic read and write operations on the identified scope.

---

## Motivation

### How LLM Agents Navigate Files

LLM agents naturally develop a sense of file structure by reading sequentially
in chunks. After reading a file, an agent typically knows:

> "The target function is roughly in the second half of the file."

This positional intuition is reliable but imprecise. Sliding Bisection converts
that imprecise intuition into a precisely bounded scope through a small number
of 3-choice selections.

### Why Not AST or Semantic Analysis?

AST-based approaches require:

- Language-specific parsers
- Valid, parseable syntax (breaks on incomplete code)
- Mapping between LLM intuition and tree nodes

This is misaligned with how LLMs naturally reason about code. LLMs think in
terms of approximate position ("around line 200"), not syntax trees.

Sliding Bisection stays close to the LLM's natural intuition: **rough position
→ 3-choice selection → progressively smaller scope**.

### Context Window Efficiency

Once a scope is identified via Sliding Bisection, subsequent re-reads target
only that scope rather than the full file:

```
Full file re-read:          5000 lines → high context cost
Post-bisection scope read:    ~50 lines → low context cost
```

If large edits shift the target's position, the agent re-reads the file and
restarts Sliding Bisection. The retry cost is low because each bisection
attempt requires only a few 3-choice selections.

---

## Algorithm

### Phase 1: File Comprehension (Agent's Responsibility)

The agent reads the file in chunks and develops positional awareness:

> "The target is approximately 60% into the file."

This phase is outside Sliding Bisection. It is the agent's natural reading
behavior.

---

### Phase 2: Sliding Bisection (Scope Localization)

#### Step 1: Trisect with Overlap

Divide the current scope into three overlapping segments:

```
Segment A: [  0% ──────── 40%]
Segment B: [30% ──────── 70%]   ← center, overlaps A and C
Segment C: [60% ──────── 100%]
```

Overlap width is 10% on each side (configurable). The overlap ensures that
a target near a boundary always falls fully within at least one segment.

#### Step 2: Agent Selects

Show the agent the three segments and ask:

> "Which segment contains the target?"

The agent selects one segment based on its positional awareness.

#### Step 3: Recurse

The selected segment becomes the new scope. Repeat from Step 1.

#### Step 4: Termination

Stop when the scope is small enough to edit directly (e.g., ≤ 50 lines or
≤ 2000 bytes). The resulting scope is passed to AnchorScope for
`read` / `write`.

---

### Boundary Uniqueness: The Sliding Mechanism

When computing a split point at 50% of the current scope, the boundary bytes
must form a unique anchor within the parent scope. If the exact midpoint
produces a non-unique sequence:

1. Shift the split point by ±0.5% of the scope size (in bytes)
2. Repeat until a unique boundary is found
3. The shift range is bounded at ±10% before falling back to a wider segment

This is the "sliding" in Sliding Bisection. It handles repeated patterns
(e.g., duplicate lines, boilerplate) without requiring semantic understanding.

```
Target split: 50.0%
→ Non-unique. Try 50.5%
→ Non-unique. Try 49.5%
→ Non-unique. Try 51.0%
→ Unique. Use this boundary.
```

---

## Properties

| Property | Description |
|:---|:---|
| **Language-agnostic** | Operates on raw bytes; works on any file type |
| **Semantic-free** | No AST, no parser, no language awareness |
| **LLM-aligned** | Matches the agent's natural positional intuition |
| **Retry-friendly** | Each attempt is lightweight; restarts are cheap |
| **Context-efficient** | Narrows scope progressively, reducing re-read cost |
| **Deterministic boundary** | Sliding ensures unique anchors for AnchorScope |

---

## Convergence

For a file of N bytes with a target scope of T bytes, the number of
bisection rounds required is approximately:

```
rounds = log₃(N / T)
```

Example:

| File size | Target scope | Rounds |
|:---|:---|:---|
| 5,000 lines | 50 lines | ≈ 4 rounds |
| 10,000 lines | 50 lines | ≈ 5 rounds |
| 100,000 lines | 50 lines | ≈ 7 rounds |

Each round requires one 3-choice selection by the agent. The total interaction
cost is low even for very large files.

---

## Relationship to AnchorScope

Sliding Bisection and AnchorScope are complementary tools with distinct roles:

| Layer | Tool | Responsibility |
|:---|:---|:---|
| Comprehension | Agent | Read file, develop positional awareness |
| Localization | Sliding Bisection (AnchorEdit) | Narrow scope to target region |
| Editing | AnchorScope | Deterministic read / write with hash verification |

Sliding Bisection produces a bounded byte range. AnchorScope's `read` command
takes that range, computes `scope_hash`, and enables safe `write`.

Neither tool depends on the other's internals. An agent may use Sliding
Bisection with any byte-level editing tool, and AnchorScope may be used
without Sliding Bisection when the target is already known.

---

## Retry Behavior

If the target's position shifts after a large edit (e.g., many lines
inserted above the target):

1. Agent re-reads the file (or the affected region)
2. Agent updates its positional estimate
3. Sliding Bisection restarts from the full file scope

Restart cost is proportional to the number of bisection rounds, not the
file size. For typical cases (4–5 rounds), this is negligible.

---

## Parameters

| Parameter | Default | Description |
|:---|:---|:---|
| `overlap_ratio` | 10% | Overlap width on each side of a segment boundary |
| `slide_step` | 0.5% | Byte offset increment when searching for unique boundary |
| `slide_limit` | 10% | Maximum slide range before widening segment |
| `termination_bytes` | 2000 | Stop bisecting when scope is smaller than this |

These parameters may be tuned based on file characteristics and agent behavior.

---

## Summary

Sliding Bisection narrows an LLM agent's positional intuition into a
precisely bounded, hash-verifiable scope through a small number of
3-choice selections.

It requires no language knowledge, no syntax parsing, and no semantic
understanding. Retries are cheap. Context usage is minimized.

> **Position over parsing.
> Selection over search.
> Scope before edit.**
