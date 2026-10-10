# SutraERP backend P1-001 — Logical data ownership and scope blueprint (initial inventory)

**Status:** DRAFT / NOT APPROVED / NOT A PHYSICAL DATABASE SCHEMA  
**Date:** 2026-10-08  
**Architecture source commit:** `933cf5d06f565cc52ef7e3b2a7f821606c2b9b6f` (SutraERP_Architecture/main)  
**Frontend source commit:** `372376d23ac6478fa4ac3f209cb97c71317d89d2` (SutraERP_frontend/main)  
**Authority scope:** Canonical Architecture and frontend implementation are current source evidence; `DhruvNaik26/SutraERP_backend` was subsequently authorized/created (2026-10-08; README-only), but no legacy backend is an approved baseline. This inventory retains its original evidence snapshot.  
**Related planning draft:** [P0 authority and decisions (draft PR #160)](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/160). This document is independent and remains reviewable even if P0 is not merged.

## 1. Purpose and non-goals

Identify foundational domain-owned business entities, scopes and invariants before defining PostgreSQL table and column names, SQLx implementations, APIs, events, or migration code.

**Not yet authorized:** creating backend runtime code; naming physical tables/columns; asserting released APIs/events/registries; modifying locked architecture; merging or running CI. The new private backend repository location has been selected and initialized, but coding remains gated. This is a planning proposal to review, not a claim of full corpus inventory.

**Vocabulary for non-coders:**

- **Entity:** business fact or record type, e.g. Person or Institution.
- **Owner:** the only business domain allowed to decide and change its authoritative facts.
- **Scope:** which Tenant/Institution/other authorized context a fact belongs to.
- **Relationship:** how record identities refer to each other; a cross-owner reference is NOT permission to directly query a peer owner's tables.
- **Invariant:** a rule the software must never violate.
- **Physical schema:** eventual exact database table names, columns, types and constraints (not decided here).

## 2. Source precedence (verified)

1. [Locked L1 Business Domain Architecture](../../architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md) §§2, 3, 9, 24–25 defines fundamental entity and capability ownership and institutional hierarchy.
2. [Locked L1 Person Scope Amendment, 2026-08-26](../../architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Amendment_2026-08-26.md) **supersedes** earlier L1 Institute-scoped Person wording: **one canonical Person per real human within a Tenant**, shared by multiple Institutions *within that same Tenant*. It establishes no cross-Tenant Person sharing.
3. Specialized [L2-A02 Person and Identity](../../architecture/L2_Domain_Workflow_Architecture/section-a/L2-A02_Person_and_Identity_Workflows.md) and [locked reconciliation amendment, 2026-09-23](../../architecture/L2_Domain_Workflow_Architecture/section-a/L2-A02_User_Account_and_Person_Reconciliation_Authority_Amendment_2026-09-23.md) control Person identity and merge/split semantics.
4. [L2-B01 Institution](../../architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Workflow.md) §2.1 distinguishes **versioned Institution Policy** from Institution Profile and from A04 authorization policy.
5. [L2-C02 Student Lifecycle](../../architecture/L2_Domain_Workflow_Architecture/section-c/L2-C02_Student_Lifecycle_and_Activation.md) §11.10.1–11.10.4 controls the Student Aggregate and StudentContext split and lifecycle invariants.
6. [L5-A04 scope propagation](../../architecture/L5_Solution_Architecture/specifications/L5-A/L5-A-04-002_Tenant_and_Institution_Context_Propagation_and_Data_Isolation_Architecture.md), [L5-B02 ownership](../../architecture/L5_Solution_Architecture/specifications/L5-B/L5-B-02-002_Domain_Persistence_Ownership_and_Isolation_Architecture.md) and [ADR-L5-0036](../../architecture/L5_Solution_Architecture/adr/ADR-L5-0036_Persistence_Implementation_Conventions_and_Technical_Identity_Standard.md) govern technical isolation and persistence.

**Source conflict/terminology warning:** older L1 §3.4 says Person is Institute-scoped. That is **historical, superseded wording**, not a new unresolved design choice. Some descriptions of Student admission may say ownership of the "individual" transfers; do not interpret that as transfer of canonical Person identity from A02 to C02. Only the student business lifecycle belongs to C02.

## 3. Initial source-backed entity and ownership inventory

`Scope` below is the business scope explicitly supported by the cited documents where clear; "source-defined" means exact lower-level scope still requires domain specification and design verification. **Entries are logical entities/capabilities, never approved SQL table names.**

| ID | Business entity / capability | Canonical owner | Known scope / caution | Source |
|---|---|---|---|---|
| P1-E001 | Tenant | Platform / Section A | Tenant is the isolation root | L1 §§3.1, 9, 24 |
| P1-E002 | Person | Person & Identity / A02 | **One per real individual per Tenant** (where identity established); not Institution-unique | L1 amendment; L2-A02 |
| P1-E003 | Identity | Person & Identity / A02 | Tenant-rooted; not a Student or HR identity replacement | L1 §2.2; L2-A02 |
| P1-E004 | User Account | A02 | Governed account lifecycle and authentication eligibility; account != Person identity | L2-A02 and 2026-09-23 amendment |
| P1-E005 | Institution (Institute) | Institution / B01 | Exactly one parent Tenant; one Tenant may have many Institutions | L1 §§3.1, 9, 24; L2-B01 |
| P1-E006 | Institution Policy | Institution / B01 | Institution-governed, versioned, distinct from authorization policy and Institution Profile | L2-B01 §2.1 |
| P1-E007 | Organisation Structure / Organisation Unit | Organisation / B02 | Within Institution; exact unit hierarchy defined by B02 | L1 §§3.1, 24; L2-B02 |
| P1-E008 | Department | Organisation / B02 | Organisational unit under applicable Institution structure | L1 §§3.1, 24 |
| P1-E009 | Institutional Campus association/status | Organisation (institutional aspect) | Distinct from physical infrastructure Campus | L1 §24 |
| P1-E010 | Physical infrastructure Campus/site | Infrastructure / Section G | Physical location, not equivalent to organisational unit | L1 §§3.2, 24 |
| P1-E011 | Programme and Programme Version | Academic Structure / B04 | Institution/curriculum applicability source-defined; policy-version trace required | L1 §24 |
| P1-E012 | Course and Course Basket | Academic Structure / B04 | Curriculum definition, not a Course Offering or attendance record | L1 §§2.3, 24 |
| P1-E013 | Academic Year / Academic Period | Academic Calendar / B05 | Institution/operational scope source-defined | L1 §§9, 24 |
| P1-E014 | Admission Cycle | Admissions / C01 | Institution/College-School scoped admission | L1 §16 |
| P1-E015 | Application and Admission Decision | Admissions / C01 | Refer to canonical Person, without taking Person ownership | L1 §§16, 24; L2-C01 |
| P1-E016 | Student Aggregate (Student persona) | Student Lifecycle / C02 | Separate from canonical Person; represents Student persona | L2-C02 §§5–6, 11.10 |
| P1-E017 | StudentContext | Student Lifecycle / C02 | Exactly one Student Aggregate **and** one Institute per context | L2-C02 §11.10.2 |
| P1-E018 | Academic Registration | Student Lifecycle / C03 | Associated with the source-owned Student context | L1 §§9, 24 |
| P1-E019 | Academic Progression decision | Student Lifecycle / C04 | Consumes **official published** Examination results; never alters them | L1 §§20, 24; L2-C04 |
| P1-E020 | Academic Credit Ledger | Student Lifecycle / C04 | Institutional credit record, source-owned, not Examination's grade computation | L1 §§9, 20, 24 |
| P1-E021 | Qualification Completion / Student Exit | Student Lifecycle / C05 | Must not be decided by I13 alone; upstream conflict remains in architecture issue #136 | L1 §§9, 21, 24; issue #136 |
| P1-E022 | Alumni transition/capability | Student Lifecycle / C08 | Domain lifecycle, not automatic conversion of Person identity | L1 §24; L2-C08 |
| P1-E023 | Course Offering | Academics / Section D | Operational delivery is distinct from Academic Structure Course | L1 §§2.3, 24 |
| P1-E024 | Teaching Session and Attendance | Academics / Section D | Attendance belongs to Teaching Session, not directly to Course definition | L1 §§2.3, 24 |
| P1-E025 | Internal Assessment | Academics / Section D | Do not confuse with approved Examination outcomes | L1 §24 |
| P1-E026 | Final Approved Marks | Examination / I08 | Governed inputs for I09; provisional marks cannot act as approved result truth | L2-I08 / L2-I09 |
| P1-E027 | Institutionally Approved Result | Examination / I09 | Depends on I08 final marks and effective grading policy; not yet published | L2-I09 |
| P1-E028 | Official published Examination Result / Grade | Examination / I10 | Only released published outcomes may feed C04/I13 | L1 §24; L2-I09/I10; frontend issue #57 |
| P1-E029 | Financial Ledger and Fee Collection | Finance / Section F | Finance owns financial facts; no automatic Student activation | L1 §§9, 24 |
| P1-E030 | Workflow orchestration capability | Workflow Engine / Section A | Coordinates; does not become business-state owner | L1 §24; L5-A05 |
| P1-E031 | Documents capability | Document Management / Section H | Preserve respective business/document ownership boundary | L1 §24; L5-B04 |
| P1-E032 | Notifications capability | Communication / Section H | Communication delivery != business-outcome authority | L1 §24; L5-C04 |
| P1-E033 | Procurement | Enterprise Resource Management / Section J | Exact purchasing aggregates require L2-J owner expansion | L1 §25 |
| P1-E034 | Inventory | Enterprise Resource Management / Section J | Exact stock/receipt aggregates require L2-J owner expansion and resolution of #140–#152 where relevant | L1 §25 |

**Inventory status:** partial foundational coverage only. Full L1–L4 entity/aggregate catalogue, cardinalities, identity correlation and lifecycle extraction across all Sections A–J remain **NOT COMPLETED**. Do not call this list the complete SutraERP entity model.

## 4. Authoritative logical relationships and critical invariants

### 4.1 Tenant, identity and institution

- A **Tenant** may contain **multiple Institutions**; every Institution belongs to **exactly one Tenant** (L1 §3.1, L2-B01).
- Within one Tenant, a real individual has **one canonical Person**, regardless of the number of Institutions they participate in (L1 locked amendment). No cross-Tenant Person identity sharing is specified.
- Business-domain contexts reference canonical Person/Identity through governed relations. A shared Person does **not** make operational records globally shared or give other domains direct database access (L1 §§2–3).
- **Institutional Campus** and **physical infrastructure Campus** are different authoritative concepts (L1 §§3.2, 24). Do not create one table with ambiguous ownership of both.
- **Institution Policy** (B01), **Authorization Policy** (A04), and configuration/policy enforcement in consumer domains are not interchangeable (L2-B01 §2.1).

### 4.2 Student and history

- Student Aggregate is a **student persona** owned by C02, separate from canonical A02 Person.
- A **StudentContext** belongs to exactly **one Student** and **one Institute**; multiple historical contexts are permitted (L2-C02 §11.10.2).
- A Student may have **at most one Active StudentContext per Institute** at a time (SC-INV-001).
- **Stronger rule:** a new StudentContext for an Institute may be opened **only once every prior context for that Institute is terminal** (SC-INV-004). Checking merely `Active` duplicates is insufficient; `Pending Activation` and `Suspended` are non-terminal.
- Terminal states: **Qualification Completed, Withdrawn, Transferred, Discontinued, Terminated**. Terminal StudentContexts are immutable and kept for history (L2-C02 §11.10.1–.2).
- Exact physical uniqueness/exclusion constraint strategy is **NOT YET DECIDED**; it must enforce all these business invariants without arbitrary hard-coded status logic.

### 4.3 Person merge/split is a distributed reconciliation workflow

- Approved **merge** selects one surviving Person, keeps non-survivors historically, and never rewrites historical audit references.
- Approved **split** may yield **two or more** Persons, based on evidence; not always one-to-two.
- Identity/User Account dispositions require individual governance; Credentials/MFA/Sessions are **not automatically transferred**.
- After A02 publishes the canonical fact, **each downstream owning domain** corrects its own Person references through governed contracts/events. Never authorize A02 to update all peer-owner authoritative tables by direct SQL.
- These rules mean a simplistic `ON UPDATE CASCADE` across business owners is **not** a substitute for the canonical reconciliation workflow.
- Source: [Locked A02 merge/split authority amendment](../../architecture/L2_Domain_Workflow_Architecture/section-a/L2-A02_User_Account_and_Person_Reconciliation_Authority_Amendment_2026-09-23.md) §§3–6.

### 4.4 Cross-owner academic chain

`I08 Final Approved Marks` → `I09 Institutionally Approved Result` → `I10 Official Result Publication` → separate owner consumption by `C04` and `I13`; C05 alone owns qualification/exit decisions. This is a **semantic dependency**, NOT a suggested SQL join or invented endpoint/event payload. The authentic owner-generated evidence remains outstanding per [frontend issue #57](https://github.com/DhruvNaik26/SutraERP_frontend/issues/57), [frontend issue #50](https://github.com/DhruvNaik26/SutraERP_frontend/issues/50), and [architecture issue #136](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/136).

## 5. Technical data constraints — existing authority (not new decisions)

1. **Owner-specific** logical PostgreSQL namespaces/roles for co-located owners; direct cross-owner authoritative SQL read/write forbidden. Physical placement can evolve under B02 governance.
2. Every protected execution has validated **Tenant root**; Institution/lower context can only narrow it, not widen it; absent, stale, switched or mismatched scope fails closed. A04 intentionally does not mandate a `tenant_id` column on every table.
3. PostgreSQL 18.x + SQLx selected; new persistent **technical** IDs use **UUIDv7** by default, stored as PostgreSQL `uuid`. Source-owned canonical **business** identities are governed by their business owner and are not silently replaced by technical IDs (ADR-L5-0036).
4. Technical instants use PostgreSQL `timestamptz`, UTC normalized, while business dates and effective policy timezones require explicit semantics.
5. No global soft-delete convention. Owner states/history/retention rule the data lifecycle.
6. Schema migrations explicit, immutable once applied, separate privileged migration identity; expand→migrate→transition→contract.
7. Runtime process, PostgreSQL schema, architectural Section and business domain are **not automatically 1:1**. A03/E04 govern composition; no database-per-domain assumption.
8. Read models/projections are non-authoritative; no cross-owner authoritative joins for reporting.

## 6. Gaps that block exact DDL or dependent implementation

| Decision ID | Status | Needed before |
|---|---|---|
| P1-D01 Complete per-owner entity/aggregate inventory (L1–L4 Sections A–J) | INCOMPLETE | Calling overall logical model complete |
| P1-D02 Explicit cardinality/reference semantics for Person↔Student and Student↔institutional contexts; same-subject correlation | PARTIAL | Exact identifier/reference column design |
| P1-D03 Exact owner scope enforcement (SQL roles, transaction scope binding, RLS evaluation/alternatives, pooling resets, negative isolation tests) | NOT DECIDED | Production multitenant schema |
| P1-D04 Exact technical SQL naming, constraints, data types, indexes, migration pack format | PROPOSED, NOT APPROVED | Owner-specific physical DDL |
| P1-D05 Historical/current-position modelling for versioned policy, StudentContext, result publication, amendments and audit | PARTIAL | Per-owner physical schema |
| P1-D06 Person merge/split downstream reference correction/reconciliation and outstanding states | SEMANTICS APPROVED, MECHANICS UNDECIDED | Full identity integration |
| P1-D07 Backend runtime target selected; branch protections/review boundaries still open | TARGET CREATED; CONTROLS UNVERIFIED | Backend production coding; prior code is obsolete by user decision |
| P1-D08 Contract implementation/registry version truth (incl. J05 issue #152) | BLOCKED FOR AFFECTED SLICES | Publishing affected positive business events |
| P1-D09 C05/I13 qualification owner conflict (#136), source-derived contract details | OPEN FOR AFFECTED SLICES | Qualification physical/contract design |
| P1-D10 AI coding stack benchmark, local environments and runner isolation | NOT TESTED | Autonomous backend coding and CI gates |

**Explicit stop rule:** Do not close these by generating plausible schemas, test fixtures or front-end mock contracts. Escalate to authoritative owner/review.

## 7. Proposed P1 planning sequence (not approval)

- **P1-A:** expand inventory per Section A–J, record evidence/owner, note any specific source conflicts and version precedence.
- **P1-B:** define logical cardinalities, identity correlation, Tenant/Institution/lower-scope constraints and time/history ownership.
- **P1-C:** define owner boundary catalogue and cross-owner contract/event relationships; **apply existing locked L5 naming and identity conventions** (including ADR-L5-0034/0036), reviewing only domain-specific physical names not predetermined by the locked architecture.
- **P1-D:** derive exact PostgreSQL physical DDL **owner-by-owner**, alongside released OpenAPI/JSON Schema and test specifications.
- **P1-E:** approve first small end-to-end implementation slice only after P0 authority and P1-A–D slice prerequisites clear.

### P1-A acceptance criteria

- All owning Sections A–J enumerated; every source entity assigned exactly one authoritative owner, with ambiguous claims escalated.
- Explicit scope for Tenant-rooted/shared Institution and lower-scoped records; historical Person/Student invariants preserved.
- Source paths/versions and current PR/issue exceptions linked.
- No inferred business outcome, API/event identity, column, permission or state recorded as an approved fact.
- Lead Architect approves any promotion to governed implementation baseline.

## 8. Beginner troubleshooting glossary and expected test patterns

- **Wrong Tenant/Institution:** a request for another customer's/institution's protected data must be **rejected**, not silently switched.
- **Duplicate Person:** check Tenant-scoped identity resolution/reconciliation, not just whether a full name matches.
- **Student cannot activate:** inspect prior **non-terminal** StudentContext for that same Institute; Pending/Suspended can block reopening.
- **Marks appear but qualification fails:** inspect the separate I08→I09→I10 official publication and C04/I13→C05 owner evidence, not frontend fixture status.
- **Data exists but another module cannot read it:** first confirm a **released owner API/event/projection**, not add a cross-owner database join.
- **AI says complete:** demand exact source commit, migration/contract version, negative tests, passing targeted tests, and real business-state evidence.

### 2026-10-08 implementation direction clarification

The user created private `DhruvNaik26/SutraERP_backend` and designated all prior backend code obsolete as an implementation baseline. The frontend's real backend APIs are not yet configured. This logical catalogue is deliberately partial; we will continue broad ownership coverage while applying exact physical table/column design **only to individually approved integration slices**. No legacy code will be imported by assumption. See [P0/DEC-011 integration gates](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/160).

## 9. Integrity and review statement

Read-only evidence extraction from the two authorized repos; **one draft planning document only**. No change to existing canonical documents, L1–L6 ownership, migrations, policies, schema, contracts, workflow, frontend product code, or infrastructure. No runtime tests or CI executed. All diagrams/tables above are planning-level representations; there is no backend implementation or production validation claim.
