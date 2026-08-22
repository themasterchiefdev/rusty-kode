# Feature Specification: MET-004 Version Option

**Feature ID**: `MET-004`
**Azure Work Item**: 244 — MET-004: Version option
**Source Attachment SHA-256**: `bd64fcee0b08f2bd3ed16c2c6025bf0eef1c729af35ace7b127eaeb356dc04be`; private tracker URL omitted by stakeholder direction
**Feature Group**: Entrypoints
**Created**: 2026-08-19
**Compatibility Reference**: [Radon pinned revision `54b88e5878b2724bf4d77f97349588b811abdff2`](https://github.com/rubik/radon/tree/54b88e5878b2724bf4d77f97349588b811abdff2)

## Repository Readiness and Implementation Sequencing

**Specification status**: READY — repository gate cleared; implementation sequencing defined.

**Repository status**: READY. SETUP-001 / Azure work item 343 is closed, and the local Rust product repository `rusty-kode` is available. The former repository-availability blocker is cleared and does not restrict planning, task generation, or implementation.

**Implementation wave**: 1
**Canonical sequence**: 2 of 102
**Hard prerequisites**: None; this story may start after the repository baseline.

Only the prerequisites listed above constrain implementation order. References elsewhere to adjacent, output-owning, or verification-owning MET stories are scope boundaries, not implicit blockers. This story must prove its own acceptance scenarios without requiring unfinished downstream stories.

## Context and Scope

Users need to identify the analyser version from the command line so that an analysis can be reproduced. This is one independently testable compatibility slice for the CLI and does not include analysis execution or support for languages other than Python.

### In Scope

- The CLI standard version option.
- Displaying the current application version as observable command-line output.

### Out of Scope

- Running an analysis, accepting source input, or changing analysis results.
- Other CLI entrypoint stories, package installation, release management, or version-number policy.
- Support for analysing languages other than Python.
- Copying the reference implementation's internal parser-initialization mechanism.

## User Scenarios & Testing

### User Story 1 - Identify the analyser version (Priority: P1)

As a CLI user, I want to request the analyser version so that I can record the exact version used for a reproducible analysis.

**Why this priority**: Version identification is the sole user value and acceptance target of MET-004.

**Independent Test**: Invoke the CLI's standard version option without providing analysis input and verify that the command completes successfully and displays the current application version.

**Acceptance Scenarios**:

1. **Given** the CLI is available, **When** a user invokes it with `--version`, **Then** it completes successfully and writes one version-reporting line containing the current application version to standard output.
2. **Given** the CLI is available, **When** a user invokes it with `--version` and provides no source input, **Then** it does not begin source analysis or require a source path.

### Edge Cases

- A version request with no analysis target remains valid and must not report a missing-input error.
- The reported value must be the current application version, not a hard-coded historical version or a value derived from analysed source.
- Arguments or behavior outside the standard version option are not established by this story and must not be claimed as MET-004 compatibility.

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST expose the standard `--version` option.
- **FR-002**: When invoked with `--version`, the CLI MUST complete successfully and write one version-reporting line to standard output.
- **FR-003**: The version-reporting line MUST contain the current application version used to identify the analyser for reproducibility.
- **FR-004**: A `--version` invocation MUST be accepted without a source path or source content and MUST NOT start source analysis.
- **FR-005**: MET-004 compatibility MUST be evaluated only by the observable option, completion status, and version-reporting output; internal construction or initialization choices are outside this specification.

### Compatibility Boundaries

- This specification preserves only the observable version-option behavior established for the pinned Radon reference. It does not require a literal translation of Radon's internals.
- The initial product compatibility scope remains Python source analysis only. This version-option slice adds no behavior or claim for other languages.

### Evidence and Discrepancy Resolution

| Requirement | Authority and evidence | Resolution |
| --- | --- | --- |
| FR-001 to FR-004 | Enriched Azure story, work item 244: “The CLI exposes the current application version through its standard version option”; [pinned reference version definition](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/__init__.py#L4-L4); [pinned reference Program initialization](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L90-L96). | The public standard option is specified as `--version`, with successful version output and no analysis input. The evidence packet directs that public behavior be specified rather than copying the Mando initialization detail. |
| FR-005 | Constitution v1.1.0, Principles I, V, and VI; the intake's code-grounded consideration. | No conflict remains: the source explains a mechanism while the higher-precedence enriched story defines the user-visible result. This specification adopts the observable result and records the internal mechanism as out of scope. |

### Acceptance Mapping

| Acceptance evidence | Verifies | Outcome |
| --- | --- | --- |
| User Story 1, scenario 1 | FR-001, FR-002, FR-003 | `--version` is available, succeeds, and emits the current version on one standard-output line. |
| User Story 1, scenario 2 | FR-004 | The version request needs no analysis target and does not begin analysis. |
| Compatibility Boundaries and evidence-resolution table | FR-005 | Verification concerns public behavior rather than the reference's internal initialization. |
| SC-001 through SC-003 | FR-001 through FR-004 | Repeated observable checks establish reproducible version identification. |

## Success Criteria

### Measurable Outcomes

- **SC-001**: In 100 consecutive CLI invocations using `--version` with no source input, 100 complete successfully and each produces exactly one standard-output line containing the current application version.
- **SC-002**: In the same 100 invocations, 0 begin source analysis or require a source path.
- **SC-003**: A tester can record the version from the `--version` output and distinguish the analyser release used for an analysis in every acceptance test run.

## Assumptions and Dependencies

- The term “standard version option” in the enriched Azure story and pinned reference safely resolves to the public CLI flag `--version`; this decision is evidence-based and independently testable.
- The current application version is the version associated with the delivered analyser, and this story does not define how that version is created, stored, or released.
- Evidence precedence is: stakeholder direction, enriched Azure story and acceptance criteria, pinned Radon source and tests, then tracker wording. No unresolved discrepancy remains for MET-004.
- [Pinned Radon reference](https://github.com/rubik/radon/tree/54b88e5878b2724bf4d77f97349588b811abdff2) is the compatibility evidence for this feature; the intake records source evidence only, with no distinct reference test cited.
- Repository readiness is satisfied by the available `rusty-kode` product repository; delivery is governed by the dependency sequence recorded below.

## Constitution Compliance

- Behavioral compatibility is limited to observable CLI input, output, completion, and non-analysis behavior (Principle I).
- MET-004 remains independently testable and traceable to Azure work item 244 (Principle IV).
- Evidence links, precedence, and the only material interpretation are recorded above (Principles V and VII).
- This specification states user-visible behavior without prescribing implementation architecture (Principle VI).
- Repository readiness is satisfied by the available `rusty-kode` product repository; delivery is governed by the dependency sequence recorded below.
