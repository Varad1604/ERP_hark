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
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H06 — Alumni Workflows; applicable locked Student Lifecycle architecture

# 2. Scenario Purpose

Govern an Alumni mentoring request from matching through active mentoring and completion while keeping the Student participant under Section C ownership and the Mentorship Relationship under H06 ownership.

# 3. Business Outcome

Suitable participants are matched, the engagement is governed, and completion or early closure is recorded

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a student or alumnus requests a mentoring relationship.

## 4.2 End Boundary

The scenario ends when H06 records completion of the Mentorship Relationship or an authoritative early-closure outcome.

## 4.3 In Scope

- Mentor registration/approval where required before matching.
- Mentor–mentee matching.
- Mentorship activation, session scheduling, outcome recording, on-hold/resume handling, and closure.
- Preservation of Student identity/lifecycle ownership in C.

## 4.4 Out of Scope

- Student lifecycle or academic-record changes.
- Faculty/employee assignment.
- Inventing an early-closure state/event that H06 does not define.

# 5. Ownership and Participating Domains

Student Lifecycle / C owns Student records and Student lifecycle. Alumni / H06 owns Mentor Profile and Mentorship Relationship, including the mentoring lifecycle.

# 6. Business Actors, Roles and Responsibilities

H06 supports Alumni/Mentor participants and Alumni administrative roles. The Student participates as mentee without transferring Student ownership to H06.

# 7. Preconditions and Entry Criteria

- A suitable eligible Mentor can be identified under H06 rules.
- The mentee is an eligible Student or Alumni participant.
- Required H06 mentorship information is available.
- The actor is authorized for matching and lifecycle actions.

# 8. Trigger

A student or alumnus requests a mentoring relationship

# 9. Main Business Flow

1. Receive the mentoring request.
2. Validate mentor eligibility/availability and mentee reference.
3. Match the Mentor with the mentee, establishing the H06 Mentorship Relationship and moving `Requested → Approved`.
4. Schedule mentoring activity and move `Approved → Active`.
5. Record mentoring sessions/outcomes under H06.
6. Where necessary, move `Active → On Hold → Active`.
7. Close the active mentorship using `CloseMentorship`, which moves the H06 transition matrix to `Completed`.
8. Publish `MentorshipCompleted` and preserve mentoring history.

# 10. Alternate Business Flows

- The relationship may be placed `On Hold` and later resumed.
- A requested mentorship may remain unmatched until a suitable Mentor is identified.
- H06's lifecycle list includes `Closed`, but its authoritative transition matrix does not define a transition into `Closed`.

# 11. Failure, Interruption and Business Recovery

If suitable participants cannot be matched, no Mentorship Relationship progresses to Active. If an active engagement is interrupted, H06 may use the defined `On Hold`/resume path. L3 does not invent an undefined early-closure transition.

# 12. Business Rules and Decision Constraints

- H06 owns Mentorship Relationship.
- Student records remain C-owned.
- `MatchMentorWithStudent` establishes the specific mentoring relationship.
- `CloseMentorship` is terminal and results in `Completed` in the H06 transition matrix.
- H06 defines `Requested`, `Approved`, `Active`, `On Hold`, `Completed`, and also lists `Closed`, but does not define how `Closed` is reached.

# 13. State and Lifecycle Impact

Defined transitions are `Requested → Approved → Active → Completed`, with `Active ↔ On Hold`. The separately listed `Closed` state has no canonical transition in H06.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H06 defines `MentorshipCreated`, `MentorshipSessionScheduled`, and `MentorshipCompleted`. The Student participant remains referenced from Section C.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H06 references the Student mentee without modifying Student lifecycle or academic records.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Participants initiate mentoring; H06-authorized roles perform mentor approval/matching and governed lifecycle actions as defined upstream.

## 15.2 Separation of Duties, Overrides and Exceptions

No local rule maps an undefined `Closed` state to another H06 state.

## 15.3 Business Audit and Evidence Obligations

Preserve mentor/mentee references, matching decision, sessions, outcomes, on-hold/resume history, closure action, and completion evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

A matched mentorship reaches the H06 `Completed` outcome through `CloseMentorship`, with history preserved.

## 16.2 Postconditions

Completed mentoring is source-grounded. The frozen phrase “early closure” is not separately represented by a defined H06 state/event, although H06 lists an otherwise unreachable `Closed` state.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

H06 lists `Closed` in the Mentorship lifecycle vocabulary but provides no transition into it; `CloseMentorship` instead moves `Active → Completed` and publishes `MentorshipCompleted`. Therefore ordinary completion is fully grounded, while a separately distinguishable early-closure outcome is ambiguous.

**Architecture impact:** this does not prevent a usable completed-mentorship Draft, but upstream lifecycle clarification is required if early closure must remain distinct from completion.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumption is made that `Completed` and `Closed` are interchangeable.

## 17.3 Deferred Questions

Clarify whether early closure is represented by the existing `CloseMentorship → Completed` path or requires the currently unreachable `Closed` state.
