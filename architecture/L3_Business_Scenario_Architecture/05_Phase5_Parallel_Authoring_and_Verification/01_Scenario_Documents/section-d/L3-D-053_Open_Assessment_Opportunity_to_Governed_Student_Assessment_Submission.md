# Open Assessment Opportunity to Governed Student Assessment Submission

# 1. Scenario Metadata

**Scenario ID:** L3-D-053  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0773 / CAN-L3-D-053

---

# 2. Scenario Purpose

Capture an eligible student internal-assessment submission and awarded internal marks during a governed Open Assessment Window.

# 3. Business Outcome

A unique Assessment Submission is recorded against the correct student, blueprint component and window, with attributable submission evidence and auditable pre-moderation mark history.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An Assessment Window is Open and an authoritative eligible student participates in the assessment opportunity.

## 4.2 End Boundary

The submission and applicable awarded internal marks are recorded and `AssessmentSubmissionRecorded` is published by D07.

## 4.3 In Scope

- Validate Open window, student, offering and blueprint component.
- Capture submission evidence and awarded internal marks.
- Apply governed pre-finalization mark updates where permitted.
- Route completed submission to moderation where required.

## 4.4 Out of Scope

- Student master/registration ownership.
- Moderation or finalization.
- Official examination-grade computation.
- Command/API/permission design.

## 4.5 Related Scenarios — Optional

L3-D-052 provides the governed Assessment Window; L3-D-054 governs moderation where required.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns AssessmentSubmission, submission evidence and awarded internal marks. |
| Section C | Owns canonical student and registration/eligibility facts consumed by D07. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Eligible student | Provides the assessment response/evidence where the assessment form requires student submission. | Acts only within the student's eligible assessment opportunity. |

The locked L2-D07 source does not define the canonical institutional assessor or pre-finalization mark-editor roles. D07 business responsibility is therefore used in the flow and the missing role mapping is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- Assessment Window is Open.
- Student and registration/eligibility references are valid.
- Blueprint component is Published and applicable.

# 8. Trigger

An eligible student participates in the Open internal-assessment opportunity.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate window, blueprint component, student eligibility and duplicate-submission constraints. | D07 using Section C facts | Submission may proceed or remains blocked. |
| 2 | Capture the student response/evidence and assessment context. | Eligible student / D07 | Submission identity and evidence are established. |
| 3 | Record awarded internal marks under the applicable D07 assessment policy. | D07 under applicable institutional assessment authority | `AssessmentSubmissionRecorded`. |
| 4 | Where policy permits a pre-moderation update, record the revised marks as governed history with reason. | D07 under applicable authority | `AssessmentSubmissionUpdated`. |
| 5 | Make the completed submission available to moderation where required. | D07 | Submission is ready for downstream D07 processing. |

# 10. Alternate Business Flows

## 10.1 Permitted Pre-Moderation Mark Update

Where institutional policy permits an awarded-mark update before moderation/finalization, D07 records the new governed mark history with reason and audit evidence; the submission remains within the same scenario outcome.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Window, student eligibility or blueprint reference is invalid | Submission cannot validly proceed. | Resolve the authoritative business condition before a valid submission is recorded. | D07 with Section C/source owner as applicable |
| Required assessment authority cannot be validated | Marks cannot be accepted into the governed submission. | Obtain valid institutional authority assignment before continuation. | D07 |
| Submission activity is interrupted before recording | No authoritative submission outcome exists. | Preserve the last valid business position and continue when the blocking condition is resolved. | D07 |

# 12. Business Rules and Decision Constraints

- A submission must be recorded before moderation. Source: L2-D07 §5.
- D07 owns AssessmentSubmission and awarded internal marks. Source: L2-D07 §6.3.
- Section C retains student/registration ownership. Source: L2-D07 §9 and dependencies.
- No command name is treated as an L3 permission or RBAC rule.

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Assessment Submission | No submission / Open opportunity | Record unique submission and governed marks history. | Recorded; optional Updated before moderation | D07 | L2-D07 §§5–8 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `AssessmentSubmissionRecorded` / `AssessmentSubmissionUpdated` | D07 | Moderation, Academic Monitoring, Notification and authorized consumers | Records submission and governed pre-finalization update. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section C provides authoritative student/registration eligibility facts to D07 without transferring ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

- The student may act only within an eligible assessment opportunity.
- Exact institutional roles for assessing and changing marks before moderation are not established in locked L2-D07 and therefore are not invented here.

## 15.2 Separation of Duties, Overrides and Exceptions

Any separation between assessor, mark editor, moderator and finalization authority must be established by authoritative institutional policy/upstream architecture.

## 15.3 Business Audit and Evidence Obligations

Retain student/assessment identity, source eligibility references, submission evidence, awarded marks, any permitted update and reason, responsible authority and published submission events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A governed Assessment Submission exists for the eligible assessment opportunity.
- Awarded marks and any permitted pre-moderation update are attributable and auditable.
- `AssessmentSubmissionRecorded` is published by D07.

## 16.2 Postconditions

- D07 retains AssessmentSubmission ownership.
- Section C retains student/registration ownership.
- The submission may enter moderation where required.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L2-D07 has no Actors section. | Canonical assessor and pre-finalization mark-editor role names/authority are not established upstream. | Define/approve D07 actor-authority mapping before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Commands, APIs, technical RBAC, UI capture and persistence are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** `L2-D07_Internal_Assessment_Workflows.md`; Template v2.0; Architecture Level Validation Standards.
