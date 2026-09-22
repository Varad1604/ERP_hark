# Workflow Definition, Versioning & Policy Resolution Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-05-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-A — Core Platform & Application Architecture`  
**Major Area:** `L5-A-05 — Workflow & Orchestration Architecture`  
**Canonical Technical Responsibility:** Technical workflow definition/versioning, scope resolution and effective policy selection.  
**Technical Architecture Owner:** `L5-A-05 — Workflow & Orchestration Architecture`  
**Assigned Author:** Devang  
**Version:** `1.2.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register 105 v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-A-05-001_Workflow_and_Orchestration_Architecture_Baseline.md`; `L5-D-02-001` through `L5-D-02-007`; `L5-C-01-001` through `L5-C-01-003`; `L5-C-02-001` through `L5-C-02-003`; `L5-A-04-001/002`; `L5-B-02-002/003`; applicable `L5-D-04`; `L2-A05_Workflow_Engine_Workflows.md`; `L4-A-014_Workflow_Definition_to_Activated_Orchestration_Availability.md`; `L4-A-017_Workflow_Change_to_Governed_New_Version_Availability.md`; supporting `L4-A-015` and `L4-A-016` for runtime/version binding.  
**Referenced By:** `L5-A-05-003`; workflow-owning domains; workflow administration/governance capabilities; runtime version resolver; D02 policy decision/enforcement; C01/C02 integration architecture; D04 evidence architecture.  
**Applicable ADRs:** `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0007`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0023`; `ADR-L5-0042`. BPMN 2.x and Flowable OSS 8.x are canonical; exact Workflow Version numbering, persistence schema/indexing and registry storage realization remain governed implementation details.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the L5 technical architecture for creating, validating, versioning, publishing, activating, resolving and evolving SutraERP Workflow Definitions and Workflow Versions.

It also defines how Workflow Definitions reference and consume external policy without taking ownership of that policy.

The core responsibility is:

```text
approved business-process orchestration requirement
        ↓
A05 technical definition
        ↓
validated workflow structure + governed references
        ↓
immutable published Workflow Version
        ↓
activated version available for new Workflow Instances
        ↓
deterministic version / policy resolution at runtime
```

The architecture preserves four independent ownership dimensions:

```text
Workflow structure
    → A05

Business rules / decisions / states / outcomes
    → Workflow-Owning Domain

Authorization policy / Authorization Decision
    → D02

Event / contract transport mechanics
    → C01 / C02
```

### 2.2 Primary Design Goal

A05-002 shall let SutraERP change workflow structure safely without:

- mutating a published version;
- silently changing running Workflow Instances;
- copying business rules into workflow definitions;
- copying policy implementations into workflow definitions;
- treating a policy change as automatically requiring a workflow version change;
- inventing business approval actors;
- introducing undefined migration lifecycle states/events;
- coupling workflow definitions to a specific engine/vendor notation.

### 2.3 In Scope

This specification owns or constrains:

- Workflow Definition technical representation;
- Workflow Version technical representation;
- definition metadata and ownership references;
- node/connector structural representation at architecture level;
- workflow input/output contract references;
- policy references;
- event references;
- dependency references;
- domain capability references;
- definition validation pipeline;
- definition lifecycle technical realization;
- publication boundary;
- immutable version snapshot;
- successor version lineage;
- activated-version resolution for new instances;
- current/effective policy-reference resolution boundary;
- workflow version compatibility checks;
- active-instance impact analysis;
- instance migration admission safeguards;
- definition/version concurrency;
- idempotent publication/activation handling;
- workflow definition/version event publication handoff;
- design-time evidence/provenance;
- caching/indexing rules for version resolution;
- Tenant/Institution isolation for definition/version resolution.

### 2.4 Out of Scope / Non-Goals

This specification does not own:

- business-process meaning;
- business rules;
- business decisions;
- business state machines;
- business approval authority;
- business event semantics;
- policy content;
- policy lifecycle;
- policy author/approver;
- D02 Authorization Policy version lifecycle;
- human task execution;
- timer execution;
- escalation execution;
- runtime retry/compensation mechanics;
- Workflow Instance detailed runtime;
- notification delivery;
- exact workflow-definition syntax;
- BPMN/DMN/BPEL or any named notation;
- exact workflow engine;
- visual workflow designer;
- UI layout;
- exact version numbering format;
- exact migration algorithm;
- exact persistence schema;
- exact cache technology;
- exact workflow registry product.

A05-003 owns detailed runtime/task/timer/escalation/compensation behavior.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Frozen L5 Responsibility

The approved 105-specification corpus defines `L5-A-05-002` as:

> Workflow Definition, Versioning & Policy Resolution Architecture — technical workflow definition/versioning, scope resolution and effective policy selection.

This file therefore owns technical definition/version mechanics and policy-reference resolution, but not policy meaning.

### 3.2 Locked L2-A05 Constraints

The following are established upstream facts, not local A05-002 decisions:

- A05 owns Workflow Definitions and Workflow Versions.
- Workflow Definitions own workflow structure.
- Business domains own business behavior.
- Definition lifecycle includes `Draft`, `Validation`, `Approved`, `Published`, `Activated`, `Retired`.
- `Approved` means governance approval exists before publication.
- `Published` represents an immutable version available for execution.
- `Activated` is the current active version for new workflow instances.
- Published Workflow Versions are immutable.
- Running Workflow Instances remain attached to their originating published version unless an explicit migration strategy succeeds.
- Workflow definition components include Metadata, Nodes, Connectors, Variables, Input Contract and Output Contract.
- Workflow validation includes structural/policy/version/dependency validation and related source-required checks.
- Workflow policy evolution is independent of Workflow Definition versioning.
- Workflow Definitions reference policy identifiers rather than concrete implementations.
- Integration contracts evolve independently from workflow definitions.
- A05 remains vendor-independent.

### 3.3 Definition Lifecycle Constraint

The canonical lifecycle is consumed as:

```text
Draft
  ↓
Validation
  ↓
Approved
  ↓
Published
  ↓
Activated
  ↓
Retired
```

This specification does not invent extra business lifecycle states.

Technical sub-status, lock/version stamps or processing markers may exist only as implementation metadata and shall never be exposed as new canonical lifecycle states.

### 3.4 L4-A-014 Exact Traceability

| L4 BH | A05-002 Material Technical Contribution |
|---|---|
| `L4-A-014-BH-001` | Definition admission requires resolved Tenant, Workflow-Owning Domain and approved business-process boundary. |
| `L4-A-014-BH-002` | Technical creation establishes the source-defined `Draft` representation and workflow-owned publication handoff for `WorkflowDefinitionCreated`. |
| `L4-A-014-BH-003` | Definition model stores orchestration references only; business rules/decisions/states/commands/events remain source-owned. |
| `L4-A-014-BH-004` | Definition Validation Pipeline performs source-required structural, policy, event, version, dependency and compatibility validation before progression. |
| `L4-A-014-BH-005` | Publication gate requires the source-governed approval outcome without inventing approver identity. |
| `L4-A-014-BH-006` | Publication creates an immutable published Workflow Version and workflow-owned `WorkflowVersionPublished` publication intent. |
| `L4-A-014-BH-007` | Activation resolver rejects any version that is not validly `Published`. |
| `L4-A-014-BH-008` | Activation makes the version resolvable for new instances and exposes governed input/output contract references; `WorkflowVersionActivated` remains workflow-owned. |
| `L4-A-014-BH-009` | Definition/version processing never starts a Workflow Instance. |
| `L4-A-014-BH-010` | Definition/version evidence preserves owner, validation, approval, immutable version, activation and canonical events. |

### 3.5 L4-A-017 Exact Traceability

| L4 BH | A05-002 Material Technical Contribution |
|---|---|
| `L4-A-017-BH-001` | Any structural change to a published version creates an independent successor Draft; in-place mutation is rejected. |
| `L4-A-017-BH-002` | Successor definition changes orchestration structure/references only; changed business semantics remain owning-domain responsibility. |
| `L4-A-017-BH-003` | Successor passes the same governed structural/policy/event/version/dependency/compatibility validation before approval. |
| `L4-A-017-BH-004` | Successor publication remains gated by source-governed approval without inventing actors. |
| `L4-A-017-BH-005` | Successful publication creates immutable successor version and preserves `WorkflowVersionPublished` semantics. |
| `L4-A-017-BH-006` | Valid activation makes successor available for new Workflow Instances. |
| `L4-A-017-BH-007` | Existing Workflow Instances retain their originating version after successor activation. |
| `L4-A-017-BH-008` | Migration admission requires source-defined Migration Plan, Compatibility Validation, Governance Approval, Administrator Authorization and Audit Recording. |
| `L4-A-017-BH-009` | Unsafe/rejected/unauthorized migration leaves instance on its originating Workflow Version. |
| `L4-A-017-BH-010` | No migration lifecycle state/event is invented. |
| `L4-A-017-BH-011` | Evidence preserves change rationale, lineage, validation, approval, publication, activation, active-instance impact and migration decisions. |

### 3.6 Supporting Runtime Constraints from L4-A-015/A016

A05-002 additionally preserves:

- a Workflow Instance binds to a selected immutable Workflow Version at start;
- subsequent runtime state does not silently substitute a successor version;
- recovery preserves originating Workflow Version;
- recovery cannot rewrite completed authoritative domain outcomes.

A05-003 realizes the runtime details.

### 3.7 Policy Independence Constraint

Canonical L2-A05 states:

```text
Policy evolution
        ≠
Workflow Definition version evolution
```

Therefore:

```text
Workflow Definition
    stores a policy identifier/reference

D02 / owning policy capability
    owns actual policy definition and authoritative current policy version

A05
    resolves/requests the applicable policy reference/result
    without embedding policy implementation
```

A policy change does not mutate an immutable Workflow Version.

A workflow version change is required only when workflow-owned orchestration structure/references change under upstream rules.

### 3.8 Contract Independence Constraint

C01/C02 contracts can evolve independently from workflow definitions.

A05-002 stores governed contract/event references and compatibility requirements.

It shall not copy wire schemas or transport implementation into the Workflow Version as the canonical source.

### 3.9 No Hidden Assumptions

This Draft does not assume:

- one specific workflow notation;
- one version-numbering scheme;
- semantic-version numbering for workflow versions;
- one active version per entire platform independent of Tenant;
- a universal migration capability;
- a universal rollback operation;
- a universal “latest version wins” rule;
- policy version pinning for all policies;
- policy auto-upgrade for all policies;
- a named definition approver;
- a named publication/activation authority;
- a workflow-designer UI.

Where source architecture is silent, the issue remains explicit.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 A05-002 Owns

- Workflow Definition technical model;
- Workflow Version technical model;
- definition/version repository boundary;
- immutable version snapshot semantics;
- predecessor/successor lineage;
- definition validation orchestration;
- reference validation;
- publication technical gate;
- activation technical gate;
- effective Workflow Version resolution;
- policy-reference binding/resolution boundary;
- active-instance impact analysis;
- migration admission validation;
- definition/version concurrency;
- definition/version evidence handoff;
- definition/version cache/index invalidation rules.

### 4.2 A05-002 Does Not Own

- business policy content;
- Authorization Policy lifecycle/approval;
- business process approval actor;
- workflow task runtime;
- timer execution;
- escalation;
- compensation;
- domain contract implementation;
- event transport;
- Tenant hierarchy;
- identity;
- authorization decision;
- audit storage.

### 4.3 A05-001 Boundary

`L5-A-05-001` owns the family baseline.

A05-002 specializes only design-time/effective-definition concerns.

If a concern is family-wide but not definition/version-specific, A05-001 remains authoritative.

### 4.4 A05-003 Boundary

A05-003 consumes:

- resolved immutable Workflow Version;
- validated runtime references;
- definition/version lineage;
- migration admission result where migration is supported.

A05-002 does not own runtime task/timer/retry/compensation mechanics.

### 4.5 D02 Policy Boundary

D02 owns:

- Authorization Policy;
- policy-version resolution for authorization;
- Permission/Role/scope/SoD semantics;
- final Authorization Decision.

A05-002 owns only:

- Workflow Definition policy references;
- technical validation that required references are resolvable/compatible;
- supplying workflow/Tenant/resource context to D02;
- preserving the exact returned policy/version decision reference where required for evidence.

### 4.6 C01/C02 Boundary

A05-002 may reference:

- synchronous capability contracts from C01;
- canonical event contracts from C02.

C01/C02 own:

- contract transport/compatibility mechanics;
- event schema/versioning;
- delivery infrastructure.

A05-002 owns whether the Workflow Version structurally references those contracts/events and whether the source-required compatibility checks pass.

### 4.7 B02 Boundary

B02 owns physical persistence, transaction and concurrency patterns.

A05-002 owns the authoritative A05 definition/version data.

No other owner may mutate A05 definition/version persistence directly.

### 4.8 Cross-Owner Write Rule

A05-002 may record references to:

- domain commands/capabilities;
- policy identifiers;
- event contracts;
- input/output contracts;
- resources.

It shall not directly modify the authoritative records behind those references.

---

## 5. Solution Architecture

### 5.1 Logical Design-Time Architecture

```text
Workflow-Owning Domain
        ↓
governed definition/change request
        ↓
C01 Contract Boundary
        ↓
Definition Admission Boundary
        ↓
Mutable Draft Repository
        ↓
Definition Validation Pipeline
  ├─ Structural Validator
  ├─ Ownership/Boundary Validator
  ├─ Policy Reference Validator
  ├─ Event Reference Validator
  ├─ Contract/Dependency Validator
  ├─ Version/Compatibility Validator
  └─ Tenant/Scope Validator
        ↓
source-governed approval outcome
        ↓
Version Publication Boundary
        ↓
Immutable Workflow Version Repository
        ↓
Activation Boundary
        ↓
Active Version Resolution Index
        ↓
A05-003 Runtime
```

### 5.2 Core Technical Elements

| Element | Responsibility |
|---|---|
| Definition Admission Boundary | Resolves Tenant, owning domain and approved process boundary before Draft creation/change. |
| Mutable Draft Repository | Stores the current editable A05-owned orchestration definition before publication. |
| Definition Structural Model | Represents metadata, nodes, connectors, variables, input/output contracts and governed references. |
| Reference Registry Adapter | Resolves policy/event/contract/domain capability references through canonical owners. |
| Definition Validation Pipeline | Produces deterministic validation results required before approval/publication. |
| Workflow Version Builder | Creates immutable version snapshot from an approved valid definition. |
| Version Repository | Stores immutable published versions and lineage. |
| Activation Boundary | Ensures only valid Published versions become Activated. |
| Active Version Resolver | Deterministically resolves the version eligible for new instances. |
| Policy Reference Resolver | Resolves policy identifiers through authoritative policy owner without embedding implementation. |
| Compatibility Analyzer | Evaluates source-required compatibility between successor, contracts, dependencies and active-instance impact. |
| Migration Admission Boundary | Validates all source-defined migration prerequisites without executing runtime migration. |
| Evidence Handoff | Supplies validation/approval/publication/activation/migration evidence to D04. |
| Publication Handoff | Publishes Workflow Engine-owned definition/version events through B02/C02 governed infrastructure. |

### 5.3 Conceptual Workflow Definition Model

Conceptually:

```text
WorkflowDefinition
{
  workflow_definition_reference
  tenant_reference
  workflow_owning_domain_reference

  metadata
  nodes[]
  connectors[]
  variables[]

  workflow_input_contract_reference
  workflow_output_contract_reference

  domain_capability_references[]
  policy_references[]
  event_references[]
  dependency_references[]

  lifecycle_state
  current_draft_revision_reference
  correlation / provenance
}
```

This is not a prescribed schema.

The model contains orchestration metadata and references.

It shall not contain copied business-rule implementation as A05-owned truth.

### 5.4 Conceptual Workflow Version Model

```text
WorkflowVersion
{
  workflow_version_reference
  workflow_definition_reference
  tenant_reference
  owning_domain_reference

  predecessor_version_reference where applicable
  immutable_orchestration_snapshot

  input_contract_reference
  output_contract_reference

  policy_references[]
  event_references[]
  dependency_references[]

  validation_evidence_reference
  governance_approval_reference

  lifecycle_state
  publication_reference
  activation_reference

  lineage / provenance
}
```

No version-number field format is prescribed.

### 5.5 Definition Components

Consistent with locked L2-A05, a definition may represent:

| Component | L5 Interpretation |
|---|---|
| Metadata | workflow identity, owning-domain and orchestration configuration metadata. |
| Nodes | technical orchestration steps referencing owner capabilities or A05 runtime constructs. |
| Connectors | permitted transitions/relationships among workflow nodes. |
| Variables | runtime orchestration data contracts; not shadow domain truth. |
| Input Contract | governed initial workflow input boundary. |
| Output Contract | governed workflow output boundary. |
| Policy References | identifiers/references to policy owned elsewhere. |
| Event References | identifiers/references to canonical event contracts. |
| Dependency References | contracts/capabilities required by the workflow. |

### 5.6 Node Ownership Rule

A node may represent orchestration intent such as:

- invoke an owning-domain capability;
- wait for an event;
- create/await a human task;
- wait for time;
- branch using an authoritative result;
- invoke source-defined compensation.

The node does not become owner of the business rule behind the outcome.

Example:

```text
INVALID A05 ownership:
"If CGPA >= 7.5 approve scholarship"

VALID orchestration reference:
"Call ScholarshipEligibility capability
 → use authoritative Eligible / NotEligible result
 → follow version-defined route"
```

### 5.7 Connector / Transition Rule

Connectors represent orchestration paths.

They may reference transition conditions whose authoritative values come from:

- owning-domain results;
- D02 authorization/policy results;
- canonical events;
- A05 runtime facts;
- source-defined timer/task facts.

A connector shall not independently re-implement domain policy.

### 5.8 Draft Mutability

`Draft` is editable.

Changes to Draft can update:

- structure;
- references;
- metadata;
- contracts;
- dependencies;

subject to ownership and validation constraints.

Draft mutability does not imply that a previously Published version changes.

### 5.9 Validation State

`Validation` represents source-governed structural/policy validation.

The pipeline shall generate a complete validation result before progression.

A failed mandatory validation result prevents the definition/version from being treated as Approved/Published/Activated.

### 5.10 Validation Pipeline

Conceptually:

```text
Draft
  ↓
Ownership & Tenant Validation
  ↓
Structural Validation
  ↓
Input / Output Contract Validation
  ↓
Domain Capability Reference Validation
  ↓
Policy Reference Validation
  ↓
Event Reference Validation
  ↓
Dependency Validation
  ↓
Version / Compatibility Validation
  ↓
Complete Validation Result
```

The order may be optimized in implementation, but the final result must be deterministic and cover every applicable mandatory category.

### 5.11 Ownership Validation

Before publication, the system confirms:

- one Tenant;
- identified Workflow-Owning Domain;
- no business-rule ownership transferred to A05;
- referenced capabilities/events/policies have canonical owners;
- A05 does not claim another owner’s command/event/state.

### 5.12 Structural Validation

Structural validation may check architecture-level conditions such as:

- definition has valid entry/termination structure as required by source model;
- node references are well formed;
- connector targets exist;
- required input/output contracts are present;
- referenced A05 runtime constructs are supported;
- no invalid graph/reference relation prevents deterministic execution.

This specification does not prescribe a specific graph notation or algorithm.

### 5.13 Policy Reference Validation

A Workflow Definition references policy identifiers rather than implementations.

Validation determines whether the reference is:

- syntactically/technically resolvable;
- owned by the correct canonical owner;
- valid for the intended Tenant/scope/purpose where source semantics permit that determination;
- compatible with the intended workflow use.

A05 does **not** validate the truth of the policy's business content.

### 5.14 Policy Resolution Architecture

The core policy-resolution pattern is:

```text
Workflow Version
    contains policy identifier/reference
        ↓
runtime or protected lifecycle operation
        ↓
A05 builds authoritative workflow context
        ↓
D02 / canonical policy owner resolves applicable policy version
        ↓
D02 evaluates policy
        ↓
policy-version-bound result returned
        ↓
A05 uses result only as orchestration input
```

### 5.15 Policy Version Independence

A Workflow Version is not automatically republished when an external policy changes.

This prevents unnecessary workflow-version churn and preserves ownership boundaries.

Example:

```text
Workflow structure unchanged
Authorization Policy version changes
        ↓
Workflow Version remains immutable and unchanged
        ↓
future protected operation resolves/evaluates authoritative current policy
```

unless the source-governed policy reference explicitly requires another binding behavior.

### 5.16 Policy Pinning

This architecture does not establish universal policy pinning.

Where upstream behavior requires exact policy-version binding, the responsible policy architecture supplies the exact version/reference used for the decision.

A05 preserves that reference in runtime/evidence.

A05 shall not invent a rule that all policy references are pinned to the version active when the Workflow Version was published.

### 5.17 Event Reference Validation

For referenced events:

- event identity must be canonical;
- source publisher ownership must remain intact;
- the active/compatible contract must be resolvable according to C02;
- A05 must not fabricate an event name because the workflow wants a wait condition;
- event schema evolution remains C02-owned.

### 5.18 Contract Reference Validation

For synchronous capability contracts:

- provider owner must be resolvable;
- released/eligible contract must exist;
- required input/output compatibility must be satisfied;
- A05 definition does not expose provider persistence;
- C01 version/compatibility rules remain authoritative.

### 5.19 Dependency Validation

A Workflow Version may depend on multiple platform/domain capabilities.

Dependency validation ensures required references are available and compatible enough for the source-defined workflow to be published/activated.

A05 does not turn those dependencies into upstream implementation coupling.

### 5.20 Approval Boundary

A05 consumes a source-governed governance approval result.

A05-002 does not invent:

- approver role;
- approval count;
- approval hierarchy;
- approval command;
- approval event.

Until required approval exists, the definition shall not progress to the source-defined Approved/Published behavior.

### 5.21 Publication

Publication creates an immutable Workflow Version.

Conceptually:

```text
Approved valid definition
        ↓
capture immutable orchestration snapshot
        ↓
assign stable Workflow Version identity
        ↓
persist version + lineage + evidence
        ↓
create durable WorkflowVersionPublished publication intent
        ↓
commit owner-local state
        ↓
C02 distributes canonical event
```

### 5.22 Publication Immutability

After `Published`:

- structure cannot be edited in place;
- policy reference identity cannot be silently replaced;
- event/contract references cannot be silently rewritten;
- input/output contracts cannot be silently changed;
- lineage/history remains preserved.

Any material definition change uses a successor Draft.

### 5.23 Publication Transaction Boundary

A05 owns the publication transaction for A05 definition/version state.

The transaction may include:

- new immutable Workflow Version;
- predecessor/successor linkage;
- publication evidence reference;
- workflow-event publication intent.

It shall not include another owner’s persistence transaction.

`ADR-L5-0007` governs durable state-to-publication handoff.

### 5.24 Activation

A version becomes available for new workflow execution only after source-defined activation conditions succeed.

The activation boundary confirms at minimum:

- version is validly Published;
- Tenant/owning-domain context is valid;
- required activation authority/outcome exists where upstream requires it;
- no mandatory dependency/compatibility condition has become invalid;
- activation does not mutate version contents.

### 5.25 Active Version Resolution

For a new Workflow Instance, A05 must deterministically resolve the eligible version.

Conceptually:

```text
Tenant
+ Workflow Definition identity
+ Workflow-Owning Domain
+ requested orchestration context
        ↓
Active Version Resolver
        ↓
one eligible Activated Workflow Version
```

If required context is missing, inconsistent or resolves ambiguously, new execution shall not start.

### 5.26 Published vs Activated

`Published` and `Activated` are not synonyms.

A Published version is immutable and available as a governed version artifact.

An Activated version is available as the current source-governed version for new workflow instances.

A resolver shall not treat every Published version as automatically current.

### 5.27 Explicit Version Selection

If an upstream contract explicitly supplies a Workflow Version:

- A05 validates that the version exists;
- version belongs to the correct Workflow Definition/Tenant;
- its lifecycle makes it eligible for the intended operation;
- caller authority is valid where protected;
- runtime binds the resulting instance to that exact version.

The baseline does not create a universal privilege to start arbitrary retired/historical versions.

### 5.28 Resolution Ambiguity

The active-version resolution result must be deterministic.

If the canonical data/configuration produces multiple incompatible “current” versions where only one should be eligible:

- do not choose arbitrarily;
- fail the resolution;
- surface a configuration/governance defect.

### 5.29 Active Version Cache

Implementations may cache active-version resolution.

Cache entries are derivative.

They must bind at least to materially relevant context such as:

- Tenant;
- Workflow Definition;
- applicable scope/context where required;
- version/reference freshness.

Activation/retirement/change must invalidate or make stale cached results non-authoritative.

No cache product or TTL is selected.

### 5.30 Successor Version Creation

A change to a Published/Activated workflow follows:

```text
immutable predecessor
        ↓
new independent successor Draft
        ↓
validation
        ↓
approval
        ↓
Published successor
        ↓
Activated successor
```

The predecessor is never rewritten.

### 5.31 Version Lineage

A successor retains lineage to its predecessor.

Lineage supports:

- auditability;
- impact analysis;
- compatibility review;
- active-instance assessment;
- migration assessment.

This architecture does not prescribe semantic version numbers.

### 5.32 Successor Activation

Activating a successor changes which version is eligible for **new** instances according to upstream rules.

It does not:

- mutate predecessor;
- rewrite historical evidence;
- move existing instances;
- change business outcomes already produced.

### 5.33 Existing Instance Binding

Existing Workflow Instances remain on the originating version by default.

A05-003 resolves the runtime version directly from the instance binding, not from “current active version.”

This prevents active-version changes from altering in-flight execution.

### 5.34 Active-Instance Impact Analysis

Before successor activation/migration decisions, the architecture can identify:

- active instance counts/references by originating version;
- structural changes relevant to execution;
- changed contract/event/policy references;
- compatibility concerns;
- migration candidates.

This is technical analysis, not migration approval.

### 5.35 Migration Admission

Migration is exceptional.

A05-002 only determines whether a migration request has all source-required prerequisites.

From L4-A-017-BH-008:

- Migration Plan;
- Compatibility Validation;
- Governance Approval;
- Administrator Authorization;
- Audit Recording.

If any is absent or invalid, migration is not admitted.

### 5.36 No Invented Migration Lifecycle

No canonical state such as:

```text
Migrating
MigrationPending
MigrationApproved
```

is created unless later upstream architecture explicitly defines it.

Likewise no `WorkflowInstanceMigrated` event is invented.

### 5.37 Unsafe Migration

If migration is:

- unsafe;
- rejected;
- unauthorized;
- incompatible;

the instance remains on its originating version.

A05-002 shall not “best effort” migrate.

### 5.38 Migration Execution Boundary

A05-002 owns migration admission and compatibility/version mapping inputs.

A05-003 owns runtime mechanics if migration is implemented.

Neither file may rewrite already completed domain outcomes.

### 5.39 Retirement

`Retired` is a canonical Workflow Definition/Version lifecycle concept from L2-A05.

Retirement prevents inappropriate new use according to source semantics.

This specification does not assume retirement cancels existing instances.

Existing instance behavior remains governed by its originating version and explicit source rules.

### 5.40 Definition Concurrency

Concurrent Draft edits/publication attempts must not:

- overwrite a newly published successor;
- publish two conflicting immutable versions from the same expected Draft revision without governed semantics;
- lose predecessor/successor lineage.

Implementation may use B02-governed optimistic concurrency or another approved owner-local mechanism.

### 5.41 Publication Idempotency

Repeated processing of the same governed publication request shall not create multiple equivalent Workflow Versions merely because of transport retry.

The operation shall be idempotent within its governed request identity/context.

Exact idempotency-key encoding remains C01/B02 implementation detail.

### 5.42 Activation Concurrency

Concurrent activation attempts must yield one authoritative active-resolution outcome under source semantics.

A stale activation request shall not silently overwrite a newer authoritative activation.

### 5.43 Policy Reference Freshness

A cached policy reference/result is not permanent authorization truth.

When a protected lifecycle/runtime operation occurs, D02 decides whether policy re-resolution/re-evaluation is required.

A05 shall not preserve stale policy Allow solely because it was valid when the Workflow Version was activated.

### 5.44 Policy Resolution Failure

If a mandatory policy reference cannot be resolved or evaluated:

- do not guess a default policy;
- do not treat missing policy as Allow;
- stop the protected progression according to D02/source behavior;
- preserve failure evidence.

### 5.45 Contract Evolution

Integration contracts evolve independently.

If a referenced contract changes:

- C01/C02 compatibility rules determine whether existing reference remains usable;
- A05 validation/impact analysis consumes that result;
- a workflow successor is needed only when the workflow-owned structure/reference must change.

### 5.46 Event Evolution

Event contract evolution remains C02-owned.

A05 references canonical event identity/version compatibility but does not own serializer/schema upgrade.

### 5.47 Tenant Isolation

Every definition/version is Tenant-bound under current L4 A014/A017 context.

Resolution shall not:

- return another Tenant’s workflow version;
- validate another Tenant’s policy as local authority;
- reuse cached active-version results across Tenants;
- permit migration across Tenants.

### 5.48 Institution / Lower Scope

Where a workflow definition or referenced policy is Institution/lower-scope-specific:

- A04/D02 supply authoritative scope;
- A05 records/uses the scope reference required for deterministic selection;
- A05 does not invent hierarchy or scope inheritance.

### 5.49 Evidence

A05-002 supplies D04 with references sufficient to reconstruct:

- definition identity;
- Tenant;
- owning domain;
- Draft/change provenance;
- validation results;
- approval reference;
- published immutable version;
- predecessor/successor lineage;
- activation outcome;
- policy/event/contract references;
- active-instance impact;
- migration admission/denial;
- canonical definition/version events.

### 5.50 Failure Model

| Failure / Condition | Required Architecture Response |
|---|---|
| Tenant unresolved | Reject definition/version lifecycle action. |
| Workflow-Owning Domain unresolved | Reject progression. |
| business-process boundary unresolved | Reject progression. |
| structural validation fails | Remain outside Approved/Published. |
| policy reference unresolved | Validation/protected progression fails; no default policy. |
| event reference unresolved/incompatible | Validation fails. |
| contract/dependency incompatible | Validation fails. |
| required governance approval absent | Do not publish. |
| attempted in-place edit of Published version | Reject; require successor Draft. |
| publication persistence fails | Do not claim Published. |
| event delivery fails after committed publication | Preserve durable publication intent; C02 recovery. |
| activation attempted on non-Published version | Reject. |
| active-version resolution ambiguous | Fail resolution; do not select arbitrarily. |
| stale active-version cache | Revalidate/invalidate; do not use stale result as authority. |
| concurrent publication conflict | Use owner-local concurrency control; preserve one authoritative lineage result. |
| successor activated | Existing instances remain on originating version. |
| migration prerequisites incomplete | Do not admit migration. |
| migration unsafe/rejected/unauthorized | Instance remains on originating version. |
| policy changes | Do not mutate immutable workflow version; future policy evaluation follows canonical policy owner. |

---

## 6. Architecture & Technology Decisions

Only A05-002-owned technical choices are recorded here.

| Decision ID | Decision | Rationale / Authority |
|---|---|---|
| `AD-001` | Workflow Definition and immutable Workflow Version are separate technical representations: mutable design-time Draft versus immutable published execution artifact. | Required to realize L2-A05 lifecycle and L4-A014/A017 without mutating history. |
| `AD-002` | Published Workflow Versions are stored/resolved through stable version identity and explicit predecessor/successor lineage rather than in-place replacement. | L4-A017 successor semantics and impact analysis. |
| `AD-003` | A05 uses reference-based composition for domain capabilities, policies, events and contracts; canonical implementations remain peer-owned. | Ownership preservation and independent evolution. |
| `AD-004` | Policy references are resolved through the canonical policy owner at the protected operation/evaluation boundary; policy evolution does not automatically version the workflow. | Locked L2-A05 §130 + D02 policy architecture. |
| `AD-005` | New instance version resolution uses an explicit Active Version Resolver; existing instances use their persisted originating-version binding instead of the current-active lookup. | Prevents silent in-flight behavior change. |
| `AD-006` | Workflow publication state and workflow-event publication intent use owner-local atomic handoff rather than distributed transaction with C02. | `ADR-L5-0007`. |
| `AD-007` | Migration admission is separated from migration execution and remains deny-by-default when any source-required safeguard is absent. | L4-A017-BH-008/BH-009 and ownership preservation. |
| `AD-008` | BPMN 2.x + Flowable OSS 8.x are canonical under `ADR-L5-0023`; Workflow Version numbering and registry/storage implementation remain separately governed. | Locked ADR + implementation boundary. |

---

## 7. Security, Privacy, Quality & Operational Impact

### 7.1 Security

- Definition lifecycle operations are protected through D02.
- Activation and migration administration are privileged where upstream/policy requires.
- Tenant context is mandatory.
- Policy references cannot be supplied by an untrusted client and accepted without canonical validation.
- Definition references cannot bypass D02 or owning-domain contracts.
- Historical versions are immutable.
- No workflow definition may embed credentials/secrets.

### 7.2 Policy Security

A05 never caches “policy says Allow” as permanent workflow configuration.

The authoritative policy owner decides current policy applicability.

Exact policy version evaluated by D02 is preserved for evidence when relevant.

### 7.3 Privacy

Workflow definitions should contain references and orchestration metadata, not unnecessary sensitive business payloads.

Validation/evidence should avoid copying confidential business data where identifiers/references are sufficient.

### 7.4 Quality

Architecture quality depends on:

- deterministic validation;
- immutable publication;
- deterministic active-version resolution;
- explicit lineage;
- no silent migration;
- independently evolvable policy/contract/event references.

### 7.5 Operational Impact

Operations may need visibility into:

- Draft/Validation/Approved/Published/Activated/Retired distribution;
- activation conflicts;
- stale resolution caches;
- dependency incompatibilities;
- unpublished approved definitions;
- migration candidates;
- instances per originating version.

Operational views are projections, not authoritative definition state.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Requirement |
|---|---|
| `CR-001` | Every Workflow Definition shall identify its Tenant and Workflow-Owning Domain before governed progression. |
| `CR-002` | A05 definition/version content shall represent orchestration structure and references, not become canonical owner of business rules, decisions, states, commands or domain events. |
| `CR-003` | Published Workflow Versions shall be immutable. |
| `CR-004` | A structural/reference change to a Published Workflow Version shall create an independent successor Draft rather than editing the predecessor. |
| `CR-005` | Every successor version shall preserve predecessor/successor lineage. |
| `CR-006` | Publication shall be blocked until all source-required mandatory validation categories and required governance approval have succeeded. |
| `CR-007` | Activation shall reject a version that is not validly Published. |
| `CR-008` | Definition activation shall not itself create or start a Workflow Instance. |
| `CR-009` | New Workflow Instances shall resolve an eligible workflow version deterministically; ambiguous active-version resolution shall fail rather than choose arbitrarily. |
| `CR-010` | Existing Workflow Instances shall resolve their originating version directly and shall not silently follow the latest/active successor. |
| `CR-011` | Workflow Definitions shall reference policy identifiers/references rather than embed canonical policy implementations. |
| `CR-012` | Policy evolution shall not by itself mutate or republish an immutable Workflow Version. |
| `CR-013` | Protected policy evaluation shall be performed by the canonical policy/authorization owner and the resulting exact policy-version reference shall be preserved where required for evidence. |
| `CR-014` | A missing/unresolved mandatory policy reference shall not default to permissive workflow behavior. |
| `CR-015` | Event references shall resolve to canonical event contracts/publishers and A05 shall not invent event identity to satisfy a definition. |
| `CR-016` | Synchronous capability references shall use C01-governed contracts and shall not expose/directly mutate provider persistence. |
| `CR-017` | Contract/event schema evolution shall remain owned by C01/C02; A05 shall consume compatibility results rather than redefine them. |
| `CR-018` | A05 version publication shall preserve owner-local transactional integrity between authoritative version state and required workflow-event publication intent. |
| `CR-019` | Workflow version/event delivery infrastructure shall not become semantic owner of Workflow Definition/Version lifecycle. |
| `CR-020` | Active-version resolution caches shall be Tenant/context/freshness bound and shall not return stale or cross-Tenant versions as authoritative. |
| `CR-021` | Concurrent definition publication/activation shall use owner-local concurrency control and shall not overwrite newer authoritative version/activation state. |
| `CR-022` | Duplicate processing of one governed publication request shall not create duplicate equivalent Workflow Versions. |
| `CR-023` | Successor activation shall not alter the originating version of existing Workflow Instances. |
| `CR-024` | Instance migration shall not be admitted unless every source-required Migration Plan, Compatibility Validation, Governance Approval, Administrator Authorization and Audit Recording prerequisite exists. |
| `CR-025` | Unsafe, rejected or unauthorized migration shall leave the affected instance on its originating Workflow Version. |
| `CR-026` | A05 shall not invent migration lifecycle states or migration events absent upstream authority. |
| `CR-027` | Workflow definition/version artifacts shall not contain credentials, secret values or private-key material. |
| `CR-028` | No BPM/workflow product, notation, version-numbering scheme, database, cache or policy engine shall be inferred as canonical from this specification. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| `OD-001` | **Resolved notation:** BPMN 2.x. Exact file serialization/packaging conventions beyond the BPMN standard remain implementation-governed. | Technology / contract | Locked notation. | `ADR-L5-0023`. |
| `OD-002` | Exact Workflow Version numbering scheme. | Local representation | Non-blocking. | Select during implementation unless repository-wide semantics require ADR. |
| `OD-003` | Exact definition/version persistence schema and indexing. | B02/A05 implementation | Non-blocking. | Define under B02 ownership/isolation/concurrency constraints. |
| `OD-004` | Exact active-version cache implementation and invalidation transport. | Runtime optimization | Non-blocking. | Select without weakening deterministic/fresh resolution. |
| `OD-005` | **Resolved runtime:** Flowable OSS 8.x. Definition-registry persistence/storage implementation remains A05/B02-governed. | Technology | Locked runtime. | `ADR-L5-0023`; `ADR-L5-0042`. |
| `OD-006` | Exact generic migration execution algorithm/state transfer mechanism. | A05-003 runtime | Non-blocking for A05-002; feature-blocking if migration is implemented. | A05-003 must preserve A017 safeguards and cannot invent lifecycle/event. |
| `OD-007` | Canonical governance approver, publication/activation authority and migration administrator actor remain undefined upstream. | Upstream business authority | Non-blocking for reusable technical architecture; blocking for any concrete feature needing those actors. | Preserve source-owned gap; do not invent in L5. |
| `OD-008` | Whether specific workflow policy references require policy-version pinning rather than authoritative-current resolution. | Upstream/policy semantics | Non-blocking baseline; feature-specific. | Canonical policy owner must specify when required. |
| `OD-009` | **Resolved:** runtime/product ADRs exist and are Locked. | ADR governance | Closed. | `ADR-L5-0023`; `ADR-L5-0042`. |

---

## 10. Traceability Summary

### 10.1 Canonical Responsibility

`L5-A-05-002` owns the technical architecture for:

- Workflow Definition representation;
- immutable versioning;
- validation;
- publication;
- activation;
- effective version resolution;
- policy-reference resolution;
- successor lineage;
- migration admission safeguards.

### 10.2 Exact L4 Primary Realization

**L4-A-014:**

`BH-001` through `BH-010` are materially realized through definition admission, validation, approval consumption, immutable publication, activation, no-instance-start boundary and evidence.

**L4-A-017:**

`BH-001` through `BH-011` are materially realized through successor Draft creation, validation, immutable publication, activation, existing-instance protection, migration admission/denial and evidence.

### 10.3 Supporting L4 Runtime Relationship

`L4-A-015-BH-001/BH-002` consume the resolved eligible immutable Workflow Version.

`L4-A-016-BH-003/BH-006` depend on preserving the originating version throughout failure/recovery.

### 10.4 L2 Policy Resolution Rule

```text
Policy evolution is independent of Workflow Definition versioning.

Workflow Definitions reference policy identifiers
rather than concrete implementations.
```

A05-002 realizes this with reference-based policy resolution through D02/canonical policy ownership.

### 10.5 Cross-L5 Flow

```text
Workflow-Owning Domain
        ↓
C01 governed definition/change contract
        ↓
A05-002 definition + validation + immutable versioning
        ├── D02 policy / authorization references
        ├── C01 contract references
        ├── C02 event references
        ├── A04 scope context
        └── D04 evidence
        ↓
Activated immutable Workflow Version
        ↓
A05-003 runtime
```

### 10.6 Applicable Approved ADRs

- `ADR-L5-0003`
- `ADR-L5-0004`
- `ADR-L5-0006`
- `ADR-L5-0007`
- `ADR-L5-0010`
- `ADR-L5-0011`

### 10.7 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-A/
            └── L5-A-05-002_Workflow_Definition_Versioning_and_Policy_Resolution_Architecture.md
```

No `L5-A/L5-A-05/` major-area subfolder is introduced.


## 2026-09-22 Backend Readiness Group 4 Synchronization

**Applicable locked decision:** `ADR-L5-0042`  
Flowable stores orchestration/runtime state only; canonical business state remains source-owned. Human task assignment is not authorization and D02 re-evaluates authority at action time. Workflow timers stay A05-owned, running instances are version-pinned by default, migration is explicit, and compensation invokes only owner-provided actions.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Removed stale product/notation deferrals now resolved by the Flowable/BPMN ADRs. Upstream publication authority, policy-pinning semantics and implementation-level version/index/cache details remain intentionally open.
