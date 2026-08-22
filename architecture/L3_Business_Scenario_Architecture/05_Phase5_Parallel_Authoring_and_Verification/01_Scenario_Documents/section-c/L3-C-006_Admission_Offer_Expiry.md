# L3-C-006 — Admission Offer Expiry

**Architecture Level:** L3 — Business Scenario Architecture  
**Scenario ID:** `L3-C-006`  
**Scenario Title:** Admission Offer Expiry  
**Primary Owner:** Admissions  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Last Updated:** 2026-08-22  
**Applicable L2 Source:** Locked L2-C01 — Admissions Workflows  

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
| Status | Draft |

Frozen identity fields are preserved without reclassification.

## 2. Scenario Purpose

Govern the institutional expiry of an Admission Offer when required actions are not completed within the configured validity period, while preserving Admission ownership, immutable history, and the distinction between Expiry and other terminal Admission outcomes.

## 3. Business Outcome

The Admission Record reaches the locked L2-C01 terminal state `Expired` under configurable institutional policy, its history remains preserved, and the canonical expiry fact is represented by `AdmissionExpired` where the upstream event contract applies.

C006 does not determine a local validity period or invent a missing canonical lifecycle transition.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An Admission Offer exists and the configured validity condition applicable to that offer reaches its expiry point without the required action being completed.

### 4.2 End Boundary
The scenario ends when the Admission is authoritatively recorded as `Expired`, the historical outcome is preserved, and the applicable canonical expiry event is published according to the locked Admissions event architecture.

### 4.3 In Scope
- Evaluate the configured offer-validity condition.
- Establish Admission Offer Expiry.
- Preserve Admission history.
- Maintain the terminal `Expired` outcome.
- Apply the canonical `AdmissionExpired` business fact where the event contract is applicable.

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
| C — Admissions | Canonical owner of Admission Record, Admission lifecycle/status, expiry business determination, and Admission-domain events. |

No additional participating domain is introduced. Any downstream consumer of `AdmissionExpired` retains ownership of its own consequences.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Admissions | Evaluates the configured expiry condition and establishes the canonical Admission outcome. | Canonical owner. |
| Applicant | Is the subject/counterparty of the Admission Offer. | No expiry decision authority is invented. |
| Applicable institutional admission authority | Governs institutional policy where required. | Exact human-role title is not invented where L2-C01 leaves authority configuration-driven. |

## 7. Preconditions and Entry Criteria

- Admission Record exists.
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
| 4 | Establish the `Expired` Admission outcome under the authoritative lifecycle. | Admissions | Admission becomes terminally expired, subject to the upstream transition clarification in Section 17. |
| 5 | Preserve the Admission history and material evidence. | Admissions | Institutional history remains immutable. |
| 6 | Publish `AdmissionExpired` where the locked event contract applies. | Admissions | Downstream authorized consumers may respond. |

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
| Upstream transition definition is ambiguous | Do not invent a transition; retain the matter in Section 17 pending upstream clarification. |
| Downstream consequence is required | Owning downstream domain determines its own response after the completed Admission fact. |

No technical retry/rollback/timer/infrastructure recovery is introduced.

## 12. Business Rules and Decision Constraints

- Admissions is the sole owner of the Admission lifecycle.
- `Expired` is a terminal Admission state in locked L2-C01.
- Expiry occurs because required actions were not completed within a configured validity period.
- Expiry is configuration-driven; L3 must not hard-code institutional policy.
- Admission history is immutable and preserved.
- Expiry must not overwrite another terminal Admission outcome.
- Readmission, where permitted, creates a new Admission Record linked to preserved history rather than reopening the old terminal record.

## 13. State and Lifecycle Impact

| Concept | Upstream State Semantics | Owner |
|---|---|---|
| Admission Record | `Expired` is an alternate terminal state. | Admissions |
| Historical Admission | Remains immutable/closed after expiry. | Admissions |

Locked L2-C01 §6.9 identifies `Admission Initiated → Expired` as an allowed transition, while §6.9.1—the later matrix labelled canonical—does not list an expiry transition. L3 does not silently choose between those conflicting upstream statements.

## 14. Business Events and Cross-Domain Collaboration

| Business Event | Canonical Domain / Publisher | Purpose |
|---|---|---|
| `AdmissionExpired` | Admissions | Represents the completed Admission expiry fact and is listed by locked L2-C01 among Admission lifecycle/domain events and downstream Notification-consumed facts. |

The exact event contract detail promised later in L2-C01 is not relied upon here beyond the upstream-established event name/domain ownership. No payload, transport, API, or technical contract is introduced.

## 15. Business Authorization, Approval and Audit Obligations

- Expiry follows configuration-driven Admission policy.
- No fixed human authority or universal approval step is invented.
- Historical Admission evidence must remain preserved and auditable under L2-C01 lifecycle principles.
- L3 does not define technical IAM/RBAC controls.

## 16. Completion Criteria and Postconditions

C006 is complete when:

- the authoritative configured expiry condition is satisfied;
- no contradictory terminal outcome governs the Admission;
- the Admission is recorded as `Expired` under the clarified canonical lifecycle;
- historical evidence is preserved; and
- `AdmissionExpired` is published where the upstream event contract applies.

The existing Admission Record remains terminal and immutable; any later readmission is a separate governed outcome/new record.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C006-01 — UPSTREAM LOCKED-DOCUMENT CONFLICT:** L2-C01 §6.9 permits `Admission Initiated → Expired`, while §6.9.1 is explicitly labelled the canonical lifecycle transition matrix and omits any transition to `Expired`, despite `Expired` being defined as a terminal state elsewhere in the same locked document. The authoritative transition must be clarified upstream before C006 is Architecture Approved/Locked.

**OI-C006-02 — EVENT DETAIL:** L2-C01 lists `AdmissionExpired` among Admission domain/lifecycle events and as a downstream-consumed fact, but the retrieved locked document does not expose a dedicated detailed `AdmissionExpired` event-definition subsection matching the promise that these events are formally defined later. L3 therefore preserves the established event name/domain but does not invent missing event-detail semantics.

**Assumption:** Expiry validity remains configuration-driven exactly as stated in locked L2-C01; C006 does not establish the institution-specific period.

## Production Verification Record

- Frozen L3 identity preserved.
- Complete locked L2-C01 source re-read; stale “source not retrieved” caveat removed.
- `Expired` terminal state and configuration-driven expiry rule preserved.
- `AdmissionExpired` upstream event name preserved.
- Internal L2-C01 transition inconsistency explicitly escalated rather than silently resolved.
- No deadline, actor, event detail, or transition invented locally.
- Transient session citations removed; stable L2-C01 section references used.
- All 17 mandatory sections present.
- Status remains **Draft** pending upstream clarification and architecture review.
