# Intra-Institute Student Academic Reassignment

**Scenario ID:** L3-C-012  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** B — Institution & Structure; C — Student Lifecycle  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-C02 — Student Lifecycle & Activation Workflows; L2-B02 — Organisation Workflows; applicable authoritative Institution & Structure / academic-context workflows; frozen Stage 3 Section C canonical semantics  
**Referenced By:** Future L4 System Behavior Specification derived from this scenario

---

The frozen Section C Stage 3 register defines C-012 as **Intra-Institute Student Academic Reassignment**, with **Student Lifecycle** as Primary Owner, **B, C** as participating domains, **Cross-domain** as Type, **P1** as Priority, and `KEEP` derivation.

The Phase 5 operating guide assigns C-012 to **Varad** and fixes the exact filename as:

`L3-C-012_Intra_Institute_Student_Academic_Reassignment.md`

The repository operating guide requires authors to use the exact assigned ID/filename, frozen Type/Priority, applicable L2 sources, and the locked v2.0 17-section structure.

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-C-012 |
| Canonical Title | Intra-Institute Student Academic Reassignment |
| Primary Owner | Student Lifecycle |
| Participating Domains | B — Institution & Structure; C — Student Lifecycle |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs an authorised **same-Institute, non-Programme** reassignment of a Student's academic organisational or placement context.

It exists to distinguish an actual Student Transfer workflow from ordinary Academic Assignment maintenance. The locked cross-domain resolution explicitly states that C-009 and C-012 may affect overlapping references such as Department, Batch, or Section, but their business outcomes are different: C-009 remains assignment maintenance, while C-012 is the authorised same-Institute non-Programme relocation processed through the Student Transfer workflow.

---

## 3. Business Outcome

The singular institutional outcome is:

> **The Student has been validly reassigned within the same Institute through the governed non-Programme Student Transfer workflow, the new organisational or placement context is authoritative, the prior assignment is preserved as history, the Student passes through the governed `Transferred` lifecycle position and returns to `Active`, and `StudentTransferred` is published by Student Lifecycle.**

The scenario does **not** change the Student's Programme.

Programme-changing mobility is explicitly outside C-012 and remains owned by L2-C04 Academic Progression through APW-08 and its `ProgrammeMigrationApproved` outcome.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a governed **Student Transfer Request** or **Administrative Approval** initiates a same-Institute Student Transfer.

The requested reassignment must be a **non-Programme organisational or placement relocation**.

## 4.2 End Boundary

The scenario ends when:

1. the transfer request/approval is accepted as authoritative;
2. the Student's new same-Institute organisational or placement context is established;
3. the prior assignment is preserved;
4. the Student enters the governed `Transferred` lifecycle position;
5. the Student returns to `Active`; and
6. `StudentTransferred` is published by Student Lifecycle.

The Main Business Flow stops at this completed transfer outcome.

## 4.3 In Scope

- Same-Institute Student Transfer.
- Non-Programme organisational reassignment.
- Non-Programme placement reassignment.
- Student Transfer Request.
- Administrative Approval.
- Validation of the transfer context.
- Preservation of the prior assignment.
- Application of the authorised new assignment.
- `Transferred` → `Active` lifecycle progression.
- Publication of `StudentTransferred`.
- Cross-domain participation by Institution & Structure.

## 4.4 Out of Scope

- Ordinary Academic Assignment maintenance — C-009.
- Programme-changing Student Programme Migration — C-033 / L2-C04 APW-08.
- Inter-Institute Student Transfer with Student Context Re-establishment — C-013.
- Academic Registration and Add/Drop.
- Teaching, timetable, attendance, examination, finance, hostel, library or transport operations.
- Any technical implementation mechanism.

## 4.5 Canonical Discriminator

The authoritative Phase 2 boundary resolution is explicit:

- **C-009:** assignment maintenance that does not invoke the Transfer workflow.
- **C-012:** an authorised same-Institute non-Programme relocation processed through the Transfer workflow.
- **Programme-changing mobility:** remains separately governed by C04 APW-08.

The distinction is therefore the **business decision and lifecycle outcome**, not merely which assignment fields change.

---

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| **B — Institution & Structure** | Provides the authoritative institutional/organisational context within which the Student is reassigned. Ownership of Organisation and Academic Structure remains with Section B. |
| **C — Student Lifecycle** | Canonical owner of Student, StudentContext, Student Academic Assignment, Student lifecycle state, the Student Transfer workflow, and the completed `StudentTransferred` business outcome. |

The locked L1 architecture assigns **Student Context**, **Academic Registration**, **Academic Progression**, and related Student lifecycle responsibilities to Student Lifecycle, while Organisation and Academic Structure remain owned by Section B.

The L2-C02 architecture states that Student Lifecycle is the exclusive owner of the Student Aggregate and associated StudentContext and that no downstream domain may directly modify Student master or StudentContext lifecycle information outside approved architecture contracts.

Participation by B therefore does **not** transfer Student or StudentContext ownership.

---

## 6. Business Actors, Roles and Responsibilities

The authoritative evidence available for C-012 establishes the initiating business conditions as:

- **Student Transfer Request**
- **Administrative Approval**.

The retrieved authoritative material does not expose a sufficiently precise actor-to-step mapping for these two initiation mechanisms. Therefore no additional actor or role is invented in this L3 document.

| Business Responsibility | Authoritative Boundary |
|---|---|
| Initiate / establish a Student Transfer Request | Must be performed through the institutionally governed transfer process. Exact actor role is not asserted here because the available authoritative C02 evidence does not define it precisely. |
| Provide Administrative Approval where required | Must be performed by the authoritative institutional approval authority. Exact role is not asserted without source support. |
| Validate and apply the Student Transfer outcome | Student Lifecycle |
| Provide/maintain authoritative organisational context | Institution & Structure |

No technical access control/technical identity profiles is introduced.

---

## 7. Preconditions and Entry Criteria

C-012 requires all of the following business conditions:

1. A valid Student and associated StudentContext exist.
2. The Student is within the same Institute for the requested reassignment.
3. The requested mobility is **non-Programme**.
4. A Student Transfer Request or Administrative Approval has initiated the transfer process.
5. The requested destination organisational or placement context is valid within the authoritative institutional structure.
6. The transfer is eligible to be processed through **WF-C02-08 Student Transfer** rather than ordinary Academic Assignment or Programme Migration.

The locked boundary resolution identifies WF-C02-08 as the workflow for an authorised same-Institute non-Programme relocation.

---

## 8. Trigger

The canonical business triggers are:

- **Student Transfer Request**
- **Administrative Approval**.

These are business triggers.

They are not represented as APIs, messages, queues, database operations, or other implementation mechanisms.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Result |
|---|---|---|---|
| 1 | Receive the Student Transfer Request or authoritative Administrative Approval. | Student Lifecycle / governed institutional process | Transfer processing begins. |
| 2 | Establish that the requested movement is within the same Institute. | Student Lifecycle with Institution & Structure context | Same-Institute transfer boundary confirmed. |
| 3 | Confirm that the requested movement is non-Programme. | Student Lifecycle | C-012 remains distinct from Programme Migration. |
| 4 | Validate the requested organisational or placement context against authoritative institutional structure. | Student Lifecycle / Institution & Structure | Destination context is established as valid. |
| 5 | Apply the governed Student Transfer outcome to the StudentContext. | Student Lifecycle | New organisational/placement association becomes authoritative. |
| 6 | Preserve the prior assignment as historical context. | Student Lifecycle | Historical assignment remains traceable. |
| 7 | Enter the governed `Transferred` lifecycle position. | Student Lifecycle | Transfer lifecycle is established. |
| 8 | Return the Student to `Active` following completion of the governed transfer outcome. | Student Lifecycle | Student is operationally Active in the new context. |
| 9 | Publish `StudentTransferred`. | Student Lifecycle | Canonical transfer fact is established. |
| 10 | End the scenario. | Student Lifecycle | Intra-Institute Student Academic Reassignment is complete. |

The locked cross-domain resolution explicitly defines WF-C02-08 as a Student Transfer workflow that performs a governed non-Programme organisational or placement relocation, enters `Transferred`, returns to `Active`, preserves the prior assignment, and publishes `StudentTransferred`.

---

## 10. Alternate Business Flows

## AF-01 — Organisational Reassignment

A Student is moved between valid organisational contexts within the same Institute without changing the Programme.

The transfer proceeds through WF-C02-08 and completes as C-012.

## AF-02 — Placement Reassignment

A Student is moved between valid non-Programme placement contexts within the same Institute.

The transfer proceeds through WF-C02-08 and completes as C-012.

## AF-03 — Administrative Approval Initiation

Where the transfer is initiated by an authoritative Administrative Approval rather than a separately raised Student Transfer Request, the approved business outcome enters the same WF-C02-08 transfer path.

## AF-04 — Request That Is Actually Programme Migration

If the requested change changes the Student's Programme, it is not an alternate C-012 path.

It is governed by L2-C04 APW-08 and the Programme Migration outcome. C02 may consume the approved `ProgrammeMigrationApproved` outcome to apply the approved Programme association, but C02 does not make or republish the Programme Migration decision.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution |
|---|---|---|
| Requested destination is not an authoritative institutional context | Transfer cannot validly complete. | Resolve the destination-context deficiency before the transfer is applied. |
| Requested movement changes Programme | C-012 is not the applicable scenario. | Route the business request through the authoritative Programme Migration process. |
| Transfer is not authorised | Transfer cannot complete. | Obtain the required authoritative approval or otherwise resolve the governed transfer request. |
| Student is not eligible for the requested transfer workflow | No transfer lifecycle transition is applied. | Resolve through the applicable Student Lifecycle or Academic Assignment process. |
| Request is ordinary assignment maintenance rather than Transfer | C-012 must not absorb the assignment change. | Use C-009 / WF-C02-04 Academic Assignment. |

No technical retry, queue, rollback, server failure, database recovery, or implementation exception mechanism is defined at L3.

---

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Effect |
|---|---|---|
| C-012 represents a governed Student Transfer, not generic assignment maintenance. | Student Lifecycle | Prevents C-009 and C-012 from being conflated. |
| The C-012 transfer is intra-Institute. | Student Lifecycle | Prevents C-012 from absorbing inter-Institute transfer. |
| The transfer is non-Programme. | Student Lifecycle / Academic Progression boundary | Programme Migration remains C04-owned. |
| WF-C02-08 is the canonical Student Transfer workflow for this outcome. | Student Lifecycle | C-012 must use the Transfer lifecycle rather than Academic Assignment maintenance. |
| The prior assignment is preserved. | Student Lifecycle | Transfer does not erase historical assignment. |
| The transfer enters `Transferred` and returns to `Active`. | Student Lifecycle | Preserves the locked lifecycle outcome. |
| `StudentTransferred` is the canonical transfer event. | Student Lifecycle | No competing transfer event is introduced. |
| Programme Migration remains exclusively owned by L2-C04 APW-08. | Academic Progression | C02 cannot decide or republish Programme Migration. |
| Runtime participation does not transfer ownership. | Repository architecture | B remains owner of its institutional/organisational concepts; C remains owner of Student lifecycle. |

The C-012 / C-009 distinction and the Programme Migration boundary are explicitly resolved in the locked Phase 2 Wave 2 decision.

---

## 13. State and Lifecycle Impact

## 13.1 StudentContext Lifecycle

The Student Transfer workflow has the following authoritative lifecycle effect:

**Current active context → `Transferred` → `Active`**

The transfer is therefore not a terminal StudentContext outcome.

The authoritative boundary resolution explicitly states that WF-C02-08 enters `Transferred`, returns to `Active`, and preserves the prior assignment.

## 13.2 Academic Assignment

The Student's non-Programme organisational or placement association changes to the authorised destination.

The previous assignment remains historical.

## 13.3 Programme

The Programme does **not** change under C-012.

Any Programme-changing mobility belongs to L2-C04 APW-08.

## 13.4 Student Ownership

Student and StudentContext remain owned by Student Lifecycle throughout.

No new StudentContext is created merely because of an intra-Institute reassignment.

---

## 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Event

**`StudentTransferred`**

**Canonical Publisher:** Student Lifecycle.

The event communicates the completed business fact that the Student Transfer workflow has completed.

## 14.2 Institution & Structure Collaboration

Institution & Structure participates by providing the authoritative institutional/organisational context relevant to the requested reassignment.

The collaboration does not transfer ownership of StudentContext or Student lifecycle state to Section B.

## 14.3 Academic Delivery Boundary

Academic Delivery may be affected by the resulting organisational/placement context, but C-012 does not perform downstream teaching, timetable, attendance, or course-delivery changes.

Any resulting academic-delivery consequences remain governed by the applicable Section D architecture.

## 14.4 Programme Migration Boundary

The `ProgrammeMigrationApproved` event is **not** a C-012 completion event.

It belongs to the C04 Programme Migration boundary and is consumed by C02 only to apply an already-approved Programme association.

---

## 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

C-012 requires the transfer to be authorised through the governed Student Transfer process.

The authoritative trigger set explicitly includes Administrative Approval.

The exact actor/approval-role mapping is not asserted because the retrieved authoritative C02 evidence does not expose enough detail to name a specific role without invention.

## 15.2 Approval

Administrative Approval is a canonical trigger for WF-C02-08.

Approval is therefore a business prerequisite where that initiation path applies.

No additional approval hierarchy is invented.

## 15.3 Audit

The locked C02 architecture requires lifecycle transitions to generate immutable audit records and states that Student lifecycle transitions publish canonical domain events.

C-012 therefore requires the completed reassignment and lifecycle transition to remain auditable.

## 15.4 Historical Evidence

The prior assignment must be preserved as history.

The authoritative C-C01 resolution explicitly states that WF-C02-08 preserves the prior assignment.

## 15.5 Separation of Duty

No additional C-012-specific separation-of-duty rule is introduced because the authoritative evidence retrieved for this scenario does not define one.

---

## 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

C-012 is complete when:

1. the Student Transfer Request or Administrative Approval is authoritative;
2. the requested movement is confirmed to be within the same Institute;
3. the requested movement is confirmed to be non-Programme;
4. the destination organisational or placement context is valid;
5. the StudentContext receives the authorised reassignment;
6. the prior assignment is preserved;
7. the Student passes through `Transferred`;
8. the Student returns to `Active`; and
9. `StudentTransferred` is published by Student Lifecycle.

## 16.2 Postconditions

| Concern | Postcondition |
|---|---|
| Student | Remains the same canonical Student |
| StudentContext | Remains owned by Student Lifecycle |
| Institute | Unchanged |
| Programme | Unchanged |
| Organisational / placement context | Updated to the authorised destination |
| Previous assignment | Preserved historically |
| Lifecycle | `Transferred` completed and Student returned to `Active` |
| Event | `StudentTransferred` published by Student Lifecycle |
| Programme Migration | Not performed |
| Inter-Institute Transfer | Not performed |

## 16.3 Downstream Consequence Boundary

Any downstream academic or operational consequences arising from the new organisational/placement context are outside C-012 once the Student Transfer outcome and `StudentTransferred` fact have been established.

C-012 does not claim ownership of downstream consequences.

## 16.4 End Boundary

The scenario ends when the Student is Active in the new authorised same-Institute non-Programme context and the canonical `StudentTransferred` event has been published.

---

## 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Exact business actor/role responsible for initiating Student Transfer Request is not exposed in the retrieved authoritative C02 evidence. | Actor/authority detail cannot be asserted without invention. | Confirm from the complete locked C02 actor/authority definitions before Architecture Approval if role-level detail is required. |
| Exact business actor/role responsible for Administrative Approval is not exposed in the retrieved authoritative C02 evidence. | Approval-role detail cannot be asserted without invention. | Confirm from the complete locked C02 actor/authority definitions before Architecture Approval if role-level detail is required. |
| Detailed Section B hand-off for the destination organisational/placement context is not explicitly exposed in the retrieved C02 material. | Detailed cross-domain hand-off wording must not be invented. | Confirm the applicable locked Section B workflow/contract definition before final approval if the hand-off must be specified at greater granularity. |

These issues do **not** alter the frozen C-012 identity, owner, participating domains, Type, Priority, scenario boundary, final outcome, lifecycle outcome, or canonical event.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| C-012 is the same-Institute non-Programme Student Transfer scenario. | Frozen Stage 3 semantics and locked C-C01 boundary resolution. | Would require architecture change control if contradicted. |
| Student Lifecycle remains the canonical owner of Student and StudentContext. | Locked L1/L2 ownership. | Would contradict locked architecture. |
| `StudentTransferred` is the canonical completion event. | Locked C-C01 boundary resolution. | Event correction would require upstream architecture review. |
| `Transferred` is followed by `Active` for the C-012 outcome. | Locked C-C01 boundary resolution. | Lifecycle correction would require upstream review. |

Assumptions do not override locked architecture.

## 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| What exact downstream business consequences are applied by Academic Delivery after `StudentTransferred`? | L3/L4 in applicable Section D scenario architecture | C-012 ends at the Student Transfer outcome and does not own downstream consequences. |
| What detailed business contract expresses the Section B organisational context required for transfer validation? | L4 | The exact contract should be derived from authoritative L2 rather than invented at L3. |

---

# Validation and Review Classification

## Repository Identity

- **Final ID:** `L3-C-012`
- **Canonical Title:** Intra-Institute Student Academic Reassignment
- **Primary Owner:** Student Lifecycle
- **Participating Domains:** B, C
- **Type:** Cross-domain
- **Priority:** P1
- **Assigned Author:** Varad
- **Exact Filename:** `L3-C-012_Intra_Institute_Student_Academic_Reassignment.md`
- **Placement:** `section-c`
- **Status:** Draft

The frozen Stage 3 row confirms all identity/classification values.

The Phase 5 workload guide confirms Varad as the assigned author and fixes the exact filename.

## L2 Boundary Validation

The authoritative Wave 2 decision establishes that:

- C-009 and C-012 are distinct despite overlapping assignment fields;
- C-012 is the authorised same-Institute non-Programme relocation;
- WF-C02-08 is the Student Transfer workflow;
- the workflow enters `Transferred`, returns to `Active`, preserves prior assignment, and publishes `StudentTransferred`;
- Programme-changing mobility remains with C04 APW-08.

## Ownership Validation

Student Lifecycle owns Student, StudentContext, Student Academic Assignment and Student lifecycle state. Institution & Structure retains ownership of its organisational and academic-structure concepts.

## Lifecycle Validation

The C-012 transfer outcome is:

**Active → Transferred → Active**

No new lifecycle state has been created.

## Event Validation

Canonical event:

**`StudentTransferred`**

Canonical publisher:

**Student Lifecycle**.

## Boundary Validation

C-012 does not absorb:

- C-009 Student Academic Assignment Change;
- Programme Migration;
- Inter-Institute Transfer;
- Academic Registration;
- Student Exit / Archive;
- downstream Academic Delivery execution.

## Implementation Independence

No API, Business Data Structure, database, service, queue, infrastructure, UI, deployment, technical access control/IAM, technical retry, rollback, or other implementation design is defined.

## Template Validation

All 17 mandatory v2.0 sections are present:

1. Scenario Metadata
2. Scenario Purpose
3. Business Outcome
4. Scenario Boundary and Scope
5. Ownership and Participating Domains
6. Business Actors, Roles and Responsibilities
7. Preconditions and Entry Criteria
8. Trigger
9. Main Business Flow
10. Alternate Business Flows
11. Failure, Interruption and Business Recovery
12. Business Rules and Decision Constraints
13. State and Lifecycle Impact
14. Business Events and Cross-Domain Collaboration
15. Business Authorization, Approval and Audit Obligations
16. Completion Criteria and Postconditions
17. Open Issues, Assumptions and Deferred Questions

The locked v2.0 template requires this exact 17-section structure and requires unresolved material upstream ambiguity to be recorded rather than silently resolved.

---

# Final Classification

**PASS WITH ISSUES — Draft**

C-012 is source-grounded and preserves the frozen canonical semantics and the locked C-C01 boundary resolution.

The remaining issues are limited to detailed actor/approval-role and Section B hand-off definitions that were not sufficiently exposed in the authoritative material retrieved for this authoring pass. They are explicitly recorded in Section 17 rather than invented.

**Status remains `Draft`.**

AI authoring is not Architecture Approval or Locking.
