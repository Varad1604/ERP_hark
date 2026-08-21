# Active Learning Opportunity to Governed Suspension

**Scenario ID:** L3-D-041  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0762 / CAN-L3-D-042 — split child A

---

# 2. Scenario Purpose

Temporarily suspend new enrollment into an Active Learning Opportunity through an authorized, reasoned and auditable decision while preserving the opportunity and existing approved enrollments.

# 3. Business Outcome

The Learning Opportunity becomes Suspended for the governed effective period, new enrollments are prohibited, and existing approved enrollments remain valid unless separately revoked.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An administrative, provider-outage or policy condition requires suspension of an Active Learning Opportunity.

## 4.2 End Boundary

The opportunity reaches Suspended and `LearningOpportunitySuspended` is published with reason, authority and effective period.

## 4.3 In Scope

- Validate Active state, suspension reason and authority.
- Apply temporary suspension and stop new enrollment.
- Notify affected students and preserve existing approved enrollments.
- Retain eligibility for separately governed reactivation.

## 4.4 Out of Scope

- Archival or deletion.
- Automatic revocation of existing approved enrollments.
- Modification of source Course Offering or provider master.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns Learning Opportunity availability, suspension state and suspension audit history. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Affairs | Requests and authorizes governed suspension. | Suspension authority. |
| D06 Learning Opportunity Service | Validates and applies suspension. | Execution only. |

# 7. Preconditions and Entry Criteria

- Learning Opportunity is Active.
- Suspension reason, authority and effective period are available.

# 8. Trigger

An authorized administrative decision, provider outage or policy condition requires temporary suspension.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register suspension request, reason, scope and effective period. | Academic Affairs | Governed request exists. |
| 2 | Validate Active state, authority, policy and affected enrollments. | D06 | Suspension is permitted or rejected. |
| 3 | Transition opportunity to Suspended and block new enrollment. | D06 | Active → Suspended. |
| 4 | Preserve existing approved enrollments unless separately governed. | D06 | No implicit revocation occurs. |
| 5 | Notify affected participants and publish completed fact. | D06 | LearningOpportunitySuspended. |

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
| Learning Opportunity | Active | Apply temporary governed enrollment suspension. | Active → Suspended | D06 | L2-D06 W06; Category A transition |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| LearningOpportunitySuspended | D06 | Enrollment, discovery and authorized operational consumers | Records temporary governed suspension. | L2-D06 W06; D06-E04 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates; completed events may still be consumed under canonical contracts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Requests and authorizes governed suspension. | Academic Affairs | Suspension authority. | Open Electives & MOOCs |
| Validates and applies suspension. | D06 Learning Opportunity Service | Execution only. | Open Electives & MOOCs |

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

- The Learning Opportunity becomes Suspended for the governed effective period, new enrollments are prohibited, and existing approved enrollments remain valid unless separately revoked.
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
