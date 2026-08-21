# Closed Monitoring Case Reopening to Reinitialized Monitoring Cycle

**Scenario ID:** L3-D-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Monitoring  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D05_Academic_Monitoring_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0754 / CAN-L3-D-034 (final L3-D-033)

---

# 2. Scenario Purpose

Reopen a Closed Academic Monitoring Case through governance approval by creating a new lifecycle and evaluation context rather than rolling back or altering the original closure.

# 3. Business Outcome

An approved reopening creates a separately identified monitoring lifecycle with current policy, a new plan and evaluation context, while the original Closed lifecycle remains immutable.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

New evidence, policy change, compliance requirement, governance decision or corrected interpretation justifies reopening a Closed monitoring case.

## 4.2 End Boundary

A new monitoring cycle is initialized and activated, and `NewMonitoringCycleStarted` is published with traceability to the original closure.

## 4.3 In Scope

- Request reopening with reason and original-closure reference.
- Assess impact and approve or reject reopening.
- Create new identifiers, context, current policy version and monitoring plan.
- Activate the new cycle.

## 4.4 Out of Scope

- Rollback of the Closed case.
- Modification of historical decisions, snapshots or evidence.
- Reuse of the old evaluation context as current truth.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D05 | Owns reopening request, approval, new lifecycle identity, context, plan and activation. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Monitoring Authority | Requests reopening and supplies new evidence or reason. | Initiates. |
| Governance Reviewer | Assesses reason, evidence and impact. | Reviews. |
| Governance Authority | Approves or rejects reopening. | Final reopening authority. |

# 7. Preconditions and Entry Criteria

- Original case is Closed and its closure record is available.
- Reopening reason and impact evidence are identifiable.
- Current applicable policy can be assigned.

# 8. Trigger

A governed reopening condition is identified for a Closed monitoring case.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create reopening request referencing original closure. | Monitoring Authority / D05 | MonitoringReopeningRequested. |
| 2 | Evaluate reason, evidence, compliance and impact. | Governance Reviewer | Review evidence retained. |
| 3 | Approve or reject reopening. | Governance Authority | MonitoringReopeningApproved or prior Closed state retained. |
| 4 | For approval, create a new lifecycle identity and governance context. | D05 | MonitoringLifecycleReopened; original remains immutable. |
| 5 | Assign current policy, create new plan and evaluation context. | D05 | ReopenedMonitoringInitialized. |
| 6 | Activate the new monitoring cycle. | D05 / Monitoring Authority | NewMonitoringCycleStarted. |

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
| Original Academic Monitoring Case | Closed | Reference only; never roll back or modify. | Remains Closed and immutable | D05 | L2-D05 §§14.8–14.11 |
| Reopened Monitoring Lifecycle | No new lifecycle | Create separate identity, context, plan and active cycle. | Created → Initialized → Active | D05 | L2-D05 §§14.8–14.10 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| MonitoringReopeningRequested / MonitoringReopeningApproved / MonitoringLifecycleReopened / ReopenedMonitoringInitialized / NewMonitoringCycleStarted | D05 | Authorized monitoring and governance consumers | Records governed reopening and distinct new monitoring cycle. | L2-D05 §§14.8–14.10 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates; completed events may still be consumed under canonical contracts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Requests reopening and supplies new evidence or reason. | Monitoring Authority | Initiates. | Academic Monitoring |
| Assesses reason, evidence and impact. | Governance Reviewer | Reviews. | Academic Monitoring |
| Approves or rejects reopening. | Governance Authority | Final reopening authority. | Academic Monitoring |

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

- An approved reopening creates a separately identified monitoring lifecycle with current policy, a new plan and evaluation context, while the original Closed lifecycle remains immutable.
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
