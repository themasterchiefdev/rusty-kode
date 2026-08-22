---

description: "Dependency-ordered vertical TDD task for MET-009 Python extension detection"
---

# Tasks: MET-009 Python Extension Detection

**Input**: Design documents from `/specs/MET-009-python-extension-detection/`

**Prerequisites**: `plan.md` and `spec.md` (required); `research.md`, `data-model.md`, `contracts/python-extension-eligibility.md`, and `quickstart.md` (available)

**Tests**: Required by the requested TDD workflow. The behavioral test uses the public library boundary and is written and observed failing before the minimum implementation is added.

**Organization**: Work is grouped under User Story 1. The acceptance scenarios are one classifier behavior and therefore one vertical red-to-green task: after implementing exact lowercase `.py` suffix matching, separate rejection tasks for `.PY`, non-terminal `.py`, or missing `.py` would already pass and would violate cumulative TDD task independence.

## Format: `[ID] [deps:...] [P?] [Story] Description`

- **[deps:none]**: No prerequisite tasks
- **[deps:T001,...]**: Depends on the listed earlier tasks
- **[P]**: Can run in parallel with another incomplete task because it changes different files and has no unmet dependency
- **[Story]**: User story mapped from `spec.md`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm existing project infrastructure is sufficient.

No setup changes are required. The existing Rust package, integration-test layout, input module, and shared test-support module provide all planned paths and dependencies.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Confirm the public test seam and feature boundary before behavior work.

No separate foundational change is required. The approved public seam is the side-effect-free library operation `rusty_kode::is_python_filename(&OsStr) -> bool`; its implementation belongs at the existing input boundary and must not depend on traversal, file I/O, analysis, or another language.

**Checkpoint**: The existing repository baseline supports the single vertical story task.

---

## Phase 3: User Story 1 - Recognize Python File Names (Priority: P1) 🎯 MVP

**Goal**: Classify a supplied platform-native filename as eligible Python source if and only if its final encoded bytes are exactly lowercase `.py`.

**Independent Test**: Call the public predicate with `module.py`, `module.py.bak`, `module.PY`, and `module`; verify that only `module.py` is accepted, then exercise boundary rows such as `.py`, nested-looking values ending in `.py`, mixed-case suffixes, empty/short values, and trailing characters without performing filesystem work.

### Vertical TDD Implementation

- [X] T001 [deps:none] [US1] Deliver the single observable classifier behavior through the public seam `rusty_kode::is_python_filename(&OsStr) -> bool`: first add MET-009/Azure-249/pinned-Radon traceability context in `tests/support/mod.rs` and a failing table-driven behavioral integration test in `tests/python_extension_detection.rs` proving exact lowercase terminal `.py` acceptance plus uppercase, mixed-case, embedded/non-terminal, missing, empty/short, and trailing-character rejection; run `docker exec 2b9126c5d406 sh -lc 'cd /workspaces/rusty-kode && cargo test --test python_extension_detection'` and confirm the new target fails specifically because the public predicate behavior is absent; then add only the minimum allocation-free, case-sensitive encoded-byte suffix predicate in `src/input.rs` and its public re-export in `src/lib.rs` to make that test pass, without traversal, normalization, I/O, analysis, diagnostics, mutation, or non-Python recognition; finally run the focused test followed by `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check` inside the same required container and record successful completion in the task checkbox. Test path: `tests/python_extension_detection.rs`; test-support path: `tests/support/mod.rs`; implementation paths: `src/input.rs`, `src/lib.rs`.

**Checkpoint**: User Story 1 is independently functional and all MET-009 acceptance evidence is available through the public library contract.

---

## Final Phase: Polish & Cross-Cutting Concerns

No standalone polish or verification task is justified for this atomic slice. Traceability, boundary coverage, scope review, focused validation, full regression tests, linting, and formatting are folded into T001 so every task produces a legitimate code or test change.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Existing infrastructure; no task required.
- **Foundational (Phase 2)**: Existing public-library architecture establishes the seam; no task required.
- **User Story 1 (Phase 3)**: T001 has `[deps:none]` and can start immediately from the repository baseline.
- **Polish (Final Phase)**: Folded into T001; no separate verification-only task.

### User Story Dependencies

- **User Story 1 (P1)**: No dependency on another story and no hard feature prerequisite.

### Cumulative TDD Dependency Check

- T001 starts from the current baseline, where `rusty_kode::is_python_filename` is absent, so its new public-seam behavioral test must fail before implementation.
- All required positive, negative, case-sensitive, placement, and short-value scenarios are merged into T001 because the minimum exact-suffix implementation needed for the first acceptance scenario would already satisfy separately scheduled rejection scenarios.
- Red and green occur within T001; there is no horizontal split between test and implementation tasks.

### Parallel Opportunities

None. This feature has one indivisible vertical behavior task, and splitting its shared test and implementation paths would break the required red-to-green slice.

---

## Parallel Example: User Story 1

No parallel launch example applies. Execute T001 as one ordered red-to-green cycle.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Execute T001: add the public-seam test and observe the behavior-specific failure.
2. Add the minimum exact lowercase `.py` suffix predicate and public re-export.
3. Complete the focused and repository-wide container verification embedded in T001.
4. Stop: User Story 1 is the complete MET-009 MVP and feature scope.

### Incremental Delivery

MET-009 contains one independently deliverable behavior. Later traversal, direct-input, analysis, metrics, reports, diagnostics, and non-Python recognition remain outside this task list.

## Notes

- T001 follows red → green at the public library seam and tests observable results rather than internal calls.
- Expected values are specification literals, not values recomputed by the implementation algorithm.
- No internal collaborator is mocked; the predicate has no external system boundary.
- Every command that builds, tests, formats, lints, or runs the program must use `docker exec` in container `2b9126c5d406` at `/workspaces/rusty-kode`.
