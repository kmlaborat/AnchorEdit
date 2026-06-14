# AnchorEdit

AnchorEdit is a specification for LLM-native code editing
built on top of AnchorScope v2.0.0.

It defines how an LLM agent selects an anchor, submits it
to AnchorScope, and handles the result.

## Core Idea

anchor = scope

Protection breadth equals anchor length.
The agent decides what to protect by choosing the anchor.

## Repository Structure

- `SPEC.md` — The official specification (v0.2.0)
- `SLIDING_BISECTION.md` — Anchor discovery algorithm (informative)

## Architecture

```
LLM Agent
  ↓ choose anchor
AnchorEdit
  ↓ read / write
AnchorScope v2.0.0
  ↓
Source File
```

## Status

Draft specification. v0.2.0.

## License

MIT License
