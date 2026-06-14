# AnchorEdit

AnchorEdit is a specification and CLI for LLM-native code editing
built on top of AnchorScope v2.0.0.

## Core Idea

anchor = scope

Protection breadth equals anchor length.
The agent decides what to protect by choosing the anchor.

## Architecture

```
LLM Agent
  ↓ ae search → narrow scope via Sliding Bisection
AnchorEdit (ae)
  ↓ ae read / ae write → delegate to AnchorScope
AnchorScope (anchorscope)
  ↓
Source File
```

## Installation

### Prerequisites

- [AnchorScope v2.0.0](https://github.com/kmlaborat/AnchorScope)

### Build & Install

```bash
cargo install --path .
```

### Optional: short alias

Add to your shell profile (`.bashrc`, `.zshrc`, etc.):

```bash
alias ae=anchoredit
```

### Environment

```bash
# Optional: specify path to anchorscope binary
export ANCHORSCOPE_BIN=/path/to/anchorscope
```

## Commands

### ae search

Narrow a target scope using [Sliding Bisection](docs/SLIDING_BISECTION.md).
Trisects the file (or range) into three overlapping segments with preview text,
and returns JSON output.

```bash
ae search --file <path>
```

**Options**

| Option | Default | Description |
| :--- | :--- | :--- |
| `--file` | (required) | Path to the target file |
| `--range` | `0.0:1.0` | Start:end as file-size fractions (e.g. `0.3:0.7`) |
| `--termination-bytes` | `512` | Stop when range is smaller than this |
| `--preview-bytes` | `256` | Bytes to include in each segment preview |
| `--overlap` | `0.1` | Overlap ratio on each side (0.0–1.0) |

**Output — narrowing** (scope still needs narrowing):

```json
{
  "range": [0.0, 1.0],
  "size_bytes": 48000,
  "segments": [
    {
      "id": "A",
      "range": [0.0, 0.4],
      "size_bytes": 19200,
      "preview": "fn main() {\n    println!(..."
    },
    {
      "id": "B",
      "range": [0.3, 0.7],
      "size_bytes": 19200,
      "preview": "    let x = 42;\n    let y = ..."
    },
    {
      "id": "C",
      "range": [0.6, 1.0],
      "size_bytes": 19200,
      "preview": "    Ok(())\n}\n// end of file"
    }
  ]
}
```

**Output — done** (scope is small enough):

```json
{
  "done": true,
  "size_bytes": 487,
  "anchor": "fn hello() {\n    println!(\"hello\");\n}"
}
```

### ae read

Read content matched by an anchor. Wraps `anchorscope read`.

```bash
ae read --file <path> --anchor "<anchor>"
```

**Options**

| Option | Description |
| :--- | :--- |
| `--file` | Path to the target file (required) |
| `--anchor` | Anchor string to match |
| `--anchor-file` | Path to a file containing the anchor |

`--anchor` and `--anchor-file` are mutually exclusive; one is required.

**Output** (on success):

```
scope_hash=<16-char hex>
content=<matched bytes>
```

### ae write

Write a replacement for the anchored scope. Wraps `anchorscope write`.

```bash
ae write \
  --file <path> \
  --anchor "<anchor>" \
  --expected-hash <scope_hash> \
  --replacement "<replacement>"
```

**Options**

| Option | Description |
| :--- | :--- |
| `--file` | Path to the target file (required) |
| `--anchor` | Anchor string to match |
| `--anchor-file` | Path to a file containing the anchor |
| `--expected-hash` | Scope hash from a previous `ae read` (required) |
| `--replacement` | Replacement string |
| `--replacement-file` | Path to a file containing the replacement |

`--anchor` / `--anchor-file` are mutually exclusive (one required).
`--replacement` / `--replacement-file` are mutually exclusive (one required).

## Typical Workflow

```bash
# Step 1: Narrow scope via Sliding Bisection
ae search --file path/to/file.rs
# → agent selects a segment, repeats with --range until done:true

# Step 2: Read (get scope_hash)
ae read --file path/to/file.rs --anchor "<anchor from search>"

# Step 3: Edit (agent constructs replacement)

# Step 4: Write
ae write \
  --file path/to/file.rs \
  --anchor "<anchor>" \
  --expected-hash <scope_hash> \
  --replacement "<replacement>"
```

## Documents

- [docs/SPEC.md](docs/SPEC.md) — The official specification (v0.2.0)
- [docs/SLIDING_BISECTION.md](docs/SLIDING_BISECTION.md) — Sliding Bisection algorithm (informative)

## Status

v0.2.0 Draft.

## License

MIT License
