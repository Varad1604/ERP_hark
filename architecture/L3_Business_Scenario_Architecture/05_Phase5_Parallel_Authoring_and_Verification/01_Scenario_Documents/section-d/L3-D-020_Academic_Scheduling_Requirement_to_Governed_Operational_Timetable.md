# Academic Scheduling Requirement to Governed Operational Timetable

**Scenario ID:** L3-D-020  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Timetable  
**Participating Domains:** B, D, E, G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D03_Timetable_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0740 / CAN-L3-D-020

---

# 2. Scenario Purpose

Transform approved academic planning, Course Offerings, Faculty Assignments, infrastructure availability and policies into one governed Active Timetable for a scheduling scope.

# 3. Business Outcome

A validated, approved, published and Active Timetable becomes the canonical operational scheduling truth for the defined scope.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Approved academic planning prerequisites and an authorized scheduling requirement are available.

## 4.2 End Boundary

The Timetable reaches Active and `TimetableActivated` is published, with only one Active timetable for the scope.

## 4.3 In Scope

- Plan and generate Draft timetable.
- Validate constraints, resources and conflicts.
- Approve, publish and activate the timetable.

## 4.4 Out of Scope

- Ownership of academic masters, faculty employment or physical-resource masters.
- Attendance or examination execution.
- Direct modification by downstream consumers.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D03 | Sole owner of Timetable aggregate, versions, lifecycle, conflicts, reservations and publication. |
| Section B | Provides institution, structure, curriculum and calendar facts. |
| D01/D02 | Provide Course Offering and Faculty Assignment facts. |
| Section G | Provides physical resource availability and master references. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Scheduler | Plans, validates and resolves scheduling conflicts. | Responsible. |
| System Scheduler | Generates and optimizes under business constraints. | Executes; no governance decision. |
| Academic Administrator | Approves validated timetable. | Accountable for approval. |
| Registrar | Publishes and activates timetable. | Decides publication and activation. |

# 7. Preconditions and Entry Criteria

- Academic Calendar approved.
- Course Offerings approved.
- Faculty Assignments complete.
- Infrastructure availability confirmed.

# 8. Trigger

Approved academic planning information becomes available for a scheduling scope.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Initiate planning and verify authoritative prerequisites. | Academic Scheduler | Planning model created. |
| 2 | Generate Draft timetable under constraints. | System Scheduler / Academic Scheduler | Draft created. |
| 3 | Validate faculty, resources, calendar, curriculum, capacity, policy and conflicts. | Academic Scheduler | Validated or returned to Draft. |
| 4 | Resolve all critical conflicts and confirm reservations. | Academic Scheduler / resource contributors | Publication blockers cleared. |
| 5 | Approve validated timetable. | Academic Administrator | Approved or rejected. |
| 6 | Publish immutable approved version. | Registrar | Published and `TimetablePublished`. |
| 7 | Activate at authorized window. | Registrar | Active and `TimetableActivated`. |

# 10. Alternate Business Flows

The scenario follows only the source-supported alternate outcomes stated in the main flow; rejection or correction does not create the successful final outcome.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the authoritative prior state. | Timetable |
| Authorization or approval failure | Requested outcome is not permitted. | Retain prior state and record the decision reason. | Timetable |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity. | Timetable |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change the owned business state. | Timetable | Prevents cross-domain ownership leakage. | `L2-D03_Timetable_Workflows.md` |
| Every transition or effective decision requires valid authority and business validation. | Timetable | Prevents bypass of governance. | `L2-D03_Timetable_Workflows.md` |
| Historical business facts remain immutable. | Timetable | Preserves auditability and lineage. | `L2-D03_Timetable_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Timetable | Planning | Generate, validate, approve, publish and activate. | Planning → Draft → Validated → Approved → Published → Active | D03 | L2-D03 §§11–13,16 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| TimetablePlanningStarted / TimetablePlanningCompleted / TimetableDraftCreated / TimetableValidated / TimetableApproved / TimetableRejected / TimetablePublished / TimetableActivated | D03 | Sections B, D, E, G and operational consumers | Records governed scheduling lifecycle. | L2-D03 §§12.8,16 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| B | Timetable | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D03_Timetable_Workflows.md` |
| E | Timetable | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D03_Timetable_Workflows.md` |
| G | Timetable | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D03_Timetable_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Plans, validates and resolves scheduling conflicts. | Academic Scheduler | Responsible. | Timetable |
| Generates and optimizes under business constraints. | System Scheduler | Executes; no governance decision. | Timetable |
| Approves validated timetable. | Academic Administrator | Accountable for approval. | Timetable |
| Publishes and activates timetable. | Registrar | Decides publication and activation. | Timetable |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Timetable |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Timetable |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A validated, approved, published and Active Timetable becomes the canonical operational scheduling truth for the defined scope.
- Required audit evidence is retained.
- Applicable completed business facts are published.

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

No deferred questions beyond recorded open issues.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D03_Timetable_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
