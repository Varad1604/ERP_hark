# Scheduled Academic Session to Governed Teaching Session Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-D-023  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Teaching Session & Attendance  
**Participating Domains:** D, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0743 / CAN-L3-D-023

---

# 2. Scenario Purpose

Convert an approved Schedule Slot into a governed Teaching Session execution and record one canonical instructional outcome.

# 3. Business Outcome

The session records one valid outcome—Completed, Partially Completed, Cancelled, Abandoned or Not Conducted—with assigned and conducting faculty evidence preserved.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An approved D03 Schedule Slot and an assigned or otherwise authoritatively permitted conducting faculty member are available.

## 4.2 End Boundary

A canonical Teaching Session outcome is recorded and the applicable D04 completed business fact is published.

## 4.3 In Scope

- Validate the Schedule Slot and faculty authority.
- Start and conduct the Teaching Session.
- Record interruptions and resume where institutionally permitted.
- Record the supported session outcome and preserve planned-versus-actual delivery evidence.

## 4.4 Out of Scope

- Timetable replanning for remaining work.
- Faculty employment or assignment ownership.
- Attendance correction after finalization.
- Technical access-control or execution mechanics.

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
| Assigned or authorized conducting faculty | Starts and conducts the Teaching Session and records the instructional outcome. | May act only where the governing D04 and faculty-assignment policy establishes authority for the session. |

The source establishes authorized administrative actions for some exceptional outcomes, but this revision does not invent a universal administrator role where the exact institutional business-role name is not explicit upstream; the authority-name gap is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- Schedule Slot is approved.
- Faculty assignment or substitution authority is valid.
- No other active Teaching Session exists for the Schedule Slot.

# 8. Trigger

Authorized conducting faculty begins instructional delivery, or a governed supported non-conducted outcome must be recorded.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate the Schedule Slot and faculty authority. | D04 using D03/D02/Section E facts | The Teaching Session may begin or remains blocked. |
| 2 | Establish and start the Teaching Session. | Conducting faculty / D04 | `TeachingSessionStarted`. |
| 3 | Record instructional execution, progress, notes, evidence and attendance as applicable. | Conducting faculty / D04 | Session is In Progress with attributable evidence. |
| 4 | Where a legitimate interruption occurs, record the interruption and resume only through the supported business path. | Conducting faculty / D04 | Interruption history remains traceable. |
| 5 | Validate and record one supported canonical outcome. | D04 under applicable business authority | Completed, Partially Completed, Cancelled, Abandoned or Not Conducted. |
| 6 | Apply later lock/archive actions only where the D04 lifecycle and policy permit. | D04 | Historical execution remains governed. |

# 10. Alternate Business Flows

## 10.1 Supported Non-Completed Session Outcomes

Where the Teaching Session cannot legitimately reach Completed, D04 may record another canonical supported terminal outcome such as Partially Completed, Cancelled, Abandoned or Not Conducted when its business conditions and authority are satisfied. These are governed session outcomes, not technical failures.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Schedule Slot or faculty authority cannot be validated | The Teaching Session cannot validly start or continue. | Resolve the authoritative reference/authority condition with its canonical owner before continuation. | D04 with source-domain participation |
| A requested exceptional outcome lacks required authority | The requested outcome cannot be recorded. | Retain the current authoritative session state and obtain a valid governed decision. | D04 |
| Session processing is interrupted before a terminal outcome is recorded | The final Teaching Session outcome has not occurred. | Preserve the last valid business position and continue only through a supported D04 continuation or terminal-outcome path. | D04 |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Teaching Session & Attendance | Prevents cross-domain ownership leakage. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Teaching Session & Attendance | Prevents governance bypass. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |
| Published or historical business facts remain immutable. | Teaching Session & Attendance | Preserves auditability and lineage. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Teaching Session | Approved Schedule Slot; no Teaching Session | Establish, execute and conclude with one supported outcome. | Started → In Progress → supported terminal outcome; later Locked/Archived where applicable | D04 | L2-D04 §§21–22 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `TeachingSessionStarted` / `TeachingSessionPaused` / `TeachingSessionResumed` / `TeachingSessionCompleted` / `TeachingSessionCancelled` / `TeachingSessionAbandoned` / `TeachingSessionArchived` | D04 | D05 and authorized D/E consumers | Records supported Teaching Session execution facts. | L2-D04 §46.1 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D03 | D04 | Approved Schedule Slot. | Establish authoritative scheduling context without ownership transfer. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |
| D02 / Section E | D04 | Faculty Assignment and faculty identity facts. | Establish faculty authority context without ownership transfer. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Start and conduct the Teaching Session. | Assigned or authorized conducting faculty | Must be validly assigned or otherwise authorized for the session. | D04 |
| Record an exceptional terminal outcome requiring administrative authority | Exact institutional role name unresolved in reviewed source | Must follow D04 policy; this scenario does not invent a placeholder role or technical permission. | D04 |

## 15.2 Separation of Duties, Overrides and Exceptions

Where D04 policy separates conducting-faculty activity from exceptional administrative decisions, those business authorities remain distinct. Overrides require business authority, justification and retained evidence.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Session lifecycle and outcome | Conducting actor, faculty authority, Schedule Slot reference, prior state, outcome reason and resulting state. | D04 |
| Cross-domain traceability | Schedule Slot, Faculty Assignment and faculty identity references. | D04 with source ownership retained upstream |
| Failure and recovery | Blocking authority/reference condition, interruption history and final supported business outcome. | D04 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Exactly one supported canonical Teaching Session outcome is recorded for the scenario execution.
- Required faculty/decision authority and evidence are retained.
- Applicable D04 business events are published by D04.

## 16.2 Postconditions

- D03 retains ownership of the Schedule Slot.
- D02/Section E retain ownership of Faculty Assignment and faculty identity facts.
- D04 retains ownership of the Teaching Session and its execution history.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The reviewed D04 source supports authorized exceptional administrative actions but does not establish one universal canonical institutional role name for every cancellation/override/archive decision used by this scenario. | L3 must not invent a generic administrator or permission code. | Confirm the applicable canonical business authority naming upstream before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Technical RBAC, permission codes, transaction processing, persistence and infrastructure recovery are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** Section D canonical register; `L2-D04_Teaching_Session_and_Attendance_Workflows.md`; `L3_Business_Scenario_Template_v2.0.md`; `Architecture_Level_Validation_Standards.md`.
