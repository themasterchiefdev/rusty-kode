# Feature Specification: MET-007 Standard Input

**Feature ID**: `MET-007`  
**Azure Work Item**: 247 — MET-007: Standard input
**Feature Group**: Input discovery  
**Target Surface**: CLI input-discovery and consumer seam
**Created**: 2026-08-19  

## Repository Readiness and Implementation Sequencing

**Specification status**: READY — repository gate cleared; implementation sequencing and acceptance boundary clarified.

**Repository status**: READY. SETUP-001 / Azure work item 343 is closed, and the local Rust product repository `rusty-kode` is available. The former repository-availability blocker is cleared and does not restrict planning, task generation, or implementation.

**Implementation wave**: 1
**Canonical sequence**: 3 of 102
**Hard prerequisites**: None; this story may start after the repository baseline.

Only the prerequisites listed above constrain implementation order. References elsewhere to adjacent, output-owning, or verification-owning MET stories are scope boundaries, not implicit blockers. This story must prove its own acceptance scenarios without requiring unfinished downstream stories.

## Context, Authority, and Compatibility Scope

This independently testable compatibility slice lets a developer pipe Python source into the analyser without creating a temporary file. When the complete supplied path set is the standard-input token `-`, discovery yields the piped source under the filename `-`. The slice preserves externally observable behavior of the pinned Radon reference and does not prescribe implementation design.

**Approved acceptance-boundary clarification (2026-08-22)**: MET-007 is accepted at the input-discovery API and an injectable `MetricInput` consumer seam. Acceptance proves that the complete discovered standard-input payload or delegated path collection reaches that seam unchanged. A real metric analyser, user-facing output, persistence, diagnostics owned by metric execution, and payload-sensitive built-binary acceptance are deferred to the later metric-analysis stories that own those behaviors. MET-007 MUST NOT invent those downstream behaviors merely to make its handoff observable.

**Authority order applied**: stakeholder direction; enriched Azure story and acceptance criteria; pinned Radon source and test evidence; tracker wording. The authoritative evidence packet is [MET-007](../../intake/stories/MET-007.md); the product governance authority is [Constitution v1.1.0](../../.specify/memory/constitution.md).

**Pinned compatibility reference**: [Radon at commit `54b88e5878b2724bf4d77f97349588b811abdff2`](https://github.com/rubik/radon/tree/54b88e5878b2724bf4d77f97349588b811abdff2), with [discovery/open evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/tools.py#L214-L224), [sentinel-handling evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/tools.py#L244-L269), and [standard-input test evidence](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/tests/test_cli_tools.py#L40-L75).

### Scope Boundaries

- In scope: recognizing standard input only when the complete supplied path set contains `-` and nothing else.
- In scope: treating one or more repeated `-` entries as the same standard-input-only path set.
- In scope: yielding piped Python source with the observable filename `-`, enabling pipelines without temporary files.
- In scope: preserving the compatibility quirk that a path set mixing `-` with file paths does not read standard input.
- In scope: handing the selected `MetricInput` to an injectable consumer seam without losing source identity, source content, path order, or path multiplicity.
- Out of scope: a real metric consumer, built-binary payload observability, user-facing output, persistence, metric-owned diagnostics, file or directory discovery, metric calculations, report formatting, support for languages other than Python, implicit standard-input fallback when no path is supplied, and defining behavior for mixed `-` plus non-file path kinds not assigned by the evidence packet.
- This slice is atomic within Input discovery. It may share delivery seams with adjacent stories, but does not combine or supersede them and retains its own acceptance evidence.

### Evidence Discrepancy Resolution

No material conflict was found. The Azure acceptance criterion says that a path set only containing `-` reads standard input and yields filename `-`. The pinned source and test evidence refine this with two observable compatibility decisions: repeated `-` entries are accepted because multiplicity is ignored, and a path set that mixes `-` with files does not read standard input. These sources are complementary, so the specification adopts all of them without expanding scope. Tracker wording is retained as lower-precedence traceability confirmation.

## User Scenarios & Testing

### User Story 1 - Analyze Piped Source (Priority: P1)

As a developer, I want to pipe Python source to the analyser using `-` so that I can analyze temporary source without creating a file.

**Why this priority**: This is the story's direct user value and required observable compatibility behavior.

**Independent Test**: Supply valid Python source and only `-` through the input-discovery API; verify that the injectable consumer seam receives the complete source once with identity `-`.

**Acceptance Scenarios**:

1. **Given** valid Python source is available through standard input and the supplied path set is one `-`, **When** the input-discovery seam runs, **Then** it reads the supplied standard input and hands the complete source to the injectable consumer once under filename `-`.
2. **Given** valid Python source is available through standard input and the supplied path set contains repeated `-` entries only, **When** the input-discovery seam runs, **Then** it reads standard input once and hands one complete input to the injectable consumer under filename `-`.

---

### User Story 2 - Keep Standard Input Explicit (Priority: P2)

As a developer, I want standard input to be used only for the dedicated `-` path set so that adding file inputs never consumes my pipeline unexpectedly.

**Why this priority**: The sentinel boundary and its mixed-input quirk are observable compatibility behavior that protects predictable command use.

**Independent Test**: Provide standard input and call the input-discovery API with `-` plus a file path; verify that it does not read standard input and that the injectable consumer receives the complete ordered path collection unchanged. Compare the discovery outcome with the pinned reference.

**Acceptance Scenarios**:

1. **Given** Python source is available through standard input and the supplied path set contains `-` plus a file path, **When** the input-discovery seam runs, **Then** it does not read standard input and hands the complete ordered path collection to the injectable consumer unchanged.
2. **Given** a supplied path set that is not composed only of `-`, **When** the input-discovery seam runs, **Then** it does not treat any `-` entry as a standard-input request and preserves the complete path collection for the injectable consumer.

### Edge Cases

- Repeated `-` entries are accepted as a standard-input-only path set; their multiplicity has no effect and standard input is read once.
- A path set mixing `-` with one or more file paths does not read standard input, even if piped source is available.
- No supplied path is not a standard-input request; this slice does not introduce implicit standard-input fallback.
- Standard-input behavior is limited to Python source in this initial compatibility scope; other languages remain future scope.

## Requirements

### Functional Requirements

- **FR-001**: When the complete supplied path set consists only of `-`, the input-discovery seam MUST read the available standard input.
- **FR-002**: For an FR-001 standard-input read, discovery MUST yield the input under the filename `-`.
- **FR-003**: A supplied path set containing one or more `-` entries and no other path values MUST be treated as the same standard-input-only request, regardless of the number of repeated `-` entries.
- **FR-004**: For an FR-003 request, the input-discovery seam MUST read standard input exactly once and hand exactly one complete input to the injectable consumer under filename `-`.
- **FR-005**: When a supplied path set includes `-` and at least one file path, the input-discovery seam MUST NOT read standard input and MUST hand the complete ordered path collection to the injectable consumer unchanged.
- **FR-006**: The input-discovery seam MUST NOT infer a standard-input request when the supplied path set is empty and MUST preserve the empty collection for the injectable consumer.
- **FR-007**: Within MET-007's clarified acceptance boundary, this feature MUST preserve the pinned reference's observable input identity, source content, path collection, read behavior, and discovery failures. Metric results and metric-owned diagnostics are outside this feature.
- **FR-008**: This feature MUST support Python source only; behavior for other languages is outside MET-007.
- **FR-009**: Acceptance MUST be demonstrated through the input-discovery API and injectable `MetricInput` consumer seam; MET-007 MUST NOT require or invent a production metric consumer or payload-sensitive built-binary output.

### Acceptance Mapping

| Requirement | Independently verifiable acceptance evidence |
| --- | --- |
| FR-001, FR-002 | User Story 1 scenario 1 through the injectable consumer seam; Azure acceptance criterion; pinned standard-input test evidence |
| FR-003, FR-004 | User Story 1 scenario 2 through the injectable consumer seam; pinned sentinel-handling evidence |
| FR-005 | User Story 2 scenarios 1–2 through the injectable consumer seam; pinned sentinel-handling evidence |
| FR-006 | Edge Cases; scope review against the absence of an implicit fallback requirement |
| FR-007 | Comparison of discovery-boundary observations for all documented scenarios against the pinned reference links |
| FR-008 | Stakeholder direction, Constitution Principle II, and Python-source scenario inputs |
| FR-009 | Approved acceptance-boundary clarification dated 2026-08-22 |

### Key Entities

- **Supplied path set**: The complete collection of positional analysis-path values supplied in one invocation; standard input is requested only when this set contains `-` and no other value.
- **Standard-input token**: The literal supplied path value `-`.
- **Discovered input**: The source handed unchanged to the injectable consumer seam, including its observable filename; standard input is identified as `-`.
- **Injectable MetricInput consumer seam**: The MET-007 acceptance boundary that observes the selected standard-input payload or delegated path collection without defining downstream metric behavior.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Across the documented standard-input acceptance runs, 100% of calls whose supplied path set is only `-` read the piped Python source and hand one complete input named `-` to the injectable consumer.
- **SC-002**: Across the documented repeated-token acceptance runs, 100% of calls with two or more `-` entries and no other path hand one complete input named `-` to the injectable consumer and consume standard input once.
- **SC-003**: Across the documented mixed-path acceptance runs, 100% of calls containing `-` plus a file path do not read standard input and hand the complete ordered path collection to the injectable consumer unchanged.
- **SC-004**: For 100% of documented acceptance scenarios, the discovery-boundary input identity, source content, path collection, read behavior, and discovery failure match the pinned compatibility reference.
- **SC-005**: All MET-007 acceptance tests pass without introducing a production metric consumer, user-facing output, persistence, metric-owned diagnostics, or a payload-sensitive built-binary contract.

## Assumptions and Dependencies

- Standard input is available to the command for scenarios that explicitly pipe source; the supplied authority defines no alternate behavior when it is unavailable.
- The scope is restricted to discovery behavior and lossless handoff at the injectable consumer seam for Python source; metric interpretation, production consumption, built-binary payload observability, and presentation are established by later compatibility slices.
- The pinned Radon commit is the reproducible compatibility reference for acceptance comparison.
- Repository readiness is satisfied by the available `rusty-kode` product repository; delivery is governed by the dependency sequence recorded below.
