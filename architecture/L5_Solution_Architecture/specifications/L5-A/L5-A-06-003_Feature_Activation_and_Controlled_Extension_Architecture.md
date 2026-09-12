# Feature Activation & Controlled Extension Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-06-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-06 — Configuration, Feature & Institutional Extensibility Architecture  
**Canonical Technical Responsibility:** Safe capability activation/deactivation and controlled extension boundaries.  
**Technical Architecture Owner:** L5-A-06 — Configuration, Feature & Institutional Extensibility Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-12`

**Depends On:**

- `L5-A-06-001` — Configuration & Institutional Extensibility Architecture Baseline;
- `L5-A-06-002` — Configuration Scope, Inheritance & Effective-Dating Architecture;
- `L5-A-03-001` — Module, Dependency & Technical Boundary Architecture;
- `L5-A-04-001` — Tenant & Institutional Isolation Architecture;
- `L5-A-04-002` — Tenant & Institution Context Propagation & Data Isolation Architecture;
- `L5-B-02-003` — Transaction, Consistency & Concurrency Architecture;
- `L5-D-03-001` — Security Architecture & Trust Boundary Baseline;
- `L5-E-04-001` — Runtime, Packaging & Deployment Architecture Baseline;
- `L5-E-04-002` — Application Packaging & Runtime Isolation Architecture;
- Locked `L2-A08` — Module Registry & Configuration Workflows;
- applicable L4 Module Registry behavior;
- L5 governance, validation, traceability and ADR standards.

**Collaborates With:**

- L5-D-01 — technical/workload identity where required;
- L5-D-02 — authorization decisions and protected-operation enforcement;
- L5-D-03-004 — secrets/certificates/machine credentials;
- L5-D-04 — technical evidence;
- L5-C-01 — governed technical contracts;
- L5-C-02 — event delivery where canonical events exist.

**Applicable ADRs:**

- `ADR-L5-0001` — Canonical Trust Boundary & Segmentation Pattern;
- `ADR-L5-0003` — Contract-Driven Modular Platform with Selective Isolation;
- `ADR-L5-0004` — Tenant & Institution Isolation Enforcement Pattern;
- `ADR-L5-0006` — Owner-Local Transaction & Optimistic Concurrency Strategy.

---

# 2. Purpose, Scope & Non-Goals

## 2.1 Purpose

This specification defines the technical architecture through which SutraERP safely:

- activates or deactivates activation-controlled capabilities;
- applies activation at governed Tenant/Institution/other permitted scopes;
- admits runtime invocation only when technical prerequisites remain valid;
- activates controlled extensions only against registered Extension Points;
- binds extension execution to an exact compatible contract and extension version;
- constrains extension authority, dependencies and trust;
- prevents runtime activation from becoming deployment, business-policy ownership or Module Registry lifecycle ownership.

---

## 2.2 Critical Boundary

There are two different concepts:

```text
Module Registry lifecycle activation
             ≠
runtime feature / extension activation
```

Module Registry owns:

- Module;
- Module Version;
- architectural certification;
- `Active`, `Suspended`, `Deprecated`, `Retired`;
- canonical Module lifecycle events.

A06-003 owns:

- technical activation control;
- scope-bound activation state;
- runtime invocation admission;
- controlled extension activation;
- controlled extension execution boundary.

An upstream `Module Activated` fact shall not itself be interpreted as deployment or executable runtime start.

---

## 2.3 In Scope

- technical Activation Target;
- Activation Policy;
- scope-aware activation;
- activation/deactivation state;
- activation version/history;
- compatibility and dependency gating;
- runtime admission;
- activation concurrency protection;
- extension runtime descriptor;
- exact extension-version binding;
- Extension Point/Contract validation;
- extension trust boundary;
- extension dependency restrictions;
- extension composition/multiplicity semantics;
- in-flight execution behavior during ordinary deactivation;
- controlled failure/containment;
- evidence/provenance handoff.

---

## 2.4 Out of Scope

- Module Registry business lifecycle ownership;
- business decision to approve/certify/suspend/deprecate/retire a Module;
- Configuration Definition registration — A06-001;
- configuration hierarchy/effective dating — A06-002;
- authentication — D01;
- authorization policy ownership — D02;
- trust/segmentation ownership — D03;
- secrets/credentials — D03-004;
- database technology — B02;
- deployment/packaging technology — E04;
- CI/CD/release mechanics — F01;
- canonical event invention;
- UI administration — L6;
- business compensation/recovery rules.

---

# 3. Authoritative Basis & Constraints

## 3.1 Primary Behavioral Basis

| Source | Exact Applicability | A06-003 Effect |
|---|---|---|
| `L4-A-028` | `BH-007`–`BH-009` | Architectural activation requires certified/compatible eligible version; activation is not runtime deployment. |
| `L4-A-029` | `BH-003`–`BH-007` | Invalid successor remains inactive; Active/Transitional successor use requires governed compatibility and bounded transition context. |
| `L4-A-030` | `BH-001`–`BH-007` | Suspension/restoration remain governed Module Registry lifecycle with preserved version/dependency state. |
| `L4-A-031` | `BH-001`–`BH-007` | Deprecation/retirement require controlled withdrawal; Retired cannot return to Active. |
| `L5-A-06-001` | Extension Point Registry, Extension Contract and Admission Boundary | Supplies registered extension boundary and compatibility baseline. |
| `L5-A-06-002` | Resolution Snapshot and scope/effective resolution | Supplies one deterministic scope-aware activation/configuration context where activation varies by scope/time. |
| `L5-A-03-001` | public technical boundaries | Prohibits peer-internal and cross-owner persistence bypass. |
| `L5-D-03-001` | trust boundary / fail-closed admission | Extension execution is explicit, contextual and non-transitively trusted. |
| `L5-E-04-001/002` | runtime placement/isolation | Physical execution and isolation remain runtime-owned. |

---

# 3.2 Mandatory Constraints

1. Technical activation shall not modify Module Registry lifecycle state.
2. Module Registry activation shall not imply software deployment.
3. An activation-controlled target shall have an explicit technical activation policy.
4. Runtime admission shall preserve validated Tenant/Institution scope.
5. Controlled extensions shall attach only to registered Extension Points.
6. Extension execution shall use the compatible governed Extension Contract.
7. Cross-owner direct persistence access remains prohibited.
8. Host process co-location shall not imply unrestricted extension trust.
9. Extension registration shall not grant authorization.
10. Mandatory security/trust prerequisites fail closed.
11. Runtime activation shall bind an exact target/version.
12. Runtime activation shall not silently select a different version when the selected version becomes unavailable/incompatible.
13. A06 shall not create new canonical Module lifecycle events.
14. Deployment, packaging and runtime-isolation technology remain E04-owned.

---

# 4. Technical Ownership & Boundaries

## 4.1 A06-003 Owns

- Activation Target technical model;
- Activation Policy;
- technical activation state;
- activation state history/version;
- activation eligibility evaluation;
- Runtime Admission Gate;
- Extension Runtime Descriptor;
- extension-version admission;
- extension composition rules;
- controlled extension invocation boundary;
- ordinary activation/deactivation runtime semantics.

---

## 4.2 A06-003 Does Not Own

- Module/Module Version lifecycle;
- business capability ownership;
- Module certification;
- dependency business meaning;
- Tenant/Institution lifecycle;
- authorization policy;
- trust classification policy;
- deployment topology;
- canonical business events;
- business recovery/compensation.

---

# 5. Solution Architecture

## 5.1 Technical Control Model

```text
          Authoritative Module / Capability State
                       │
                       ▼
                Activation Target
                       │
                       ▼
                Activation Policy
                       │
        ┌──────────────┴──────────────┐
        │                             │
        ▼                             ▼
 A06-002 Scope / Time          Eligibility Checks
 Resolution Snapshot          compatibility
        │                     dependencies
        │                     authorization
        │                     trust
        └──────────────┬──────────────┘
                       ▼
             Activation State Boundary
                       │
                       ▼
              Runtime Admission Gate
                       │
          ┌────────────┴────────────┐
          │                         │
          ▼                         ▼
   Native Capability        Controlled Extension
                                  │
                                  ▼
                         Extension Execution
                              Boundary
```

---

# 5.2 Activation Target

An **Activation Target** is a technical reference to an existing governed capability or extension that is allowed to participate in activation control.

It is not a new business capability entity.

Conceptually:

```text
ActivationTarget
{
    target identity reference
    governing Module / capability reference
    target version reference
    target category
    activation-policy reference
}
```

Permitted target categories include only architecture-authorized targets, such as:

- an activation-controlled registered capability;
- a controlled extension bound to a registered Extension Point.

A06-003 shall not transform arbitrary executable code into an Activation Target.

---

# 5.3 Activation Policy

Every activation-controlled target shall bind to an explicit **Activation Policy**.

Conceptually:

```text
ActivationPolicy
{
    target/version reference
    permitted activation scopes
    default activation behavior
    compatibility prerequisites
    dependency prerequisites
    extension rules where applicable
    ordinary deactivation semantics
}
```

The policy determines technical activation mechanics.

It shall not define business-policy meaning.

---

# 5.4 Default Activation Semantics

A06 shall never guess whether a target should be active.

An Activation Policy shall explicitly establish whether the target:

- requires explicit activation; or
- is technically active by default when all upstream eligibility prerequisites are satisfied.

If no valid Activation Policy exists for an activation-controlled target:

```text
effective activation = unavailable
```

This is fail-safe and prevents accidental feature exposure.

---

# 5.5 Scope-Aware Activation

Where activation varies by Tenant, Institution or another permitted configuration scope, A06-003 shall reuse A06-002.

It shall not create a second scope/inheritance algorithm.

```text
Activation Policy / State
        ↓
A06-002 Resolution Snapshot
        ↓
Effective activation for scope
        ↓
A06-003 Runtime Admission
```

Therefore:

- scope hierarchy;
- inheritance;
- overrides;
- effective dating;
- ambiguity handling;

remain A06-002-owned.

---

# 5.6 Technical Activation State

The minimum A06 technical state is:

```text
INACTIVE
ACTIVE
```

These are technical runtime-availability states and are not Module Registry lifecycle states.

A derived result may also be:

```text
UNAVAILABLE
```

when required eligibility prerequisites fail.

`UNAVAILABLE` is an admission result, not a new upstream Module lifecycle state.

---

# 5.7 Activation Record

An activation change shall be represented through a versioned technical Activation Record.

Conceptually:

```text
ActivationRecord
{
    Activation Target reference
    exact target version
    governing scope
    Activation Policy reference
    requested technical state
    predecessor activation version
    authoritative governance evidence reference
}
```

Physical representation belongs to B02.

Historical activation state shall not be destructively rewritten.

---

# 5.8 Activation Concurrency

Activation changes shall protect against lost updates.

```text
Activation State V1
      ├── Change A based on V1
      └── Change B based on V1

A commits → V2

B attempts commit
      ↓
expected predecessor != V2
      ↓
explicit conflict
```

The platform shall not silently overwrite the newer activation state.

Physical conflict control follows B02-003.

---

# 5.9 Activation Eligibility Gate

A target may become technically `ACTIVE` only when all applicable prerequisites succeed.

```text
Target registered?
      ↓
Exact target version eligible?
      ↓
Required upstream governance state valid?
      ↓
Dependencies / compatibility valid?
      ↓
Activation scope valid?
      ↓
Required authorization succeeds?
      ↓
Required trust/security prerequisites succeed?
      ↓
Runtime artifact/execution capability available?
      ↓
ACTIVE
```

Failure at any mandatory step prevents activation.

---

# 5.10 Module Registry Lifecycle Relationship

Module Registry lifecycle provides governance eligibility information.

A06-003 shall not rewrite it.

### Active / Certified Eligibility

Where the upstream Module architecture requires a certified/Active Module Version before capability activation, A06 shall require that condition.

### Suspended

A `Suspended` Module shall not be treated as eligible for a **new** technical activation unless the source-governed restoration succeeds.

However, L4 does not define whether suspension automatically terminates already-running runtime execution.

A06-003 therefore shall not invent automatic termination solely from the registry state.

Security containment may independently restrict execution through D03 where required.

### Deprecated

A Deprecated Module shall not receive new activation inconsistent with its source-governed withdrawal/transition strategy.

Existing runtime use shall follow the governing migration/retirement plan rather than an invented instantaneous shutdown.

### Retired

A Retired Module or retired version:

- shall not be newly activated;
- shall not be reactivated;
- remains historically traceable.

Runtime decommission realization follows the authoritative retirement plan and E04.

---

# 5.11 Activation Does Not Deploy Software

```text
Activation
   ≠
Deployment
```

Technical activation is permission/availability within an already-realizable runtime architecture.

If the required executable/runtime artifact is not available through E04/F01-governed realization:

```text
activation shall not fabricate deployment
```

The target remains unavailable.

---

# 5.12 Runtime Admission Gate

Activation is enforced at invocation admission.

For every activation-controlled invocation:

```text
Invocation
    ↓
Validated execution scope
    ↓
Activation Target/version
    ↓
Effective activation state
    ↓
Compatibility/dependency validity
    ↓
D02 authorization where required
    ↓
D03 trust admission
    ↓
E04 runtime availability
    ↓
ADMIT / REJECT
```

A stale client-side flag shall not override authoritative activation state.

---

# 5.13 Invocation Binding

An admitted invocation shall bind the exact technical context under which it was admitted.

Conceptually:

```text
ActivationExecutionContext
{
    target identity/version
    activation-state version
    protected execution scope
    A06-002 Resolution Snapshot reference
    configuration context reference
    extension descriptor/version where applicable
    Extension Contract version where applicable
    correlation/provenance context
}
```

This is not a physical API schema.

---

# 5.14 Ordinary Deactivation

Once an ordinary deactivation becomes authoritative:

```text
new invocation
      ↓
Runtime Admission Gate
      ↓
REJECT
```

Already-admitted execution retains its established execution context.

Ordinary deactivation shall not silently mutate a running execution to another feature/extension version.

---

# 5.15 Security / Emergency Containment

Ordinary deactivation and security containment are different mechanisms.

```text
ordinary deactivation
→ A06-003

security/trust compromise containment
→ D03 + applicable E04 enforcement
```

If D03 determines that an affected trust relationship must be contained, A06 shall not preserve execution merely because it was previously activated.

Mandatory security containment takes precedence for the affected protected operation.

A06 does not invent the business recovery outcome after containment.

---

# 5.16 Controlled Extension Descriptor

Each executable controlled extension shall have a stable technical descriptor.

Conceptually:

```text
ControlledExtensionDescriptor
{
    stable extension identity
    exact extension version
    Extension Point reference
    Extension Contract version
    immutable executable-artifact reference
    required public dependencies
    trust / isolation requirement reference
}
```

The descriptor does not make the extension a business-domain owner.

The executable artifact representation remains E04/F01-owned.

---

# 5.17 Extension Admission Preconditions

A controlled extension may be activated only if:

1. Extension Point exists.
2. Extension Point canonical owner remains valid.
3. exact Extension Contract version exists.
4. extension version is compatible with that contract.
5. applicable activation scope is valid.
6. dependencies are permitted.
7. required authorization succeeds.
8. D03 trust prerequisites succeed.
9. required executable artifact is available.
10. no upstream retirement/suspension condition prohibits new activation.

Failure of any mandatory prerequisite prevents activation.

---

# 5.18 Extension Trust Boundary

Every controlled extension executes through an explicit trust boundary.

Co-location does not remove this rule.

```text
Host Capability
      ↓
Extension Contract Boundary
      ↓
D03 Trust Admission
      ↓
Controlled Extension
```

An extension does not automatically inherit:

- host privileges;
- host Tenant breadth;
- host persistence access;
- host secrets;
- host internal APIs;
- unrelated trusted relationships.

Trust remains contextual and non-transitive.

---

# 5.19 Extension Dependency Boundary

An extension may interact only through explicitly permitted governed boundaries.

It shall not directly access:

- another technical owner's persistence;
- host-private implementation state;
- arbitrary internal modules;
- unrelated Tenant data;
- secrets outside its granted credential boundary.

Cross-parent interaction must conform to A03 and applicable C01/C02 contracts.

---

# 5.20 Extension Persistence Boundary

Extension activation does not create persistence ownership.

If an extension requires technical persisted state:

- one canonical technical owner must be established;
- B02 ownership/isolation rules apply;
- direct writes into host/peer authoritative persistence remain prohibited.

---

# 5.21 Extension Secrets & Credentials

Controlled extensions shall not receive raw secrets simply because they are activated.

Any:

- secret;
- certificate;
- machine credential;
- key material;

must be obtained through the applicable D03 mechanism under the extension's own permitted technical identity/trust context.

---

# 5.22 Exact Version Binding

Extension invocation shall bind:

- exact extension version;
- exact Extension Point;
- exact Extension Contract version.

The runtime shall not silently substitute:

- newer extension version;
- older extension version;
- different compatible-looking contract;
- alternate extension implementation;

unless the governing Activation Policy explicitly defines that substitution mechanism.

---

# 5.23 Extension Upgrade

A new extension version shall be treated as a distinct activation candidate.

```text
Extension V1 active
       ↓
V2 registered
       ↓
compatibility validation
       ↓
separate governed activation
       ↓
new invocations may use V2
```

Existing invocation contexts remain version-bound.

A06 shall not hot-swap executable extension behavior inside an already-admitted execution.

---

# 5.24 Extension Multiplicity

An Extension Point shall explicitly define whether it allows:

```text
SINGLE
```

or:

```text
MULTIPLE_WITH_EXPLICIT_COMPOSITION
```

Multiple simultaneously active extensions shall not be permitted when the Extension Point contract does not define multiplicity.

---

# 5.25 Multiple-Extension Composition

Where multiple extensions are permitted, the Extension Contract shall define sufficient technical composition semantics, including where applicable:

- execution ordering;
- independence/parallelism;
- aggregation semantics;
- short-circuit behavior;
- failure behavior;
- conflicting result handling.

A06 shall not use:

- registration order;
- database order;
- lexical extension ID;
- activation timestamp;

as an implicit extension execution order.

Undefined composition:

```text
→ activation rejected
```

---

# 5.26 Extension Invocation

```text
Host operation
      ↓
Extension Point reached
      ↓
Resolve active extension binding
      ↓
Verify exact descriptor/version
      ↓
Validate activation scope
      ↓
D03 trust admission
      ↓
Invoke governed Extension Contract
      ↓
Return contract-defined outcome
```

The host does not expose its internal implementation surface to the extension.

---

# 5.27 Extension Failure Semantics

An extension failure shall remain explicit.

A06 shall not silently:

- switch to another extension;
- retry side-effecting work;
- ignore an extension declared mandatory;
- reinterpret an error as business success.

Retry/fallback behavior is permitted only where the governing Extension Contract or owning business architecture explicitly establishes safe semantics.

---

# 5.28 Retry & Idempotency Boundary

If extension execution can be repeated due to retry/recovery:

- the contract must identify whether repetition is safe;
- side-effecting extension execution shall not be blindly retried;
- business compensation remains with the owning workflow/domain;
- transport retry remains with the applicable C/E architecture.

No retry count or timeout is invented here.

---

# 5.29 Resource / Runtime Containment

Where an extension is treated as restricted or higher-risk execution, applicable D03/E04 architecture may impose:

- stronger logical isolation;
- constrained runtime access;
- limited permitted dependencies;
- execution/resource controls;
- compromise containment.

A06 defines the requirement that those controls be satisfied before admission.

It does not select:

- process isolation;
- containers;
- WASM;
- scripting engine;
- plugin framework;
- JVM/classloader model;
- operating-system sandbox;
- cloud runtime.

A material selection of such a mechanism requires normal ADR governance.

---

# 5.30 Effective Activation Snapshot

One admitted execution shall use a coherent activation/configuration view.

```text
A06-002 Resolution Snapshot
        +
A06-003 Activation State Version
        +
Extension Descriptor Version
        ↓
ActivationExecutionContext
```

The invocation shall not mix:

- old activation state;
- new extension version;
- incompatible configuration snapshot;

inside one execution.

---

# 5.31 Failure Model

| Failure | Required Outcome |
|---|---|
| Activation Target unknown | Reject. |
| No applicable Activation Policy | Unavailable. |
| Target/version not upstream-eligible | Reject new activation. |
| Activation scope invalid | Fail closed. |
| Activation state ambiguous | Reject admission. |
| Stale concurrent activation update | Explicit conflict; no overwrite. |
| Required dependency incompatible | Block activation/admission. |
| Runtime artifact unavailable | Target remains unavailable; do not deploy implicitly. |
| Required authorization denied/unavailable | Reject protected activation/invocation. |
| Trust prerequisite fails | Fail closed. |
| Extension Point missing | Reject extension. |
| Extension Contract mismatch | Reject extension. |
| Extension artifact/version mismatch | Reject invocation. |
| Undefined multi-extension ordering/composition | Reject conflicting activation. |
| Extension attempts private/direct persistence access | Block boundary crossing. |
| Extension failure | Surface governed failure; no silent success. |
| New version activates during existing invocation | Existing invocation remains bound to prior context. |
| Ordinary deactivation during existing invocation | Block new invocation; preserve existing context. |
| Security compromise | D03 containment may restrict/terminate affected protected execution. |
| Target Retired | No new activation/reactivation. |

---

# 5.32 Evidence & Provenance

A06 shall make available sufficient technical evidence to relate:

- activation target/version;
- governing scope;
- Activation Policy;
- predecessor/current activation versions;
- activation/deactivation outcome;
- compatibility/dependency checks;
- authorization result reference where applicable;
- trust-boundary result;
- Extension Point/Contract version;
- extension version;
- runtime admission result;
- correlation context.

D04 owns evidence storage/model.

No new canonical business event is created by this requirement.

---

# 6. Architecture & Technology Decisions

| ID | Decision | Status / Basis |
|---|---|---|
| `AD-001` | Module Registry lifecycle activation and A06 runtime activation remain distinct. | Required by L4-A-028 boundary. |
| `AD-002` | Activation-controlled targets require an explicit Activation Policy; activation is never guessed. | A06 technical decision. |
| `AD-003` | Scope/effective-date activation uses A06-002 rather than a second resolution algorithm. | A06 ownership consistency. |
| `AD-004` | Activation changes are versioned and concurrency conflicts are explicit. | B02-003. |
| `AD-005` | Runtime invocation is guarded by an authoritative Runtime Admission Gate. | A06 responsibility. |
| `AD-006` | Ordinary deactivation blocks new admission without silently mutating in-flight execution. | Execution-consistency decision. |
| `AD-007` | Security containment remains D03-owned and may override ordinary execution continuity. | ADR-L5-0001. |
| `AD-008` | Controlled extensions execute only through registered Extension Points and compatible contracts. | A06-001. |
| `AD-009` | Extension execution uses explicit, non-transitive trust boundaries even when physically co-located. | ADR-L5-0001. |
| `AD-010` | Extension invocation binds exact extension and contract versions. | Compatibility/integrity requirement. |
| `AD-011` | Multiple extensions require explicitly declared multiplicity and composition semantics. | Determinism requirement. |
| `AD-012` | Extension activation does not grant persistence, secrets, authorization or broader Tenant authority. | A03/A04/D03/B02 boundaries. |
| `AD-013` | Activation does not deploy software; deployment remains E04/F01-owned. | L4-A-028 + E04 boundary. |
| `AD-014` | No plugin/runtime/sandbox product is selected in A06-003. | Technology/ADR governance. |

---

# 7. Security, Quality & Operational Impact

| Concern | Architectural Requirement |
|---|---|
| Tenant isolation | Activation and invocation preserve validated Tenant/Institution scope. |
| Least privilege | Extension obtains only explicitly admitted capability/dependency access. |
| Trust | Extension trust is explicit and non-transitive. |
| Version integrity | Activation binds exact capability/extension/contract versions. |
| Determinism | No implicit extension order, fallback or substitution. |
| Concurrency | Activation state changes detect stale updates. |
| Execution stability | Running invocation context is immutable under ordinary configuration/activation change. |
| Compromise containment | D03 can restrict the affected relationship without broad trust collapse. |
| Auditability | Activation/admission/version/trust outcomes remain attributable. |
| Portability | No product-specific extension framework or sandbox is mandated. |
| Maintainability | Institutional extension occurs at governed Extension Points, not core patches. |

---

# 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | A06 technical activation shall not modify Module Registry lifecycle state. |
| `CR-002` | Module Registry activation shall not be treated as deployment. |
| `CR-003` | Every activation-controlled target shall have an applicable Activation Policy. |
| `CR-004` | Activation scope/effective-date resolution shall reuse A06-002. |
| `CR-005` | A06 shall not invent an activation default absent an explicit policy. |
| `CR-006` | Activation shall bind an exact target/version. |
| `CR-007` | Activation state history shall be version traceable. |
| `CR-008` | Concurrent activation updates shall prevent lost-update behavior. |
| `CR-009` | Runtime admission shall validate authoritative activation state. |
| `CR-010` | Runtime admission shall preserve validated Tenant/Institution scope. |
| `CR-011` | Required authorization shall remain D02-owned. |
| `CR-012` | Mandatory trust prerequisites shall fail closed. |
| `CR-013` | Activation shall not fabricate deployment/runtime availability. |
| `CR-014` | An unavailable runtime artifact shall prevent invocation. |
| `CR-015` | Ordinary deactivation shall prevent new admission after becoming authoritative. |
| `CR-016` | Ordinary deactivation shall not silently switch or rewrite an in-flight execution. |
| `CR-017` | Security containment shall remain D03-owned and shall not be bypassed by prior activation. |
| `CR-018` | Controlled extensions shall target registered Extension Points only. |
| `CR-019` | Controlled extensions shall bind a compatible Extension Contract version. |
| `CR-020` | Extension invocation shall bind an exact extension version. |
| `CR-021` | Extension execution shall preserve A03 public-boundary rules. |
| `CR-022` | Extension execution shall not acquire direct cross-owner persistence access. |
| `CR-023` | Extension activation shall not grant Tenant scope widening. |
| `CR-024` | Extension activation shall not grant host privileges transitively. |
| `CR-025` | Extension secrets/credentials shall remain D03-004-controlled. |
| `CR-026` | Extension physical co-location shall not erase the logical trust boundary. |
| `CR-027` | Extension upgrades shall use separately validated/version-bound activation. |
| `CR-028` | Existing invocations shall not be hot-swapped to a newly activated extension version. |
| `CR-029` | Extension Point multiplicity shall be explicit. |
| `CR-030` | Multiple active extensions shall require explicit composition semantics. |
| `CR-031` | Registration/storage order shall not determine extension execution order. |
| `CR-032` | Undefined composition shall prevent ambiguous multi-extension activation. |
| `CR-033` | Extension failure shall not be silently converted to success/fallback. |
| `CR-034` | Side-effecting extension retry shall require explicit safe retry/idempotency semantics. |
| `CR-035` | Runtime isolation technology shall remain E04-owned. |
| `CR-036` | A06-003 shall not select a plugin/sandbox/runtime technology without applicable ADR governance. |
| `CR-037` | A Retired target shall not be newly activated or reactivated. |
| `CR-038` | A Suspended target shall not be newly activated before source-governed restoration. |
| `CR-039` | A06 shall not invent automatic termination of existing runtime solely from an upstream suspension/deprecation fact when that consequence is not defined upstream. |
| `CR-040` | Technical activation/admission evidence shall be exposable to D04 without creating a new canonical business event. |

---

# 9. Open Decisions & Deferred Design

| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| `OD-002` | Upstream Module suspension/deprecation documents do not fully define the runtime fate of already-active/in-flight executions. | Upstream semantic gap | No for Draft | A06 blocks new activation where safe; does not invent automatic termination. D03 containment remains available for security cases. |
| `OD-003` | D02 authorization family may still be under Stage-2 finalization. | Peer dependency | No for Draft | A06 defines authorization handoff and shall reconcile with finalized D02. |
| `OD-004` | Physical extension isolation/sandbox technology is not selected. | Technology | No | E04/D03/ADR governance chooses only when required. |
| `OD-005` | Canonical feature-activation lifecycle events are not established upstream. | Upstream event gap | No | Do not invent them; preserve technical evidence only. |
| `OD-006` | Specific extension runtime limits/timeouts are not repository-defined. | Operational/technology | No | E02/E04/D03 may define later; no numeric invention here. |

---

# 10. Traceability Summary

## Canonical Responsibility

`L5-A-06-003`:

**Safe capability activation/deactivation and controlled extension boundaries.**

## Primary L4 Applicability

- `L4-A-028-BH-007`–`BH-009`
- `L4-A-029-BH-003`–`BH-007`
- `L4-A-030-BH-001`–`BH-007`
- `L4-A-031-BH-001`–`BH-007`

## Core L5 Dependencies

- `L5-A-06-001`
- `L5-A-06-002`
- `L5-A-03-001`
- `L5-A-04-001`
- `L5-A-04-002`
- `L5-B-02-003`
- `L5-D-03-001`
- `L5-E-04-001`
- `L5-E-04-002`

## Architecture Decisions

`AD-001`–`AD-014`

## Conformance Requirements

`CR-001`–`CR-040`

## Review Position

The specification remains a production **Draft** pending normal independent L5 architecture review. No L4-status blocker is asserted by this document under the current validation program.