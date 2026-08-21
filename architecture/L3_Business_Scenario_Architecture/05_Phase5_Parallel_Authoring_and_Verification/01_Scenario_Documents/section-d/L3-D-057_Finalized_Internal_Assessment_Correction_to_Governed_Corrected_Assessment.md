# Finalized Internal Assessment Correction to Governed Corrected Assessment

**Scenario ID:** L3-D-057  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0777 / CAN-L3-D-057

---

# 2. Scenario Purpose

Correct a Finalized Internal Assessment through the controlled D07 moderation/correction workflow while preserving the original finalized audit trail and informing Section I through completed canonical facts.

# 3. Business Outcome

An authorized correction is recorded as new history, moderated and re-finalized as the current governed D07 outcome; the original finalized facts remain immutable and traceable.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A justified discrepancy or approved correction requirement identifies a Finalized Internal Assessment needing correction.

## 4.2 End Boundary

The corrected outcome is approved, re-finalized and communicated through `AssessmentSubmissionUpdated`, `AssessmentModerationCompleted` and `InternalAssessmentFinalized` without direct Section I mutation.

## 4.3 In Scope

- Request correction with reason/evidence.
- Open governed moderation/correction case.
- Update marks only through authorized correction history.
- Approve/reject, re-finalize and publish canonical completed facts.

## 4.4 Out of Scope

- Direct edit of finalized data.
- Deletion of original marks or evidence.
- Direct correction of Section I examination/result records.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns finalized internal-assessment correction, moderation, updated marks and re-finalization. |
| Section I Examination | Consumes corrected finalized facts and governs any examination-side consequence. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Policy-configured D07 correction requester | Submits reason and evidence for correction. | Initiates controlled correction. |
| Policy-configured independent moderator | Reviews original outcome, evidence and proposed correction. | Independent review. |
| Policy-configured D07 correction/finalization authority | Approves or rejects correction and authorizes re-finalization. | Final correction authority. |
| Section I Examination | Consumes corrected finalized fact. | No D07 correction authority. |

# 7. Preconditions and Entry Criteria

- Internal Assessment is Finalized and not irreversibly Archived under policy.
- Original marks/evidence and correction justification exist.
- Applicable correction authority and policy are available.

# 8. Trigger

A validated error, appeal, audit finding or authorized academic decision requires correction of finalized internal assessment.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create correction/moderation case linked to original Finalized record. | D07 correction requester | AssessmentModerationRequested with correction evidence. |
| 2 | Validate authority, reason, evidence and affected submission/component. | D07 | Correction may proceed or is rejected. |
| 3 | Independently review original marks and proposed correction. | D07 moderator | Moderated recommendation recorded. |
| 4 | Approve or reject correction with explicit reason. | D07 correction authority | Decision retained. |
| 5 | For approval, record updated marks as new history; never rewrite original fact. | D07 | AssessmentSubmissionUpdated. |
| 6 | Complete moderation and re-finalize corrected assessment. | D07 finalization authority | AssessmentModerationCompleted and InternalAssessmentFinalized. |
| 7 | Publish corrected finalized facts for Section I handling. | D07 | Section I consumes without ownership transfer. |

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
| Assessment Moderation/Correction Case | Finalized assessment / correction requested | Review, decide, update through history and re-finalize. | Requested → Under Review → Approved/Rejected → Corrected → Finalized | D07 | L2-D07 §§2,5–8,11 |
| Original Finalized Assessment | Finalized | Reference and preserve; never edit in place. | Remains immutable historical fact | D07 | L2-D07 §§5,11 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| AssessmentModerationRequested / AssessmentSubmissionUpdated / AssessmentModerationCompleted / InternalAssessmentFinalized | D07 | Section I Examination, Academic Monitoring and authorized consumers | Records controlled correction, moderation and corrected re-finalization using existing canonical events. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section I Examination | Internal Assessment | Consumes corrected finalized facts and governs any examination-side consequence. | Preserve ownership while enabling the scenario. | `L2-D07_Internal_Assessment_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Submits reason and evidence for correction. | Policy-configured D07 correction requester | Initiates controlled correction. | Internal Assessment |
| Reviews original outcome, evidence and proposed correction. | Policy-configured independent moderator | Independent review. | Internal Assessment |
| Approves or rejects correction and authorizes re-finalization. | Policy-configured D07 correction/finalization authority | Final correction authority. | Internal Assessment |
| Consumes corrected finalized fact. | Section I Examination | No D07 correction authority. | Internal Assessment |

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

- An authorized correction is recorded as new history, moderated and re-finalized as the current governed D07 outcome; the original finalized facts remain immutable and traceable.
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
