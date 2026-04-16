# AnchorEdit Specification

Version: 0.1.0

Status: Draft



## 1. Overview

**AnchorEdit** is a specification for LLM-native code editing built on top of AnchorScope.

While AnchorScope provides deterministic verification and buffer management,
AnchorEdit defines how LLM agents reason, navigate, and perform edits using anchor buffers.

AnchorEdit introduces a Buffer-First paradigm and Tree-Navigation editing,
enabling safe, deterministic, and context-aware modifications to source code.



## 2. Design Goals

- Enable safe and deterministic code editing using AnchorScope.
- Provide a standardized reasoning framework for LLM agents.
- Reduce cognitive and operational complexity in code modification tasks.
- Externalize working memory through anchor buffers.
- Support hierarchical and scalable navigation of source code.
- Ensure interoperability across tools and platforms.



## 3. Layered Architecture

```

LLM
│
▼
AnchorEdit (Reasoning & Navigation Layer)
│
▼
AnchorScope (Deterministic Editing Protocol)
│
▼
Source Files

```


### Layer Responsibilities

| Layer | Responsibility | Nature |
|-------|---------------|--------|
| AnchorEdit | Reasoning, navigation, and editing strategy | Non-deterministic |
| AnchorScope | Hash verification and buffer management | Deterministic |
| Filesystem | Source code storage | Static |



## 4. Core Concepts

### 4.1 Buffer-First Editing
Editing is performed by navigating anchor buffers rather than directly searching source files.

### 4.2 Tree-Navigation Editing
Code is navigated hierarchically using nested anchoring.

### 4.3 Root Scope
The root scope is the initial anchored context from which navigation begins.

### 4.4 True ID as Address
Each anchored scope is uniquely identified by a True ID.

### 4.5 Externalized Working Memory
Anchor buffers serve as persistent working memory for LLM agents.

### 4.6 Deterministic Verification
All edits are validated by AnchorScope.



## 5. Design Principles

1. **Tool–Skill Separation**  
   Deterministic execution is separated from probabilistic reasoning.

2. **Buffer-First Principle**  
   Editing begins from anchor buffers.

3. **Hierarchical Anchoring**  
   Scopes are established through nested anchors.

4. **Locality of Context**  
   Edits are performed within the smallest valid scope.

5. **Deterministic Safety**  
   All changes are verified through hash validation.

6. **LLM-Native Reasoning**  
   The system is optimized for AI-driven workflows.



## 6. Root Scope Establishment

### 6.1 Objective
To establish a stable and unique context that contains the intended modification.

### 6.2 Strategy

1. Identify the target file.
2. Attempt to anchor the most relevant symbol.
3. Expand the scope hierarchically if necessary.
4. Fall back to anchoring the entire file.

### 6.3 Priority Order

| Priority | Scope |
|----------|-------|
| 1 | Function or Method |
| 2 | Class or Module |
| 3 | File |
| 4 | Project (optional) |

### 6.4 Pseudocode

```pseudo
function establish_root_scope(intent):
    file = resolve_target_file(intent)

    candidates = generate_candidate_anchors(intent)

    for anchor in candidates:
        result = anchorscope.read(file, anchor)
        if result.is_unique():
            return result

    return anchorscope.read(file, full_file_anchor)
```



## 7. Standard Workflow

1. Interpret user intent.
2. Establish the root scope.
3. Navigate to the target scope.
4. Generate modifications.
5. Validate using AnchorScope.
6. Commit changes.
7. Handle errors and retry if necessary.



## 8. State Model

```

INTENT
  ↓
ROOT_ESTABLISHED
  ↓
NAVIGATING
  ↓
TARGET_IDENTIFIED
  ↓
EDITING
  ↓
VERIFYING
  ↓
COMMITTED

```



## 9. Error Handling

| Error | Strategy |
|-------|----------|
| Anchor Not Found | Re-anchor with broader scope |
| Multiple Matches | Refine anchor |
| Hash Mismatch | Re-read and retry |
| Stale Buffer | Re-establish root scope |



## 10. Compliance

An implementation is considered AnchorEdit-compliant if it:

* Uses AnchorScope for deterministic verification.
* Follows the Buffer-First paradigm.
* Establishes a root scope before editing.
* Supports hierarchical anchoring.
* Ensures hash-verified writes.



## 11. Reference Implementations

| Implementation | Description |
|----------------|-------------|
| pi-anchoredit | Reference implementation for Pi |



## 12. Versioning

This specification follows Semantic Versioning.

| Version | Status |
|---------|--------|
| 0.1.0 | Initial Draft |



## 13. License

This specification is released under the MIT License.
