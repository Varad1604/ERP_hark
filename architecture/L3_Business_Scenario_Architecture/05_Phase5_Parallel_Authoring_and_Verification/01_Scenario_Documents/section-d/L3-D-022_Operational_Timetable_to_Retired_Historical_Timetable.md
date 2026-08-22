# Operational Timetable to Retired Historical Timetable

**Scenario ID:** L3-D-022  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Timetable  
**Participating Domains:** D  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
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

The timetable reaches Archived after `TimetableRetired` and `TimetableArchived` are published.

## 4.3 In Scope

- Verify retirement eligibility and replacement/closure conditions.
- Close operational scheduling use.
- Retire and archive the immutable timetable record.

## 4.4 Out of Scope

- Deleting timetable history.
- Reactivating an archived timetable.
- Changing downstream execution records.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D03 | Owns retirement, archival, timetable history and lifecycle. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Administrator | Verifies eligibility and authorizes retirement. | Decides retirement. |
| Enterprise Archival Service | Preserves the retired timetable as an immutable historical record. | Executes archival only. |

# 7. Preconditions and Entry Criteria

- Timetable is Active and no longer required as the operational schedule.
- Retirement does not leave an invalid active-scope condition.

# 8. Trigger

The operational period ends or a governed replacement makes the timetable retirement-eligible.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate eligibility, dependencies and operational closure. | Academic Administrator / D03 | Retirement is permitted or rejected. |
| 2 | Close the timetable for operational scheduling use. | D03 | No new operational reliance is permitted. |
| 3 | Retire the timetable. | Academic Administrator | Active → Retired and `TimetableRetired`. |
| 4 | Preserve and archive the complete record. | Enterprise Archival Service | Retired → Archived and `TimetableArchived`. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Timetable |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Timetable |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Timetable |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Timetable | Prevents cross-domain ownership leakage. | `L2-D03_Timetable_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Timetable | Prevents governance bypass. | `L2-D03_Timetable_Workflows.md` |
| Published or historical business facts remain immutable. | Timetable | Preserves auditability and lineage. | `L2-D03_Timetable_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Timetable | Active | Authorize retirement and preserve historical record. | Active → Retired → Archived | D03 | L2-D03 WF-11 and lifecycle transitions |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| TimetableRetired / TimetableArchived | D03 | Audit, reporting and interested D domains | Records retirement and completed archival. | L2-D03 WF-11 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates; completed events may still be consumed under canonical contracts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Verifies eligibility and authorizes retirement. | Academic Administrator | Decides retirement. | Timetable |
| Preserves the retired timetable as an immutable historical record. | Enterprise Archival Service | Executes archival only. | Timetable |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Timetable |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Timetable |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Timetable |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The timetable is retired, archived and available for historical search, audit and reporting without operational re-entry.
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
**Validation Basis:** Section D canonical register; `L2-D03_Timetable_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
