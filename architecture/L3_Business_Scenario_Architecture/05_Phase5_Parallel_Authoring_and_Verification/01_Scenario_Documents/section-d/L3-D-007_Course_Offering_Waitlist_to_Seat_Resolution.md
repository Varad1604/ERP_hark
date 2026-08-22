# Course Offering Waitlist to Seat Resolution

**Scenario ID:** L3-D-007  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0727 / CAN-L3-D-007

---

# 2. Scenario Purpose

Resolve waitlisted eligible demand when a governed Course Offering seat becomes available, while leaving the Student Academic Registration transaction with Section C.

# 3. Business Outcome

A waitlisted Student demand receives a governed promotion/seat-offer outcome under institutional waitlist policy, and Section C receives that outcome to govern registration.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A governed seat becomes available for a Course Offering that has an ordered waitlist.

## 4.2 End Boundary

The applicable waitlisted demand is selected under authoritative policy and the promotion/seat-offer fact is published for Section C, or no eligible promotion occurs.

## 4.3 In Scope

- Confirm authoritative seat availability.
- Apply the configured waitlist promotion policy.
- Select eligible waitlisted demand.
- Publish the promotion outcome for Section C.

## 4.4 Out of Scope

- Creation of the Section C registration.
- Inventing or changing waitlist ranking policy.
- Course Offering capacity governance.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns waitlist governance, seat limits and promotion outcome. |
| Section C — Student Academic Registration | Owns Student eligibility/registration and decides the resulting registration transaction. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Authorized administrator | May perform supported manual waitlist administration or trigger promotion. | Performs / triggers where policy permits. |

Promotion ordering is resolved by the authoritative institution-configured waitlist policy; the scenario does not substitute a local FIFO, merit or priority rule.

# 7. Preconditions and Entry Criteria

- A Course Offering waitlist exists.
- A seat is authoritatively available.
- Waitlisted demand and applicable promotion policy are available.

# 8. Trigger

A governed Course Offering seat becomes available while eligible demand remains waitlisted.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm authoritative seat availability from configured capacity, reserved seats and Section C registration facts. | D01 / Section C | A seat is available for governed resolution. |
| 2 | Apply the authoritative waitlist promotion rule. | D01 | The candidate demand is selected according to policy. |
| 3 | Confirm the selected demand remains eligible through Section C-owned eligibility context. | Section C | The candidate is eligible or must remain unresolved. |
| 4 | Record the governed promotion/seat-offer outcome. | D01 | The waitlist position is resolved for the selected demand. |
| 5 | Publish the completed promotion fact for registration handling. | D01 | `WaitlistStudentPromoted` is available to Section C. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Manual promotion | Institutional policy permits an authorized administrator to trigger or select promotion. | Apply the supported manual path under the same governing policy. | Governed promotion outcome. | D01 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| No authoritative seat is available | Promotion cannot proceed. | Retain the waitlist position. | D01 |
| Selected demand is no longer eligible | The seat cannot be offered to that demand. | Apply the authoritative promotion policy to the next eligible demand or retain unresolved position; exact ordering remains policy-owned. | D01 / Section C |
| Promotion rule is undefined | Selection would be arbitrary. | Stop and refer the policy gap; do not invent ranking. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A Student is waitlisted only when enrollment cannot be confirmed under active capacity and policy. | D01 / Section C registration ownership | Controls waitlist entry context. | L2-D01 §12 |
| Waitlists are ordered and promotion must respect institutional policy. | D01 | Controls selection. | L2-D01 §§34–35 |
| Registration ownership remains with Section C. | Section C | Prevents promotion from being described as registration completion. | L2-D01 §34; L2-C03 §§5.6,13.7 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Waitlist demand | Waitlisted | Select and promote under policy. | Promoted / seat offered, or remains Waitlisted | D01 | L2-D01 §§27,34–35 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| WaitlistPromotionTriggered | D01 | Section C | Records initiation of promotion where materially used. | L2-D01 §§42,70 |
| WaitlistStudentPromoted | D01 | Section C | Records the completed promotion outcome. | L2-D01 §§42,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section C | D01 | Authoritative registration and Student-eligibility facts | Confirm seat availability and continuing eligibility. | L2-D01 §§27,34–35; L2-C03 |
| D01 | Section C | Governed waitlist promotion / seat-offer outcome | Allow Section C to govern registration. | L2-D01 §§35,42 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Manual waitlist administration or promotion trigger | Authorized administrator | Permitted only under institutional waitlist policy. | D01 |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Waitlist promotion | Seat-availability basis, applicable promotion policy, selected demand and promotion outcome. | D01 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Seat availability was confirmed.
- Selection followed authoritative policy.
- The promotion outcome is recorded and published for Section C.

## 16.2 Postconditions

- D01 owns the waitlist outcome.
- Section C owns any resulting registration transaction.

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
