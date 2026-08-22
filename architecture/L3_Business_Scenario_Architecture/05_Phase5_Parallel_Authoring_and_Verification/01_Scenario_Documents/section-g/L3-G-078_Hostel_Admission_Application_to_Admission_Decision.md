# Hostel Admission Application to Admission Decision

**Scenario ID:** L3-G-078  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

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
- Approval, waitlisting, rejection, or withdrawal.
- Publication of the canonical hostel admission outcome.

### 4.4 Out of Scope

- Academic admission.
- Student lifecycle state changes.
- Room allocation and occupancy activation.
- Hostel infrastructure ownership.
- Financial accounting.
- Technical authentication, authorization enforcement, notification delivery, or implementation mechanics.

### 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-G-080 — Approved Hostel Admission to Active Residency | An approved Hostel Admission creates the business precondition for occupancy activation. |

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Hostel Management (G) | Owns Hostel Admission, hostel eligibility determination, admission decision and admission lifecycle. |
| Student Lifecycle (C) | Provides canonical student identity/context; does not own hostel admission. |
| People & Workforce (E) | Provides relevant workforce context where hostel operational roles participate; does not own hostel admissions. |
| Section H | Provides notification infrastructure for business-event-driven communications; does not own the admission outcome. |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student | Submits the hostel application where the configured admission source is student-originated. | Initiates the request. |
| Hostel Administrator | Reviews and governs hostel admission processing. | Business decision authority supported by L2-G03. |
| Hostel Warden | May participate in admission approval under configurable hostel authority. | Approval participation supported by L2-G03. |

No additional business role is invented.

## 7. Preconditions and Entry Criteria

- A valid student identity is available from Student Lifecycle.
- The application concerns institutional hostel residency.
- The admission source is one of the configured sources supported by L2-G03.
- The Hostel Management process is available to process the application.

## 8. Trigger

A hostel admission application is submitted for a student.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | A hostel admission application is submitted using an applicable configured admission source. | Student / Hostel Management | Hostel Admission enters `Applied`. |
| 2 | Hostel Management verifies canonical student identity and evaluates hostel admission criteria. | Hostel Management with Student Lifecycle context | Required eligibility facts are available. |
| 3 | Eligibility is assessed using applicable hostel rules, including student status, academic programme, campus, hostel category, gender policy, institution rules, hostel availability, and administrative approval. | Hostel Management | Application reaches a decision basis. |
| 4 | The application proceeds through the configured hostel admission review and approval path. | Hostel Administrator / Hostel Warden | A governed business decision is reached. |
| 5 | Hostel Management records Approved, Waitlisted, Rejected, or Withdrawn as applicable. | Hostel Management | Admission reaches its corresponding decision state. |
| 6 | The applicable canonical hostel admission event is published. | Hostel Management | The outcome is available to downstream business participants without ownership transfer. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | Hostel capacity is unavailable under applicable policy. | Place the application on the configured waiting list. | `Waitlisted`. | Hostel Management |
| A2 | Eligibility requirements are not satisfied. | Decline the hostel admission through the governed decision path. | `Rejected`. | Hostel Management |
| A3 | Applicant withdraws before final approved residency. | Record withdrawal under the Hostel Admission lifecycle. | `Withdrawn`. | Hostel Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required student identity or eligibility information is unavailable or invalid. | Eligibility cannot be authoritatively determined. | Resolve the information deficiency through the owning source before concluding the hostel decision. | Hostel Management / Student Lifecycle |
| Configured approval path has not completed. | Admission cannot be concluded as approved. | Keep the application in the applicable review stage until the governed decision is completed. | Hostel Management |
| No suitable room capacity exists under applicable policy. | Immediate approval cannot proceed to residency allocation. | Place the application on the waiting list when permitted by policy. | Hostel Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Hostel admission governs residency eligibility, not academic admission. | Hostel Management | May determine hostel eligibility without modifying Student Lifecycle. | L2-G03 §1, §16 |
| Eligibility may consider Student Status, Academic Programme, Campus, Hostel Category, Gender Policy, Institution Rules, Hostel Availability, and Administrative Approval. | Hostel Management | These factors constrain the decision. | L2-G03 §16 |
| Admission states include Applied, Under Review, Approved, Waitlisted, Rejected, and Withdrawn. | Hostel Management | No additional states are invented. | L2-G03 §16, §30 |
| Applications may be waitlisted when capacity is unavailable. | Hostel Management | Waiting-list placement is a governed alternate outcome. | L2-G03 §16 |
| Hostel Management does not manage Student Lifecycle. | Student Lifecycle | Student information is consumed but student lifecycle state is not changed. | L2-G03 §4, §13–§14 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Hostel Admission | Applied / Under Review | Eligibility verification and governed decision determine outcome. | Approved / Waitlisted / Rejected / Withdrawn | G03 Hostel Management | L2-G03 §16, §30 |

No Student Context lifecycle transition is owned by this scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| HostelAdmissionApplied | G03 Hostel Management | Interested downstream consumers | Records application submission. | L2-G03 §28 |
| HostelAdmissionApproved | G03 Hostel Management | Interested downstream consumers | Communicates approved outcome. | L2-G03 §28 |
| HostelAdmissionRejected | G03 Hostel Management | Interested downstream consumers | Communicates rejected outcome. | L2-G03 §28 |
| HostelAdmissionWaitlisted | G03 Hostel Management | Interested downstream consumers | Communicates waiting-list placement. | L2-G03 §28 |

The event publisher remains G03 Hostel Management.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Student Lifecycle (C) | Hostel Management (G) | Student Identity / student information | Provide canonical student context for hostel eligibility. | L2-G03 §9 |
| Hostel Management (G) | Section H / downstream consumers | Hostel admission business events | Make authoritative outcome available for notification and other downstream use. | L2-G03 §9, §27–§28 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Submit hostel admission application | Student or other configured admission source | Admission sources are configurable. | Hostel Management |
| Approve or reject hostel admission | Hostel Administrator / Hostel Warden | Admission approval is configurable within hostel policy. | Hostel Management |

### 15.2 Separation of Duties, Overrides and Exceptions

L2-G03 identifies configurable approval workflows but does not define a separate scenario-specific separation-of-duty rule. No additional rule is invented.

Administrative override is available where institutional policy allows it; exact authority is configuration dependent.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Hostel admission decisions must remain auditable. | Immutable admission history and audit information associated with the admission decision. | Hostel Management, with Section A audit infrastructure |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- A valid Hostel Admission application has a governed decision state.
- The decision is Approved, Waitlisted, Rejected, or Withdrawn.
- The resulting state is recorded in the canonical Hostel Admission lifecycle.
- The applicable canonical business event is published where defined.

### 16.2 Postconditions

- Approved applications may proceed to separate room allocation/residency processing.
- Waitlisted applications remain governed by waiting-list policy.
- Rejected or withdrawn applications do not create active hostel occupancy.
- Student Lifecycle remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issue prevents this scenario from expressing the canonical business outcome.

### 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Hostel admission approval is governed through the configurable approval path described by L2-G03 and may involve Hostel Administrator and/or Hostel Warden. | L2-G03 §31. | Exact institution-specific sequencing would need applicable configuration. |

### 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| How notification delivery is technically performed after hostel admission events. | L5 | Notification delivery and implementation architecture are outside L3. |
