# Scheduled Academic Session to Governed Teaching Session Outcome

**Scenario ID:** L3-D-023  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Teaching Session & Attendance  
**Participating Domains:** D, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0743 / CAN-L3-D-023

---

# 2. Scenario Purpose

Convert an approved Schedule Slot into a governed Teaching Session execution and record exactly one canonical instructional outcome.

# 3. Business Outcome

The session records one valid outcome—Completed, Partially Completed, Cancelled, Abandoned or Not Conducted—with assigned and conducting faculty evidence preserved.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An approved D03 Schedule Slot and an assigned or authorized conducting faculty member are available.

## 4.2 End Boundary

A canonical session outcome is recorded and its completed business fact is published; successful delivery may proceed to lock and archive under policy.

## 4.3 In Scope

- Validate schedule slot and faculty authorization.
- Start, execute, pause/resume where needed, and conclude the Teaching Session.
- Preserve planned versus actual delivery and faculty facts.

## 4.4 Out of Scope

- Timetable replanning for remaining work.
- Faculty employment or assignment ownership.
- Attendance correction after finalization.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D04 | Owns Teaching Session execution, outcome, attendance register and instructional evidence. |
| D03 | Owns the approved Schedule Slot consumed by D04. |
| D02 / Section E | Own Faculty Assignment and faculty identity facts consumed as references. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Assigned or authorized conducting faculty | Explicitly starts and conducts the Teaching Session and records its outcome. | Requires applicable business permission. |
| Authorized academic administrator | Applies governed cancellation, override or archival action when policy requires. | Policy-bound authority. |

# 7. Preconditions and Entry Criteria

- Schedule Slot is approved.
- Faculty authorization is valid, including substitution authorization where applicable.
- No other active Teaching Session exists for the Schedule Slot.

# 8. Trigger

Authorized conducting faculty explicitly starts instructional delivery or an authorized non-conducted outcome must be recorded.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate Schedule Slot, tenant, campus and faculty authorization. | D04 | Session may be instantiated or request is rejected. |
| 2 | Instantiate and start the Teaching Session. | Conducting faculty / D04 | TeachingSessionStarted. |
| 3 | Record instructional execution, progress, notes, evidence and attendance as applicable. | Conducting faculty | Session is In Progress with attributable evidence. |
| 4 | Pause and resume only when interruption requires it. | Conducting faculty | Pause history remains auditable. |
| 5 | Validate and record exactly one canonical outcome. | Authorized faculty or administrator | Completed, Partially Completed, Cancelled, Abandoned or Not Conducted. |
| 6 | Lock and later archive when policy conditions are met. | D04 authorized action | Immutable historical execution record. |

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
| Teaching Session | Approved Schedule Slot; no Teaching Session | Instantiate, execute and conclude with one outcome. | Started → In Progress → canonical terminal outcome; optional Locked → Archived | D04 | L2-D04 §§21–22 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| TeachingSessionStarted / TeachingSessionPaused / TeachingSessionResumed / TeachingSessionCompleted / TeachingSessionCancelled / TeachingSessionAbandoned / TeachingSessionArchived | D04 | D05 and interested D/E consumers | Records execution and supported completed outcomes. | L2-D04 §46.1 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D03 | Teaching Session & Attendance | Owns the approved Schedule Slot consumed by D04. | Preserve ownership while enabling the scenario. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |
| D02 / Section E | Teaching Session & Attendance | Own Faculty Assignment and faculty identity facts consumed as references. | Preserve ownership while enabling the scenario. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Explicitly starts and conducts the Teaching Session and records its outcome. | Assigned or authorized conducting faculty | Requires applicable business permission. | Teaching Session & Attendance |
| Applies governed cancellation, override or archival action when policy requires. | Authorized academic administrator | Policy-bound authority. | Teaching Session & Attendance |

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

- The session records one valid outcome—Completed, Partially Completed, Cancelled, Abandoned or Not Conducted—with assigned and conducting faculty evidence preserved.
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
