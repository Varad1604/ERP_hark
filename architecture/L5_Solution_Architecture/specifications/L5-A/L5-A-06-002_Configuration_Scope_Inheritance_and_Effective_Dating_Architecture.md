# Configuration Scope, Inheritance & Effective-Dating Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-06-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-06 — Configuration, Feature & Institutional Extensibility Architecture  
**Canonical Technical Responsibility:** Technical scope hierarchy, inheritance, overrides and effective dating.  
**Technical Architecture Owner:** L5-A-06 — Configuration, Feature & Institutional Extensibility Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-12`  
**Depends On:** `L5-A-06-001`; `L5-A-04-001`; `L5-A-04-002`; `L5-B-02-001`; `L5-B-02-002`; `L5-B-02-003`; `L2-A01` v1.0.0 Locked; applicable L4 configuration specifications; L5 governance, validation, traceability and ADR standards.  
**Referenced By:** Configuration-consuming L5 specifications and `L5-A-06-003` where feature/extension activation consumes resolved configuration.  
**Applicable ADRs:** `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0006`.

---

# 2. Purpose, Scope & Non-Goals

## 2.1 Purpose

This specification defines how SutraERP technically determines a **single deterministic resolved configuration context** applicable to an execution when governed configuration may exist at multiple scopes and across multiple effective periods.

It defines:

- configuration scope hierarchy;
- parent-to-child inheritance;
- controlled lower-scope overrides;
- Configuration Resolution Policy;
- effective-dated applicability;
- ambiguity detection;
- missing mandatory configuration handling;
- stable Resolution Snapshots;
- deterministic resolution provenance;
- version-consistent resolution;
- handoff into the immutable Execution Configuration Context defined by `L5-A-06-001`.

It does not transfer business-policy ownership to the configuration platform.

---

## 2.2 In Scope

- canonical technical scope chain;
- Configuration Resolution Policy bound to an exact Configuration Definition version;
- scope-specific applicability;
- inheritance eligibility;
- full-replacement override;
- sparse override;
- explicit sparse-composition semantics;
- non-overridable configuration;
- deterministic precedence;
- effective-start/effective-end applicability;
- source-owned temporal-semantics normalization;
- same-scope overlap detection;
- mandatory completeness validation;
- stable Resolution Snapshot;
- resolution provenance;
- coherent version-set resolution;
- immutable execution-context handoff.

---

## 2.3 Out of Scope

- Configuration Definition registration and version lifecycle — `L5-A-06-001`;
- configuration candidate validation/governance admission — `L5-A-06-001`;
- feature activation/deactivation — `L5-A-06-003`;
- controlled extension execution — `L5-A-06-003`;
- Tenant/Institution identity or lifecycle — A04/upstream owners;
- Organisation lifecycle or structure ownership;
- business-policy meaning of configuration values;
- approval authorities;
- lifecycle events/publishers not defined upstream;
- physical persistence technology — B02;
- API/wire protocol — C01;
- authorization implementation — D02;
- audit infrastructure — D04;
- deployment topology — E04;
- UI — L6.

---

# 3. Authoritative Basis, Constraints & Approved Decisions

## 3.1 Primary Upstream Applicability

| Source | Reference | Effect on A06-002 |
|---|---|---|
| `L2-A01` | §6 Platform Configuration Framework | Establishes the common configuration framework and hierarchy. |
| `L2-A01` | §§6.4–6.8 | Establishes Platform → Tenant → Institution → Organisation → Business Domain → Operational hierarchy, deterministic inheritance and prohibition of circular relationships. |
| `L2-A01` | §§6.9–6.10 | Only validated/governed configuration may become eligible for active use. |
| `L2-A01` | §6.13 | Parent-before-child dependency and missing mandatory configuration constraints. |
| `L2-A01` | §6.14 | Resolved configuration must be complete, validated, Tenant-aware, version-consistent and immutable during execution. |
| `L4-A-056` | `BH-003` | Inherited relationships must be source-governed, acyclic and reference eligible parent context. |
| `L4-A-056` | `BH-005` | Only eligible Active versions participate in future resolution. |
| `L4-A-056` | `BH-006` | Resolution must be deterministic and preserve Tenant isolation and version consistency. |
| `L4-A-056` | `BH-007` | Resolved configuration is read-only to consuming domains. |
| `L4-A-056` | `BH-008` | One ownership layer cannot silently modify another. |
| `L5-A-06-001` | §§5.6–5.8 | Supplies eligible configuration versions and immutable execution-context boundary. |
| `L5-A-04-001/002` | Scope/isolation architecture | Supplies validated Tenant/Institution context and fail-closed isolation. |
| `L5-B-02-003` | Transaction/concurrency architecture | Supplies explicit conflict detection, optimistic concurrency and owner-local consistency mechanisms. |

---

## 3.2 Approved Decision-Led Architecture Decisions

### DEC-A06-002-01 — Override Mode

Each Configuration Definition explicitly declares:

- `NO_OVERRIDE`;
- `FULL_REPLACEMENT`;
- `SPARSE_OVERRIDE`.

There is no universal merge rule.

### DEC-A06-002-02 — Same-Scope Collision

Two independently eligible configuration versions for the same Configuration Definition, governing scope and overlapping applicability period constitute invalid ambiguity.

No creation timestamp, storage order or highest-version rule may silently choose a winner.

### DEC-A06-002-03 — Inheritance Eligibility

Parent configuration may be inherited only where the applicable Configuration Resolution Policy explicitly permits inheritance.

### DEC-A06-002-04 — Missing Mandatory Configuration

If mandatory configuration cannot be completely resolved after permitted inheritance and override processing, resolution fails.

### DEC-A06-002-05 — Running Execution Stability

An established Execution Configuration Context retains its resolved versions.

A later effective configuration applies to subsequent resolution and does not silently mutate an existing execution context.

---

# 4. Technical Ownership & Boundaries

## 4.1 A06-002 Owns

A06-002 owns:

- Configuration Resolution Policy semantics;
- resolution-policy binding to Configuration Definition versions;
- scope resolution;
- ancestor traversal;
- inheritance eligibility;
- override precedence;
- full-replacement semantics;
- sparse-composition control;
- temporal applicability normalization;
- same-scope ambiguity rejection;
- Resolution Snapshot semantics;
- deterministic composition;
- mandatory-completeness evaluation;
- resolution provenance;
- resulting resolved version-set handoff.

---

## 4.2 A06-002 Does Not Own

A06-002 shall not own:

- business meaning of configuration;
- approval authority;
- Configuration Definition lifecycle;
- Tenant/Institution/Organisation lifecycle;
- business hierarchy ownership;
- configuration version creation;
- feature activation;
- extension execution;
- database implementation;
- business events;
- UI.

---

# 5. Solution Architecture

## 5.1 Canonical Configuration Scope Chain

Locked L2-A01 establishes:

```text
Platform Defaults
        ↓
Tenant
        ↓
Institution
        ↓
Organisation
        ↓
Business Domain
        ↓
Operational
```

This represents resolution precedence.

It does not transfer configuration ownership between layers.

A06-002 consumes authoritative scope identities and relationships from their owning architectures.

---

# 5.2 Configuration Resolution Policy

A06-002 introduces a logical **Configuration Resolution Policy**.

Each policy is bound to an exact Configuration Definition version owned through `L5-A-06-001`.

```text
A06-001
Configuration Definition vN
          │
          ▼
A06-002
Configuration Resolution Policy vN
```

The Resolution Policy contains the technical rules necessary to resolve configuration consistently.

Conceptually:

```text
Configuration Resolution Policy
{
    configuration-definition-version reference

    permitted scope classes

    inheritance policy

    override mode

    sparse-composition semantics where applicable

    mandatory / optional requirement

    temporal applicability semantics
}
```

This is an architecture contract, not a physical schema.

The policy shall not redefine the business meaning of configuration.

---

# 5.3 Resolution Policy Binding

A Resolution Policy shall correspond to the Configuration Definition version whose values it resolves.

Therefore:

```text
Configuration Definition v3
        ↓
Resolution Policy for v3
```

shall not silently be replaced by:

```text
Resolution Policy for v4
```

when resolving historical configuration governed under Definition v3.

Where Configuration Definition evolution changes inheritance, override or temporal semantics, the successor definition shall carry the corresponding successor Resolution Policy relationship.

Historical configuration remains interpreted according to its governed definition/policy context.

---

# 5.4 Resolution Request

Resolution begins with sufficient information to identify:

- Configuration Definition/version;
- validated execution scope;
- effective evaluation point.

Conceptually:

```text
ConfigurationResolutionRequest
{
    configuration definition reference
    validated execution scope
    effective evaluation point
}
```

The technical representation remains implementation-owned.

---

# 5.5 Resolution Snapshot

A06-002 shall not repeatedly resolve against mutable configuration state while one resolution is in progress.

Before hierarchy composition begins, the resolver establishes a logical **Resolution Snapshot**.

```text
A06-001
Eligible Configuration View
        ↓
Capture Resolution Snapshot
        ↓
A06-002
Deterministic Resolution
```

The Resolution Snapshot represents the coherent set of eligible configuration versions visible to that resolution.

Conceptually:

```text
ResolutionSnapshot
{
    Configuration Definition/version
    validated execution scope
    evaluation point
    eligible configuration-version set
    snapshot identity / consistency reference
}
```

This does not require copying every configuration value into a separate physical store.

It requires a stable logical view sufficient to ensure that one resolution does not observe incompatible portions of configuration state from different logical moments.

---

# 5.6 Resolution Snapshot Consistency Boundary

The following is prohibited:

```text
read Tenant state
        ↓
configuration changes
        ↓
read Institution state
        ↓
another configuration changes
        ↓
compose mixed result
```

Instead:

```text
establish eligible-version snapshot
        ↓
resolve all applicable scopes
against that same logical snapshot
        ↓
produce resolved context
```

A06-002 owns the consistency requirement.

`L5-B-02-003` owns physical transaction/concurrency realization.

The architecture does not require distributed ACID across canonical business owners.

The A06 configuration-control plane provides the governed eligible-version view; B02 supplies sufficient persistence consistency for A06 to capture or reference that stable view.

---

# 5.7 Scope-Chain Construction

For a resolution request, A06-002 constructs the applicable source-governed scope path.

Example:

```text
Platform
   ↓
Tenant T1
   ↓
Institution I4
   ↓
Organisation O7
   ↓
Business Domain Finance
   ↓
Operational Context
```

The resolver rejects:

- circular ancestry;
- Tenant-inconsistent ancestry;
- ambiguous parentage required for resolution;
- scope outside the protected execution context.

---

# 5.8 Parent-Before-Child Evaluation

Resolution proceeds from general to specific:

```text
Parent
  ↓
Child
```

Where inheritance requires parent state, a child candidate shall not be resolved independently from that required parent context.

---

# 5.9 Inheritance Model

Inheritance is opt-in through the Resolution Policy.

### Inheritance Permitted

```text
Eligible Parent Configuration
            ↓
No eligible lower-scope override
            ↓
Inherited result
```

The child does not become owner of inherited configuration.

### Inheritance Prohibited

```text
Parent Configuration
        X
Required Child Scope
```

The parent value shall not be used.

If configuration is mandatory and no eligible local value exists, resolution fails.

---

# 5.10 Override Modes

## 5.10.1 NO_OVERRIDE

A lower-scope value cannot replace the applicable parent configuration.

A conflicting lower-scope candidate shall not become part of effective resolution.

---

## 5.10.2 FULL_REPLACEMENT

An eligible lower-scope configuration replaces the inherited configuration as a complete resolved value for that scope.

```text
Parent P
   ↓
Child C
   ↓
Resolved = C
```

Omitted child elements are not implicitly filled from the parent unless the Configuration Definition models those elements as independently resolvable configuration.

---

## 5.10.3 SPARSE_OVERRIDE

Sparse override begins from the inherited parent result and applies only explicitly permitted lower-scope contributions.

Example:

```text
Parent
A = 1
B = 2
C = 3

Child Override
B = 5

Result
A = 1
B = 5
C = 3
```

Sparse override shall operate only according to composition semantics explicitly declared in the Configuration Resolution Policy.

---

# 5.11 Sparse-Composition Semantics

`SPARSE_OVERRIDE` does not imply a universal object or collection merge algorithm.

For every sparsely overridable element, the Resolution Policy shall define sufficient composition semantics to determine how a lower-scope contribution affects the inherited value.

This is particularly required for:

- nested structures;
- maps;
- sets;
- ordered collections;
- lists;
- composite values.

For example, a lower-scope collection:

```text
[Card]
```

shall not be interpreted implicitly as:

```text
replace inherited collection
```

or:

```text
append to inherited collection
```

or:

```text
remove all unlisted elements
```

unless the applicable Resolution Policy defines that behavior.

If sparse composition is requested but the necessary composition semantics are undefined:

```text
resolution fails
```

The platform shall not invent merge semantics from data structure alone.

---

# 5.12 Multi-Level Composition

Where permitted:

```text
Platform
   ↓
Tenant
   ↓
Institution
   ↓
Organisation
```

For sparse override:

```text
Platform result
      +
Tenant contribution
      ↓
Tenant result
      +
Institution contribution
      ↓
Institution result
      +
Organisation contribution
      ↓
Final result
```

Every contribution remains traceable.

---

# 5.13 Source-Owned Temporal Semantics

Business/domain owners retain the meaning of effective periods.

A06-002 shall not invent whether an upstream effective period represents:

- local calendar date;
- academic date;
- payroll period;
- absolute timestamp;
- operational date;
- another domain-defined temporal concept.

The applicable Resolution Policy must identify sufficient source-owned temporal semantics for deterministic evaluation.

---

# 5.14 Temporal Normalization

Once source-owned temporal meaning is known, A06-002 normalizes it into deterministic technical interval semantics.

Internally, interval evaluation shall behave equivalently to:

```text
[start inclusive, end exclusive)
```

This does not change the business meaning of the source dates.

Example:

```text
Source business semantics:
Effective from 1 April
Effective through 31 March
```

may be normalized at the source-defined date granularity into the equivalent technical interval ending at the next boundary after 31 March.

The normalization rule preserves the source meaning while eliminating technical boundary ambiguity.

---

# 5.15 Effective-Dating Evaluation

Only configuration applicable at the Resolution Snapshot's evaluation point may participate.

```text
start
  │
  ├──────── applicable ────────┤
                               │
                              end
```

A06-002 shall use the temporal semantics bound to the applicable Resolution Policy.

---

# 5.16 Adjacent Effective Versions

The normalized interval model permits:

```text
V1 [T1,T2)
V2 [T2,T3)
```

without overlap.

V1 stops being applicable exactly when V2 begins.

---

# 5.17 Same-Scope Temporal Ambiguity

For the same:

- Configuration Definition;
- Configuration Definition version where relevant;
- governing scope;
- applicability context;

two independently eligible overlapping versions are invalid.

```text
Version A: [T1 -------- T4)
Version B:      [T2 -------- T5)
```

Result:

```text
AMBIGUOUS
→ resolution failure
```

The resolver shall not silently use:

- latest-created;
- highest-version;
- latest-updated;
- lexical ID;
- storage order.

---

# 5.18 Cross-Scope Temporal Overlap

Parent/child temporal overlap is valid when allowed by the Resolution Policy.

Example:

```text
Tenant Configuration
[Jan ---------------- Dec]

Institution Override
        [Mar ------ Jun]
```

Result:

```text
Jan-Feb → Tenant result
Mar-Jun → Institution-resolved result
Jul-Dec → Tenant result
```

The child does not alter the Tenant-owned configuration.

---

# 5.19 Eligibility Filtering

A06-002 receives A06-001 eligible configuration state and filters the Resolution Snapshot by:

1. Configuration Definition/version compatibility;
2. Resolution Policy binding;
3. protected execution scope;
4. permitted scope class;
5. effective-period applicability;
6. inheritance eligibility;
7. override eligibility;
8. sparse-composition validity;
9. same-scope non-ambiguity.

A failing candidate does not participate.

---

# 5.20 Deterministic Resolution Algorithm

```text
1. Validate execution scope
        ↓
2. Identify Configuration Definition/version
        ↓
3. Resolve bound Configuration Resolution Policy
        ↓
4. Construct source-governed scope chain
        ↓
5. Establish Resolution Snapshot
        ↓
6. Filter by temporal applicability
        ↓
7. Detect same-scope ambiguity
        ├── ambiguity → FAIL
        ↓
8. Resolve highest applicable scope
        ↓
9. Traverse parent → child
        ↓
10. At each scope:

    no candidate
        → inherit only if permitted

    candidate + NO_OVERRIDE
        → reject unauthorized override

    candidate + FULL_REPLACEMENT
        → replace inherited result

    candidate + SPARSE_OVERRIDE
        → apply only declared composition semantics

        ↓
11. Validate mandatory completeness
        ├── incomplete → FAIL
        ↓
12. Produce Resolution Provenance
        ↓
13. Establish immutable Execution Configuration Context
```

The entire algorithm operates against the same Resolution Snapshot.

---

# 5.21 Missing Configuration Semantics

## Optional

No applicable result may be valid where the Resolution Policy declares configuration optional.

## Mandatory + Inheritance Permitted

Permitted ancestors may satisfy the requirement.

## Mandatory + Inheritance Prohibited

The required governing scope must provide an eligible value.

## Mandatory + No Valid Result

```text
mandatory
+
no local result
+
no permitted inherited result
        ↓
FAIL
```

No implicit:

- null;
- empty object;
- hardcoded implementation default;
- cross-Tenant fallback;
- undocumented global value;

may be substituted.

---

# 5.22 Resolution Provenance

Every resolved result preserves enough information to explain and reproduce it.

Conceptually:

```text
Resolved Configuration
    │
    ├── Configuration Definition/version
    ├── Resolution Policy reference
    ├── protected execution scope
    ├── effective evaluation point
    ├── Resolution Snapshot reference
    ├── contributing configuration versions
    ├── contribution scopes
    ├── inherited contributions
    ├── override contributions
    └── final resolved context identity
```

For sparse override, provenance must identify the contributing versions and composition path.

D04 owns audit/evidence persistence.

---

# 5.23 Version Consistency

One completed resolution shall derive from exactly one coherent Resolution Snapshot.

Therefore a result shall not contain components unintentionally observed across changing eligible-version states.

Physical implementation may use:

- an appropriate consistent read;
- immutable version-set references;
- repository generation/version identity;
- another B02-conformant mechanism.

A06-002 does not select the mechanism.

---

# 5.24 Execution Configuration Context Handoff

```text
A06-002
Resolved Configuration
+
Resolution Provenance
+
Snapshot Reference
        ↓
A06-001
Execution Configuration Context
        ↓
Consumer
```

After establishment:

```text
Execution Configuration Context
        =
immutable for that execution
```

If V5 becomes effective after Execution X began under V4:

```text
Execution X → keeps V4-derived context

New Execution Y → resolves against
                   applicable current snapshot
```

---

# 5.25 Failure Model

| Failure | Required Outcome |
|---|---|
| Invalid Tenant scope | Fail closed. |
| Unknown required parent | Resolution fails. |
| Circular ancestry | Reject relationship/resolution. |
| Missing Resolution Policy | Resolution fails where policy is required. |
| Policy/Definition version mismatch | Resolution fails. |
| Inheritance prohibited | Do not inherit. |
| Override prohibited | Lower-scope override does not become effective. |
| Undefined sparse-composition semantics | Sparse resolution fails. |
| Same-scope temporal overlap | Resolution fails for affected applicability. |
| Mandatory result missing | Resolution fails. |
| Temporal meaning unavailable | Do not invent interpretation. |
| Tenant mismatch | Fail closed. |
| Stable snapshot cannot be established | Do not produce resolved context. |
| Eligible state changes after snapshot capture | Current resolution remains bound to its snapshot. |
| Configuration changes after execution context established | Existing context remains unchanged. |

---

# 5.26 Persistence & Concurrency Boundary

A06-002 defines:

- required resolution consistency;
- overlap invariants;
- snapshot semantics;
- version-set coherence.

`L5-B-02-001/002/003` own:

- physical persistence;
- persistence isolation;
- transaction mechanics;
- optimistic concurrency;
- stronger owner-local serialization where justified;
- physical consistency mechanism.

B02 realization must support A06's ability to:

- identify eligible versions by scope;
- preserve immutable version identity;
- detect conflicting publication;
- prevent lost updates;
- establish or reference a stable eligible-version view;
- enforce temporal overlap invariants where appropriate.

A06-002 does not prescribe a database or isolation level.

---

# 5.27 No Distributed-ACID Requirement

A06-002 does not require configuration resolution to open one distributed transaction across all canonical business owners.

Configuration meaning remains owner-controlled, while A06 owns the shared configuration-control and resolution capability.

Therefore:

```text
Business owner semantics
        ↓
A06-governed configuration versions
        ↓
A06 Resolution Snapshot
        ↓
deterministic resolution
```

Cross-owner distributed ACID is not required.

---

# 5.28 Cache / Projection Boundary

Resolved configuration may be cached/projected only if the cache identity preserves sufficient resolution identity, including:

- Tenant/protected scope;
- Configuration Definition/version;
- Resolution Policy version/reference;
- effective evaluation semantics;
- applicable eligible-version/snapshot identity.

A cache shall not implement alternative precedence, inheritance, temporal or fallback semantics.

If cache consistency with the authoritative Resolution Snapshot cannot be established, the cached value shall not be treated as authoritative configuration.

No cache technology or TTL is selected.

---

# 6. Architecture & Technology Decisions

| ID | Decision | Status |
|---|---|---|
| `AD-001` | Resolution follows Platform → Tenant → Institution → Organisation → Business Domain → Operational hierarchy. | Approved for Draft |
| `AD-002` | Inheritance occurs only when explicitly permitted by the Resolution Policy bound to the applicable Configuration Definition version. | User-approved |
| `AD-003` | Override mode is declared per configuration definition/policy as no override, full replacement or sparse override. | User-approved |
| `AD-004` | Sparse override requires explicit composition semantics; structure alone never determines merge behavior. | Approved for Draft |
| `AD-005` | Same-scope overlapping effective configuration is invalid ambiguity. | User-approved |
| `AD-006` | Parent-to-child traversal defines deterministic precedence. | Approved for Draft |
| `AD-007` | Source-owned temporal meaning is normalized into deterministic half-open technical intervals without changing business semantics. | Approved for Draft |
| `AD-008` | Missing mandatory configuration after permitted inheritance causes resolution failure. | User-approved |
| `AD-009` | Every resolution executes against one stable logical Resolution Snapshot. | Approved for Draft |
| `AD-010` | Resolution produces provenance sufficient to reproduce the result. | Approved for Draft |
| `AD-011` | Established executions retain their resolved context; later effective configuration applies to subsequent resolutions. | User-approved |
| `AD-012` | No distributed ACID across business owners is required for configuration resolution. | Inherits B02 architecture |
| `AD-013` | No implicit hardcoded/global/null fallback may bypass governed resolution. | Approved for Draft |

No new product or framework technology is selected.

---

# 7. Security, Privacy, Quality & Operational Impact

| Concern | Required Property |
|---|---|
| Tenant isolation | Candidate and scope must remain inside validated Tenant context. |
| Determinism | Same snapshot and request produce the same result. |
| Integrity | Lower scope cannot mutate parent-owned configuration. |
| Temporal integrity | Same-scope overlap is rejected. |
| Snapshot consistency | Resolution observes one coherent eligible-version view. |
| Execution stability | Established execution context remains immutable. |
| Auditability | Snapshot, policy and contributing-version provenance is retained. |
| Maintainability | Override and composition semantics are explicit. |
| Scalability | Resolution traverses a bounded configuration hierarchy. |
| Portability | No product-specific persistence/cache mechanism is required. |

---

# 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Resolution shall use only source-governed scope relationships. |
| `CR-002` | Scope relationships participating in resolution shall be acyclic. |
| `CR-003` | Resolution shall preserve Tenant isolation. |
| `CR-004` | Every resolvable Configuration Definition version shall have an applicable Resolution Policy where resolution behavior is required. |
| `CR-005` | Resolution Policy shall remain bound to the applicable Configuration Definition version. |
| `CR-006` | Resolution shall not apply a successor policy to historical configuration unless compatibility is explicitly governed. |
| `CR-007` | Each Resolution Policy shall declare whether inheritance is permitted. |
| `CR-008` | The resolver shall not inherit where inheritance is prohibited. |
| `CR-009` | Every overridable definition shall declare its override mode. |
| `CR-010` | Sparse override shall occur only where explicitly permitted. |
| `CR-011` | Sparse composition shall use explicitly declared composition semantics. |
| `CR-012` | Nested/collection structures shall not be implicitly merged from data structure alone. |
| `CR-013` | Full-replacement mode shall not implicitly inherit omitted elements. |
| `CR-014` | NO_OVERRIDE shall prevent unauthorized lower-scope replacement. |
| `CR-015` | Scope resolution shall traverse from parent to applicable child. |
| `CR-016` | Only A06-001 eligible configuration shall participate. |
| `CR-017` | Each resolution shall establish one logical Resolution Snapshot before composition. |
| `CR-018` | All hierarchy composition within one resolution shall use the same Resolution Snapshot. |
| `CR-019` | Failure to establish a coherent Resolution Snapshot shall prevent resolved-context production. |
| `CR-020` | Source-owned temporal meaning shall be preserved during normalization. |
| `CR-021` | Technical temporal evaluation shall use deterministic non-overlapping interval boundaries. |
| `CR-022` | Only configuration applicable at the resolution evaluation point shall participate. |
| `CR-023` | Two eligible same-scope versions shall not overlap for the same governed applicability. |
| `CR-024` | Same-scope ambiguity shall fail rather than invoke an implicit winner. |
| `CR-025` | Cross-scope overlap shall be interpreted only through governed inheritance/override semantics. |
| `CR-026` | Missing mandatory configuration shall fail after all permitted inheritance is exhausted. |
| `CR-027` | Resolution failure shall not produce hardcoded, null, cross-Tenant or undocumented fallback. |
| `CR-028` | Successful resolution shall retain Configuration Definition, Resolution Policy, snapshot and contributing-version provenance. |
| `CR-029` | One resolved context shall represent one coherent eligible-version snapshot. |
| `CR-030` | Consumers shall receive resolved configuration as read-only context. |
| `CR-031` | A later effective version shall not mutate an already-established Execution Configuration Context. |
| `CR-032` | New executions shall resolve using the snapshot applicable to their own resolution. |
| `CR-033` | A06-002 shall not create approval authorities, commands, events or publishers absent upstream authority. |
| `CR-034` | Physical transaction/concurrency implementation shall remain B02-owned. |
| `CR-035` | A06-002 shall not require distributed ACID across canonical business owners. |
| `CR-036` | Cache/projection mechanisms shall preserve resolution identity and shall not implement alternative semantics. |
| `CR-037` | Configuration ownership shall not transfer to consuming domains. |
| `CR-038` | A06-002 shall not invent domain-owned temporal meaning. |

---

# 9. Open Decisions & Deferred Design

| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| `OD-001` | Primary L4 configuration sources remain Draft/Review. | Upstream approval | Yes for Architecture Approved; no for Draft | Revalidate upon upstream approval. |
| `OD-002` | Exact approval/activation commands and lifecycle events remain undefined upstream. | Upstream gap | No for this architecture | Do not invent. |
| `OD-003` | Individual Configuration Definitions may require domain-specific temporal granularity. | Domain-owned semantics | No for baseline | Owner supplies semantics through governed definition/policy. |
| `OD-004` | Physical representation of Resolution Snapshot identity/view. | B02 realization | No | B02 selects a conformant consistent-read/version-set mechanism. |
| `OD-005` | Physical enforcement mechanism for temporal overlap. | B02 realization | No | B02 may use optimistic concurrency or justified stronger local serialization. |

---

# 10. Traceability Summary

## Primary Upstream

- `L2-A01` §§6.4–6.16
- `L4-A-056-BH-003`
- `L4-A-056-BH-005`
- `L4-A-056-BH-006`
- `L4-A-056-BH-007`
- `L4-A-056-BH-008`

## L5 Dependencies

- `L5-A-06-001`
- `L5-A-04-001`
- `L5-A-04-002`
- `L5-B-02-001`
- `L5-B-02-002`
- `L5-B-02-003`

## User-Approved Decisions

- Override model: explicit per definition/policy.
- Same-scope overlap: reject ambiguity.
- Inheritance: explicitly permitted only.
- Missing mandatory configuration: resolution failure.
- Existing execution after new effective version: retains existing context.

## Feasibility Corrections

- Stable Resolution Snapshot added.
- Resolution Policy explicitly bound to Configuration Definition version.
- Sparse composition requires explicit element/composite semantics.
- Business temporal meaning is normalized rather than overwritten by technical interval conventions.

## Architecture Decisions

`AD-001`–`AD-013`

## Conformance Requirements

`CR-001`–`CR-038`

## Promotion Constraint

The specification is suitable as a production **Draft**.

Formal Architecture Approved status remains blocked by material upstream L4 Draft/Review status until the required governance conditions are satisfied.
