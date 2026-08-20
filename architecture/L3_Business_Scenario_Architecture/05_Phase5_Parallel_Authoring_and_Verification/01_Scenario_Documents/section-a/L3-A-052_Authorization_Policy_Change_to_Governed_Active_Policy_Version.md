# Authorization Policy Change to Governed Active Policy Version

# 1. Scenario Metadata

**Scenario ID:** L3-A-052  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authorization & Policy  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** All domains relying on centralized policy-based authorization

---

# 2. Scenario Purpose

Govern a successor Authorization Policy version from changed institutional access-control requirements through composition, validation, approval and activation while preserving deterministic evaluation, explicit-deny precedence and historical decision traceability.

---

# 3. Business Outcome

A new authorization policy version is composed, validated, approved and activated with deterministic evaluation order and explicit-deny precedence preserved, while historical authorization decisions continue to reference the policy version under which they were evaluated

Locked L2-A04 requires controlled policy versioning, approval, deployment, history and rollback but does not define canonical policy states, transitions, commands, events, publishers or named approval and activation authorities. This scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an institutional access-control requirement changes and requires a governed revision of an authorization policy.

## 4.2 End Boundary

The intended end boundary is reached when a validated and approved successor policy version becomes the governed active version, future decisions reference it, and historical decisions retain the exact prior policy versions used. Exact completion is blocked by missing L2-A04 lifecycle and event contracts.

## 4.3 In Scope

- Tenant and institutional policy scope.
- Successor policy version composition.
- Deterministic evaluation-order validation.
- Explicit-deny precedence validation.
- Least-privilege, SoD, scope and configuration validation.
- Approval and controlled activation.
- Preservation of prior policy versions and decision references.
- Audit evidence for the complete policy change.

## 4.4 Out of Scope

- Assigning Roles or Permissions.
- Changing a Person, Identity, User Account or Authentication Session.
- Executing a protected business operation.
- Rewriting historical Authorization Decisions.
- Technical rule-engine, deployment pipeline, API, data-model or UI design.
- Inventing policy states, commands, canonical events, publishers or approval authorities.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Related | L3-A-010 — Institutional Responsibility Assignment to Effective Authorized Access | Role Assignment is evaluated under policy but is not changed here. |
| Related | L3-A-051 — Conflicting Responsibility Risk to Enforced Separation of Duties | SoD Policy is a specialized authorization control with separate configuration gaps. |
| Referenced by | Protected business-operation scenarios | They consume the active policy version during centralized authorization. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authorization & Policy / Section A | Primary owner. Owns Authorization Policy composition, versioning, validation, approval, activation and decision references. |
| Platform & Tenant / Section A | Supplies Tenant and institution configuration boundaries. |
| Event Infrastructure / Section A | Distributes canonical policy events after they are defined upstream. |
| Audit Infrastructure / Section A | Preserves immutable policy-governance and decision evidence. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Institutional policy stakeholder | Supplies the changed access-control requirement named by the trigger. | Does not imply policy-composition, approval or activation authority. |
| Authorization Decision Point | Uses the governed active policy version for future protected requests. | Does not approve or version policy. |

L2-A04 refers to policy approval and controlled deployment but does not name the policy author, validator, approver, activation authority or rollback authority. No actor is invented.

---

# 7. Preconditions and Entry Criteria

- Exactly one Tenant and applicable institutional scope can be resolved.
- A currently governed Authorization Policy version exists or an initial governed policy is required.
- The changed access-control requirement is known.
- Applicable Role, Permission, scope, SoD, explicit-deny and institution-configuration rules can be evaluated.
- Historical policy versions and Authorization Decision references are preservable.
- The required approval and controlled-deployment process is expected to exist, although its canonical contracts and actors are undefined.

---

# 8. Trigger

An institutional access-control requirement changes and requires a governed revision of an authorization policy

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive and scope the changed institutional access-control requirement. | Authorization & Policy | A policy-change need is identified within one Tenant. |
| 2 | Resolve the current policy version, institution configuration and affected authorization scope. | Authorization & Policy; Platform & Tenant | The predecessor and change boundary are established. |
| 3 | Create an intended successor version without modifying the predecessor. | Authorization & Policy — Authorization Policy | Version lineage is established; the canonical creation command and state are undefined. |
| 4 | Compose the successor rules, conditions, scope and evaluation order. | Authorization & Policy — Authorization Policy | The intended policy semantics are defined. |
| 5 | Validate deterministic evaluation order and explicit-deny precedence. | Authorization & Policy | Conflicting rules cannot yield an ambiguous result and Deny retains precedence. |
| 6 | Validate Tenant isolation, least privilege, Role and Permission references, SoD and institution override boundaries. | Authorization & Policy | The successor satisfies architectural constraints. |
| 7 | Assess impact on protected operations and policy consumers. | Authorization & Policy | Affected scopes and consumers are known without changing historical decisions. |
| 8 | Complete the required policy approval. | Authorization & Policy | Intended approval is obtained; the approving actor and transition are undefined upstream. |
| 9 | Activate the approved successor through controlled policy deployment. | Authorization & Policy — Authorization Policy | Intended active availability is reached; canonical state, command and event are undefined. |
| 10 | Apply the active version to future Authorization Decisions. | Authorization Decision Point | New protected requests reference the successor version. |
| 11 | Retain the predecessor and preserve each historical decision's original policy-version reference. | Authorization & Policy; Audit Infrastructure | Historical outcomes remain reproducible and immutable. |
| 12 | Preserve policy composition, validation, approval, activation, lineage and actor evidence. | Authorization & Policy; Audit Infrastructure | The intended policy change is traceable. |

Steps 3, 8 and 9 cannot be finalized as canonical transitions because L2-A04 omits the necessary workflow, commands, states and events.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Validation failure | The successor is ambiguous or violates deny precedence or scope constraints. | Reject or revise the successor before approval. | Current active policy remains unchanged. | Authorization & Policy |
| Approval rejected | Required policy approval is denied. | Retain the proposal as non-active or abandon it under the governing process. | No active-version change occurs; exact proposal state is undefined. | Authorization & Policy |
| Future-effective policy | Approved policy has a later effective time. | Retain it as non-active until the governed effective time. | Current version continues to govern; pre-effective state is undefined. | Authorization & Policy |
| Controlled rollback | A governance decision requires restoration of a prior valid policy. | Follow controlled rollback while preserving all policy and decision history. | A governed version becomes active; canonical rollback command, transition and event are undefined. | Authorization & Policy |
| Institution-specific rule | A permitted institutional configuration changes. | Validate it against platform policy and Tenant scope before inclusion. | Only a permitted scoped rule participates. | Authorization & Policy |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant or policy scope is unresolved. | The successor cannot preserve isolation. | Reject the change. | Platform & Tenant; Authorization & Policy |
| Evaluation order is non-deterministic. | Authorization results could be inconsistent. | Do not approve or activate the successor. | Authorization & Policy |
| Explicit-deny precedence is not preserved. | Mandatory authorization semantics would be violated. | Reject the successor. | Authorization & Policy |
| Referenced Role, Permission or scope is invalid. | Policy evaluation would depend on invalid facts. | Correct the proposal before approval. | Authorization & Policy |
| Approval is absent. | The successor cannot become governed and active. | Retain the current active version. | Authorization & Policy |
| Activation is interrupted. | The authoritative policy version is uncertain. | Do not treat the successor as active; exact recovery states are undefined. | Authorization & Policy |
| Historical decision reference would be rewritten. | Audit reproducibility would be lost. | Reject the operation and preserve the original version reference. | Authorization & Policy; Audit Infrastructure |
| Canonical policy event cannot be identified. | Consumers lack an authoritative version-change fact. | Do not invent an event; obtain an L2-A04 definition. | Authorization & Policy |

No technical retry, rollback implementation or deployment mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Authorization Policy is owned by L2-A04. | Authorization & Policy | Other domains cannot version or activate it. | L2-A04 — Aggregate Ownership |
| Policies require controlled versioning, approval, deployment, tracking and history. | Authorization & Policy | Successor governance and predecessor preservation are mandatory. | L2-A04 — Policy Governance |
| Evaluation order must be deterministic. | Authorization & Policy | The successor cannot introduce ambiguous ordering. | L2-A04 — Authorization Evaluation |
| Explicit Deny overrides Allow. | Authorization & Policy | Deny precedence must survive every version change. | L2-A04 — AP-06 |
| Permissions do not independently authorize execution. | Authorization & Policy | Policy and context still govern each request. | L2-A04 — Permission Constraints |
| Tenant isolation and institution configuration must be preserved. | Platform & Tenant; Authorization & Policy | Cross-Tenant policy scope is prohibited. | L2-A04 — Policy Configuration |
| Historical decisions reference the policy version evaluated. | Authorization & Policy; Audit Infrastructure | Policy change cannot rewrite past outcomes. | Frozen outcome; L2-A04 — Policy History and Audit |
| Every policy change and authorization decision is auditable. | Authorization & Policy; Audit Infrastructure | Complete lineage and decision evidence are required. | L2-A04 — AP-11 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Required Scenario Impact | Intended Result | Canonical Owner |
|---|---|---|---|---|
| Authorization Policy predecessor | Current governed version | Retain immutably with lineage and decision references. | Historical version remains available. | Authorization & Policy |
| Authorization Policy successor | Does not yet exist | Compose, validate, approve and activate. | Governed active version; canonical states are undefined. | Authorization & Policy |
| Authorization Decision — historical | Already recorded | Preserve original policy-version reference. | Unchanged and reproducible. | Authorization & Policy |
| Authorization Decision — future | Not yet produced | Evaluate using the active successor version. | Immutable decision referencing that version. | Authorization & Policy |
| Role and Permission | Existing | Referenced only. | Unchanged. | Authorization & Policy |

L2-A04 does not define canonical policy lifecycle states or a transition matrix; `Draft`, `Approved`, `Active`, `Superseded` and similar states are not introduced.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Event Determination

| Required Business Fact | Source Resolution Result | Scenario Treatment |
|---|---|---|
| Policy successor created | No canonical event or publisher defined. | No event is invented. |
| Policy version approved | No canonical event or publisher defined. | No event is invented. |
| Policy version activated | No canonical event or publisher defined. | No event is invented. |
| Policy version rolled back or superseded | No canonical event or publisher defined. | No event is invented. |

L2-A04 lists human-readable audit activities `Policy Created`, `Policy Modified`, `Policy Activated` and `Policy Deactivated`; it does not define them as canonical event contracts or identify sole publishers.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Tenant and institution configuration | Platform & Tenant | Authorization Policy | Scope and permitted configuration. |
| Role and Permission references | Authorization & Policy aggregates | Authorization Policy | Governed subjects of policy rules. |
| Active policy version | Authorization Policy | Authorization Decision Point | Exact rules and deterministic evaluation order for future requests. |
| Historical policy version | Authorization Policy | Audit and investigation consumers | Version used by a historical decision. |
| Policy audit input | Authorization & Policy | Audit Infrastructure | Lineage, validation, approval, activation and decision references. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- A policy change does not itself authorize a protected operation.
- Future requests undergo the complete centralized evaluation using the active version.
- Explicit Deny remains authoritative.

## 15.2 Approval and Separation of Duty

- L2-A04 requires an approval workflow and controlled deployment.
- It does not name the author, validator, approver, activation authority or rollback authority.
- No approval or separation-of-duty actor is invented.

## 15.3 Audit Obligations

Change requirement, Tenant, predecessor, successor, rule composition, evaluation order, deny validation, scope, approval, activation, lineage, actor, timestamp and correlation must be auditable. Historical decisions must retain their evaluated policy-version references.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are:

- The change was scoped within exactly one Tenant.
- A successor version was composed without rewriting its predecessor.
- Deterministic ordering and explicit-deny precedence were validated.
- Scope, least privilege, SoD and configuration constraints were satisfied.
- Required approval was granted.
- The successor became the governed active version.
- Future decisions reference it.
- Historical decisions retain their original version references.
- Complete policy-governance evidence was preserved.

These criteria cannot be validated canonically because L2-A04 provides no policy lifecycle model, commands, events, publishers or named authorities. The scenario remains `BLOCKED`.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Precise Scenario Impact | Required Resolution |
|---|---|---|---|---|
| A052-BLOCKER-01 | Blocking | L2-A04 requires policy versioning and controlled deployment but defines no Authorization Policy lifecycle states or transition matrix. | Exact predecessor-to-active-successor progression cannot be specified. | Define canonical states and transitions, including rollback and supersession. |
| A052-BLOCKER-02 | Blocking | No canonical commands or workflow identify policy creation, validation, approval, activation or rollback. | L4 cannot derive authoritative operations from this scenario. | Define the owned commands and workflow contracts. |
| A052-BLOCKER-03 | Blocking | No canonical policy-version events or sole publishers are defined. | The active-version change cannot produce a canonical published fact. | Define exact events, publication conditions and publishers. |
| A052-BLOCKER-04 | Blocking | Policy author, validator, approver, activation authority and rollback authority are unnamed. | Responsibility, approval and SoD obligations cannot be allocated without invention. | Define authoritative actors and approval boundaries. |

No unsupported actor, state, command, event, publisher, approval authority or implementation mechanism is assumed.
