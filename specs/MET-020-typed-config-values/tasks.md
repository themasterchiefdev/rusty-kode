---

description: "Dependency-ordered TDD tasks for MET-020 typed configuration values"
---

# Tasks: MET-020 Typed Configuration Values

**Input**: Design documents from `/specs/MET-020-typed-config-values/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/config-value-interpretation.md`, `quickstart.md`

**Tests**: Required by the requested TDD workflow. Each task is one vertical red → green slice through the confirmed public library seam; there are no separate test-only or implementation-only tasks.

**Confirmed public test seam**: Integration tests call `rusty_kode::{resolve_config_value, ConfigValue, ConfigValueError}` directly. They do not invoke the executable, configuration discovery, command-parser wiring, private helpers, or concrete command settings.

**Task format**: `- [ ] T### [deps:...] [P?] [US#?] Description with exact test and implementation paths`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm whether repository initialization is needed.

The existing Rust package, integration-test layout, and shared `tests/support/mod.rs` helper are sufficient. No setup task is created because every task must require a legitimate code, test, or documentation change beyond this file.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish prerequisites shared by every user story.

The first user-story slice creates the public value/error types and function seam together with its first behavior. Creating that API as a separate foundational task would violate the required vertical TDD slicing, so there is no standalone foundational task.

**Checkpoint**: The repository baseline is ready; begin with T001.

---

## Phase 3: User Story 1 — Apply typed configured defaults (Priority: P1) 🎯 MVP

**Goal**: Resolve configured integer and boolean text to values of the type selected by the caller's declared default, and classify invalid typed text without substitution.

**Independent Test**: Run `cargo test --test typed_config_values` in dev container `2b9126c5d406` and observe that strict signed-`i64` examples and the pinned case-insensitive boolean vocabulary resolve correctly while invalid inputs retain their exact opaque key/value in the correct error variant.

### Vertical TDD slices for User Story 1

- [ ] T001 [deps:none] [US1] **Observable behavior**: an integer-declared default resolves complete `[+-]?[0-9]+` text within the `i64` range and returns `InvalidInteger` with the exact opaque key/value for every grammar or range failure. **Public seam**: `rusty_kode::resolve_config_value` with `ConfigValue::Integer`. **Red**: first add the table-driven behavioral test `integer_defaults_resolve_only_strict_signed_i64_text`—covering signs, zero, both bounds, whitespace, separators, Unicode digits, empty/sign-only text, trailing content, and one-step overflow—and MET-020/Azure-260/reference assertion context; it must fail because the public API does not exist. **Green minimum**: add the public `ConfigValue` and `ConfigValueError` enums, export the module API, validate the complete ASCII integer grammar, parse only into `i64`, and map both grammar and range failures to the exact classified payload; leave boolean, text, and absent branches without their later behavior so their future tests remain red. **Test paths**: `tests/typed_config_values.rs`, `tests/support/mod.rs`. **Implementation paths**: `src/config.rs`, `src/lib.rs`.
- [ ] T002 [deps:T001] [US1] **Observable behavior**: a boolean-declared default resolves exactly `1|yes|true|on` to true and `0|no|false|off` to false using complete ASCII-case-insensitive matching, while every other token returns `InvalidBoolean` with the exact opaque key/value. **Public seam**: `rusty_kode::resolve_config_value` with `ConfigValue::Boolean`. **Red**: add the table-driven behavioral test `boolean_defaults_resolve_only_the_pinned_untrimmed_tokens`, including lower/upper/mixed case plus empty, padded, and unsupported text; after T001 it must fail because T001 implements only the integer branch. **Green minimum**: implement only the boolean branch with the eight pinned aliases and exact error payload, without trimming or fallback; keep configured-text and absent-value behavior for other variants untouched. **Test path**: `tests/typed_config_values.rs`. **Implementation path**: `src/config.rs`.

**Checkpoint**: User Story 1 is independently testable through the public library seam. T001 and T002 form the suggested MVP.

---

## Phase 4: User Story 2 — Preserve text defaults and absent defaults (Priority: P2)

**Goal**: Preserve configured text exactly for text-declared defaults and return any declared default unchanged when configured text is absent.

**Independent Test**: Run `cargo test --test typed_config_values` in dev container `2b9126c5d406` and observe that number-, boolean-, list-, enum-, whitespace-, and Unicode-looking text remains exact text, while `None` returns the original integer, boolean, or text value unchanged.

### Vertical TDD slices for User Story 2

- [ ] T003 [deps:T001,T002] [US2] **Observable behavior**: a text-declared default returns the configured UTF-8 text exactly, even when it resembles an integer, boolean, list, or enum value. **Public seam**: `rusty_kode::resolve_config_value` with `ConfigValue::Text` and `Some(&str)`. **Red**: add the table-driven behavioral test `text_defaults_preserve_configured_text_exactly`; after T001 and T002 it must fail because neither dependency implements the text branch. **Green minimum**: implement only the present-text branch by copying the supplied text into `ConfigValue::Text`, with no parsing, trimming, or validation. **Test path**: `tests/typed_config_values.rs`. **Implementation path**: `src/config.rs`.
- [ ] T004 [deps:T001,T002,T003] [US2] **Observable behavior**: an absent configured value returns the caller's integer, boolean, or text declared default exactly unchanged. **Public seam**: `rusty_kode::resolve_config_value` with `configured_value: None`. **Red**: add the table-driven behavioral test `absent_configuration_preserves_every_declared_default`; after T001–T003 it must still fail because all prior slices exercise `Some` and deliberately leave `None` unimplemented. **Green minimum**: return the owned declared default immediately for `None`, before inspecting its variant. Then complete final verification—inside dev container `2b9126c5d406` at `/workspaces/rusty-kode` only—using `cargo test --test typed_config_values`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --all -- --check`, correcting only MET-020 code/test paths as needed while preserving the specified public behavior. **Test path**: `tests/typed_config_values.rs`. **Implementation path**: `src/config.rs`.

**Checkpoint**: Both user stories are independently observable through the public seam, and the focused plus repository-wide checks pass.

---

## Phase 5: Polish & Cross-Cutting Concerns

No standalone polish or verification task is created. The feature is intentionally one small, stateless standard-library boundary; final verification is folded into T004, and speculative refactoring, CLI integration, configuration loading, concrete setting names, or unrelated documentation would exceed scope.

---

## Dependencies & Execution Order

### Task dependency graph

```text
T001 (integer seam and behavior)
  └── T002 (boolean behavior)
        └── T003 (configured text preservation)
              └── T004 (absent-value identity + final verification)
```

### Cumulative red-test proof

| Task | State after all dependency implementations | Why the new test is still red |
|---|---|---|
| T001 | Repository baseline only | The public types and `resolve_config_value` function do not exist. |
| T002 | Integer branch implemented | The boolean branch has no behavior yet. |
| T003 | Integer and boolean branches implemented | The configured-text branch has no behavior yet. |
| T004 | All `Some` branches implemented | The `None` branch has no behavior yet. |

No scenario is duplicated across tasks. Integer success and classified integer rejection remain in T001 because the minimum strict parser needed for valid inputs necessarily determines rejection; boolean success and classified boolean rejection similarly belong together in T002.

### User story dependencies

- **User Story 1 (P1)**: T001 → T002; no dependency on another story.
- **User Story 2 (P2)**: T003 → T004 after the shared public seam and typed branches from User Story 1. It remains independently testable at its documented seam.

### Parallel opportunities

There are no legitimate intra-feature parallel tasks. Every slice changes `src/config.rs` and `tests/typed_config_values.rs`, and each test is intentionally defined against the minimum implementation produced by the preceding slice. Parallel execution would create file conflicts or break the cumulative red-before-green guarantee.

---

## Implementation Strategy

### MVP first (User Story 1)

1. Complete T001 and observe red before adding the strict integer implementation.
2. Complete T002 and observe red before adding the pinned boolean implementation.
3. Run the focused integration target to validate User Story 1 independently.

### Incremental delivery

1. Deliver T001–T002 as the typed-default MVP.
2. Add T003 for exact configured-text preservation.
3. Add T004 for absent-value identity and run all final checks.

### TDD execution rules

- Execute one task at a time in dependency order.
- For each task, add and run only its named behavioral test first; confirm the expected red reason before changing implementation.
- Implement only the minimum described behavior needed for that test to pass; do not anticipate later task behavior.
- Test only through the confirmed public seam and mock nothing, because this boundary has no external dependency.
- Run all build, test, linter, formatter, and program commands via `docker exec` in container `2b9126c5d406` with working directory `/workspaces/rusty-kode`; never run them on the host.

---

## Task Summary

- **Total tasks**: 4
- **User Story 1**: 2 tasks (T001–T002)
- **User Story 2**: 2 tasks (T003–T004)
- **Parallel tasks**: 0
- **Suggested MVP**: User Story 1 (T001–T002)
- **Format**: Every task has a checkbox, sequential ID, immediate dependency marker, user-story label, one observable behavior, confirmed public seam, named failing behavioral test, minimum implementation, and exact test/implementation paths.
