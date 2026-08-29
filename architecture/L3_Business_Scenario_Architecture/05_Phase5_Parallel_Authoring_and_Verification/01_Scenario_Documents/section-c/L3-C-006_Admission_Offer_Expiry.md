# L3-C-006 — Admission Offer Expiry

**Architecture Level:** L3 — Business Scenario Architecture  
**Scenario ID:** `L3-C-006`  
**Scenario Title:** Admission Offer Expiry  
**Primary Owner:** Admissions  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Author:** Varad  
**Version:** 1.1  
**Last Updated:** 2026-08-22  
**Applicable L2 Source:** Locked L2-C01 — Admissions Workflows, as amended by `L2-C01_Admissions_Workflows_Amendment_2026-08-22.md` under `ADR-STU-ADM-EXP-001`  

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-006` |
| Canonical Title | Admission Offer Expiry |
| Primary Owner | Admissions |
| Participating Domains | C |
| Scenario Type | Simple |
| Priority | P1 |
| Derivation | KEEP |
| Status | Locked |

Frozen identity fields are preserved without reclassification.

## 2. Scenario Purpose

Govern the institutional expiry of an Admission Offer when required actions are not completed within the configured validity period, while preserving Admissions ownership, immutable history, and the distinction between Expiry and other terminal Admission outcomes.

## 3. Business Outcome

The Admission Record follows the approved canonical transition `Admission Initiated → Expired`, reaches the terminal state `Expired` under configurable institutional policy, its history remains preserved, and Admissions publishes the canonical event `AdmissionExpired` after the expiry outcome is conclusively established.

C006 does not determine a local validity period or redefine the upstream lifecycle.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An Admission Offer exists in the `Admission Initiated` lifecycle position and the configured validity condition applicable to that offer reaches its expiry point without the required action being completed.

### 4.2 End Boundary
The scenario ends when the Admission is authoritatively recorded as `Expired`, the historical outcome is preserved, and `AdmissionExpired` is published by Admissions.

### 4.3 In Scope
- Evaluate the configured offer-validity condition.
- Establish Admission Offer Expiry.
- Apply `Admission Initiated → Expired`.
- Preserve Admission history.
- Publish `AdmissionExpired`.

### 4.4 Out of Scope
- Admission cancellation, withdrawal, or revocation.
- Readmission.
- Student creation.
- Finance-owned settlement/refund consequences.
- Technical schedulers, timers, APIs, queues, persistence, UI, or infrastructure.

### 4.5 Boundary Rule
Expiry must remain distinct from `Cancelled`, `Withdrawn`, and `Revoked`; those are separately governed terminal Admission outcomes.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| C — Admissions | Canonical owner of Admission Record, Admission lifecycle/status, expiry business determination, and `AdmissionExpired`. |

No additional participating domain is introduced. Any downstream consumer of `AdmissionExpired` retains ownership of its own consequences.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Admissions | Evaluates the configured expiry condition and establishes the canonical Admission outcome. | Canonical owner. |
| Applicant | Is the subject/counterparty of the Admission Offer. | No expiry decision authority. |
| Applicable institutional admission authority | Governs institutional expiry policy where required. | Exact human-role title remains institution-configured where upstream architecture does not fix one. |

## 7. Preconditions and Entry Criteria

- Admission Record exists in `Admission Initiated`.
- Admission Offer is governed by an applicable configured validity period/policy.
- Required actions have not been completed within that validity condition.
- Admission has not already reached a contradictory terminal outcome.

No fixed number of days or local deadline is introduced at L3.

## 8. Trigger

The configured validity period or expiry condition for the Admission Offer is reached before the required action is completed.

The trigger is a business condition; technical timing implementation remains outside L3.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | Identify the Admission Offer subject to expiry evaluation. | Admissions | Relevant Admission established. |
| 2 | Evaluate the configured validity condition. | Admissions | Expiry eligibility established or rejected. |
| 3 | Confirm no conflicting terminal outcome already governs the Admission. | Admissions | Lifecycle integrity preserved. |
| 4 | Apply the canonical transition `Admission Initiated → Expired`. | Admissions | Admission reaches terminal `Expired`. |
| 5 | Preserve the Admission history and material evidence. | Admissions | Institutional history remains immutable. |
| 6 | Publish `AdmissionExpired`. | Admissions | Downstream authorized consumers may respond. |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Required action completes before expiry | Do not expire the offer. | Admission continues under the applicable lifecycle. |
| Admission already `Expired` | Preserve the existing outcome. | No duplicate terminal transition. |
| Admission already `Cancelled`, `Withdrawn`, or `Revoked` | Do not replace that outcome with expiry. | Existing terminal outcome remains authoritative. |
| Configured expiry condition is not yet satisfied | No expiry is established. | Existing lifecycle position remains. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Applicable validity policy cannot be established | Do not determine expiry until the authoritative policy is available. |
| Conflicting terminal outcome exists | Preserve the existing terminal state. |
| Downstream consequence is required | Owning downstream domain determines its own response after `AdmissionExpired`. |

No technical retry/rollback/timer/infrastructure recovery is introduced.

## 12. Business Rules and Decision Constraints

- Admissions is the sole owner of the Admission lifecycle and publisher of `AdmissionExpired`.
- `Admission Initiated → Expired` is the approved canonical expiry transition under `ADR-STU-ADM-EXP-001`.
- `Expired` is a terminal Admission state.
- Expiry occurs because required actions were not completed within a configured validity period.
- Expiry is configuration-driven; L3 must not hard-code institutional policy.
- Admission history is immutable and preserved.
- Expiry must not overwrite another terminal Admission outcome.
- Readmission, where permitted, creates a new Admission Record linked to preserved history rather than reopening the old terminal record.

## 13. State and Lifecycle Impact

| Concept | Canonical State Semantics | Owner |
|---|---|---|
| Admission Record | `Admission Initiated → Expired`; `Expired` is terminal. | Admissions |
| Historical Admission | Remains immutable/closed after expiry. | Admissions |

The prior L2-C01 matrix inconsistency is resolved by the locked 2026-08-22 amendment; this scenario consumes that amendment and does not independently reinterpret the lifecycle.

## 14. Business Events and Cross-Domain Collaboration

| Business Event | Canonical Publisher | Purpose |
|---|---|---|
| `AdmissionExpired` | Admissions | Represents the conclusively established Admission expiry fact. |

No payload, transport, API, or technical contract is introduced.

## 15. Business Authorization, Approval and Audit Obligations

- Expiry follows configuration-driven Admissions policy.
- No fixed human authority or universal approval step is invented.
- Historical Admission evidence must remain preserved and auditable.
- L3 does not define technical IAM/RBAC controls.

## 16. Completion Criteria and Postconditions

C006 is complete when:

- the authoritative configured expiry condition is satisfied;
- no contradictory terminal outcome governs the Admission;
- `Admission Initiated → Expired` is applied;
- historical evidence is preserved; and
- Admissions publishes `AdmissionExpired`.

The existing Admission Record remains terminal and immutable; any later readmission is a separate governed outcome/new record.

## 17. Open Issues, Assumptions and Deferred Questions

**Open Issues:** None for the previously identified Admission expiry lifecycle conflict. It is resolved by `ADR-STU-ADM-EXP-001` and the locked L2-C01 amendment effective 2026-08-22.

**Assumption:** Expiry validity remains configuration-driven exactly as established by upstream architecture; C006 does not establish the institution-specific period.

**Deferred:** Implementation scheduling, notification delivery, and technical enforcement belong to later architecture levels.

## Production Verification Record

- Frozen L3 identity preserved.
- Locked L2-C01 amendment consumed.
- `Admission Initiated → Expired`, terminal `Expired`, and `AdmissionExpired` aligned to the approved upstream decision.
- Admissions ownership preserved.
- No deadline, actor, technical mechanism, or duplicate event invented locally.
- Stable repository traceability used.
- All 17 mandatory sections present.
- Status remains **Draft** pending normal L3 architecture review.
