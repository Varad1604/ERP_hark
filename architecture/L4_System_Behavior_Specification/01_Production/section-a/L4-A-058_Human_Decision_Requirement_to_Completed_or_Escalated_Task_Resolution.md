# Human Decision Requirement to Completed or Escalated Task Resolution

## 1. Specification Metadata

**Specification ID:** L4-A-058  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Workflow Engine  
**Realizes Scenario:** L3-A-058  
**Primary Business Owner:** Workflow Engine  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-A-058_Human_Decision_Requirement_to_Completed_or_Escalated_Task_Resolution.md`; L2-A02 — Person & Identity Workflows; L2-A04 — Authorization & Policy Workflows; L2-A05 — Workflow Engine Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall coordinate a workflow Human Task from creation and eligible-candidate resolution through claim, work, completion or policy-governed escalation, while keeping identity and authorization with their canonical owners and keeping the substantive business decision with `VAR:Task-Owning Domain`.

The behavior begins when an executing workflow reaches a human-decision node. It ends when the Human Task reaches `Completed` and returns a domain-owned decision to orchestration, or when an applicable SLA/policy produces the source-defined governed escalation outcome and corresponding continuation path.

## 3. Required Business Context

- An executing workflow instance has reached a Human Task node.
- The Task-owning business domain and required decision context are identifiable.
- Candidate identity/responsibility facts and authorization context can be resolved from their canonical owners.
- The Human Task lifecycle and applicable SLA/escalation policy are available.
- The Workflow Engine coordinates task state but does not make the Task-owning domain's business decision.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-058-BH-001 | When an executing workflow reaches a human-decision node, the system shall establish the Human Task under Workflow Engine ownership and associate it with the originating workflow context and Task-owning domain. | L3-A-058 §§2–5, 7–9, 12–13 |
| L4-A-058-BH-002 | The system shall resolve eligible task candidates using source-governed identity/responsibility and authorization facts without transferring candidate ownership to the Workflow Engine. | L3-A-058 §§2, 5, 9, 12, 14 |
| L4-A-058-BH-003 | When candidate resolution is valid, the system shall place the Human Task in `Available` and make it eligible for governed claim. | L3-A-058 §§3, 9, 13 |
| L4-A-058-BH-004 | On a valid `ClaimTask` request, the system shall accept an effective claim for only one eligible claimant at a time and shall place the task in `Claimed`. | L3-A-058 §§9, 11–13 |
| L4-A-058-BH-005 | The system shall permit the claimed task to progress to `In Progress` only for the valid current claimant under the applicable authorization context. | L3-A-058 §§9, 12–13 |
| L4-A-058-BH-006 | The substantive business decision recorded for the Human Task shall be supplied under the authority of `VAR:Task-Owning Domain`; the Workflow Engine shall coordinate but shall not redefine that decision. | L3-A-058 §§2, 4–5, 9, 12, 14 |
| L4-A-058-BH-007 | On a valid `CompleteTask` outcome, the system shall place the Human Task in `Completed`, preserve the domain-owned decision result, and make that result available to the originating workflow continuation. | L3-A-058 §§3, 9, 12–13, 16 |
| L4-A-058-BH-008 | When an applicable SLA/policy escalation condition becomes conclusive before completion, the system shall apply the source-defined escalation path and shall not silently mark the task Completed. | L3-A-058 §§3, 9–13, 16 |
| L4-A-058-BH-009 | Where the source-governed escalation path returns the task to `Available`, the system shall preserve escalation history and re-open candidate handling without rewriting the prior task history. | L3-A-058 §§9–13 |

## 5. Cross-Domain & Event Behavior

Person & Identity supplies canonical identity facts. Authorization & Policy supplies candidate/access decisions. `VAR:Task-Owning Domain` supplies the substantive business decision. Workflow Engine owns the Human Task lifecycle and orchestration continuation.

Locked L2-A05 contains two inconsistent task-event naming families: `WorkflowTaskCreated` / `WorkflowTaskAssigned` / `WorkflowTaskClaimed` / `WorkflowTaskCompleted` / `WorkflowTaskEscalated`, and `TaskCreated` / `TaskAssigned` / `TaskClaimed` / `TaskCompleted` / `TaskEscalated`. This L4 shall not select one family as canonical until the upstream nomenclature conflict is resolved.

## 6. Failure / Rejection / Continuation

If no eligible candidate can be resolved, claim authorization fails, the claimant is no longer valid, or the business decision cannot be completed, the system shall keep the task outside `Completed` and follow only a source-governed task/SLA continuation. Competing claim attempts shall not create multiple effective claimants. No locking algorithm, persistence transaction, technical retry or checkpoint implementation is specified.

## 7. Audit & Evidence Behavior

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-058-BH-010 | The system shall preserve evidence linking the Human Task to its workflow context, candidate-resolution basis, current claimant where applicable, lifecycle progression, Task-owning domain decision, SLA/escalation outcome and orchestration continuation. | L3-A-058 §§12, 15–16 |

## 8. Observable Completion Conditions

A normal completion is observable when the Human Task reaches `Completed`, exactly one valid claimed-task path has produced the domain-owned decision, and the result is available to workflow continuation. An escalation outcome is observable only when the applicable source-governed SLA/policy condition has produced the defined escalation path. Event-name ambiguity does not permit invention of a new event family.

## 9. Traceability & Provenance

**Parent:** L3-A-058 — Human Decision Requirement to Completed or Escalated Task Resolution  
**Primary verification sources:** L2-A02 — Person & Identity Workflows; L2-A04 — Authorization & Policy Workflows; L2-A05 — Workflow Engine Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows

Every BH preserves the parent distinction between Workflow Engine coordination and domain-owned business decisions. Implementation-specific task locking, persistence and retry mechanics remain outside L4.

## 10. Open Issues / Deferred Design

L2-A05 uses inconsistent canonical-looking names for the same Human Task event family; the repository must select or formally relate those names before this L4 can name a sole canonical event for each task transition. The source also uses assignment facts without an `Assigned` lifecycle state; this L4 treats assignment as source-governed task metadata/result and does not invent an `Assigned` state.
