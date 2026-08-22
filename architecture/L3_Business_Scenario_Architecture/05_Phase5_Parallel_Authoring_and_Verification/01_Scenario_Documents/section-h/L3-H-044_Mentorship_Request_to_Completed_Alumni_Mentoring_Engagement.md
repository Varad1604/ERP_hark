# Mentorship Request to Completed Alumni Mentoring Engagement

# 1. Scenario Metadata

**Scenario ID:** L3-H-044  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Alumni  
**Participating Domains:** C, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-C02 — Student Lifecycle & Activation Workflows; L2-H06 — Alumni Workflows  
**Referenced By:** Alumni mentoring and student engagement scenarios

# 2. Scenario Purpose

Govern a mentorship request through matching and an H06-owned mentoring engagement while preserving Section C ownership of Student information.

# 3. Business Outcome

Suitable participants are matched, the engagement is governed, and completion or early closure is recorded

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A student or alumnus requests a mentoring relationship.

## 4.2 End Boundary

The frozen outcome requires both student/alumnus request origins and completion or early closure. H06 defines mentor–student matching and completed mentorship, but not an alumnus-as-mentee matching path or a distinct early-closure lifecycle outcome.

## 4.3 In Scope

Approved mentor eligibility, mentor capacity, student reference validation, matching, mentorship sessions, On Hold/resume, outcome recording, and completion.

## 4.4 Out of Scope

Modification of Student records, invented alumnus-as-mentee eligibility, and invented early-closure state/event semantics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / C | Owns canonical Student and StudentContext information referenced for student mentees. |
| Alumni / H | Owns Mentor Profile, Mentorship Relationship, matching, session history, feedback, and mentorship lifecycle. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student | May initiate the H06-defined student mentoring request and participate as mentee. | Initiates / participates |
| Alumni | May register as a mentor and, in frozen semantics, may also initiate a mentoring request. | Initiates / participates |
| Mentor | Participates in an approved mentoring relationship and records supported mentoring activity. | Performs |
| Alumni Administrator | May approve mentors, match participants, and perform supported mentorship administration. | Reviews / performs |
| Institutional Administrator | May approve mentors or perform H06-supported matching governance. | Reviews / performs |

# 7. Preconditions and Entry Criteria

- Mentor Profile is approved.
- Mentor capacity is available.
- For the defined student path, the student is eligible for mentoring and remains canonically owned by Section C.
- H06-owned membership/mentor prerequisites are satisfied.

# 8. Trigger

A student or alumnus requests a mentoring relationship

# 9. Main Business Flow

1. Receive the mentoring request.
2. For the H06-defined student path, identify suitable approved mentors using the supported matching criteria.
3. Validate mentor approval and capacity and preserve Section C Student ownership.
4. Establish the H06 Mentorship Relationship through the defined matching action.
5. Schedule mentoring activity and move the relationship through the defined Requested, Approved, and Active lifecycle.
6. Record supported sessions, goals, outcomes, and feedback.
7. Complete the mentorship through the H06-defined completion path.
8. Stop before claiming an alumnus-as-mentee path or early-closure outcome because those semantics are not defined upstream.

# 10. Alternate Business Flows

An Active mentorship may move On Hold and later resume Active. Mentor acceptance may be required where institution policy so defines.

# 11. Failure, Interruption and Business Recovery

Ineligible students, unapproved mentors, or unavailable mentor capacity prevent matching. Unsupported alumnus-as-mentee or early-closure handling requires upstream architecture resolution.

# 12. Business Rules and Decision Constraints

- Only approved mentors may participate.
- Mentor capacity must be validated before assignment.
- Mentorship assignments must remain auditable.
- Completed mentorship relationships become read-only except through governed administrative actions.
- Student data remains owned by Section C.

# 13. State and Lifecycle Impact

H06 defines `Requested → Approved → Active → Completed` and `Active → On Hold → Active`. No distinct early-closure state is defined.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H06 defines `MentorshipCreated`, `MentorshipSessionScheduled`, and `MentorshipCompleted`. No canonical early-closure event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section C provides canonical Student references for student mentees. H06 owns the Mentorship Relationship and does not modify the Student aggregate.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H06 supports student mentoring requests, mentor participation, and Alumni/Institutional Administrator matching or mentor approval. The frozen alumnus-as-mentee authority is not fully defined.

## 15.2 Separation of Duties, Overrides and Exceptions

L3 shall not infer that `Completed` includes an early-closure outcome, or extend `MatchMentorWithStudent` to alumnus mentees.

## 15.3 Business Audit and Evidence Obligations

Preserve request, mentor approval/capacity evidence, matching, sessions, feedback, state history, completion evidence, and Student references.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The H06-defined student-to-alumni-mentor path may complete through `MentorshipCompleted`. The complete frozen scenario cannot be claimed until alumnus-as-mentee and early-closure semantics are defined.

## 16.2 Postconditions

Supported mentorship history is preserved; Section C ownership remains unchanged; unsupported branches are not fabricated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen trigger permits a **student or alumnus** to request mentoring, but H06's matching workflow and command are explicitly mentor–student.
- H06 defines Student assignment within the Mentorship Aggregate but no canonical alumnus-as-mentee matching rule, state, or contract.
- Frozen outcome requires **completion or early closure**; H06 defines `Completed` and `On Hold` but no distinct early-closure state/event/authority.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream architecture must define alumnus-as-mentee matching and the governed early-closure outcome, or formally amend frozen semantics.
