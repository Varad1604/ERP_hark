# Submitted Internal Assessment Outcome to Governed Moderated Assessment Outcome

**Scenario ID:** L3-D-054  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0774 / CAN-L3-D-054

---

# 2. Scenario Purpose

Govern moderation of a submitted internal-assessment outcome while preserving the original submission and mark audit trail.

# 3. Business Outcome

The Assessment Moderation Case is approved or rejected with reason, and an approved moderated outcome becomes the governed input to finalization without erasing the original submission.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A recorded Assessment Submission requires moderation under assessment policy or authorized review.

## 4.2 End Boundary

Moderation reaches an approved or rejected terminal decision and `AssessmentModerationCompleted` is published.

## 4.3 In Scope

- Create moderation case linked to original submission.
- Review submission evidence, awarded marks, blueprint and policy.
- Approve or reject moderated outcome with immutable audit history.

## 4.4 Out of Scope

- Changing original submission history in place.
- Finalization of the whole internal assessment.
- Examination-grade computation.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns AssessmentModerationCase, moderation evidence, decision and approved moderated outcome. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Policy-configured D07 moderation requester | Requests moderation for the submitted outcome. | RequestAssessmentModeration authority. |
| Policy-configured independent moderator | Reviews evidence and recommends/records moderated outcome. | Moderation responsibility. |
| Policy-configured D07 moderation authority | Approves or rejects moderation decision. | Approve/RejectAssessmentModeration authority. |

# 7. Preconditions and Entry Criteria

- Assessment Submission is recorded.
- Original evidence, marks, blueprint and policy binding are available.

# 8. Trigger

Assessment policy, quality control or authorized review requires moderation.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create Moderation Case linked to immutable original submission. | D07 moderation requester | AssessmentModerationRequested. |
| 2 | Validate case, evidence, blueprint component and policy. | D07 | Review basis complete. |
| 3 | Independently review awarded marks and assessment evidence. | D07 moderator | Moderated recommendation recorded. |
| 4 | Approve or reject moderation with reason. | D07 moderation authority | Governed decision retained. |
| 5 | For approval, preserve original and record moderated outcome as new governed fact. | D07 | AssessmentModerationCompleted. |
| 6 | Route approved outcome to finalization; rejection retains prior valid outcome. | D07 | No historical evidence is overwritten. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Internal Assessment |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Internal Assessment |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Internal Assessment |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Internal Assessment | Prevents cross-domain ownership leakage. | `L2-D07_Internal_Assessment_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Internal Assessment | Prevents governance bypass. | `L2-D07_Internal_Assessment_Workflows.md` |
| Published or historical business facts remain immutable. | Internal Assessment | Preserves auditability and lineage. | `L2-D07_Internal_Assessment_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Assessment Moderation Case | Requested / submitted outcome | Review and approve or reject moderation. | Requested → Under Review → Approved or Rejected → Completed | D07 | L2-D07 §§5–8,11 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| AssessmentModerationRequested / AssessmentModerationCompleted | D07 | Finalization, Academic Monitoring and authorized consumers | Records governed moderation lifecycle. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates; completed events may still be consumed under canonical contracts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Requests moderation for the submitted outcome. | Policy-configured D07 moderation requester | RequestAssessmentModeration authority. | Internal Assessment |
| Reviews evidence and recommends/records moderated outcome. | Policy-configured independent moderator | Moderation responsibility. | Internal Assessment |
| Approves or rejects moderation decision. | Policy-configured D07 moderation authority | Approve/RejectAssessmentModeration authority. | Internal Assessment |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Internal Assessment |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Internal Assessment |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Internal Assessment |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Assessment Moderation Case is approved or rejected with reason, and an approved moderated outcome becomes the governed input to finalization without erasing the original submission.
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
**Validation Basis:** Section D canonical register; `L2-D07_Internal_Assessment_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
