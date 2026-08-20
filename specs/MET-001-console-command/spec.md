# Feature Specification: MET-001 Console Command

**Feature Branch**: `Not created (no before-specify branch hook is registered)`

**Created**: 2026-08-19

**Status**: Ready for planning — repository availability gate cleared on 2026-08-20

**Input**: Azure story MET-001, work item 241: provide an installed console command that starts the analyzer and dispatches to the selected analysis subcommand.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Invoke an Analysis from the Shell (Priority: P1)

As a developer with the product installed, I can invoke `radon` from a shell with an explicitly selected supported analysis subcommand so that I can start source-code analysis from command-line workflows.

**Why this priority**: A stable installed command is the entrypoint through which shell users and automation reach every command-line analysis capability. This story is the complete and only independently testable slice in MET-001.

**Independent Test**: Install a release candidate in a clean supported environment, invoke `radon` with each registered analysis subcommand using an input accepted by that subcommand, and verify that every invocation reaches the selected subcommand. Metric calculations and report contents are verified by the separate story that owns each subcommand.

**Acceptance Scenarios**:

1. **Given** the product has been installed successfully in a clean supported environment and its installed commands are available to the shell, **When** a developer invokes `radon` with a registered analysis subcommand and valid arguments for that subcommand, **Then** the command starts the analyzer and dispatches the invocation to that selected subcommand.
2. **Given** the same installed product, **When** the developer invokes each registered analysis subcommand through `radon` in turn, **Then** each invocation reaches the subcommand named by the developer without being redirected to a different analysis subcommand.

### Edge Cases

- The command is invoked from a working directory other than the installation directory; it remains available wherever installed commands are normally exposed to that shell.
- A registered subcommand rejects its arguments or its analysis fails after dispatch; the invocation still satisfies this feature only if control reached the selected subcommand. Argument validation, analysis failures, output, and exit behavior belong to the stories that define those behaviors.
- Invocation without an explicit subcommand is outside MET-001; empty-invocation help behavior belongs to MET-003.
- Alternate runtime/module invocation is outside MET-001 and belongs to MET-002.
- Version reporting, uncaught-error presentation, discovery, analysis semantics, and output formatting are outside MET-001.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: A successful product installation MUST expose a shell-invocable command named `radon`.
- **FR-002**: Invoking `radon` MUST start the product's command-line analyzer.
- **FR-003**: When an invocation names a registered analysis subcommand, `radon` MUST dispatch control to exactly that selected subcommand.
- **FR-004**: The `radon` command MUST be usable from shell working directories other than the product's installation directory when the environment exposes installed commands in its normal manner.
- **FR-005**: Acceptance evidence MUST demonstrate installation followed by invocation and dispatch; the pinned reference's packaging declaration alone MUST NOT be treated as proof that the installed command works.
- **FR-006**: MET-001 verification MUST stop at successful dispatch. It MUST NOT redefine the supported arguments, calculations, reports, failures, or exit semantics owned by other MET stories.

### Requirement Acceptance Mapping

| Requirement | Acceptance verification |
|---|---|
| FR-001 | Acceptance Scenario 1 verifies that a clean successful installation exposes `radon` to the shell. |
| FR-002 | Acceptance Scenario 1 verifies that invoking `radon` starts the command-line analyzer. |
| FR-003 | Acceptance Scenarios 1 and 2 verify correct selected-subcommand dispatch, including the complete registered-subcommand matrix. |
| FR-004 | The first edge case repeats Acceptance Scenario 1 from a working directory other than the installation directory. |
| FR-005 | The Independent Test requires a clean installation followed by actual invocation and dispatch evidence. |
| FR-006 | The second through fifth edge cases verify the MET-001 boundary and defer owned behavior to its separate stories. |

### Key Entities

- **Installed command**: The user-visible `radon` name made available to a shell by a successful installation.
- **Analysis subcommand selection**: The registered analysis operation explicitly named in an invocation; MET-001 verifies that the entrypoint routes to it, not what it calculates or reports.
- **Dispatch evidence**: A reproducible record tying the installed invocation to the selected subcommand being reached.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of clean supported-environment acceptance runs, a successful installation makes `radon` invocable through the shell's normal installed-command lookup.
- **SC-002**: In a test matrix containing every registered analysis subcommand, 100% of valid invocations through `radon` reach the subcommand explicitly selected by the user.
- **SC-003**: A developer can begin a supported analysis from a shell with one `radon <subcommand> ...` invocation after installation, without navigating to the installation directory.
- **SC-004**: Acceptance evidence for MET-001 contains zero failures attributable to redirection to an unselected analysis subcommand.

## Assumptions

- Installation itself has completed successfully; installer selection, installation failures, and environment command-path configuration are outside this story.
- The set and detailed behavior of registered analysis subcommands are defined by their owning MET stories. MET-001 tests all subcommands registered in the release under test without independently prescribing that set.
- The exact command name `radon` is observable compatibility behavior established by the enriched Azure story and both pinned packaging declarations.
- A clean supported environment is one declared supported by the eventual product release and configured to expose installed commands through its normal shell lookup.
- No response-time target is inferred because neither the Azure story nor pinned evidence establishes one; completeness and correct dispatch provide measurable outcomes without inventing unsupported behavior.

## Scope Boundaries

**In scope**:

- Availability of the installed `radon` shell command.
- Starting the command-line analyzer through that command.
- Correct dispatch to an explicitly selected registered analysis subcommand.
- Installation-level acceptance evidence for this entrypoint.

**Out of scope**:

- Empty invocation and help behavior (MET-003).
- Alternate runtime/module invocation (MET-002).
- Version reporting (MET-004) and top-level error presentation (MET-005).
- Definition of subcommands, their arguments, analysis semantics, outputs, errors, or exit statuses.
- A prescribed package manifest, build system, programming construct, module layout, or dispatch mechanism.
- Languages other than Python in the initial compatibility slices.

## Evidence and Traceability

### Azure Story

- **Story**: MET-001 — Console command
- **Azure work item**: [241](https://dev.azure.com/yvrkarthik/code-metrics/_workitems/edit/241)
- **Canonical user need**: A developer invokes the analyzer from a command line to analyze source code from a shell.
- **Canonical acceptance criterion**: The installed command starts the CLI and dispatches to the selected analysis subcommand.
- **Requirement mapping**: Azure acceptance is expressed by FR-001 through FR-003 and Acceptance Scenarios 1–2; independent installation evidence is strengthened by FR-005 because the reference has no focused installation test.

### Pinned Compatibility Reference

- **Reference commit**: [Radon `54b88e5878b2724bf4d77f97349588b811abdff2`](https://github.com/rubik/radon/tree/54b88e5878b2724bf4d77f97349588b811abdff2)
- **Entrypoint declaration**: [`setup.py` lines 32–39](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/setup.py#L32-L39)
- **Second packaging declaration**: [`pyproject.toml` lines 29–34](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/pyproject.toml#L29-L34)
- **Command registration context**: [`radon/cli/__init__.py` lines 90–319](https://github.com/rubik/radon/blob/54b88e5878b2724bf4d77f97349588b811abdff2/radon/cli/__init__.py#L90-L319)

### Discrepancies and Resolutions

- **Two packaging descriptions**: The pinned reference declares packaging in both `setup.py` and `pyproject.toml`, while the research guide recommends one canonical product manifest. Under the evidence precedence rule, both pinned declarations corroborate the same observable `radon` command; their internal duplication does not change Azure acceptance. Resolution: FR-001 preserves the command, FR-005 closes the reference's missing focused installation-test gap, and manifest choice remains outside this technology-agnostic specification.
- **Tracker phrasing versus observable outcome**: Reference wording says the command calls `radon:main`, which is an internal Python target. The Azure criterion requires starting the CLI and dispatching to the selected subcommand. Resolution: FR-002 and FR-003 preserve the observable outcome without requiring the reference's internal call target.
- **Cross-cutting adjacent behaviors**: The guide identifies shared entrypoint and command wiring, but MET-002 through MET-005 separately own alternate invocation, empty invocation, version, and top-level error behavior. Resolution: those behaviors are explicit exclusions so MET-001 remains one independently testable specification.

## Active Blocker and Dependencies

**Cleared Dependency Gate**: The repository-availability blocker was revalidated and cleared on 2026-08-20. The required conditions were evaluated as follows:

1. The stakeholder confirmed that the practical repository-setup outcome governed by SETUP-001 is satisfied. Synchronizing Azure DevOps work item 343 remains pending because the Azure DevOps MCP connection is unavailable.
2. This `rusty-kode` repository is the separately available local Rust product repository.
3. The revalidation evidence and clearing decision are recorded in [`docs/specs/MET-001-blocker-clearance.md`](../../docs/specs/MET-001-blocker-clearance.md).

The `code-metrics-specs` directory is specification-only. It is not, and MUST NOT be used or represented as, the local Rust product repository.

**Dependencies**:

- [SETUP-001 Azure work item 343](https://dev.azure.com/yvrkarthik/code-metrics/_workitems/edit/343), including traceable evidence of satisfaction.
- A separately available local Rust product repository suitable for planning and implementation.
- A recorded revalidation result linking work item 343, the available repository, the evidence reviewed, and the decision that clears or retains the gate.
- The owning MET specifications for the registered analysis subcommands used in end-to-end dispatch verification.
- Constitution v1.1.0 remains the controlling governance source.

Repository delivery alone did not clear this blocker. The linked revalidation and stakeholder-authorized clearing decision satisfy the gate, so `$speckit-plan`, `$speckit-tasks`, and `$speckit-implement` MAY proceed for MET-001. Azure DevOps synchronization remains a traceability follow-up and does not reverse the recorded decision.
