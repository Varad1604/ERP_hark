# Faculty Academic Assignment Establishment

**Scenario ID:** L3-D-017  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Faculty Academic Assignment  
**Participating Domains:** D, E  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D02_Faculty_Academic_Assignment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0737 / CAN-L3-D-017

---

# 2. Scenario Purpose

Establish and activate a governed Faculty Academic Assignment against valid academic requirements and authoritative faculty identity.

# 3. Business Outcome

An approved Faculty Academic Assignment becomes Active with immutable identity, valid references, approval history and published lifecycle facts.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An approved academic operational requirement requires assignment of an eligible faculty reference.

## 4.2 End Boundary

The assignment reaches Active and `FacultyAssignmentActivated` is published.

## 4.3 In Scope

- Validate academic requirement and faculty reference.
- Create, submit, review, approve and activate assignment.
- Preserve approval and lifecycle evidence.

## 4.4 Out of Scope

- Faculty identity, employment or HR lifecycle.
- Course Offering ownership.
- Timetable creation.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D02 | Owns Faculty Assignment aggregate, approval process and lifecycle. |
| Section E | Owns faculty/employee identity and employment facts consumed as references. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized assignment initiator | Creates and submits assignment. | Initiates. |
| Authorized approver | Approves or rejects assignment under configured authority. | Decides. |

# 7. Preconditions and Entry Criteria

- Academic requirement and context exist.
- Faculty reference is authoritative and operationally eligible.
- Assignment period is valid.

# 8. Trigger

A course-delivery or academic-activity requirement requests faculty assignment.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate academic requirement, faculty reference, period and operational context. | D02 using Section E facts | Assignment basis valid. |
| 2 | Create assignment in Draft. | D02 | `FacultyAssignmentCreated` published. |
| 3 | Submit and review assignment. | Authorized initiator / D02 | Submitted → Under Review. |
| 4 | Approve or reject. | Authorized approver | Approved or Rejected with reason. |
| 5 | Activate approved assignment. | D02 | Active and `FacultyAssignmentActivated` published. |

# 10. Alternate Business Flows

The scenario follows only the source-supported alternate outcomes stated in the main flow; rejection or correction does not create the successful final outcome.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the authoritative prior state. | Faculty Academic Assignment |
| Authorization or approval failure | Requested outcome is not permitted. | Retain prior state and record the decision reason. | Faculty Academic Assignment |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity. | Faculty Academic Assignment |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change the owned business state. | Faculty Academic Assignment | Prevents cross-domain ownership leakage. | `L2-D02_Faculty_Academic_Assignment_Workflows.md` |
| Every transition or effective decision requires valid authority and business validation. | Faculty Academic Assignment | Prevents bypass of governance. | `L2-D02_Faculty_Academic_Assignment_Workflows.md` |
| Historical business facts remain immutable. | Faculty Academic Assignment | Preserves auditability and lineage. | `L2-D02_Faculty_Academic_Assignment_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Faculty Assignment | No assignment / Draft | Create, submit, review, approve and activate. | Draft → Submitted → Under Review → Approved → Active | D02 | L2-D02 §§6–9,12 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| FacultyAssignmentCreated / FacultyAssignmentSubmitted / FacultyAssignmentReviewed / FacultyAssignmentApproved / FacultyAssignmentRejected / FacultyAssignmentActivated | D02 | Section E and interested D domains | Records exact establishment lifecycle facts. | L2-D02 §§8.1,10 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| E | Faculty Academic Assignment | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D02_Faculty_Academic_Assignment_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Creates and submits assignment. | Authorized assignment initiator | Initiates. | Faculty Academic Assignment |
| Approves or rejects assignment under configured authority. | Authorized approver | Decides. | Faculty Academic Assignment |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Faculty Academic Assignment |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Faculty Academic Assignment |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An approved Faculty Academic Assignment becomes Active with immutable identity, valid references, approval history and published lifecycle facts.
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
**Validation Basis:** Section D canonical register; `L2-D02_Faculty_Academic_Assignment_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
