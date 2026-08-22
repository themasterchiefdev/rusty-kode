# Feature Specification: MET-010 Python shebang detection

**Feature ID**: `MET-010`
**Azure Work Item**: 250 — MET-010: Python shebang detection
**Source Attachment SHA-256**: `53afca0493f8e02515308ee671973acf1b8feb773b1187ffac5953828a56f440`; private tracker URLs omitted by stakeholder direction
**Created**: 2026-08-19

**Compatibility reference**: [Radon predicate at pinned revision `54b88e5878b2724bf4d77f97349588b811abdff2`](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/tools.py#L226-L241)

## Repository Readiness and Implementation Sequencing

**Specification status**: READY — repository gate cleared; implementation sequencing defined.

**Repository status**: READY. SETUP-001 / Azure work item 343 is closed, and the local Rust product repository `rusty-kode` is available. The former repository-availability blocker is cleared and does not restrict planning, task generation, or implementation.

**Implementation wave**: 1
**Canonical sequence**: 5 of 102
**Hard prerequisites**: None; this story may start after the repository baseline.

Only the prerequisites listed above constrain implementation order. References elsewhere to adjacent, output-owning, or verification-owning MET stories are scope boundaries, not implicit blockers. This story must prove its own acceptance scenarios without requiring unfinished downstream stories.

## Context and Scope

Developers need executable Python scripts that do not use a `.py` filename extension to be included in Python source analysis. This atomic Input discovery slice defines only the eligibility decision for a non-`.py` candidate based on its first line. It preserves observable Radon-compatible behavior for Python analysis; other languages remain future scope.

**In scope**: deciding whether a non-`.py` candidate is eligible from its first line.
**Out of scope**: `.py` extension eligibility, directory traversal, metric calculation, report presentation, non-Python language detection, and any change to later analysis after a candidate is eligible.

## Evidence and Decision Record

| Decision | Evidence and precedence | Resolution |
|---|---|---|
| Story identity and user need | Azure work item 250, enriched story | Preserve MET-010 as an independent Input discovery specification for executable Python scripts without `.py` extensions. |
| Eligibility predicate | Enriched acceptance criterion, then [pinned Radon predicate](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/tools.py#L226-L241) | A non-`.py` candidate is eligible only when its first line begins exactly with `#!` and contains the literal lowercase substring `python`. |
| Case semantics discrepancy | The Azure acceptance criterion says `python`; the enriched description expressly says lowercase; the pinned reference evidence notes exact case-sensitive substring semantics. | Interpret `python` as a case-sensitive lowercase substring. Thus `Python` alone does not qualify, while `python3` qualifies. This records, rather than silently resolves, the broader tracker wording. |
| Unreadable input quirk | [Pinned Radon predicate](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/tools.py#L226-L241) and supplied code-grounded consideration | If the first line cannot be read as ordinary text, the candidate is not eligible; it does not produce eligibility through this rule. |

## User Scenarios & Testing

### User Story 1 - Discover executable Python scripts (Priority: P1)

As a developer, I want an executable Python script without a `.py` extension to be accepted for Python source analysis when its shebang identifies Python, so that valid scripts are not omitted solely because of their filename.

**Why this priority**: This is the MET-010 compatibility behavior and independently changes the analysis boundary for a common executable-script form.

**Independent Test**: Evaluate non-`.py` candidate files using only their first line and confirm the resulting eligible/not-eligible decision against the scenarios below; no metric calculation or report is required.

**Acceptance Scenarios**:

1. **Given** a non-`.py` candidate whose first line is `#!/usr/bin/env python`, **When** eligibility is evaluated, **Then** the candidate is accepted for Python source analysis. *(Azure 250 acceptance mapped to FR-001 and FR-002.)*
2. **Given** a non-`.py` candidate whose first line is `#!/usr/bin/python3`, **When** eligibility is evaluated, **Then** the candidate is accepted because the line contains the lowercase substring `python`. *(Case/subsequence resolution mapped to FR-002.)*
3. **Given** a non-`.py` candidate whose first line is `#!/usr/bin/env Python`, **When** eligibility is evaluated, **Then** the candidate is not accepted by this shebang rule. *(Case-sensitive compatibility resolution mapped to FR-003.)*
4. **Given** a non-`.py` candidate whose first line contains `python` but does not begin with `#!`, **When** eligibility is evaluated, **Then** the candidate is not accepted by this shebang rule. *(Azure 250 acceptance mapped to FR-001.)*
5. **Given** a non-`.py` candidate whose first line begins `#!` but does not contain lowercase `python`, **When** eligibility is evaluated, **Then** the candidate is not accepted by this shebang rule. *(Azure 250 acceptance mapped to FR-002.)*
6. **Given** a non-`.py` candidate whose first line cannot be read as ordinary text, **When** eligibility is evaluated, **Then** the candidate is not accepted by this shebang rule. *(Pinned-reference quirk mapped to FR-004.)*

### Edge Cases

- An empty candidate, or one with a first line that is empty, is not accepted by this shebang rule.
- A later line containing `#!` or lowercase `python` does not affect the decision.
- A first line beginning with whitespace before `#!` does not satisfy the requirement that the line begins with `#!`.
- A `.py` candidate is outside this rule; its eligibility is not changed by MET-010.

## Requirements

### Functional Requirements

- **FR-001**: The product MUST accept a non-`.py` candidate for Python source analysis through MET-010 only when the candidate's first line begins exactly with `#!`.
- **FR-002**: The product MUST accept that candidate through MET-010 only when the same first line contains the literal lowercase substring `python`; the substring may be part of a longer token, such as `python3`.
- **FR-003**: The product MUST treat the `python` substring check as case-sensitive; a line containing `Python` but no lowercase `python` MUST not be accepted through MET-010.
- **FR-004**: The product MUST treat a non-`.py` candidate whose first line cannot be read as ordinary text as not accepted through MET-010.
- **FR-005**: The product MUST base the MET-010 decision on the first line only; later content MUST not make a candidate eligible.
- **FR-006**: The product MUST leave `.py` extension eligibility and all non-Python language behavior outside MET-010.
- **FR-007**: The product MUST retain MET-010 evidence traceability to Azure work item 250 and the pinned Radon predicate for every acceptance verification of this slice.

### Key Entities

- **Candidate file**: A file presented for input discovery; for MET-010, it is a non-`.py` candidate evaluated only by its first line.
- **First-line shebang evidence**: The first line's exact starting characters, lowercase `python` substring presence, and readability, which together determine MET-010 eligibility.
- **Eligibility decision**: The independently verifiable accepted/not-accepted outcome of this MET-010 rule; it does not represent metric analysis or results.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All six acceptance scenarios for MET-010 produce their stated eligibility decision in repeatable verification.
- **SC-002**: A verification set containing at least one accepted non-`.py` Python shebang and one rejected case for each condition in FR-001 through FR-005 has 100% expected-result agreement.
- **SC-003**: Each MET-010 acceptance verification records links to Azure work item 250 and the pinned Radon predicate.
- **SC-004**: No MET-010 verification accepts a non-Python or `.py` eligibility case solely because of this shebang rule.

## Assumptions and Dependencies

- This specification uses only the supplied enriched MET-010 evidence packet and constitution v1.1.0 as authority; no unprovided product-repository behavior is assumed.
- “Contains `python`” is a literal, case-sensitive substring condition as resolved in the Evidence and Decision Record; no interpreter-path normalization or language inference is implied.
- MET-010 is an atomic compatibility requirement within Input discovery and may share later seams with adjacent stories without combining their acceptance evidence.
- The pinned Radon revision is the compatibility reference for this slice; observable behavior, including the unreadable-input rejection quirk, takes precedence over a more permissive interpretation.

## Constitution Compliance Review

- Behavioral compatibility is defined as observable eligibility, rejection, and unreadable-input behavior, not a literal translation.
- The scope is Python-first; non-Python support is expressly excluded.
- MET-010 remains independently testable and traceable to Azure work item 250.
- Evidence precedence and the lowercase-case discrepancy resolution are recorded above.
- The specification is technology-agnostic and leaves architecture decisions outside this feature slice.
- Repository readiness is satisfied by the available `rusty-kode` product repository; delivery is governed by the dependency sequence recorded below.
