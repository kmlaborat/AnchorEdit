# AnchorEdit

**AnchorEdit** is a specification for LLM-native code editing built on top of AnchorScope.

It defines how AI agents reason, navigate, and perform deterministic edits using anchor buffers.

## Key Features

- Buffer-First Editing
- Tree-Navigation Editing
- Deterministic Verification via AnchorScope
- Externalized Working Memory
- Tool–Skill Separation

## Architecture

```

LLM → AnchorEdit → AnchorScope → Source Code

```

## Reference Implementation

- **pi-anchoredit** (coming soon)

## Repository Structure

- `SPEC.md` — The official specification
- `examples/` — Usage examples
- `docs/` — Design documents
- `diagrams/` — Architecture diagrams

## Status

Draft specification. Contributions are welcome.

## License

MIT License
