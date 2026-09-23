# L2-A05 — Workflow Definition and Version Governance Authority — Locked Amendment

| Attribute | Value |
|---|---|
| Amendment Version | 1.0.0 |
| Status | Locked |
| Architecture Level | L2 — Business Workflow Architecture |
| Canonical Owner | Workflow Engine |
| Approved Through | Lead Architect continuation of approved post-R8 upstream remediation, 2026-09-23 |
| Scope | Workflow Definition / Workflow Version governance and governed instance migration |

---

## 1. Purpose

This amendment closes the authority, operation-contract and lifecycle-vocabulary gaps recorded by L3-A-014 and L3-A-017.

It does not transfer business-process ownership to the Workflow Engine. The Workflow-Owning Domain remains owner of its business rules, decisions, states, commands, events and outcomes.

---

## 2. Canonical Actors and Authorities

### 2.1 Authorized Workflow Designer / Administrator

`Authorized Workflow Designer / Administrator` is the canonical abstract actor category permitted to prepare a Workflow Definition Draft or successor Draft and submit it for governed validation.

Institutional policy maps this actor category to eligible persons or administrative responsibilities.

Preparation does not constitute governance approval.

### 2.2 Workflow Governance Authority

`Workflow Governance Authority` is the canonical abstract business authority for:

- approval of a validated Workflow Definition / successor version;
- authorization of immutable Workflow Version publication;
- authorization of activation for new Workflow Instances;
- authorization of retirement where retirement is separately invoked;
- governance approval of an exceptional instance migration.

Institutional policy maps this authority to the eligible governance office, committee, administrator or authority holder.

No universal job title is imposed.

### 2.3 Workflow Migration Administrator

`Workflow Migration Administrator` is the canonical abstract actor category permitted to request/execute an individually governed Workflow Instance migration after every migration prerequisite has succeeded.

The Migration Administrator does not approve their own migration merely by executing it. Governance Approval remains a Workflow Governance Authority responsibility.

No universal rule is introduced requiring the Migration Administrator and Governance Authority to be different persons; institutional policy may impose stronger separation of duties.

### 2.4 Workflow Engine

Workflow Engine / L2-A05 remains sole owner of:

- Workflow Definition;
- Workflow Version;
- definition/version lifecycle;
- validation execution;
- immutable publication;
- activation;
- retirement;
- Workflow Instance → originating Workflow Version binding;
- governed change of that binding after admitted migration;
- canonical Workflow Definition / Version events.

### 2.5 Workflow-Owning Domain

The Workflow-Owning Domain:

- owns the business-process requirement and business semantics;
- supplies/approves the domain-owned orchestration requirement under its own governance;
- remains owner of domain business rules, decisions, states, commands, events and outcomes;
- does not directly create, publish, activate or mutate Workflow Engine aggregates.

---

## 3. Canonical Definition / Version Operations

The following are canonical business operations. They define architecture-level intent and preconditions, not APIs or message schemas.

| Operation | Required Preconditions | Authoritative Outcome |
|---|---|---|
| Create Workflow Definition Draft | Tenant and Workflow-Owning Domain resolve; approved business-process requirement exists; actor is authorized | New independent `Draft`; `WorkflowDefinitionCreated` may be published |
| Submit Workflow Definition for Validation | Draft is complete enough for mandatory checks | `Draft → Validation` |
| Record Workflow Governance Approval | Mandatory validation succeeds; Workflow Governance Authority approves | `Validation → Approved` |
| Publish Workflow Version | Definition is Approved; Workflow Governance Authority authorizes publication | Immutable `Approved → Published`; `WorkflowVersionPublished` |
| Activate Workflow Version | Version is Published; Workflow Governance Authority authorizes activation | `Published → Activated`; `WorkflowVersionActivated`; eligible for new instances |
| Create Successor Workflow Draft | Published/Activated predecessor exists; governed change requirement exists | Independent successor `Draft`; predecessor remains immutable |
| Retire Workflow Version | Applicable retirement conditions and Workflow Governance Authority approval exist | `Activated → Retired` or other source-permitted retirement edge; `WorkflowVersionRetired` where the canonical retirement transition succeeds |

Existing L2-A05 validation, immutability and version-binding rules remain unchanged.

---

## 4. Workflow Migration Governance

Migration is exceptional and shall never occur automatically.

### 4.1 Canonical Migration Operations

| Operation | Required Preconditions | Outcome |
|---|---|---|
| Request Workflow Instance Migration | Instance and originating version resolve; target successor is eligible; Migration Plan exists | Migration request becomes eligible for compatibility/governance evaluation |
| Validate Workflow Migration Compatibility | Migration Plan exists; source/target compatibility can be evaluated | Compatibility result is authoritative for admission |
| Approve Workflow Instance Migration | Migration Plan + successful Compatibility Validation + Workflow Governance Authority approval + Workflow Migration Administrator authorization + required audit context exist | Migration may be admitted |
| Apply Governed Workflow Instance Migration | Approved migration remains valid at execution time | Instance-version binding changes to the approved target under Workflow Engine ownership |
| Reject / Retain Workflow Instance Version | Any safeguard fails or migration is unsafe | Instance remains on originating Workflow Version |

### 4.2 No Migration Lifecycle/Event Invention

Migration is **not** made a separate aggregate lifecycle by this amendment.

No canonical `MigrationPending`, `MigrationApproved`, `WorkflowInstanceMigrated` or equivalent state/event is introduced.

The authoritative business facts are:

- the governed migration decision and evidence;
- the Workflow Engine-owned instance/version relationship before and after a successfully admitted migration.

Audit Infrastructure may preserve evidence, but it does not own migration semantics.

---

## 5. Canonical Events

The existing canonical definition/version event family remains:

- `WorkflowDefinitionCreated`;
- `WorkflowVersionPublished`;
- `WorkflowVersionActivated`;
- `WorkflowVersionRetired` when retirement succeeds.

Workflow Engine / L2-A05 is the sole semantic publisher.

Event Infrastructure distributes these facts without acquiring semantic ownership.

This amendment introduces no new event name.

---

## 6. Lifecycle Vocabulary Alignment

The canonical Workflow Definition / Version governance lifecycle is:

```text
Draft → Validation → Approved → Published → Activated → Retired
```

Section 28 Workflow Metadata `Status` shall be interpreted and maintained using this complete canonical lifecycle vocabulary.

The earlier abbreviated metadata list `Draft, Published, Retired` is superseded for lifecycle-status interpretation by this Locked amendment.

No lifecycle state is added beyond the states already defined by L2-A05 §26.

---

## 7. Authorization and Separation of Duties

Authorization & Policy / L2-A04 evaluates and enforces protected actions according to applicable policy.

Workflow Governance Authority remains the business authority for the governed definition/version decision.

The following are conceptually distinct responsibilities:

- preparation;
- governance approval;
- publication authorization;
- activation authorization;
- migration execution.

This amendment does not impose a universal rule that each responsibility must be held by a different person. Institutional policy may impose stronger separation-of-duty constraints.

---

## 8. Locked Decisions

1. `Authorized Workflow Designer / Administrator` is the canonical abstract preparation actor category.
2. `Workflow Governance Authority` is the canonical abstract approval/publication/activation/retirement and migration-governance authority.
3. `Workflow Migration Administrator` is the canonical abstract migration request/execution actor category.
4. Workflow Engine retains sole ownership of Workflow Definition, Workflow Version, their lifecycle and instance-version binding.
5. Workflow-Owning Domain retains ownership of all domain business semantics.
6. Definition/version state-changing operations are the architecture-level operations defined in §3; no API/message contract is specified at L2.
7. Migration operations and safeguards are those defined in §4; migration is exceptional and never automatic.
8. No migration-specific lifecycle state or canonical migration event is introduced.
9. Existing definition/version events remain Workflow Engine-owned.
10. The complete canonical lifecycle-status vocabulary is `Draft → Validation → Approved → Published → Activated → Retired`.
11. No universal named institutional job title or universal SoD rule is introduced.
