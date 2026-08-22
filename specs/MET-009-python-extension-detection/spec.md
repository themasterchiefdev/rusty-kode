# Feature Specification: MET-009 Python Extension Detection

**Feature ID**: `MET-009`  
**Azure work item**: [249 — MET-009: Python extension detection](https://dev.azure.com/yvrkarthik/code-metrics/_workitems/edit/249)  
**Feature group**: Input discovery  
**Created**: 2026-08-19  
**Compatibility scope**: Observable Python-source input-eligibility behavior only; this is one independently testable compatibility slice.

## Repository Readiness and Implementation Sequencing

**Specification status**: READY — repository gate cleared; implementation sequencing defined.

**Repository status**: READY. SETUP-001 / Azure work item 343 is closed, and the local Rust product repository `rusty-kode` is available. The former repository-availability blocker is cleared and does not restrict planning, task generation, or implementation.

**Implementation wave**: 1
**Canonical sequence**: 4 of 102
**Hard prerequisites**: None; this story may start after the repository baseline.

Only the prerequisites listed above constrain implementation order. References elsewhere to adjacent, output-owning, or verification-owning MET stories are scope boundaries, not implicit blockers. This story must prove its own acceptance scenarios without requiring unfinished downstream stories.

## Context, Authority, and Traceability

This feature lets a developer have a filename recognized automatically as eligible Python source when, and only when, the filename ends exactly in `.py`. It preserves the reference behavior at the input-discovery boundary before any analysis occurs.

Requirements apply the supplied evidence in this order: stakeholder direction; the enriched Azure story and acceptance criteria; pinned Radon source and tests; then tracker wording. The compatibility reference is pinned to [Radon commit `54b88e5878b2724bf4d77f97349588b811abdff2`](https://github.com/rubik/radon/tree/54b88e5878b2724bf4d77f97349588b811abdff2), with behavior evidence in [`predicate`](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/tools.py#L226-L241) and [`discovery test`](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/tests/test_cli_tools.py#L78-L143).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Recognize Python File Names (Priority: P1)

As a developer, I want filenames ending exactly in `.py` recognized as eligible Python source so that they enter Python input discovery automatically.

**Why this priority**: This is the complete Azure work item acceptance behavior and the entry condition for later Python analysis.

**Independent Test**: Evaluate filename eligibility with `module.py`, `module.py.bak`, `module.PY`, and a filename without the `.py` ending; verify that only `module.py` is eligible Python source.

**Acceptance Scenarios**:

1. **Given** a filename ending exactly in lowercase `.py`, **When** Python-source eligibility is evaluated, **Then** the filename is accepted as Python source.
2. **Given** a filename ending in uppercase `.PY`, **When** Python-source eligibility is evaluated, **Then** the filename is not accepted as Python source.
3. **Given** a filename that contains `.py` but does not end exactly in `.py`, **When** Python-source eligibility is evaluated, **Then** the filename is not accepted as Python source.
4. **Given** a filename without a `.py` ending, **When** Python-source eligibility is evaluated, **Then** the filename is not accepted as Python source.

### Edge Cases

- The matching ending is exactly lowercase `.py`; the case-distinct ending `.PY` is intentionally not recognized.
- A filename with additional characters after `.py` is not eligible because it does not end exactly in `.py`.
- A filename containing `.py` in another position is not eligible unless its final characters are exactly `.py`.
- This feature decides only Python-source eligibility; it does not define directory traversal, direct-input handling, analysis, metrics, reports, or errors outside this predicate.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST accept a filename as eligible Python source when its final characters are exactly lowercase `.py`.
- **FR-002**: The system MUST NOT accept a filename as eligible Python source when the filename does not end exactly in lowercase `.py`.
- **FR-003**: The system MUST treat extension matching as case-sensitive; a filename ending in `.PY` MUST NOT be accepted as Python source.
- **FR-004**: This feature MUST apply only to Python-source eligibility and MUST NOT add support for recognizing or analyzing other languages.

### Acceptance Mapping

| Requirement | Acceptance evidence | Authority |
|---|---|---|
| FR-001 | User Story 1 scenario 1 | Azure work item 249 acceptance criteria; pinned [`predicate`](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/tools.py#L226-L241) and [`discovery test`](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/tests/test_cli_tools.py#L78-L143) |
| FR-002 | User Story 1 scenarios 3 and 4 | Enriched Azure story feature explanation; pinned predicate and discovery test |
| FR-003 | User Story 1 scenario 2; Edge Cases | Enriched Azure story code-grounded consideration; pinned predicate and discovery test |
| FR-004 | Functional requirement test; scope review | Stakeholder direction; Constitution II |

### Key Entities

- **Filename**: The name evaluated at the Python-source eligibility boundary.
- **Eligible Python source**: A filename accepted by this feature because it ends exactly in lowercase `.py`.
- **Ineligible filename**: A filename not accepted by this feature because it lacks that exact lowercase ending, including an uppercase `.PY` ending.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In an acceptance set containing one filename ending exactly in lowercase `.py`, that filename is accepted in 1 of 1 evaluations.
- **SC-002**: In an acceptance set containing filenames ending in `.PY`, containing `.py` without ending in it, and lacking `.py`, 0 of 3 such filenames are accepted.
- **SC-003**: All four acceptance scenarios pass using the pinned compatibility evidence as the expected behavior.
- **SC-004**: The MET-009 acceptance evidence remains independently traceable to Azure work item 249 and its supplied reference links.

## Scope Boundaries

**In scope**:

- Whether a filename ending exactly in lowercase `.py` is eligible Python source.
- The observable case-sensitive exclusion of filenames ending in `.PY`.

**Out of scope**:

- Directory traversal and file discovery beyond this eligibility decision.
- Direct-input handling, analysis, metrics, reports, output, or error behavior.
- Recognition or analysis of non-Python languages.
- Any filename rule other than the exact `.py` ending stated in the supplied evidence.

## Evidence Discrepancies and Resolutions

No material conflict exists among the supplied authorities. The enriched Azure story and acceptance criterion require acceptance of filenames ending in `.py`; the pinned reference evidence confirms the exact predicate behavior. The enriched story further resolves the otherwise plausible case-normalization alternative: matching is case-sensitive and `.PY` is not recognized. This specification preserves that observable compatibility quirk under Constitution VIII in FR-003 and User Story 1 scenario 2. No stakeholder question is needed because the evidence packet resolves the material behavior.

## Assumptions

- “Filename” is evaluated as supplied; this specification makes no normalization or transformation guarantee because the evidence defines the ending predicate only.
- The supplied evidence defines eligibility, not discovery order, path traversal, or post-eligibility behavior; those concerns remain outside this atomic story.
- Python source analysis is the initial and only supported language scope; other languages remain future scope under Constitution II.
