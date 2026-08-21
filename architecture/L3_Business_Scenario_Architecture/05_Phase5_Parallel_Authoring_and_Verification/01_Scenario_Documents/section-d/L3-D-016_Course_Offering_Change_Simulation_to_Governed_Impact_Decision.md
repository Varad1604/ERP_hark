# Course Offering Change Simulation to Governed Impact Decision

**Scenario ID:** L3-D-016  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** C, D  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0736 / CAN-L3-D-016

---

# 2. Scenario Purpose

Evaluate the prospective impact of a significant Course Offering change without modifying transactional state and produce a governed proceed, revise or reject decision.

# 3. Business Outcome

A read-only simulation report and traceable impact decision exist; no operational state changes occur unless a separate authorized scenario is initiated.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized administrator proposes a significant change requiring impact analysis.

## 4.2 End Boundary

The Course Offering Governance Authority records Proceed, Revise or Reject against the completed simulation report.

## 4.3 In Scope

- Define simulation proposal and scope.
- Use authoritative read models for capacity, enrollment, faculty, resource and timetable impact.
- Record report and governed decision.

## 4.4 Out of Scope

- Executing the simulated change.
- Publishing a business state transition from simulation.
- Changing Section C-owned registrations.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D01 | Owns simulation request, offering-impact report and decision record. |
| Section C | Provides authoritative enrollment impact facts and retains registration ownership. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized Course Offering administrator | Initiates simulation. | Initiates. |
| Course Offering Governance Authority | Decides whether to proceed, revise or reject. | Decides. |

# 7. Preconditions and Entry Criteria

- Proposed change is identifiable.
- Required authoritative read models are available.

# 8. Trigger

A material Course Offering change is proposed.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Define proposed change, scope and decision criteria. | Authorized administrator | Simulation request exists. |
| 2 | Run read-only impact analysis using authoritative facts. | D01 with Section C read facts | Simulation report produced without state change. |
| 3 | Review operational, enrollment, faculty, resource and timetable impact. | Course Offering Governance Authority | Decision basis established. |
| 4 | Record Proceed, Revise or Reject. | Course Offering Governance Authority | Governed impact decision retained. |
| 5 | Publish simulation completion. | D01 | `CourseOfferingSimulationCompleted` records completion. |

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
| Course Offering | Current state | Read-only simulation and impact decision. | Lifecycle and transactional state unchanged | D01 | L2-D01 §65 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| CourseOfferingSimulationCompleted | D01 | Section C and interested D domains | Records completed simulation. | L2-D01 §§68,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| C | Course & Course Offering | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D01_Course_and_Course_Offering_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Initiates simulation. | Authorized Course Offering administrator | Initiates. | Course & Course Offering |
| Decides whether to proceed, revise or reject. | Course Offering Governance Authority | Decides. | Course & Course Offering |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Course & Course Offering |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Course & Course Offering |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A read-only simulation report and traceable impact decision exist; no operational state changes occur unless a separate authorized scenario is initiated.
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
