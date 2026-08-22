# Feature Specification: MET-020 Typed Configuration Values

**Feature ID**: `MET-020`
**Azure Work Item**: 260 — MET-020: Typed config values
**Source Attachment SHA-256**: `9551eb88a8f206be90eeb19dd65413d07c7713dff2836e368ed670126e12ce68`; private tracker URLs omitted by stakeholder direction
**Created**: 2026-08-19
**Revised**: 2026-08-22 — stakeholder clarification after the first delivery run

**Input Authority**: `intake/stories/MET-020.md`; stakeholder clarification dated 2026-08-22; Code Metrics Analyzer Constitution v1.1.0

## Repository Readiness and Implementation Sequencing

**Specification status**: READY — repository gate cleared; implementation sequencing defined.

**Repository status**: READY. SETUP-001 / Azure work item 343 is closed, and the local Rust product repository `rusty-kode` is available. The former repository-availability blocker is cleared and does not restrict planning, task generation, or implementation.

**Implementation wave**: 1
**Canonical sequence**: 6 of 102
**Hard prerequisites**: None; this story may start after the repository baseline.

Only the prerequisites listed above constrain implementation order. References elsewhere to adjacent, output-owning, or verification-owning MET stories are scope boundaries, not implicit blockers. This story must prove its own acceptance scenarios without requiring unfinished downstream stories.

## Context and Scope

The Python-source metrics command needs a configuration-value interpretation boundary that preserves the meaning of the command defaults supplied to it. A configured signed whole-number default must be supplied as a signed whole number, a configured true/false default as a boolean value, and every other configured default as text. A missing configuration key must leave its declared default unchanged.

This specification is an atomic configuration compatibility slice. It covers only interpretation of one already-looked-up optional configuration value against its declared default. The independently observable boundary accepts the configuration key, declared default, and optional configured text, then returns the resolved typed default or a classified invalid-value outcome. It does not require starting the executable, loading configuration sources, applying command-line overrides, or wiring settings into the command parser. Those integrations belong to the stories that introduce real configuration sources and real configurable command settings.

The current repository has no configurable command settings. MET-020 therefore MUST NOT introduce, name, or simulate settings merely to demonstrate this rule. Its acceptance evidence is the public configuration-value interpretation boundary, restricted to values intended for Python-source command configuration.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Apply typed configured defaults (Priority: P1)

As a configuration consumer, I want configured integer and boolean defaults to retain their intended value types so that later command integration can apply compatible typed defaults without reinterpreting text.

**Why this priority**: Correctly interpreting these values is the core observable compatibility behavior of MET-020.

**Independent Test**: Through the public configuration-value interpretation boundary, supply integer and boolean declared defaults with corresponding configured text and verify that both results retain their respective value meanings.

**Acceptance Scenarios**:

1. **Given** a default declared as a signed 64-bit integer and configured text matching an optional ASCII sign followed by one or more ASCII decimal digits, **When** the value is interpreted, **Then** the result is that signed 64-bit whole-number value rather than text.
2. **Given** a command default declared as a boolean and a configuration entry containing a valid boolean text value, **When** the configuration is applied, **Then** the resulting command default is the corresponding boolean value rather than text.

---

### User Story 2 - Preserve text defaults and absent defaults (Priority: P2)

As a command user, I want text-valued defaults to remain text and unspecified configuration keys to preserve the command's existing defaults, so that unrelated settings are not changed.

**Why this priority**: The reference behavior limits typed conversion to integer and boolean defaults and explicitly preserves absent defaults.

**Independent Test**: Through the same public boundary, provide configured text for a text-valued default and omit configured text for integer, boolean, and text defaults; verify that provided text remains text and every omitted value returns its original declared default.

**Acceptance Scenarios**:

1. **Given** a command default that is neither an integer nor a boolean and a configuration entry for it, **When** the configuration is applied, **Then** the resulting default is retrieved as text.
2. **Given** a command default for which no configuration entry is present, **When** the configuration is applied, **Then** the command retains its original default unchanged.

### Edge Cases

- A declared default whose configured value is invalid under this story's bounded integer or boolean grammar is not silently converted to a different value; it returns the classified outcome defined by FR-005.
- Integer text containing whitespace, separators, non-ASCII digits, no digits, or a value outside the signed 64-bit range is invalid for this bounded slice and returns a classified failure containing the supplied key and value.
- A text-valued default, including a value that resembles a number or boolean, is retrieved as text; later validation of text values is outside MET-020.
- A list- or enum-like default is treated as text for this story, even if later command behavior validates it.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST interpret configured text as a signed 64-bit whole-number value when, and only when, the corresponding default is declared as an integer and the complete text matches `[+-]?[0-9]+` with a value in the signed 64-bit range.
- **FR-002**: The system MUST interpret a configured value as a boolean value when, and only when, the corresponding command default is declared as a boolean.
- **FR-003**: The system MUST retrieve a configured value as text when the corresponding command default is neither an integer nor a boolean, including list- and enum-like defaults.
- **FR-004**: The system MUST retain the original command default unchanged when the relevant configuration key is absent.
- **FR-005**: The system MUST return a classified invalid-integer or invalid-boolean outcome containing the supplied key and value for invalid typed text and MUST NOT substitute a different value silently.
- **FR-006**: The system MUST apply these rules only to Python-source command configuration within the initial compatibility scope.
- **FR-007**: MET-020 acceptance MUST be observable through the public configuration-value interpretation boundary without requiring configuration discovery, executable startup, or command-parser integration.
- **FR-008**: MET-020 MUST NOT add, name, or simulate command settings or configuration keys; callers supply the key and declared default owned by their later integration story.

### Acceptance Mapping

| Requirement | Acceptance evidence | Authority |
|---|---|---|
| FR-001 | User Story 1, scenario 1; SC-001 | Enriched Azure acceptance criterion; [pinned conversion evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L39-L61) |
| FR-002 | User Story 1, scenario 2; SC-001 | Enriched Azure acceptance criterion; [pinned conversion evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L39-L61) |
| FR-003 | User Story 2, scenario 1; SC-002 | Enriched Azure acceptance criterion; [pinned conversion evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L39-L61) |
| FR-004 | User Story 2, scenario 2; SC-003 | Enriched Azure acceptance criterion; [pinned test evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/tests/test_cli.py#L69-L87) |
| FR-005 | Edge case 1; SC-004 | Constitution I and VIII; pinned conversion evidence |
| FR-006 | Context and Scope; SC-005 | Constitution II |
| FR-007 | Both independent tests; SC-006 | Stakeholder clarification, 2026-08-22 |
| FR-008 | Context and Scope; SC-007 | Stakeholder clarification, 2026-08-22 |

### Key Entities

- **Declared default**: A value supplied by a later command-setting integration; its declared type determines whether configured text is converted.
- **Configured value**: The text supplied for a command setting in configuration; it either becomes a typed value, remains text, or is absent.
- **Configuration key**: An opaque caller-supplied identifier associated with a configured value and used in classified failures; absence preserves the declared default.

## Evidence, Discrepancies, and Resolutions

Evidence precedence is: stakeholder direction, enriched Azure story and acceptance criteria, pinned Radon source and tests, then tracker wording, as required by Constitution v1.1.0.

- **E-001 — Azure work item 260**: The story requires integer and boolean configuration values to be interpreted correctly, with absent keys retaining command defaults.
- **E-002 — Pinned Radon behavior**: The supplied reference records distinct integer, boolean, and text retrieval behavior. [Conversion source](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L39-L61)
- **E-003 — Pinned Radon test**: The supplied test evidence covers conversion of configured types. [Test source](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/tests/test_cli.py#L69-L87)
- **E-004 — Stakeholder clarification, 2026-08-22**: After the first delivery run exposed that the repository has no real configurable command settings, the stakeholder authorized retrying MET-020 at the public configuration-value interpretation boundary, without invented settings or executable-wiring claims, and authorized pinning the integer grammar for this slice so the contract is finite and independently testable.
- Repository readiness is satisfied by the available `rusty-kode` product repository; delivery is governed by the dependency sequence recorded below.

The stakeholder clarification has higher precedence than the enriched Azure story and pinned Radon evidence. It resolves two discrepancies exposed by the first delivery run:

- Executable command integration cannot be demonstrated without inventing settings, so MET-020 now ends at the public value-interpretation boundary. Later stories that own real configuration sources and settings own executable integration.
- Python's integer lexer accepts forms beyond this story's bounded need, including arbitrary-size values, separators, and Unicode decimal digits. MET-020 deliberately narrows valid integer text to signed-64-bit `[+-]?[0-9]+`; broader lexical compatibility is not claimed by this story.

The remaining evidence is consistent: only declared integer and boolean defaults are typed; all other defaults are text; absent values retain their defaults; invalid typed text never silently substitutes another value.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: At the public value-interpretation boundary, 100% of signed-64-bit integer examples matching `[+-]?[0-9]+` and all pinned boolean tokens (`1`, `yes`, `true`, `on`, `0`, `no`, `false`, `off`, case-insensitively) resolve to the specified typed values.
- **SC-002**: In a compatibility check containing at least one configured non-integer, non-boolean default, 100% of those values are retained as text, including number- or boolean-looking text.
- **SC-003**: With configured text absent, integer, boolean, and text declared defaults each remain exactly unchanged.
- **SC-004**: Integer text outside FR-001 and boolean text outside the pinned token vocabulary each produce the corresponding classified failure with the exact supplied key and value, with no silent replacement.
- **SC-005**: The MET-020 acceptance evidence covers Python-source command configuration only and makes zero claims of support for another analysis language.
- **SC-006**: All MET-020 acceptance evidence invokes the public value-interpretation boundary directly and makes zero claims that executable configuration loading or parser wiring is delivered.
- **SC-007**: The MET-020 implementation and tests introduce zero concrete command-setting names or configuration keys.

## Assumptions

- Later command-setting stories will supply their own keys and declared defaults to this boundary; this story does not add settings or keys.
- Later integration applies configuration before command-line parsing; command-line precedence is outside this story.
- Later configuration sources overriding earlier sources is group context only and is not specified or redefined by MET-020.
- No security, privacy, accessibility, localization, throughput, availability, rate-limiting, concurrency, import/export, or external-service requirement is introduced by this local value-interpretation rule.
