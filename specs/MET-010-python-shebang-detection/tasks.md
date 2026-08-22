---

description: "Dependency-ordered TDD task list for MET-010 Python shebang detection"
---

# Tasks: MET-010 Python Shebang Detection

**Input**: Design documents from `/specs/MET-010-python-shebang-detection/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/python-shebang-eligibility.md`, `quickstart.md`

**Tests**: TDD is required. Each behavior-changing task is a vertical red → green slice through the public library seam defined by the contract.

**Organization**: Tasks are grouped by user story so the story remains independently implementable and testable.

## Format: `[ID] [deps:...] [P?] [Story?] Description`

- **[deps:none]**: No prerequisite tasks
- **[deps:T###,...]**: Complete the listed earlier tasks first
- **[P]**: Can run in parallel without conflicting files or incomplete dependencies
- **[Story]**: User story traceability label
- Every task names the exact code, test, or documentation paths it changes

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm the existing Rust package, input module, integration-test layout, and standard-library temporary-fixture approach.

No setup changes are required. The existing package and test support are sufficient, and adding speculative setup would not produce a legitimate non-`tasks.md` change.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Confirm the pre-agreed public seam and compatibility evidence before behavior work.

The public seam is already specified by `contracts/python-shebang-eligibility.md` as `rusty_kode::has_python_shebang(&Path) -> bool`; no foundational code change is required.

**Checkpoint**: The existing `src/input.rs`, `src/lib.rs`, and filesystem-backed integration-test structure are ready for the vertical slice.

---

## Phase 3: User Story 1 - Discover Executable Python Scripts (Priority: P1) 🎯 MVP

**Goal**: Classify a non-`.py` candidate as Python-shebang eligible exactly when its readable first line starts with `#!` and contains the case-sensitive lowercase substring `python`.

**Independent Test**: Through the public `rusty_kode::has_python_shebang(&Path) -> bool` seam, filesystem fixtures reproduce all six acceptance scenarios plus the specified empty-file, leading-whitespace, later-line-only, `python3`, missing-path, and `.py`-rule-separation cases with MET-010 / Azure 250 / pinned-Radon evidence in assertion context.

### Vertical TDD Slice for User Story 1

- [X] T001 [deps:none] [US1] Prove and implement the observable behavior “a candidate is eligible only from a readable first line that starts exactly with `#!` and contains lowercase `python`” at the public `rusty_kode::has_python_shebang(&Path) -> bool` seam: first add a failing filesystem-backed behavioral test in `tests/python_shebang_detection.rs` (with fixture cleanup and MET-010, Azure work item 250, and Radon commit `54b88e5878b2724bf4d77f97349588b811abdff2` assertion context added in `tests/support/mod.rs`) covering `#!/usr/bin/env python`, `#!/usr/bin/python3`, uppercase-only `Python`, `python` without an initial `#!`, `#!` without lowercase `python`, invalid UTF-8, empty input, leading whitespace, later-line-only evidence, a missing path, and independence from `is_python_filename`; run `cargo test --test python_shebang_detection` inside dev container `2b9126c5d406` at `/workspaces/rusty-kode` and require the new test to fail because the public predicate is absent; then make the minimum implementation in `src/input.rs` by opening the path read-only, using buffered UTF-8 `read_line` for only the first line, returning `false` on open/read/decode failure, and evaluating exact `starts_with("#!") && contains("python")`, re-export it from `src/lib.rs`, rerun the focused test to green, and fold final verification into this task by running `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check` in that same container and working directory

**Checkpoint**: User Story 1 is fully functional and independently verified at its public seam.

---

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Close the slice without introducing work outside MET-010.

No separate polish or verification task is required. T001 owns the focused red → green cycle, traceability evidence, cleanup behavior, and final repository checks. Any additional task would either overlap the same observable behavior or lack a legitimate non-`tasks.md` change.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Existing infrastructure is sufficient; no task is required.
- **Foundational (Phase 2)**: The contract has already fixed the public seam; no task is required.
- **User Story 1 (Phase 3)**: T001 has `[deps:none]` and can start immediately.
- **Polish (Phase 4)**: Folded into T001; no standalone task is permitted.

### User Story Dependencies

- **User Story 1 (P1)**: No dependency on another story and no hard feature prerequisite.

### Task Dependency Graph

```text
T001 [US1, deps:none]
```

### Cumulative TDD Dependency Check

- T001 starts from the current public API, where `has_python_shebang` is absent, so its behavioral integration test must fail before implementation.
- The exact-prefix, lowercase-substring, first-line-only, unreadable-input, and edge scenarios belong to the same classifier behavior. Splitting them would create overlapping tasks because the minimum correct predicate makes the rejection scenarios pass together; they are therefore merged into the earliest and only owning task.

### Parallel Opportunities

- None. This atomic feature has one vertical slice touching the same integration-test and input-boundary files; subdividing it would violate cumulative red-before-green ordering or create file conflicts.

---

## Parallel Example: User Story 1

No parallel task launch is valid for User Story 1. Execute T001 as one ordered red → green slice.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Execute T001: add the public-seam behavioral test and observe red.
2. In the same task, add only the classifier and re-export needed for green.
3. Run the focused test and the folded final repository verification inside the required dev container.

### Incremental Delivery

MET-010 contains one P1 story and one observable classification behavior, so completion of T001 is the complete independently deliverable increment.

---

## Notes

- All build, test, formatter, linter, and program execution commands in T001 must run with `docker exec` in container `2b9126c5d406` from `/workspaces/rusty-kode`.
- Test only through the agreed public library seam; do not test private helpers or mock internal Rust modules.
- Use literal expected booleans from the specification rather than recomputing the classifier predicate in the test.
- Do not add traversal, `.py` eligibility, executable-bit checks, parsing, analysis, reporting, or non-Python detection.
- Stop after the minimum green implementation; refactoring is outside this TDD task.
