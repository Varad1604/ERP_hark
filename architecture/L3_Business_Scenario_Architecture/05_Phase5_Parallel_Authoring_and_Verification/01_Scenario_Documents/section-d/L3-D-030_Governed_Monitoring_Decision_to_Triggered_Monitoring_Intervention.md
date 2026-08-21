# Governed Monitoring Decision to Triggered Monitoring Intervention

**Scenario ID:** L3-D-030  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Monitoring  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D05_Academic_Monitoring_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0751 / CAN-L3-D-031 (final L3-D-030)

---

# 2. Scenario Purpose

Convert an approved monitoring governance decision requiring corrective action into a governed Monitoring Intervention with explicit ownership and traceable source evidence.

# 3. Business Outcome

A Monitoring Intervention is created, assigned to an Intervention Owner and formally initiated from the approved governance decision without modifying the originating monitoring or source-domain facts.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A governance decision has been evaluated and approved and its outcome requires a monitoring intervention.

## 4.2 End Boundary

The intervention reaches Assigned or In Progress and `MonitoringInterventionInitiated` is published with decision and evidence traceability.

## 4.3 In Scope

- Validate decision approval, action requirement and evidence.
- Create intervention and link it to decision, case and risks/exceptions.
- Assign Intervention Owner and initiate corrective work.

## 4.4 Out of Scope

- Automatic modification of Section C registration or D01–D04 records.
- Treating an unapproved decision as authorization.
- Completion/validation of the intervention beyond its trigger boundary.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D05 | Owns governance decision linkage, Monitoring Intervention, assignment and progress lifecycle. |
| Section C and D01–D04 | Retain ownership of any source records referenced by the intervention. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Governance Authority | Approves the decision and authorizes the controlled intervention. | Decides. |
| Monitoring Authority | Validates monitoring context and tracks the triggered action. | Requests/tracks. |
| Intervention Owner | Accepts assignment and executes corrective action. | Manages assigned intervention only. |
| D05 Application Service | Creates and initiates the intervention after authority validation. | Orchestrates; no governance decision. |

# 7. Preconditions and Entry Criteria

- Governance decision record exists and is approved.
- Decision explicitly requires corrective action.
- Evidence, policy version, authority and proposed owner are valid.

# 8. Trigger

`GovernanceDecisionApproved` records an approved decision whose outcome requires intervention.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Load the approved governance decision and linked monitoring context. | D05 Application Service | Authoritative trigger basis established. |
| 2 | Validate approval, policy, evidence, action requirement and no duplicate intervention. | D05 | Intervention is authorized or rejected. |
| 3 | Create the Monitoring Intervention linked to decision and case. | D05 | Intervention reaches Created. |
| 4 | Assign the accountable Intervention Owner. | Governance Authority / D05 | Created → Assigned. |
| 5 | Notify the owner and initiate corrective action. | D05 / Intervention Owner | Assigned → In Progress as accepted; `MonitoringInterventionInitiated`. |
| 6 | Expose progress to monitoring and governance read models. | D05 | Tracking begins without source-domain mutation. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Academic Monitoring |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Academic Monitoring |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Academic Monitoring |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Academic Monitoring | Prevents cross-domain ownership leakage. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Academic Monitoring | Prevents governance bypass. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Published or historical business facts remain immutable. | Academic Monitoring | Preserves auditability and lineage. | `L2-D05_Academic_Monitoring_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Governance Decision | Created / evaluated | Obtain required approval before intervention trigger. | Approved | D05 | L2-D05 §§13.5–13.7 |
| Monitoring Intervention | No intervention | Create, assign and initiate from approved decision. | Created → Assigned → In Progress | D05 | L2-D05 §8.10 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| GovernanceDecisionApproved | D05 | D05 intervention orchestration | Authorizes intervention where the decision outcome requires it. | L2-D05 §13.7 |
| MonitoringInterventionInitiated | D05 | Intervention Owner, Monitoring Authority and authorized cross-domain consumers | Records formally initiated corrective action. | L2-D05 §§6.2,8.10 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C and D01–D04 | Academic Monitoring | Retain ownership of any source records referenced by the intervention. | Preserve ownership while enabling the scenario. | `L2-D05_Academic_Monitoring_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Approves the decision and authorizes the controlled intervention. | Governance Authority | Decides. | Academic Monitoring |
| Validates monitoring context and tracks the triggered action. | Monitoring Authority | Requests/tracks. | Academic Monitoring |
| Accepts assignment and executes corrective action. | Intervention Owner | Manages assigned intervention only. | Academic Monitoring |
| Creates and initiates the intervention after authority validation. | D05 Application Service | Orchestrates; no governance decision. | Academic Monitoring |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Academic Monitoring |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Academic Monitoring |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Academic Monitoring |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A Monitoring Intervention is created, assigned to an Intervention Owner and formally initiated from the approved governance decision without modifying the originating monitoring or source-domain facts.
- All required authority, validation and audit evidence is retained.
- Applicable completed business facts are published exactly once in business effect.

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

No deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D05_Academic_Monitoring_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
