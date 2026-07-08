# Contributing to SutraERP Architecture

This repository stores the architectural specifications, domain models, and workflows for SutraERP. Since the code implementation directly follows the documents in this repository, all contributors must strictly adhere to the following processes and guidelines.

---

## 1. Branch Strategy

We maintain a strict branch segregation model:

- **`main`**: The source of truth. Contains only fully approved, peer-reviewed, and locked architectures. Directly guides development cycles. **No direct commits allowed.**
- **`dhruv`**: Active working branch for Lead Architect (Platform Foundation, Institution Structure, People & Workforce, and Governance reviews).
- **`pallav`**: Active working branch for Domain Architect (Student Lifecycle, Academic Delivery, Campus Operations, Services & Intelligence).

For hotfixes or specific task groupings, short-lived feature branches targeting `dhruv` or `pallav` can be created (e.g. `dhruv/feature-auth-workflows`).

---

## 2. Commit Conventions

To make tracking, version history, and reverting architectural decisions straightforward, follow these rules:

1. **One Domain File per Commit**: Do not bundle changes to multiple domain specifications in a single commit.
2. **Commit Message Format**:
   - `docs(L2-A01): draft initial platform tenant workflows`
   - `docs(L2-C02): update student lifecycle states to locked`
   - `docs(L1): clarify multi-tenancy schema policy`
3. **Draft Commits**: Mark unfinished drafts with a `[WIP]` prefix in the commit message if working in shared feature branches.

---

## 3. Pull Request (PR) Rules

All additions, edits, or promotions of documents to `Locked` must go through a Pull Request targeting `main`.

- **PR Titles**: Follow the commit format, e.g. `docs(Level 2): lock Platform Foundation workflows`.
- **Description Requirements**:
  - Summarize the business workflows modified or introduced.
  - List cross-domain impacts or side effects.
  - Check off dependencies that must be reviewed.
- **Review Requirement**:
  - The Lead Architect (Dhruv) must review and approve all PRs before merging.
  - Domain-specific additions must have reciprocal feedback if they impact boundaries of other active domains.

---

## 4. Architecture Review Process

Before a document can be merged into `main`, it undergoes a formal review process:

```text
Draft Created  ──>  Self-Review  ──>  Pull Request  ──>  Cross-Review  ──>  Locked Status  ──>  Merge
```

1. **Alignment Check**: Reviewer ensures that the specifications do not conflict with the baseline requirements and the locked Level 1 Architecture.
2. **Constraint Enforcement**: Verify that the document includes all required standard sections (e.g. Failure Scenarios, Audit Requirements, Synchronous Contracts) as per the templates.
3. **No Code Check**: Verify that the spec does not include language-specific types (e.g., Rust structs), concrete database tables, or UI/UX layouts.

---

## 5. Versioning Rules

We use a semantic versioning system for all architectural documents:

- **`v0.x` (Draft)**: Work in progress. Sections are being filled, questions are open, and review comments are being resolved.
- **`v1.0` (Locked / Base Version)**: Approved, locked, and merged to `main`. This is ready to guide development.
- **`v1.x` (Minor Revision)**: Backward-compatible clarifications, additional failures, or sub-workflow details.
- **`v2.0` (Major Revision)**: Breaking architectural change (e.g., merging domains, modifying synchronous contracts, or altering baseline rules). Requires a new round of approvals and team sync.

---

## 6. Repository Maintenance

Whenever an architectural specification or workflow document is successfully reviewed and merged into `main`, the author of the changes must immediately update the repository indexes to reflect the new state. This guarantees that `main` is always self-consistent:

1. **[ARCHITECTURE_INDEX.md](file:///g:/Pravaha/ERP_Planning/ARCHITECTURE_INDEX.md)**: Update the version number, owner, status, and dependency mapping.
2. **[CHANGELOG.md](file:///g:/Pravaha/ERP_Planning/CHANGELOG.md)**: Record the exact changes, folder relocations, or new document locked status under the appropriate version release block.
3. **[Level 2 matrix.md](file:///g:/Pravaha/ERP_Planning/docs/level-2/Level%202%20matrix.md)**: Mark the target domain checkboxes or statuses as completed.
4. **[DEPENDENCY_MAP.md](file:///g:/Pravaha/ERP_Planning/DEPENDENCY_MAP.md)**: If the document alters domain relationships or dependencies, the corresponding maps must be updated immediately.

