# L5-B-01-002 — Search Index Lifecycle, Access Filtering & Synchronization Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-01-002 |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-01 — Enterprise Search & Indexing Architecture |
| Canonical Technical Responsibility | Search index lifecycle, freshness, synchronization and authorization-aware filtering. |
| Technical Architecture Owner | Search Index Lifecycle, Access Filtering & Synchronization technical architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft — Authoring |
| Last Updated | 2026-09-11 |
| Depends On | L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5-B-01-001; L5-A-04; L5-D-02; L5-D-04; L5-E-01; L5-E-02; L4-H-018; L4-H-019; L4-H-020; L2-H03; L3-H-018–H-020 |
| Referenced By | L5-B-01-001 and downstream L6/implementation architecture |
| Applicable ADRs | None identified; technology choices remain subject to ADR governance |

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the reusable technical architecture that governs the lifecycle, synchronization, freshness and authorization-aware filtering of H03 search indexes and searchable projections. The architecture ensures that search representations are derived from authoritative source information, become searchable only through governed lifecycle controls, remain aligned with source-version lineage and cannot bypass applicable access restrictions.

The architecture complements L5-B-01-001: B01-001 owns search execution and result assembly, while B01-002 owns the technical control plane for index eligibility, lifecycle, synchronization and filtering.

### In Scope

- Search index and searchable-projection lifecycle management.
- Admission and activation of governed search projection definitions.
- Source-to-projection synchronization architecture.
- Source-version and synchronization lineage.
- Freshness assessment and currentness determination.
- Rebuild, replay and recovery coordination for search representations.
- Authorization-aware search filtering and visibility enforcement at the search boundary.
- Index validity, activation and retirement controls.
- Technical synchronization evidence and failure handling.

### Out of Scope / Non-Goals

- Canonical business entity ownership or mutation.
- Business approval, eligibility or institutional search-policy decisions.
- Search query execution, relevance/ranking and result assembly owned by L5-B-01-001.
- Authentication, authorization policy ownership or permission semantics owned by applicable L5-D architecture.
- Canonical audit/evidence meaning owned by L5-D-04.
- Business lifecycle redesign or canonical event ownership.
- API protocol/product selection, database schema, infrastructure topology or search-product selection.
- UI/UX design.

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Version / Status | Exact Reference | What It Establishes | L5 Use |
|---|---|---|---|---|
| 105-specification corpus | v1.1.0 / Architecture Approved | L5-B-01-002 | Index lifecycle/freshness/synchronization and authorization-aware filtering. | Canonical technical responsibility |
| L2-H03 | Locked | Enterprise Directory & Search Workflows | H03 maintains searchable projections and governed search participation without becoming the source of canonical business facts. | Ownership and terminology verification |
| L3-H-018 | v1.0 / Locked | §§7–16 | Authoritative source changes update the corresponding search projection while preserving source ownership and traceable synchronization. | Synchronization and freshness basis |
| L3-H-019 | v1.0 / Locked | §§7–16 | Missing, corrupt, stale or failed search representations are reconstructed/reconciled from authoritative source information and validated before availability. | Recovery and rebuild basis |
| L3-H-020 | v1.0 / Locked | §§7–16 | Search participation uses a governed Search Projection Definition with controlled approval and activation. | Projection admission/activation basis |
| L4-H-018 | v1.0 / Draft | BH-001–004, BH-006–007 | Source changes update H03 representation under an indexing contract and preserve source/version lineage. | Planning applicability; not approval authority |
| L4-H-019 | v1.0 / Draft | BH-001–004, BH-006–007 | Recovery uses authoritative source information and validated source/synchronization lineage before search availability. | Planning applicability; not approval authority |
| L4-H-020 | v1.0 / Draft | BH-001–007 | Only an Approved projection definition may become Active and only approved searchable attributes/references may be activated. | Planning applicability; not approval authority |

The applicable L4 sources currently show `Draft`. They support controlled planning and technical derivation but cannot by themselves produce a final `Validated` L4→L5 relationship until approved or explicitly governance-authorized.

### 3.2 Canonical Constraints

- Search representations remain derived technical artifacts and never become canonical business records.
- A Searchable Projection remains associated with its canonical source reference and source ownership.
- Only an approved search projection definition may be technically activated for search participation.
- Synchronization shall use authoritative source information and preserve source-version/synchronization lineage.
- Stale, corrupt, incomplete or source-uncertain representations shall not silently become current/searchable.
- Authorization-aware filtering shall not broaden the applicable caller or execution scope.
- Search filtering does not create or own the underlying authorization policy.
- Search Index lifecycle and synchronization shall remain distinct from search query execution owned by B01-001.

### 3.3 Assumptions / Open Authority

**AS-001 — L4 Approval State:** L4-H-018, L4-H-019 and L4-H-020 are currently Draft in the reviewed branch. Exact production traceability remains pending approval/current-state reconciliation.

**AS-002 — Technology Choice:** No search technology/product has been selected in the reviewed sources. Product/framework choices remain subject to governed ADR/technology-selection decisions.

**AS-003 — Authorization Policy Source:** This architecture consumes authoritative authorization/visibility context from the applicable security architecture; it does not invent permission policy or business discoverability rules.

## 4. Technical Architecture

### 4.1 Logical Components

| Component | Responsibility | Boundary |
|---|---|---|
| Projection Definition Registry | Maintains the technical representation of approved search-participation definitions and their effective technical status. | Does not decide business approval; consumes authoritative approval decisions. |
| Projection Admission Controller | Validates that a projection definition is eligible for technical activation and that required source/security metadata is present. | Cannot activate an unapproved definition or invent searchable attributes. |
| Source Change Intake | Receives authoritative source changes and synchronization requests with source identity and version context. | Does not become the source of business truth. |
| Synchronization Coordinator | Coordinates source-to-projection synchronization, lineage checks and controlled progression of the search representation. | Does not own source-domain lifecycle or rules. |
| Projection Builder / Reconciler | Constructs or reconciles searchable representations from authoritative source information. | Cannot invent missing source facts. |
| Freshness & Lineage Controller | Determines technical currentness from source-version and synchronization lineage. | Does not define business validity. |
| Index Lifecycle Controller | Controls technical index readiness, rebuild, activation, suspension and retirement states. | Does not redefine H03 business lifecycle semantics. |
| Access Filtering Boundary | Applies authoritative authorization/visibility context to determine whether a representation is eligible for search exposure. | Does not own authorization policy. |
| Active Index Gate | Exposes only technically eligible, current and permitted representations to B01-001 search execution. | Cannot bypass lifecycle, freshness or filtering controls. |
| Synchronization Evidence Boundary | Produces technical lineage, validation and synchronization evidence for diagnosis and assurance. | Canonical audit/evidence meaning remains D04. |

### 4.2 Projection Admission and Activation Flow

```text
Canonical Source Domain
        ↓
Search Participation Request / Approved Definition
        ↓
Projection Definition Registry
        ↓
Projection Admission Controller
        ↓
Source Ownership + Scope + Security Validation
        ↓
Technically Eligible Definition
        ↓
Controlled Activation
        ↓
Active Search Projection Definition
        ↓
Synchronization / Index Lifecycle
```

The technical architecture does not substitute for Search Governance Authority approval. L4-H-020 requires only an Approved definition to become Active and preserves source ownership throughout activation.

### 4.3 Source Synchronization Flow

```text
Authoritative Source Change
        ↓
Source Change Intake
        ↓
Source Identity / Version Resolution
        ↓
Synchronization Contract Validation
        ↓
Projection Reference Resolution
        ↓
Projection Build / Reconcile
        ↓
Lineage + Integrity Validation
        ↓
Freshness Evaluation
        ↓
Access / Visibility Evaluation
        ↓
Eligible Search Representation
        ↓
Active Index Gate
```

A source change that cannot be associated with authoritative source identity, valid synchronization context or required security metadata does not become a current searchable representation.

### 4.4 Freshness and Lineage Model

The technical model maintains sufficient lineage to determine whether a search representation corresponds to the authoritative source position it claims to represent.

```text
Canonical Source Identity
        +
Source Version / Change Position
        +
Projection Identity
        +
Synchronization Attempt / Outcome
        ↓
Lineage Evaluation
        ↓
Current / Stale / Unknown
```

`Current` is a technical synchronization condition, not a new business lifecycle state. `Stale` or `Unknown` representations shall not silently be treated as current.

Where source truth is unavailable or synchronization lineage is unresolved, the architecture preserves the unresolved condition and routes it to governed recovery rather than guessing.

### 4.5 Index Lifecycle Control

The technical lifecycle controller provides a controlled boundary around index availability. A representation may be prepared, validated and made eligible before it becomes exposed through the Active Index Gate.

For full rebuild/recovery, the architecture supports the source-defined technical sequence represented by L4-H-019:

```text
Ready → Rebuilding → Activating → Ready
```

A critical technical failure may leave the index in a recovery-required condition. The architecture shall not expose a recovered representation until required source, contract, security and lineage validation succeeds.

This technical lifecycle does not redefine H03 business lifecycle semantics; it controls availability of the derived search artifact.

### 4.6 Access Filtering Architecture

The Access Filtering Boundary receives authoritative authorization and applicable visibility context and applies it to search-side representations before they become eligible for search consumption.

```text
Caller / Execution Context
        ↓
Authoritative Authorization Context
        ↓
Scope Resolution
        ↓
Search Representation Eligibility
        ↓
Access Filter
        ↓
Eligible / Restricted Representation
        ↓
Active Index Gate
```

The filtering boundary shall:

- preserve Tenant, Institution and other applicable scope;
- prevent restricted representations from reaching the eligible search set;
- preserve source-defined discoverability constraints;
- avoid inferring permissions from search relevance or index presence;
- avoid expanding caller authority;
- avoid turning technical filtering into canonical authorization ownership.

### 4.7 Synchronization and Index Separation

B01-002 owns the technical path that determines which search representation is current and eligible. B01-001 consumes the resulting active/eligible index boundary for query execution.

```text
B01-002
Definition + Sync + Freshness + Access Filtering
                 ↓
          Active Index Gate
                 ↓
B01-001
Search Execution + Ranking + Result Assembly
```

This separation prevents query execution from independently deciding index freshness, synchronization state or access eligibility.

### 4.8 Recovery / Rebuild Architecture

Recovery begins from an identified affected projection/index and authoritative source references. The recovery path preserves identity and prior lineage, reconstructs or reconciles the derived representation, validates source and security context, and only then returns it to eligible availability.

```text
Missing / Corrupt / Stale / Failed Representation
        ↓
Recovery Admission
        ↓
Authoritative Source Resolution
        ↓
Rebuild / Reconcile
        ↓
Contract + Security + Lineage Validation
        ↓
Activation Gate
        ↓
Eligible / Current Representation
```

Unavailable authoritative source truth leaves recovery unresolved; the architecture never fabricates business data to restore search availability.

### 4.9 Technical State Boundary

The architecture may maintain technical states needed to govern derived search artifacts, but these states shall not be presented as replacements for upstream business lifecycles.

Search participation definition states remain governed by H03. Technical index/readiness conditions are implementation architecture used to determine exposure through the Active Index Gate.

### 4.10 Evidence and Observability

Material synchronization and lifecycle conditions shall be observable, including:

- projection identity and canonical source reference;
- source-version/synchronization lineage;
- admission/activation validation result;
- freshness evaluation;
- access-filtering decision context at the technical level;
- rebuild/recovery outcome;
- active-index eligibility;
- synchronization failure and unresolved conditions.

Sensitive business values shall not be unnecessarily replicated into operational evidence. Business audit meaning remains with L5-D-04.

## 5. Dependencies / Integration

### 5.1 Upstream Dependencies

- L2-H03 — Enterprise Directory & Search Workflows.
- L3-H-018, L3-H-019 and L3-H-020 for business outcome and ownership verification.
- Applicable current approved L4 behavior specifications once promoted.
- Applicable Tenant/Institution context architecture from L5-A-04.
- Applicable authorization architecture from L5-D-02.

### 5.2 Peer L5 Dependencies

- **L5-B-01-001:** consumes the active/eligible index boundary for search execution, ranking and result assembly.
- **L5-D-02:** supplies authorization and policy-enforcement context; B01-002 does not redefine permission semantics.
- **L5-D-04:** receives/consumes technical synchronization and lifecycle evidence while retaining canonical audit/evidence responsibility.
- **L5-E-01:** provides availability/resilience architecture for index and synchronization paths.
- **L5-E-02:** provides performance/capacity architecture for synchronization throughput, rebuild workload and capacity protection.
- **L5-A-08:** may provide background execution mechanisms for rebuild/synchronization workloads; B01-002 retains search synchronization responsibility.
- **L5-C-01:** may provide technical contract mechanisms for source-change or projection-participation boundaries where applicable.

### 5.3 Governed Handoff

```text
Source Domain
     ↓
Authoritative Source Representation / Change
     ↓
B01-002 Synchronization + Lineage + Freshness + Access Filtering
     ↓
Active / Eligible Index Boundary
     ↓
B01-001 Search Execution
     ↓
Source-Derived Search Result
```

No handoff transfers canonical business ownership or authorization policy ownership.

## 6. Technical Behavior / Failure Concerns

### TB-001 — Definition Eligibility

Only a technically eligible search projection definition backed by the authoritative approval state may enter technical activation processing.

### TB-002 — Source Ownership Preservation

Synchronization shall preserve canonical source identity and shall not mutate or reinterpret the source-domain business record.

### TB-003 — Version Lineage Integrity

Every synchronized representation shall retain sufficient source/version lineage to determine its technical currentness.

### TB-004 — Freshness Control

A stale or unknown representation shall not be presented through the eligible search boundary as current without successful synchronization/revalidation.

### TB-005 — Access Filtering Integrity

Representations shall pass applicable authorization/visibility filtering before becoming eligible for search consumption; filtering shall not expand authority.

### TB-006 — Synchronization Failure Containment

Contract, source-identity, metadata or validation failure shall prevent the affected representation from being treated as current/eligible and shall preserve the prior valid state where supported by the governed recovery path.

### TB-007 — Recovery Integrity

Rebuild/recovery shall use authoritative source information and shall not fabricate missing business facts.

### TB-008 — Activation Safety

An index or projection shall not become searchable merely because construction completed; required validation, lineage, security and eligibility gates must succeed first.

### TB-009 — Lifecycle Separation

Technical index/readiness states shall not replace or alter H03 business lifecycle states or decisions.

### TB-010 — Scope Preservation

Tenant, Institution and applicable execution scope shall remain preserved across synchronization, filtering, rebuild and activation.

### TB-011 — Failure Isolation

Search-index failure shall not mutate canonical business records or invoke source-domain workflows as an alternative to technical recovery.

### TB-012 — Evidence

Material lifecycle, synchronization, freshness, filtering and recovery outcomes shall produce sufficient technical evidence for diagnosis and traceability.

## 7. Security / Trust / Evidence

### 7.1 Security

- Authorization policy and permission semantics remain owned by applicable L5-D architecture.
- Access filtering shall consume authoritative authorization/visibility context and shall not broaden it.
- Search representations remain subject to applicable data-protection, cryptographic and trust controls.
- Tenant/Institution scope shall be preserved throughout synchronization and index exposure.
- Restricted representations shall not become eligible merely because they are technically present in an index.

### 7.2 Evidence

Technical evidence shall correlate projection identity, source reference, source-version lineage, synchronization outcome, filtering outcome and index eligibility. Canonical business audit meaning and evidence governance remain with L5-D-04.

## 8. Conformance

- **CR-001:** Only an eligible/approved search projection definition may enter technical activation.
- **CR-002:** Search representations shall preserve canonical source identity and ownership.
- **CR-003:** Synchronization shall preserve source-version/synchronization lineage.
- **CR-004:** Stale or unresolved representations shall not silently become current.
- **CR-005:** Applicable authorization/visibility context shall be enforced before search exposure.
- **CR-006:** Access filtering shall not broaden caller authority or redefine authorization policy.
- **CR-007:** Search-index recovery shall use authoritative source information and shall not fabricate business data.
- **CR-008:** Index activation shall require required contract, security, lineage and eligibility validation.
- **CR-009:** Technical index lifecycle shall not redefine H03 business lifecycle semantics.
- **CR-010:** Tenant/Institution and applicable execution scope shall be preserved.
- **CR-011:** B01-001 shall consume the eligible index boundary rather than independently deciding synchronization/freshness eligibility.
- **CR-012:** Material synchronization, filtering and lifecycle failures shall be technically observable.
- **CR-013:** The architecture shall remain independent of an unapproved search technology/product choice.

## 9. Traceability

| L4 BH | L4 Status | L5 Contribution | Evidence |
|---|---|---|---|
| L4-H-018-BH-001 | Draft | Source-change intake, source/version resolution and synchronization admission. | §4.3, §6 TB-003 |
| L4-H-018-BH-002 | Draft | Projection-only synchronization and preservation of source ownership. | §4.3, §6 TB-002 |
| L4-H-018-BH-003 | Draft | Currentness, lifecycle-controlled update and eligible-index gating. | §4.4, §4.5 |
| L4-H-018-BH-004 | Draft | Contract/source uncertainty containment and governed recovery. | §4.8, §6 TB-006/TB-007 |
| L4-H-018-BH-006 | Draft | Source/version/synchronization lineage and technical outcome evidence. | §4.10, §7.2 |
| L4-H-018-BH-007 | Draft | Validated synchronization and eligible current representation. | §4.3–§4.7 |
| L4-H-019-BH-001 | Draft | Preservation of affected projection identity and synchronization lineage before recovery. | §4.8 |
| L4-H-019-BH-002 | Draft | Authoritative-source rebuild/reconciliation architecture. | §4.8 |
| L4-H-019-BH-003 | Draft | Rebuild/activation technical lifecycle and recovery gating. | §4.5, §4.8 |
| L4-H-019-BH-004 | Draft | Validation gate before recovered content becomes searchable. | §4.6, §4.8 |
| L4-H-019-BH-006 | Draft | Recovery action, lineage, validation and resulting eligibility evidence. | §4.10, §7.2 |
| L4-H-019-BH-007 | Draft | Completion through authoritative reconstruction, validation and eligible return. | §4.8 |
| L4-H-020-BH-001 | Draft | Technical admission validation for source ownership, projection scope and security metadata. | §4.2 |
| L4-H-020-BH-002 | Draft | Technical enforcement of governed activation/readiness transitions without redefining business lifecycle. | §4.2, §4.5 |
| L4-H-020-BH-004 | Draft | Activation gate limited to approved attributes/references. | §4.2 |
| L4-H-020-BH-006 | Draft | Technical preservation of participation and activation evidence. | §4.10 |
| L4-H-020-BH-007 | Draft | Active-definition eligibility and constrained search exposure. | §4.2, §4.6 |

The traceability entries above are planning mappings while the referenced L4 specifications remain Draft. Repository-level atomic L4-BH → L5 relation validation shall occur after applicable L4 approval/current-state reconciliation.

## 10. Open Issues / Deferred Design

1. **L4 approval/current-state reconciliation:** L4-H-018, L4-H-019 and L4-H-020 remain Draft in the reviewed branch; Architecture Approval of this L5 specification is gated accordingly unless governance explicitly authorizes the dependency.
2. **Atomic traceability registration:** The final repository-level `(L4 BH ID, L4 Version, L5 Specification ID, L5 Version)` mappings require registration/validation through the approved L5 traceability mechanism.
3. **Authorization integration detail:** Exact authorization contract and policy-decision mechanisms remain with the applicable L5-D architecture and its governed ADRs.
4. **Search technology choice:** No product/provider/framework is selected by this specification. Any material selection remains subject to technology-selection and ADR governance.
5. **Operational recovery mechanics:** Background execution, deployment and infrastructure realization are consumed from their canonical L5 owners rather than duplicated here.

---

**Status:** Draft — Authoring  
**Architecture Level:** L5 — Solution Architecture  
**Canonical Technical Responsibility:** Search Index Lifecycle, Access Filtering & Synchronization Architecture