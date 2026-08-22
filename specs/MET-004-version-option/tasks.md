---

description: "Dependency-ordered implementation tasks for MET-004 Version Option"
---

# Tasks: MET-004 Version Option

**Input**: Design documents from `/specs/MET-004-version-option/`

**Prerequisites**: `plan.md` and `spec.md` (required); `research.md`, `data-model.md`, `contracts/version-option.md`, and `quickstart.md` (available)

**Tests**: TDD is required. Each behavior-changing task is one vertical red-to-green slice through a public seam.

**Organization**: Tasks are grouped by user story so the feature remains independently implementable and testable.

## Format: `[ID] [deps:...] [P?] [Story?] Description`

- **[deps:none]**: The task has no prerequisites.
- **[deps:T001,T002]**: The task depends on the listed earlier tasks.
- **[P]**: The task can run in parallel without touching the same files or relying on unfinished work.
- **[Story]**: The user story served by the task.
- Every task names exact code, test, or documentation paths.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm whether project initialization or shared tooling changes are needed.

No setup changes are required: the existing Rust package, `clap` dependency, binary target, public runner, and integration-test harness already provide the required infrastructure.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Confirm whether shared prerequisites must be implemented before the user story.

No separate foundational changes are required. Cargo package metadata in `Cargo.toml` is already the authoritative version source, and `rusty_kode::run` is already the public CLI seam.

---

## Phase 3: User Story 1 - Identify the analyser version (Priority: P1) 🎯 MVP

**Goal**: A standalone `rusty-kode --version` invocation succeeds, writes exactly one stdout line containing the current Cargo package version, writes no stderr, and completes before dispatch or source discovery.

**Independent Test**: Invoke the built binary 100 times with the sole user token `--version` and verify every run succeeds with one UTF-8 stdout line containing `env!("CARGO_PKG_VERSION")` and empty stderr; invoke `rusty_kode::run` with the same token, a panic/recording dispatch delegate, and a supplied writer to prove output uses the public runner seam and dispatch remains untouched.

### Vertical TDD Slice for User Story 1

- [X] T001 [deps:none] [US1] Deliver the observable standalone-version behavior through the public seams `rusty_kode::run` and `CARGO_BIN_EXE_rusty-kode`: first add a failing behavioral acceptance in `tests/version_option.rs` (with MET-004/Azure 244/reference provenance helpers in `tests/support/mod.rs`) proving the exact `--version` token renders one newline-terminated stdout line containing `env!("CARGO_PKG_VERSION")`, returns success across 100 built-binary runs, emits no stderr, and never calls the supplied dispatch delegate; include a nearby non-trigger case proving additional/different tokens remain delegated unchanged; then make only the minimum implementation in `src/cli.rs` needed to register `env!("CARGO_PKG_VERSION")` on the shared `clap::Command`, recognize exactly one UTF-8 `--version` argument, write clap's standard version rendering through the supplied output writer, propagate write failures, and return before dispatch; finally verify inside `rusty-kode-dev` from `/workspaces/rusty-kode` with `cargo test --test version_option`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check`.

**Checkpoint**: User Story 1 is independently complete when T001 is green at both the runner and built-process seams.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No changes required.
- **Foundational (Phase 2)**: No changes required.
- **User Story 1 (Phase 3)**: T001 has no prerequisites and can start immediately.

### User Story Dependencies

- **User Story 1 (P1)**: Independent; no dependency on another story.

### Cumulative TDD Dependency Check

- **T001**: With no prior minimum implementations, its standalone `--version` behavioral test fails because the current runner delegates every non-empty argument sequence instead of rendering a version. The built-binary, repetition, no-dispatch, exact-trigger, and current-version scenarios are intentionally kept in this earliest owning slice because they become satisfied by the same minimum route and would overlap if split into later tasks.

### Parallel Opportunities

- None. The feature is one irreducible vertical behavior slice whose tests and minimum implementation share `tests/version_option.rs`, `tests/support/mod.rs`, and `src/cli.rs`.

---

## Parallel Example: User Story 1

No parallel task launch is valid for this story. Execute T001 as one red-to-green slice so its public behavioral test is observed failing before the minimum implementation is added.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Execute T001 and observe the new standalone-version acceptance fail at the public runner/process seam.
2. Add only the `src/cli.rs` behavior required to make that acceptance pass.
3. Run the focused and full verification commands embedded in T001.

### Incremental Delivery

MET-004 contains one user story and one observable compatibility behavior. Completion of T001 is the MVP and the complete feature; combined arguments, alternate flags, version policy, packaging, and analysis behavior remain outside this slice.

---

## Notes

- Tests must observe behavior only through `rusty_kode::run` and the built `rusty-kode` executable, never private helpers or parser internals.
- Red must be observed before green; do not pre-implement speculative argument semantics.
- The exact standalone trigger is one UTF-8 token equal to `--version`; all other sequences stay on the existing path unchanged.
- Final verification is folded into T001; there is no standalone verification-only task.
