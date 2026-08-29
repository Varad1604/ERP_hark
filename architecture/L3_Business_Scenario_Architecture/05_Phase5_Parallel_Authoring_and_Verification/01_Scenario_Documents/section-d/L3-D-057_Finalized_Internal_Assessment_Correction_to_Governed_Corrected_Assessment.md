# Finalized Internal Assessment Correction to Governed Corrected Assessment

# 1. Scenario Metadata

**Scenario ID:** L3-D-057  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0777 / CAN-L3-D-057

---

# 2. Scenario Purpose

Correct a Finalized Internal Assessment through the controlled D07 moderation/correction workflow while preserving the original finalized audit trail and informing Section I only through completed D07 business facts.

# 3. Business Outcome

An authorized correction is recorded as new history, governed through moderation/correction and re-finalized as the current D07 outcome; the original finalized facts remain immutable and traceable.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A justified discrepancy or approved correction requirement identifies a Finalized Internal Assessment needing correction.

## 4.2 End Boundary

The corrected outcome is approved, recorded and re-finalized; existing canonical events `AssessmentSubmissionUpdated`, `AssessmentModerationCompleted` and `InternalAssessmentFinalized` communicate the completed D07 facts as applicable.

## 4.3 In Scope

- Establish a correction/moderation case with reason and evidence.
- Review the original outcome and proposed correction.
- Decide the correction under applicable institutional authority.
- Record corrected marks/outcome as new history and re-finalize where approved.
- Publish existing canonical D07 business facts to Section I.

## 4.4 Out of Scope

- Direct editing/deletion of finalized history.
- Direct correction of Section I examination/result records.
- Command/API/permission design.

## 4.5 Related Scenarios — Optional

L3-D-054 supplies the moderation model; L3-D-055 supplies finalization behaviour; Section I consumes corrected finalized facts without ownership transfer.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns finalized internal-assessment correction, moderation, updated marks and re-finalization. |
| Section I Examination | Consumes corrected finalized facts and governs any examination-side consequence. |

# 6. Business Actors, Roles and Responsibilities

Locked L2-D07 has no Actors section. Canonical correction requester, independent reviewer and correction/finalization authority roles are therefore not invented in L3.

# 7. Preconditions and Entry Criteria

- Internal Assessment is Finalized and not irreversibly Archived under policy.
- Original marks/evidence and correction justification exist.
- Applicable correction policy is available.

# 8. Trigger

A validated error, appeal, audit finding or authorized academic decision requires correction of finalized internal assessment.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish a correction/moderation case linked to the original Finalized record. | D07 | Correction request/evidence is traceable. |
| 2 | Validate reason, evidence, affected submission/component and applicable authority requirements. | D07 | Correction may proceed or remains blocked. |
| 3 | Conduct the required independent academic review. | D07 under applicable institutional authority | Corrected recommendation is recorded. |
| 4 | Approve or reject correction with explicit reason. | Applicable business authority — role unresolved upstream | Governed decision is retained. |
| 5 | For approval, record updated marks/outcome as new history without rewriting the original fact. | D07 | `AssessmentSubmissionUpdated`. |
| 6 | Complete required moderation and re-finalize the corrected assessment. | D07 under applicable business authority | `AssessmentModerationCompleted` and `InternalAssessmentFinalized` as applicable. |
| 7 | Publish corrected finalized facts for Section I consumption. | D07 | Section I consumes without ownership transfer. |

# 10. Alternate Business Flows

## 10.1 Correction Rejected

A valid correction review may end in rejection. The original Finalized assessment remains authoritative and the rejection reason/evidence remain traceable.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Correction evidence or policy prerequisites are incomplete | Correction cannot validly proceed. | Resolve the missing business evidence/policy condition before review continues. | D07 |
| Required correction/finalization authority cannot be validated | Correction cannot be approved/re-finalized. | Preserve the original Finalized truth until authoritative role assignment exists. | D07 |
| Correction activity is interrupted | No corrected Finalized outcome has occurred. | Preserve the last valid business position and continue when the blocking condition is resolved. | D07 |

# 12. Business Rules and Decision Constraints

- Finalized records are immutable except through controlled correction workflow. Source: L2-D07 §5.
- Moderation/correction preserves the original submission history. Source: L2-D07 §§5,6.4,11.
- D07 owns corrected internal-assessment facts; Section I only consumes finalized facts. Source: L2-D07 §§9,12.
- No new event, state, rule or actor is introduced.

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Assessment Moderation/Correction Case | Finalized assessment / correction requested | Review, decide, record correction through new history and re-finalize. | Requested → Under Review → Approved/Rejected → Corrected → Finalized | D07 | L2-D07 §§2,5–8,11 |
| Original Finalized Assessment | Finalized | Reference and preserve; never edit in place. | Remains immutable historical fact | D07 | L2-D07 §§5,11 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `AssessmentModerationRequested` / `AssessmentSubmissionUpdated` / `AssessmentModerationCompleted` / `InternalAssessmentFinalized` | D07 | Section I Examination, Academic Monitoring and authorized consumers | Records controlled correction, moderation and corrected re-finalization using existing canonical events. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

D07 publishes corrected finalized facts; Section I consumes them and governs any examination-side consequence without modifying D07 history.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Exact institutional correction requester, reviewer and correction/finalization authority roles are not defined in locked L2-D07 and therefore are not invented here.

## 15.2 Separation of Duties, Overrides and Exceptions

Any required independence between correction initiation, review, correction decision and re-finalization must be established by authoritative institutional policy/upstream architecture.

## 15.3 Business Audit and Evidence Obligations

Retain original Finalized facts, correction reason/evidence, review history, authority/decision once established, corrected history, re-finalized outcome and published D07 facts.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Approved correction is recorded as new D07 history and re-finalized, or a governed rejection retains the original Finalized truth.
- Original facts remain immutable and traceable.
- Section I receives only D07-published business facts.

## 16.2 Postconditions

- D07 retains correction/finalization ownership.
- Section I retains ownership only of its own downstream examination/result consequences.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L2-D07 has no Actors section. | Canonical institutional correction requester, independent reviewer and correction/finalization roles are not established upstream. | Define/approve D07 actor-authority mapping before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Commands, APIs, technical RBAC, persistence and implementation recovery are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** `L2-D07_Internal_Assessment_Workflows.md`; Template v2.0; Architecture Level Validation Standards.
