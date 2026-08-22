# Recorded Attendance to Governed Corrected Attendance

**Scenario ID:** L3-D-025  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Teaching Session & Attendance  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0745 / CAN-L3-D-025

---

# 2. Scenario Purpose

Correct finalized attendance through a governed, justified and auditable workflow without deleting or rewriting the original attendance fact.

# 3. Business Outcome

An approved correction is applied and re-finalized as the current official attendance truth, while original and correction history remain immutable.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A justified correction request identifies finalized attendance requiring change.

## 4.2 End Boundary

The correction is approved, applied and re-finalized, and `AttendanceCorrected` records the completed correction; a rejection retains the existing final truth.

## 4.3 In Scope

- Request correction with reason and evidence.
- Validate correction window, eligibility, workflow state and authority.
- Review, approve/reject, apply and re-finalize.

## 4.4 Out of Scope

- Direct edit of finalized attendance.
- Deletion of original history.
- Modification of Section C registration facts.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D04 | Owns finalized attendance, correction request, decision, corrected fact and history. |
| Section C | Provides authoritative registration/eligibility references only. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized correction requester | Submits correction reason and evidence. | Correct Attendance permission. |
| Authorized approver | Reviews and approves or rejects the correction under configured policy. | Governed correction authority. |
| Authorized finalizer | Applies an approved correction and re-finalizes attendance. | Finalize Attendance permission. |

# 7. Preconditions and Entry Criteria

- Attendance is Finalized.
- Correction window and policy permit a request or governed override is approved.
- Original attendance and supporting evidence are available.

# 8. Trigger

An error, authorized exception or evidence-backed discrepancy is identified in finalized attendance.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create a correction request linked to finalized attendance. | Authorized requester | AttendanceCorrectionRequested. |
| 2 | Validate window, state, authority, evidence and student reference. | D04 | Request is reviewable or rejected. |
| 3 | Approve or reject the proposed correction with reason. | Authorized approver | Decision and corresponding event recorded. |
| 4 | For approval, add the corrected business history without altering the original fact. | D04 | Attendance updated through governed correction. |
| 5 | Re-finalize the corrected attendance. | Authorized finalizer | Current official truth established and `AttendanceCorrected` published. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Teaching Session & Attendance |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Teaching Session & Attendance |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Teaching Session & Attendance |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Teaching Session & Attendance | Prevents cross-domain ownership leakage. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Teaching Session & Attendance | Prevents governance bypass. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |
| Published or historical business facts remain immutable. | Teaching Session & Attendance | Preserves auditability and lineage. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Attendance | Finalized | Govern request, review, decision, correction and re-finalization. | Finalized → Correction Requested → Review → Approved/Rejected → Attendance Updated → Re-Finalized | D04 | L2-D04 §26 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| AttendanceCorrectionRequested / AttendanceCorrectionApproved / AttendanceCorrectionRejected / AttendanceCorrected | D04 | Section C, D05 and authorized consumers | Records correction lifecycle and completed corrected fact. | L2-D04 §46.2 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C | Teaching Session & Attendance | Provides authoritative registration/eligibility references only. | Preserve ownership while enabling the scenario. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Submits correction reason and evidence. | Authorized correction requester | Correct Attendance permission. | Teaching Session & Attendance |
| Reviews and approves or rejects the correction under configured policy. | Authorized approver | Governed correction authority. | Teaching Session & Attendance |
| Applies an approved correction and re-finalizes attendance. | Authorized finalizer | Finalize Attendance permission. | Teaching Session & Attendance |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Teaching Session & Attendance |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Teaching Session & Attendance |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Teaching Session & Attendance |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An approved correction is applied and re-finalized as the current official attendance truth, while original and correction history remain immutable.
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
**Validation Basis:** Section D canonical register; `L2-D04_Teaching_Session_and_Attendance_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
