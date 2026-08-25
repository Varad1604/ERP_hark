# Teaching Session Attendance Capture to Attendance Finalization

**Scenario ID:** L3-D-024  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Teaching Session & Attendance  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0744 / CAN-L3-D-024

---

# 2. Scenario Purpose

Capture one canonical attendance record per eligible student in a Teaching Session and govern the Attendance Register through submission, any institutionally required approval, and finalization.

# 3. Business Outcome

The Attendance Register is Finalized as the official attendance truth for the Teaching Session and cannot be changed through normal operations.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An active Teaching Session has an open Attendance Register and authoritative student eligibility facts are available.

## 4.2 End Boundary

Attendance reaches Finalized and `AttendanceFinalized` is published by Teaching Session & Attendance.

## 4.3 In Scope

- Record the canonical attendance status and applicable attendance segments for eligible students.
- Validate attendance uniqueness, eligibility, classification, segment and lifecycle requirements.
- Submit attendance for governed processing.
- Apply attendance approval when institutional policy requires it.
- Finalize the Attendance Register.

## 4.4 Out of Scope

- Student registration or eligibility ownership.
- Post-finalization attendance correction.
- Deriving attendance from participation.
- Technical access-control, transaction, retry, messaging or infrastructure behaviour.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D04 — Teaching Session & Attendance | Owns the Attendance Register, Student Attendance records, attendance lifecycle and finalization. |
| Section C | Provides authoritative student registration and eligibility facts and retains ownership of those facts. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Business Authority |
|---|---|---|
| Authorized faculty | Records and submits attendance for the applicable Teaching Session. | May perform attendance capture and submission only where authorized by the governing attendance policy. |
| Department Head / Academic Coordinator / Academic Office, where configured by institutional policy | Reviews and approves submitted attendance when the institution requires an approval stage. | Approval authority exists only where the governing attendance policy assigns it. |

L2-D04 establishes that finalization is an authorized attendance action, but the reviewed source does not establish one canonical institutional business-role name that universally owns the finalization decision. This scenario therefore does not invent a placeholder finalizer role; the authority-name gap is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- The Teaching Session exists in a state that permits attendance processing.
- The Attendance Register belongs to exactly one Teaching Session.
- Student registration and eligibility references are authoritative.
- Attendance classification and applicable institutional attendance policy are available.

# 8. Trigger

Attendance capture opens during or after the governed Teaching Session execution in accordance with institutional attendance policy.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Open the Attendance Register for the Teaching Session. | Authorized faculty / D04 | `AttendanceRegisterOpened` is established. |
| 2 | Record one canonical attendance status per eligible student and any applicable attendance segments. | Authorized faculty | Attendance records exist without duplicate student attendance for the same Teaching Session. |
| 3 | Validate student eligibility, canonical attendance classification, segment requirements, uniqueness and lifecycle position. | D04 using Section C-owned facts where required | Valid attendance records may proceed. |
| 4 | Submit attendance for governed institutional processing. | Authorized faculty / D04 | `AttendanceSubmitted` is established. |
| 5 | Where institutional policy requires approval, review and approve the submitted attendance. | Configured business approver / D04 | `AttendanceApproved` is established for the approval-required path. |
| 6 | Finalize the validated attendance under the governing D04 attendance-finalization authority. | D04 | Attendance becomes Finalized and `AttendanceFinalized` is published. |

# 10. Alternate Business Flows

## 10.1 Approval Not Required

Where institutional policy does not require an attendance-approval stage, validated Submitted attendance follows the permitted direct path to finalization. The scenario reaches the same Finalized outcome without publishing `AttendanceApproved` for that path.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Student eligibility or authoritative reference cannot be validated | Attendance for the affected student cannot validly progress. | Resolve the authoritative eligibility/reference issue with the owning domain before continuation. | D04 with Section C for Section C-owned facts |
| Duplicate or invalid attendance entry is detected | Submission or finalization cannot proceed for the affected attendance scope. | Correct the attendance entry in accordance with D04 business rules before resubmission. | D04 |
| Required attendance approval is rejected or withheld | Attendance cannot proceed to finalization through the approval-required path. | Retain the current authoritative attendance state and follow the governed correction/resubmission or terminal decision path supported by policy. | D04 |
| Attendance processing is interrupted before finalization | The Finalized business outcome has not occurred. | Preserve the last valid business state and continue only when the blocking business condition is resolved. | D04 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owner | Scenario Effect | Source |
|---|---|---|---|
| An Attendance Register belongs to exactly one Teaching Session. | D04 | Prevents attendance from being detached from its governing Teaching Session. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` BR-011 |
| Attendance does not exist independently from a Teaching Session. | D04 | Preserves the aggregate and lifecycle boundary. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` BR-012 |
| Each student has one canonical attendance record per Teaching Session. | D04 | Prevents duplicate canonical attendance for the same student/session combination. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` BR-013 |
| Attendance uses canonical attendance classifications. | D04 | Requires attendance values to use the governed classification model. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` BR-014 |
| Attendance may use one or more Attendance Segments where applicable. | D04 | Allows segment-based attendance without changing canonical ownership. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` BR-015 |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Attendance Register | Recorded | Submit, apply approval where required, and finalize through the governed D04 lifecycle. | Recorded → Submitted → Approved where required → Finalized | D04 | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` attendance lifecycle |

No additional lifecycle state is introduced by this scenario.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `AttendanceRegisterOpened` | D04 | Authorized consumers | Records that attendance capture is open for the Teaching Session. | L2-D04 §46.2 |
| `AttendanceRecorded` | D04 | Authorized consumers | Records governed attendance capture. | L2-D04 §46.2 |
| `AttendanceSubmitted` | D04 | Attendance approval/finalization processes and authorized consumers | Records submission for governed processing. | L2-D04 §46.2 |
| `AttendanceApproved` | D04 | Attendance finalization and authorized consumers | Records approval where institutional policy requires it. | L2-D04 §46.2 |
| `AttendanceFinalized` | D04 | D05 and other authorized downstream consumers | Records the finalized attendance fact. | L2-D04 §46.2 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C | D04 — Teaching Session & Attendance | Authoritative student registration and eligibility facts | Enable attendance validation without transferring Section C ownership. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Record and submit attendance | Authorized faculty | May act only for the applicable Teaching Session and under the governing attendance policy. | D04 |
| Approve attendance when approval is required | Department Head / Academic Coordinator / Academic Office, where configured by policy | Approval applies only when the institutional attendance model requires it. | D04 |
| Finalize attendance | Exact institutional business-role name unresolved in the reviewed authoritative source | Finalization may occur only under the governing D04 attendance-finalization authority; no technical permission or placeholder role is defined here. | D04 |

## 15.2 Separation of Duties, Overrides and Exceptions

Where institutional policy separates attendance capture, approval and finalization responsibilities, those business authorities remain distinct. Any governed override or exception must retain the required business justification and audit evidence defined by D04 policy.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Attendance lifecycle and decisions | Actor, business authority, prior business state, validation evidence, decision and resulting state. | D04 |
| Source and cross-domain traceability | Authoritative source references for student eligibility and other consumed business facts. | D04 with source ownership retained by the providing domain |
| Failure and recovery | Rejected validations, interrupted progression, correction/resubmission decisions and final business outcome. | D04 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Attendance Register has reached Finalized under the governed D04 attendance lifecycle.
- Required approval has occurred where institutional policy requires it.
- `AttendanceFinalized` has been published by D04.
- Required business authority, validation and audit evidence is retained.

## 16.2 Postconditions

- The finalized Attendance Register is the authoritative attendance truth for the Teaching Session.
- Normal attendance modification is no longer permitted; any post-finalization correction belongs to the governed correction scenario/workflow.
- Section C retains ownership of student registration and eligibility facts.
- Downstream domains may consume finalized attendance facts without modifying the D04-owned attendance aggregate.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The reviewed L2-D04 establishes authorized attendance finalization but does not provide one canonical institutional business-role name that universally performs finalization. | L3 must not invent a placeholder finalizer role or convert the requirement into a technical permission. | Confirm the canonical business role/authority in authoritative upstream architecture or governance before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Technical RBAC/access-control design, transaction processing, retries, messaging, persistence and infrastructure recovery are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** Section D canonical register; `L2-D04_Teaching_Session_and_Attendance_Workflows.md`; `L3_Business_Scenario_Template_v2.0.md`; `Architecture_Level_Validation_Standards.md`.
