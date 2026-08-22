# Feature Specification: MET-003 Help on Empty Invocation

**Feature ID**: `MET-003`  
**Azure Work Item**: 243 — MET-003: Help on empty invocation
**Feature Group**: Entrypoints  
**Created**: 2026-08-19  
**Input Authority**: `intake/stories/MET-003.md` and Code Metrics Analyzer Constitution v1.1.0 only

## Repository Readiness and Implementation Sequencing

**Specification status**: READY — repository gate cleared; implementation sequencing defined.

**Repository status**: READY. SETUP-001 / Azure work item 343 is closed, and the local Rust product repository `rusty-kode` is available. The former repository-availability blocker is cleared and does not restrict planning, task generation, or implementation.

**Implementation wave**: 1
**Canonical sequence**: 1 of 102
**Hard prerequisites**: None; this story may start after the repository baseline.

Only the prerequisites listed above constrain implementation order. References elsewhere to adjacent, output-owning, or verification-owning MET stories are scope boundaries, not implicit blockers. This story must prove its own acceptance scenarios without requiring unfinished downstream stories.

## Context, Scope, and Evidence

New users who invoke the command-line analyzer without supplying a command need discoverable usage guidance instead of a command-parser failure. This is one independently testable compatibility slice for the Python-first analyzer.

### In Scope

- The command-line entrypoint invoked with no user-supplied arguments.
- Displaying the command's help/usage information for that invocation.
- Preserving the reference's observable result: help rather than a parser failure.

### Out of Scope

- Help content, formatting, localization, or behavior for non-empty invocations.
- New commands, command aliases, argument validation changes, or changes to parser behavior outside the no-argument case.
- Analysis of non-Python languages and all future-language behavior.
- Implementation planning, task generation, and implementation now that repository readiness is satisfied.

### Evidence and Precedence

1. Stakeholder direction in the MET-003 evidence packet: a Rust behavioral port of pinned Radon, Python source analysis first; other languages are future scope.
2. Azure work item 243: a new user omitting a command receives useful help; its acceptance criterion says `-h` is appended before parsing.
3. Pinned Radon reference at commit [`54b88e5878b2724bf4d77f97349588b811abdff2`](https://github.com/rubik/radon/tree/54b88e5878b2724bf4d77f97349588b811abdff2), [`radon/__init__.py` lines 7–17](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/__init__.py#L7-L17): the reference's no-argument path causes help to be shown; the packet records no direct reference test.

### Discrepancy Resolution

- **Affected requirements**: FR-001 through FR-004 and all acceptance scenarios.
- **Discrepancy**: The Azure acceptance wording describes appending `-h` before command parsing, while the code-grounded consideration says the reference mutates global argument state but the behavioral requirement is “show help,” not “append `-h`.”
- **Resolution**: The specification requires the observable outcome—help/usage rather than parser failure—and does not require any particular internal argument mutation. This applies the constitution's behavioral-compatibility rule and treats the enriched story's `-h` wording as evidence of intended user-visible help, not a required internal mechanism.

## User Scenarios & Testing

### User Story 1 - Discover Usage Without a Command (Priority: P1)

As a new user, I want to run the analyzer without a command and receive useful help so that I can discover valid usage instead of interpreting a parser failure.

**Why this priority**: This is the entire MET-003 compatibility outcome and independently gives first-time users a usable next step.

**Independent Test**: Invoke the CLI with no user-supplied arguments and verify that help/usage is displayed and no parser-failure diagnostic is presented.

**Acceptance Scenarios**:

1. **Given** the CLI entrypoint is available and the user supplies no arguments, **When** the invocation is processed, **Then** the user is shown the command's help or usage information.
2. **Given** the CLI entrypoint is available and the user supplies no arguments, **When** the invocation is processed, **Then** the observable result is help/usage rather than a parser-failure diagnostic.
3. **Given** a compatibility review of MET-003, **When** evidence is recorded for the no-argument scenario, **Then** it links the result to MET-003, Azure work item 243, and the pinned Radon reference.

### Edge Cases

- An invocation with one or more user-supplied arguments is outside this story's no-argument trigger and MUST retain its existing observable behavior; MET-003 does not redefine it.
- A help display produced for an empty invocation MUST NOT be classified or presented as a parser failure.
- The reference has no direct test for this path; acceptance evidence for MET-003 MUST therefore verify the stated observable outcome against the pinned source and story acceptance criterion.

## Requirements

### Functional Requirements

- **FR-001**: When the CLI entrypoint receives no user-supplied arguments, the system MUST display its help or usage information.
- **FR-002**: For a no-argument invocation, the system MUST present help/usage rather than a parser-failure diagnostic.
- **FR-003**: The no-argument behavior MUST be compatible with the observable behavior of the pinned Radon reference identified in this specification.
- **FR-004**: MET-003 acceptance evidence MUST identify MET-003, Azure work item 243, and the pinned Radon reference as the basis for verifying FR-001 through FR-003.
- **FR-005**: MET-003 MUST NOT change observable behavior for invocations that include one or more user-supplied arguments.

### Acceptance Mapping

| Requirement | Acceptance evidence |
|---|---|
| FR-001 | User Story 1, scenario 1 |
| FR-002 | User Story 1, scenario 2; Edge Case 2 |
| FR-003 | User Story 1, scenario 3; pinned reference link |
| FR-004 | User Story 1, scenario 3 |
| FR-005 | Edge Case 1 |

## Success Criteria

### Measurable Outcomes

- **SC-001**: In acceptance verification, 100% of no-argument CLI invocations display help or usage information.
- **SC-002**: In acceptance verification, 0% of no-argument CLI invocations present a parser-failure diagnostic.
- **SC-003**: 100% of MET-003 acceptance records link the observed no-argument result to Azure work item 243 and the pinned Radon reference.
- **SC-004**: The MET-003 verification set contains at least one independently executable no-argument scenario and one boundary check confirming that non-empty invocations remain outside this feature's changed behavior.

## Assumptions

- “No user-supplied arguments” means the command is invoked without a command or option after its executable name.
- “Help or usage information” means the CLI's standard user-facing guidance sufficient to discover valid usage; this specification does not prescribe its exact text or presentation.
- No direct reference test exists, so compatibility verification uses the pinned source, story acceptance criterion, and independently recorded observable result.
- The behavioral requirement is limited to Python-first canonical scope; it creates no behavior or commitment for other languages.
