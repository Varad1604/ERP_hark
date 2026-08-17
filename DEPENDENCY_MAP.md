# SutraERP Architecture Dependency Map

## 1. Purpose

The Dependency Map defines architectural dependency direction across the SutraERP repository.

It ensures that:

- higher-level documents constrain lower-level documents
- domain ownership remains unique
- workflow collaboration does not get mistaken for shared ownership
- dependency direction remains acyclic at the architectural level

This document defines **architectural dependency direction**, not implementation topology.

---

## 2. Architecture Dependency Hierarchy

```text
L0 - Business Vision
    |
    v
L1 - Business Domain Architecture
    |
    v
L2 - Domain Workflow Architecture
    |
    v
L3 - Business Scenario Architecture
    |
    v
L4 - System Behavior Specification
    |
    v
L5 - Solution Architecture
    |
    v
L6 - UI / UX Architecture
```

---

## 3. Canonical Architecture Definitions

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

## 4. Level Dependency Matrix

| Architecture Level | Depends On | Used By | Purpose |
|--------------------|------------|---------|---------|
| L0 - Business Vision | None | L1 | Strategic intent, architectural context, and constraints |
| L1 - Business Domain Architecture | L0 | L2 | Canonical domains, boundaries, and ownership |
| L2 - Domain Workflow Architecture | L1 | L3 | Domain workflows, lifecycles, rules, and dependencies |
| L3 - Business Scenario Architecture | L2 | L4 | Independent end-to-end business scenarios that may derive from one or many L2 workflow documents |
| L4 - System Behavior Specification | L3 | L5 | Expected system behavior without implementation design |
| L5 - Solution Architecture | L4 | L6 | Engineering solution architecture without changing business rules |
| L6 - UI / UX Architecture | L5 | Development | Interaction, navigation, screens, and user journeys |

---

## 5. L1 Business Domain Dependency Graph

```text
Section A  Platform Foundation
    |
    v
Section B  Institution & Structure
    |
    +--> Section C  Student Lifecycle
    |
    +--> Section E  People & Workforce
    |
    +--> Section G  Campus Operations
               |
Section C  ----+----> Section D  Academic Delivery
Section E  ---------------^
Section D  ------------------> Section I  Examination
Section C  ------------------> Section F  Finance
Section A/B/C/D/E/F/G/I ------------> Section J  Enterprise Resource Management

Section H consumes approved read contracts and published events from
all sections but does not own canonical business entities.

Cross-cutting services exposed by Section H do not, by themselves,
create a foundational upstream dependency for every consuming section.
```

---

## 6. L1 Business Domain Dependency Matrix

The matrix below records **foundational architectural dependencies** only.
Runtime event feedback does not reverse ownership or create a new upstream dependency.

| Section | Foundational Depends On |
|----------|-------------------------|
| Section A - Platform Foundation | L0 |
| Section B - Institution & Structure | Section A |
| Section C - Student Lifecycle | Sections A, B |
| Section D - Academic Delivery | Sections A, B, C, E, G |
| Section E - People & Workforce | Sections A, B |
| Section F - Finance | Sections A, B, C |
| Section G - Campus Operations | Sections A, B, E |
| Section H - Institutional Services & Intelligence | All Sections |
| Section I - Examination | Sections A, B, C, D, F |
| Section J - Enterprise Resource Management | Sections A, B, C, D, E, F, G, I |

> **Corrected 2026-08-06 under integrity finding INT-204.** The Section I row previously
> read "Sections C, D, F", omitting A and B. It was the only row in this matrix that
> excluded Section A, which section 7 below describes as providing "shared infrastructure
> used by every business domain". `L2-I13` line 39 depends on L2-A01/A02/A04/A06/A07 and
> on L2-B04 Curriculum Framework and L2-B05 Academic Calendar; `L2-I14`, `L2-I15` and
> `L2-I16` line 39 each depend on L2-A01–A08 and L2-B01–B05. The four Section I documents
> were correct and this row was wrong.

Special rule:

- Student Lifecycle may consume **published official results** from Examination for progression and qualification completion.
- That runtime consumption does not make Examination an upstream architectural dependency of Section C.

---

## 7. Cross-Section Dependency Principles

## Section A - Platform Foundation

Provides shared infrastructure used by every business domain.

Owns:

- Identity
- Authentication
- Authorization
- Workflow Engine
- Events
- Audit
- Module Registry
- Master Data
- Operational Readiness

## Section B - Institution & Structure

Provides institutional structure and canonical academic structure.

Owns:

- Institution
- Organisation hierarchy
- Institutional Campus (identity and Institution association; the physical site model is owned by Section G / L2-G10)
- Academic Structure
- Curriculum framework
- Academic Calendar

Does not own:

- Infrastructure Management
- Course Offering
- Student lifecycle state

## Section C - Student Lifecycle

Owns the student lifecycle and student academic state.

Examples:

- Admissions
- Activation
- Academic Registration
- Academic Progression
- Qualification Completion and Student Exit
- Guardian relationships
- Scholarship Eligibility
- Alumni transition

## Section D - Academic Delivery

Consumes:

- Students
- Faculty
- Curriculum
- Campus and infrastructure context

Owns:

- Course Offering
- Faculty Academic Assignment
- Timetable
- Teaching Sessions
- Attendance
- Academic Monitoring

Does not own:

- Course
- Student promotion
- Official results

## Section E - People & Workforce

Owns workforce information.

Examples:

- Employees
- Employee Profile
- Faculty workforce profile
- HR
- Leave
- Payroll
- Position Assignment

## Section F - Finance

Owns all monetary operations.

Examples:

- Fees
- Payments
- Refunds
- Accounting
- Scholarship Financial Processing
- Financial Operations

No other section owns financial transactions.

## Section G - Campus Operations

Consumes institutional and workforce information.

Owns:

- Facilities
- Lab Management
- Hostel
- Library
- Transport
- Medical
- Security
- Housekeeping
- Teacher Guardian Scheme (TGS)
- Infrastructure Management

## Section H - Institutional Services & Intelligence

Provides supporting services and read-model surfaces.

Examples:

- APAAR & DigiLocker
- Notifications
- Directory
- Placement
- Research
- Dashboards
- Reporting
- Compliance
- External Integrations

Section H provides services and projections.

It does **not** own canonical business entities such as Student Context, Alumni, Course, Finance Ledger, or Examination Result.

## Section I - Examination

Consumes:

- Student Lifecycle context
- Academic Delivery inputs
- Finance status where policy requires it
- Finalized Internal Assessment outputs where policy requires it

Owns:

- Assessment
- Examination
- Evaluation
- Grade Processing
- Result Processing
- Academic outcomes

No other section owns official examination results.

---

## Section J - Enterprise Resource Management

Consumes approved demand, policy, budget, institutional, workforce, and operational context from upstream domains as required.

Owns Enterprise Resource Management capabilities such as:

- Vendor Management
- Supplier Management
- Procurement
- Inventory Management
- Stock Management
- Warehouse Management
- Asset Lifecycle Management
- Resource Management

No other section owns the enterprise resource lifecycle for procurement, stock, warehouse, inventory, vendor, supplier, asset, or resource governance.

---

## 7A. Dependency Classes

> Added 2026-08-05 under integrity finding INT-035. This register previously recorded
> only foundational dependencies, so genuine runtime integrations were invisible to any
> reader treating it as the repository dependency register.

A dependency between two sections belongs to exactly one of the following classes. Only
**foundational** dependencies constrain build order; the others do not, and must never be
used to argue that ownership or upstream direction has changed.

| Class | Meaning | Constrains build order | Reverses ownership |
|---|---|---|---|
| **Foundational** | The section cannot be architected or built without the upstream capability. Recorded in Section 6. | Yes | No |
| **Runtime contract** | The section calls a published synchronous contract of another section at run time. | No | No |
| **Event consumption** | The section subscribes to business events published by another section. | No | No |
| **Read model** | The section reads a published read model or projection owned elsewhere. | No | No |

### Runtime Contract and Event Consumption Register

| Consumer | Provider | Class | Fact consumed |
|---|---|---|---|
| `L2-F14` Procurement Finance Consumption | Section J | Event consumption | Procurement and resource facts published by Section J, consumed for finance recognition. F14 metadata declares "Depends On ... Section J"; this is a runtime dependency, **not** a foundational one. Finance remains foundationally dependent on A, B and C only. |
| Section C Student Lifecycle | Section I Examination | Event consumption | Published official results, consumed for progression and qualification completion. Does not make Examination upstream of Section C. |
| `L2-C02` Student Lifecycle | `L2-C01` Admissions | Event consumption | **Resolved 2026-08-17 (CF-06).** After Student Context creation, C02 consumes `AdmissionRevoked` and routes the C02-owned Student Context through its existing `WF-C02-10` / `ArchiveStudent()` path according to institutional policy. C01 retains ownership of the revocation decision and does not modify Student Context; C02 retains Student Context ownership and preserves the archived history. |
| `L2-C02` Student Lifecycle | `L2-C04` Academic Progression | Event consumption | **Resolved 2026-08-17 (C-C02, approved Option B).** C04 exclusively owns Programme-changing mobility through APW-08, including eligibility, curriculum mapping, credit mapping, approval, and `ProgrammeMigrationApproved`. C02 consumes the approved result only to update the C02-owned Student Context Programme association; C02 WF-C02-08 remains limited to non-Programme organisational or placement transfers. |
| Section J | `L2-J07` Stock Operations | Event consumption | `StockIssued`, `StockReturned`, `StockTransferDispatched`, `StockTransferReceived`. J06 applies ledger consequences; J07 remains sole publisher. |
| `L2-G02` Laboratory | Section D | Read model | Practical Timetable. Academic execution scheduling remains owned by Section D. |
| `L2-G03` Hostel | Section C | Read model | Student Identity. |
| `L2-G04` Transport | Section C | Read model | Student Identity. |
| `L2-G04` Transport | Section F | Runtime contract | Transport Billing. |
| `L2-G05` Library | Section C | Read model | Student Identity. |
| `L2-G05` Library | Section F | Runtime contract | Finance & Payments. |
| `L2-G06` Health Services | Section C | Read model | Student Identity. |
| `L2-G06` Health Services | Section F | Runtime contract | Medical Billing. |
| `L2-G07` Security | Section C | Read model | Student Identity. |
| `L2-G07` Security | Section F | Runtime contract | Accounting and payment consequences of Security Vendor Contracts. Supplier and procurement operations belong to Section J. |
| `L2-G08` Housekeeping | Section F | Runtime contract | Accounting and payment consequences of Vendor Contracts. Supplier and procurement operations belong to Section J. |
| `L2-G09` Teacher Guardian Scheme | Section C | Read model | Student master, Student profile, Student admission, Student academic lifecycle. |
| `L2-G09` Teacher Guardian Scheme | Section D | Read model | Academic Session, TGS Timetable Schedule, Attendance Summary, Academic Performance Summary. Academic remains canonical owner of scheduling and assignment. |
| `L2-G09` Teacher Guardian Scheme | Section F | Read model | Fee Summary, Outstanding Amount, Installment Status, Financial Hold Status. Financial ownership remains with Finance. |
| `L2-G07` Security | Section J | Runtime contract | Security Vendor Contracts — supplier and procurement operations (`L2-G07` line 2162). Accounting and payment consequences route to Section F, above. |
| `L2-G08` Housekeeping | Section J | Runtime contract | Vendor Contracts — supplier and procurement operations (`L2-G08` line 2080). Accounting and payment consequences route to Section F, above. |
| `L2-C04` Academic Progression | `L2-H10` External Education Integration | Runtime contract | Verified external credit-transfer request, provider evidence and mapping context. C04 alone owns equivalency, academic approval/rejection and the Academic Credit Ledger. |
| `L2-H10` External Education Integration | `L2-C04` Academic Progression | Event consumption | Authoritative `CreditTransferRecognised` or `CreditTransferRejected` decision reference, consumed only to synchronize the external request and provider boundary. |
| `L2-H07` Enterprise Dashboard | Sections F and G | Read model | Financial and campus-operational reports, KPIs and governed projections. H07 owns dashboard composition; source domains retain fact, computation, report and KPI ownership. |
| `L2-H09` Accreditation and Compliance | `L2-G00` Campus Operations Review | Runtime contract | Campus operational governance evidence, readiness reports and review history used as accreditation evidence. H09 owns the institutional accreditation-readiness decision. |

> **Section G entries added 2026-08-06 under integrity finding INT-202.** Ten Section G
> documents declared Sections C, D and F under a `**Upstream Dependencies**` heading,
> against a foundational set of A, B and E at section 6. Because section 6 also makes
> Section D dependent on Section G, the Section D declarations formed a **D ↔ G cycle**,
> contrary to the acyclicity requirement in section 1.
>
> Each declaration was checked against the document's Appendix B Canonical Ownership
> Matrix and body text. All of the entries above resolve to ownership routing — "another
> section owns this concept, we consume it" — so they are recorded here as runtime
> dependencies and have been moved to a `**Consumed From**` block in each source file.
> **Section G's foundational set at section 6 is unchanged, and the D ↔ G cycle is
> resolved.**
>
> Three declarations were deleted rather than reclassified: `L2-G04`, `L2-G05` and
> `L2-G06` each listed Section D upstream while containing no Academic Delivery
> consumption anywhere in the document. They were boilerplate.
>
> Not yet reconciled: several Section G ownership matrices route concepts to **Section H**
> (Notifications) and **Section J** (supplier and procurement operations) without
> declaring either in any dependency block. Those routings are real but undeclared, and
> are left for a later round.

New entries shall be added here whenever a section declares a dependency that is not
foundational. A section listing another section under "Depends On" without a matching
entry in Section 6 is a defect in that section, not a new foundational dependency.

### Section H notification routing — registered exemption

> Added 2026-08-06. Nine Section G documents route Notifications to Section H in their
> Appendix B ownership matrices (`L2-G00` line 1777, `G01` 2987, `G02` 2340, `G03` 2457,
> `G04` 2384, `G05` 2364, `G06` 2165, `G07` 2165, `G08` 2083) without declaring Section H
> in any dependency block. This is **not** a defect: section 5 of this document states
> that cross-cutting services exposed by Section H do not, by themselves, create a
> foundational upstream dependency for every consuming section. Notification routing is
> therefore exempt from the register. The same exemption applies to any section routing
> notifications to Section H.

## 7B. Known Missing Bounded Contexts

> Added 2026-08-06. A **missing bounded context** is one that L2 documents depend on by
> name but that no document in this repository defines, owns, or assigns to a section. It
> is not a naming error to be corrected by renaming — the capability genuinely does not
> exist in the architecture. Recorded here so the gap is visible repository-wide rather
> than buried in the consuming documents.

| Missing context | Depended on by | What is consumed | Status |
|---|---|---|---|
| **Policy Management** | `L2-C08` (10 references), `L2-C04` (3 references) | Institutional business policy definitions and their change events — `AlumniTransitionPolicyUpdated` (C08), `AcademicPolicyUpdated` (C04), and a `Get Academic Policy` query contract (C04). | **Resolved 2026-08-15 (RC-03).** Institutional policy is owned by `L2-B01`. Publishers assigned: `AcademicPolicyUpdated` → `L2-C04`, `AlumniTransitionPolicyUpdated` → `L2-C08`. |

**Why no owner has been assigned.** The three candidate homes were each checked and
excluded on the evidence:

- **The consuming domain itself.** `L2-C08` line 5037 states "Policy ownership remains
  external to the Alumni Transition Aggregate," and line 3754 names "the canonical Policy
  Management bounded context" as the source. C08 evaluates policy and publishes
  `TransitionPolicyValidated`; it does not define policy.
- **`L2-A04` Authorization & Policy.** Its stated purpose is access control — authorization
  decisions, permissions, roles and policy *evaluation* — and it is explicitly
  "independent from business domain implementations." It governs who may act, not what
  institutional rules apply.
- **`L2-A08` Module Registry & Configuration.** Its section 18 policy material governs
  "architectural metadata only" and explicitly excludes implementation, runtime and
  operational concerns. It is architectural governance policy, not business policy.

- **`L2-B01` Institution.** **Not evaluated when this gap was first recorded.** On review it
  is the owner. `L2-B01` section 2.1 *Approved Institution Policy Ownership Clarification*
  cites `SutraERP_L1_Business_Domain_Architecture_Locked.md`, which states in the Institution
  domain business rules that **"Institute owns institutional policies."** Institution Policy is
  a canonical entity of that domain. The same section separates it explicitly from
  authorization policy — "`L2-A04` owns the Authorization Policy" — and records that `L2-A00`
  states the Platform Foundation "does not own institutional policies."

**Resolution, approved 2026-08-15 (RC-03).** Institutional business policy is owned by
`L2-B01` Institution, on the authority of the locked L1 Business Domain Architecture. The
three exclusions above remain correct; the gap arose because B01 was not among the candidate
homes originally checked. Under the L3 Phase 1 authority hierarchy the Locked L1 Architecture
ranks above this Dependency Map, so this entry is a documentation correction bringing the
register into line with L1 — not a new architecture decision.

The two orphaned events are assigned to their own domains, following the convention already
established for qualified policy capabilities below: `AcademicPolicyUpdated` is published by
`L2-C04` and `AlumniTransitionPolicyUpdated` by `L2-C08`. Each domain publishes change events
for the policy it applies, while the institutional policy definition itself remains B01's.
The `Get Academic Policy` query contract is served by `L2-B01`.

The prior rule barring subscribers and query clients against Policy Management is lifted.
Consuming documents carrying that instruction at the point of use should be updated.

Note that *qualified* policy capabilities are owned and were never part of this gap:
`Examination Policy Management` is `L2-I01`, `Leave Policy Management` is `L2-E05`, and
`Eligibility Policy Management` is a workflow within `L2-C07`.

---

## 8. Dependency Rules

1. Dependencies are one-directional at the architectural level.
2. Upstream documents define architecture.
3. Downstream documents consume architecture.
4. Downstream documents must never redefine upstream decisions.
5. Every business concept has exactly one canonical owner.
6. Runtime events do not transfer ownership.
7. Business scenarios must reference the owning domains instead of duplicating domain workflow logic.
8. Any dependency change must be reviewed before implementation.
9. L3 Business Scenario Architecture is independent of L2 numbering.
10. An L3 business scenario may derive information from one or many L2 workflow documents.

---

## 9. Future Expansion Rules

- Every new L1 or L2 document must declare upstream dependencies.
- Multi-domain orchestration belongs in L3 Business Scenario Architecture.
- L4 defines system behavior without redefining business logic.
- L5 derives technical constraints from L4.
- L6 derives user interaction specifications from L5.
- New sections or domains must update this document before architecture approval.

---

## 10. Dependency Change Policy

Whenever a new architectural dependency is introduced:

1. Update the affected L1/L2 document.
2. Update this Dependency Map.
3. Validate upstream and downstream impacts.
4. Review architectural consistency.
5. Obtain Lead Architect approval before locking the document.

---

**Project:** SutraERP

**Document:** Dependency Map

**Status:** Active

**Document Owner:** Dhruv

**Maintained By:** Dhruv (Lead Architect)

**Last Updated:** 2026-08-05
