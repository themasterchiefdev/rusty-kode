<!--
Sync Impact Report
- Version change: 1.0.0 -> 1.1.0
- Modified principles:
  - IX. Specification Quality Gates -> expanded with the repository availability gate
- Added principles:
  - X. Repository Availability Gate (NON-NEGOTIABLE)
- Modified sections:
  - Product and Architecture Constraints
  - Specification and Review Workflow
  - Governance
- Added sections: none
- Removed sections: none
- Follow-up TODOs:
  - Revalidate and clear the repository blocker only after SETUP-001 Azure work item 343 is
    satisfied and a local Rust product repository is available
-->
# Code Metrics Analyzer Constitution

## Core Principles

### I. Behavioral Compatibility, Not Literal Translation
The product MUST be a behavioral port into Rust of the pinned Radon reference. It MUST preserve
externally observable Python-analysis behavior within the defined compatibility scope. The
implementation MUST NOT treat Radon's Python internals as an architecture to translate literally;
equivalent observable results, including required errors and output semantics, are the compatibility
target. This keeps compatibility measurable while allowing an idiomatic, maintainable Rust design.

### II. Python-First, Canonical Initial Slices
Python source analysis MUST be the only initially supported language. MET-001 through MET-102 MUST
define the canonical initial compatibility slices. Support or behavior for other languages MUST be
treated as future scope and MUST NOT be invented, implied, or accepted as part of these slices. This
fixes the initial product boundary and prevents speculative requirements from weakening compatibility.

### III. Extensible Language-Analyzer Boundary
The architecture MUST expose a language-analyzer boundary through which Python analysis is accessed
and future language analyzers can later be added. The boundary MUST support the known Python
requirements without defining speculative semantics for unsupported languages. Extension readiness
is an architectural separation requirement, not authorization to implement non-Python behavior.

### IV. Independent and Traceable MET Stories
Every MET story MUST remain an independent, testable specification. Each specification MUST preserve
its Azure work-item identity and MUST link its acceptance scenarios and resulting evidence to that
identity. Combining stories MUST NOT obscure story-level scope, verification, or acceptance status.
This enables compatibility progress and failures to be audited slice by slice.

### V. Evidence Precedence and Discrepancy Recording
Requirements decisions MUST apply evidence in this order: stakeholder clarification; enriched Azure
story and acceptance criteria; pinned Radon source and tests; tracker wording. When sources disagree,
the discrepancy and its resolution MUST be recorded with the affected requirement. A lower-precedence
source MUST NOT silently override a higher-precedence source, and conflicting evidence MUST NOT be
silently selected or discarded. This makes requirement provenance reviewable and repeatable.

### VI. Technology-Agnostic Feature Specifications
Feature specifications MUST state the user-visible WHAT and WHY and MUST remain technology-agnostic.
They MUST NOT prescribe Rust constructs, module layouts, algorithms, or implementation architecture.
The Rust mandate and cross-cutting architecture constraints belong in this constitution; justified
feature assumptions or later implementation plans MAY carry further technical decisions. This keeps
acceptance criteria stable when implementation details change.

### VII. Testable, Traceable Ambiguity Resolution
Every ambiguity resolution MUST produce a testable decision and MUST cite the evidence that supports
it. Existing evidence MUST be exhausted according to Principle V before asking a stakeholder. When
that evidence supports one safe, non-conflicting answer, the specification author MUST adopt and
record it rather than ask the stakeholder. Stakeholder clarification is required only when remaining
choices materially alter observable behavior or scope and the evidence cannot safely resolve them.

### VIII. Explicit Quirk Compatibility
Documented Radon quirks MUST be preserved whenever the backlog requires compatibility with them.
Specifications MUST describe such quirks as observable behavior and acceptance scenarios MUST verify
them. Any deliberate deviation MUST be explicit, traceable to the authorizing evidence, and scoped to
the affected MET story; cleaner or more intuitive behavior alone is not sufficient justification.

### IX. Specification Quality Gates
A generated specification MUST contain no unresolved clarification markers. It MUST include complete
acceptance scenarios and relevant edge cases, define measurable outcomes, and pass the applicable
Spec Kit requirements checklist. Every MET specification MUST also contain the explicit blocker and
dependency required by Principle X. A failed or incomplete gate MUST block progression until
corrected, ensuring that downstream work starts only from verifiable scope and satisfied dependencies.

### X. Repository Availability Gate (NON-NEGOTIABLE)
As clarified by the stakeholder on 2026-08-19, no local Rust product repository is currently
available. The separate `code-metrics-specs` directory is a specification workspace only and MUST
NOT be described, treated, or evidenced as the product repository. Specification and clarification
work MAY proceed using the pinned Radon reference and enriched Azure stories. Every MET
specification MUST state an explicit blocker and dependency that implementation planning, task
generation, and implementation MUST NOT begin until SETUP-001 Azure work item 343 is satisfied and
a local Rust product repository is available. Repository delivery alone MUST NOT silently clear the
gate: once the repository is supplied, the blocker MUST be revalidated against both conditions and
MAY be cleared only with traceability to the satisfied work item, the available repository, the
revalidation result, and the clearing decision. This prevents specification-only assets from being
mistaken for an implementation target and prevents unauthorised downstream work.

## Product and Architecture Constraints

- Rust is the required implementation language for the behavioral port.
- `code-metrics-specs` is only the specification workspace and MUST NOT be represented as the local
  Rust product repository.
- The absence of a local Rust product repository is a blocking product dependency governed by
  Principle X; the specification workspace MUST NOT be used as a substitute.
- The pinned Radon source and tests are the technical compatibility reference at the evidence level
  established in Principle V; the exact pin MUST be recorded in project planning or dependency
  records so results can be reproduced.
- The initial analyzer MUST accept Python source only and MUST implement compatibility incrementally
  through MET-001 to MET-102.
- The language-analyzer boundary MUST isolate language-specific analysis from consumers while adding
  no contracts, defaults, or claims for languages not yet specified.
- Compatibility claims MUST concern observable inputs, outputs, diagnostics, failure behavior, and
  documented quirks, rather than correspondence between Rust and Python internal structures.

## Specification and Review Workflow

1. Preserve the MET identifier and Azure work-item identity when creating or revising a feature
   specification.
2. Gather and cite available evidence, apply the mandated precedence, and record all material
   discrepancies and resolutions.
3. Express requirements as user-visible behavior and rationale. Move implementation choices to
   assumptions or planning artifacts unless this constitution establishes them globally.
4. Convert every ambiguity resolution, compatibility quirk, and authorized deviation into one or
   more independently verifiable acceptance scenarios.
5. Review scenarios for normal paths, boundary conditions, invalid input, error behavior, and any
   story-specific edge cases supported by evidence.
6. Run and pass the Spec Kit requirements checklist. No specification may advance with unresolved
   clarification markers, missing evidence links, incomplete scenarios, or non-measurable outcomes.
7. In every MET specification, record the explicit SETUP-001 Azure work item 343 and local Rust
   product repository blocker and dependency. Specification and clarification MAY continue, but
   implementation planning, task generation, and implementation MUST remain blocked.
8. When the local Rust product repository is supplied, revalidate both gate conditions and record
   traceability to SETUP-001 Azure work item 343, the repository, the revalidation evidence, and the
   decision before clearing the blocker.
9. During every generated-spec review, explicitly verify compliance with all principles in this
   constitution and record any justified exception as a proposed constitutional amendment.

## Governance

This constitution is the controlling source for product-wide specification and compatibility rules.
Every generated specification MUST be checked against it before approval, planning, or implementation.
Reviewers MUST reject work that violates a principle or bypasses a quality gate.

An amendment requires either a traceable stakeholder clarification or a documented correction to the
evidence base. The amendment record MUST identify the authority, affected principles, downstream
specifications requiring review, and any migration or revalidation work. Amendments MUST use semantic
versioning: MAJOR for incompatible governance changes or principle removals/redefinitions, MINOR for a
new principle or materially expanded obligation, and PATCH for non-semantic clarification or wording
correction. The last-amended date MUST change whenever the constitution content changes; the original
ratification date MUST remain unchanged.

Compliance review MUST confirm story identity, evidence precedence, discrepancy records, observable
compatibility, architecture-boundary discipline, completion of the specification quality gates, and
presence and current status of the repository availability blocker in every MET specification.
Reviewers MUST verify that `code-metrics-specs` has not been represented as the product repository
and that no implementation planning, task generation, or implementation began before the gate was
traceably cleared. Exceptions MUST NOT be granted informally; they require an amendment under this
governance process.

**Version**: 1.1.0 | **Ratified**: 2026-08-19 | **Last Amended**: 2026-08-19
