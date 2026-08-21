# Open Assessment Opportunity to Governed Student Assessment Submission

**Scenario ID:** L3-D-053  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0773 / CAN-L3-D-053

---

# 2. Scenario Purpose

Capture an eligible student internal-assessment submission and awarded internal marks during a governed Open Assessment Window.

# 3. Business Outcome

A unique Assessment Submission is recorded against the correct student, blueprint component and window, with immutable submission identity and auditable mark updates before moderation/finalization.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An Assessment Window is Open and an authoritative eligible student attempts or completes the assessment opportunity.

## 4.2 End Boundary

The submission and assessor marks are recorded and `AssessmentSubmissionRecorded` is published.

## 4.3 In Scope

- Validate Open window, student, offering and component.
- Capture submission evidence and awarded marks.
- Permit governed pre-finalization mark update with audit.

## 4.4 Out of Scope

- Student master/registration ownership.
- Moderation or finalization.
- Official examination grade computation.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns AssessmentSubmission, captured submission record and awarded internal marks. |
| Section C | Owns canonical student and registration/eligibility facts consumed by D07. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Eligible student | Provides the assessment response/evidence where the assessment form requires submission. | Acts within own eligible opportunity. |
| Policy-configured D07 assessor | Records submission and awarded internal marks. | RecordAssessmentSubmission authority. |
| Policy-configured D07 mark editor | Applies permitted pre-finalization updates. | UpdateAssessmentMarks authority. |

# 7. Preconditions and Entry Criteria

- Assessment Window is Open.
- Student and registration/eligibility references are valid.
- Blueprint component is Published and applicable.

# 8. Trigger

An eligible student participates in the open internal-assessment opportunity.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate window, blueprint component, student eligibility and duplicate identity. | D07 using Section C facts | Submission may proceed or is rejected. |
| 2 | Capture submission reference/evidence and assessment context. | D07 assessor | Submission identity established. |
| 3 | Record awarded internal marks under component policy. | D07 assessor | AssessmentSubmissionRecorded. |
| 4 | Apply any permitted update before moderation/finalization with reason and audit. | Authorized mark editor | AssessmentSubmissionUpdated. |
| 5 | Route completed submission for moderation where required. | D07 | Submission ready for governed downstream processing. |

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
| Assessment Submission | No submission / Open opportunity | Capture unique student submission and assessor marks. | Recorded; optional Updated before moderation | D07 | L2-D07 §§5–8 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| AssessmentSubmissionRecorded / AssessmentSubmissionUpdated | D07 | Moderation, Academic Monitoring, Notification and authorized consumers | Records submission and governed pre-finalization update. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C | Internal Assessment | Owns canonical student and registration/eligibility facts consumed by D07. | Preserve ownership while enabling the scenario. | `L2-D07_Internal_Assessment_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Provides the assessment response/evidence where the assessment form requires submission. | Eligible student | Acts within own eligible opportunity. | Internal Assessment |
| Records submission and awarded internal marks. | Policy-configured D07 assessor | RecordAssessmentSubmission authority. | Internal Assessment |
| Applies permitted pre-finalization updates. | Policy-configured D07 mark editor | UpdateAssessmentMarks authority. | Internal Assessment |

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

- A unique Assessment Submission is recorded against the correct student, blueprint component and window, with immutable submission identity and auditable mark updates before moderation/finalization.
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
