# Completed Internal Assessment Outcomes to Governed Finalized Internal Assessment

# 1. Scenario Metadata

**Scenario ID:** L3-D-055  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0775 / CAN-L3-D-055

---

# 2. Scenario Purpose

Finalize completed and, where required, moderated internal-assessment outcomes as immutable D07 business facts for downstream Examination consumption.

# 3. Business Outcome

The Internal Assessment reaches Finalized, its marks and evidence become immutable except through the governed correction workflow, and Section I receives only the finalized D07 fact.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

All required assessment components are completed and applicable moderation decisions are resolved.

## 4.2 End Boundary

The internal assessment reaches Finalized and `InternalAssessmentFinalized` is published by D07 for downstream consumption.

## 4.3 In Scope

- Validate component completeness, marks, moderation and window closure.
- Obtain the required institutional finalization decision.
- Apply the D07 Finalized lifecycle state.
- Publish the finalized internal-assessment fact.

## 4.4 Out of Scope

- Official examination-grade/result computation.
- Direct mutation of Section I records.
- Normal edits after finalization.
- Command/API/permission design.

## 4.5 Related Scenarios — Optional

L3-D-054 provides moderated outcomes where required; Section I scenarios consume the finalized D07 business fact.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns internal-assessment lifecycle, marks, evidence and finalization. |
| Section I Examination | Consumes finalized D07 facts and owns examination/result computation. |

# 6. Business Actors, Roles and Responsibilities

Locked L2-D07 does not define canonical actors. This scenario therefore does not invent finalization-administrator or finalization-authority roles. The required institutional actor mapping is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- All mandatory component outcomes exist.
- Required moderation is completed.
- Assessment Window is Closed.
- No blocking correction/review remains.

# 8. Trigger

The assessment cycle satisfies all D07 completion and governance requirements.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Assemble completed component and moderated outcomes. | D07 | Finalization set is established. |
| 2 | Validate blueprint completeness, marks, moderation, window and policy. | D07 | Assessment is ready for finalization or remains blocked. |
| 3 | Obtain the required institutional finalization decision. | Applicable business authority — role unresolved upstream | Finalization is approved or rejected. |
| 4 | For approval, transition Internal Assessment to Finalized and make marks/evidence immutable except through controlled correction. | D07 | Internal Assessment is Finalized. |
| 5 | Publish the finalized business fact. | D07 | `InternalAssessmentFinalized`. |
| 6 | Section I consumes the finalized fact for its own examination processing. | Section I | Examination ownership remains unchanged. |

# 10. Alternate Business Flows

No distinct alternate path reaches the Finalized outcome. Failed completeness/moderation checks or rejected finalization decisions block completion and are handled under Section 11.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Required component/moderation/window evidence is incomplete | Finalization cannot proceed. | Resolve the missing D07 business condition before finalization is reconsidered. | D07 |
| Required finalization authority cannot be validated or rejects finalization | Internal Assessment remains non-Finalized. | Retain the current authoritative state and follow the governed review/resubmission path. | D07 |
| Finalization activity is interrupted | `InternalAssessmentFinalized` has not occurred. | Preserve the last valid business position and continue when the blocking condition is resolved. | D07 |

# 12. Business Rules and Decision Constraints

- Finalized records are immutable except through controlled correction. Source: L2-D07 §5.
- D07 owns Internal Assessment marks/evidence/finalization. Source: L2-D07 §§1,3,6.
- Section I consumes only finalized internal-assessment facts and does not own D07 lifecycle. Source: L2-D07 §§9,12.
- No L2 command name is treated as an L3 permission or actor authority.

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Internal Assessment | Moderated / completed outcomes | Validate and finalize complete assessment cycle. | Moderated → Finalized | D07 | L2-D07 §§5,7–9 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `InternalAssessmentFinalized` | D07 | Section I Examination, Academic Monitoring and Notification | Records finalized internal-assessment outcome. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

D07 publishes the finalized business fact to Section I; Section I consumes it without modifying D07-owned assessment state.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The institutional role that validates/prepares finalization and the role that authorizes finalization are not named in locked L2-D07. This scenario therefore records the required business decision without inventing role names or technical permissions.

## 15.2 Separation of Duties, Overrides and Exceptions

Where institutional policy requires separation between preparation, moderation and finalization approval, those authorities remain distinct and must be established upstream.

## 15.3 Business Audit and Evidence Obligations

Retain component completeness evidence, moderation decisions, window status, finalization authority/decision once established, resulting Finalized state and `InternalAssessmentFinalized` publication evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- All D07 completion prerequisites are satisfied.
- Required institutional finalization decision is valid.
- Internal Assessment reaches Finalized and `InternalAssessmentFinalized` is published by D07.

## 16.2 Postconditions

- D07 marks/evidence are immutable except through controlled correction.
- Section I may consume the finalized fact but retains no D07 mutation authority.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L2-D07 has no Actors section. | Canonical institutional finalization roles are not established upstream. | Define/approve D07 actor-authority mapping before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Commands, APIs, technical RBAC, persistence and event-delivery mechanisms are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** `L2-D07_Internal_Assessment_Workflows.md`; Template v2.0; Architecture Level Validation Standards.
