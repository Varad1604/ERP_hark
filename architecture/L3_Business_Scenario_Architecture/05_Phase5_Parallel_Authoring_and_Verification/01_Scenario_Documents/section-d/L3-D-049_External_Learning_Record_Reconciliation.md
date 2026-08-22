# External Learning Record Reconciliation

**Scenario ID:** L3-D-049  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0769 / CAN-L3-D-049

---

# 2. Scenario Purpose

Reconcile discrepancies between canonical institutional learning records and external-provider records while preserving institutional ownership and complete evidence.

# 3. Business Outcome

Each discrepancy has a governed reconciliation outcome, institutional truth remains authoritative, and `LearningRecordsReconciled` is published without destructive overwrite.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A comparison or synchronization process detects a discrepancy between institutional and provider learning records.

## 4.2 End Boundary

The reconciliation outcome is recorded with evidence, authority and any required recovery/escalation, and canonical institutional records remain consistent.

## 4.3 In Scope

- Compare institutional and provider identifiers, status, progress, assessment and completion facts.
- Classify discrepancies and apply approved reconciliation policy.
- Record correction/reference outcome with immutable evidence.
- Escalate identity, fraud or unresolved historical inconsistency.

## 4.4 Out of Scope

- Treating provider data as automatically canonical.
- Overwriting institutional history.
- Changing provider records without provider-governed action.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns institutional external-learning registration, progress, completion and reconciliation outcome. |
| External Learning Provider | Owns provider-side records supplied as comparison evidence only. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Integration Administrator | Initiates/reviews reconciliation and provider evidence. | Operational reconciliation authority. |
| Academic Affairs | Approves academic interpretation where discrepancy affects completion or recognition. | Academic decision authority. |
| D06 Reconciliation Service | Compares records and applies deterministic approved rules. | Automated execution only. |

# 7. Preconditions and Entry Criteria

- Comparison report and correlated institutional/provider identifiers exist.
- Applicable reconciliation policy and authoritative institutional version are available.

# 8. Trigger

Synchronization, audit or manual review detects a learning-record discrepancy.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create reconciliation case with comparison evidence. | D06 / Integration Administrator | Discrepancy is traceable. |
| 2 | Compare registration, progress, assessment, completion and identity facts. | D06 Reconciliation Service | Differences classified. |
| 3 | Confirm institutional canonical record and applicable policy/version. | D06 / Academic Affairs | Decision basis established. |
| 4 | Apply permitted reconciliation or request provider-side correction. | Authorized owner | No destructive historical overwrite. |
| 5 | Escalate identity, fraud or unresolved inconsistency where required. | Academic Affairs / Compliance | Governed exception path starts. |
| 6 | Validate consistency and record final evidence/outcome. | D06 | LearningRecordsReconciled. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Open Electives & MOOCs |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Open Electives & MOOCs |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Open Electives & MOOCs |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Open Electives & MOOCs | Prevents cross-domain ownership leakage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Open Electives & MOOCs | Prevents governance bypass. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Published or historical business facts remain immutable. | Open Electives & MOOCs | Preserves auditability and lineage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Learning Reconciliation Case | Discrepancy Detected | Compare, decide, reconcile and validate institutional consistency. | Resolved / Escalated with immutable evidence | D06 | L2-D06 W70–W71 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| ProviderSynchronizationRecovered / ProviderSynchronizationRecoveryFailed / LearningRecordsReconciled | D06 | Integration, academic governance and audit consumers | Records recovery and governed reconciliation outcome. | L2-D06 W70–W71; D06-E61/E62 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| External Learning Provider | Open Electives & MOOCs | Owns provider-side records supplied as comparison evidence only. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Initiates/reviews reconciliation and provider evidence. | Integration Administrator | Operational reconciliation authority. | Open Electives & MOOCs |
| Approves academic interpretation where discrepancy affects completion or recognition. | Academic Affairs | Academic decision authority. | Open Electives & MOOCs |
| Compares records and applies deterministic approved rules. | D06 Reconciliation Service | Automated execution only. | Open Electives & MOOCs |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Open Electives & MOOCs |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Open Electives & MOOCs |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Open Electives & MOOCs |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Each discrepancy has a governed reconciliation outcome, institutional truth remains authoritative, and `LearningRecordsReconciled` is published without destructive overwrite.
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
**Validation Basis:** Section D canonical register; `L2-D06_Open_Electives_and_MOOCs_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
