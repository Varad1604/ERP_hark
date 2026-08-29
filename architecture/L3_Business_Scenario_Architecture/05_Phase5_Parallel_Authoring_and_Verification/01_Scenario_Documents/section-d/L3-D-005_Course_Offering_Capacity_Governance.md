# Course Offering Capacity Governance

**Scenario ID:** L3-D-005  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0725 / CAN-L3-D-005

---

# 2. Scenario Purpose

Govern an effective-dated increase, reduction or other permitted change to Course Offering capacity without invalidating confirmed Section C registrations.

# 3. Business Outcome

The Course Offering has an authorized capacity position that satisfies lifecycle, policy, confirmed-enrollment, reserved-seat and waitlist constraints, with the completed capacity fact published for Section C.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized capacity-change requirement exists for a modifiable Course Offering.

## 4.2 End Boundary

The governed capacity value becomes effective and the applicable exact capacity event is published, or the request is rejected without changing capacity.

## 4.3 In Scope

- Evaluate lifecycle and capacity policy.
- Use authoritative confirmed-registration information from Section C.
- Govern capacity increase or reduction.
- Preserve capacity history and publish the completed fact.

## 4.4 Out of Scope

- Creating or cancelling Student registrations.
- Waitlist promotion and seat resolution.
- Quota governance and physical-resource capacity ownership.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns Course Offering capacity limits and effective-dated capacity history. |
| Section C — Student Academic Registration | Owns confirmed registration information and all registration transactions. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Authorized administrator | Initiates or performs capacity modification under applicable policy. | Initiates / performs. |

The Course Offering Governance Authority decides capacity changes whenever institutional policy requires approval; otherwise an authorized administrator may apply a policy-compliant change.

# 7. Preconditions and Entry Criteria

- The Course Offering exists and its lifecycle permits capacity modification.
- Current configured capacity and reserved seats are known.
- Authoritative confirmed-registration and waitlist information is available.

# 8. Trigger

An authorized requirement is raised to change the Course Offering capacity.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm the offering lifecycle permits capacity modification. | D01 | The request is eligible for capacity evaluation. |
| 2 | Obtain authoritative confirmed-registration information from Section C and current reserved-seat/waitlist constraints. | Section C / D01 | Capacity consequences are assessed without transferring registration ownership. |
| 3 | Evaluate the proposed capacity under applicable minimum, maximum, institutional and reserved-seat policy. | D01 | A governed capacity decision is available. |
| 4 | For a reduction, confirm that no confirmed registration would be invalidated. | D01 using Section C fact | Protected registrations remain valid. |
| 5 | Apply the authorized effective-dated capacity change. | D01 | The new capacity position is authoritative and historical capacity is preserved. |
| 6 | Publish the applicable capacity fact. | D01 | `CourseOfferingCapacityIncreased`, `CourseOfferingCapacityReduced` and/or `CourseOfferingCapacityChanged` records the completed outcome. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Capacity increase | The authorized value is greater than current capacity. | Validate and apply the increase. | Increased capacity becomes effective. | D01 |
| Capacity reduction | The authorized value is lower than current capacity and confirmed registrations remain valid. | Validate and apply the reduction. | Reduced capacity becomes effective without invalidating registrations. | D01 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Lifecycle does not permit modification | Capacity cannot change. | Reject the request and preserve current capacity. | D01 |
| Proposed capacity violates policy | The value is invalid. | Reject or return the request for correction. | D01 |
| Reduction would invalidate confirmed registrations | Protected registrations would be harmed. | Reject the reduction; Section C records remain unchanged. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Capacity cannot be negative. | D01 | Rejects invalid capacity. | L2-D01 §§12,26 |
| Capacity changes must satisfy lifecycle, policy, reserved-seat, waitlist and existing-enrollment constraints. | D01 with Section C facts | Controls the capacity decision. | L2-D01 §26 |
| Capacity reductions shall never invalidate confirmed enrollments. | D01 / Section C owns registrations | Prohibits harmful reduction. | L2-D01 §30 |
| Capacity changes are effective-dated and historically auditable. | D01 | Preserves capacity history. | L2-D01 §§26,72,74 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Course Offering capacity | Current governed capacity | Apply authorized effective-dated change; lifecycle state remains unchanged. | Increased, reduced or unchanged governed capacity | D01 | L2-D01 §§26,29–30 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CourseOfferingCapacityIncreased | D01 | Section C | Records a completed increase. | L2-D01 §§42,70 |
| CourseOfferingCapacityReduced | D01 | Section C | Records a completed reduction. | L2-D01 §§42,70 |
| CourseOfferingCapacityChanged | D01 | Section C | Records a completed general capacity modification where applicable. | L2-D01 §§42,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section C | D01 | Authoritative confirmed-registration information | Validate that capacity governance does not invalidate registration. | L2-D01 §§26–30; L2-C03 ownership boundary |
| D01 | Section C | Completed Course Offering capacity fact | Support registration and seat-availability decisions. | L2-D01 §§26,42 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate capacity change | Authorized Course Offering administrator | Request must include proposed value, effective position, reason and impact basis. | D01 |
| Approve a capacity change where policy requires approval | Course Offering Governance Authority | Decision must respect configured limits, confirmed registrations and applicable capacity policy. | D01 |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Capacity decision | Previous and new governed capacity, applicable policy and completed outcome. | D01 |
| Confirmed-registration protection | Authoritative registration position used to prove no invalidation. | D01 / Section C retains source ownership. |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The request has a governed decision.
- Any effective change satisfies capacity and registration-protection rules.
- The applicable exact event is published.

## 16.2 Postconditions

- D01 owns the new capacity and preserved capacity history.
- Section C records remain unchanged by D01.
- Waitlist promotion is not executed in this scenario.

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
