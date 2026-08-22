# Feature Specification: MET-020 Typed Configuration Values

**Feature ID**: `MET-020`
**Azure Work Item**: 260 — MET-020: Typed config values
**Source Attachment SHA-256**: `9551eb88a8f206be90eeb19dd65413d07c7713dff2836e368ed670126e12ce68`; private tracker URLs omitted by stakeholder direction
**Created**: 2026-08-19

**Input Authority**: `intake/stories/MET-020.md`; Code Metrics Analyzer Constitution v1.1.0 only

## Repository Readiness and Implementation Sequencing

**Specification status**: READY — repository gate cleared; implementation sequencing defined.

**Repository status**: READY. SETUP-001 / Azure work item 343 is closed, and the local Rust product repository `rusty-kode` is available. The former repository-availability blocker is cleared and does not restrict planning, task generation, or implementation.

**Implementation wave**: 1
**Canonical sequence**: 6 of 102
**Hard prerequisites**: None; this story may start after the repository baseline.

Only the prerequisites listed above constrain implementation order. References elsewhere to adjacent, output-owning, or verification-owning MET stories are scope boundaries, not implicit blockers. This story must prove its own acceptance scenarios without requiring unfinished downstream stories.

## Context and Scope

Users of the Python-source metrics command need configuration values to preserve the meaning of the command defaults they replace. A configured whole-number default must be supplied as a whole number, a configured true/false default as a boolean value, and every other configured default as text. A missing configuration key must leave its command default unchanged.

This specification is an atomic configuration compatibility slice. It covers only interpretation of configured command-default values before command-line parsing; it does not broaden configuration discovery, source precedence, command-line override behavior, validation of text values, or analysis for languages other than Python.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Apply typed configured defaults (Priority: P1)

As a command user, I want configured integer and boolean defaults to retain their intended value types so that the command behaves as though I supplied compatible defaults directly.

**Why this priority**: Correctly interpreting these values is the core observable compatibility behavior of MET-020.

**Independent Test**: Supply one configuration section containing an integer-valued default and a boolean-valued default for a command, start the command without corresponding command-line overrides, and verify that both resulting defaults retain their respective value meanings.

**Acceptance Scenarios**:

1. **Given** a command default declared as an integer and a configuration entry containing a valid whole-number text value, **When** the configuration is applied, **Then** the resulting command default is that whole-number value rather than text.
2. **Given** a command default declared as a boolean and a configuration entry containing a valid boolean text value, **When** the configuration is applied, **Then** the resulting command default is the corresponding boolean value rather than text.

---

### User Story 2 - Preserve text defaults and absent defaults (Priority: P2)

As a command user, I want text-valued defaults to remain text and unspecified configuration keys to preserve the command's existing defaults, so that unrelated settings are not changed.

**Why this priority**: The reference behavior limits typed conversion to integer and boolean defaults and explicitly preserves absent defaults.

**Independent Test**: Apply a configuration that provides one text-valued default and omits another supported key, then verify that the provided value is text and the omitted default remains exactly the command's original default.

**Acceptance Scenarios**:

1. **Given** a command default that is neither an integer nor a boolean and a configuration entry for it, **When** the configuration is applied, **Then** the resulting default is retrieved as text.
2. **Given** a command default for which no configuration entry is present, **When** the configuration is applied, **Then** the command retains its original default unchanged.

### Edge Cases

- A command default whose configured value is an invalid whole-number or boolean representation is not silently converted to a different value; its observable failure behavior remains compatible with the pinned reference.
- A text-valued default, including a value that resembles a number or boolean, is retrieved as text; later validation of text values is outside MET-020.
- A list- or enum-like default is treated as text for this story, even if later command behavior validates it.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST interpret a configured value as a whole-number value when, and only when, the corresponding command default is declared as an integer.
- **FR-002**: The system MUST interpret a configured value as a boolean value when, and only when, the corresponding command default is declared as a boolean.
- **FR-003**: The system MUST retrieve a configured value as text when the corresponding command default is neither an integer nor a boolean, including list- and enum-like defaults.
- **FR-004**: The system MUST retain the original command default unchanged when the relevant configuration key is absent.
- **FR-005**: The system MUST preserve the pinned reference's observable outcome for invalid integer or boolean configuration text and MUST NOT substitute a different value silently.
- **FR-006**: The system MUST apply these rules only to Python-source command configuration within the initial compatibility scope.

### Acceptance Mapping

| Requirement | Acceptance evidence | Authority |
|---|---|---|
| FR-001 | User Story 1, scenario 1; SC-001 | Enriched Azure acceptance criterion; [pinned conversion evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L39-L61) |
| FR-002 | User Story 1, scenario 2; SC-001 | Enriched Azure acceptance criterion; [pinned conversion evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L39-L61) |
| FR-003 | User Story 2, scenario 1; SC-002 | Enriched Azure acceptance criterion; [pinned conversion evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L39-L61) |
| FR-004 | User Story 2, scenario 2; SC-003 | Enriched Azure acceptance criterion; [pinned test evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/tests/test_cli.py#L69-L87) |
| FR-005 | Edge case 1; SC-004 | Constitution I and VIII; pinned conversion evidence |
| FR-006 | Context and Scope; SC-005 | Constitution II |

### Key Entities

- **Command default**: The pre-existing value associated with a command setting; its declared type determines whether a provided configuration value is converted.
- **Configured value**: The text supplied for a command setting in configuration; it either becomes a typed value, remains text, or is absent.
- **Configuration key**: The named setting that associates a configured value with a command default; absence preserves the command default.

## Evidence, Discrepancies, and Resolutions

Evidence precedence is: stakeholder direction, enriched Azure story and acceptance criteria, pinned Radon source and tests, then tracker wording, as required by Constitution v1.1.0.

- **E-001 — Azure work item 260**: The story requires integer and boolean configuration values to be interpreted correctly, with absent keys retaining command defaults.
- **E-002 — Pinned Radon behavior**: The supplied reference records distinct integer, boolean, and text retrieval behavior. [Conversion source](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L39-L61)
- **E-003 — Pinned Radon test**: The supplied test evidence covers conversion of configured types. [Test source](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/tests/test_cli.py#L69-L87)
- Repository readiness is satisfied by the available `rusty-kode` product repository; delivery is governed by the dependency sequence recorded below.

No material discrepancy exists among the supplied authorities. The tracker shorthand is resolved consistently with the higher-precedence enriched acceptance criteria and pinned evidence: only declared integer and boolean defaults are typed; all other defaults are text; absent keys retain their defaults.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In a compatibility check containing at least one configured integer default and one configured boolean default, 100% of those values have the same resulting value meanings as the pinned reference.
- **SC-002**: In a compatibility check containing at least one configured non-integer, non-boolean default, 100% of those values are retained as text, including number- or boolean-looking text.
- **SC-003**: In a compatibility check covering each supported command setting with its configuration key absent, 100% of original command defaults remain unchanged.
- **SC-004**: In compatibility checks containing invalid integer and boolean configuration text, 100% of observed outcomes match the pinned reference, with no silent replacement by a different value.
- **SC-005**: The MET-020 acceptance evidence covers Python-source command configuration only and makes zero claims of support for another analysis language.

## Assumptions

- The command defaults whose configuration is in scope have already been identified by the command surface; this story does not add settings or keys.
- Configuration is applied before command-line parsing, as stated in the supplied evidence packet; command-line precedence is outside this story.
- Later configuration sources overriding earlier sources is group context only and is not specified or redefined by MET-020.
- No security, privacy, accessibility, localization, throughput, availability, rate-limiting, concurrency, import/export, or external-service requirement is introduced by this local value-interpretation rule.
