# SutraERP Repository Constitution
## Volume 2 - Repository Standards

---

Version: 1.0

Status: LOCKED

Authority: Repository Constitution

Owner: Lead Architect

Depends On: Repository Constitution Volume 1

---

# 1. Purpose

This document defines the mandatory standards governing every artifact within the SutraERP repository.

It establishes repository structure, document conventions, naming standards, versioning, diagram standards, traceability, and quality requirements.

All documents shall comply with these standards unless explicitly exempted by the Repository Constitution.

---

# 2. Scope

These standards apply to:

- Markdown documents
- Architecture specifications
- Business documentation
- Technical documentation
- API specifications
- Database documentation
- UI documentation
- Repository assets
- Diagrams
- AI-generated documents

---

# 3. Canonical Architecture Hierarchy

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

Every document belongs to exactly one level unless it is a repository governance or metadata document.

---

# 4. Repository Structure

The repository follows this structure:

```text
Repository Constitution
|-- README
|-- L0 Business Vision
|-- L1 Business Domain Architecture
|-- L2 Domain Workflow Architecture
|-- L3 Business Scenario Architecture
|-- L4 System Behavior Specification
|-- L5 Solution Architecture
|-- L6 UI / UX Architecture
|-- Assets
`-- Archive
```

Planned folders shall not be referenced as existing paths until they are created.

---

# 5. Documentation Hierarchy

Every lower-level document inherits from higher-level documents:

```text
Repository Constitution
README
L0
L1
L2
L3
L4
L5
L6
```

No document may redefine higher-level architecture.

---

# 6. Document Metadata

Architecture specifications should begin with:

```text
Title
Version
Status
Owner
Architectural Level
Domain
Last Updated
Depends On
Referenced By
```

Repository guides, indexes, and governance documents may use simplified headers where the file already self-identifies its authority and scope.

---

# 7. Document Status

Every document shall have exactly one status:

- Draft
- Proposed
- Approved
- Locked
- Deprecated
- Archived

---

# 8. Versioning Standard

Versioning follows semantic versioning:

- **Major**: breaking architectural change.
- **Minor**: new architectural capability.
- **Patch**: corrections, examples, formatting, and clarifications.

---

# 9. Naming Conventions

Documents use stable level-oriented identifiers:

```text
L0-[name]
L1-[domain-id]
L2-[section][number]
L3-[ScenarioName]
L4-[BehaviorName]
L5-[SolutionName]
L6-[InteractionName]
```

Entities use PascalCase, such as `StudentContext`, `ProgrammeVersion`, `AcademicCreditLedger`, and `CourseOffering`.

Events use past tense, such as `AdmissionApproved`, `StudentActivated`, `AttendanceFinalized`, `ResultPublished`, and `QualificationAwarded`.

Commands use imperative form, such as `CreateStudent`, `PublishResult`, `ActivateStudent`, and `GenerateHallTicket`.

REST resources use plural nouns.

---

# 10. Markdown Standards

Markdown documents shall use:

- Heading hierarchy
- Lists
- Tables
- Code blocks
- Mermaid diagrams
- Horizontal separators

Large paragraphs should be avoided.

---

# 11. Mermaid Standards

Mermaid diagrams are the official diagram language.

Supported diagram types include flowchart, state diagram, sequence diagram, entity relationship, class diagram, journey, Gantt, and mindmap.

Every workflow document shall include at least one Mermaid workflow. Every lifecycle shall include a state diagram. Images shall never replace editable diagrams.

---

# 12. Tables

Tables shall be used for ownership, responsibilities, entity definitions, version history, dependencies, decision matrices, and business rules.

---

# 13. Cross References

Documents shall reference parent documents, dependent documents, and related domains.

No orphan documents shall exist.

---

# 14. Canonical References

Business entities shall always reference their canonical owner.

Example: Student Context is owned by Student Lifecycle, not Academic Delivery.

---

# 15. Examples

Examples illustrate architecture. They do not define architecture.

Examples are informative. Architecture is normative.

---

# 16. Diagrams vs Text

Text defines architecture.

Diagrams explain architecture.

If a conflict exists, text takes precedence.

---

# 17. AI-Generated Content

AI-generated content shall:

- Follow repository standards.
- Reference canonical architecture.
- Never invent entities.
- Never duplicate ownership.
- Never contradict locked decisions.
- Preserve canonical terminology.

Every AI-generated architectural change requires human review.

---

# 18. Traceability

Every architectural decision shall be traceable through:

```text
Higher Level
Business Requirement
Affected Domains
Dependent Documents
Implementation
```

No undocumented architectural changes are permitted.

---

# 19. Repository Quality Gates

Before a document becomes Approved or Locked it shall satisfy:

- Correct metadata
- Valid Markdown
- Renderable Mermaid
- Correct version
- Dependency references
- Canonical ownership
- No duplicate entities
- No broken references
- No architectural conflicts
- No undefined terms

---

# 20. Repository Review Checklist

Every architectural review shall verify:

- Repository hierarchy
- Naming conventions
- Entity ownership
- Business consistency
- Multi-domain dependencies
- Event naming
- Diagram accuracy
- Version correctness
- Document completeness
- Traceability

---

# 21. Locked Repository Standards

The following standards are constitutional:

- Markdown is the official documentation format.
- Mermaid is the official diagram language.
- Semantic versioning is mandatory.
- Every document requires metadata or an approved simplified header.
- Every architectural decision requires traceability.
- Every entity has one canonical owner.
- Every architecture document belongs to one architectural level.
- AI-generated content requires human review.
- Images shall never replace editable architecture diagrams.
- Repository hierarchy shall not be violated.

---

Status: LOCKED

Authority Level: Repository Standards

Next Document:

Repository Constitution

Volume 3 - Architecture Standards
