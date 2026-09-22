# ADR-L5-0042 — Flowable Workflow Runtime, Task, Timer, Version & Compensation Profile

**ADR ID:** `ADR-L5-0042`  
**Title:** Flowable Workflow Runtime, Task, Timer, Version & Compensation Profile  
**Scope:** Shared / Family  
**Decision Owner:** L5-A-05 — Workflow & Orchestration Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-A-05-001`; `L5-A-05-002`; `L5-A-05-003`; constrains D02 and A08 collaboration  
**Related ADRs:** `ADR-L5-0013`; `ADR-L5-0023`; `ADR-L5-0039`; `ADR-L5-0041`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

BPMN 2.x and Flowable OSS 8.x are already canonical. The remaining workflow-runtime decisions concern the boundary between orchestration state and business state, task authorization, timer ownership, running-instance version behavior and compensation.

## 2. Decision

1. **Orchestration state only:** Flowable persists workflow/orchestration runtime state, task routing state, timers and technical execution context. Canonical business aggregate state remains with its owning domain and is changed only through owner contracts.
2. **No business truth in workflow variables:** workflow variables may carry references, snapshots or transient orchestration context but shall not become the sole authoritative store of a business fact.
3. **Human task assignment is not authorization:** Flowable may calculate/store candidate users/groups or task routing information, but D02 authorization is re-evaluated when the protected action is performed. Task assignment never grants authority by itself.
4. **Timer ownership:** BPMN/Flowable timers own workflow-relative waits, deadlines, escalations and orchestration timers. A08 owns recurring technical/background schedules that are not workflow-instance semantics.
5. **Running-version pinning:** a workflow instance is normally pinned to the published workflow-definition version under which it started. Publication of a newer definition does not silently mutate existing running instances.
6. **Explicit migration only:** migration of running instances to another workflow-definition version is explicit, governed, compatibility-checked and tested. No automatic mass migration to the latest version occurs.
7. **Compensation:** compensation executes only explicit source-owner-provided compensating commands/actions where upstream architecture allows them. A05/Flowable shall not invent business rollback/reversal policy.
8. **Technical failures:** workflow-engine/adapter transient failures may use A08/C02 technical retry mechanisms, but business rejection is not converted into a technical retry.
9. **External work:** heavy/bulk/long-running technical computation is handed to A08 Rust workers through governed contracts; Flowable waits on the governed result/status rather than performing generic computation itself.
10. **Authorization freshness:** delegation, temporary authority, privileged access, SoD/maker-checker and revocation remain D02-controlled and are not frozen solely because a task was created earlier.
11. **Workflow ordering:** Flowable sequence semantics apply within the workflow instance; they do not create repository-wide global ordering.
12. **Audit/evidence:** workflow execution emits attributable technical/business-reference evidence to the appropriate owners without making Flowable the canonical audit store.

## 3. Conformance Rules

- A completed Flowable task does not by itself prove a canonical business state transition unless the owning domain confirms that transition.
- Workflow instance deletion/cleanup shall not delete canonical business state or D04 evidence.
- Candidate resolution may consume D01/D02 facts but shall not redefine Role Assignment, Permission or scope ownership.
- Compensation failure remains explicit and recoverable; it is not silently marked as successful.
- Workflow timers shall use durable engine/runtime semantics appropriate to Flowable and shall survive ordinary process restarts according to the approved deployment/recovery profile.
- Technical job status from A08 is referenced, not copied into a second authoritative business lifecycle.

## 4. Consequences

Flowable remains a powerful orchestration engine without becoming a parallel domain model or authorization system. Running process behavior is reproducible because versions are pinned, and compensation stays faithful to source-owned business semantics.

## 5. Approval Record

**Decision selected:** Flowable orchestration state only; D02 re-authorization at action time; Flowable workflow timers vs A08 technical schedules; running instances pinned by default; explicit governed migration; owner-provided compensation only; heavy work delegated to Rust workers.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
