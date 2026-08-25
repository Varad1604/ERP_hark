# Operational Timetable to Governed Revised Timetable

# 1. Scenario Metadata

**Scenario ID:** L3-D-021  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Timetable  
**Participating Domains:** D, E, G  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
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
- Changing faculty, Course Offering or resource-master ownership.
- Technical scheduling, persistence, transaction, retry or recovery design.

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
| Academic Scheduler | Performs impact analysis, prepares the revised draft and validates timetable constraints. | Responsible for governed timetable revision preparation. |
| Academic Administrator | Approves or rejects the revised timetable. | Decides revision approval. |
| Registrar | Publishes the approved revised version. | Decides republication. |

# 7. Preconditions and Entry Criteria

- An operational timetable and immutable published version exist.
- The revision reason, affected scope and authoritative dependencies are available.

# 8. Trigger

A governed operational, academic, faculty or resource change requires timetable revision.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register the revision request and affected scope. | Academic Scheduler | Governed revision case exists. |
| 2 | Analyze academic, faculty, resource and downstream impact using authoritative facts. | Academic Scheduler with authoritative contributors | Impact evidence is complete. |
| 3 | Create a distinct revised timetable version while retaining the prior published version unchanged. | D03 | A governed revised version exists. |
| 4 | Validate constraints, conflicts and reservations. | Academic Scheduler | Revised version is valid or returned for correction. |
| 5 | Approve or reject the revised version. | Academic Administrator | Approval decision is recorded. |
| 6 | Publish the approved revised version. | Registrar / D03 | Revised → Published; `TimetableRevisionCreated` and `TimetableRepublished` are established as applicable. |

# 10. Alternate Business Flows

No separate alternate path is asserted beyond the governed revision path above. A validation failure, rejection or interruption blocks successful republication and is handled under Section 11.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Validation or authoritative reference failure | The revised timetable cannot proceed to approval or publication. | Correct the invalid revision or authoritative reference while retaining the prior published timetable unchanged. | Timetable |
| Revision approval is rejected or authority is invalid | The revised timetable does not become operational. | Retain the prior authoritative timetable and record the rejection/authority decision. | Timetable |
| Revision activity is interrupted before publication | No new published timetable outcome has occurred. | Preserve the last valid business position and continue only when the blocking business condition is resolved. | Timetable |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Timetable | Prevents cross-domain ownership leakage. | `L2-D03_Timetable_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Timetable | Prevents governance bypass. | `L2-D03_Timetable_Workflows.md` |
| Published or historical business facts remain immutable. | Timetable | Preserves auditability and version lineage. | `L2-D03_Timetable_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Timetable | Frozen operational version | Create and govern a distinct revised version. | Frozen → Revised → Published | D03 | L2-D03 WF-10 and lifecycle transitions |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `TimetableRevisionCreated` / `TimetableRepublished` | D03 | D02, D04, Section E, Section G and authorized operational consumers | Records version creation and governed republication. | L2-D03 WF-10 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D02 / Section E | Timetable | Authoritative Faculty Assignment and faculty-reference facts. | Preserve ownership while enabling revision impact analysis. | `L2-D03_Timetable_Workflows.md` |
| Section G | Timetable | Authoritative physical-resource availability facts. | Preserve ownership while enabling revision impact analysis. | `L2-D03_Timetable_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Prepare and validate the revision. | Academic Scheduler | Acts within governed timetable-revision responsibility. | Timetable |
| Approve or reject the revised timetable. | Academic Administrator | Decides revision approval. | Timetable |
| Publish the approved revised version. | Registrar | Republishes only an approved revision. | Timetable |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, approval and publication remain distinct wherever the authoritative source or institutional policy requires separation. Any governed override requires authority, justification, evidence and preserved decision history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Revision lifecycle and decision | Actor, authority, reason, prior version, validation evidence, approval decision and resulting version. | Timetable |
| Source and cross-domain traceability | Authoritative source references, versions and business facts used in impact analysis. | Timetable |
| Failure and recovery | Validation rejection, correction/resubmission, interruption and final business outcome. | Timetable |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A new approved timetable version is Published and becomes the governed operational timetable.
- Every preceding published timetable version remains immutable and traceable.
- `TimetableRepublished` is published by D03 for the completed republication outcome.
- Required authority, validation and audit evidence is retained.

## 16.2 Postconditions

- Canonical ownership remains unchanged.
- Historical timetable versions remain immutable.
- Downstream domains consume published facts without modifying the D03-owned Timetable aggregate.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues identified by this purity correction.

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Technical scheduling, persistence, transaction, retry, orchestration and infrastructure recovery design are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** Section D canonical register; `L2-D03_Timetable_Workflows.md`; `L3_Business_Scenario_Template_v2.0.md`; `Architecture_Level_Validation_Standards.md`.
