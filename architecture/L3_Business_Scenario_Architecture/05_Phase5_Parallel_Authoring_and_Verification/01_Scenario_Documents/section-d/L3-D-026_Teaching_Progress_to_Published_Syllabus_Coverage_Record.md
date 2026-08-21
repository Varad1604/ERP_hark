# Teaching Progress to Published Syllabus Coverage Record

**Scenario ID:** L3-D-026  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Teaching Session & Attendance  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0746 / CAN-L3-D-026

---

# 2. Scenario Purpose

Record, validate and publish actual instructional progress and topic coverage for a Teaching Session as immutable execution facts.

# 3. Business Outcome

A validated Teaching Progress record is Published with canonical topic-coverage states and retained references to the Teaching Session, curriculum topic and learning outcomes.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An eligible Teaching Session is active or otherwise permits progress recording under policy.

## 4.2 End Boundary

Teaching Progress reaches Published and becomes immutable; later corrections add new business history.

## 4.3 In Scope

- Record atomic progress and actual topic coverage.
- Validate session state, topic and learning-outcome references.
- Publish immutable progress facts.

## 4.4 Out of Scope

- Curriculum or syllabus planning ownership.
- Editing a published progress fact.
- Analytics-derived conclusions.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D04 | Owns Teaching Progress, actual topic coverage and publication history. |
| D01 | Owns planned curriculum, topic and Learning Outcome references consumed by D04. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized conducting faculty | Records actual teaching progress and topic coverage. | Record Teaching Progress permission. |
| Authorized progress publisher | Validates and publishes progress under institutional policy. | Policy-bound publication authority. |

# 7. Preconditions and Entry Criteria

- Teaching Session exists in a permitted state.
- Curriculum topic and Learning Outcome references are authoritative where applicable.

# 8. Trigger

Instructional delivery creates a progress or coverage fact requiring official publication.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Record atomic instructional progress against the Teaching Session. | Authorized conducting faculty | Progress Recorded. |
| 2 | Record actual coverage as Planned, Started, Partially Covered, Fully Covered, Deferred or Cancelled. | Authorized conducting faculty | Canonical execution state captured. |
| 3 | Validate session state, topic reference, learning outcomes and consistency. | D04 | Progress is Validated or returned for correction. |
| 4 | Publish the validated progress record. | Authorized publisher / D04 | Validated → Published. |
| 5 | Preserve publication as immutable; any correction creates new history. | D04 | Published syllabus-coverage truth remains auditable. |

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
| Teaching Progress | Progress Recorded | Validate and publish actual instructional facts. | Progress Recorded → Validated → Published | D04 | L2-D04 §§27–28 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| TeachingProgressRecorded / TopicCoverageRecorded | D04 | D05 and authorized academic consumers | Records actual teaching progress and topic coverage facts. | L2-D04 §46.3 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D01 | Teaching Session & Attendance | Owns planned curriculum, topic and Learning Outcome references consumed by D04. | Preserve ownership while enabling the scenario. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Records actual teaching progress and topic coverage. | Authorized conducting faculty | Record Teaching Progress permission. | Teaching Session & Attendance |
| Validates and publishes progress under institutional policy. | Authorized progress publisher | Policy-bound publication authority. | Teaching Session & Attendance |

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

- A validated Teaching Progress record is Published with canonical topic-coverage states and retained references to the Teaching Session, curriculum topic and learning outcomes.
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
