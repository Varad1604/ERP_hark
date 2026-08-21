# Operational Timetable to Governed Revised Timetable

**Scenario ID:** L3-D-021  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Timetable  
**Participating Domains:** D, E, G  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D03_Timetable_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0741 / CAN-L3-D-021

---

# 2. Scenario Purpose

Govern a material change to an operational timetable through impact analysis, revision, validation, approval and republication without rewriting any published historical version.

# 3. Business Outcome

A new approved timetable version is published and becomes the governed operational timetable, while every preceding version remains immutable and traceable.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized revision request exists for an operational timetable.

## 4.2 End Boundary

The revised timetable reaches Published and `TimetableRepublished` is published; activation follows the applicable authorized operating window.

## 4.3 In Scope

- Classify and justify the revision.
- Assess faculty, resource, calendar, Course Offering and operational impact.
- Create, validate, approve and publish a revised immutable version.

## 4.4 Out of Scope

- Editing a published timetable version in place.
- Changing faculty, Course Offering or resource master ownership.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D03 | Owns the Timetable aggregate, version lineage, revision, validation, approval and republication. |
| D02 / Section E | Provide authoritative Faculty Assignment and faculty-reference facts. |
| Section G | Provides authoritative physical-resource availability facts. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Scheduler | Performs impact analysis, prepares revised draft and validates constraints. | Responsible. |
| Academic Administrator | Approves or rejects the revised timetable. | Decides revision approval. |
| Registrar | Publishes the approved revised version. | Decides republication. |
| System Scheduler | Supports constraint evaluation and optimization. | No governance decision. |

# 7. Preconditions and Entry Criteria

- Operational timetable and immutable published version exist.
- Revision reason, affected scope and authoritative dependencies are available.

# 8. Trigger

A governed operational, academic, faculty or resource change requires timetable revision.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register the revision request and affected scope. | Academic Scheduler | Governed revision case exists. |
| 2 | Analyze academic, faculty, resource and downstream impact. | Academic Scheduler with authoritative contributors | Impact evidence is complete. |
| 3 | Create a new revised timetable version; retain the prior version unchanged. | D03 | Frozen → Revised on the new governed version path. |
| 4 | Validate constraints, conflicts and reservations. | Academic Scheduler | Revised version is valid or returned for correction. |
| 5 | Approve or reject the revised version. | Academic Administrator | Approval decision recorded. |
| 6 | Publish the approved revised version. | Registrar | Revised → Published; `TimetableRevisionCreated` and `TimetableRepublished` available. |

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
| Timetable | Frozen operational version | Create and govern a distinct revised version. | Frozen → Revised → Published | D03 | L2-D03 WF-10 and lifecycle transitions |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| TimetableRevisionCreated / TimetableRepublished | D03 | D02, D04, Section E, Section G and operational consumers | Records version creation and governed republication. | L2-D03 WF-10 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D02 / Section E | Timetable | Provide authoritative Faculty Assignment and faculty-reference facts. | Preserve ownership while enabling the scenario. | `L2-D03_Timetable_Workflows.md` |
| Section G | Timetable | Provides authoritative physical-resource availability facts. | Preserve ownership while enabling the scenario. | `L2-D03_Timetable_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Performs impact analysis, prepares revised draft and validates constraints. | Academic Scheduler | Responsible. | Timetable |
| Approves or rejects the revised timetable. | Academic Administrator | Decides revision approval. | Timetable |
| Publishes the approved revised version. | Registrar | Decides republication. | Timetable |
| Supports constraint evaluation and optimization. | System Scheduler | No governance decision. | Timetable |

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

- A new approved timetable version is published and becomes the governed operational timetable, while every preceding version remains immutable and traceable.
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
