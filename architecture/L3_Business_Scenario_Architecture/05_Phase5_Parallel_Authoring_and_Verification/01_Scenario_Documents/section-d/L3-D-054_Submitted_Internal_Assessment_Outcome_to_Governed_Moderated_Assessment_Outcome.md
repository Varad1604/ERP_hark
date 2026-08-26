# Submitted Internal Assessment Outcome to Governed Moderated Assessment Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-D-054  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0774 / CAN-L3-D-054

---

# 2. Scenario Purpose

Govern moderation of a submitted internal-assessment outcome while preserving the original submission and mark history.

# 3. Business Outcome

The Assessment Moderation Case reaches an approved or rejected governed decision; an approved moderated outcome becomes the D07 input to finalization without erasing the original submission.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A recorded Assessment Submission requires moderation under assessment policy or authorized review.

## 4.2 End Boundary

Moderation reaches a governed terminal decision and `AssessmentModerationCompleted` is published where the moderated outcome is completed.

## 4.3 In Scope

- Establish a moderation case linked to the original submission.
- Review submission evidence, awarded marks, blueprint and policy.
- Decide the moderated outcome under applicable institutional authority.
- Preserve the original submission and governed moderation history.

## 4.4 Out of Scope

- Changing original submission history in place.
- Finalization of the full internal assessment.
- Examination-grade computation.
- Command/API/permission design.

## 4.5 Related Scenarios — Optional

L3-D-053 supplies recorded submissions; L3-D-055 governs finalization of completed/moderated outcomes.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns AssessmentModerationCase, moderation evidence, decision and approved moderated outcome. |

# 6. Business Actors, Roles and Responsibilities

Locked L2-D07 does not define canonical actors. This scenario therefore does not invent a moderation requester, moderator or approval authority. Exact institutional role mapping is an open issue in Section 17.

# 7. Preconditions and Entry Criteria

- Assessment Submission is recorded.
- Original evidence, marks, blueprint and policy binding are available.

# 8. Trigger

Assessment policy, quality control or authorized review requires moderation.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish a moderation case linked to the immutable original submission. | D07 | `AssessmentModerationRequested`. |
| 2 | Validate the case, evidence, blueprint component and policy. | D07 | Review basis is complete or blocked. |
| 3 | Conduct the required independent academic review under applicable institutional authority. | D07 under applicable business authority | Moderated recommendation/outcome is recorded. |
| 4 | Approve or reject the moderation decision with reason. | Applicable business authority — role unresolved upstream | Governed moderation decision is retained. |
| 5 | For an approved outcome, preserve the original and record the moderated outcome as a new governed fact. | D07 | `AssessmentModerationCompleted`. |
| 6 | Make the approved moderated outcome available to finalization; rejection retains the prior valid outcome. | D07 | Historical evidence is not overwritten. |

# 10. Alternate Business Flows

## 10.1 Moderation Rejected

Where a valid moderation review results in rejection, the proposed moderated outcome is not adopted and the prior valid submission outcome remains authoritative. The rejection reason and decision evidence remain traceable.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Moderation evidence or policy context is incomplete | Moderation cannot validly reach decision. | Complete or correct the required business evidence before review continues. | D07 |
| Required moderation authority cannot be validated | A valid approval/rejection decision cannot be made. | Retain the current business position until an authoritative role assignment exists. | D07 |
| Moderation activity is interrupted | No new moderated outcome has been completed. | Preserve the last valid business position and continue once the blocking condition is resolved. | D07 |

# 12. Business Rules and Decision Constraints

- A submission must exist before moderation. Source: L2-D07 §5.
- D07 owns AssessmentModerationCase and moderation/correction workflow. Source: L2-D07 §6.4.
- Moderation preserves the original submission audit trail. Source: L2-D07 §§5,11.
- No L2 command name is treated as an L3 permission or RBAC rule.

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Assessment Moderation Case | Requested / submitted outcome | Review and decide moderation. | Requested → Under Review → Approved or Rejected → Completed where applicable | D07 | L2-D07 §§5–8,11 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `AssessmentModerationRequested` / `AssessmentModerationCompleted` | D07 | Finalization, Academic Monitoring and authorized consumers | Records governed moderation lifecycle. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No cross-domain ownership transfer occurs; moderation remains within D07.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Exact institutional moderation requester/reviewer/decision roles are not defined in locked L2-D07 and therefore are not invented at L3.

## 15.2 Separation of Duties, Overrides and Exceptions

Any required independence between submission assessment, moderation review and moderation approval must be established by authoritative institutional policy/upstream architecture.

## 15.3 Business Audit and Evidence Obligations

Retain original submission, moderation request, evidence, reviewer/decision authority once established, moderated recommendation, decision reason and completed moderation fact.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Moderation has reached a governed approved or rejected decision.
- Original submission history remains unchanged.
- Approved moderated outcome is available to D07 finalization where applicable.

## 16.2 Postconditions

- D07 retains moderation ownership.
- No examination result or final internal-assessment outcome is created by this scenario.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L2-D07 has no Actors section. | Canonical requester, independent moderator and moderation-decision role names are not established upstream. | Define/approve D07 actor-authority mapping before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Commands, APIs, technical RBAC and implementation recovery are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** `L2-D07_Internal_Assessment_Workflows.md`; Template v2.0; Architecture Level Validation Standards.
