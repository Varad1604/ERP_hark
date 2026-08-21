# Course Offering Reactivation

**Scenario ID:** L3-D-003  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0723 / CAN-L3-D-003

---

# 2. Scenario Purpose

Restore a Suspended Course Offering to operational availability after the suspension reason is resolved and required readiness and policy validations succeed.

# 3. Business Outcome

The Suspended Course Offering returns to Active with its prior suspension history preserved and `CourseOfferingReactivated` recorded as the completed business fact.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A Course Offering is Suspended and the suspension reason has been reported as resolved.

## 4.2 End Boundary

The offering reaches Active and `CourseOfferingReactivated` is published.

## 4.3 In Scope

- Confirm resolution of the suspension reason.
- Validate policy and readiness for restoration.
- Transition Suspended → Active.

## 4.4 Out of Scope

- Creation, publication, revision or archival.
- Restoration of Cancelled, Completed or Archived offerings.
- Downstream registration or timetable changes.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns readiness assessment input, reactivation decision and Suspended → Active transition. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Course Offering Governance Authority | Approves reactivation after resolution and readiness are confirmed. | Decides. |
| Authorized Course Offering administrator | Submits the reactivation requirement and supporting evidence. | Initiates. |

The institution designates the named role fulfilling Course Offering Governance Authority.

# 7. Preconditions and Entry Criteria

- The Course Offering exists in Suspended.
- The suspension reason has been resolved.
- Applicable reactivation policy and readiness evidence are available.

# 8. Trigger

Resolution of the suspension reason creates an authorized requirement to return the Course Offering to operation.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm the Course Offering is Suspended and the recorded suspension reason is resolved. | D01 | The offering is eligible for reactivation assessment. |
| 2 | Evaluate applicable policy and authoritative readiness results. | D01 | The reactivation decision has a valid basis. |
| 3 | Approve and apply reactivation when all conditions are satisfied. | D01 | The offering transitions from Suspended to Active. |
| 4 | Publish the completed reactivation fact. | D01 | `CourseOfferingReactivated` records restoration to Active. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Suspension reason remains unresolved | Reactivation cannot proceed. | Retain Suspended until authoritative resolution exists. | D01 |
| Policy or readiness validation fails | The offering cannot resume operation. | Reject reactivation and retain Suspended with validation evidence. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Reactivation requires the suspension reason to be resolved. | D01 | Controls entry to the decision. | L2-D01 §22 |
| Policy and readiness validation must succeed. | D01 | Prevents unsafe restoration. | L2-D01 §22 |
| Suspended may transition to Active. | D01 | Defines the lifecycle effect. | L2-D01 §71 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Course Offering | Suspended | Governed reactivation. | Active | D01 | L2-D01 §§22,71 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CourseOfferingReactivated | D01 | Interested D domains | Records completed restoration to Active. | L2-D01 §§25,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain business contract or hand-off materially participates in this scenario.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate reactivation | Authorized Course Offering administrator | Must provide resolution and readiness evidence. | D01 |
| Approve or reject reactivation | Course Offering Governance Authority | Requires resolved suspension reason plus successful policy and readiness validation. | D01 |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Reactivation decision | Suspension resolution, validation results, decision and resulting Active state. | D01 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The suspension reason is resolved.
- Policy and readiness validation succeeded.
- The offering is Active and `CourseOfferingReactivated` is published.

## 16.2 Postconditions

- Suspension history remains preserved.
- The offering resumes operational availability without creating a new identity.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** `Section_D_Canonical_Register.md` and `L2-D01_Course_and_Course_Offering_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500; accepted Batch 1 resolution decisions 1–7 (2026-08-20).
