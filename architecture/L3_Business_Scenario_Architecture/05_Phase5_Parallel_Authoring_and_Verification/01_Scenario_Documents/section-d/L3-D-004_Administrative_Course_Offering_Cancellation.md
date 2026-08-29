# Administrative Course Offering Cancellation

**Scenario ID:** L3-D-004  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0724 / CAN-L3-D-004

---

# 2. Scenario Purpose

Terminate an unpublished Course Offering through an administrative cancellation decision while preserving its history and preventing future activation or enrollment.

# 3. Business Outcome

An eligible Draft or Approved Course Offering reaches Cancelled, remains auditable, cannot be activated or accept enrollment, and exposes the completed cancellation fact to Section C.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized administrative cancellation requirement exists for an unpublished Course Offering in Draft or Approved.

## 4.2 End Boundary

The offering reaches Cancelled and `CourseOfferingCancelled` is published.

## 4.3 In Scope

- Validate cancellation eligibility and policy.
- Apply the Draft/Approved → Cancelled transition.
- Preserve cancellation history and inform Section C.

## 4.4 Out of Scope

- Cancellation of Published, Active, Completed or Archived offerings.
- Temporary suspension.
- Cancellation or correction of Student Academic Registration records.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns cancellation eligibility, decision and Course Offering state. |
| Section C — Student Academic Registration | Consumes the cancellation fact and owns every registration consequence. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Course Offering Governance Authority | Approves administrative cancellation after impact assessment. | Decides. |
| Authorized Course Offering administrator | Raises the cancellation requirement and supporting reason. | Initiates. |

Administrative cancellation authority is assigned to the Course Offering Governance Authority; the institution designates the role fulfilling that authority.

# 7. Preconditions and Entry Criteria

- The Course Offering exists in Draft or Approved.
- The offering is unpublished.
- Applicable cancellation policy is available.

# 8. Trigger

An authorized administrative requirement is raised to cancel an unpublished Course Offering.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm the offering is unpublished and in Draft or Approved. | D01 | The cancellation entry state is valid. |
| 2 | Validate the requirement against applicable cancellation policy. | D01 | A governed cancellation decision can be made. |
| 3 | Apply the cancellation decision. | D01 | The offering reaches Cancelled and remains historically traceable. |
| 4 | Publish the cancellation fact. | D01 | `CourseOfferingCancelled` is available to Section C. |
| 5 | Receive the fact for independently owned registration consequences. | Section C | Section C governs any affected registration position. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Offering is Published, Active, Completed or Archived | Administrative cancellation is not supported by the defined workflow. | Reject the cancellation request and preserve the current state. | D01 |
| Cancellation policy not satisfied | Cancellation cannot be authorized. | Reject or return the request according to policy. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only unpublished Draft or Approved offerings may follow Administrative Cancellation. | D01 | Controls cancellation eligibility. | L2-D01 §24 and §71 |
| Cancelled offerings cannot be activated or accept enrollment. | D01 | Defines the terminal operational consequence. | L2-D01 §24 |
| Section C owns registration transactions. | Section C | Prevents D01 from cancelling registrations. | L2-D01 §26; L2-C03 §§5.6,13.7 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Course Offering | Draft or Approved | Administrative cancellation. | Cancelled | D01 | L2-D01 §§24,71 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CourseOfferingCancelled | D01 | Section C and interested D domains | Records completed cancellation. | L2-D01 §§25,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| D01 | Section C | Course Offering cancellation fact | Enable Section C to govern any registration consequence. | L2-D01 §§24–25; L2-C03 ownership boundary |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate administrative cancellation | Authorized Course Offering administrator | Request must identify the offering, reason and known impacts. | D01 |
| Approve or reject administrative cancellation | Course Offering Governance Authority | Requires eligible Draft or Approved state, impact assessment and recorded reason. | D01 |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Cancellation decision | Prior state, eligibility basis, cancellation decision/reason and resulting Cancelled state. | D01 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The offering was eligible for administrative cancellation.
- The offering is Cancelled.
- `CourseOfferingCancelled` is published.

## 16.2 Postconditions

- The offering cannot be activated or accept enrollment.
- Its identity and cancellation history remain preserved.
- Section C owns all affected registration consequences.

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
