# Course Offering Establishment to Activation

# 1. Scenario Metadata

**Scenario ID:** L3-D-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** B, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-24  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Establish a governed operational Course Offering for an existing active Course and carry that offering through approval, publication and the final activation decision required for academic delivery.

# 3. Business Outcome

A newly established Course Offering has a preserved identity and governed lifecycle history and reaches Active through the authoritative enrollment lifecycle after successful readiness validation.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized institutional need exists to operationalize an active Section B-owned Course for a valid Academic Period and institutional context.

## 4.2 End Boundary

The D01-owned Course Offering reaches Active through Published → Enrollment Open → Enrollment Closed → Active after successful readiness validation.

## 4.3 In Scope

- Validate the authoritative Course, Programme, Department, Academic Period and institutional context.
- Create the Course Offering in Draft.
- Complete the applicable approval and publication decisions.
- Determine the activation outcome from authoritative readiness results.

## 4.4 Out of Scope

- Creation or revision of Course, Programme, Curriculum, Department, Academic Calendar or Campus masters.
- Faculty Academic Assignment, Timetable, resource allocation, Student Academic Registration and Assessment execution.
- Student-specific registration transactions occurring while enrollment is open.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns Course Offering identity, configuration, lifecycle, approval/publication facts and activation decision. |
| Section B | Provides authoritative Course, Programme, Department, Academic Period and institutional context; retains ownership of those concepts. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Department Head / Academic Office / Registrar / Dean / Institutional Approver | Participates in the configurable Course Offering approval path where selected by institutional policy. | Reviews or decides approval. |

Institutional configuration selects the named participant acting as the Course Offering Governance Authority; that authority remains accountable for the governed decisions in this scenario.

# 7. Preconditions and Entry Criteria

- The referenced Course exists and is Active.
- Programme and Department relationships are valid.
- The Academic Period exists and permits Course Offering creation.
- Required institution, campus, academic-calendar, capacity and enrollment configuration is available.

# 8. Trigger

An institutional need to establish an operational Course Offering is submitted under the applicable Course Offering governance.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm the authoritative Course, Programme, Department, Academic Period and institutional context. | Section B / D01 | The offering basis is authoritative and eligible for establishment. |
| 2 | Validate mandatory offering attributes, configuration, capacity and applicable creation rules. | D01 | The establishment proposal is valid or is rejected for correction. |
| 3 | Create the Course Offering with immutable identity in Draft. | D01 | A Draft Course Offering exists and `CourseOfferingCreated` is published. |
| 4 | Submit the complete Draft into the applicable approval path. | D01 / Course Offering Governance Authority | The offering moves to Pending Approval and `CourseOfferingSubmittedForApproval` is published. |
| 5 | Determine the approval outcome. | Course Offering Governance Authority / D01 | Approval produces Approved and `CourseOfferingApproved`; rejection returns the offering to Draft and publishes `CourseOfferingRejected`. |
| 6 | Publish the approved and complete Course Offering. | Course Offering Governance Authority / D01 | The offering becomes Published and `CourseOfferingPublished` is published. |
| 7 | Open and close enrollment through the governed enrollment lifecycle. | D01 | The offering progresses Published → Enrollment Open → Enrollment Closed. |
| 8 | Obtain authoritative readiness results and determine activation. | Course Offering Governance Authority / D01 | Successful readiness transitions Enrollment Closed → Active and publishes `CourseOfferingActivated`; failure retains Enrollment Closed and publishes `CourseOfferingActivationFailed`. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Configurable approval not required | Institutional policy does not require approval before publication. | Use the source-supported policy route to publication without inventing an approval decision. | Published offering eligible for activation assessment. | D01 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Invalid academic or institutional reference | Establishment cannot proceed. | Reject the proposal for correction; no Draft offering is created. | D01 |
| Approval rejected | The offering cannot be published. | Return the offering to Draft with comments and retained approval history. | D01 |
| Publication prerequisites not satisfied | The offering cannot become discoverable. | Keep the offering outside Published until prerequisites are corrected. | D01 |
| Readiness validation fails | The offering cannot become Active. | Reject activation, retain the lifecycle state and record the validation report. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A Course Offering references exactly one Course and belongs to one Academic Period. | B04/B05 for references; D01 for offering | Constrains establishment identity and context. | L2-D01 §12; L2-B04 §2.1; L2-B05 Academic Time Unit model |
| Published offerings require successful approval where approval is required by institutional policy. | D01 | Prevents unauthorized publication. | L2-D01 §§12,17–18 |
| Creation does not imply publication, enrollment availability or readiness. | D01 | Keeps establishment stages distinct. | L2-D01 §15 |
| Activation requires successful readiness validation. | D01 | Controls the final activation decision. | L2-D01 §20 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Course Offering | No Course Offering / Draft proposal | Create, approve, publish, govern enrollment availability and activate. | Draft → Pending Approval → Approved → Published → Enrollment Open → Enrollment Closed → Active | D01 | L2-D01 §§15–20 and §71; accepted Batch 1 decision 1 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CourseOfferingCreated | D01 | Interested D domains | Records Draft establishment. | L2-D01 §§25,70 |
| CourseOfferingSubmittedForApproval | D01 | Approval participants | Records approval submission. | L2-D01 §§25,70 |
| CourseOfferingApproved / CourseOfferingRejected | D01 | Interested D domains | Records approval decision. | L2-D01 §§25,70 |
| CourseOfferingPublished | D01 | Downstream academic consumers | Records discoverable publication. | L2-D01 §§25,70 |
| CourseOfferingActivated / CourseOfferingActivationFailed | D01 | Downstream academic consumers | Records the final activation decision. | L2-D01 §§25,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section B | D01 | Authoritative Course, Programme, Department and Academic Period context | Validate the establishment basis. | L2-D01 §§9,15; L2-B04 §2.1; L2-B05 |
| D-owned readiness contributors | D01 | Faculty and Timetable readiness results | Support the Course Offering activation decision. | L2-D01 §20 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate Course Offering establishment | Institutionally authorized Course Offering administrator | Creation requires authorization under Course Offering governance. | D01 |
| Approve or reject Course Offering | Course Offering Governance Authority | The institution-configured Department Head, Academic Office, Registrar, Dean or Institutional Approver fulfils this authority. | D01 |
| Publish, govern enrollment lifecycle and activate Course Offering | Course Offering Governance Authority | Publication and activation require authorization and applicable validation. | D01 |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Creation and identity establishment | Draft offering identity, authoritative references and creation outcome. | D01 |
| Approval and publication | Submission, decision, comments where rejected, and publication outcome. | D01 |
| Activation decision | Readiness result, activation outcome and retained lifecycle history. | D01 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A Course Offering with immutable identity exists.
- Its approval, publication and enrollment-lifecycle outcomes are governed.
- The offering is Active following successful readiness validation.

## 16.2 Postconditions

- Section B continues to own the referenced academic masters.
- D01 remains the sole owner of the Course Offering lifecycle.
- Downstream domains may consume the exact published lifecycle facts without modifying the Course Offering.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
