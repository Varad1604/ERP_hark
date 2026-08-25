# Operational Timetable to Retired Historical Timetable

# 1. Scenario Metadata

**Scenario ID:** L3-D-022  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Timetable  
**Participating Domains:** D  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D03_Timetable_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0742 / CAN-L3-D-022

---

# 2. Scenario Purpose

Retire an operational timetable that is no longer eligible for use and preserve it as an immutable historical record.

# 3. Business Outcome

The timetable is retired, archived and available for historical search, audit and reporting without operational re-entry.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized retirement requirement exists and the timetable satisfies retirement eligibility.

## 4.2 End Boundary

The timetable reaches Archived after `TimetableRetired` and `TimetableArchived` are published by D03.

## 4.3 In Scope

- Verify retirement eligibility and replacement or closure conditions.
- Close operational scheduling use.
- Retire and preserve the timetable as historical truth.

## 4.4 Out of Scope

- Deleting timetable history.
- Reactivating an archived timetable.
- Changing downstream execution records.
- Technical archival, persistence or infrastructure design.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D03 | Owns retirement, archival, timetable history and lifecycle. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Administrator | Verifies retirement eligibility and authorizes retirement. | Decides retirement under the governing timetable policy. |

Historical preservation is a D03 business responsibility and is not represented as a separate technical service actor at L3.

# 7. Preconditions and Entry Criteria

- The timetable is operational and no longer required as the governing schedule.
- Retirement does not leave an invalid active-scope condition.
- Required historical-retention obligations can be satisfied.

# 8. Trigger

The operational period ends or a governed replacement makes the timetable retirement-eligible.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate retirement eligibility, dependencies and operational closure. | Academic Administrator / D03 | Retirement is permitted or blocked. |
| 2 | Close the timetable for operational scheduling use. | D03 | New operational reliance is prohibited. |
| 3 | Authorize and record timetable retirement. | Academic Administrator / D03 | Active → Retired and `TimetableRetired`. |
| 4 | Preserve the complete timetable history under the governed D03 archival lifecycle. | D03 | Retired → Archived and `TimetableArchived`. |

# 10. Alternate Business Flows

No distinct alternate business path is asserted. A failed eligibility check, rejected retirement decision or interruption blocks completion and is governed under Section 11.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Retirement eligibility or authoritative reference cannot be validated | Retirement cannot proceed. | Resolve the blocking eligibility/reference condition while retaining the current authoritative timetable state. | Timetable |
| Retirement authority rejects or cannot authorize the request | The timetable remains in its prior governed state. | Record the decision and continue only through a later valid retirement request. | Timetable |
| Retirement or archival progression is interrupted | The Archived outcome has not yet occurred. | Preserve the last valid business position and continue only when the blocking business condition is resolved. | Timetable |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Timetable | Prevents ownership leakage. | `L2-D03_Timetable_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Timetable | Prevents governance bypass. | `L2-D03_Timetable_Workflows.md` |
| Published or historical business facts remain immutable. | Timetable | Preserves historical truth and auditability. | `L2-D03_Timetable_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Timetable | Active | Authorize retirement and preserve historical record. | Active → Retired → Archived | D03 | L2-D03 WF-11 and lifecycle transitions |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `TimetableRetired` / `TimetableArchived` | D03 | Audit, reporting and authorized D-domain consumers | Records retirement and completed archival. | L2-D03 WF-11 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates. Downstream consumers may use completed D03 business facts without gaining Timetable ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Verify eligibility and authorize retirement. | Academic Administrator | Acts under the governing timetable retirement policy. | Timetable |
| Preserve the retired timetable as historical truth. | D03 | Preservation is a domain responsibility and carries no separate governance authority. | Timetable |

## 15.2 Separation of Duties, Overrides and Exceptions

Any separation between retirement decision and historical preservation follows the authoritative D03 policy. Any override requires business authority, justification, evidence and preserved decision history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Retirement lifecycle and decision | Actor, authority, reason, prior state, eligibility evidence, decision and resulting state. | Timetable |
| Historical traceability | Prior timetable version, retirement evidence and archival business facts. | Timetable |
| Failure and recovery | Blocking condition, decision, correction/resubmission and final business outcome. | Timetable |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The timetable reaches Retired and then Archived under the D03 lifecycle.
- `TimetableRetired` and `TimetableArchived` are published by D03 as applicable.
- Required authority and audit evidence is retained.

## 16.2 Postconditions

- The timetable cannot return to normal operational use through this scenario.
- Historical timetable truth remains immutable and available for authorized institutional use.
- Canonical ownership remains with D03.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issue identified by this purity correction.

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Technical archival, persistence, storage, indexing and infrastructure recovery are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** Section D canonical register; `L2-D03_Timetable_Workflows.md`; `L3_Business_Scenario_Template_v2.0.md`; `Architecture_Level_Validation_Standards.md`.
