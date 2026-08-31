# Workflow Execution Failure to Controlled Recovery Outcome

## 1. Specification Metadata

**Specification ID:** L4-A-016  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Workflow Engine  
**Realizes Scenario:** L3-A-016  
**Primary Business Owner:** Workflow Engine  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-A-016_Workflow_Execution_Failure_to_Controlled_Recovery_Outcome.md`; `L2-A05_Workflow_Engine_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A06_Event_Infrastructure_Workflows.md`; `L2-A07_Audit_Infrastructure_Workflows.md`; `L3-A-015`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall expose controlled business-visible recovery for a failed Workflow Instance while preserving its Tenant, originating immutable Workflow Version, correlation, completed domain outcomes and canonical lifecycle. L4 specifies observable recovery outcomes only; persistence, technical retry, rollback and infrastructure mechanisms remain downstream design.

## 3. Required Business Context

- One failed Workflow Instance in one Tenant and its originating immutable Workflow Version.
- Failure classification and current canonical instance state.
- Correlation and completed domain-owned outcomes.
- Applicable recovery, authorization and cancellation policies.
- Source-defined Workflow Instance transitions and workflow events.
- Authoritative business outcome from the Workflow-Owning Domain where needed.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-016-BH-001 | When an active Workflow Instance can no longer continue, the system shall classify the recognized failure without transferring ownership of domain-specific business failures. | L3-A-016 §§8–12 |
| L4-A-016-BH-002 | When the source-defined failure transition succeeds, the system shall place the instance in `Failed` and publish `WorkflowFailed`. | L3-A-016 §§9, 13–14 |
| L4-A-016-BH-003 | The system shall preserve the Tenant, instance identity, originating Workflow Version, correlation and completed authoritative domain outcomes throughout recovery. | L3-A-016 §§4, 7, 9, 12–13 |
| L4-A-016-BH-004 | The system shall prevent recovery progression when recovery is not permitted by the originating version, applicable policy, failure category or current lifecycle state. | L3-A-016 §§9, 11–12 |
| L4-A-016-BH-005 | The system shall prevent `Failed → Recovering` until the required recovery authorization outcome exists. | L3-A-016 §§9, 13, 15 |
| L4-A-016-BH-006 | While `Recovering`, the system shall require an authoritative valid continuation position and shall not rewrite completed domain outcomes or silently change the originating version. | L3-A-016 §§9–13 |
| L4-A-016-BH-007 | When governed recovery succeeds, the system shall transition `Recovering → Running` and publish only the source-supported recovery/resumption events whose transition conditions are satisfied. | L3-A-016 §§9, 13–14 |
| L4-A-016-BH-008 | When governed recovery cannot resume and authorized closure exists, the system shall transition `Recovering → Cancelled` and publish `WorkflowCancelled`. | L3-A-016 §§9, 11, 13–14 |
| L4-A-016-BH-009 | The system shall never resume a `Completed` or `Cancelled` instance through A016. | L3-A-016 §§4, 12–13 |

## 5. Cross-Domain & Event Behavior

The Workflow Engine remains sole publisher of `WorkflowFailed`, `WorkflowCheckpointCreated`, `WorkflowRecovered`, `WorkflowResumed` and `WorkflowCancelled` under their source-defined conditions. The Workflow-Owning Domain remains authoritative for whether its business action completed, failed or may be performed again. No recovery event may overwrite a domain result.

## 6. Failure / Rejection / Continuation

Unclassified failure, unresolved Tenant/version context, denied recovery, absent valid continuation position, ambiguous participating-domain outcome or failed recovery shall prevent a return to `Running`. If cancellation authority is also unavailable, the system shall preserve the unresolved canonical lifecycle position rather than invent a new state or approver.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-016-BH-010 | The system shall preserve attributable evidence of failure category, Tenant, instance/version/correlation, prior domain outcomes, recovery eligibility, authority decision, lifecycle transitions, recovery/resumption or cancellation events, and final status. | L3-A-016 §15.3–16 |

## 8. Observable Completion Conditions

Completion is observable when the failed instance has either returned from `Recovering` to `Running` under governed authorization with the applicable canonical recovery/resumption events, or has reached authorized `Cancelled` with `WorkflowCancelled`. The originating version and prior domain outcomes remain preserved.

## 9. Traceability & Provenance

**Parent:** L3-A-016 — Workflow Execution Failure to Controlled Recovery Outcome  
**Primary verification source:** L2-A05 — Workflow Engine Workflows  
**Additional verification:** L2-A04 — Authorization & Policy; L2-A06 — Event Infrastructure; L2-A07 — Audit Infrastructure; L3-A-015 execution boundary

## 10. Open Issues / Deferred Design

The parent leaves the universal recovery initiator/approver, terminal-resolution authority and complete generic recovery contract undefined. It also references a persisted checkpoint as a required recovery position; L4 preserves the observable requirement for a valid prior execution position while deferring persistence and retry mechanics to later architecture.
