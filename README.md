# AnchorEdit

AnchorEdit is a lightweight apply engine built on top of AnchorScope v2.
Its purpose is not file discovery or code generation.
Its purpose is safe and deterministic application of edits.

Core workflow:

Anchor → Apply → Verified Write

## Installation

cargo install --path .

## Prerequisites

AnchorScope v2.0.0: https://github.com/kmlaborat/AnchorScope

## Usage

anchoredit apply \
  --file <path> \
  --anchor "<string>" \
  --replacement "<string>"

## Library Usage

use anchoredit::apply;

let result = apply(file_path, anchor.as_bytes(), replacement.as_bytes())?;

## Documents

- docs/SPEC.md — Specification
- v1/ — Legacy v1 (Sliding Bisection, archived)

## License

MIT License
