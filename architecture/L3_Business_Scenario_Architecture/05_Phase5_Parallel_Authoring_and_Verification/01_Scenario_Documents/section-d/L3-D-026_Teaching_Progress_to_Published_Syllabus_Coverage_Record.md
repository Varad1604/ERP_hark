# Teaching Progress to Published Syllabus Coverage Record

# 1. Scenario Metadata

**Scenario ID:** L3-D-026  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Teaching Session & Attendance  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0746 / CAN-L3-D-026

---

# 2. Scenario Purpose

Record, validate and publish actual instructional progress and topic coverage for a Teaching Session as governed D04 execution facts.

# 3. Business Outcome

A validated Teaching Progress record reaches Published with canonical topic-coverage states and retained references to the Teaching Session, curriculum topic and Learning Outcomes.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An eligible Teaching Session is active or otherwise permits progress recording under D04 policy.

## 4.2 End Boundary

Teaching Progress reaches Published and becomes immutable as historical truth; any later correction creates new governed history.

## 4.3 In Scope

- Record instructional progress and actual topic coverage.
- Validate Teaching Session state and authoritative curriculum/Learning Outcome references.
- Publish the governed progress fact.

## 4.4 Out of Scope

- Curriculum or syllabus-planning ownership.
- Editing a Published progress fact in place.
- Analytics-derived conclusions.
- Technical permissions, persistence, retries or atomic processing.

## 4.5 Related Scenarios — Optional

Related scenarios may consume the Published D04 progress fact without changing ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D04 | Owns Teaching Progress, actual topic coverage and publication history. |
| D01 | Owns planned curriculum, topic and Learning Outcome references consumed by D04. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Business Authority |
|---|---|---|
| Authorized conducting faculty | Records actual teaching progress and topic coverage. | May act only for the applicable Teaching Session under D04/faculty-assignment policy. |

The reviewed D04 source does not clearly establish one canonical institutional role name for publication authority; that gap is recorded in Section 17 rather than represented as a technical permission or generic publisher role.

# 7. Preconditions and Entry Criteria

- Teaching Session exists in a permitted state.
- Curriculum topic and Learning Outcome references are authoritative where applicable.

# 8. Trigger

Instructional delivery creates a progress or coverage fact requiring official D04 publication.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Record instructional progress against the Teaching Session. | Authorized conducting faculty / D04 | Progress Recorded. |
| 2 | Record actual coverage using the canonical supported coverage states. | Authorized conducting faculty / D04 | Topic coverage fact is established. |
| 3 | Validate session state, topic/Learning Outcome references and consistency. | D04 | Progress is valid or returned for correction. |
| 4 | Publish the validated progress under the applicable D04 business authority. | D04 | Progress becomes Published. |
| 5 | Preserve Published history; any later correction creates new governed history. | D04 | Historical syllabus-coverage truth remains traceable. |

# 10. Alternate Business Flows

No distinct alternate business path is asserted. Validation or authority failure blocks publication and is handled under Section 11.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Teaching Session or curriculum reference cannot be validated | Progress cannot be published. | Resolve the authoritative reference before publication continues. | D04 with source owner as applicable |
| Required publication authority cannot be validated | Published outcome cannot occur. | Retain the current authoritative progress state until valid business authority exists. | D04 |
| Progress publication is interrupted | Published outcome has not occurred. | Preserve the last valid business position and continue when the blocking condition is resolved. | D04 |

# 12. Business Rules and Decision Constraints

- D04 owns Teaching Progress and actual coverage. Source: L2-D04 §§27–28.
- Planned curriculum/topic/Learning Outcome definitions remain owned by D01.
- Published/historical D04 facts remain immutable; correction creates new history.
- No L2 command or implementation mechanism is treated as L3 authorization.

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Teaching Progress | Progress Recorded | Validate and publish actual instructional facts. | Progress Recorded → Validated → Published | D04 | L2-D04 §§27–28 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `TeachingProgressRecorded` / `TopicCoverageRecorded` | D04 | D05 and authorized academic consumers | Records actual teaching progress and topic coverage. | L2-D04 §46.3 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

D01 supplies authoritative curriculum/topic/Learning Outcome references; D04 retains Teaching Progress ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Conducting faculty records progress only under valid session/faculty authority. Exact canonical institutional role naming for publication is unresolved upstream and is not invented here.

## 15.2 Separation of Duties, Overrides and Exceptions

Any required separation between progress recording, validation and publication must be established by authoritative D04/institutional policy.

## 15.3 Business Audit and Evidence Obligations

Retain Teaching Session reference, faculty authority, curriculum references, recorded coverage, validation evidence, publication authority once established, and Published history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Teaching Progress is valid and Published.
- Canonical coverage states and authoritative references are retained.
- Historical publication remains immutable and auditable.

## 16.2 Postconditions

- D04 retains Teaching Progress ownership.
- D01 retains curriculum/topic/Learning Outcome ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Reviewed D04 source does not clearly establish one canonical institutional business-role name for Teaching Progress publication authority. | L3 cannot safely retain the generic `Authorized progress publisher` role or a technical permission. | Confirm the canonical business authority upstream before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Technical RBAC, persistence, atomicity, retries and implementation recovery are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`; Template v2.0; Architecture Level Validation Standards.
