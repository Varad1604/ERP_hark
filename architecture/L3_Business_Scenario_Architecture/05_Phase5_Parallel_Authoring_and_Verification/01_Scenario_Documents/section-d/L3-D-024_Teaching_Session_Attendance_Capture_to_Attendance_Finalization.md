# Teaching Session Attendance Capture to Attendance Finalization

**Scenario ID:** L3-D-024  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Teaching Session & Attendance  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0744 / CAN-L3-D-024

---

# 2. Scenario Purpose

Capture one canonical attendance record per eligible student in a Teaching Session and govern it through submission, optional approval and finalization.

# 3. Business Outcome

The Attendance Register is Finalized as the official attendance truth for the Teaching Session and cannot be changed through normal operations.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An active Teaching Session has an open Attendance Register and authoritative student eligibility facts are available.

## 4.2 End Boundary

Attendance reaches Finalized and `AttendanceFinalized` is published.

## 4.3 In Scope

- Open register and record canonical status/segments.
- Validate uniqueness, eligibility, status, segment and workflow state.
- Submit, optionally approve and finalize attendance.

## 4.4 Out of Scope

- Student registration or eligibility ownership.
- Post-finalization correction.
- Deriving attendance from participation.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D04 | Owns Attendance Register, Student Attendance records, lifecycle and finalization. |
| Section C | Provides authoritative student registration and eligibility facts; retains registration ownership. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized faculty | Records and submits attendance. | Record Attendance and Submit Attendance permissions. |
| Configured attendance approver | Approves attendance only when institutional policy requires. | Department Head, Academic Coordinator or Academic Office as configured. |
| Authorized finalizer | Finalizes validated attendance. | Finalize Attendance permission. |

# 7. Preconditions and Entry Criteria

- Teaching Session exists in a state that permits attendance.
- Attendance Register belongs to exactly one Teaching Session.
- Student registration/eligibility references are authoritative.

# 8. Trigger

Attendance capture opens during or after the governed Teaching Session execution.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Open the Attendance Register for the Teaching Session. | Authorized faculty / D04 | AttendanceRegisterOpened. |
| 2 | Record one canonical attendance status per student and applicable segments. | Authorized faculty | Recorded attendance exists without duplication. |
| 3 | Validate eligibility, status, segments, duplicates and policy. | D04 using Section C facts | Valid records may proceed. |
| 4 | Submit attendance for institutional processing. | Authorized faculty | Recorded → Submitted. |
| 5 | Approve when configured; otherwise follow the permitted direct path. | Configured approver | Submitted → Approval where required. |
| 6 | Finalize attendance. | Authorized finalizer | Finalized and `AttendanceFinalized`; normal modification prohibited. |

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
| Attendance Register | Recorded | Submit, optionally approve and finalize. | Recorded → Submitted → optional Approval → Finalized | D04 | L2-D04 §§23–24 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| AttendanceRegisterOpened / AttendanceRecorded / AttendanceSubmitted / AttendanceApproved / AttendanceFinalized | D04 | Section C, D05 and authorized consumers | Records governed attendance lifecycle. | L2-D04 §46.2 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C | Teaching Session & Attendance | Provides authoritative student registration and eligibility facts; retains registration ownership. | Preserve ownership while enabling the scenario. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Records and submits attendance. | Authorized faculty | Record Attendance and Submit Attendance permissions. | Teaching Session & Attendance |
| Approves attendance only when institutional policy requires. | Configured attendance approver | Department Head, Academic Coordinator or Academic Office as configured. | Teaching Session & Attendance |
| Finalizes validated attendance. | Authorized finalizer | Finalize Attendance permission. | Teaching Session & Attendance |

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

- The Attendance Register is Finalized as the official attendance truth for the Teaching Session and cannot be changed through normal operations.
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
