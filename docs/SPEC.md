# AnchorEdit Specification

Version: 0.2.0
Status: Draft

The key words "MUST", "MUST NOT", "SHOULD", and "MAY" in this document are to
be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

---

## 1. Overview

**AnchorEdit** is a specification for LLM-native code editing built on top of
[AnchorScope v2.0.0](https://github.com/kmlaborat/AnchorScope).

AnchorEdit defines how an LLM agent selects an anchor, submits it to AnchorScope,
and handles the result. It does not prescribe how the anchor is discovered.

---

## 2. Design Goals

- Enable precise, deterministic code editing via AnchorScope
- Define the agent's responsibilities clearly
- Remain agnostic to anchor discovery strategy
- Keep the protocol minimal and stable across discovery algorithm changes

---

## 3. Layered Architecture

```
LLM Agent
│  comprehend file, decide what to edit, choose anchor
▼
AnchorEdit  (this specification)
│  submit anchor, handle result
▼
AnchorScope  (read / write)
│  hash-verified deterministic editing
▼
Source File
```

AnchorEdit sits between the agent's intent and AnchorScope's execution.
It defines the contract for that boundary.

---

## 4. Core Concepts

### 4.1 Anchor

An **anchor** is an exact byte sequence that uniquely identifies the target
scope within a file. The agent is responsible for choosing an anchor that:

* Exists exactly once in the file
* Covers the full byte range the agent intends to protect and replace

The breadth of protection equals the length of the anchor. A longer anchor
protects a larger range; a shorter anchor protects less.

### 4.2 Scope

The **scope** is the byte range matched by the anchor. It is defined entirely
by the anchor string. There is no separate scope boundary mechanism.

```
anchor = scope
```

### 4.3 Replacement

The **replacement** is the byte sequence the agent provides to substitute for
the matched scope. AnchorScope writes it verbatim to the file.

The agent is responsible for the correctness and encoding of the replacement.

### 4.4 scope_hash

The **scope_hash** is returned by AnchorScope `read`. It is the hash of the
matched byte sequence.

The agent retains `scope_hash` and passes it to `write` as `--expected-hash`.
It serves as a confirmation that the anchor seen at `read` time is the same
anchor seen at `write` time.

`scope_hash` is **not** a file-level consistency check. Changes outside the
anchored scope do not affect `scope_hash`.

---

## 5. Standard Workflow

### 5.1 Read

The agent submits an anchor to AnchorScope `read`:

```bash
as read --file <path> --anchor "<anchor>"
```

AnchorScope returns:

```
scope_hash=<16-char hex>
content=<matched bytes>
```

The agent retains `scope_hash` and inspects `content`.

### 5.2 Edit

The agent constructs `replacement` based on `content` and its intent.
This step is entirely the agent's responsibility.

### 5.3 Write

The agent submits the replacement to AnchorScope `write`:

```bash
as write \
  --file <path> \
  --anchor "<anchor>" \
  --expected-hash <scope_hash> \
  --replacement "<replacement>"
```

---

## 6. Success and Failure Conditions

| Condition | AnchorScope output | Agent action |
| :--- | :--- | :--- |
| Anchor found exactly once, hash matches | Write succeeds | Done |
| Anchor not found | `NO_MATCH` | Revise anchor; re-run `read` |
| Anchor found more than once | `MULTIPLE_MATCHES` | Widen anchor to make it unique |
| Anchor found, hash does not match | `HASH_MISMATCH` | Verify `--expected-hash`; re-run `read` |
| File I/O error | `IO_ERROR: ...` | Inspect file permissions and path |

### Notes

* `NO_MATCH` after a successful `read` means the anchored byte sequence was
  modified or deleted between `read` and `write`.
* `HASH_MISMATCH` means the `--expected-hash` value does not match the current
  file state. This typically indicates a stale or incorrect hash was provided.
* `MULTIPLE_MATCHES` means the anchor is not unique. The agent MUST widen
  the anchor until it matches exactly once.
* If the returned anchor appears incomplete
  (e.g., truncated mid-statement), the agent MAY
  retry `ae search` with a larger `--termination-bytes`
  value to obtain a wider anchor.

---

## 7. Agent Responsibilities

The agent **MUST**:

1. Choose an anchor that exists exactly once in the file
2. Retain `scope_hash` between `read` and `write`
3. Construct a valid UTF-8 replacement
4. Handle all error conditions defined in Section 6

The agent **MAY** use any strategy to discover a suitable anchor, including
but not limited to manual selection, Sliding Bisection, or semantic analysis.

---

## 8. Guarantees

AnchorEdit inherits all guarantees from AnchorScope v2.0.0. In addition:

1. The scope is defined entirely by the anchor; no implicit boundary detection
2. Protection breadth equals anchor length; the agent controls the trade-off
3. Changes outside the anchored scope never cause write failure

---

## 9. Non-Goals

* Anchor discovery strategy (see Informative References)
* Multi-file operations
* Version history or snapshots
* Automatic anchor generation
* Semantic understanding of file content

---

## 10. Informative References

The following documents describe strategies and algorithms that MAY be used
with AnchorEdit but are not part of this specification:

* **SLIDING_BISECTION.md** — A scope localization algorithm for LLM agents
  that narrows a target region through repeated 3-choice selections without
  semantic analysis. Recommended for large files where the agent has approximate
  positional awareness.

Additional anchor discovery strategies may be defined in future documents
without requiring a version change to this specification.

---

## 11. Versioning

| Version | Status |
| :------ | :----- |
| 0.1.0   | Initial Draft (superseded; mixed algorithm details into spec) |
| 0.2.0   | Current Draft (protocol only; algorithm details moved to informative references) |

---

## 12. License

MIT License
