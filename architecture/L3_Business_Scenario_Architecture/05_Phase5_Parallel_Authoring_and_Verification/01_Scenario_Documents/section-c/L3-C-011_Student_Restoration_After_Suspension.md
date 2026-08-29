# Student Restoration After Suspension

**Scenario ID:** L3-C-011  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** C — Student Lifecycle; F — Finance  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  

---

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-011` |
| Canonical Title | Student Restoration After Suspension |
| Primary Owner | Student Lifecycle |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |
| Applicable L2 Workflow | `WF-C02-07 — Student Restoration` |
| Canonical Owner | Student Lifecycle |
| Canonical Completion Event | `StudentRestored` |
| Canonical Publisher | Student Lifecycle |

The frozen Section C Stage 3 register identifies C-011 as **Student Restoration After Suspension**, owned by **Student Lifecycle**, with participating domains **C, F**, Type **Critical**, Priority **P1**, and `KEEP` derivation.

The Stage 4 final-ID register establishes Section C as a direct positional mapping, making C-011 the final identity `L3-C-011`.

The locked L2-C02 workflow catalogue identifies `WF-C02-07 — Student Restoration` as the workflow whose purpose is to restore a suspended Student.

**Exact filename:** `L3-C-011_Student_Restoration_After_Suspension.md`

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-C-011 |
| Canonical Title | Student Restoration After Suspension |
| Primary Owner | Student Lifecycle |
| Participating Domains | C — Student Lifecycle; F — Finance |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional restoration of a Student who is currently suspended and for whom the authoritative restoration conditions have been satisfied.

The scenario returns the Student to the authoritative active institutional lifecycle position while preserving the complete suspension and restoration history.

Student Lifecycle remains the canonical owner of the Student Aggregate and StudentContext throughout the restoration. Finance may participate where financial clearance is part of the authoritative restoration process, but Finance ownership of financial concepts remains unchanged.

The locked L2-C02 architecture explicitly separates Student Restoration from Student Suspension, Student Transfer, and Student Exit / Archive. `WF-C02-07` is the canonical restoration workflow.

---

## 3. Business Outcome

The singular institutional business outcome is:

> **The previously suspended Student has been validly restored to the Active lifecycle position after satisfying the authoritative restoration conditions, the restoration is recorded with its required business evidence and history, and `StudentRestored` has been published by Student Lifecycle.**

Successful completion means the Student is again in the Active institutional lifecycle position.

Restoration does not erase the prior suspension, alter the Student's permanent identity, transfer ownership, or complete any separate academic, financial, or exit process.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authoritative restoration request or institutional restoration decision is raised for a Student currently in the Suspended lifecycle position.

The request may arise from the business conditions identified by the locked L2 workflow, including:

- Administrative Approval;
- Financial Clearance; or
- Disciplinary Resolution.

## 4.2 End Boundary

The scenario ends when:

1. the Student is confirmed to be in the Suspended position;
2. the authoritative restoration conditions have been satisfied;
3. the Student is restored to Active;
4. restoration history and required audit evidence are recorded; and
5. `StudentRestored` is published by Student Lifecycle.

The Main Business Flow stops at the completed restoration outcome.

It does not continue into unrelated downstream service reactivation, academic registration, finance processing, or other consequences.

## 4.3 In Scope

- Restoration of a suspended Student.
- Validation that the Student is Suspended.
- Validation of authoritative restoration conditions.
- Application of the Active lifecycle position.
- Preservation of suspension and restoration history.
- Required audit/evidence obligations.
- Cross-domain participation with Finance where financial clearance is authoritative.
- Publication of `StudentRestored`.

## 4.4 Out of Scope

- Student Suspension — `L3-C-010`.
- Student Transfer — `L3-C-012` is a distinct intra-institute reassignment scenario.
- Student Exit / Archive.
- Academic Progression decisions.
- Academic Registration.
- Finance-owned financial processing.
- Programme Migration decision-making.
- Technical APIs, databases, services, queues, infrastructure, UI, deployment, technical access control/IAM, or implementation mechanics.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by / Related | `L3-C-010 — Student Suspension` | Establishes the Suspended state from which C-011 restores the Student. |
| Distinct | `L3-C-012 — Intra-Institute Student Academic Reassignment` | Academic reassignment is a separate canonical outcome and is not part of restoration. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| **C — Student Lifecycle** | Canonical owner of Student, StudentContext, Student Status, suspension/restoration lifecycle and the completed restoration outcome. |
| **F — Finance** | Participates where financial clearance is an authoritative restoration condition; retains ownership of all Finance-owned concepts and decisions. |

The locked L2-C02 architecture states that Student Lifecycle is the exclusive owner of the Student Aggregate and associated StudentContext and identifies Student Restoration as a Student Lifecycle responsibility.

The Stage 3 frozen semantics explicitly retain Finance as a participating domain for C-011.

Finance participation therefore does not transfer ownership of Student, StudentContext, or lifecycle state.

---

## 6. Business Actors, Roles and Responsibilities

The locked L2-C02 workflow identifies restoration triggers including Administrative Approval, Financial Clearance, and Disciplinary Resolution. It also identifies `Registrar` and `Authorized Administrative Authority` in the authoritative service-contract material for restoration.

The L3 scenario therefore uses only those source-supported business responsibilities.

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| **Registrar** | Governs/authorizes Student restoration where the institutional process assigns this responsibility. | Student Lifecycle authority within institutional policy. |
| **Authorized Administrative Authority** | Performs or authorizes restoration where institutional policy delegates the responsibility. | Must remain within governed institutional authority. |
| **Finance** | Provides financial clearance where required by the authoritative restoration policy. | Finance-owned financial decision; does not own Student restoration. |
| **Student Lifecycle** | Applies the canonical Student restoration lifecycle outcome and publishes `StudentRestored`. | Canonical owner of Student lifecycle. |

No technical IAM/RBAC role is introduced.

---

## 7. Preconditions and Entry Criteria

The locked `WF-C02-07 — Student Restoration` defines the following preconditions:

- **Student Suspended**
- **Restoration authorized**.

The authoritative workflow also identifies restoration triggers of:

- Administrative Approval;
- Financial Clearance; and
- Disciplinary Resolution.

Accordingly, C-011 begins only when:

1. the Student exists;
2. the Student is currently Suspended;
3. the required restoration authority/approval has been established;
4. any authoritative financial clearance required by institutional policy has been satisfied; and
5. any authoritative disciplinary resolution required by institutional policy has been satisfied.

No additional local restoration eligibility rule is invented.

---

## 8. Trigger

The authoritative business triggers are:

- **Administrative Approval**
- **Financial Clearance**
- **Disciplinary Resolution**.

The trigger is expressed as a business condition/decision.

It is not represented as an API, service invocation, queue message, database update, technical scheduler, or other implementation mechanism.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the authoritative restoration request or restoration decision. | Registrar / Authorized Administrative Authority / Student Lifecycle | Restoration processing begins. |
| 2 | Confirm that the Student exists and is currently Suspended. | Student Lifecycle | Valid restoration candidate is established. |
| 3 | Confirm that required restoration authorization/approval has been completed. | Registrar / Authorized Administrative Authority | Restoration authority is established. |
| 4 | Where required by institutional policy, confirm Financial Clearance. | Finance | Finance-owned clearance requirement is satisfied. |
| 5 | Where applicable, confirm the relevant Disciplinary Resolution. | Authoritative institutional process | Suspension basis has been resolved sufficiently for restoration. |
| 6 | Restore the Student to the Active lifecycle position. | Student Lifecycle | Student is restored to Active. |
| 7 | Preserve the suspension and restoration history. | Student Lifecycle | Historical lifecycle remains intact. |
| 8 | Record required business audit/evidence. | Student Lifecycle | Restoration becomes auditable. |
| 9 | Publish `StudentRestored`. | Student Lifecycle | Canonical restoration business fact is established. |
| 10 | End the scenario. | Student Lifecycle | Student Restoration is complete. |

The locked L2 workflow sequence is:

```text
Restoration Request
        ↓
Validate Suspension
        ↓
Restore Student
        ↓
Update Status
        ↓
Publish StudentRestored
        ↓
Audit
        ↓
Complete
```

C-011 stops at the completed restoration outcome.

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result |
|---|---|---|---|
| AF-01 | Restoration follows Administrative Approval | Apply the authoritative restoration approval and proceed through C-011. | Student restored to Active. |
| AF-02 | Restoration requires Financial Clearance | Confirm the Finance-owned clearance before applying restoration. | Student restored once the authoritative clearance condition is satisfied. |
| AF-03 | Restoration follows Disciplinary Resolution | Apply the authoritative resolution as the basis for restoration. | Student restored once the restoration authority is satisfied. |
| AF-04 | Student is not Suspended | C-011 cannot complete because the authoritative precondition is not satisfied. | No restoration transition. |

The three authoritative restoration triggers are explicitly defined by `WF-C02-07`.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Student is not Suspended | Restoration cannot validly complete. | Do not apply the restoration transition; resolve through the correct lifecycle scenario. | Student Lifecycle |
| Required restoration authorization is absent | Restoration cannot validly complete. | Obtain the required authoritative approval before restoration. | Registrar / Authorized Administrative Authority |
| Required Financial Clearance is absent | Where financial clearance is mandatory, restoration cannot complete. | Resolve the Finance-owned clearance condition before restoration. | Finance |
| Required Disciplinary Resolution is absent | Where disciplinary resolution is required, restoration cannot complete. | Complete the authoritative disciplinary process before restoration. | Authoritative institutional process |
| Requested action is actually academic reassignment | Restoration must not absorb a separate academic assignment outcome. | Apply the applicable academic reassignment scenario after restoration where independently required. | Student Lifecycle |
| Requested action is Student Exit / Archive | C-011 does not own the terminal lifecycle outcome. | Apply the authoritative Student Exit / Archive process. | Student Lifecycle |

No technical retry, queue, rollback, server failure, timeout, infrastructure recovery, or exception-handling mechanism is defined at L3.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Student must be Suspended before restoration. | Student Lifecycle | Only a Suspended Student can enter C-011. | `WF-C02-07` Preconditions |
| Restoration must be authorized. | Student Lifecycle / institutional authority | C-011 cannot apply restoration without authoritative authorization. | `WF-C02-07` Preconditions |
| Administrative Approval may trigger restoration. | Student Lifecycle / institutional authority | An approved administrative restoration may initiate C-011. | `WF-C02-07` Trigger |
| Financial Clearance may trigger/support restoration. | Finance | Finance-owned clearance remains Finance-owned. | `WF-C02-07` Trigger; frozen C-011 participation |
| Disciplinary Resolution may trigger/support restoration. | Authoritative institutional process | Resolution must be established before restoration where applicable. | `WF-C02-07` Trigger |
| Suspension history must be preserved. | Student Lifecycle | Restoration does not erase the prior suspension. | Locked C02 lifecycle/audit principles |
| Lifecycle transitions are auditable. | Student Lifecycle | Restoration must produce audit evidence. | Locked C02 principles |
| Student identity remains unchanged. | Student Lifecycle | Restoration changes lifecycle position, not canonical Student identity. | Locked C02 ownership/invariant model |
| Restoration is distinct from suspension. | Student Lifecycle | C-011 does not re-execute or alter the suspension decision. | Workflow catalogue |
| Restoration is distinct from exit/archive. | Student Lifecycle | C-011 ends at Active; it does not archive the Student. | Workflow catalogue |

No new local rule identifier is created.

---

## 13. State and Lifecycle Impact

## 13.1 Student Aggregate

| Concern | Entry | Transition | Result | Owner |
|---|---|---|---|---|
| Student lifecycle state | Suspended | Restore Student | Active | Student Lifecycle |
| Student identity | Existing canonical identity | No identity change | Same Student identity | Student Lifecycle |
| Suspension history | Existing suspension history | Preserve | Historical suspension retained | Student Lifecycle |
| Restoration history | None for current restoration | Record restoration | Restoration becomes historical lifecycle evidence | Student Lifecycle |

The locked Student Lifecycle state model identifies **Suspended → Restored → Active** as the canonical restoration path.

## 13.2 StudentContext

The locked C02 architecture distinguishes the Student Aggregate persona from the per-Institute StudentContext.

For a suspended StudentContext, restoration returns the applicable StudentContext to **Active**.

The restoration does not create a new StudentContext, transfer the context, or terminate the institutional relationship.

## 13.3 No New State

C-011 does not invent a new lifecycle state.

`Restored` is the canonical intermediate outcome represented by the locked Student lifecycle model, followed by the Student returning to **Active**.

---

## 14. Business Events and Cross-Domain Collaboration

## 14.1 Published Business Event

The locked L2-C02 event catalogue explicitly defines:

**`StudentRestored`**

as the canonical event for successful completion of `WF-C02-07 — Student Restoration`.

| Business Event | Canonical Publisher | Purpose |
|---|---|---|
| `StudentRestored` | **Student Lifecycle** | Announces that the previously suspended Student has been restored to Active institutional participation. |

## 14.2 Finance Collaboration

Finance participates only where its authoritative financial clearance is a restoration condition.

The business collaboration is:

> Finance establishes its own Finance-owned clearance outcome; Student Lifecycle uses that authoritative business outcome to determine whether the Student restoration process may proceed.

This does not transfer Student ownership to Finance.

## 14.3 Downstream Consequence Boundary

Domains consuming `StudentRestored` may apply their own governed consequences.

For example, downstream domains may independently determine whether the Student becomes eligible again for their services.

Those downstream consequences are **not** part of C-011's Main Business Flow.

## 14.4 Event Ownership

`StudentRestored` is published only by Student Lifecycle.

No participating domain may publish a competing Student restoration event.

---

## 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

The authoritative L2 service-contract material identifies:

- **Registrar**
- **Authorized Administrative Authority**

for `RestoreStudent()`. The workflow also requires restoration to be authorized.

C-011 therefore treats restoration authorization as a source-supported business obligation.

It does not define technical access control/IAM.

## 15.2 Approval

The L2 workflow identifies **Administrative Approval** as a restoration trigger.

Where an approval is required, C-011 applies the authoritative approval outcome and does not invent a separate approval hierarchy.

## 15.3 Financial Clearance

Where institutional policy makes financial clearance a restoration condition, Finance remains the canonical owner of that clearance.

C-011 does not calculate, alter, waive, or otherwise redefine Finance-owned obligations.

## 15.4 Disciplinary Resolution

Where restoration follows Disciplinary Resolution, C-011 consumes the authoritative business resolution rather than recreating the disciplinary decision.

## 15.5 Audit

The locked C02 architecture requires lifecycle transitions to generate immutable audit records.

The authoritative `WF-C02-07` sequence explicitly includes **Audit** after `StudentRestored` publication.

C-011 therefore requires restoration to be fully auditable.

## 15.6 Separation of Duty

No additional restoration-specific separation-of-duty rule is introduced because the authoritative workflow evidence available for this scenario does not define one beyond the governed authorization/approval boundary.

---

## 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

C-011 is complete when:

1. the Student exists;
2. the Student is currently Suspended;
3. restoration authorization has been established;
4. required Financial Clearance has been satisfied where applicable;
5. required Disciplinary Resolution has been satisfied where applicable;
6. the Student has been restored to Active;
7. suspension history remains preserved;
8. restoration history/evidence has been recorded;
9. required audit evidence has been generated; and
10. `StudentRestored` has been published by Student Lifecycle.

## 16.2 Postconditions

| Postcondition | Result |
|---|---|
| Student identity | Unchanged |
| Student ownership | Remains with Student Lifecycle |
| StudentContext ownership | Remains with Student Lifecycle |
| Student lifecycle | Active |
| Suspension history | Preserved |
| Restoration history | Recorded |
| Finance | Retains ownership of Finance-owned clearance and consequences |
| `StudentRestored` | Published by Student Lifecycle |
| Audit | Completed |
| Student Exit / Archive | Not performed |
| Academic reassignment | Not performed unless separately initiated through its own canonical scenario |

## 16.3 Downstream Consequence Boundary

The publication of `StudentRestored` establishes the completed business fact.

Any subsequent downstream reactivation or eligibility change belongs to the relevant consuming domain and its own architecture.

C-011 does not claim ownership of those downstream consequences.

## 16.4 End Boundary

The scenario ends after the Student is restored to Active, the restoration is audited, and `StudentRestored` has been published.

---

## 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

### OI-C011-01 — Exact Finance Clearance Contract

The frozen C-011 semantics identify Finance as a participating domain, and the locked `WF-C02-07` identifies **Financial Clearance** as a restoration trigger. The accessible L2-C02 source does not define the detailed Finance-side clearance contract or exact Finance workflow that produces that business outcome.

**Impact:** Non-blocking for the C02-owned restoration outcome, but the exact Finance collaboration must not be invented.

**Disposition:** Preserve Finance participation and its ownership boundary. Verify the applicable locked Finance L2 workflow before Architecture Approval/Locking if the exact Finance hand-off must be documented.

---

### OI-C011-02 — Detailed Disciplinary Resolution Authority

The locked workflow identifies **Disciplinary Resolution** as a restoration trigger but does not, in the retrieved C02 material, define the complete disciplinary authority/role model.

**Impact:** Non-blocking for C-011's core outcome.

**Disposition:** Treat the disciplinary resolution as an authoritative upstream business outcome. Do not invent a disciplinary role or decision process.

---

### OI-C011-03 — Detailed Restoration Eligibility Matrix

The locked workflow establishes `Student Suspended` and `Restoration authorized` as preconditions but does not expose a complete matrix for every institution-specific restoration circumstance.

**Impact:** Non-blocking at L3 level.

**Disposition:** Apply only authoritative upstream restoration conditions. Do not create additional local eligibility rules.

---

## 17.2 Assumptions

1. The frozen Stage 3 C-011 semantics are authoritative for identity, owner, participating domains, Type and Priority.
2. `L3-C-011` is the final canonical identity.
3. `WF-C02-07 — Student Restoration` is the applicable locked L2 workflow.
4. Student Lifecycle owns the restoration outcome.
5. Finance participates without acquiring Student lifecycle ownership.
6. The Student must be Suspended before restoration.
7. Restoration must be authorized.
8. `StudentRestored` is the canonical completion event.
9. Student identity and suspension history are preserved.
10. Restoration does not itself perform academic reassignment or Student Exit / Archive.

These assumptions do not override locked architecture.

## 17.3 Deferred Questions

| Question | Required Authority | Reason |
|---|---|---|
| What exact Finance-owned business outcome constitutes Financial Clearance for C-011? | Locked Finance L2 | Needed for detailed cross-domain hand-off. |
| Which exact disciplinary authority produces Disciplinary Resolution? | Applicable locked disciplinary architecture | Avoid inventing an actor or authority. |
| Are there additional institution-specific restoration prerequisites? | Locked C02 rules / institutional policy | Avoid creating local rules. |
| Which downstream domains consume `StudentRestored` and what exact business consequences do they own? | Applicable downstream L2 architectures | Preserve downstream ownership boundaries. |

## 17.4 Locking Consequence

C-011 is suitable as a **source-grounded Draft**.

The unresolved Finance and disciplinary details do not alter:

- Final L3 ID;
- title;
- Primary Owner;
- participating domains;
- Type;
- Priority;
- restoration outcome;
- `StudentRestored` publisher; or
- Student Lifecycle ownership.

They should be resolved before final Architecture Approval/Locking if detailed cross-domain contracts are required.

---

# Validation and Review Classification

## Repository Identity Check

- **Final ID:** `L3-C-011`
- **Title:** Student Restoration After Suspension
- **Primary Owner:** Student Lifecycle
- **Participating Domains:** C, F
- **Type:** Critical
- **Priority:** P1
- **Assigned Author:** Varad
- **Section Folder:** `section-c`
- **Exact Filename:** `L3-C-011_Student_Restoration_After_Suspension.md`

The frozen Stage 3 register confirms the C-011 semantics.

The Stage 4 register establishes Section C's direct positional final-ID mapping.

## L2 Workflow Check

The locked C02 workflow catalogue explicitly defines:

`WF-C02-07 — Student Restoration — Restore suspended student`.

Its authoritative definition states:

- Purpose: restore a suspended Student to Active;
- Triggers: Administrative Approval, Financial Clearance, Disciplinary Resolution;
- Preconditions: Student Suspended, Restoration authorized;
- Postcondition: Status = Active;
- Event: `StudentRestored`;
- Publisher: Student Lifecycle.

## Ownership Check

Student Lifecycle remains the canonical owner of Student, StudentContext, and lifecycle state.

Finance remains a participating domain without acquiring Student ownership.

## State Check

The scenario preserves the canonical restoration path:

**Suspended → Restored → Active**

No new lifecycle state is created.

## Event Check

Canonical event:

`StudentRestored`

Canonical publisher:

**Student Lifecycle**.

## Boundary Check

C-011 does not absorb:

- Student Suspension;
- Intra-Institute Student Academic Reassignment;
- Student Transfer;
- Programme Migration;
- Academic Registration;
- Student Exit / Archive;
- Finance-owned financial processing.

## Cross-Domain Check

Finance remains a participating domain exactly as frozen by Stage 3.

The Finance clearance boundary is preserved without inventing Finance-owned rules or contracts.

## Audit Check

The authoritative workflow explicitly ends with:

**Publish `StudentRestored` → Audit → Complete.**

## Implementation Leakage Check

No APIs, payloads, database schemas, Business Data Structures, services, queues, infrastructure, UI, technical access control/IAM, deployment mechanics, technical retries, rollback, or exception-handling implementation is introduced.

## Template Check

All 17 mandatory L3 sections are present:

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

---

# Final Classification

**PASS WITH ISSUES — Draft**

C-011 is a complete, source-grounded L3 business scenario preserving the frozen identity and locked C02 restoration architecture.

The remaining issues are limited to detailed upstream Finance/disciplinary collaboration that the available authoritative C02 material does not fully specify. Those gaps are explicitly recorded rather than invented.

**Status remains `Draft`.**

AI authoring is not Architecture Approval or Locking.
