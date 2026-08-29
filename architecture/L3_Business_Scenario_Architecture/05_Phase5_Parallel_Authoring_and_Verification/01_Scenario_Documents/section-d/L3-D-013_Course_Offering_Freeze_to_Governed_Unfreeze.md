# Course Offering Freeze to Governed Unfreeze

**Scenario ID:** L3-D-013  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0733 / CAN-L3-D-013

---

# 2. Scenario Purpose

Temporarily protect a Course Offering from operational modification and later restore its prior lifecycle position through governed unfreeze.

# 3. Business Outcome

The offering is frozen with its prior lifecycle position preserved and later returns to that exact position after authorized validation, with complete audit history.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized audit, accreditation, regulatory or administrative requirement requires a Course Offering freeze.

## 4.2 End Boundary

The freeze is removed after authorization and the offering returns to its preserved prior lifecycle state.

## 4.3 In Scope

- Validate freeze reason and authority.
- Apply the Frozen protection overlay.
- Reject prohibited changes while frozen.
- Validate and apply unfreeze to the preserved prior state.

## 4.4 Out of Scope

- Cancellation, archive or rollback.
- Modification of the preserved prior lifecycle state while frozen.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D01 | Owns freeze status, protected offering, prior-state reference and unfreeze decision. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized administrator | Requests or applies governed freeze/unfreeze. | Initiates / performs. |
| Course Offering Governance Authority | Approves freeze or unfreeze where policy requires approval. | Decides. |

# 7. Preconditions and Entry Criteria

- Offering exists and is not Archived.
- Freeze reason and prior lifecycle position can be recorded.

# 8. Trigger

A governed protection requirement is raised.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate offering, reason, policy and authority. | D01 | Freeze decision is supported. |
| 2 | Record prior lifecycle position and freeze the offering. | D01 | `CourseOfferingFrozen` records protection. |
| 3 | Reject prohibited modifications while retaining read access. | D01 | Historical and operational integrity protected. |
| 4 | Evaluate authorized unfreeze request. | Course Offering Governance Authority | Unfreeze approved or rejected. |
| 5 | Restore the preserved prior lifecycle position. | D01 | `CourseOfferingUnfrozen` records restoration. |

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
| Course Offering | Any eligible non-Archived lifecycle position | Apply Frozen protection then restore preserved position. | Frozen → preserved prior lifecycle state | D01 | L2-D01 §§49–50 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| CourseOfferingFrozen / CourseOfferingUnfrozen | D01 | Interested D domains | Records freeze and governed removal. | L2-D01 §§56,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Requests or applies governed freeze/unfreeze. | Authorized administrator | Initiates / performs. | Course & Course Offering |
| Approves freeze or unfreeze where policy requires approval. | Course Offering Governance Authority | Decides. | Course & Course Offering |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Course & Course Offering |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Course & Course Offering |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The offering is frozen with its prior lifecycle position preserved and later returns to that exact position after authorized validation, with complete audit history.
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
