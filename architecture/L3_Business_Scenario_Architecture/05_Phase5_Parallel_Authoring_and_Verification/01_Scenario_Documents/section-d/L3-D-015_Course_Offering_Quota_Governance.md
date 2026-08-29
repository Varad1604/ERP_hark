# Course Offering Quota Governance

**Scenario ID:** L3-D-015  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0735 / CAN-L3-D-015

---

# 2. Scenario Purpose

Govern Course Offering enrollment quotas independently from capacity and provide authoritative quota availability to Section C.

# 3. Business Outcome

An approved effective quota configuration exists, remains consistent with capacity policy, and is available for Section C-owned registration decisions.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized quota creation or change requirement exists for a Course Offering.

## 4.2 End Boundary

The governed quota configuration becomes effective and `CourseOfferingQuotaUpdated` is published.

## 4.3 In Scope

- Define quota categories and values.
- Validate policy, capacity relationship and effective period.
- Approve and publish the effective configuration.

## 4.4 Out of Scope

- Student registration confirmation.
- Capacity ownership transfer to Section C.
- Student-category master ownership.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D01 | Owns Course Offering quota configuration and quota availability. |
| Section C | Consumes quota facts and owns each registration transaction. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized Course Offering administrator | Initiates quota configuration. | Initiates. |
| Course Offering Governance Authority | Approves quota configuration or material change. | Decides. |

# 7. Preconditions and Entry Criteria

- Course Offering and applicable quota categories exist.
- Capacity and policy facts are available.

# 8. Trigger

A policy or operational requirement requests quota establishment or change.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Capture quota categories, values, period and rationale. | D01 | Quota proposal exists. |
| 2 | Validate independence from capacity and applicable limits/policy. | D01 | Proposal is valid or rejected. |
| 3 | Approve the quota configuration. | Course Offering Governance Authority | Decision recorded. |
| 4 | Make configuration effective and publish update. | D01 | `CourseOfferingQuotaUpdated` published. |
| 5 | Use effective quota during registration governance. | Section C | Individual transaction remains Section C-owned. |

# 10. Alternate Business Flows

The scenario follows only the source-supported alternate outcomes stated in the main flow; rejection or correction does not create the successful final outcome.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the authoritative prior state. | Course & Course Offering |
| Authorization or approval failure | Requested outcome is not permitted. | Retain prior state and record the decision reason. | Course & Course Offering |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity. | Course & Course Offering |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change the owned business state. | Course & Course Offering | Prevents cross-domain ownership leakage. | `L2-D01_Course_and_Course_Offering_Workflows.md` |
| Every transition or effective decision requires valid authority and business validation. | Course & Course Offering | Prevents bypass of governance. | `L2-D01_Course_and_Course_Offering_Workflows.md` |
| Historical business facts remain immutable. | Course & Course Offering | Preserves auditability and lineage. | `L2-D01_Course_and_Course_Offering_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Course Offering quota configuration | Absent / current effective configuration | Approve new effective quota version. | Effective governed quota configuration | D01 | L2-D01 §37 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| CourseOfferingQuotaUpdated | D01 | Section C | Records effective quota change. | L2-D01 §§42,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| C | Course & Course Offering | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D01_Course_and_Course_Offering_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Initiates quota configuration. | Authorized Course Offering administrator | Initiates. | Course & Course Offering |
| Approves quota configuration or material change. | Course Offering Governance Authority | Decides. | Course & Course Offering |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Course & Course Offering |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Course & Course Offering |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An approved effective quota configuration exists, remains consistent with capacity policy, and is available for Section C-owned registration decisions.
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
**Validation Basis:** Section D canonical register; `L2-D01_Course_and_Course_Offering_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
