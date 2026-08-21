---

description: "Implementation tasks for MET-003 help on empty invocation"
---

# Tasks: MET-003 Help on Empty Invocation

**Input**: Design documents from `/specs/MET-003-help-on-empty-invocation/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/empty-invocation.md`, `quickstart.md`

**Tests**: TDD is required. Each behavior-changing task is one vertical red-to-green slice: add the named behavioral test at a public seam, prove it fails for the intended reason, then add only the minimum implementation needed to pass it. Do not split a slice into separate test and implementation tasks or refactor beyond the green state.

**Organization**: Tasks are grouped by user story so MET-003 remains independently implementable and testable.

## Format: `[ID] [deps:...] [P?] [Story?] Description`

- **[deps:none]**: The task has no prerequisite.
- **[deps:T001,...]**: The task depends on the listed earlier tasks.
- **[P]**: The task can run in parallel with other ready tasks because it does not modify the same files.
- **[Story]**: The user story served by the task.

## Path Conventions

- Product code: `src/`
- Integration and acceptance tests: `tests/`
- Feature acceptance evidence: `specs/MET-003-help-on-empty-invocation/`
- All Cargo, test, formatter, linter, and binary commands run in dev container `rusty-kode-dev` at `/workspaces/rusty-kode`.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm the existing single-package Rust CLI baseline before behavior is added.

No separate setup task is needed: `Cargo.toml` and `src/main.rs` already establish the `rusty-kode` package and binary. Dependency and module changes belong to the first vertical TDD slice so the test is written before its supporting implementation.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Avoid speculative infrastructure and keep shared command construction, process capture, and routing code inside the first behavior that requires them.

No standalone foundational task is needed. The repository-availability gate is already traceably cleared, and User Story 1 has no hard prerequisite.

**Checkpoint**: The initialized repository baseline is ready for the first red-to-green slice.

---

## Phase 3: User Story 1 - Discover Usage Without a Command (Priority: P1) 🎯 MVP

**Goal**: An invocation with zero user-supplied tokens displays standard top-level help instead of a parser-failure diagnostic, while every non-empty token sequence remains outside the empty-invocation route.

**Independent Test**: Run `docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo test --test help_on_empty`; the built binary must show recognizable `rusty-kode` help/usage with no parser-failure diagnostic, the empty route must not call the normal delegate, and representative non-empty sequences must reach that delegate unchanged exactly once.

### Vertical TDD Slices for User Story 1

- [ ] T001 [deps:none] [US1] Deliver the observable behavior "an empty `rusty-kode` invocation renders shared top-level help, is not presented as a parser failure, and does not invoke normal dispatch" through the public built-binary seam plus the public application runner seam; RED: add the failing behavioral test `empty_invocation_displays_help_without_parser_failure_or_dispatch` in `tests/help_on_empty.rs`, with only the process-capture and MET-003/Azure-243/Radon-`54b88e5878b2724bf4d77f97349588b811abdff2` evidence helpers it needs in `tests/support/mod.rs`, asserting semantic command identity/usage across combined output and rejecting missing-command or required-argument diagnostics without fixing stream, exact text, or exit status; GREEN: add only `clap` 4.x command construction and the zero-token route needed to pass in `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/cli.rs`, and `src/main.rs`, deriving help from the shared command definition and keeping analyzer/dispatch side effects out of that route; run the focused test first to record the intended red result and again to record green.

- [ ] T002 [deps:T001] [US1] Deliver the observable boundary behavior "each non-empty ordered OS-string token sequence is delegated unchanged exactly once and never intercepted as MET-003 help" through the public application runner seam; RED: add the failing table-driven behavioral test `non_empty_invocations_are_forwarded_unchanged_exactly_once` in `tests/help_on_empty.rs` for option-like, command-like, invalid-looking, ordered, and non-UTF-8-capable OS-string inputs, observing calls only through a recording public delegate and making no assertion about downstream parser output; GREEN: make the minimum routing change in `src/cli.rs` and public exposure in `src/lib.rs` needed to pass, without consuming, inserting, removing, reordering, decoding, normalizing, reparsing, or redefining any non-empty token and without changing the empty behavior from T001; run the focused test first to record red and again to record green.

**Checkpoint**: User Story 1 is independently functional and its changed behavior and non-empty boundary are covered at public seams.

---

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Validate the complete MET-003 slice and preserve reproducible acceptance evidence.

- [ ] T003 [deps:T001,T002] Validate the completed slice inside `rusty-kode-dev` using the commands in `specs/MET-003-help-on-empty-invocation/quickstart.md` (`cargo test`, focused `cargo test --test help_on_empty`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --all -- --check`, and a manual zero-argument binary invocation), then record the observed help=true, parser-failure=false, dispatch-invoked=false, non-empty identity-boundary result, feature `MET-003`, Azure work item `243`, and pinned Radon commit in `specs/MET-003-help-on-empty-invocation/acceptance.md` without claiming exact formatting, output stream, exit status, or downstream non-empty behavior.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No tasks; the repository baseline already exists.
- **Foundational (Phase 2)**: No tasks; shared infrastructure is introduced by the first TDD slice that needs it.
- **User Story 1 (Phase 3)**: Starts immediately. T002 follows T001 because both modify `tests/help_on_empty.rs`, `src/cli.rs`, and `src/lib.rs`, and because the non-empty boundary uses the public runner introduced by T001.
- **Polish (Phase 4)**: T003 runs after both behavior slices are green.

### User Story Dependencies

- **User Story 1 (P1)**: Has no dependency on another story and is the complete MVP.

### Dependency Graph

```text
T001 (empty invocation help) -> T002 (non-empty identity delegation) -> T003 (full validation and evidence)
```

### Within Each Behavior Slice

1. Add only the named test and the smallest test support required at the stated public seam.
2. Run the focused test in `rusty-kode-dev` and confirm it fails because the behavior is absent.
3. Add only the stated minimum implementation.
4. Run the focused test again in `rusty-kode-dev` and confirm it passes.
5. Do not refactor or anticipate a later slice.

### Parallel Opportunities

- There are no safe parallel implementation tasks: T001 and T002 intentionally form sequential vertical slices over the same public API and files, and T003 validates both.
- Within T003, the full test suite, clippy, and formatting checks may be launched as independent container commands after T001 and T002 are complete, but the acceptance record must wait for all results.

---

## Parallel Example: User Story 1

User Story 1 has no parallel task pair because preserving the TDD learning loop requires T001 to reach green before T002 begins. After both slices are green, these verification commands are independent:

```sh
docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo test
docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo clippy --all-targets --all-features -- -D warnings
docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo fmt --all -- --check
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete T001 as one red-to-green empty-invocation tracer bullet.
2. Complete T002 as one red-to-green non-empty-boundary tracer bullet.
3. Stop and validate User Story 1 independently with T003.
4. Preserve the acceptance record as the MET-003 delivery evidence.

### Incremental Delivery

1. **T001**: Empty invocations become discoverable and non-failing in presentation.
2. **T002**: The out-of-scope non-empty boundary gains independent regression protection.
3. **T003**: The complete compatibility claim becomes reproducible and traceable.

---

## Notes

- T001 and T002 are vertical TDD tasks; neither may be split into separate test and implementation work.
- Tests observe only the built binary or public application runner/delegate boundary, never private helpers or internal collaborator calls.
- Expected values come from the feature contract and pinned evidence, not by recomputing implementation output or snapshotting full help text.
- Exact help words, formatting, stream, localization, and exit status remain out of scope.
- Non-empty parser, command, validation, analysis, output, and exit semantics remain out of scope.
- Commit after each completed task or coherent slice when implementation is executed.
