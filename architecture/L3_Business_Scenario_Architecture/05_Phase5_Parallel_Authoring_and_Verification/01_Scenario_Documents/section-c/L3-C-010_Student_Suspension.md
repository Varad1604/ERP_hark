# Student Suspension

**Scenario ID:** L3-C-010  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** Locked L2-C02 — Student Lifecycle & Activation Workflows, especially `WF-C02-06 — Student Suspension`; applicable locked Finance architecture for Finance-owned consequences  

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-010` |
| Canonical Title | Student Suspension |
| Primary Owner | Student Lifecycle |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |
| Applicable L2 Workflow | `WF-C02-06 — Student Suspension` |

Frozen identity fields and exact Phase 5 filename are preserved.

## 2. Scenario Purpose

Govern the institutional suspension of a Student when the authoritative Student Lifecycle process determines that the Student must enter the canonical suspended lifecycle position, while preserving Student identity, history, ownership, and Finance ownership of any financial consequences.

## 3. Business Outcome

The Student enters the authoritative `Suspended` lifecycle position, the suspension reason/history is preserved and auditable, and Student Lifecycle publishes `StudentSuspended`. Participating domains may apply their own governed consequences without acquiring Student ownership.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An authoritative institutional suspension condition or decision requires an eligible Student to enter the suspended lifecycle position.

### 4.2 End Boundary
The scenario ends when suspension has been validly applied, the reason/history and audit evidence are preserved, and `StudentSuspended` has been published.

### 4.3 In Scope
- Validate eligibility for suspension.
- Apply the governed Student suspension outcome.
- Record the suspension reason and history.
- Preserve Student identity and prior lifecycle history.
- Publish `StudentSuspended`.
- Allow Finance to consume the completed suspension fact for Finance-owned consequences where applicable.

### 4.4 Out of Scope
- Student restoration after suspension.
- Student transfer or terminal exit/archive.
- Finance-owned fee, payment, accounting, or settlement decisions.
- Academic progression decisions.
- APIs, databases, services, queues, UI, deployment, or infrastructure.

### 4.5 Related Scenarios
| Relationship | Scenario |
|---|---|
| Successor / related | L3-C-011 — Student Restoration After Suspension |

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| C — Student Lifecycle | Canonical owner of Student, StudentContext, Student status, suspension lifecycle transition, and `StudentSuspended`. |
| F — Finance | Consumes the suspension fact where Finance-owned business consequences apply; retains ownership of all Finance concepts and decisions. |

Finance participation does not transfer Student ownership or event publication ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Student Lifecycle | Validates and applies the governed suspension and records the canonical outcome. | Canonical suspension owner. |
| Authoritative institutional suspension decision responsibility | Establishes the suspension decision where required by institutional policy. | Exact human-role title is not invented where locked C02 does not establish one. |
| Finance | Applies its own governed financial consequences after consuming the completed suspension fact. | Finance-owned decisions only. |

No technical actor or access-control model is introduced.

## 7. Preconditions and Entry Criteria

- Student exists.
- Student is in a lifecycle position from which suspension is permitted.
- The authoritative business basis/decision for suspension exists.
- Required policy conditions are satisfied.
- Finance participation, if any, remains governed by Finance-owned architecture.

## 8. Trigger

An authoritative institutional suspension condition or decision requires the Student to enter the suspended lifecycle position.

## 9. Main Business Flow

| Step | Business Behaviour | Owner / Responsibility | Result |
|---|---|---|---|
| 1 | Receive the authoritative suspension condition or decision. | Student Lifecycle | Suspension evaluation begins. |
| 2 | Validate the Student and permitted lifecycle transition. | Student Lifecycle | Student is eligible for suspension. |
| 3 | Apply the governed suspension. | Student Lifecycle | Student enters `Suspended`. |
| 4 | Record the suspension reason and preserve lifecycle history. | Student Lifecycle | Historical evidence preserved. |
| 5 | Record required audit evidence. | Student Lifecycle | Suspension is auditable. |
| 6 | Publish `StudentSuspended`. | Student Lifecycle | Completed business fact becomes available to authorized consumers. |
| 7 | Participating domains apply only their own governed consequences. | Finance and other authorized consumers where applicable | Ownership boundaries preserved. |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Suspension decision is withdrawn before completion | Do not apply suspension. | Existing lifecycle remains authoritative. |
| Finance has no applicable consequence | Finance takes no further action. | Suspension remains complete in Student Lifecycle. |

Restoration is not an alternate path; it is the distinct L3-C-011 scenario.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Student is not eligible for suspension | Do not transition the Student. |
| Required authoritative suspension decision is absent | Do not complete suspension until the governed decision exists. |
| Requested action is actually restoration, transfer, or exit | Route to the appropriate canonical scenario. |
| Finance consequence is disputed or unresolved | Finance resolves its own concept under Finance architecture; Student Lifecycle does not re-own it. |

No technical recovery mechanics are introduced.

## 12. Business Rules and Decision Constraints

- Student Lifecycle canonically owns Student lifecycle and status.
- Suspension is a distinct governed lifecycle transition.
- Prior Student lifecycle history must remain preserved.
- Suspension does not delete Student identity or complete terminal exit.
- Finance remains owner of Finance concepts and decisions.
- L3 applies locked upstream rules and does not invent local suspension criteria.

## 13. State and Lifecycle Impact

| Concept | Transition / Impact | Owner |
|---|---|---|
| Student lifecycle | Eligible pre-suspension position → `Suspended` | Student Lifecycle |
| Student identity | Remains unchanged. | Student Lifecycle |
| Student history | Suspension outcome/reason added without erasing prior history. | Student Lifecycle |
| Finance-owned concepts | May respond to suspension under Finance-owned lifecycle/rules. | Finance |

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

| Business Event | Canonical Publisher | Consumers / Purpose |
|---|---|---|
| `StudentSuspended` | Student Lifecycle | Announces successful governed suspension. Locked C02 identifies Finance among authorized business consumers. |

Locked L2-C02 explicitly defines `StudentSuspended` as `EVT-C02-006`, publishes it after successful suspension, and maps it to `WF-C02-06`. A generic `StudentStatusChanged` may also exist for generic status transitions, but downstream domains must not infer the dedicated suspension transition from the generic event when `StudentSuspended` exists.

## 15. Business Authorization, Approval and Audit Obligations

- Suspension must be based on the authoritative institutional decision/policy applicable upstream.
- This L3 does not invent a human role where C02 does not provide a canonical title.
- Suspension reason and material evidence must remain auditable.
- Finance participation does not authorize Finance to change Student lifecycle.
- No technical IAM/RBAC design is introduced.

## 16. Completion Criteria and Postconditions

C010 is complete when:

- the Student is validly eligible for suspension;
- the authoritative suspension condition/decision exists;
- the Student is in `Suspended`;
- reason/history and audit evidence are preserved;
- `StudentSuspended` is published by Student Lifecycle; and
- downstream consequences remain owned by their canonical domains.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No unresolved source-access or event-name issue remains: direct reconciliation with locked L2-C02 confirms `WF-C02-06`, `EVT-C02-006 StudentSuspended`, the `Suspended` lifecycle outcome, and Finance as an authorized consumer of the suspension fact.

The exact human institutional decision-role title remains intentionally unspecified because no canonical title is required to preserve the business outcome and none is invented here.

### 17.2 Assumptions

Any Finance consequence is determined exclusively by applicable locked Finance architecture after Finance consumes the suspension fact.

### 17.3 Deferred Questions

Technical synchronization and enforcement mechanisms remain outside L3.

## Production Verification Record

- Frozen `L3-C-010`, title, owner, C/F participants, Critical type, and P1 priority preserved.
- Exact `StudentSuspended` event restored from locked L2-C02.
- `Suspended` lifecycle position preserved from locked C02.
- Student Lifecycle publisher ownership preserved.
- Finance participation limited to Finance-owned consequences.
- Stale source-access caveat and transient session citations removed.
- All 17 mandatory sections present.
- Status remains **Draft** pending architecture review.
