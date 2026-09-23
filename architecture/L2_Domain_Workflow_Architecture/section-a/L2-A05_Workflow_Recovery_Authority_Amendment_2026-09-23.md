# L2-A05 — Workflow Recovery Authority — Locked Amendment

| Attribute | Value |
|---|---|
| Amendment Version | 1.1.0 |
| Status | Locked |
| Architecture Level | L2 — Business Domain Architecture |
| Canonical Owner | Platform Foundation — Workflow Engine |
| Approved Through | Explicit Lead Architect decision, 2026-09-23 |
| Authority | Post-R8 Option A upstream business-architecture remediation |

---

## 1. Purpose

This amendment is part of canonical L2-A05. It closes the recovery-authority gap recorded by L3-A-016 without changing L2-A05 ownership of Workflow Instance execution or transferring any business-domain decision to the Workflow Engine.

The original L2-A05 document and this amendment shall be read together.

---

## 2. Canonical Recovery Authority

### 2.1 Workflow Recovery Authority

`Workflow Recovery Authority` is the canonical abstract business authority for protected Workflow Instance recovery actions.

It is institution-independent and may be mapped by institutional policy to an eligible platform operations/governance actor or authority holder.

It authorizes:

- entry from `Failed → Recovering`;
- return from `Recovering → Running` after all recovery preconditions are satisfied;
- terminal recovery closure through `Recovering → Cancelled` when resumption is not permitted.

It does **not**:

- own Workflow Instance state transitions;
- decide whether a domain-owned business action succeeded;
- authorize a domain business outcome;
- alter a Workflow Version;
- replace Authorization & Policy enforcement.

### 2.2 Workflow Engine

Workflow Engine / L2-A05 remains canonical owner of:

- Workflow Instance lifecycle;
- failure classification;
- recovery-position determination;
- retry/recovery execution;
- lifecycle transitions;
- workflow recovery/cancellation events.

The Workflow Engine executes the authorized lifecycle action. It does not become the business approver merely because it owns execution.

### 2.3 Workflow-Owning Domain

The Workflow-Owning Domain remains authoritative for whether its domain-owned action:

- completed;
- failed;
- remains valid;
- may be requested again;
- must not be repeated.

Recovery may not infer, reverse, or overwrite an authoritative domain outcome.

### 2.4 Authorization & Policy

Authorization & Policy / L2-A04 evaluates and enforces the applicable access/policy decision for the protected recovery action.

Authorization enforcement does not transfer Workflow Instance ownership or domain-business decision ownership.

---

## 3. Canonical Recovery Operations

The following business operations are canonical:

| Operation | Preconditions | Outcome |
|---|---|---|
| Request Workflow Recovery | Instance is `Failed`; Tenant, instance and originating version resolve; failure is classified | Recovery eligibility may be evaluated |
| Authorize Workflow Recovery | Recovery policy permits continuation; required domain-owned business outcome is known; Workflow Recovery Authority authorizes | `Failed → Recovering` may proceed |
| Resume Recovered Workflow | A valid governed continuation position exists; protected recovery action remains authorized | `Recovering → Running` |
| Authorize Workflow Cancellation | Recovery cannot or must not resume; Workflow Recovery Authority authorizes terminal closure | `Recovering → Cancelled` |

`RetryWorkflow` remains the existing representative L2-A05 command concept. This amendment does not define an API, message schema or implementation command contract.

---

## 4. Recovery Position Boundary

A recovery continuation position is the last source-valid workflow execution position from which L2-A05 may safely continue the same Workflow Instance and originating Workflow Version.

L2 may require such a governed continuation position.

Persistence technology, checkpoint storage, transaction mechanism, replay implementation and infrastructure recovery remain L4/L5 concerns.

Therefore a business scenario may require a valid governed recovery position without specifying persisted-checkpoint implementation.

---

## 5. Canonical Events

This amendment relies only on the existing canonical L2-A05 recovery lifecycle event family:

- `WorkflowFailed`;
- `WorkflowRecovered`;
- `WorkflowCancelled`.

Workflow Engine / L2-A05 is the sole semantic publisher of these workflow lifecycle facts.

No additional recovery or checkpoint event is made canonical by this amendment.

Event Infrastructure may distribute these facts but does not own their semantics.

---

## 6. Locked Decisions

The following decisions are Locked:

1. `Workflow Recovery Authority` is the canonical abstract authority for protected Workflow Instance recovery/resumption/cancellation decisions.
2. Workflow Engine owns Workflow Instance lifecycle execution and transitions.
3. Workflow-Owning Domain owns all domain-specific business outcomes and repeatability decisions.
4. Authorization & Policy enforces protected recovery authorization but does not own Workflow Instance lifecycle.
5. Recovery uses the existing `Failed → Recovering → Running/Cancelled` lifecycle; no new recovery state is introduced.
6. `RetryWorkflow` remains the representative recovery command concept; no implementation contract is defined at L2.
7. Recovery requires a valid governed continuation position, while persistence/checkpoint mechanics remain downstream.
8. `WorkflowFailed`, `WorkflowRecovered`, and `WorkflowCancelled` are the canonical recovery lifecycle event family used by this amendment.
9. No universal named institutional job title is introduced for recovery authority.
10. No universal separation-of-duties rule is introduced beyond applicable institutional policy.
