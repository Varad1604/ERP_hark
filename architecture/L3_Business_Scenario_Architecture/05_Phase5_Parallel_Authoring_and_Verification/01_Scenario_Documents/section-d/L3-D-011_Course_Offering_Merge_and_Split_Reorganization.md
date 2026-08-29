# Course Offering Merge and Split Reorganization

**Scenario ID:** L3-D-011  
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
**Frozen Stage 3 Source:** P2W-D-0731 / CAN-L3-D-011

---

# 2. Scenario Purpose

Govern the merger of multiple Course Offerings or the split of one Course Offering while preserving identity, lineage, capacity boundaries and cross-domain ownership.

# 3. Business Outcome

A governed merged or split Course Offering structure exists with immutable lineage; Section C receives the completed reorganization facts and exclusively governs enrollment redistribution.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized reorganization requirement identifies eligible Course Offerings for merge or split.

## 4.2 End Boundary

The approved merge or split is completed, lineage is preserved, and `CourseOfferingMerged` or `CourseOfferingSplit` is published.

## 4.3 In Scope

- Classify merge or split path.
- Validate lifecycle, capacity, enrollment impact, resources, policy and approval.
- Create the reorganized offering structure and preserve original identities.
- Publish configuration and capacity boundaries for Section C.

## 4.4 Out of Scope

- Direct redistribution or modification of Student Academic Registration.
- Deletion or rewriting of original Course Offering history.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D01 | Owns merge/split decision, reorganized Course Offering configuration, capacity boundaries and lineage. |
| Section C | Owns enrollment redistribution, registration updates and progression invariants. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized Course Offering administrator | Initiates reorganization and provides justification. | Initiates. |
| Course Offering Governance Authority | Approves or rejects merge/split after impact validation. | Decides. |

# 7. Preconditions and Entry Criteria

- All source offerings and identities exist.
- Lifecycle and policy permit reorganization.
- Enrollment and capacity impact can be evaluated.

# 8. Trigger

An administrative, campus, capacity or resource requirement requests merge or split.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Classify the requirement as merge or split. | D01 | The governed path is selected. |
| 2 | Validate lifecycle, capacity, enrollment impact, resources and policy. | D01 with Section C impact facts | The request is valid or rejected. |
| 3 | Approve the reorganization. | Course Offering Governance Authority | Authorization is recorded. |
| 4 | Create the merged offering or split successors and preserve lineage. | D01 | Reorganized D01 structure exists. |
| 5 | Publish the completed fact and new configuration/capacity boundaries. | D01 | Exact merge/split event is available. |
| 6 | Govern enrollment redistribution independently. | Section C | Registration consequences remain Section C-owned. |

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
| Course Offering | Eligible operational offering(s) | Complete merge or split and preserve predecessor/successor lineage. | Merged successor or split successors with originals historically accessible | D01 | L2-D01 §§46–48,55 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| CourseOfferingMerged / CourseOfferingSplit | D01 | Section C and interested D domains | Records completed reorganization. | L2-D01 §§56,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| C | Course & Course Offering | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D01_Course_and_Course_Offering_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Initiates reorganization and provides justification. | Authorized Course Offering administrator | Initiates. | Course & Course Offering |
| Approves or rejects merge/split after impact validation. | Course Offering Governance Authority | Decides. | Course & Course Offering |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Course & Course Offering |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Course & Course Offering |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A governed merged or split Course Offering structure exists with immutable lineage; Section C receives the completed reorganization facts and exclusively governs enrollment redistribution.
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
