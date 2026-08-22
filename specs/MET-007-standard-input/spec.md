# Feature Specification: MET-007 Standard Input

**Feature ID**: `MET-007`  
**Azure Work Item**: [247 — MET-007: Standard input](https://dev.azure.com/yvrkarthik/code-metrics/_workitems/edit/247)  
**Feature Group**: Input discovery  
**Target Surface**: CLI  
**Created**: 2026-08-19  

## Repository Readiness and Implementation Sequencing

**Specification status**: READY — repository gate cleared; implementation sequencing defined.

**Repository status**: READY. SETUP-001 / Azure work item 343 is closed, and the local Rust product repository `rusty-kode` is available. The former repository-availability blocker is cleared and does not restrict planning, task generation, or implementation.

**Implementation wave**: 1
**Canonical sequence**: 3 of 102
**Hard prerequisites**: None; this story may start after the repository baseline.

Only the prerequisites listed above constrain implementation order. References elsewhere to adjacent, output-owning, or verification-owning MET stories are scope boundaries, not implicit blockers. This story must prove its own acceptance scenarios without requiring unfinished downstream stories.

## Context, Authority, and Compatibility Scope

This independently testable compatibility slice lets a developer pipe Python source into the analyser without creating a temporary file. When the complete supplied path set is the standard-input token `-`, discovery yields the piped source under the filename `-`. The slice preserves externally observable behavior of the pinned Radon reference and does not prescribe implementation design.

**Authority order applied**: stakeholder direction; enriched Azure story and acceptance criteria; pinned Radon source and test evidence; tracker wording. The authoritative evidence packet is [MET-007](../../intake/stories/MET-007.md); the product governance authority is [Constitution v1.1.0](../../.specify/memory/constitution.md).

**Pinned compatibility reference**: [Radon at commit `54b88e5878b2724bf4d77f97349588b811abdff2`](https://github.com/rubik/radon/tree/54b88e5878b2724bf4d77f97349588b811abdff2), with [discovery/open evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/tools.py#L214-L224), [sentinel-handling evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/tools.py#L244-L269), and [standard-input test evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/tests/test_cli_tools.py#L40-L75).

### Scope Boundaries

- In scope: recognizing standard input only when the complete supplied path set contains `-` and nothing else.
- In scope: treating one or more repeated `-` entries as the same standard-input-only path set.
- In scope: yielding piped Python source with the observable filename `-`, enabling pipelines without temporary files.
- In scope: preserving the compatibility quirk that a path set mixing `-` with file paths does not read standard input.
- Out of scope: file or directory discovery, metric calculations, report formatting, support for languages other than Python, implicit standard-input fallback when no path is supplied, and defining behavior for mixed `-` plus non-file path kinds not assigned by the evidence packet.
- This slice is atomic within Input discovery. It may share delivery seams with adjacent stories, but does not combine or supersede them and retains its own acceptance evidence.

### Evidence Discrepancy Resolution

No material conflict was found. The Azure acceptance criterion says that a path set only containing `-` reads standard input and yields filename `-`. The pinned source and test evidence refine this with two observable compatibility decisions: repeated `-` entries are accepted because multiplicity is ignored, and a path set that mixes `-` with files does not read standard input. These sources are complementary, so the specification adopts all of them without expanding scope. Tracker wording is retained as lower-precedence traceability confirmation.

## User Scenarios & Testing

### User Story 1 - Analyze Piped Source (Priority: P1)

As a developer, I want to pipe Python source to the analyser using `-` so that I can analyze temporary source without creating a file.

**Why this priority**: This is the story's direct user value and required observable compatibility behavior.

**Independent Test**: Pipe valid Python source to a metric command while supplying only `-`; verify that it analyzes the piped source and identifies the resulting input as `-`.

**Acceptance Scenarios**:

1. **Given** valid Python source is available through standard input and the supplied path set is one `-`, **When** the developer invokes a metric command, **Then** discovery reads the supplied standard input and yields it under filename `-`.
2. **Given** valid Python source is available through standard input and the supplied path set contains repeated `-` entries only, **When** the developer invokes a metric command, **Then** discovery reads standard input once and yields one input under filename `-`.

---

### User Story 2 - Keep Standard Input Explicit (Priority: P2)

As a developer, I want standard input to be used only for the dedicated `-` path set so that adding file inputs never consumes my pipeline unexpectedly.

**Why this priority**: The sentinel boundary and its mixed-input quirk are observable compatibility behavior that protects predictable command use.

**Independent Test**: Provide standard input and invoke a metric command with `-` plus a file path; verify that discovery does not read standard input. Compare the outcome with the pinned reference.

**Acceptance Scenarios**:

1. **Given** Python source is available through standard input and the supplied path set contains `-` plus a file path, **When** the developer invokes a metric command, **Then** discovery does not read standard input.
2. **Given** a supplied path set that is not composed only of `-`, **When** the developer invokes a metric command, **Then** the command does not treat any `-` entry as a standard-input request.

### Edge Cases

- Repeated `-` entries are accepted as a standard-input-only path set; their multiplicity has no effect and standard input is read once.
- A path set mixing `-` with one or more file paths does not read standard input, even if piped source is available.
- No supplied path is not a standard-input request; this slice does not introduce implicit standard-input fallback.
- Standard-input behavior is limited to Python source in this initial compatibility scope; other languages remain future scope.

## Requirements

### Functional Requirements

- **FR-001**: When the complete supplied path set consists only of `-`, the analyser MUST read the available standard input.
- **FR-002**: For an FR-001 standard-input read, discovery MUST yield the input under the filename `-`.
- **FR-003**: A supplied path set containing one or more `-` entries and no other path values MUST be treated as the same standard-input-only request, regardless of the number of repeated `-` entries.
- **FR-004**: For an FR-003 request, the analyser MUST read standard input exactly once and yield exactly one input under filename `-`.
- **FR-005**: When a supplied path set includes `-` and at least one file path, the analyser MUST NOT read standard input.
- **FR-006**: The analyser MUST NOT infer a standard-input request when the supplied path set is empty.
- **FR-007**: This feature MUST preserve the pinned reference's observable input identity, results, diagnostics, and failure behavior for its documented acceptance scenarios.
- **FR-008**: This feature MUST support Python source only; behavior for other languages is outside MET-007.

### Acceptance Mapping

| Requirement | Independently verifiable acceptance evidence |
| --- | --- |
| FR-001, FR-002 | User Story 1 scenario 1; Azure acceptance criterion; pinned standard-input test evidence |
| FR-003, FR-004 | User Story 1 scenario 2; pinned sentinel-handling evidence |
| FR-005 | User Story 2 scenarios 1–2; pinned sentinel-handling evidence |
| FR-006 | Edge Cases; scope review against the absence of an implicit fallback requirement |
| FR-007 | Comparison of all documented scenarios against the pinned reference links |
| FR-008 | Stakeholder direction, Constitution Principle II, and Python-source scenario inputs |

### Key Entities

- **Supplied path set**: The complete collection of positional analysis-path values supplied in one invocation; standard input is requested only when this set contains `-` and no other value.
- **Standard-input token**: The literal supplied path value `-`.
- **Discovered input**: The source made available for analysis, including its observable filename; standard input is identified as `-`.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Across the documented standard-input acceptance runs, 100% of invocations whose supplied path set is only `-` read the piped Python source and yield one input named `-`.
- **SC-002**: Across the documented repeated-token acceptance runs, 100% of invocations with two or more `-` entries and no other path yield one input named `-` and consume standard input once.
- **SC-003**: Across the documented mixed-path acceptance runs, 100% of invocations containing `-` plus a file path do not read standard input.
- **SC-004**: For 100% of documented acceptance scenarios, the observable result, input identity, diagnostics, and failure behavior match the pinned compatibility reference.

## Assumptions and Dependencies

- Standard input is available to the command for scenarios that explicitly pipe source; the supplied authority defines no alternate behavior when it is unavailable.
- The scope is restricted to discovery behavior for Python source; metric interpretation and presentation are established by other compatibility slices.
- The pinned Radon commit is the reproducible compatibility reference for acceptance comparison.
- Repository readiness is satisfied by the available `rusty-kode` product repository; delivery is governed by the dependency sequence recorded below.
