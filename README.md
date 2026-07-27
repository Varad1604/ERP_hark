# SutraERP Architecture Repository

Welcome to the official **SutraERP Architecture & Planning Repository**. This repository is the single source of truth for the business architecture, workflow architecture, governance rules, and planning controls for SutraERP.

---

## 1. Project Overview

### What is SutraERP
SutraERP is a modular ERP platform for educational institutions. It is designed to support multi-tenant operation, academic administration, student lifecycle management, workforce administration, institutional services, campus operations, and financial and examination workflows.

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
- **Planning Repository Only**: this repository contains architecture, governance, templates, generated presentation artifacts, and helper scripts. It does not contain application runtime code.

---

## 2. Current Repository Structure

The current repository structure is:

- [architecture/](/F:/Pravaha/ERP_Planning/architecture/)
  Authoritative architecture documents currently present in the workspace, including the L0 and L1 source documents, the L2 control artifacts, the architecture index, and migration notes. L3 through L6 are reserved as downstream levels and remain empty placeholders until their source documents are created.
- [governance/](/F:/Pravaha/ERP_Planning/governance/)
  Repository constitution and governance volumes.
- [templates/](/F:/Pravaha/ERP_Planning/templates/)
  Standard templates for L1 through L6 documents.
- [generated/](/F:/Pravaha/ERP_Planning/generated/)
  Generated HTML presentation artifacts derived from architecture documents.
- [tools/](/F:/Pravaha/ERP_Planning/tools/)
  Local helper scripts used for presentation and conversion support.

Planned folders such as `assets/`, `archive/`, and `ADR/` are not yet present in this workspace and must not be treated as existing repository paths until they are created.

---

## 3. Documentation Hierarchy

The architecture is divided into seven levels. Each level answers a different question.

### L0 - Business Vision
- **Purpose**: defines why SutraERP exists and the architectural context it must satisfy.
- **Output**: business vision / architecture context document.

### L1 - Business Domain Architecture
- **Purpose**: defines what domains exist and who owns them.
- **Output**: canonical business domain architecture.

### L2 - Domain Workflow Architecture
- **Purpose**: defines how an individual domain works, including lifecycle, workflow, rules, and dependencies.
- **Output**: domain workflow specifications.

### L3 - Business Scenario Architecture
- **Purpose**: defines end-to-end multi-domain business scenarios.
- **Output**: business scenario specifications.

### L4 - System Behavior Specification
- **Purpose**: defines system behavior without introducing implementation design.
- **Output**: non-technical behavior specifications.

### L5 - Solution Architecture
- **Purpose**: defines the engineering solution needed to implement L4 behavior.
- **Output**: technical solution architecture.

### L6 - UI / UX Architecture
- **Purpose**: defines user interaction, navigation, screens, and user journeys.
- **Output**: UI / UX architecture.

L0 and L1 source documents currently exist in this workspace, along with the L2 control matrix and a subset of L2 section files. L3 through L6 are planned downstream levels and are not yet present as source documents.

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
  - Section G (Campus Operations)
  - Section H (Institutional Services & Intelligence)
- Repository governance:
  - reviews every Level 2 architecture document
  - approves or rejects lock changes
  - maintains repository-wide architectural consistency

### Pallav (Architect)
- Primary ownership:
  - Section B (Institution & Structure)
  - Section D (Academic Delivery)

### Varad (Architect)
- Primary ownership:
  - Section C (Student Lifecycle)
  - Section E (People & Workforce)

### Chetan (Architect)
- Primary ownership:
  - Section F (Finance)

### Devang (Architect)
- Primary ownership:
  - Section I (Examination)

The owner map above must match `architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md` and `architecture/ARCHITECTURE_INDEX.md`.

---

## 7. Document Lifecycle

Architecture documents move through these states:

1. **Draft**: being authored or revised.
2. **Review**: submitted for architectural review.
3. **Locked**: approved and frozen as the current authoritative version.

Documents should not be described as merged, present, or implemented unless the corresponding file actually exists in the repository.

---

## 8. Naming Conventions

### Folder Naming
Current architecture folders follow level-prefixed names such as:
- `L0_Business_Vision`
- `L1_Business_Domain_Architecture`
- `L2_Domain_Workflow_Architecture`

### Document Naming
Documents should use stable level-oriented names that clearly identify scope.

### Architecture IDs
- **L1**: `L1-[stage or domain id]`
- **L2**: `L2-[section][number]`
- **L3**: `L3-[ScenarioName]`

Examples should use current canonical terminology such as `QualificationCompletion`, not superseded terminology such as `Graduation`.

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

---

## 11. Repository Governance

Core governance and control documents include:

- [architecture/ARCHITECTURE_INDEX.md](/F:/Pravaha/ERP_Planning/architecture/ARCHITECTURE_INDEX.md)
- [DEPENDENCY_MAP.md](/F:/Pravaha/ERP_Planning/DEPENDENCY_MAP.md)
- [CONTRIBUTING.md](/F:/Pravaha/ERP_Planning/CONTRIBUTING.md)
- [architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md](/F:/Pravaha/ERP_Planning/architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md)
- [architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md](/F:/Pravaha/ERP_Planning/architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md)

These documents must remain synchronized on ownership, hierarchy, dependency direction, and terminology.

