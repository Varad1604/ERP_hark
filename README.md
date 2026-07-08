# SutraERP Architecture Repository

Welcome to the official **SutraERP Architecture & Planning Repository**. This repository serves as the single source of truth for the architecture, domain models, and systemic workflows of SutraERP.

---

## 1. Project Overview

### What is SutraERP?
SutraERP is a next-generation Enterprise Resource Planning (ERP) platform custom-designed for educational institutions. It is architected to handle platform multi-tenancy, high-concurrency academic operations, identity federation, workforce administration, student lifecycle management, campus logistics, and comprehensive financial reporting.

### Purpose of this Repository
The purpose of this repository is to store, version, and govern all **architecture, design, and planning documentation** for the SutraERP project. It functions as the blueprint of the platform before any code is written, ensuring that stakeholders, product managers, and software engineers share a unified understanding of system boundaries and business logic.

### Repository Philosophy
- **Design Before Code**: No feature or module is implemented in the application without first receiving an approved, locked architectural design.
- **Traceability**: Every software component, database boundary, or service boundary must trace directly back to a specific level of architecture in this repository.
- **Tech-Stack Agnostic at Higher Levels**: The design starts at a pure business-logic and domain-interaction level before narrowing down to interface contracts.

### Difference Between Architecture and Application Repositories
- **This Repository (Architecture & Planning)**: Contains ONLY markdown documentation, sequence/architecture diagrams, wireframes/mockups, integration templates, and slide presentations. It does **NOT** contain any application code, database migration scripts, or actual running code.
- **Application Repository (Implementation)**: Contains the functional codebase (Rust, Next.js, etc.), deployment configurations, database schemas, and unit/integration tests built strictly in accordance with the locked designs in this repository.

---

## 2. Repository Structure

This repository is organized into five core top-level directories:

- [docs/](file:///g:/Pravaha/ERP_Planning/docs/)
  The central documentation directory. It is structured hierarchically by design level (Baseline through Level 6), providing a clear separation between overarching system architecture and granular workflow definitions.
- [presentation/](file:///g:/Pravaha/ERP_Planning/presentation/)
  Contains HTML slide decks, styling sheets, and Javascript-based conversion tools used to render architectural plans into human-friendly interactive visual slide presentations.
- [templates/](file:///g:/Pravaha/ERP_Planning/templates/)
  Contains standard blueprint markdown files for various architectural levels. These files must be copied and filled out when creating new architectural documents to ensure consistency across the team.
- [assets/](file:///g:/Pravaha/ERP_Planning/assets/)
  Stores visual assets supporting the documentation, structured into `diagrams/` (architecture diagram source files and exports), `images/` (screenshots, system mockups), and `icons/` (UI/UX icon packs).
- [archive/](file:///g:/Pravaha/ERP_Planning/archive/)
  Retains outdated, superceded, or deprecated versions of architecture documents for historical reference and traceability.

---

## 3. Documentation Hierarchy

The architecture is divided into seven hierarchical levels, moving from high-level workflows down to concrete role behaviors and system actions.

### Baseline
- **Purpose**: Establishes the initial technical requirements, product context, and core system constraints.
- **Expected Documents**: [SutraERP_Architecture_Workflow_Context_Baseline.md](file:///g:/Pravaha/ERP_Planning/docs/baseline/SutraERP_Architecture_Workflow_Context_Baseline.md).
- **Output**: Core platform scope, high-level business vision, and non-functional requirements.

### Level 0
- **Purpose**: Defines the enterprise-wide context, showing SutraERP's position in relation to external entities and institutions.
- **Expected Documents**: Level 0 System Context diagrams and high-level external interface definitions.
- **Output**: Global context diagrams mapping the boundary between the ERP and external systems.

### Level 1
- **Purpose**: Defines the platform-wide architectural lock, delineating core structural paradigms and system tiers.
- **Expected Documents**: [SutraERP_Level_1_Architecture_Locked.md](file:///g:/Pravaha/ERP_Planning/docs/level-1/SutraERP_Level_1_Architecture_Locked.md).
- **Output**: Systemic patterns, database tenancy approaches, API protocols, caching architectures, and authentication rules.

### Level 2
- **Purpose**: Defines domain-level workflows, boundaries, state lifecycle transitions, and business rules without referencing specific API payloads or database models.
- **Expected Documents**: [Level 2 matrix.md](file:///g:/Pravaha/ERP_Planning/docs/level-2/Level%202%20matrix.md) and individual domain documents inside subfolders.
- **Subfolders**:
  - `00-control/`: Project management, architectural alignment, and coordination.
  - `A-platform-foundation/`: Platform tenancy, identity management, workflow engines, configuration registries, audit infrastructure, and master data management.
  - `B-institution-structure/`: Institutional structures, organizational hierarchies, campus mappings, academic calendars, and period setups.
  - `C-student-lifecycle/`: Admissions, activations, registrations, academic progression, exit pathways, guardians, and scholarships.
  - `D-academic-delivery/`: Course definitions, course offerings, faculty assignments, timetabling, session attendance, and academic monitoring.
  - `E-people-workforce/`: HR contracts, employee profiles, positions, faculty workforce, leaves, and payroll administration.
  - `F-finance-examination/`: General ledger, billing, student fees, assessment planning, marksheet registries, and grading rules.
  - `G-campus-operations/`: Campus facilities, laboratory schedules, hostel assignments, transit routing, libraries, security, and cleaning operations.
  - `H-services-intelligence/`: Shared media/document stores, global notification queues, directories, student placement drives, research trackers, and reporting/analytics.
  - `reviews/`: Cross-domain reviews and feedback summaries.
- **Expected Output**: Structured domain documents specifying purpose, scope, ownership, entities, states, workflows, boundaries, and dependencies.

### Level 3
- **Purpose**: Focuses on cross-domain integrations, detailing transactional boundaries and real-time/asynchronous interactions between two or more Level 2 domains.
- **Expected Documents**: Cross-domain message sequences, Saga transaction boundaries, and event-driven data flow designs.
- **Output**: Integration blueprints and message routing specifications.

### Level 4
- **Purpose**: Details internal module designs, components, and API interface contracts.
- **Expected Documents**: Module interface schemas, API definitions, state models, and configuration JSON schemas.
- **Output**: Detailed interface designs ready to guide software developers during coding.

### Level 5
- **Purpose**: Defines user personas, roles, and granular workspace activities.
- **Expected Documents**: Role Permission matrices, workspace action logs, and dashboard content mappings.
- **Output**: Precise role-based access control (RBAC) and UI dashboard scopes.

### Level 6
- **Purpose**: Details end-to-end user-level business workflows, mapping user interactions with specific UI elements and systemic feedback.
- **Expected Documents**: Screen navigation flows, UI/UX interaction steps, localized error-handling behavior, and UI audit requirements.
- **Output**: Highly granular user-centric workflow specifications.

---

## 4. Repository Workflow

To ensure that implementation never drifts from the approved system design, we follow a strict sequence:

```text
Architecture Design
       ↓
 Architectural Review
       ↓
  Decision Lock
       ↓
   Merge to Main
       ↓
Code Implementation (in Application Repo)
```

1. **Architecture Design**: The responsible architect writes a detailed draft based on the required level template.
2. **Review**: The draft undergoes peer review and Lead Architect review via Pull Request.
3. **Lock**: Once all constraints are satisfied, the architecture file status is changed to `Locked`.
4. **Merge**: The approved design is merged into the `main` branch.
5. **Development**: Software engineers build features using the `main` branch documentation as their definitive reference.

---

## 5. Git Workflow

### Branch Strategy
- `main`: Represents the approved and locked architecture. Directly reflects the actual blueprint for the development team. No commits are pushed directly to `main`.
- `dhruv`: Working branch for the Lead Architect. Used to draft foundation, structure, and workforce schemas.
- `pallav`: Working branch for the Domain Architect. Used to draft student lifecycle, academic delivery, operations, and services.

### Pull Request Lifecycle
```text
Commit  ──>  Pull Request  ──>  Architecture Review  ──>  Approval  ──>  Merge to main
```
1. **Commit**: Work must be committed in logical units (typically one complete `.md` file per commit).
2. **Pull Request**: Open a PR from the working branch (e.g. `dhruv` or `pallav`) targeting `main`.
3. **Architecture Review**: The Lead Architect reviews the PR for cross-domain integrity and standard compliance.
4. **Approval & Merge**: Once approved, the changes are merged into `main` and the version tag is incremented if applicable.

---

## 6. Team Responsibilities

### Dhruv (Lead Architect)
- **Primary Ownership**:
  - Section A (Platform Foundation)
  - Section B (Institution & Structure)
  - Section E (People & Workforce)
- **System Governance**:
  - Directs overall architectural guidelines and enforces compliance.
  - Reviewer and approver for all Pull Requests.
  - Maintains repository integrity, indexing, and main branch status.

### Pallav (Domain Architect)
- **Primary Ownership**:
  - Section C (Student Lifecycle)
  - Section D (Academic Delivery)
  - Section G (Campus Operations)
  - Section H (Institutional Services)
  - Section F03 (Team Contract Boundary Drafts)
- **Responsibilities**:
  - Drafts and aligns operational workflows for academic and logistics domains.
  - Coordinates cross-domain boundaries with the Lead Architect.

---

## 7. Document Lifecycle

Every architecture document progresses through these sequential phases:

1. **Draft**: Document is currently being drafted by the owner on a working branch. It may have open questions and tentative workflows.
2. **Review**: Document is complete and submitted via Pull Request. Open for peer feedback and conflict resolution.
3. **Locked**: Document is reviewed, approved, and status marked as Locked. No further changes can be made without version bump.
4. **Merged**: The locked document is merged into the `main` branch.
5. **Versioned**: Post-merge, any modifications require a new Pull Request, status update to `Draft` (with version increment), and re-approval.

---

## 8. Naming Conventions

### Folder Naming
Folders should be lowercase and hyphen-separated.
- Good: `level-2`, `platform-foundation`
- Bad: `Level 2`, `platform_foundation`

### Document Naming
Documents should follow CamelCase or snake_case with level prefixes as appropriate.
- Good: `SutraERP_Level_1_Architecture_Locked.md`, `Level2_Domain_Template.md`
- Bad: `level 1 architecture locked.md`

### Architecture IDs
Every domain and cross-domain workflow must have a unique identifier for mapping:
- **Level 2 (Domains)**: `L2-[Section Letter][Domain Number]` (e.g., `L2-A01` for Platform & Tenant, `L2-C02` for Student Lifecycle).
- **Level 3 (Cross-Domain Integration)**: `L3-[DomainA]-[DomainB]` (e.g., `L3-Admissions-StudentFinance`).

---

## 9. Templates

The [templates/](file:///g:/Pravaha/ERP_Planning/templates/) directory contains standardized files mapping to documentation levels:
- [Level2_Domain_Template.md](file:///g:/Pravaha/ERP_Planning/templates/Level2_Domain_Template.md): Used for all domain definition files under `docs/level-2/`.
- [Level3_CrossDomain_Template.md](file:///g:/Pravaha/ERP_Planning/templates/Level3_CrossDomain_Template.md): Used for integration workflows.
- [Level4_Module_Template.md](file:///g:/Pravaha/ERP_Planning/templates/Level4_Module_Template.md): Used for developer interface contracts.
- [Level5_Role_Template.md](file:///g:/Pravaha/ERP_Planning/templates/Level5_Role_Template.md): Used for persona privilege setups.
- [Level6_Workflow_Template.md](file:///g:/Pravaha/ERP_Planning/templates/Level6_Workflow_Template.md): Used for UX step-by-step descriptions.

---

## 10. Future Repository Growth

- **Adding New Documents**: Copy the relevant template from `templates/`, name it according to the naming guidelines, place it in the appropriate folder under `docs/`, assign a unique ID, and update [ARCHITECTURE_INDEX.md](file:///g:/Pravaha/ERP_Planning/ARCHITECTURE_INDEX.md).
- **Archiving Obsolete Versions**: If a locked architecture document is superceded, move the older version into [archive/](file:///g:/Pravaha/ERP_Planning/archive/) with a suffix specifying the date and version it was superceded.
- **Assets Management**: All diagrams, illustrations, and UI mockups must be saved in [assets/](file:///g:/Pravaha/ERP_Planning/assets/). No ad-hoc directories for images should be created outside the assets root.
- **No Implementation Code**: Under no circumstances should executable application code, test suites, or package configuration files (such as `Cargo.toml` or `package.json`) be committed here. This repository is strictly reserved for planning and system documentation.

---

## 11. Repository Governance

To maintain coordination between architects and prevent structural drift, the repository relies on the following core governance documents:

* **[ARCHITECTURE_INDEX.md](file:///g:/Pravaha/ERP_Planning/ARCHITECTURE_INDEX.md)**: Serves as the central registry mapping file paths, ownership, version numbers, status, and direct upstream dependencies.
* **[CHANGELOG.md](file:///g:/Pravaha/ERP_Planning/CHANGELOG.md)**: Details the evolution of repository guidelines, locked baseline configurations, structure, templates, and indexes.
* **[DEPENDENCY_MAP.md](file:///g:/Pravaha/ERP_Planning/DEPENDENCY_MAP.md)**: Details upstream/downstream level structures, domain relationships, ChatGPT context segmentation rules, and validation workflows.
* **[Level 2 matrix.md](file:///g:/Pravaha/ERP_Planning/docs/level-2/Level%202%20matrix.md)**: Tracks execution progress, section reviews, and status checkboxes for all Level 2 business domains.

