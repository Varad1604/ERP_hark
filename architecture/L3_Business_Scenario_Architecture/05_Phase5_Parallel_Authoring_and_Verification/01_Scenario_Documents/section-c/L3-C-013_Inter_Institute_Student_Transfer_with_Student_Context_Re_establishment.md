# Inter-Institute Student Transfer with Student Context Re-establishment

**Scenario ID:** L3-C-013  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** B, C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Frozen Stage 3 Section C canonical register; locked L1 Business Domain Architecture; locked L2-C02 — Student Lifecycle & Activation Workflows; locked Section B Institution & Structure architecture  
**Referenced By:** Future L4 System Behavior Specification derived from this scenario

---

The frozen Section C Stage 3 canonical register defines `L3-C-013` as **Inter-Institute Student Transfer with Student Context Re-establishment**, with:

- **Primary Owner:** Student Lifecycle
- **Participating Domains:** B, C
- **Scenario Type:** Critical
- **Priority:** P1
- **Derivation:** KEEP
- **Construction Status:** Not Authored

These values are frozen and are not reclassified by this scenario.

The exact Phase 5 filename is:

`L3-C-013_Inter-Institute_Student_Transfer_with_Student_Context_Re-establishment.md`

This scenario remains `Draft`. It does not constitute Architecture Approval or Locking.

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-C-013 |
| Canonical Title | Inter-Institute Student Transfer with Student Context Re-establishment |
| Primary Owner | Student Lifecycle |
| Participating Domains | B, C |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional business outcome in which an existing Student moves from one Institute to another Institute and the Student's institutional context is re-established for the destination Institute.

The scenario preserves the canonical Student identity while establishing the Student's new Institute-specific institutional relationship under Student Lifecycle ownership.

The scenario is distinct from C-012 because C-012 governs an intra-Institute, non-Programme organisational or placement transfer, whereas C-013 governs an **inter-Institute** transfer with Student Context re-establishment.

---

## 3. Business Outcome

The singular institutional outcome is:

> **The Student's authorised move from the source Institute to the destination Institute is completed, the Student's canonical identity remains intact, the source Institute relationship is appropriately concluded, a valid Student Context is re-established for the destination Institute, and the Student becomes established in the destination Institute under Student Lifecycle ownership.**

The scenario does not create a new person identity and does not transfer ownership of the canonical Student Aggregate away from Student Lifecycle.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised inter-Institute Student Transfer is initiated under the applicable institutional transfer process.

The authoritative Stage 3 semantics define the scenario as **Inter-Institute Student Transfer with Student Context Re-establishment**.

## 4.2 End Boundary

The scenario ends when:

1. the inter-Institute transfer has been validly authorised;
2. the source Institute relationship has reached its applicable transfer-completion position;
3. the Student's canonical identity remains preserved;
4. the destination Institute relationship has been established through a new/re-established Student Context;
5. the destination context is valid for the Student's continued institutional lifecycle; and
6. the Student is established in the destination Institute.

The Main Business Flow stops at this institutional outcome.

## 4.3 In Scope

- Inter-Institute Student Transfer.
- Source Institute relationship handling as required by the transfer.
- Destination Institute establishment.
- Preservation of canonical Student identity.
- Student Context re-establishment for the destination Institute.
- Validation of destination institutional context.
- Preservation of historical institutional context.
- Cross-domain participation by Institution & Structure.
- Student Lifecycle ownership of the resulting Student Context.

## 4.4 Out of Scope

- Same-Institute non-Programme reassignment — C-012.
- Programme-changing mobility where the applicable upstream architecture assigns the decision to Academic Progression.
- Admission processing for a new person.
- Creation of a second canonical Student identity for the same person.
- Academic teaching execution.
- Course registration.
- Attendance.
- Examination processing.
- Finance settlement beyond any business clearance required by the authoritative transfer process.
- Hostel, library, transport, placement, or other downstream operational processing.
- Technical implementation mechanisms.

## 4.5 Boundary with C-012

C-012 is the authorised same-Institute non-Programme relocation processed through WF-C02-08.

C-013 is the distinct inter-Institute outcome in which the Student's institutional context is re-established for another Institute.

The distinction is therefore the **institutional boundary and resulting Student Context**, not merely a change to Department, Batch, Section, Campus, or other assignment references.

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| **C — Student Lifecycle** | Canonical owner of Student, Student Aggregate, StudentContext, Student institutional lifecycle, and the completed inter-Institute Student Transfer outcome. |
| **B — Institution & Structure** | Owns authoritative institutional and organisational structure information required to identify and validate source/destination Institute context. |

The L2-C02 architecture explicitly establishes Student Lifecycle as the exclusive owner of the Student Aggregate and associated StudentContext.

The Section B organisation architecture establishes Institution & Structure as the authoritative source for organisational information and explicitly excludes Student Management from its ownership.

Participation by B therefore does not transfer Student or StudentContext ownership from C.

---

## 6. Business Actors, Roles and Responsibilities

The authoritative material retrieved for this scenario establishes the business responsibility boundaries but does not expose a sufficiently precise canonical actor/role mapping for the complete inter-Institute transfer process.

Accordingly, no new role is invented.

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Authoritative transfer participant | Initiates or participates in the governed inter-Institute transfer process. | Exact canonical role requires upstream confirmation. |
| Student Lifecycle | Validates and applies the Student lifecycle and Student Context outcome. | Canonical owner. |
| Institution & Structure | Provides authoritative source/destination institutional context. | Canonical owner of institutional/organisational structure. |

The exact actor/approval-role mapping is recorded as an open issue in Section 17 rather than being silently inferred.

No technical access control/technical identity profiles is defined.

---

## 7. Preconditions and Entry Criteria

The following business conditions are required:

1. A canonical Student exists.
2. The Student has an existing StudentContext associated with an Institute.
3. A valid source Institute relationship exists.
4. A destination Institute is identified within the authoritative institutional structure.
5. The proposed movement is an inter-Institute transfer rather than a same-Institute reassignment.
6. The transfer is authorised through the applicable institutional transfer process.
7. The destination institutional context is valid for establishing a Student Context.
8. The transfer does not require creation of a second canonical Student identity.

The L2-C02 model explicitly states that a Student may have multiple historical StudentContexts across institutional history and that at most one StudentContext may be Active within a given Institute at any point in time.

---

## 8. Trigger

The business trigger is:

> **An authorised inter-Institute Student Transfer is initiated under the applicable institutional transfer process.**

The available locked C02 workflow explicitly identifies **Administrative Approval** and **Student Transfer Request** as triggers for the Student Transfer workflow.

For C-013, the transfer crosses the Institute boundary and therefore requires destination-context re-establishment rather than the same-Institute transfer outcome of C-012.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the authorised inter-Institute Student Transfer request. | Governed transfer process / Student Lifecycle | Inter-Institute transfer processing begins. |
| 2 | Confirm the canonical Student identity and existing source StudentContext. | Student Lifecycle | Existing Student and source institutional relationship are identified. |
| 3 | Confirm that the destination Institute is distinct from the source Institute. | Student Lifecycle with Institution & Structure context | Inter-Institute boundary is established. |
| 4 | Validate the destination Institute against authoritative institutional structure. | Institution & Structure / Student Lifecycle | Destination institutional context is valid. |
| 5 | Apply the authorised source-to-destination institutional transfer outcome. | Student Lifecycle | Source institutional relationship is progressed according to the transfer outcome. |
| 6 | Preserve the historical source StudentContext and its institutional history. | Student Lifecycle | Prior institutional relationship remains historically traceable. |
| 7 | Establish the Student's destination Institute relationship through a Student Context. | Student Lifecycle | Destination Student Context is established under the canonical Student. |
| 8 | Establish the destination context in the applicable active institutional lifecycle position. | Student Lifecycle | Student is established for the destination Institute. |
| 9 | Complete the inter-Institute transfer outcome. | Student Lifecycle | Student's institutional relationship is now established at the destination Institute. |
| 10 | End the scenario. | Student Lifecycle | C-013 Business Outcome is achieved. |

The flow intentionally does not extend into downstream academic registration, teaching, attendance, examination, finance, hostel, library, transport, or other operational consequences. Those remain separate business capabilities.

---

## 10. Alternate Business Flows

## AF-01 — Destination Context Re-established After Source Context Completion

Where the source Institute relationship must first reach its governed transfer-completion position, the destination Student Context is established only after the applicable source-side business condition has been satisfied.

The resulting outcome remains C-013.

## AF-02 — Existing Historical Destination Relationship

If the Student has historical institutional association with the destination Institute, the scenario does not create a second canonical Student identity.

The Student remains the same canonical Student, while the applicable destination Student Context is re-established according to authoritative lifecycle rules.

This is consistent with the locked C02 model permitting multiple historical StudentContexts across institutional history.

## AF-03 — Programme Association Requires a Separate Authoritative Decision

If the inter-Institute movement also involves a Programme change, C-013 does not invent or assume the Programme decision.

Where Programme Migration is applicable, the C02 architecture states that L2-C04 exclusively owns Programme-changing mobility and publishes `ProgrammeMigrationApproved`; C02 may apply that already-approved Programme association but does not make or republish the migration decision.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Destination Institute is not valid in the authoritative institutional structure | Destination Context cannot be validly established. | Resolve the institutional destination before transfer completion. | Institution & Structure / Student Lifecycle |
| Transfer is not authorised | Inter-Institute transfer cannot complete. | Obtain the required authoritative transfer decision/approval. | Governed transfer authority / Student Lifecycle |
| Source StudentContext cannot validly be progressed for transfer | Destination establishment cannot complete as a valid continuation. | Resolve the source-context lifecycle condition through the applicable Student Lifecycle process. | Student Lifecycle |
| Proposed movement is actually same-Institute reassignment | C-013 boundary does not apply. | Route to C-012 / applicable same-Institute transfer or assignment scenario. | Student Lifecycle |
| Proposed movement requires a Programme decision that has not been established | Programme association cannot be changed as part of C-013. | Obtain the authoritative Programme Migration decision through L2-C04 where applicable. | Academic Progression |
| Destination Student Context cannot be validly established | Inter-Institute transfer outcome remains incomplete. | Resolve the destination-context business deficiency before completion. | Student Lifecycle |

No technical retry, queue, database rollback, server failure, exception-handler, or infrastructure recovery behaviour is defined.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Student is the canonical master entity after Admission. | Student Lifecycle | Transfer preserves the same canonical Student identity. | L2-C02 |
| A Student may own multiple historical StudentContexts across institutional history. | Student Lifecycle | Source context remains historical while destination context may be established. | L2-C02 |
| At most one StudentContext may be Active within a given Institute at any point in time. | Student Lifecycle | Destination establishment must respect the canonical lifecycle invariant. | L2-C02 |
| Student Lifecycle exclusively owns Student and StudentContext. | Student Lifecycle | Inter-Institute participation does not transfer lifecycle ownership. | L2-C02 |
| Student ID is immutable. | Student Lifecycle | Inter-Institute transfer does not create a new Student identity. | L2-C02 |
| Historical lifecycle information remains immutable. | Student Lifecycle | Source institutional history remains preserved. | L2-C02 |
| Cross-domain updates occur only through approved business contracts. | Student Lifecycle / repository architecture | B participates without acquiring Student lifecycle ownership. | L2-C02 |
| Programme-changing mobility is owned by L2-C04 APW-08. | Academic Progression | C-013 cannot make a Programme Migration decision. | L2-C02 |
| Institution & Structure remains authoritative for organisational information. | Institution & Structure | Destination Institute must be established from authoritative institutional structure. | L2-B02 |

The L2-C02 Student Aggregate invariants explicitly require immutable Student identity, canonical Student ownership, lifecycle-constrained transitions, historical preservation, and approved cross-domain updates.

---

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Student Aggregate | Existing Student | Identity remains unchanged throughout transfer. | Same canonical Student | Student Lifecycle | L2-C02 |
| Source StudentContext | Active institutional relationship, subject to applicable transfer lifecycle | Source relationship is concluded/progressed according to the governed transfer outcome while historical information is preserved. | Historical source context / applicable transfer-completed position | Student Lifecycle | L2-C02 + C-013 semantics |
| Destination StudentContext | No active destination relationship | A new/re-established destination institutional context is established for the same Student. | Applicable active destination institutional position | Student Lifecycle | L2-C02 |
| Student lifecycle | Active institutional lifecycle | Transfer changes institutional association without replacing canonical Student identity. | Active in destination context when transfer completion permits | Student Lifecycle | L2-C02 |

The locked C02 model states that each StudentContext represents the Student's relationship with a specific Institute and that a Student may have multiple historical StudentContexts while at most one StudentContext may be Active for a given Institute.

No new lifecycle state is introduced by C-013.

---

## 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

The authoritative material retrieved for L2-C02 explicitly defines `StudentTransferred` as the canonical event for WF-C02-08 Student Transfer.

For C-013, the exact canonical event sequence specific to **inter-Institute transfer with Student Context re-establishment** is not sufficiently exposed in the retrieved L2 material to safely assert a new event name.

Therefore:

> **No additional C-013-specific event is invented.**

If the locked upstream architecture defines `StudentTransferred` as the event covering this inter-Institute outcome, that event must be used with Student Lifecycle as publisher. If a distinct inter-Institute event is defined elsewhere, it must be adopted exactly as locked rather than renamed or created locally.

## 14.2 Institution & Structure Collaboration

Institution & Structure provides authoritative institutional and organisational context.

The Organisation Domain is the authoritative source for organisational information and does not own Student Management.

The collaboration therefore supports destination Institute validation without transferring StudentContext ownership.

## 14.3 Programme Migration Collaboration

Where Programme Migration is separately required, C02 consumes the already-approved `ProgrammeMigrationApproved` outcome to apply the Programme association; it does not make or republish the Programme Migration decision.

## 14.4 Synchronous Business Hand-offs

The precise business contract for inter-Institute Student Context re-establishment is not exposed sufficiently in the retrieved authoritative material.

It is therefore not invented here.

Any future L4 specification must derive the exact hand-off from the locked upstream contract architecture.

---

## 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

C-013 requires an authorised inter-Institute Student Transfer.

The L2-C02 Student Transfer workflow identifies Administrative Approval and Student Transfer Request as business triggers and requires the transfer to be permitted by applicable policy.

The exact canonical approving actor/role for the inter-Institute case is not established by the retrieved sources and is therefore not invented.

## 15.2 Approval

Administrative Approval is an authoritative trigger for WF-C02-08.

For C-013, any additional cross-Institute approval hierarchy must come from the applicable locked upstream architecture.

No local approval authority is created.

## 15.3 Audit

The L2-C02 architecture requires lifecycle operations to be auditable and preserves historical lifecycle information.

C-013 therefore requires the transfer outcome and Student Context lifecycle changes to remain historically traceable.

## 15.4 Historical Evidence

The source Student Context and its institutional history must remain preserved.

The destination Student Context represents a new institutional relationship for the same canonical Student; it does not erase the Student's historical source relationship.

## 15.5 Separation of Duty

No C-013-specific separation-of-duty rule is asserted because the retrieved authoritative sources do not define one specifically for this scenario.

---

## 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

C-013 is complete when:

1. the inter-Institute Student Transfer is authorised;
2. the canonical Student identity is confirmed;
3. the source StudentContext is validly progressed for transfer;
4. the destination Institute is validated against authoritative institutional structure;
5. the source institutional history is preserved;
6. a valid destination Student Context is established for the same Student;
7. the destination context reaches its applicable active institutional position; and
8. the Student is established in the destination Institute.

## 16.2 Postconditions

| Concern | Postcondition |
|---|---|
| Canonical Student | Same Student identity remains authoritative |
| Student ID | Unchanged |
| Source StudentContext | Preserved as historical institutional context according to its lifecycle outcome |
| Destination StudentContext | Established for the destination Institute |
| Student ownership | Remains with Student Lifecycle |
| Institute | Student's active institutional relationship is now associated with the destination Institute |
| Historical record | Prior institutional context remains traceable |
| Programme | Unchanged unless a separately authorised Programme Migration outcome has been established |
| Downstream operations | Not executed as part of C-013 |
| Scenario boundary | Inter-Institute transfer and destination Student Context re-establishment are complete |

## 16.3 Downstream Consequence Boundary

The scenario does not continue into every consequence of becoming established at the destination Institute.

Academic registration, course participation, attendance, examinations, finance, hostel, library, transport, placement, guardian, scholarship, and other downstream consequences remain governed by their respective business architectures.

## 16.4 End Boundary

The scenario ends once the Student's destination Institute relationship and Student Context are validly established and the inter-Institute transfer outcome is complete.

---

## 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The retrieved L2-C02 source defines the same-Institute WF-C02-08 Student Transfer but does not expose a dedicated detailed inter-Institute transfer workflow specification. | The exact workflow-level sequencing for C-013 cannot be asserted beyond the frozen Stage 3 scenario semantics without risking invention. | Confirm the authoritative locked L2 workflow governing inter-Institute transfer and Student Context re-establishment before Architecture Approval. |
| The exact canonical event for the completed inter-Institute Student Transfer is not explicitly exposed in the retrieved sources. | A new event must not be invented. | Confirm whether `StudentTransferred` is the canonical event for C-013 or whether another already-locked event governs this outcome. |
| The exact source-context terminal/resulting lifecycle position for inter-Institute transfer is not explicitly exposed. | L3 cannot invent or rename a lifecycle state. | Confirm from the authoritative locked Student Lifecycle workflow before Locking. |
| The exact actor/role and approval authority for inter-Institute transfer is not exposed. | L3 cannot invent an approving role. | Confirm from authoritative Institution & Structure / Student Lifecycle governance sources. |
| The exact business contract/hand-off for destination Student Context re-establishment is not exposed. | Detailed cross-domain contract wording must not be invented at L3. | Derive from the authoritative locked upstream contract architecture at L4/L3 refinement as applicable. |

These issues are material enough that this document remains **Draft** and is not classified as fully PASS.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| C-013 represents a genuine inter-Institute transfer rather than an intra-Institute reassignment. | Frozen Stage 3 title and semantics. | Would require architecture change control. |
| The same canonical Student identity is preserved across institutional history. | L2-C02 Student Aggregate ownership and immutable Student identity. | Would contradict canonical Student ownership if false. |
| Destination institutional context remains owned by Student Lifecycle. | L2-C02 explicitly assigns StudentContext ownership to Student Lifecycle. | Would require upstream ownership change if contradicted. |
| Institution & Structure supplies authoritative institutional context without owning Student lifecycle. | L2-B02 ownership boundary. | Would require boundary correction if contradicted. |

Assumptions do not override locked architecture.

## 17.3 Deferred Questions

1. What exact locked L2 workflow owns the complete inter-Institute transfer sequence?
2. Does `StudentTransferred` cover both same-Institute and inter-Institute transfer outcomes, or is there an already-defined inter-Institute event?
3. What exact lifecycle position represents completion of the source StudentContext transfer?
4. What exact lifecycle position is required before the destination StudentContext becomes Active?
5. What canonical business actor/authority approves inter-Institute transfer?
6. What exact business contract governs destination Student Context re-establishment?

These questions must be resolved from authoritative repository architecture before the scenario can advance to Architecture Approved or Locked.

---

# Validation and Review Classification

## Repository Identity

- **Final ID:** `L3-C-013`
- **Canonical Title:** Inter-Institute Student Transfer with Student Context Re-establishment
- **Primary Owner:** Student Lifecycle
- **Participating Domains:** B, C
- **Type:** Critical
- **Priority:** P1
- **Derivation:** KEEP
- **Status:** Draft

The Stage 3 canonical register confirms the exact C-013 identity and classification.

## Ownership Validation

Student Lifecycle is the canonical owner of Student and StudentContext.

Institution & Structure owns institutional/organisational information and explicitly excludes Student Management from its ownership.

## Lifecycle Validation

The C02 architecture establishes:

- one canonical Student Aggregate;
- multiple historical StudentContexts across institutional history;
- at most one Active StudentContext for a given Institute;
- immutable Student identity;
- StudentContext ownership by Student Lifecycle.

C-013 applies these principles without inventing a new lifecycle state.

## Transfer Boundary Validation

The locked C02 Student Transfer workflow is explicitly limited to **non-Programme organisational or placement dimensions**, with Programme-changing mobility excluded and owned by L2-C04 APW-08.

C-013 is not collapsed into C-012 because its frozen outcome explicitly crosses the Institute boundary and requires Student Context re-establishment.

## Event Validation

The locked C02 source defines `StudentTransferred` for WF-C02-08.

Because the retrieved source does not explicitly establish whether that event is the canonical completion event for the inter-Institute variant, no new C-013 event is invented. This is recorded as an open issue.

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

The locked v2.0 template requires these 17 sections and explicitly requires upstream ambiguity to be recorded rather than silently resolved.

---

# Final Classification

**BLOCKED — Draft**

The frozen C-013 scenario identity is clear and the ownership boundary is clear. However, the retrieved authoritative L2 evidence does not expose enough detail to safely assert the complete inter-Institute transfer workflow, canonical completion event, source/destination lifecycle transitions, exact approval authority, and destination Student Context re-establishment contract.

Per the L3 authoring rules, these gaps are recorded rather than invented.

The document is therefore complete as a **source-grounded Draft**, but it must not be treated as Architecture Approved or Locked until the upstream definitions are confirmed.
