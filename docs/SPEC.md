# AnchorEdit v2 Specification

Version: 2.0.0
Status: Draft

---

## 1. Overview

**AnchorEdit v2** is a lightweight apply engine built on top of
[AnchorScope v2.0.0](https://github.com/kmlaborat/AnchorScope).

Its purpose is **not** file discovery or code generation.
Its purpose is safe and deterministic application of edits.

Core workflow:

```
Anchor → Apply → Verified Write
```

## 2. Design Goals

- Single responsibility: apply an edit at an anchored location
- No discovery logic — the caller provides the anchor
- Hash-verified writes via AnchorScope
- Minimal API surface

## 3. Architecture

```
Caller (LLM agent, script, CLI)
  ↓ provides anchor + replacement
AnchorEdit (apply)
  ↓ anchorscope::read() → scope_hash
  ↓ anchorscope::write() → verified write
AnchorScope
  ↓
Source File
```

AnchorEdit v2 delegates all matching and hashing to AnchorScope.
It does not implement its own search, bisection, or discovery logic.

## 4. Core Concepts

### 4.1 Anchor

An **anchor** is an exact byte sequence that uniquely identifies the target
scope within a file. The caller is responsible for providing an anchor that
exists exactly once in the file.

### 4.2 Replacement

The **replacement** is the byte sequence that substitutes for the matched scope.

### 4.3 scope_hash

The **scope_hash** is the xxh3_64 hash of the matched scope, returned by
`anchorscope::read()`. It is used internally to verify the match before writing.

## 5. API

### 5.1 Library

```rust
pub fn apply(
    file_path: &str,
    anchor: &[u8],
    replacement: &[u8],
) -> Result<ApplyResult, ApplyError>
```

**Flow:**

1. `anchorscope::read(file_path, anchor)` → `scope_hash`
2. `anchorscope::write(file_path, anchor, scope_hash, replacement)` → `bytes_written`
3. Return `ApplyResult { bytes_written, scope_hash }`

### 5.2 CLI

```bash
anchoredit apply \
  --file <path> \
  --anchor "<string>" \
  --replacement "<string>"
```

Or with files:

```bash
anchoredit apply \
  --file <path> \
  --anchor-file anchor.txt \
  --replacement-file replacement.txt
```

**Success output:**

```
OK: written N bytes
```

**Error output:**

```
NO_MATCH
MULTIPLE_MATCHES
HASH_MISMATCH
IO_ERROR: ...
```

## 6. Error Conditions

| Error | Cause |
| :--- | :--- |
| `NO_MATCH` | Anchor not found in file |
| `MULTIPLE_MATCHES` | Anchor matches more than once |
| `HASH_MISMATCH` | File modified between read and write |
| `IO_ERROR` | File I/O failure |

Because `apply()` obtains the hash from `read()` and passes it directly to
`write()`, `HASH_MISMATCH` can only occur if the file is modified by an
external process between the two calls.

## 7. Non-Goals

- Anchor discovery (handled by the caller or v1 Sliding Bisection)
- Multi-file operations
- Code generation
- Semantic understanding of file content

## 8. Versioning

| Version | Status |
| :--- | :--- |
| 2.0.0 | Current — apply engine on AnchorScope library |

## 9. License

MIT License
