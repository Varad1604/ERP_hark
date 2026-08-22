# Student Academic Assignment Change

**Scenario ID:** L3-C-009  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** C — Student Lifecycle; D — Academic Delivery  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L2-C02 — Student Lifecycle & Activation Workflows; applicable L2-D academic-structure authority; L2-C04 Programme Migration where Programme association changes  
**Referenced By:** None yet

---

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-009` |
| Canonical Title | Student Academic Assignment Change |
| Primary Owner | Student Lifecycle |
| Participating Domains | C, D |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |
| Applicable L2 Workflow | `WF-C02-04 — Academic Assignment` |
| Canonical Student Lifecycle Owner | Student Lifecycle |
| Canonical Successful Event | `AcademicAssignmentCompleted` |
| Canonical Publisher | Student Lifecycle |

The frozen Section C canonical register identifies C-009 as **Student Academic Assignment Change**, owned by **Student Lifecycle**, with participating domains **C, D**, Type **Cross-domain**, Priority **P1**, and derivation **KEEP**. fileciteturn102file0L2-L2

The Phase 5 workload allocation assigns the complete C `001–075` block to Varad, so C-009 is within the current author's locked workload allocation. fileciteturn111file0L2-L2

The Phase 5 permanent placement standard requires Section C scenarios to reside under `01_Scenario_Documents/section-c/`, with the exact Phase 5 filename determined by the operating guide and naming standard. fileciteturn108file0L2-L2

**Exact Phase 5 filename:** `L3-C-009_Student_Academic_Assignment_Change.md`

---

## 2. Scenario Purpose

This scenario governs a change to the academic assignment of an active Student while preserving Student Lifecycle ownership of the Student and StudentContext.

The scenario establishes the institutional outcome in which the Student's applicable academic placement is updated against valid academic structure, with historical assignment preserved and the completed assignment change recorded and published as the canonical business outcome.

The scenario covers academic assignment rather than academic curriculum ownership, course registration, Programme Migration decision-making, or general institutional transfer.

The locked L2-C02 architecture identifies Academic Assignment as the workflow responsible for assigning Programme, Batch and Section, and explicitly states that Student Lifecycle owns assignment while Academic Delivery owns curriculum. fileciteturn115file0L2-L2

---

## 3. Business Outcome

The singular business outcome is:

> **The Student's authorised academic assignment has been completed against valid academic structure, the previous assignment has been preserved as history, required audit evidence has been recorded, and `AcademicAssignmentCompleted` has been published by Student Lifecycle.**

Successful completion means that the applicable academic assignment information in the Student's institutional context reflects the completed change.

The outcome does **not** mean that Student Lifecycle has acquired ownership of academic curriculum, course offerings, teaching, or other Academic Delivery-owned business capabilities.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an academic assignment change is raised through one of the authoritative business triggers for `WF-C02-04`:

- Student Activation;
- Administrative Assignment;
- Academic Reassignment; or
- `ProgrammeMigrationApproved` from L2-C04, where the workflow is applying an already-approved Programme association change. fileciteturn99file0L2-L2

## 4.2 End Boundary

The scenario ends when:

1. the applicable academic assignment has been completed;
2. historical assignment has been preserved;
3. the business change has been audited; and
4. `AcademicAssignmentCompleted` has been published.

The Main Business Flow does not continue into downstream academic operations merely because those domains may consume the completed assignment.

## 4.3 In Scope

- Academic assignment change for an active Student.
- Validation against applicable academic structure.
- Programme assignment where permitted by the authoritative migration boundary.
- Batch assignment.
- Section assignment.
- Preservation of previous assignment.
- Cross-domain participation with Academic Delivery.
- Audit evidence.
- Publication of `AcademicAssignmentCompleted`.

## 4.4 Out of Scope

The following are materially adjacent but distinct:

- Student Enrollment Update — `L3-C-008`.
- Programme Migration decision-making owned by L2-C04 APW-08.
- Student Status Change.
- Student Suspension.
- Student Restoration.
- Student Transfer between non-Programme organisational or placement dimensions.
- Academic Registration.
- Curriculum definition or ownership.
- Course or course-offering ownership.
- Teaching delivery.
- Attendance.
- Examination processing.
- Technical APIs, databases, services, queues, infrastructure, UI, technical authorization or implementation mechanics.

The locked L2-C02 architecture explicitly separates Academic Assignment from Enrollment Update and Student Transfer, and separately assigns Programme-changing mobility to L2-C04. fileciteturn98file0turn99file0

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by / Related | `L3-C-008 — Student Enrollment Update` | Enrollment information and academic assignment are adjacent Student Lifecycle capabilities but remain distinct outcomes. |

No other final L3 ID is asserted here where the canonical final-ID mapping was not directly retrieved.

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| **C — Student Lifecycle** | Canonical owner of Student, StudentContext, academic assignment, assignment change, historical assignment, and final `AcademicAssignmentCompleted` outcome. |
| **D — Academic Delivery** | Participates through the academic structure and context required for valid assignment; does not acquire ownership of Student or StudentContext lifecycle information. |

The locked L2-C02 ownership model explicitly identifies Programme Association, Batch Assignment and Section Assignment as Student Lifecycle-owned capabilities and states that Student Lifecycle is the exclusive owner of the Student Aggregate and associated StudentContext. fileciteturn115file0L2-L2

The same architecture states that Academic Delivery owns curriculum while Student Lifecycle owns academic assignment. fileciteturn115file0L2-L2

Participation by Academic Delivery therefore does not transfer ownership.

---

## 6. Business Actors, Roles and Responsibilities

The retrieved authoritative L2 material identifies business triggers and domain responsibilities but does **not** define a canonical human role title for each Academic Assignment trigger.

Accordingly, no unsupported role name is invented.

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student Lifecycle | Owns and performs the Student academic-assignment lifecycle outcome. | Canonical owning domain. |
| Academic Delivery | Provides or participates in the academic-structure context required for assignment. | Participating domain; does not own Student assignment lifecycle. |
| Authorised business requester / institutional role | Initiates or causes an administrative or academic reassignment where permitted. | Exact canonical human role is not defined by the retrieved WF-C02-04 source. |

The absence of a canonical human-role definition is recorded as a non-blocking Section 17 issue rather than converted into a new role.

---

## 7. Preconditions and Entry Criteria

The authoritative `WF-C02-04 — Academic Assignment` defines the following preconditions:

- **Student Active**
- **Programme exists**
- **Batch exists**
- Where an existing Programme association will change, the matching `ProgrammeMigrationApproved` result from L2-C04 has been consumed. fileciteturn99file0L2-L2

Therefore C-009 begins only when:

1. the Student exists;
2. the Student is Active;
3. the required Programme reference exists where Programme assignment is involved;
4. the required Batch reference exists where Batch assignment is involved;
5. the required academic structure is valid;
6. a Programme change, if applicable, is backed by the authoritative L2-C04 migration outcome; and
7. the assignment request is otherwise valid under the governing institutional policy.

No additional local precondition is invented.

---

## 8. Trigger

The authoritative triggers for Academic Assignment are:

- **Student Activation**
- **Administrative Assignment**
- **Academic Reassignment**
- **`ProgrammeMigrationApproved` from L2-C04**, specifically for application of an already-approved Programme association. fileciteturn99file0L2-L2

The trigger is expressed at business level.

It is not an API call, message mechanism, database operation, scheduled job, or other implementation construct.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the valid academic-assignment change or applicable activation/assignment condition. | Student Lifecycle | Academic assignment processing begins. |
| 2 | Validate the Student and confirm the Student is Active. | Student Lifecycle | Eligible Student is established. |
| 3 | Validate the relevant academic structure. | Student Lifecycle with Academic Delivery participation | Required academic references are established as valid. |
| 4 | Where Programme association is changing, confirm that the authoritative `ProgrammeMigrationApproved` outcome from L2-C04 has been received and is being applied rather than re-decided. | Student Lifecycle | Approved Programme change is available for application. |
| 5 | Apply the Programme assignment where applicable. | Student Lifecycle | Student's Programme association reflects the authorised assignment. |
| 6 | Apply the Batch assignment where applicable. | Student Lifecycle | Student's Batch association reflects the authorised assignment. |
| 7 | Apply the Section assignment where applicable. | Student Lifecycle | Student's Section association reflects the authorised assignment. |
| 8 | Preserve the previous academic assignment as historical assignment information. | Student Lifecycle | Assignment history remains preserved. |
| 9 | Record the required audit evidence for the completed assignment change. | Student Lifecycle | The business change is auditable. |
| 10 | Publish `AcademicAssignmentCompleted`. | Student Lifecycle | The canonical completed assignment business fact is established. |
| 11 | End the scenario. | Student Lifecycle | Academic Assignment Change is complete. |

The authoritative L2 behaviour is:

```text
Assignment Request
        ↓
Validate Academic Structure
        ↓
Assign Programme
        ↓
Assign Batch
        ↓
Assign Section
        ↓
Publish AssignmentCompleted
        ↓
Audit
        ↓
Complete
```

The L2 contract names the canonical event as `AcademicAssignmentCompleted`. fileciteturn99file0L2-L2

### Scenario End Discipline

C-009 stops when the academic assignment has been completed and the canonical completion fact has been published.

It does not extend into course registration, teaching allocation, attendance, examination, finance, hostel, library, transport, placement, or other downstream consequences.

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| AF-01 | Student activation requires academic assignment | Apply the Academic Assignment workflow as part of the activation-related business sequence. | Required academic assignment is established for the active Student. | Student Lifecycle |
| AF-02 | Administrative Assignment | Process the authorised assignment against valid academic structure. | Academic assignment completed. | Student Lifecycle |
| AF-03 | Academic Reassignment | Validate the requested reassignment and apply the valid academic assignment. | Updated assignment completed with history preserved. | Student Lifecycle |
| AF-04 | Approved Programme Migration | Consume/apply `ProgrammeMigrationApproved` without re-evaluating or republishing the migration decision. | Approved Programme association is applied as part of academic assignment. | Student Lifecycle; L2-C04 owns migration decision |

The four trigger conditions are grounded in the authoritative `WF-C02-04` definition. fileciteturn99file0L2-L2

The Programme Migration boundary is especially explicit: L2-C04 APW-08 owns Programme-changing mobility, while C02 applies the approved Programme association and does not make or republish the migration decision. fileciteturn96file0L2-L2

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Student is not Active | Academic assignment cannot satisfy the authoritative precondition. | Do not complete C-009 until the Student is in the required lifecycle position under the applicable upstream process. | Student Lifecycle |
| Required academic structure is invalid or unavailable as a valid business reference | Assignment cannot be validly completed. | Correct or establish the applicable academic structure under its canonical owner before assignment proceeds. | Student Lifecycle / Academic Delivery |
| Programme association is being changed without the required authoritative migration outcome | Programme-changing assignment cannot be completed through C-009. | Obtain the authoritative `ProgrammeMigrationApproved` outcome through L2-C04's Programme Migration process; C-009 then applies it. | L2-C04 / Student Lifecycle |
| Requested change is actually Programme Migration decision-making | C-009 must not make the migration decision. | Route the Programme-changing mobility decision to L2-C04 APW-08. | Academic Progression |
| Requested change is actually non-Programme organisational/placement transfer | C-009 must not absorb a distinct transfer scenario where the canonical boundary places it elsewhere. | Apply the appropriate Student Transfer workflow. | Student Lifecycle |

The L2 source explicitly requires Programme to exist and requires `ProgrammeMigrationApproved` when an existing Programme association changes. fileciteturn99file0L2-L2

### Technical Failure Boundary

L3 does not define technical retries, queues, rollback, server failures, database recovery, business exception workflows, timeout handling, or infrastructure recovery.

No such mechanics are introduced here.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Student must be Active for Academic Assignment. | Student Lifecycle | Assignment cannot proceed for an ineligible lifecycle position. | `WF-C02-04` Preconditions |
| Programme must exist where Programme assignment is required. | Student Lifecycle / applicable academic structure authority | Programme assignment must reference valid academic structure. | `WF-C02-04` Preconditions |
| Batch must exist where Batch assignment is required. | Student Lifecycle / applicable academic structure authority | Batch assignment must reference valid academic structure. | `WF-C02-04` Preconditions |
| A Programme association change requires the authoritative `ProgrammeMigrationApproved` result. | Academic Progression / Student Lifecycle application boundary | C-009 may apply an approved migration but may not make the migration decision. | `WF-C02-04` Preconditions and Governance |
| Assignment must reference valid academic entities. | Student Lifecycle | Invalid academic references cannot produce a completed assignment. | `WF-C02-04` Governance |
| Historical assignment records are preserved. | Student Lifecycle | Reassignment does not erase prior assignment history. | `WF-C02-04` Governance |
| Every reassignment generates an audit record. | Student Lifecycle | Assignment changes remain auditable. | `WF-C02-04` Governance |
| The Programme migration decision remains C04-owned. | Academic Progression | C-009 applies, but does not re-evaluate or republish, the migration decision. | Locked C02 programme-migration boundary |

The authoritative L2 governance explicitly requires valid academic references, historical assignment preservation, and audit for reassignment. fileciteturn99file0L2-L2

No local business-rule identifiers are created.

---

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| StudentContext academic assignment | Student Active | Academic Programme/Batch/Section association is changed or established. | Updated academic assignment within the existing StudentContext lifecycle. | Student Lifecycle | L2-C02 |
| Student lifecycle status | Active | C-009 does not change Student status as its business outcome. | Active, unless a separate canonical scenario changes status. | Student Lifecycle | L2-C02 |
| Programme association | Existing Programme association where applicable | Applies an authorised Programme association change after `ProgrammeMigrationApproved`. | Updated Programme association. | Student Lifecycle for assignment; L2-C04 for migration decision | L2-C02 |
| Batch assignment | Existing/required academic assignment | Applies the valid Batch assignment. | Updated Batch assignment. | Student Lifecycle | L2-C02 |
| Section assignment | Existing/required academic assignment | Applies the valid Section assignment. | Updated Section assignment. | Student Lifecycle | L2-C02 |

C-009 does **not** invent a new Student lifecycle state.

The locked L2-C02 architecture distinguishes the Student persona from the StudentContext, with StudentContext owning the institutional academic lifecycle while Student Lifecycle remains the canonical owner. fileciteturn115file0L2-L2

The Programme migration boundary remains separate: C04 owns the Programme-changing decision and canonical `ProgrammeMigrationApproved` outcome; C02 applies the approved Programme association. fileciteturn96file0L2-L2

---

## 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AcademicAssignmentCompleted` | **Student Lifecycle** | Downstream interested academic/administrative domains as governed by their architecture | Records completed academic assignment. | `WF-C02-04` |
| `ProgrammeMigrationApproved` | **L2-C04 Academic Progression** | Student Lifecycle | Provides the authoritative Programme migration decision for C02 to apply. | Locked C02 programme-migration boundary |

The L2-C02 workflow explicitly publishes `AcademicAssignmentCompleted`. fileciteturn99file0L2-L2

The same locked architecture explicitly identifies `ProgrammeMigrationApproved` as an event consumed by C02 for Programme-association application only; the migration decision remains C04-owned. fileciteturn96file0L2-L2

## 14.2 Synchronous Business Contracts / Hand-offs

The business hand-off is:

> **Academic Delivery provides the applicable academic-structure context required for Student Lifecycle to perform valid academic assignment; Student Lifecycle retains ownership of the Student and StudentContext and produces the completed assignment outcome.**

No technical protocol, API, payload, queue, or service is defined at L3.

## 14.3 Ownership Preservation

Cross-domain collaboration does not transfer:

- Student ownership;
- StudentContext ownership;
- academic assignment ownership; or
- event publication ownership.

Academic Delivery's participation is therefore collaborative rather than ownership-transferring.

## 14.4 Event Boundary

`AcademicAssignmentCompleted` is a business event representing a completed institutional fact.

It is not a technical integration mechanism.

---

## 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

The retrieved L2-C02 Academic Assignment definition identifies business triggers including Administrative Assignment and Academic Reassignment but does not define a canonical human-role title or detailed authorization matrix for those triggers.

Therefore C-009 records the existence of the governed business authorization requirement without inventing a technical or institutional role name.

## 15.2 Approval

Where Programme association is changing, the authoritative `ProgrammeMigrationApproved` result from L2-C04 is required.

This is not a C-009 approval decision.

C-009 applies the authoritative C04 result and does not re-evaluate or republish the migration decision. fileciteturn99file0L2-L2

No additional approval chain is invented.

## 15.3 Audit

The locked L2 governance requires every reassignment to generate an audit record.

C-009 therefore requires auditable evidence of the completed academic assignment change. fileciteturn99file0L2-L2

## 15.4 Separation of Duty

No scenario-specific separation-of-duty rule is explicitly defined in the retrieved WF-C02-04 material.

No local separation-of-duty rule is invented.

## 15.5 Historical Evidence

Previous assignment records must be preserved.

The change is therefore not represented as an erasure of the prior academic assignment.

---

## 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

C-009 is complete when:

1. the Student is Active;
2. the applicable academic structure has been validated;
3. Programme, Batch and Section assignments applicable to the request have been completed;
4. any Programme association change is backed by the authoritative `ProgrammeMigrationApproved` result;
5. the previous academic assignment has been preserved;
6. the assignment change has been audited; and
7. `AcademicAssignmentCompleted` has been published by Student Lifecycle.

## 16.2 Postconditions

| Postcondition | Result |
|---|---|
| Student | Remains owned by Student Lifecycle |
| StudentContext | Remains owned by Student Lifecycle |
| Student lifecycle status | No C-009 status transition is introduced |
| Academic assignment | Updated as applicable |
| Programme | Updated only where authorised; migration decision remains C04-owned |
| Batch | Updated as applicable |
| Section | Updated as applicable |
| Previous assignment | Preserved as history |
| Audit | Completed |
| `AcademicAssignmentCompleted` | Published by Student Lifecycle |
| Academic Delivery | Participates without acquiring Student assignment ownership |

## 16.3 Downstream Consequence Boundary

Downstream academic domains may use the completed assignment outcome according to their own authoritative architecture.

Those downstream consequences are not extended into the C-009 Main Business Flow.

## 16.4 End Boundary

The scenario ends at the completed academic assignment outcome and publication of the canonical completion event.

---

## 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

### OI-C009-01 — Exact Human Actor / Institutional Role

The authoritative retrieved `WF-C02-04` identifies triggers such as Administrative Assignment and Academic Reassignment but does not define the exact canonical human role responsible for initiating, approving, or performing each trigger.

**Impact:** Non-blocking for the L3 business outcome because the owning domain and business responsibility are defined, but the human-role mapping cannot be safely made more specific.

**Disposition:** Preserve the business responsibility without inventing a role title. Resolve through the authoritative upstream actor/role architecture before Architecture Approved/Locked if a role-specific authorization decision is required.

---

### OI-C009-02 — Exact Academic Structure Authority in Domain D

The C-009 frozen semantics identify Domain D as a participating domain, and L2-C02 identifies Academic Delivery as an external participant in Academic Assignment. The retrieved C02 workflow does not enumerate every Domain D-owned academic-structure rule or exact collaboration contract.

**Impact:** Non-blocking for the C02-owned assignment outcome.

**Disposition:** Academic Delivery participation is retained without inventing specific D-side events, contracts, actors, or ownership.

---

### OI-C009-03 — Exact Attribute-Level Change Matrix

The retrieved `WF-C02-04` identifies Programme, Batch and Section assignment but does not provide a complete authoritative matrix describing which assignment attributes may change under each institutional circumstance.

**Impact:** Non-blocking at L3 business-scenario level.

**Disposition:** C-009 remains at the canonical business capability level and does not invent an attribute-level policy.

---

### OI-C009-04 — Programme Migration Boundary Dependency

The C02 architecture contains an explicit locked Programme Migration boundary in which L2-C04 owns Programme-changing mobility and C02 applies `ProgrammeMigrationApproved`.

**Impact:** No conflict remains for C-009.

**Disposition:** Treat C04 as the decision owner and C02 as the assignment-application owner. C-009 must not recreate the migration decision.

---

## 17.2 Assumptions

1. The frozen Stage 3 C-009 semantics are authoritative.
2. C-009 is owned by Student Lifecycle.
3. Domain D is a material participating domain.
4. `WF-C02-04 — Academic Assignment` is the applicable locked L2 workflow.
5. Student must be Active.
6. Required academic references must exist and be valid.
7. Historical assignment is preserved.
8. Audit is mandatory for reassignment.
9. `AcademicAssignmentCompleted` is the canonical completion event published by Student Lifecycle.
10. Programme Migration decision-making remains outside C-009.
11. `ProgrammeMigrationApproved` is consumed only as an authoritative approved result where Programme association changes.
12. No new Student lifecycle state is created by C-009.

These assumptions do not override locked architecture.

---

## 17.3 Deferred Questions

| Question | Required Authority | Reason |
|---|---|---|
| Which exact institutional roles may initiate Administrative Assignment? | Locked actor/role architecture | Not enumerated in retrieved WF-C02-04. |
| Which exact institutional roles may initiate Academic Reassignment? | Locked actor/role architecture | Not enumerated in retrieved WF-C02-04. |
| Which academic structure rules are owned by Domain D for each assignment type? | Applicable locked L2-D architecture | C02 establishes the C-side boundary but does not enumerate all D-side rules. |
| Which exact assignment attributes are mutable under each policy? | Locked L2 architecture / institutional policy | Not fully enumerated in retrieved WF-C02-04. |

## 17.4 Locking Consequence

The unresolved items above are **non-blocking for this Draft** because they do not change:

- the Final L3 ID;
- canonical title;
- Primary Owner;
- participating domains;
- Scenario Type;
- Priority;
- final business outcome;
- Student Lifecycle ownership;
- the canonical `AcademicAssignmentCompleted` publisher; or
- the Programme Migration ownership boundary.

They must not be silently resolved in this L3 document.

---

# Validation and Review Classification

## Repository Identity Check

- **Final ID:** `L3-C-009`
- **Title:** Student Academic Assignment Change
- **Primary Owner:** Student Lifecycle
- **Participating Domains:** C, D
- **Type:** Cross-domain
- **Priority:** P1
- **Assigned Author:** Varad
- **Section Folder:** `section-c`
- **Exact Filename:** `L3-C-009_Student_Academic_Assignment_Change.md`

The frozen Stage 3 register confirms the C-009 semantic identity and allocation characteristics. fileciteturn102file0L2-L2

## Applicable L2 Check

`WF-C02-04 — Academic Assignment` defines the purpose as assigning Programme, Batch and Section; its triggers include Student Activation, Administrative Assignment, Academic Reassignment, and `ProgrammeMigrationApproved`; its preconditions include Student Active, Programme exists, Batch exists, and the required migration approval for Programme changes. fileciteturn99file0L2-L2

## Workflow Check

The authoritative business sequence is:

```text
Assignment Request
        ↓
Validate Academic Structure
        ↓
Assign Programme
        ↓
Assign Batch
        ↓
Assign Section
        ↓
Publish AcademicAssignmentCompleted
        ↓
Audit
        ↓
Complete
```

fileciteturn99file0L2-L2

## Ownership Check

Student Lifecycle owns academic assignment.

Academic Delivery participates but does not acquire Student or StudentContext ownership. fileciteturn115file0L2-L2

## Programme Migration Check

C-009 does not make the Programme Migration decision.

L2-C04 owns Programme-changing mobility and its `ProgrammeMigrationApproved` outcome; C02 applies the approved Programme association without re-evaluating or republishing the migration decision. fileciteturn96file0L2-L2

## Event Check

Canonical completion event:

`AcademicAssignmentCompleted`

Canonical publisher:

**Student Lifecycle**. fileciteturn99file0L2-L2

## Rule Check

The scenario preserves the authoritative C02 constraints:

- valid academic entities;
- historical assignment preservation;
- audit for reassignment;
- approved Programme migration result where applicable. fileciteturn99file0L2-L2

## Boundary Check

C-009 does not absorb:

- Student Enrollment Update;
- Programme Migration;
- Student Transfer;
- Student Status Change;
- Academic Registration;
- curriculum ownership.

## Implementation Leakage Check

No APIs, payloads, database schemas, Business Data Structures, services, queues, infrastructure, technical access control/IAM, UI design, technical retries, rollback, business exception workflows, or deployment mechanics are introduced.

## Template Check

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

The locked v2.0 template defines this exact 17-section structure and requires Draft as an allowed initial status. fileciteturn113file0L2-L2

---

# Final Classification

**PASS WITH ISSUES — Draft**

C-009 is source-grounded and complete at L3 business-scenario level.

The issues are explicitly limited to upstream details that the retrieved authoritative sources do not enumerate, principally exact human-role mapping, detailed Domain D participation, and attribute-level assignment policy.

No unresolved issue currently changes the frozen identity, ownership, outcome, event publisher, or major scenario boundary.

**Status remains `Draft`.**

AI authoring is not Architecture Approval or Locking.
