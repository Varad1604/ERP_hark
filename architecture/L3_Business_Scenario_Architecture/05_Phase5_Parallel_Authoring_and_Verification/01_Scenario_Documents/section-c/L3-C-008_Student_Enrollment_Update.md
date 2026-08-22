# L3-C-008 — Student Enrollment Update

**Architecture Level:** L3 — Business Scenario Architecture  
**Scenario ID:** `L3-C-008`  
**Scenario Title:** Student Enrollment Update  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Last Updated:** 2026-08-20  
**Applicable L2 Workflow:** `WF-C02-03 — Student Enrollment Update`  
**Canonical Business Record:** Student / StudentContext Enrollment  
**Canonical Outcome:** Enrollment Updated  
**Canonical Business Event:** `EnrollmentUpdated`  
**Canonical Publisher:** Student Lifecycle

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-008` |
| Title | Student Enrollment Update |
| Primary Owner | Student Lifecycle |
| Participating Domains | C, D |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Source Phase 2 Row | `P2W-C-1057` |
| Source Phase 1 Candidate | `CAN-L3-C-008` |
| Derivation | KEEP |
| Status | Draft |
| Applicable L2 Workflow | `WF-C02-03 — Student Enrollment Update` |
| Canonical Owner | Student Lifecycle |
| Successful Outcome | Enrollment Updated |
| Canonical Event | `EnrollmentUpdated` |

The frozen Section C canonical register identifies C-008 as **Student Enrollment Update**, Primary Owner **Student Lifecycle**, Participating Domains **C, D**, Type **Cross-domain**, Priority **P1**, with derivation **KEEP**. fileciteturn91file0L2-L2

The locked L2-C02 workflow catalogue identifies `WF-C02-03 — Student Enrollment Update` as the workflow responsible for maintaining enrollment-related information throughout the student's lifecycle. fileciteturn98file0

---

## 2. Scenario Purpose

The purpose of this scenario is to maintain the authoritative enrollment information associated with an active Student while preserving the student's historical enrollment record.

The scenario provides the institutional business outcome through which a valid enrollment change, academic-session update, or authorized administrative request is evaluated and applied to the Student Lifecycle-owned enrollment information.

The scenario is intentionally limited to **enrollment information**.

It does not silently become:

- Academic Assignment;
- Programme Migration;
- Student Status Change;
- Student Transfer;
- Academic Registration; or
- any other Student Lifecycle scenario.

The locked L2-C02 architecture defines `WF-C02-03 — Student Enrollment Update` specifically for this capability. fileciteturn98file0

---

## 3. Business Outcome

The successful business outcome is:

> **The Student's authoritative enrollment information has been updated, the previous enrollment history has been preserved, required audit evidence has been recorded, and `EnrollmentUpdated` has been published.**

Successful completion establishes:

1. the Student is eligible for enrollment update;
2. the requested enrollment change is authorized;
3. the enrollment information is updated within the Student Lifecycle ownership boundary;
4. the previous enrollment history remains preserved;
5. the current enrollment remains uniquely identifiable;
6. audit evidence is recorded; and
7. `EnrollmentUpdated` represents the completed business fact.

The scenario ends at **Enrollment Updated**.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an enrollment-change business request is raised for an eligible Student.

The L2 workflow identifies these triggers:

- Enrollment Change;
- Academic Session Update; and
- Administrative Request. fileciteturn99file0

### 4.2 End Boundary

The scenario ends after:

- the enrollment information has been updated;
- the historical enrollment has been preserved;
- the completed business outcome has been audited; and
- `EnrollmentUpdated` has been published.

### 4.3 In Scope

- Enrollment change request.
- Academic session update.
- Authorized administrative enrollment update.
- Student eligibility validation.
- Enrollment information update.
- Historical enrollment preservation.
- Current enrollment uniqueness.
- Audit evidence.
- Publication of `EnrollmentUpdated`.
- Cross-domain academic context where required by the authoritative workflow.

### 4.4 Out of Scope

- Student creation.
- Student activation.
- Academic programme assignment.
- Programme migration.
- Batch/section reassignment as an independent academic assignment decision.
- Student status change.
- Suspension.
- Restoration.
- Institutional transfer.
- Academic registration.
- Course enrollment/subject registration.
- Curriculum ownership.
- Academic Delivery-owned academic structures.
- Finance-owned financial records.
- Technical APIs.
- Databases.
- Business Data Structures.
- Services.
- Queues.
- Technical retries.
- Rollback.
- Infrastructure.
- UI/UX.

### 4.5 Boundary Rule

Enrollment Update modifies enrollment information within the Student Lifecycle domain.

It does not transfer ownership of the Student Aggregate or StudentContext to Academic Delivery.

The locked L2-C02 architecture states that Student Lifecycle is the exclusive owner of the Student Aggregate and associated StudentContext, while academic domains consume Student information without owning it. fileciteturn97file0

---

## 5. Ownership and Participating Domains

### 5.1 Canonical Owner

**Student Lifecycle — Section C**

Student Lifecycle owns:

- Student Aggregate;
- StudentContext;
- enrollment lifecycle;
- enrollment information;
- enrollment history;
- enrollment update decision; and
- `EnrollmentUpdated`.

The L2 domain ownership model explicitly lists **Enrollment** as a Student Lifecycle-owned business capability. fileciteturn97file0

### 5.2 Participating Domains

The frozen C-008 scenario metadata identifies:

- **C — Student Lifecycle**
- **D — Academic Delivery**

as participating domains. fileciteturn91file0L2-L2

The L2-C02 workflow catalogue identifies **Academic Delivery** as the external participant for Enrollment Update. fileciteturn98file0

### 5.3 Academic Delivery Boundary

Academic Delivery may provide academic context/reference information relevant to enrollment.

It does not own:

- Student;
- StudentContext;
- enrollment lifecycle; or
- the `EnrollmentUpdated` business outcome.

Academic Delivery ownership of academic structures remains intact.

### 5.4 Ownership Preservation

Cross-domain participation does not transfer Student Lifecycle ownership.

No participating domain may directly modify Student Lifecycle-owned lifecycle information outside approved architecture contracts.

---

## 6. Business Actors, Roles and Responsibilities

### 6.1 Student Lifecycle Responsibility

Student Lifecycle is responsible for:

- validating the Student;
- validating the enrollment update request;
- applying the enrollment update;
- preserving historical enrollment;
- maintaining current enrollment;
- generating audit evidence; and
- publishing `EnrollmentUpdated`.

### 6.2 Authorized Requester

The L2 workflow requires an **Authorized Request** as a precondition.

The exact institutional human role is not fixed by the retrieved L2 workflow.

Therefore no unsupported role title is invented.

### 6.3 Academic Delivery Responsibility

Academic Delivery participates where academic context is required.

Its participation does not make it the owner of enrollment.

### 6.4 Student

The Student is the subject of the enrollment record.

The retrieved L2 workflow does not define a universal requirement that the Student personally initiate every enrollment update.

Therefore no Applicant/Student initiation rule is invented beyond the authoritative trigger category.

---

## 7. Preconditions and Entry Criteria

The authoritative L2 workflow establishes:

- **Student Active**
- **Authorized Request**

as preconditions. fileciteturn99file0

Accordingly, C-008 requires:

1. the Student exists;
2. the Student is Active;
3. the requester is authorized;
4. an enrollment update condition exists;
5. the requested change falls within the enrollment capability; and
6. the current enrollment can be identified.

### 7.1 Historical Integrity

Existing enrollment history must remain preserved.

### 7.2 Current Enrollment Uniqueness

The L2 governance rule states:

> **Current enrollment unique.**

Therefore C-008 must not establish contradictory simultaneous current enrollment records.

### 7.3 Lifecycle Boundary

An archived Student is not treated as an eligible active Student for this scenario because the authoritative workflow requires **Student Active**.

---

## 8. Trigger

The canonical business triggers are:

1. **Enrollment Change**
2. **Academic Session Update**
3. **Administrative Request**

These are explicitly identified by `WF-C02-03 — Student Enrollment Update`. fileciteturn99file0

The trigger starts enrollment-update processing.

It does not itself establish the completed `EnrollmentUpdated` outcome.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Responsibility | Business Result |
|---|---|---|---|
| 1 | Receive the enrollment change, academic-session update, or authorized administrative request. | Student Lifecycle | Enrollment update processing begins. |
| 2 | Validate the Student. | Student Lifecycle | Student is confirmed as eligible for processing. |
| 3 | Validate requester authorization. | Student Lifecycle / Platform participation | Request is established as authorized. |
| 4 | Determine the enrollment information affected by the request. | Student Lifecycle | Applicable enrollment information is identified. |
| 5 | Update the current enrollment information. | Student Lifecycle | Current enrollment reflects the approved change. |
| 6 | Preserve the previous enrollment information as historical enrollment. | Student Lifecycle | Enrollment history remains intact. |
| 7 | Record required audit evidence. | Student Lifecycle / Audit participation | The business change is traceable. |
| 8 | Publish `EnrollmentUpdated`. | Student Lifecycle | Completed enrollment-update business fact is established. |
| 9 | End the scenario. | Student Lifecycle | Enrollment Update is complete. |

The authoritative L2 workflow sequence is:

```text
Enrollment Request
        ↓
Validate Request
        ↓
Update Enrollment
        ↓
Store History
        ↓
Publish EnrollmentUpdated
        ↓
Audit
        ↓
Complete
```

This sequence is defined by `WF-C02-03`. fileciteturn99file0

### 9.1 Main Flow Boundary

The scenario stops once the enrollment update has completed.

Any separate academic assignment, programme migration, registration, or status transition is governed by its own canonical workflow.

---

## 10. Alternate Business Flows

### 10.1 AF-01 — Academic Session Update

Where an academic session update triggers the scenario:

1. identify the affected active Student;
2. validate the update request;
3. update the authoritative enrollment information;
4. preserve prior enrollment history;
5. record audit evidence;
6. publish `EnrollmentUpdated`.

The trigger is explicitly supported by L2-C02. fileciteturn99file0

### 10.2 AF-02 — Administrative Enrollment Update

Where an authorized administrative request triggers the scenario:

1. validate the Student;
2. validate authorization;
3. apply the approved enrollment update;
4. preserve historical enrollment;
5. audit;
6. publish `EnrollmentUpdated`.

### 10.3 AF-03 — Repeated Business Request

If the same business update is presented again, C-008 must respect the L2-C02 architectural requirement for idempotent lifecycle operations.

A repeated request must not create contradictory duplicate current enrollment outcomes.

### 10.4 AF-04 — Academic Context Participation

Where Academic Delivery information is required to establish the enrollment context, Academic Delivery participates through the defined architecture boundary.

The Student Lifecycle domain remains responsible for the enrollment outcome.

### 10.5 Boundary with Academic Assignment

If the requested change is actually a Programme, Batch, or Section assignment change, it belongs to `WF-C02-04 — Academic Assignment` rather than being silently absorbed into C-008.

The L2-C02 catalogue separately defines Academic Assignment for Programme, Batch, and Section assignment. fileciteturn98file0

### 10.6 Boundary with Programme Migration

If the request changes the student's Programme, C-008 must not make that migration decision.

The locked L2-C02 architecture explicitly states that Programme-changing mobility is owned by L2-C04 APW-08, and C02 only applies the authoritative `ProgrammeMigrationApproved` result to the Student context. fileciteturn99file0

---

## 11. Failure, Interruption and Business Recovery

### 11.1 Student Not Active

**Condition:** The Student is not Active.

**Business result:** The enrollment update cannot proceed under the stated precondition.

**Resolution:** Follow the applicable upstream lifecycle process before attempting enrollment update.

### 11.2 Request Not Authorized

**Condition:** The requester does not satisfy the authorization requirement.

**Business result:** The enrollment update does not proceed.

**Resolution:** Establish the required business authorization.

### 11.3 Invalid Enrollment Change

**Condition:** The requested change is not a valid enrollment update.

**Business result:** The update is rejected rather than being forced into C-008.

**Resolution:** Route the business request to the correct canonical workflow.

### 11.4 Current Enrollment Conflict

**Condition:** Applying the requested change would violate the requirement for unique current enrollment.

**Business result:** The update cannot establish contradictory current enrollment.

**Resolution:** Resolve the enrollment condition according to the authoritative institutional policy.

### 11.5 Historical Record Conflict

**Condition:** The requested change would overwrite or destroy prior enrollment history.

**Business result:** Historical information must remain preserved.

**Resolution:** Maintain historical enrollment and apply only the valid current change.

### 11.6 Academic Assignment Request

**Condition:** The request is actually for Programme, Batch, or Section assignment.

**Business result:** C-008 does not perform the unrelated lifecycle change.

**Resolution:** Use the authoritative Academic Assignment workflow.

### 11.7 Programme Migration Request

**Condition:** The request changes Programme.

**Business result:** C-008 does not make the Programme Migration decision.

**Resolution:** L2-C04 APW-08 owns the Programme Migration decision; C02 may subsequently apply the authoritative `ProgrammeMigrationApproved` result. fileciteturn99file0

### 11.8 Technical Failure Boundary

L3 does not define:

- technical retries;
- queues;
- rollback;
- database recovery;
- service failures;
- infrastructure failure;
- API error handling.

---

## 12. Business Rules and Decision Constraints

### 12.1 Authoritative L2 Rules

The directly applicable `WF-C02-03` governance rules are:

| Rule | Business Constraint |
|---|---|
| Historical enrollment immutable | Prior enrollment history cannot be overwritten. |
| Current enrollment unique | Only one current enrollment outcome may exist for the applicable Student context. |
| Audit mandatory | Enrollment update must be auditable. |

These rules are explicitly defined in the L2 workflow governance. fileciteturn99file0

### 12.2 Student Lifecycle Architectural Rules

The locked L2-C02 architecture additionally establishes:

- Student is the canonical master entity after Admission.
- Student identity is immutable.
- Student lifecycle transitions are deterministic.
- Historical lifecycle information is immutable.
- Student ownership remains within Student Lifecycle.
- Cross-domain updates occur only through approved architecture contracts. fileciteturn97file0

### 12.3 Cross-Domain Ownership Constraint

Academic Delivery may participate but cannot become the owner of enrollment.

### 12.4 Programme Migration Constraint

Programme-changing mobility is not an enrollment update decision.

L2-C04 owns Programme Migration and its canonical outcome. C02 only applies the approved Programme association after consuming `ProgrammeMigrationApproved`. fileciteturn99file0

### 12.5 No Local Rule Invention

C-008 does not introduce:

- new enrollment states;
- new deadlines;
- new approval rules;
- new Programme eligibility rules;
- new academic policy;
- new ownership.

---

## 13. State and Lifecycle Impact

### 13.1 Canonical Business Record

**Student / StudentContext Enrollment**

Student Lifecycle remains the canonical owner.

### 13.2 Lifecycle Effect

The authoritative workflow describes the operation as an **update to enrollment information**, not as a new Student lifecycle state.

Therefore C-008 does not invent a new Student status.

### 13.3 Business Transition

The business change is:

```text
Current Enrollment
        │
        ▼
Updated Enrollment
```

while preserving:

```text
Historical Enrollment
        │
        └── preserved
```

### 13.4 Current Enrollment Uniqueness

The updated enrollment becomes the authoritative current enrollment.

The previous enrollment remains historical.

### 13.5 Student Status

The workflow requires the Student to be Active before the update.

C-008 does not itself change Student status.

### 13.6 Academic Assignment Boundary

C-008 does not perform the Academic Assignment lifecycle.

Programme, Batch, and Section assignment remains governed by `WF-C02-04`. fileciteturn98file0

### 13.7 Programme Migration Boundary

C-008 does not make Programme Migration decisions.

The authoritative C04 migration outcome is applied by the C02 Academic Assignment workflow when applicable. fileciteturn99file0

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published Event

| Event | Canonical Publisher | Meaning |
|---|---|---|
| `EnrollmentUpdated` | **Student Lifecycle** | Enrollment information has been successfully updated. |

The L2 workflow explicitly identifies `EnrollmentUpdated` as its domain event. fileciteturn99file0

### 14.2 Consumed Events

The directly applicable L2 workflow does not define a canonical consumed event for C-008.

Therefore no consumed event is invented.

### 14.3 Academic Delivery Participation

Academic Delivery is a participating domain according to the frozen C-008 scenario metadata and the C02 workflow catalogue.

Its role is contextual academic participation.

It does not own:

- Student;
- StudentContext;
- enrollment;
- enrollment history; or
- `EnrollmentUpdated`.

### 14.4 Student Lifecycle Event Ownership

`EnrollmentUpdated` is published by Student Lifecycle because Student Lifecycle owns enrollment.

### 14.5 Event Boundary

`EnrollmentUpdated` represents the completed business fact.

It is not a technical API, queue, payload, or implementation mechanism.

---

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authorization

The L2 workflow requires an **Authorized Request**.

The exact human role is not specified in the retrieved `WF-C02-03` definition.

Therefore no specific role title is invented.

### 15.2 Approval

The directly applicable enrollment workflow does not define a separate approval hierarchy.

C-008 therefore does not invent an approval chain.

If a particular enrollment change requires an upstream institutional approval policy, that policy must be authoritative before it is represented as a mandatory L3 approval step.

### 15.3 Audit

The L2 governance explicitly requires audit.

The completed scenario therefore requires business evidence that the enrollment update occurred.

### 15.4 Historical Evidence

The previous enrollment must remain preserved as history.

### 15.5 Separation of Duties

No C-008-specific separation-of-duty rule is established by the retrieved L2 workflow.

No local separation-of-duty rule is introduced.

---

## 16. Completion Criteria and Postconditions

### 16.1 Successful Completion Criteria

C-008 is complete when:

1. the Student exists;
2. the Student is Active;
3. the enrollment update request is authorized;
4. the affected enrollment information has been identified;
5. the current enrollment has been updated;
6. previous enrollment information has been preserved;
7. current enrollment uniqueness remains satisfied;
8. audit evidence has been recorded; and
9. `EnrollmentUpdated` has been published by Student Lifecycle.

### 16.2 Successful Postconditions

| Postcondition | Result |
|---|---|
| Student | Remains owned by Student Lifecycle |
| Student status | Remains Active unless a separate canonical workflow changes it |
| Current enrollment | Updated |
| Historical enrollment | Preserved |
| Current enrollment uniqueness | Preserved |
| Audit | Completed |
| `EnrollmentUpdated` | Published by Student Lifecycle |
| Academic Delivery | Participates without acquiring enrollment ownership |
| Programme Migration | Not performed |
| Academic Registration | Not performed |

### 16.3 No Ownership Transfer

Completion does not transfer ownership of Student, StudentContext, or Enrollment to Academic Delivery.

### 16.4 End Boundary

The scenario ends after the completed enrollment update and publication of `EnrollmentUpdated`.

---

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

#### OI-C008-01 — Exact Enrollment Data Elements

The L2 workflow identifies the business capability as **Enrollment Update** and its input as **Enrollment Data**, but the retrieved `WF-C02-03` section does not enumerate the exact canonical enrollment fields that may be changed.

**Architecture impact:** Non-blocking for the L3 business outcome.

**Disposition:** Do not invent a field-level enrollment model. The L3 scenario remains at the business-information level.

---

#### OI-C008-02 — Academic Delivery Participation Detail

The frozen scenario identifies Academic Delivery as a participating domain, and L2-C02 lists Academic Delivery as an external participant for Enrollment Update. However, the retrieved `WF-C02-03` contract does not specify a detailed Academic Delivery interaction.

**Architecture impact:** Non-blocking.

**Disposition:** Preserve Academic Delivery as a participant without inventing a specific contract, actor, event, or ownership transfer.

---

### 17.2 Assumptions

1. The frozen Stage 3 identity for C-008 is authoritative.
2. `WF-C02-03 — Student Enrollment Update` is the applicable L2 workflow.
3. Student Lifecycle remains the canonical owner of Enrollment.
4. The Student must be Active.
5. The request must be authorized.
6. Historical enrollment remains immutable.
7. Current enrollment remains unique.
8. Audit is mandatory.
9. `EnrollmentUpdated` is the canonical successful business event.
10. Programme-changing mobility remains owned by L2-C04 APW-08.
11. Academic Assignment remains separate from Enrollment Update.
12. No new enrollment lifecycle state is created by C-008.

---

### 17.3 Deferred Questions

| Question | Target Authority | Reason |
|---|---|---|
| Which exact enrollment attributes are mutable? | Locked L2 / upstream Student Lifecycle architecture | Not enumerated in WF-C02-03. |
| What exact Academic Delivery information is required? | L2-C02 / L2-D01 | Participation is identified but detailed interaction is not defined in the retrieved workflow. |
| Which institutional roles may initiate each enrollment update type? | Institutional policy / L2 | Only “Authorized Request” is defined. |
| Which enrollment changes require additional approval? | Institutional policy / L2 | No specific approval chain is defined in WF-C02-03. |

---

# Validation and Authoring Classification

**Classification: PASS WITH ISSUES — Draft**

## Identity Verification

- Final ID: `L3-C-008`
- Title: `Student Enrollment Update`
- Primary Owner: Student Lifecycle
- Participating Domains: C, D
- Scenario Type: Cross-domain
- Priority: P1

These frozen values are confirmed by the Section C canonical register. fileciteturn91file0L2-L2

## L2 Verification

The locked L2-C02 workflow catalogue identifies:

`WF-C02-03 — Student Enrollment Update`

with the purpose of maintaining enrollment-related information throughout the student's lifecycle. Its defined triggers are Enrollment Change, Academic Session Update, and Administrative Request; its preconditions are Student Active and Authorized Request; and its postconditions include updated enrollment, preserved history, completed audit, and event publication. fileciteturn99file0

## Workflow Verification

The authoritative sequence is:

```text
Enrollment Request
        ↓
Validate Request
        ↓
Update Enrollment
        ↓
Store History
        ↓
Publish EnrollmentUpdated
        ↓
Audit
        ↓
Complete
```

fileciteturn99file0

## Ownership Verification

Student Lifecycle owns Enrollment and the Student Aggregate/StudentContext.

Academic Delivery participates without acquiring ownership. fileciteturn97file0turn98file0

## Event Verification

Canonical successful event:

`EnrollmentUpdated`

Publisher:

**Student Lifecycle**. fileciteturn99file0

## Rule Verification

The directly applicable governance rules are:

- Historical enrollment immutable.
- Current enrollment unique.
- Audit mandatory. fileciteturn99file0

## Boundary Verification

C-008 does not absorb:

- Academic Assignment;
- Programme Migration;
- Student Status Change;
- Registration;
- Student Transfer.

Programme-changing mobility remains explicitly owned by L2-C04 APW-08. fileciteturn99file0

## Implementation Leakage Check

No APIs, databases, Business Data Structures, services, queues, deployment, infrastructure, technical access control/IAM, technical retries, rollback, or UI design are introduced.

## Final Status

**PASS WITH ISSUES — Draft**

The document is complete across all 17 mandatory L3 sections. The non-blocking issues are limited to source-defined details that the retrieved `WF-C02-03` does not enumerate, so they are explicitly deferred rather than invented.

> **Governance note:** Complete authoring artifact ≠ Architecture Approved/Locked. This remains `Draft` until human architecture review.
