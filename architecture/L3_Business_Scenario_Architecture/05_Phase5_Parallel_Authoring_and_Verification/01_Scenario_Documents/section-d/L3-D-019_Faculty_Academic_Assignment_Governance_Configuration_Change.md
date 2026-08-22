# Faculty Academic Assignment Governance Configuration Change

**Scenario ID:** L3-D-019  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Faculty Academic Assignment  
**Participating Domains:** D  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D02_Faculty_Academic_Assignment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0739 / CAN-L3-D-019

---

# 2. Scenario Purpose

Govern a change to Faculty Academic Assignment rules, approval routing or operational policy configuration after impact and architecture review.

# 3. Business Outcome

An approved versioned governance configuration becomes Effective, the previous version becomes Superseded, historical assignment decisions remain unchanged, and the completed change event is published.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A justified requirement proposes a change to D02 assignment governance configuration.

## 4.2 End Boundary

The approved configuration reaches Effective, the prior configuration is Superseded, and `FacultyAssignmentGovernanceConfigurationChanged` is published.

## 4.3 In Scope

- Classify proposed rule, approval or policy change.
- Perform impact and architecture review.
- Approve, validate and make the configuration effective.

## 4.4 Out of Scope

- Changing existing historical assignments automatically.
- Technical deployment design.
- Section E employment policy ownership.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D02 | Owns assignment operational rules, approval enforcement and governance configuration. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| D02 governance administrator | Initiates configuration change. | Initiates. |
| D02 Governance Authority | Approves or rejects after architecture and impact review. | Decides. |

# 7. Preconditions and Entry Criteria

- Current configuration and proposed change are identifiable.
- Impact can be evaluated.

# 8. Trigger

A policy, compliance or operational requirement requests governance configuration change.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create a Draft configuration version with rationale and intended effective position. | D02 governance administrator | Draft version exists. |
| 2 | Perform impact and architecture review. | D02 Governance Authority | Configuration reaches Under Review with compatibility and ownership evidence. |
| 3 | Approve or reject the reviewed configuration. | D02 Governance Authority | Approved version or retained Draft/rejected outcome. |
| 4 | Validate and make the approved configuration Effective. | D02 | Approved → Effective; prior Effective version becomes Superseded. |
| 5 | Publish the completed configuration change and preserve history. | D02 | `FacultyAssignmentGovernanceConfigurationChanged` is published without retroactive alteration. |

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
| D02 governance configuration | Draft / current Effective version | Review and approve a new configuration version, then make it effective while superseding the prior version. | Draft → Under Review → Approved → Effective; prior Effective → Superseded | D02 | L2-D02 §§5.4,9.3–9.4,9.11; accepted L3-D-019 decision (2026-08-20) |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| FacultyAssignmentGovernanceConfigurationChanged | D02 | Interested D domains | Records the new Effective governance configuration and superseded predecessor. | Accepted L3-D-019 decision (2026-08-20) |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Initiates configuration change. | D02 governance administrator | Initiates. | Faculty Academic Assignment |
| Approves or rejects after architecture and impact review. | D02 Governance Authority | Decides. | Faculty Academic Assignment |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Faculty Academic Assignment |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Faculty Academic Assignment |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An approved versioned governance configuration becomes Effective, the previous version becomes Superseded, historical assignment decisions remain unchanged, and the completed change event is published.
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
