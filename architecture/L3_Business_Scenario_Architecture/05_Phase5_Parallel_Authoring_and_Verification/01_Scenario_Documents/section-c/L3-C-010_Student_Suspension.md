# Student Suspension

**Scenario ID:** L3-C-010  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** C — Student Lifecycle; F — Finance  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L2-C02 — Student Lifecycle & Activation Workflows; applicable locked Finance workflow governing the participation of Finance in suspension-related business consequences  
**Referenced By:** None yet

---

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-010` |
| Canonical Title | Student Suspension |
| Primary Owner | Student Lifecycle |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |
| Applicable L2 Capability | Student Suspension |
| Applicable L2 Workflow | `WF-C02-06 — Student Suspension` |
| Canonical Owner | Student Lifecycle |

The frozen Section C Stage 3 register identifies C-010 as **Student Suspension**, with **Student Lifecycle** as Primary Owner, **C, F** as participating domains, **Critical** Type, **P1** Priority, and `KEEP` derivation. fileciteturn120file0L2-L2

The Stage 4 final-ID register establishes the C section as a direct positional mapping and therefore assigns the tenth canonical Section C scenario the final identity `L3-C-010`. fileciteturn121file0L2-L2

The Phase 5 allocation assigns the complete C `001–075` block to Varad. C-010 therefore belongs to the current author's locked workload allocation. fileciteturn111file0L2-L2

**Exact Phase 5 filename:** `L3-C-010_Student_Suspension.md`

---

## 2. Scenario Purpose

This scenario governs the institutional suspension of a Student when the authoritative Student Lifecycle process determines that the Student's institutional relationship must enter the canonical suspended lifecycle position.

The scenario ensures that suspension is treated as a Student Lifecycle outcome rather than as a deletion, termination, or transfer of Student ownership.

Student Lifecycle remains the canonical owner of the Student Aggregate and StudentContext throughout the suspension lifecycle. Academic and administrative domains may participate in or respond to the suspension without acquiring ownership of the Student or StudentContext.

The locked L2-C02 architecture identifies Student Suspension as a core Student Lifecycle responsibility and explicitly separates it from Student Restoration, Student Transfer, Student Exit / Archive, and other lifecycle capabilities. fileciteturn125file0L2-L2

---

## 3. Business Outcome

The singular institutional business outcome is:

> **The Student has been placed into the authoritative suspended lifecycle position under the applicable institutional decision, the suspension is recorded with its business evidence and history, and the canonical suspension outcome is available to participating downstream domains.**

Successful completion means that the Student's institutional lifecycle reflects suspension.

Suspension does not terminate the Student's identity, erase the Student's history, transfer Student ownership, or itself complete Student Exit / Archive.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authoritative business condition or decision requires suspension of an active Student and the Student Suspension workflow is therefore initiated.

The precise initiating business authority must remain the one defined by the locked upstream Student Lifecycle architecture; this L3 does not manufacture a new role or authority label.

## 4.2 End Boundary

The scenario ends when:

1. the governed suspension decision has been applied;
2. the Student's lifecycle position reflects suspension;
3. the suspension is recorded as part of the Student's historical lifecycle;
4. required audit evidence has been recorded; and
5. the canonical suspension business outcome has been made available to participating domains according to upstream architecture.

The Main Business Flow stops at the completed suspension outcome.

It does not continue into restoration, exit, finance settlement, academic remediation, or any other downstream business consequence.

## 4.3 In Scope

- Suspension of a Student.
- Validation of the Student's applicable lifecycle position.
- Application of the authoritative suspension outcome.
- Preservation of suspension history.
- Required business audit/evidence.
- Cross-domain participation with Finance where defined by upstream architecture.
- Publication/consumption of canonical business events where defined upstream.

## 4.4 Out of Scope

- Student Restoration After Suspension — `L3-C-011`.
- Student Status Change where the change is not a suspension.
- Student Exit / Archive.
- Student Transfer.
- Academic Progression decisions.
- Academic Registration.
- Fee calculation or accounting.
- Any Finance-owned financial consequence beyond the governed participation relevant to suspension.
- Deletion of Student or StudentContext.
- Technical authorization, APIs, databases, services, queues, UI, deployment, or infrastructure.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by / Related | `L3-C-011 — Student Restoration After Suspension` | Restoration is a distinct canonical outcome owned by Student Lifecycle and is not part of suspension completion. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| **C — Student Lifecycle** | Canonical owner of Student, StudentContext, Student Status and the suspension lifecycle transition. |
| **F — Finance** | Participates where suspension has a Finance-owned business consequence or governed cross-domain response; Finance ownership remains limited to its own financial concepts and lifecycle. |

The locked L2-C02 architecture establishes Student Lifecycle as the exclusive owner of the Student Aggregate and associated StudentContext and identifies Student Status and Student Suspension among its responsibilities. fileciteturn125file0L2-L2

Finance participation does not transfer ownership of Student, StudentContext, or Student Status.

No Finance-owned concept is re-owned by Student Lifecycle merely because suspension may affect it.

---

## 6. Business Actors, Roles and Responsibilities

The retrieved authoritative L2 material establishes Student Lifecycle as the owning domain and identifies the Student Suspension workflow, but the retrieved material does not safely establish a complete canonical human-role matrix for every suspension initiation, review, decision, or execution responsibility.

Accordingly, this scenario does not invent role names.

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student Lifecycle | Owns the Student suspension lifecycle and applies the canonical suspension outcome. | Canonical owning domain. |
| Finance | Participates in suspension-related business consequences where governed by its own authoritative architecture. | Participating domain; retains ownership of Finance concepts. |
| Authoritative institutional suspension decision-maker | Initiates or establishes the suspension decision where required by upstream policy. | Exact canonical role title is not asserted without authoritative upstream actor evidence. |

No technical access control/technical identity profiles is defined.

---

## 7. Preconditions and Entry Criteria

The scenario may begin only when the authoritative Student Lifecycle suspension process permits suspension for the Student.

The following business conditions are preserved from the Student Lifecycle lifecycle boundary:

- the Student exists as a canonical Student;
- the Student is within the Student Lifecycle domain;
- the Student is in a lifecycle position from which suspension is permitted by the authoritative lifecycle;
- the business basis for suspension has been established according to the governing institutional process;
- any required upstream decision or authority has been established before the lifecycle transition is applied.

No local suspension eligibility rule is invented.

Where the authoritative L2 source does not expose a sufficiently specific human-role or decision matrix, the missing detail remains an Open Issue rather than being converted into a new L3 rule.

---

## 8. Trigger

The business trigger is:

> **An authoritative institutional suspension condition or decision requires the Student's institutional lifecycle to enter the suspended position.**

The trigger is intentionally expressed at business level.

It is not expressed as an API call, database update, service invocation, scheduled job, message-broker operation, or other implementation mechanism.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the authoritative business condition or decision requiring Student suspension. | Student Lifecycle / authorised institutional participant | Suspension processing begins. |
| 2 | Confirm that the Student is eligible for the suspension transition under the authoritative Student Lifecycle rules. | Student Lifecycle | Valid suspension candidate is established. |
| 3 | Apply the authoritative suspension decision to the Student's lifecycle. | Student Lifecycle | Student enters the canonical suspended lifecycle position. |
| 4 | Preserve the Student's prior lifecycle information and the suspension history. | Student Lifecycle | Historical lifecycle remains available and ownership is preserved. |
| 5 | Record the required business audit/evidence associated with the suspension. | Student Lifecycle | Suspension becomes auditable. |
| 6 | Make the canonical suspension outcome available to participating domains according to the authoritative event/collaboration architecture. | Student Lifecycle | Participating domains can apply their own governed consequences. |
| 7 | End the scenario. | Student Lifecycle | Student Suspension is complete. |

The locked L2-C02 architecture identifies Student Suspension as `WF-C02-06` within the Student Lifecycle workflow catalogue and identifies Student Status as a Student Lifecycle-owned capability. fileciteturn125file0L2-L2

The scenario deliberately does not add technical workflow mechanics.

---

## 10. Alternate Business Flows

No scenario-specific alternate business flow is asserted beyond the authoritative suspension workflow because the retrieved source material does not safely establish distinct governed alternative paths that achieve the same suspension outcome.

The distinction between **suspension** and **restoration** is handled as a separate lifecycle scenario rather than an alternate path within C-010.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Student is not eligible for suspension under the authoritative lifecycle | Suspension cannot validly complete. | Do not apply the suspension transition; resolve the governing business condition through the authoritative upstream process. | Student Lifecycle |
| Required suspension decision/authority is absent | The suspension outcome cannot be validly established. | Obtain the required authoritative business decision before applying suspension. | Authoritative institutional process / Student Lifecycle |
| Requested action is actually restoration rather than suspension | C-010 does not own the requested outcome. | Apply `L3-C-011 — Student Restoration After Suspension`. | Student Lifecycle |
| Requested action is actually Student Exit / Archive | C-010 does not own the terminal lifecycle outcome. | Apply the applicable Student Exit / Archive process. | Student Lifecycle |
| Requested consequence belongs to Finance | C-010 must not assume Finance ownership. | Finance applies its own authoritative business response where one is defined. | Finance |

No technical retry, queue, rollback, timeout, server failure, exception handling, or infrastructure recovery is defined.

Suspension is a business lifecycle transition, not a technical failure state.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Student lifecycle is canonically owned by Student Lifecycle. | Student Lifecycle | Suspension changes the Student lifecycle without transferring ownership. | Locked L2-C02 |
| Student Status is owned by Student Lifecycle. | Student Lifecycle | Suspension affects the Student lifecycle position within C02 ownership. | Locked L2-C02 |
| Student Suspension is a distinct Student Lifecycle capability. | Student Lifecycle | Suspension is not collapsed into generic status change, transfer, or exit. | `WF-C02-06` workflow catalogue |
| Historical Student lifecycle information is preserved. | Student Lifecycle | Suspension does not erase prior lifecycle history. | Locked L2-C02 lifecycle principles |
| Student records are not physically deleted as part of suspension. | Student Lifecycle | Suspension remains reversible/continuable through separate lifecycle processes rather than deletion. | Locked L2-C02 lifecycle principles |
| Finance remains owner of Finance concepts. | Finance | Any financial consequence of suspension remains Finance-owned. | Cross-domain ownership principle / applicable Finance L2 |

No local `BR-01`-style rule identifiers are manufactured.

No additional suspension criterion is invented.

---

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| StudentContext / Student lifecycle | Authoritative pre-suspension lifecycle position | Suspension is applied as the governed lifecycle transition. | **Suspended** | Student Lifecycle | `WF-C02-06` / locked C02 lifecycle |
| Student Aggregate | Existing Student | Student remains the canonical institutional identity. | Student remains active as an identity; institutional lifecycle reflects suspension. | Student Lifecycle | Locked L2-C02 |
| Student history | Existing historical lifecycle | Suspension adds a new historical lifecycle outcome without erasing prior history. | Historical suspension evidence preserved. | Student Lifecycle | Locked L2-C02 |
| Finance-owned concepts, if affected | Existing applicable Finance lifecycle | Finance may apply its own governed business consequence. | Finance-owned lifecycle remains Finance-owned. | Finance | Applicable Finance L2 |

No new lifecycle state is invented.

The scenario uses **Suspended** only as the canonical lifecycle position where established by the locked Student Suspension workflow/lifecycle architecture.

Student Suspension does not itself transition the Student to Exit, Archived, Alumni, or another terminal lifecycle position.

---

## 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

The authoritative L2-C02 architecture establishes that lifecycle transitions publish canonical domain events. fileciteturn125file0L2-L2

For C-010, the canonical suspension event must be the exact event defined by `WF-C02-06` in the locked L2 event catalogue.

**Important source-control note:** the retrieved source excerpt available for this authoring pass does not expose the exact event-name row for `WF-C02-06`. Therefore this L3 document does **not** manufacture or rename a suspension event.

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Exact canonical Student Suspension event — **name to be taken verbatim from locked C02 event catalogue** | Student Lifecycle | Participating domains defined upstream | Announces the completed suspension business fact. | Locked L2-C02 `WF-C02-06` / event catalogue |

This preserves the non-negotiable rule that L3 must use the canonical event name and publisher rather than inventing one.

## 14.2 Synchronous Business Collaboration

Student Lifecycle remains responsible for the Student suspension outcome.

Where Finance must respond to the completed suspension, the collaboration is a business hand-off in which Finance applies its own Finance-owned consequences.

No technical integration mechanism is defined.

## 14.3 Ownership Preservation

Participation by Finance does not transfer:

- Student ownership;
- StudentContext ownership;
- Student Status ownership;
- Student Suspension lifecycle ownership; or
- canonical event publication ownership.

---

## 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

The suspension must be based on the authoritative institutional authority and business process defined upstream.

The retrieved L2 source does not safely expose a complete canonical human-role matrix for the suspension decision. C-010 therefore does not create a role such as `Suspension Authority`, `Disciplinary Committee and Dean of Student Affairs`, or equivalent.

## 15.2 Approval / Decision

Where the authoritative Student Suspension workflow requires an institutional suspension decision, that decision remains governed by upstream architecture.

C-010 applies the authoritative outcome; it does not create a new approval rule.

## 15.3 Audit

The locked Student Lifecycle architecture establishes immutable audit treatment for lifecycle transitions. fileciteturn125file0L2-L2

C-010 therefore requires the suspension outcome and its material business evidence to be auditable.

## 15.4 Separation of Duty

No specific suspension separation-of-duty rule is asserted because the retrieved authoritative material does not establish one sufficiently to reproduce it safely.

No local separation-of-duty rule is invented.

## 15.5 Historical Evidence

The suspension becomes part of the Student's historical lifecycle.

Prior Student lifecycle history remains preserved.

---

## 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

C-010 is complete when:

1. the Student is validly eligible for the suspension transition;
2. the authoritative suspension condition or decision has been established;
3. the Student's lifecycle has entered the canonical suspended position;
4. prior lifecycle history has been preserved;
5. required audit/evidence has been recorded; and
6. the canonical suspension outcome has been made available to participating domains according to the authoritative event architecture.

## 16.2 Postconditions

| Postcondition | Result |
|---|---|
| Student | Remains owned by Student Lifecycle |
| StudentContext | Remains owned by Student Lifecycle |
| Student lifecycle | Reflects suspension |
| Student history | Prior lifecycle history preserved |
| Suspension evidence | Recorded according to authoritative audit obligations |
| Finance | May apply its own governed consequence where defined |
| Event | Canonical suspension event is published by Student Lifecycle, using the exact upstream event name |
| Restoration | Not performed; remains a separate scenario |
| Exit / Archive | Not performed; remains a separate scenario |

## 16.3 Downstream Consequence Boundary

Any Finance, academic, administrative, communication, or other downstream consequence of the suspension belongs to the relevant canonical domain and its own architecture.

C-010 ends at the completed Student Suspension business outcome.

## 16.4 End Boundary

The scenario ends when the Student's suspension lifecycle outcome has been established and the required business evidence and canonical cross-domain notification have been completed.

---

## 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

### OI-C010-01 — Exact Suspension Event Name

The retrieved L2-C02 source confirms that lifecycle transitions publish canonical domain events but the accessible excerpt for this authoring pass does not expose the exact `WF-C02-06` event-catalogue row.

**Impact:** Material to Section 14 because the L3 architecture must use the exact canonical event name and publisher.

**Disposition:** Do not invent the event name. Before Architecture Approved/Locked, verify the exact event name and any canonical consumers directly from the locked L2-C02 event catalogue.

**Classification impact:** Non-blocking for this Draft, but blocking for final locking if the event name remains unresolved.

---

### OI-C010-02 — Exact Suspension Decision Actor / Role

The retrieved L2-C02 source establishes Student Suspension as a Student Lifecycle workflow but does not expose a sufficiently specific canonical human-role matrix for the suspension decision in the accessible source excerpt.

**Impact:** Prevents safe invention of an actor or authority title.

**Disposition:** Preserve the business responsibility without creating a new role. Resolve from authoritative actor/role architecture before Architecture Approved/Locked if role-specific authorization is required.

---

### OI-C010-03 — Exact Finance Participation

C-010's frozen semantics explicitly identify Finance as a participating domain, but the accessible C02 source does not expose the complete Finance-side suspension consequence contract.

**Impact:** Finance participation is known, but the exact Finance-owned business response must not be invented.

**Disposition:** Retain Finance as a participating domain and keep its responsibility at the ownership boundary. Verify the applicable locked Finance L2 workflow before locking any detailed Finance consequence.

---

### OI-C010-04 — Exact Suspension Eligibility Conditions

The accessible source establishes the Student Suspension workflow and lifecycle ownership but does not expose the complete detailed eligibility matrix for every possible suspension condition.

**Impact:** Prevents invention of suspension grounds or local eligibility rules.

**Disposition:** Apply only the upstream-defined eligibility conditions. Any detailed matrix not retrieved remains deferred.

---

## 17.2 Assumptions

1. The frozen Stage 3 C-010 row is authoritative for title, owner, participating domains, Type and Priority.
2. `L3-C-010` is the final canonical identity.
3. `WF-C02-06 — Student Suspension` is the applicable locked L2 workflow.
4. Student Lifecycle owns the suspension outcome.
5. Finance participates without acquiring Student lifecycle ownership.
6. Suspension is distinct from Restoration.
7. Suspension is distinct from Student Exit / Archive.
8. Student identity and historical lifecycle information are preserved.
9. Lifecycle transitions are auditable.
10. The exact canonical suspension event name must be copied from the locked L2-C02 event catalogue and is deliberately not invented in this Draft.

These assumptions do not override locked architecture.

## 17.3 Deferred Questions

| Question | Required Authority | Reason |
|---|---|---|
| What is the exact canonical suspension event name? | Locked L2-C02 event catalogue | Required for exact event traceability. |
| Which exact human role(s) may decide or initiate suspension? | Locked actor/role architecture | Not safely exposed in retrieved source. |
| What exact Finance-owned business consequence follows suspension? | Locked Finance L2 architecture | C-010 semantics identify Finance participation but do not define its internal lifecycle. |
| What exact suspension grounds and eligibility conditions apply? | Locked C02 rules / institutional policy | Avoid inventing local suspension criteria. |

## 17.4 Locking Consequence

C-010 is suitable as a **Draft**, but the exact suspension event name must be verified before final Architecture Approval/Locking.

No unresolved item changes the frozen:

- Final L3 ID;
- title;
- Primary Owner;
- participating domains;
- Scenario Type;
- Priority; or
- core business outcome.

---

# Validation and Review Classification

## Repository Identity Check

- **Final ID:** `L3-C-010`
- **Title:** Student Suspension
- **Primary Owner:** Student Lifecycle
- **Participating Domains:** C, F
- **Type:** Critical
- **Priority:** P1
- **Assigned Author:** Varad
- **Section Folder:** `section-c`
- **Exact Filename:** `L3-C-010_Student_Suspension.md`

The Stage 3 register confirms the frozen C-010 semantics. fileciteturn120file0L2-L2

The Stage 4 register confirms the final C-section identity scheme. fileciteturn121file0L2-L2

## L2 Ownership Check

L2-C02 is locked and identifies Student Lifecycle as the canonical owner of the Student Aggregate and StudentContext, with Student Status and Student Suspension among its responsibilities. fileciteturn125file0L2-L2

## Workflow Check

The locked C02 workflow catalogue explicitly includes:

`WF-C02-06 — Student Suspension — Temporarily suspend student`

and separately defines:

`WF-C02-07 — Student Restoration — Restore suspended student`

and:

`WF-C02-10 — Student Exit / Archive — Complete institutional relationship`. fileciteturn125file0L2-L2

This preserves the canonical separation between suspension, restoration, and terminal exit.

## Ownership Check

Student Lifecycle remains the sole owner of the Student Aggregate and StudentContext.

Finance participation does not transfer ownership. fileciteturn125file0L2-L2

## Lifecycle Check

C-010 applies the suspension lifecycle outcome without creating a new local state or merging suspension with exit, restoration, or transfer.

## Event Check

The authoritative C02 architecture confirms canonical event publication for lifecycle transitions. fileciteturn125file0L2-L2

The exact `WF-C02-06` event name is intentionally **not invented** because the accessible source excerpt does not expose the event-catalogue row.

## Cross-Domain Check

Finance is retained as a participating domain exactly as frozen by Stage 3.

No Finance-owned business concept is re-owned by Student Lifecycle.

## Boundary Check

C-010 does not absorb:

- Student Restoration After Suspension;
- Student Exit / Archive;
- Student Transfer;
- Academic Progression;
- Academic Registration;
- Finance-owned accounting or fee processing.

## Implementation Leakage Check

No APIs, payloads, databases, Business Data Structures, services, queues, infrastructure, UI, technical access control/IAM, deployment mechanics, technical retries, rollback, or exception handling are introduced.

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

The locked v2.0 template establishes this 17-section canonical structure and requires the scenario to remain within L3 business architecture rather than implementation design. fileciteturn113file0L2-L2

---

# Final Classification

**PASS WITH ISSUES — Draft**

C-010 is a source-grounded L3 Draft preserving the frozen C-010 identity and the locked Student Lifecycle ownership boundary.

The principal unresolved item is the **exact canonical suspension event name**, which must be verified directly from the locked `WF-C02-06` event catalogue before the scenario can be considered fully lock-ready.

No unsupported event name, actor, suspension rule, Finance consequence, lifecycle state, or ownership transfer has been invented.

**Status remains `Draft`.**

AI authoring is not Architecture Approval or Locking.
