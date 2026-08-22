---

description: "Implementation tasks for MET-007 standard input"
---

# Tasks: MET-007 Standard Input

**Input**: Design documents from `/specs/MET-007-standard-input/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/standard-input.md`, `quickstart.md`

**Tests**: TDD is required. Each behavior-changing task is one vertical red-to-green slice at the specification-approved public input-discovery API and injectable `MetricInput` consumer/delegate seams. Add the named behavioral test, run it to prove the behavior is missing, add only the minimum implementation needed to pass, and do not refactor beyond green.

**Organization**: Tasks are grouped by user story and ordered cumulatively. After every dependency's minimum implementation, the next task's named behavioral test still fails specifically because that task's behavior is absent.

## Format: `[ID] [deps:...] [P?] [Story?] Description`

- **[deps:none]**: The task has no prerequisite.
- **[deps:T001,...]**: The task depends on the listed earlier tasks.
- **[P]**: The task can run in parallel with other ready tasks because it changes different files.
- **[Story]**: The user story served by the task.

## Path Conventions

- Product code: `src/`
- Integration and acceptance tests: `tests/`
- Feature acceptance evidence: `specs/MET-007-standard-input/`
- All build, test, formatter, linter, and program execution commands run with `docker exec -w /workspaces/rusty-kode rusty-kode-dev ...`; none run directly on the host.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Reuse the existing Rust package and public library boundary without speculative infrastructure.

No separate setup task is needed. `Cargo.toml`, `src/lib.rs`, and the integration-test layout already establish the package and public test surface. The first vertical slice introduces only the input module and test support that its observable behavior requires.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Keep classification, injected I/O, consumer/delegate abstractions, and evidence helpers inside the earliest behavior that needs each one.

No standalone foundational task is needed. The repository-availability gate is traceably cleared, MET-007 has no hard prerequisite, and the approved public seams are already recorded in `specs/MET-007-standard-input/spec.md` and `specs/MET-007-standard-input/contracts/standard-input.md`.

**Checkpoint**: The existing repository baseline is ready for the first red-to-green slice.

---

## Phase 3: User Story 1 - Analyze Piped Source (Priority: P1) 🎯 MVP

**Goal**: A non-empty path collection containing only the exact token `-` consumes the supplied Python stream once logically and hands one complete input named `-` to the injectable consumer, regardless of repeated-token multiplicity.

**Independent Test**: Run `docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo test --test standard_input`; the public discovery API must hand the complete source and identity to the recording consumer once for one or repeated `-` tokens, and an injected read failure must return without a partial consumer handoff.

### Vertical TDD Slices for User Story 1

- [X] T001 [deps:none] [US1] Deliver the observable behavior "one exact `-` path atomically hands one complete stdin-derived Python input named `-`, while a read failure is propagated with no partial handoff" through the public input-discovery API and injectable `MetricInput` consumer seam; RED: add the failing behavioral test `single_standard_input_is_handed_off_atomically` in `tests/standard_input.rs`, with only the counting/failing reader, recording consumer, and MET-007/Azure-247/Radon-`54b88e5878b2724bf4d77f97349588b811abdff2` evidence support it needs in `tests/support/mod.rs`, asserting known literal source content, exact `-` identity, one logical consumer handoff, preserved I/O failure, and zero handoffs after failure without asserting low-level read-call count or private structure; GREEN: add only the public discovered-input/consumer boundary, exact single-token classification, read-to-completion, and atomic error propagation needed to pass in `src/input.rs` and `src/lib.rs`, leaving repeated-token and non-standard delegation behavior unimplemented so their later tests remain red; run the focused named test in `rusty-kode-dev` once to record the intended red result and again to record green.

- [X] T002 [deps:T001] [US1] Deliver the observable behavior "two or more exact `-` paths have multiplicity-insensitive semantics: one complete stream consumption and one complete input named `-`" through the same public input-discovery API and injectable consumer seam; RED: add the failing table-driven behavioral test `repeated_standard_input_tokens_collapse_to_one_handoff` in `tests/standard_input.rs` for two and three tokens using independently known Python payloads and the counting reader from `tests/support/mod.rs`, proving that T001's exact-single-token minimum does not yet hand off repeated-token requests; GREEN: make the minimum all-elements-equal-`-`, non-empty classification change in `src/input.rs` needed to pass without cloning per token, reading more than one logical stream, producing duplicate inputs, or adding non-standard path delegation; run the focused named test in `rusty-kode-dev` once to record red and again to record green.

**Checkpoint**: User Story 1 is independently functional at the approved public seam, including atomic failure behavior, and repeated tokens do not duplicate reads or inputs.

---

## Phase 4: User Story 2 - Keep Standard Input Explicit (Priority: P2)

**Goal**: Every path collection that is not a non-empty all-`-` collection bypasses stdin and reaches the injectable non-standard-input delegate unchanged, including mixed, ordinary, and empty collections.

**Independent Test**: Run `docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo test --test standard_input`; mixed `-` plus file paths, ordinary paths, and the empty collection must make zero reader accesses, create no stdin-derived input, and be observed by the delegate with original order, multiplicity, and OS/path values intact.

### Vertical TDD Slice for User Story 2

- [X] T003 [deps:T001,T002] [US2] Deliver the observable behavior "every collection that is not non-empty and all `-` bypasses stdin and is delegated exactly once with its complete ordered values and multiplicity unchanged" through the public input-discovery API and injectable non-standard-input delegate seam; RED: add the failing table-driven behavioral test `non_standard_input_collections_bypass_stdin_and_delegate_unchanged` in `tests/standard_input.rs` for `[-, sample.py]`, `[first.py, -, second.py, -]`, `[sample.py]`, and `[]`, using a panic-on-read reader and recording only public consumer/delegate observations from `tests/support/mod.rs` so T002's all-token implementation still fails because delegation is absent; GREEN: add only the borrowed-path non-standard route and injectable delegate handoff needed to pass in `src/input.rs` and `src/lib.rs`, preserving path order, multiplicity, and OS-string identity without opening paths, removing or reinterpreting mixed `-`, reading stdin, parsing Python, calculating metrics, or adding output/diagnostics; after green, fold final verification into this implementation task by running the focused test, full `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check` exactly as containerized in `specs/MET-007-standard-input/quickstart.md`, then create `specs/MET-007-standard-input/acceptance.md` recording the observed single-token, repeated-token, mixed-path, ordinary-path, empty-path, and read-failure evidence with feature `MET-007`, Azure work item `247`, and the pinned Radon commit.

**Checkpoint**: Both stories are independently testable, standard input remains explicit, and the complete MET-007 acceptance claim is reproducible without downstream metric behavior.

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Avoid standalone cleanup or verification work that would weaken the vertical TDD slices.

No separate polish task is needed. Formatting, linting, full regression verification, and acceptance-evidence recording are part of T003, the last behavior-changing implementation task. Refactoring is intentionally deferred beyond this red-to-green task list.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No tasks; the existing package baseline is sufficient.
- **Foundational (Phase 2)**: No tasks; shared seams are introduced by the first owning behavior.
- **User Story 1 (Phase 3)**: Starts immediately. T002 depends on T001 because both use `tests/standard_input.rs` and `src/input.rs`, and T002 must prove repeated-token behavior remains absent after the exact-single-token minimum reaches green.
- **User Story 2 (Phase 4)**: T003 depends on T001 and T002 because it extends their public discovery boundary and must prove non-standard delegation remains absent after all-stdin classification reaches green.
- **Polish (Phase 5)**: No standalone task; final verification is folded into T003.

### User Story Dependencies

- **User Story 1 (P1)**: No dependency on another story and defines the MVP.
- **User Story 2 (P2)**: Depends on User Story 1's public discovery surface and completed all-`-` classifier, but remains independently testable through its own no-read/delegation scenarios.

### Dependency Graph

```text
T001 (single-token atomic handoff) -> T002 (repeated-token collapse) -> T003 (non-standard no-read delegation + final verification)
```

### Cumulative Red Guarantee

1. Before T001, `single_standard_input_is_handed_off_atomically` fails because no public input-discovery API or consumer handoff exists.
2. After T001's minimum implementation, `repeated_standard_input_tokens_collapse_to_one_handoff` still fails because only the exact one-token shape is recognized.
3. After T002's minimum implementation, `non_standard_input_collections_bypass_stdin_and_delegate_unchanged` still fails because non-all-`-` collections have no delegate handoff.
4. Overlapping success/failure cases stay in their earliest owning slice; no later scenario is expected to pass incidentally from an earlier minimum implementation.

### Within Each Behavior Slice

1. Add only the named behavioral test and smallest support needed at its stated public seam.
2. Run that focused named test with `docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo test --test standard_input <test_name>` and confirm it fails because the behavior is absent.
3. Add only the stated minimum implementation.
4. Run the focused named test through the same container and confirm it passes.
5. Do not refactor, anticipate a later slice, or test private helpers/internal calls.

### Parallel Opportunities

- There are no safe parallel implementation tasks. All three slices intentionally evolve `tests/standard_input.rs` and `src/input.rs` in dependency order so each red result is meaningful.
- After T003 reaches green, its full test, clippy, and formatting commands may run independently as separate `docker exec` commands before `acceptance.md` is recorded.

---

## Parallel Example: Post-Green Verification

There is no parallel task pair during implementation. Once T003 is green, these independent checks are all part of T003:

```sh
docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo test
docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo clippy --all-targets --all-features -- -D warnings
docker exec -w /workspaces/rusty-kode rusty-kode-dev cargo fmt --all -- --check
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete T001 as the single-token atomic-handoff tracer bullet.
2. Complete T002 as the repeated-token compatibility tracer bullet.
3. Stop and validate User Story 1 independently with the focused `standard_input` target.

### Incremental Delivery

1. **T001**: One explicit token produces one complete, atomic input handoff.
2. **T002**: Repeated explicit tokens collapse to that same one-input behavior.
3. **T003**: Every other collection bypasses stdin, preserves delegation identity, and completes the regression and acceptance record.

---

## Notes

- The public seams are pre-agreed by the approved acceptance-boundary clarification: input-discovery API, injectable `MetricInput` consumer, and injectable non-standard-input delegate.
- Tests assert behavior through those public seams, never private classifiers or internal collaborator calls.
- Expected source, identity, paths, error, and provenance values are contract literals, not values recomputed with production logic or broad snapshots.
- A logical full-stream consumption may invoke `Read::read` multiple times; tests observe one route/consumer outcome and use zero low-level reads only for non-standard routes.
- File/directory discovery, source parsing, metric calculation, production metric consumers, built-binary payload output, diagnostics, persistence, report formatting, and non-Python behavior remain out of scope.
- Commit after each completed vertical slice when implementation is executed.

---

## Phase 6: Convergence

- [ ] T004 [deps:T003] Add an explicit standard-input origin/provenance discriminator to the public `MetricInput` contract in `src/input.rs`, set it when stdin discovery creates an input, re-export any public provenance type through `src/lib.rs`, and extend `tests/standard_input.rs` plus its recording consumer support to verify the discriminator through the public consumer seam as required by `data-model.md:33-42` and `contracts/standard-input.md:31-33` per tasks.md:163 (partial)
