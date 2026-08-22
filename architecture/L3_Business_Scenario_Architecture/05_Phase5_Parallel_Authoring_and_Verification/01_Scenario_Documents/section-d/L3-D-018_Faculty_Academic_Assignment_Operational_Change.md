# Faculty Academic Assignment Operational Change

**Scenario ID:** L3-D-018  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Faculty Academic Assignment  
**Participating Domains:** D, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D02_Faculty_Academic_Assignment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0738 / CAN-L3-D-018

---

# 2. Scenario Purpose

Apply a controlled operational change to an existing Faculty Academic Assignment after impact, policy and authority validation.

# 3. Business Outcome

The approved assignment change is applied and closed with historical accuracy preserved and the effective change fact available to affected domains.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized modification requirement exists for an operational Faculty Academic Assignment.

## 4.2 End Boundary

The Operational Change reaches Closed after application and `OperationalChangeClosed` is published.

## 4.3 In Scope

- Capture justification and impact.
- Validate assignment state, faculty facts and dependencies.
- Approve, apply and close change.

## 4.4 Out of Scope

- Direct modification of Section E faculty master.
- Rewriting prior assignment history.
- Timetable revision execution.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D02 | Owns Operational Change aggregate and changed assignment facts. |
| Section E | Owns authoritative faculty and employment facts. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized change requester | Raises justified operational change. | Initiates. |
| Authorized approver | Approves or rejects after impact validation. | Decides. |

# 7. Preconditions and Entry Criteria

- Assignment exists and permits controlled change.
- Reason and impact information are available.

# 8. Trigger

Operational, faculty-availability or policy change requires assignment modification.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create justified change request. | Authorized requester | Requested. |
| 2 | Validate lifecycle, policy, operational impact and faculty facts. | D02 / Section E facts | Validated or rejected. |
| 3 | Approve the change. | Authorized approver | Approved. |
| 4 | Apply without rewriting history. | D02 | Applied and event published. |
| 5 | Close change after confirmation. | D02 | Closed and `OperationalChangeClosed` published. |

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
| Operational Change | No request | Request, validate, approve, apply and close. | Requested → Validated → Approved → Applied → Closed | D02 | L2-D02 §§6,7.4,8.3 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| OperationalChangeRequested / OperationalChangeValidated / OperationalChangeApproved / OperationalChangeApplied / OperationalChangeClosed | D02 | Section E and interested D domains | Records exact change lifecycle. | L2-D02 §§8.3,10.4 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| E | Faculty Academic Assignment | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D02_Faculty_Academic_Assignment_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Raises justified operational change. | Authorized change requester | Initiates. | Faculty Academic Assignment |
| Approves or rejects after impact validation. | Authorized approver | Decides. | Faculty Academic Assignment |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Faculty Academic Assignment |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Faculty Academic Assignment |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The approved assignment change is applied and closed with historical accuracy preserved and the effective change fact available to affected domains.
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
