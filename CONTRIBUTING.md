# Contributing to SutraERP Architecture

This repository stores the architectural specifications, domain models, workflows, governance rules, and planning controls for SutraERP. Since implementation follows the documents in this repository, all contributors must strictly adhere to the following processes and guidelines.

---

## 1. Branch Strategy

We maintain a strict branch segregation model:

- **`main`**: The source of truth. Contains only fully approved, peer-reviewed, and locked architecture. Direct commits are not allowed.
- Architect working branches: active drafting and review preparation by assigned owners.

For hotfixes or specific task groupings, short-lived feature branches can be created using the responsible owner and level identifier, such as `devang/L2-G-review` or `chetan/L2-J-governance`.

---

## 2. Canonical Architecture Hierarchy

All contributions must preserve this hierarchy and terminology.

```text
L0 -> L1 -> L2 -> L3 -> L4 -> L5 -> L6
```

| Level | Name | Definition |
|-------|------|------------|
| L0 | Business Vision | Defines why SutraERP exists, strategic intent, architectural context, and constraints. |
| L1 | Business Domain Architecture | Defines the canonical business domains, domain boundaries, and ownership. |
| L2 | Domain Workflow Architecture | Defines workflows, lifecycles, rules, and dependencies within an individual business domain. |
| L3 | Business Scenario Architecture | Defines independent end-to-end business scenarios that may derive information from one or many L2 workflow documents; L3 numbering is not dependent on L2 numbering. |
| L4 | System Behavior Specification | Defines expected system behavior for approved business scenarios without implementation design. |
| L5 | Solution Architecture | Defines engineering solution architecture needed to implement L4 behavior without changing business rules. |
| L6 | UI / UX Architecture | Defines user interaction, navigation, screens, and user journeys derived from L5 solution architecture without owning business logic. |

---

## 3. Commit Conventions

To make tracking, version history, and reverting architectural decisions straightforward, follow these rules:

1. **One Domain File per Commit**: Do not bundle changes to multiple domain specifications in a single commit unless the change is an approved governance synchronization.
2. **Commit Message Format**:
   - `docs(L2-A01): draft initial platform tenant workflows`
   - `docs(L3-QualificationCompletion): update approved business scenario status`
   - `docs(L5): clarify multi-tenancy solution architecture policy`
3. **Draft Commits**: Mark unfinished drafts with a `[WIP]` prefix in the commit message if working in shared feature branches.

---

## 4. Pull Request Rules

All additions, edits, or promotions of documents to `Locked` must go through a Pull Request targeting `main`.

- **PR Titles**: Follow the commit format, e.g. `docs(L2): lock Platform Foundation workflows`.
- **Description Requirements**:
  - Summarize the governance rule, architecture definition, or business workflow modified.
  - List multi-domain impacts or side effects.
  - Check off dependencies that must be reviewed.
- **Review Requirement**:
  - The Lead Architect, Dhruv, must review and approve all PRs before merging.
  - Domain-specific additions must have reciprocal feedback if they impact boundaries of other active domains.
  - Finance changes require review by the dedicated Finance Review Team: Varad and Devang.
  - Finance review responsibility is review-only. Chetan remains the implementation and domain owner of Finance.

---

## 5. Architecture Review Process

Before a document can be merged into `main`, it undergoes a formal review process:

```text
Draft Created -> Self-Review -> Pull Request -> Architecture Review -> Trust Certification Ready -> Locked Status -> Merge
```

1. **Alignment Check**: Reviewer ensures that the specification does not conflict with upstream architecture in the L0 -> L1 -> L2 -> L3 -> L4 -> L5 -> L6 hierarchy.
2. **Constraint Enforcement**: Verify that the document includes all required standard sections as defined by the relevant template.
3. **No Code Check**: Verify that the specification does not include language-specific types, concrete database tables, or UI / UX layouts unless the document level explicitly permits them.
4. **L3 Independence Check**: Verify that any L3 Business Scenario Architecture is independent of L2 numbering and may derive information from one or many L2 workflow documents without redefining them.

---

## 6. Versioning Rules

We use semantic versioning for architectural documents:

- **`v0.x` (Draft)**: Work in progress. Sections are being filled, questions are open, and review comments are being resolved.
- **Trust Certification Ready**: Repository-consistent and ready for final certification review, but not yet locked or merged.
- **`v1.0` (Locked / Base Version)**: Approved, locked, and merged to `main`. This is ready to guide development.
- **`v1.x` (Minor Revision)**: Backward-compatible clarifications, additional failures, or sub-workflow details.
- **`v2.0` (Major Revision)**: Breaking architectural change, such as merging domains, transferring ownership, or altering baseline dependency rules. Requires a new round of approvals and team sync.

---

## 7. Repository Maintenance

Whenever an architectural specification or workflow document is successfully reviewed and merged into `main`, the author of the changes must immediately update the repository indexes to reflect the new state. This guarantees that `main` is always self-consistent:

1. **[ARCHITECTURE_INDEX.md](/F:/Pravaha/ERP_Planning/architecture/ARCHITECTURE_INDEX.md)**: Update version, owner, status, and dependency mapping.
2. **[CHANGELOG.md](/F:/Pravaha/ERP_Planning/CHANGELOG.md)**: Record exact changes, folder relocations, or new document locked status under the appropriate version release block.
3. **[L2_DOMAIN_WORKFLOW_MATRIX.md](/F:/Pravaha/ERP_Planning/architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md)**: Mark the target domain checkboxes or statuses as completed.
4. **[DEPENDENCY_MAP.md](/F:/Pravaha/ERP_Planning/DEPENDENCY_MAP.md)**: If the document alters domain relationships or dependencies, update the corresponding maps immediately.
