# Hostel Admission Application to Admission Decision

**Scenario ID:** L3-G-079  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-G03 — Hostel Management Workflows; locked L1 Business Domain Architecture; Section C Student Lifecycle context; Section E People & Workforce context; Section H Notifications; DEPENDENCY_MAP.md  
**Referenced By:** L3-G-080 — Approved Hostel Admission to Active Residency  

## 2. Scenario Purpose

Determine whether a student is granted operational eligibility for hostel residency and conclude the hostel admission request with an authoritative decision, without creating active occupancy or altering the Student Lifecycle.

## 3. Business Outcome

The hostel admission application reaches a governed decision of **Approved**, **Waitlisted**, **Rejected**, or **Withdrawn**, with the corresponding hostel admission state recorded and available for the next permitted residency step.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a hostel admission application is submitted from an approved hostel-admission source.

### 4.2 End Boundary

The scenario ends when the Hostel Admission aggregate reaches a governed decision state and the resulting admission fact is available for the next hostel process, including room allocation when approved.

### 4.3 In Scope

- Hostel admission application submission.
- Eligibility validation for hostel residency.
- Business review and decision.
- Approval, waitlisting, rejection, or withdrawal of the hostel admission request.
- Publication of the canonical hostel admission outcome through the G03 business event model.

### 4.4 Out of Scope

- Academic admission to the institution.
- Student lifecycle state changes.
- Room allocation and occupancy activation; these are subsequent hostel processes.
- Creation, modification, or ownership of hostel infrastructure.
- Financial accounting.
- Technical authentication, authorization enforcement, notifications, or implementation mechanics.

### 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-G-080 — Approved Hostel Admission to Active Residency | An approved hostel admission creates the business precondition for the later occupancy-activation scenario. |

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Hostel Management (G) | Owns Hostel Admission, eligibility determination within hostel policy, the admission decision, admission lifecycle, and the resulting residency eligibility. |
| Student Lifecycle (C) | Provides canonical Student Identity / student context consumed for hostel eligibility; it does not own the hostel admission decision. |
| People & Workforce (E) | Provides relevant workforce/employee context where hostel operational roles participate; it does not own hostel admissions. |
| Institutional Services & Intelligence (H) | Provides notification infrastructure for business-event-driven communications; it does not own the hostel admission outcome. |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student | Submits the hostel application where the applicable admission source is student-originated. | Initiates the request. |
| Hostel Administrator | Reviews and governs hostel admission processing. | Business decision authority for hostel admission is supported by L2-G03. |
| Hostel Warden | May participate in admission approval under configurable hostel authority. | Business approval participation supported by L2-G03. |

The authoritative source defines these operational roles and configurable approval participation. No additional business role is invented.

## 7. Preconditions and Entry Criteria

- A valid student identity is available from the Student Lifecycle domain.
- The hostel admission application concerns an institutional hostel and an applicable hostel category or residency scope.
- The admission source is one of the configured sources supported by L2-G03, such as Student Application, Administrative Allocation, Management Quota, International Student Allocation, Special Accommodation Request, or Hostel Renewal.
- The Hostel Management business process is available to process the application.

## 8. Trigger

A hostel admission application is submitted for a student.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | A hostel admission application is submitted using an applicable configured admission source. | Student / Hostel Management | A Hostel Admission request enters the `Applied` lifecycle state. |
| 2 | Hostel Management verifies the student's canonical identity and evaluates the hostel admission criteria. | Hostel Management with Student Lifecycle context | Required student and hostel eligibility facts are available for decision. |
| 3 | Eligibility is assessed using applicable hostel rules, which may consider student status, academic programme, campus, hostel category, gender policy, institution rules, hostel availability, and administrative approval. | Hostel Management | The application is determined eligible, ineligible, or subject to an availability outcome under configured policy. |
| 4 | The application proceeds through the configured hostel admission review and approval path. | Hostel Administrator / Hostel Warden | A governed business decision is reached. |
| 5 | Hostel Management records the resulting decision as Approved, Waitlisted, Rejected, or Withdrawn according to the governing outcome. | Hostel Management | The Hostel Admission lifecycle reaches its corresponding decision state. |
| 6 | The applicable canonical hostel admission event is published for interested consumers. | Hostel Management | The admission outcome is available to downstream business participants without transferring ownership. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | Hostel capacity is unavailable under applicable allocation policy. | Place the hostel application on the configured waiting list rather than approve immediate residency eligibility. | Admission reaches `Waitlisted`. | Hostel Management |
| A2 | Eligibility requirements are not satisfied. | Decline the hostel admission request through the governed admission decision path. | Admission reaches `Rejected`. | Hostel Management |
| A3 | The applicant withdraws the hostel request before a final approved residency is established. | Record the application withdrawal under the Hostel Admission lifecycle. | Admission reaches `Withdrawn`. | Hostel Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required student identity or eligibility information is unavailable or invalid. | Hostel eligibility cannot be authoritatively determined. | Resolve the student-information deficiency through the owning source before the hostel decision is concluded. | Hostel Management / Student Lifecycle |
| A configured approval path requires business review that has not yet occurred. | Admission cannot be concluded as approved. | Keep the application in the applicable review stage until the governed decision is completed. | Hostel Management |
| No suitable room capacity exists under the applicable policy. | Immediate approval cannot proceed to residency allocation. | Place the application on the waiting list when permitted by policy. | Hostel Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Hostel admission governs residency eligibility, not academic admission. | Hostel Management | The scenario may approve hostel residency eligibility without activating a student in Student Lifecycle. | L2-G03 §1, §16 |
| Eligibility may consider Student Status, Academic Programme, Campus, Hostel Category, Gender Policy, Institution Rules, Hostel Availability, and Administrative Approval. | Hostel Management | These factors constrain the admission decision. | L2-G03 §16 |
| Admission states include Applied, Under Review, Approved, Waitlisted, Rejected, and Withdrawn, with values remaining configurable. | Hostel Management | The scenario concludes in an authorized admission lifecycle state without inventing additional states. | L2-G03 §16, §30 |
| Where capacity is unavailable, applications may be placed on a waiting list. | Hostel Management | Waiting-list placement is a governed alternate outcome. | L2-G03 §16 |
| Hostel Management does not manage Student Lifecycle. | Student Lifecycle | The scenario consumes student information but never creates or modifies Student Context lifecycle state. | L2-G03 §4, §13–§14 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Hostel Admission | Applied / Under Review | Eligibility verification and governed decision determine the admission outcome. | Approved / Waitlisted / Rejected / Withdrawn | G03 Hostel Management | L2-G03 §16, §30 |

No Student Context lifecycle transition is owned by this scenario; Student Lifecycle remains canonical owner of student state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| HostelAdmissionApplied | G03 Hostel Management | Interested downstream hostel consumers | Records that a hostel application was submitted. | L2-G03 §28 |
| HostelAdmissionApproved | G03 Hostel Management | Student Operations, Finance, Security, Institutional Reporting, Notifications as applicable | Communicates the approved hostel admission outcome. | L2-G03 §28, §9 |
| HostelAdmissionRejected | G03 Hostel Management | Interested downstream consumers | Communicates the rejected admission outcome. | L2-G03 §28 |
| HostelAdmissionWaitlisted | G03 Hostel Management | Interested downstream consumers | Communicates waiting-list placement. | L2-G03 §28 |

The event publisher remains G03 Hostel Management. Consuming a hostel event does not transfer ownership of Hostel Admission.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Student Lifecycle (C) | Hostel Management (G) | Student Identity / student information | Provide canonical student context required for hostel eligibility. | L2-G03 §9 |
| Hostel Management (G) | Section H / downstream consumers | Hostel admission business events | Make the authoritative admission outcome available for notification and other downstream use. | L2-G03 §9, §27–§28 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Submit hostel admission application | Student or other configured admission source | Admission sources are configurable. | Hostel Management |
| Approve or reject hostel admission | Hostel Administrator / Hostel Warden | Admission approval is configurable within hostel policy. | Hostel Management |

### 15.2 Separation of Duties, Overrides and Exceptions

L2-G03 identifies configurable approval workflows but does not define a separate scenario-specific separation-of-duty rule for hostel admission. No additional separation rule is invented.

Administrative override is permitted as a recovery/control capability where institutional policy allows it; the exact override authority is configuration dependent.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Hostel admission decisions must remain auditable. | Immutable admission history and audit information associated with the admission decision. | Hostel Management, with Section A audit infrastructure |

L2-G03 specifically states that significant residency activity is auditable and that Hostel Admission is an audited activity.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- A valid Hostel Admission application has a governed decision state.
- The decision is one of the authoritative outcomes permitted by the lifecycle: Approved, Waitlisted, Rejected, or Withdrawn.
- The resulting admission state is recorded in Hostel Management's canonical lifecycle.
- The corresponding canonical business event is published where defined by the outcome.

### 16.2 Postconditions

- An approved application is eligible to proceed to the separate room-allocation/residency process.
- A waitlisted application remains governed by waiting-list policy until further action.
- A rejected or withdrawn application does not create active hostel occupancy.
- Student Lifecycle remains unchanged by the hostel admission decision.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues that prevent this scenario from expressing the canonical business outcome have been identified.

### 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Hostel admission approval is governed through the configurable approval path described by L2-G03 and may involve Hostel Administrator and/or Hostel Warden. | L2-G03 §31 identifies these roles and configurable approval. | Exact institution-specific approval sequencing would need the applicable configuration. |

### 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| How notification delivery is technically performed after hostel admission events is deferred. | L5 | Section H owns notification delivery and L3 must remain implementation independent. |

