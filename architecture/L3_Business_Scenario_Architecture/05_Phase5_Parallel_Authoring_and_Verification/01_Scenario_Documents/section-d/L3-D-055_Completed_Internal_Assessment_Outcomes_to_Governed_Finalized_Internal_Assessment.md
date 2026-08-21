# Completed Internal Assessment Outcomes to Governed Finalized Internal Assessment

**Scenario ID:** L3-D-055  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0775 / CAN-L3-D-055

---

# 2. Scenario Purpose

Finalize completed and, where required, moderated internal-assessment outcomes as immutable D07 facts for downstream Examination consumption.

# 3. Business Outcome

The Internal Assessment reaches Finalized, its marks and evidence become immutable except through controlled correction, and Section I receives only the finalized business fact.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

All required assessment components are completed and applicable moderation decisions are resolved.

## 4.2 End Boundary

The internal assessment reaches Finalized and `InternalAssessmentFinalized` is published to Section I and authorized consumers.

## 4.3 In Scope

- Validate component completeness, marks, moderation and window closure.
- Authorize and apply finalization.
- Publish finalized internal-assessment fact to Section I.

## 4.4 Out of Scope

- Official examination grade/result computation.
- Direct mutation of Section I records.
- Normal edits after finalization.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns internal-assessment lifecycle, marks, evidence and finalization. |
| Section I Examination | Consumes finalized D07 facts and owns examination/result computation. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Policy-configured D07 finalization administrator | Prepares and validates finalization set. | FinalizeInternalAssessment execution. |
| Policy-configured D07 finalization authority | Approves finalization after completeness and moderation checks. | Final decision authority. |
| Section I Examination | Consumes completed finalization event. | No D07 mutation authority. |

# 7. Preconditions and Entry Criteria

- All mandatory component outcomes exist.
- Required moderation is completed.
- Assessment Window is Closed and no blocking correction/review remains.

# 8. Trigger

The assessment cycle satisfies all D07 completion and governance requirements.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Collect completed component and moderated outcomes. | D07 | Finalization set assembled. |
| 2 | Validate blueprint completeness, marks, moderation, window and policy. | D07 finalization administrator | Ready or blocked with explicit deficiencies. |
| 3 | Approve finalization. | D07 finalization authority | Authority decision recorded. |
| 4 | Transition internal assessment to Finalized and lock marks/evidence. | D07 | Finalized; normal modification prohibited. |
| 5 | Publish finalized fact for downstream consumption. | D07 | InternalAssessmentFinalized. |
| 6 | Consume finalized facts for examination processing. | Section I | Examination ownership remains unchanged. |

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
| Internal Assessment | Moderated / completed outcomes | Validate and finalize complete assessment cycle. | Moderated → Finalized | D07 | L2-D07 §§5,7–9 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| InternalAssessmentFinalized | D07 | Section I Examination, Academic Monitoring and Notification | Records immutable finalized internal-assessment outcome. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section I Examination | Internal Assessment | Consumes finalized D07 facts and owns examination/result computation. | Preserve ownership while enabling the scenario. | `L2-D07_Internal_Assessment_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Prepares and validates finalization set. | Policy-configured D07 finalization administrator | FinalizeInternalAssessment execution. | Internal Assessment |
| Approves finalization after completeness and moderation checks. | Policy-configured D07 finalization authority | Final decision authority. | Internal Assessment |
| Consumes completed finalization event. | Section I Examination | No D07 mutation authority. | Internal Assessment |

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

- The Internal Assessment reaches Finalized, its marks and evidence become immutable except through controlled correction, and Section I receives only the finalized business fact.
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
