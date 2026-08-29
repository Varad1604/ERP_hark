# Course Offering Suspension

**Scenario ID:** L3-D-002  
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
**Frozen Stage 3 Source:** P2W-D-0722 / CAN-L3-D-002

---

# 2. Scenario Purpose

Temporarily halt an Active Course Offering through a governed suspension while preserving its identity, history and the ownership of affected Student Academic Registration records.

# 3. Business Outcome

The Active Course Offering is placed in Suspended with its suspension decision and history preserved, and Section C is informed of the completed suspension fact for its independently owned registration consequences.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An Active Course Offering becomes subject to an authorized temporary suspension requirement.

## 4.2 End Boundary

The Course Offering reaches Suspended and `CourseOfferingSuspended` is published.

## 4.3 In Scope

- Validate the Active entry state and applicable suspension policy.
- Record the governed suspension decision and reason.
- Transition the Course Offering to Suspended.
- Make the completed suspension fact available to Section C.

## 4.4 Out of Scope

- Permanent cancellation, completion or archival.
- Reactivation of the offering.
- Modification, cancellation or reassignment of Section C-owned registrations.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns the suspension decision and Active → Suspended lifecycle transition. |
| Section C — Student Academic Registration | Owns all consequences for Student Academic Registration and consumes the suspension fact without modifying the Course Offering. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Course Offering Governance Authority | Evaluates and approves suspension under applicable policy. | Decides. |
| Authorized emergency suspending officer | Applies an emergency suspension where policy permits. | Acts subject to retrospective governance review. |

The institution designates the role fulfilling Course Offering Governance Authority; emergency action does not remove the required retrospective review.

# 7. Preconditions and Entry Criteria

- The Course Offering exists in Active.
- The suspension requirement concerns a temporary operational halt.
- The applicable suspension policy is available.

# 8. Trigger

An authorized temporary suspension requirement is raised for an Active Course Offering.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm that the Course Offering is Active and eligible for suspension. | D01 | The Active → Suspended transition is valid. |
| 2 | Evaluate the suspension requirement and retain the business reason. | Course Offering Governance Authority | The suspension is approved or rejected. |
| 3 | Apply the approved suspension, or apply an emergency suspension where policy permits. | D01 / authorized emergency suspending officer | The offering reaches Suspended with historical integrity preserved. |
| 4 | Complete retrospective review for any emergency suspension. | Course Offering Governance Authority | Emergency authority is confirmed, corrected or escalated with evidence retained. |
| 5 | Publish the completed suspension fact. | D01 | `CourseOfferingSuspended` is available to Section C and other interested domains. |
| 6 | Receive the suspension fact for independently governed registration consequences. | Section C | Section C determines consequences without changing the Course Offering. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Offering is not Active | The requested transition is invalid. | Reject the suspension request; preserve the current state. | D01 |
| Suspension policy is not satisfied | Suspension cannot be authorized. | Reject or return the requirement according to the applicable policy. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only an Active Course Offering may follow the defined suspension workflow. | D01 | Controls entry and transition validity. | L2-D01 §21 and §71 |
| Suspension is temporary and preserves historical integrity. | D01 | Prevents cancellation or archival semantics. | L2-D01 §§7,21 |
| Section C remains sole owner of registration transactions. | Section C | Prevents D01 from changing affected registrations. | L2-D01 §§4,26; L2-C03 §§5.6,13.7 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Course Offering | Active | Apply governed suspension. | Suspended | D01 | L2-D01 §21 and §71 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CourseOfferingSuspended | D01 | Section C and interested D domains | Communicates the completed suspension fact. | L2-D01 §§25,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| D01 | Section C | Course Offering suspension fact | Allow Section C to govern registration consequences. | L2-D01 §§9,21,25; L2-C03 ownership boundary |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Approve ordinary suspension | Course Offering Governance Authority | Requires an Active offering, valid reason and applicable policy satisfaction. | D01 |
| Apply emergency suspension | Authorized emergency suspending officer | Permitted only by policy and subject to retrospective review. | D01 |
| Review emergency suspension | Course Offering Governance Authority | Confirms or corrects the emergency action and retains the decision evidence. | D01 |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Suspension decision | Offering identity, prior state, suspension reason, decision and resulting Suspended state. | D01 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The offering has transitioned from Active to Suspended.
- The suspension reason and decision are retained.
- `CourseOfferingSuspended` has been published.

## 16.2 Postconditions

- The Course Offering remains owned by D01 and cannot be treated as cancelled or archived.
- Section C remains responsible for every affected registration consequence.

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
