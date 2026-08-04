# SutraERP Architecture Repository

Welcome to the official **SutraERP Architecture & Planning Repository**. This repository is the single source of truth for the business architecture, domain workflow architecture, business scenario architecture, governance rules, and planning controls for SutraERP.

---

## 1. Project Overview

### What is SutraERP
SutraERP is a modular ERP platform for educational institutions. It is designed to support multi-tenant operation, academic administration, student lifecycle management, workforce administration, institutional services, campus operations, financial workflows, examination workflows, and enterprise resource management.

### Purpose of this Repository
This repository stores and governs the architecture and planning artifacts that must exist before implementation work begins. It is used to align architects, product stakeholders, and engineering teams on:

- domain boundaries
- ownership rules
- lifecycle definitions
- workflow dependencies
- review and lock controls

### Repository Philosophy
- **Design Before Code**: implementation must follow approved architecture.
- **Traceability**: every downstream artifact must trace to an upstream architectural source.
- **Canonical Ownership**: every business concept has exactly one authoritative owner.
- **Hierarchy Discipline**: the canonical architecture hierarchy is L0 -> L1 -> L2 -> L3 -> L4 -> L5 -> L6.
- **Planning Repository Only**: this repository contains architecture, governance, templates, generated presentation artifacts, and helper scripts. It does not contain application runtime code.

---

## 2. Current Repository Structure

The current repository structure is:

- [architecture/](/F:/Pravaha/ERP_Planning/architecture/)
  Authoritative architecture documents currently present in the workspace, including the L0 and L1 source documents, the L2 control artifacts, the architecture index, migration notes, and placeholder folders for L3 through L6.
- [AI_CONTEXT/](/F:/Pravaha/ERP_Planning/AI_CONTEXT/)
  Generated and pending AI context artifacts for Sections A through J.
- [governance/](/F:/Pravaha/ERP_Planning/governance/)
  Repository constitution, policy, and ADR folders.
- [prompts/](/F:/Pravaha/ERP_Planning/prompts/)
  Reusable repository prompts for AI context and dependency contract generation.
- [templates/](/F:/Pravaha/ERP_Planning/templates/)
  Standard templates for L1 through L6 documents.
- [assets/](/F:/Pravaha/ERP_Planning/assets/)
  Repository assets, diagrams, icons, images, and logos.
- [generated/](/F:/Pravaha/ERP_Planning/generated/)
  Generated HTML presentation artifacts derived from architecture documents.
- [archive/](/F:/Pravaha/ERP_Planning/archive/)
  Archived repository material.
- [tools/](/F:/Pravaha/ERP_Planning/tools/)
  Local helper scripts used for presentation and conversion support.

Architecture placeholder folders currently present:

- `architecture/L3_Business_Scenario_Architecture/`
- `architecture/L4_System_Behavior_Specification/`
- `architecture/L5_Solution_Architecture/`
- `architecture/L6_UI_UX_Architecture/`

---

## 3. Documentation Hierarchy

The architecture is divided into seven levels. Each level answers a different question.

| Level | Name | Definition |
|-------|------|------------|
| L0 | Business Vision | Defines why SutraERP exists, strategic intent, architectural context, and constraints. |
| L1 | Business Domain Architecture | Defines the canonical business domains, domain boundaries, and ownership. |
| L2 | Domain Workflow Architecture | Defines workflows, lifecycles, rules, and dependencies within an individual business domain. |
| L3 | Business Scenario Architecture | Defines independent end-to-end business scenarios that may derive information from one or many L2 workflow documents; L3 numbering is not dependent on L2 numbering. |
| L4 | System Behavior Specification | Defines expected system behavior for approved business scenarios without implementation design. |
| L5 | Solution Architecture | Defines engineering solution architecture needed to implement L4 behavior without changing business rules. |
| L6 | UI / UX Architecture | Defines user interaction, navigation, screens, and user journeys derived from L5 solution architecture without owning business logic. |

The dependency direction is:

```text
L0 -> L1 -> L2 -> L3 -> L4 -> L5 -> L6
```

L0 and L1 source documents currently exist in this workspace, along with the L2 control matrix and L2 section files. L3 through L6 have placeholder folders only and are not yet present as source architecture documents.

L3 Business Scenario Architecture is independent of L2 numbering. A business scenario may derive information from one or many L2 workflow documents, but it must not redefine L2 domain ownership or workflow logic.

---

## 4. Repository Workflow

```text
Architecture Design
      ->
Architecture Review
      ->
Decision Lock
      ->
Merge to Main
      ->
Implementation in the application repository
```

1. An architect creates or updates the relevant design document.
2. The document is reviewed for ownership, dependency, and lifecycle consistency.
3. Once approved, the document status is locked.
4. Locked architecture is merged to `main`.
5. Engineering teams implement from the locked architecture.

Business Scenario Architecture, System Behavior Specification, Solution Architecture, and UI / UX Architecture must be authored only after their upstream sources are approved and traceable.

---

## 5. Git Workflow

### Branch Strategy
- `main`: approved and locked architecture only.
- architect working branches: active drafting and review preparation.

### Pull Request Lifecycle
```text
Commit -> Pull Request -> Architecture Review -> Approval -> Merge to main
```

---

## 6. Team Responsibilities

### Dhruv (Lead Architect)
- Primary ownership:
  - Section A (Platform Foundation)
  - Section H (Institutional Services & Intelligence)
- Repository governance:
  - reviews every Level 2 architecture document
  - approves or rejects lock changes
  - maintains repository-wide architectural consistency

### Pallav (Architect)
- Primary ownership:
  - Section B (Institution & Structure)
  - Section D (Academic Delivery, including D07 Internal Assessment)

### Varad (Architect)
- Primary ownership:
  - Section C (Student Lifecycle)
  - Section E (People & Workforce)

### Chetan (Architect)
- Primary ownership:
  - Section F (Finance)
  - Section J (Enterprise Resource Management)
- Finance review team:
  - Varad
  - Devang
- Finance review team responsibility is review-only. Chetan remains the implementation and domain owner of Finance.

### Devang (Architect)
- Primary ownership:
  - Section G (Campus Operations)
  - Section I (Examination)

The owner map above must match `architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md` and `architecture/ARCHITECTURE_INDEX.md`.

---

## 7. Document Lifecycle

Architecture documents follow this **simplified progression**. It is a reading
aid, not the status vocabulary -- the complete and canonical list is the
**13-value** table in `governance/constitution/Volume_2_Repository_Standards.md`
§7, and every document carries exactly one status drawn from it.

1. **Draft**: being authored or revised.
2. **Review**: submitted for architectural review.
3. **Trust Certification Ready**: complete enough for final repository-integrity certification review, but not yet locked or merged.
4. **Locked**: approved and frozen as the current authoritative version.

Do not treat these four stages as the set of legal status values.

Documents should not be described as merged, present, or implemented unless the corresponding file actually exists in the repository.

---

## 8. Naming Conventions

### Folder Naming
Current architecture folders follow level-prefixed names such as:
- `L0_Business_Vision`
- `L1_Business_Domain_Architecture`
- `L2_Domain_Workflow_Architecture`
- `L3_Business_Scenario_Architecture`
- `L4_System_Behavior_Specification`
- `L5_Solution_Architecture`
- `L6_UI_UX_Architecture`

### Document Naming
Documents should use stable level-oriented names that clearly identify scope.

### Architecture IDs
- **L1**: `L1-[stage or domain id]`
- **L2**: `L2-[section][number]`
- **L3**: `L3-[ScenarioName]`

Examples should use current canonical terminology such as `QualificationCompletion`.

---

## 9. Templates

The [templates/](/F:/Pravaha/ERP_Planning/templates/) directory currently contains:

- [L1_BusinessDomain_Template.md](/F:/Pravaha/ERP_Planning/templates/L1_BusinessDomain_Template.md)
- [L2_DomainWorkflow_Template.md](/F:/Pravaha/ERP_Planning/templates/L2_DomainWorkflow_Template.md)
- [L3_BusinessScenario_Template.md](/F:/Pravaha/ERP_Planning/templates/L3_BusinessScenario_Template.md)
- [L4_SystemBehavior_Template.md](/F:/Pravaha/ERP_Planning/templates/L4_SystemBehavior_Template.md)
- [L5_SolutionArchitecture_Template.md](/F:/Pravaha/ERP_Planning/templates/L5_SolutionArchitecture_Template.md)
- [L6_UIUX_Template.md](/F:/Pravaha/ERP_Planning/templates/L6_UIUX_Template.md)

---

## 10. Future Repository Growth

- New architecture documents must be created from the correct template and then added to [architecture/ARCHITECTURE_INDEX.md](/F:/Pravaha/ERP_Planning/architecture/ARCHITECTURE_INDEX.md).
- A planned folder must not be referenced as if it exists until the folder is created in the repository.
- Downstream L3-L6 artifacts must not be indexed as present until their files are created.
- Section J ownership includes Enterprise Resource Management capabilities such as vendor management, supplier management, procurement, inventory management, stock management, warehouse management, asset lifecycle management, and resource management.
- Future L3 Business Scenario Architecture references must remain independent of L2 numbering and may derive from one or many L2 workflow documents.

---

## 11. Repository Governance

Core governance and control documents include:

- [architecture/ARCHITECTURE_INDEX.md](/F:/Pravaha/ERP_Planning/architecture/ARCHITECTURE_INDEX.md)
- [DEPENDENCY_MAP.md](/F:/Pravaha/ERP_Planning/DEPENDENCY_MAP.md)
- [CONTRIBUTING.md](/F:/Pravaha/ERP_Planning/CONTRIBUTING.md)
- [architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md](/F:/Pravaha/ERP_Planning/architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md)
- [architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md](/F:/Pravaha/ERP_Planning/architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md)

These documents must remain synchronized on ownership, hierarchy, dependency direction, and terminology.

