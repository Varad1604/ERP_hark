# Approved Participation to Governed External Learning Registration

**Scenario ID:** L3-D-037  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0758 / CAN-L3-D-038 (final L3-D-037)

---

# 2. Scenario Purpose

Register an institution-approved learner with an approved external learning provider using an idempotent, traceable and recoverable integration workflow.

# 3. Business Outcome

The External Learning Registration is accepted with one immutable provider enrollment identifier linked to the institutional enrollment, or remains in a governed recoverable failure state.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A D06 Flexible Learning Enrollment is Approved for an external opportunity and provider prerequisites are healthy.

## 4.2 End Boundary

The learner is registered, `LearnerRegisteredWithProvider` is published and provider enrollment status synchronization is established.

## 4.3 In Scope

- Validate enrollment, provider, identity and integration health.
- Submit idempotent provider registration.
- Persist immutable provider enrollment identifier.
- Record failure and support governed retry/recovery.

## 4.4 Out of Scope

- Allowing provider ownership of institutional enrollment.
- Duplicate provider registration.
- Credit recognition or transcript posting.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns institutional External Learning Registration, provider reference and synchronization history. |
| External Learning Provider | Returns provider-side registration acknowledgement; never becomes institutional system of record. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| D06 Integration Service | Builds, submits and records provider registration. | Authorized integration execution. |
| External Learning Provider | Accepts or rejects provider-side learner registration. | External acknowledgement only. |
| Integration Administrator | Resolves governed integration failures where automation cannot recover. | Authorized recovery action. |

# 7. Preconditions and Entry Criteria

- D06 enrollment is Approved.
- Provider is active, approved and healthy.
- Student identity synchronization and credentials are valid.

# 8. Trigger

Approved external-learning participation requests provider registration.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate enrollment, provider status, identity and integration health. | D06 Integration Service | Registration may proceed or is blocked. |
| 2 | Build a correlated idempotent registration request. | D06 | Duplicate provider enrollment prevented. |
| 3 | Authenticate and submit request to provider. | Integration Service | Provider processes request. |
| 4 | Validate acknowledgement and provider enrollment identifier. | D06 | Accepted registration evidence established. |
| 5 | Persist immutable provider reference and publish success. | D06 | LearnerRegisteredWithProvider. |
| 6 | On failure, retain recoverable state and retry/resolve without duplicating enrollment. | Integration Administrator / D06 | Retry Pending or governed failure resolution. |
| 7 | Synchronize subsequent provider enrollment status while D06 remains canonical owner. | D06 Synchronization Engine | EnrollmentStatusSynchronized. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Open Electives & MOOCs |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Open Electives & MOOCs |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Open Electives & MOOCs |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Open Electives & MOOCs | Prevents cross-domain ownership leakage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Open Electives & MOOCs | Prevents governance bypass. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Published or historical business facts remain immutable. | Open Electives & MOOCs | Preserves auditability and lineage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| External Learning Registration | Requested | Submit to provider, accept and activate or retain recoverable failure. | Requested → Submitted → Accepted → Active; alternate Rejected / Retry Pending / Failed | D06 | L2-D06 Aggregate 4; W33–W34 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| LearnerRegisteredWithProvider / ProviderRegistrationFailed / EnrollmentStatusSynchronized / SynchronizationRecovered / SynchronizationFailureResolved | D06 | Provider integration, progress and authorized operational consumers | Records registration, synchronization and recovery facts. | L2-D06 W33–W39; Event Catalogue |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| External Learning Provider | Open Electives & MOOCs | Returns provider-side registration acknowledgement; never becomes institutional system of record. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Builds, submits and records provider registration. | D06 Integration Service | Authorized integration execution. | Open Electives & MOOCs |
| Accepts or rejects provider-side learner registration. | External Learning Provider | External acknowledgement only. | Open Electives & MOOCs |
| Resolves governed integration failures where automation cannot recover. | Integration Administrator | Authorized recovery action. | Open Electives & MOOCs |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Open Electives & MOOCs |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Open Electives & MOOCs |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Open Electives & MOOCs |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The External Learning Registration is accepted with one immutable provider enrollment identifier linked to the institutional enrollment, or remains in a governed recoverable failure state.
- All required authority, validation and audit evidence is retained.
- Applicable completed business facts are published exactly once in business effect.

## 16.2 Postconditions

- Canonical ownership remains unchanged.
- Historical decisions and prior states remain immutable.
- Downstream domains consume facts without modifying the owning aggregate.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D06_Open_Electives_and_MOOCs_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
