# Workflow Initiation to Completed Orchestrated Execution

## 1. Specification Metadata

**Specification ID:** L4-A-015  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Workflow Engine  
**Realizes Scenario:** L3-A-015  
**Primary Business Owner:** Workflow Engine  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-A-015_Workflow_Initiation_to_Completed_Orchestrated_Execution.md`; `L2-A05_Workflow_Engine_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A06_Event_Infrastructure_Workflows.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall execute one approved published Workflow Version within one Tenant, coordinate only the orchestration responsibilities defined by that immutable version, preserve participating-domain business ownership, and return a governed `Completed` or `Cancelled` outcome and output contract. Failure recovery and later archival remain outside this specification.

## 3. Required Business Context

- One Tenant and eligible Workflow-Owning Domain.
- One approved published immutable Workflow Version with defined input/output contracts.
- Valid execution context, correlation and business references.
- Domain-owned commands/outcomes and Authorization Decisions required by the version.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-015-BH-001 | The system shall prevent workflow initiation unless the Tenant, selected published version, initiating authority, execution context and Workflow Input Contract are valid. | L3-A-015 §§7, 9, 11–12 |
| L4-A-015-BH-002 | When initiation is valid, the system shall create one Workflow Instance permanently bound to the selected immutable Workflow Version and begin `Created → Running`. | L3-A-015 §§9, 12–13 |
| L4-A-015-BH-003 | When governed execution begins, the system shall publish `WorkflowStarted` on behalf of the Workflow Engine. | L3-A-015 §§9, 14 |
| L4-A-015-BH-004 | The system shall coordinate only steps, routing and transition conditions defined by the originating Workflow Version and shall not invent business decisions or change participating-domain state directly. | L3-A-015 §§2–5, 9, 12 |
| L4-A-015-BH-005 | For a domain-owned action or decision, the system shall obtain the authoritative result from the owning domain and use that result only as governed orchestration input. | L3-A-015 §§5, 9, 12, 14 |
| L4-A-015-BH-006 | When a defined wait condition is unsatisfied, the system shall retain the instance in `Waiting` and resume only when the source-defined condition is satisfied. | L3-A-015 §§9–13 |
| L4-A-015-BH-007 | When an authorized suspension applies, the system shall place the instance in `Suspended` and resume the same instance/version only through the governed resumption path. | L3-A-015 §§10, 13, 15 |
| L4-A-015-BH-008 | When all completion conditions succeed, the system shall transition the instance to `Completed`, publish `WorkflowCompleted`, and return the defined Workflow Output Contract. | L3-A-015 §§9, 13–16 |
| L4-A-015-BH-009 | When a permitted authorized cancellation succeeds, the system shall transition the instance to `Cancelled`, publish `WorkflowCancelled`, and return the governed cancellation outcome. | L3-A-015 §§9, 11, 13–16 |
| L4-A-015-BH-010 | If execution reaches `Failed`, the system shall publish `WorkflowFailed`, stop normal A015 progression, and expose the failed outcome for separately governed recovery without defining recovery mechanics here. | L3-A-015 §§4.4, 11, 14 |

## 5. Cross-Domain & Event Behavior

`WorkflowStarted`, `WorkflowCompleted`, `WorkflowCancelled`, and `WorkflowFailed` retain the Workflow Engine as canonical publisher. Participating domains retain ownership and publication of their own business facts. Event Infrastructure distributes events without becoming semantic publisher.

## 6. Failure / Rejection / Continuation

Invalid initiation shall not create a running instance. A domain rejection shall follow only a path defined by the immutable Workflow Version. Unauthorized cancellation shall preserve the current valid state. `Failed` exits this scenario and shall not be silently recovered by L4-A-015.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-015-BH-011 | The system shall preserve attributable, correlated evidence of Tenant, instance/version, initiator, input validation, orchestration state transitions, domain outcomes, authorization decisions, waiting/suspension/resumption, output and terminal event. | L3-A-015 §15.3 |

## 8. Observable Completion Conditions

Completion requires the instance to reach `Completed` or `Cancelled`, the corresponding canonical event to be published, the defined output/final status to be returned to the owning domain, and participating-domain ownership to remain unchanged. Later `Archived` handling is not part of this completion boundary.

## 9. Traceability & Provenance

**Parent:** L3-A-015 — Workflow Initiation to Completed Orchestrated Execution  
**Primary verification source:** L2-A05 — Workflow Engine Workflows

## 10. Open Issues / Deferred Design

The parent records non-blocking actor, event-vocabulary and execution-terminal-versus-archival terminology issues. This draft uses the explicit runtime contract and event semantics without inventing universal human actors or treating `Archived` as an A015 completion state.