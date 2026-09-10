# Tenant & Institutional Isolation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-04-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-04 — Tenant & Institutional Isolation Architecture  
**Canonical Technical Responsibility:** Technical Tenant/Institution isolation while preserving the locked business model.  
**Technical Architecture Owner:** L5-A-04 — Tenant & Institutional Isolation Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-10`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Pilot ADR Candidate Disposition v1.0.0; `L5-A-01-001` v1.0.0 Draft; `L4-A-001` v1.0.0 Draft; `L4-A-002` v1.0.0 Draft; `L4-A-003` v1.0.0 Draft; `L4-B-001` v1.0.0 Review; `L4-B-002` v1.0.0 Review; `L4-B-003` v1.0.0 Review; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `L2-B01_Institution_Workflow.md`; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** None yet  
**Applicable ADRs:** Required but unresolved — pilot candidate `ADR-C1`, Canonical Tenant/Institution isolation-enforcement pattern. No canonical ADR ID is allocated yet.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the technical isolation architecture that makes Tenant the mandatory root execution boundary for protected SutraERP operations and constrains any Institution scope to the resolved Tenant, while preserving upstream Tenant and Institution ownership, lifecycle and authority.

This specification establishes isolation semantics, isolation control points and fail-closed execution rules. `L5-A-04-002` separately owns Tenant/Institution context propagation and data-isolation enforcement across execution and persistence.

### In Scope

- Tenant as the mandatory root technical isolation scope for protected execution.
- Institution as a subordinate scope that shall be valid only inside its authoritative Tenant.
- Isolation-scope resolution and consistency validation control points at protected execution entry.
- Fail-closed behavior for missing, ambiguous, conflicting or cross-Tenant scope.
- Consumption of authoritative Tenant/Institution lifecycle facts when determining whether normal business execution is permitted.
- Isolation invariants that remain valid whether technical modules are co-located or selectively isolated.
- Boundary obligations to collaborating authorization, contract, persistence, security, runtime and audit architectures.

### Out of Scope / Non-Goals

- Tenant or Institution business lifecycle ownership, transition authority, business rules or canonical event publication.
- Propagation format, carrier mechanics or lower-scope propagation rules — `L5-A-04-002`.
- Physical persistence isolation, schema, transaction or concurrency design — `L5-B-02-*` and `L5-A-04-002`.
- Authentication/session establishment — `L5-D-01-*`.
- Authorization policy evaluation or authorization-decision ownership — `L5-D-02-*`.
- Contract/event transport mechanics — `L5-C-01-*` / `L5-C-02-*`.
- Cryptographic, network-trust or secrets design — `L5-D-03-*`.
- Process, container, host, deployment or cloud topology — `L5-E-04-*`.
- Universal physical infrastructure per Tenant.
- UI/UX or client navigation design.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-A-04-001` and `L5-A-04-002` rows | A04-001 owns technical Tenant/Institution isolation; A04-002 owns propagation/enforcement of Tenant, Institution and lower-scope context across execution/persistence. | Establishes A04-001 ownership and prevents absorption of A04-002 mechanics. |
| L5 Governance | `L5_Pilot_ADR_Candidate_Disposition_2026-09-09.md` | v1.0.0 / Locked | `ADR-C1` | Canonical Tenant/Institution isolation-enforcement pattern is a material Shared/Family ADR candidate owned by A04; A04-001 isolation semantics/control points must remain distinct from A04-002 propagation/data-isolation mechanics. | Requires `AD-001` to remain provisional until canonical ADR allocation and approval. |
| L5 Peer | `L5-A-01-001_Application_and_Platform_Architecture.md` | v1.0.0 / Draft | §§4–5; `AD-001` | Draft application direction is a Contract-Driven Modular Platform with Selective Isolation and delegates Tenant/Institution isolation to A04. | Requires A04 isolation to remain invariant across modular-core and selectively isolated execution placement. |
| L4 | `L4-A-001_Tenant_Provisioning_to_Active_Tenant_Availability.md` | v1.0.0 / Draft — explicitly authorized for controlled A04 authoring | `BH-002`, `BH-003`, `BH-005`, `BH-009`, `BH-011`, `BH-013` | Tenant remains unavailable for downstream business execution until Active; provisioning establishes one Tenant isolation boundary; Active Tenant context becomes available to downstream setup; failed provisioning is non-operational. | Defines root Tenant isolation and execution-eligibility obligations. |
| L4 | `L4-A-002_Active_Tenant_Governance_Concern_to_Controlled_Restriction_and_Restored_Availability.md` | v1.0.0 / Draft — explicitly authorized for controlled A04 authoring | `BH-003`, `BH-004`, `BH-005`, `BH-006`, `BH-009` | Suspended Tenant prevents normal business execution while identity, isolation boundary and institutional context remain preserved; restoration requires authoritative resolution. | Requires isolation controls to consume authoritative suspension state without owning lifecycle decisions. |
| L4 | `L4-A-003_Active_Tenant_to_Governed_Archival.md` | v1.0.0 / Draft — explicitly authorized for controlled A04 authoring | `BH-003`–`BH-006` | Archived Tenant is read-only, terminal and unavailable for normal business execution while canonical identity/history remain preserved. | Requires terminal non-operational Tenant isolation behavior without treating archival as deletion. |
| L4 | `L4-B-001_Institution_Establishment_to_Final_Activation_Decision.md` | v1.0.0 / Review — explicitly authorized for controlled A04 authoring | `BH-001`, `BH-006` | Institution must resolve to exactly one authoritative Tenant association before progression; Institution becomes Active only through its own governed activation. | Requires Institution scope to be subordinate to an authoritative Tenant and prevents A04 from owning Institution activation. |
| L4 | `L4-B-002_Institution_Suspension_and_Operational_Restoration.md` | v1.0.0 / Review — explicitly authorized for controlled A04 authoring | `BH-002`, `BH-003`, `BH-005`, `BH-006` | Suspended Institution blocks new operational activity while preserving identity/configuration/history; governed restoration returns it to Active. | Requires Institution-level operational isolation without collapsing Tenant scope. |
| L4 | `L4-B-003_Institution_Archival.md` | v1.0.0 / Review — explicitly authorized for controlled A04 authoring | `BH-003`–`BH-007` | Archived Institution is read-only and permanently non-operational while Tenant association and historical relationships remain preserved. | Requires institution-scoped isolation to preserve historical resolution but block new operational use. |
| L2 | `L2-A01_Platform_Tenant_Foundation_Architecture.md` | Locked | Tenant foundation / isolation sections | Tenant Foundation establishes the root SutraERP execution boundary and Tenant isolation context for downstream architecture. | Verifies Tenant as root isolation scope. |
| L2 | `L2-B01_Institution_Workflow.md` | Canonical L2 | Institution rules | One Tenant may own multiple Institutions; Institution Code is unique within a Tenant. | Verifies Institution is subordinate to Tenant and Tenant-scoped rather than globally independent. |
| L1 | `SutraERP_L1_Business_Domain_Architecture_Locked.md` | v2.1 / Locked | §3.1 and canonical ownership rules | Multi-Tenant isolation is defined through the Tenant foundation; institutional hierarchy exists within a single Tenant and business ownership remains canonical. | Prevents isolation mechanics from redefining institutional hierarchy or business ownership. |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Source | L5 Effect |
|---|---|---|---|
| C-001 | Tenant is the root execution/isolation boundary for SutraERP. | L2-A01; L1 §3.1; L4-A-001-BH-003 | Every protected execution shall resolve exactly one Tenant before proceeding. |
| C-002 | Institution exists within one authoritative Tenant association. | L2-B01; L4-B-001-BH-001 | Institution scope may narrow a Tenant scope but may not replace or contradict it. |
| C-003 | Normal business execution is unavailable for non-operational Tenant lifecycle positions defined upstream, including Provisioning/Provisioning Failed, Suspended and Archived as applicable. | L4-A-001; L4-A-002; L4-A-003 | Isolation gating must consume authoritative Tenant lifecycle facts and fail closed when normal execution is not permitted. |
| C-004 | Suspended or Archived Institution shall not admit new normal operational activity. | L4-B-002-BH-002/006; L4-B-003-BH-003/004/006 | Institution-scoped protected execution must consume authoritative Institution operational eligibility. |
| C-005 | Isolation mechanisms do not own Tenant or Institution lifecycle transitions, business decisions or events. | L4-A-001–003; L4-B-001–003; L5 Governance | A04 enforces technical eligibility but never becomes the business owner or publisher. |
| C-006 | Technical isolation does not automatically require separate physical infrastructure. | L5 Governance §8 | A04 defines logical enforcement independent of deployment topology. |
| C-007 | A04-002, B02, D02, D03, C01/C02 and E04 retain ownership of their respective propagation, persistence, authorization, trust, contract and runtime mechanics. | Frozen L5 corpus; L5 Governance §5 | A04-001 defines the invariant/control-point contract without duplicating peer internals. |

### 3.3 Assumptions

No material assumption is treated as approved architecture. The selected layered isolation pattern is recorded as `AD-001` and remains ADR-required through `OD-001`.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The technical meaning of the Tenant isolation boundary at execution time.
- The hierarchical isolation invariant: one resolved Tenant root, with optional Institution scope valid only within that Tenant.
- Isolation entry validation and isolation guard semantics for protected execution.
- Fail-closed behavior where Tenant/Institution scope cannot be proven consistent.
- Consumption rules for authoritative Tenant/Institution operational eligibility.
- The rule that shared runtime or shared infrastructure shall not weaken logical Tenant/Institution isolation.
- The boundary contract that collaborating L5 specifications must honor without transferring their technical ownership to A04-001.

### Consumes / Collaborates With

| Exact L5 Specification / Area | Capability / Collaboration | Boundary |
|---|---|---|
| `L5-A-01-001` | Application/platform structure | A04 isolation applies identically whether execution remains in the modular core or crosses a selectively isolated execution boundary. |
| `L5-A-03-001/002` | Technical module/dependency boundaries | Module composition shall not create an isolation bypass; A04 does not define module dependency enforcement. |
| `L5-A-04-002` | Scope propagation and data-isolation enforcement | A04-001 defines the validated isolation semantics and guards; A04-002 owns carriage, propagation and persistence/data-isolation mechanics. |
| `L5-B-02-*` | Persistence ownership/transaction architecture | B02 owns physical persistence and consistency; A04-001 requires protected persistence to honor the validated isolation boundary through A04-002. |
| `L5-C-01-*` / `L5-C-02-*` | Synchronous/asynchronous technical contracts | Contract owners carry the context required by A04-002; A04-001 does not define wire schemas or transport. |
| `L5-D-01-*` | Authentication/session context | Authentication may provide identity/session evidence; A04 does not equate authenticated identity with valid Tenant/Institution scope. |
| `L5-D-02-*` | Authorization evaluation/enforcement | Authorization consumes correctly scoped resource/execution context; isolation does not substitute for permission. |
| `L5-D-03-*` | Security/trust boundaries | D03 may strengthen isolation through trust controls; A04-001 does not define cryptography or network policy. |
| `L5-D-04-*` | Audit/evidence | A04 supplies isolation-decision context when evidence is required; D04 owns evidence persistence and integrity. |
| `L5-E-04-*` | Runtime/packaging/deployment | E04 realizes placement without weakening A04 invariants or requiring one physical deployment per Tenant by default. |

### Must Not Own

- Tenant or Institution lifecycle state transitions, approval authority or canonical events.
- User authentication or authorization policy decisions.
- Context-propagation carrier mechanics or physical data-isolation mechanism.
- Domain business data or persistence ownership.
- Physical topology, cloud/provider choice or technology product selection.

---

## 5. Solution Architecture

### 5.1 Canonical Isolation Model

`AD-001` establishes the provisional Draft direction selected by the Lead Architect: **layered, fail-closed isolation with validated execution scope**.

The isolation hierarchy is:

```text
Tenant                     mandatory root isolation scope
  └── Institution          optional subordinate scope where the operation is Institution-scoped
        └── lower scope    only where separately defined by authoritative architecture
```

A lower scope may only narrow its parent scope. It shall never replace, widen or contradict the resolved Tenant boundary.

A caller-supplied Tenant or Institution identifier is evidence/input for scope resolution; it is not itself trusted isolation context.

### 5.2 Technical Realization Model

| Element | Type | Responsibility | Input / Context | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|
| Protected Execution Entry | Control point | Marks entry into an operation that can read, mutate, coordinate or expose Tenant/Institution-scoped protected behavior. Requires isolation validation before protected execution. | Available identity/session evidence, requested resource/business context, supplied scope references. | Isolation Resolution Request. | A01, A03, C01/C02, D01 as applicable | Protected execution shall not continue if the required root scope cannot be resolved. | C-001; AD-001. |
| Isolation Scope Resolver | Logical control point | Resolves exactly one authoritative Tenant for the operation and, where required, the applicable Institution association using authoritative architecture/context sources. | Requested operation context and authoritative references. | Resolved Tenant plus optional Institution candidate and provenance. | Platform/Tenant and Institution authoritative sources; A04-002 for propagation mechanics | Multiple, missing or contradictory Tenant resolution is invalid. | C-001/C-002. |
| Scope Consistency Validator | Logical control point | Verifies that any Institution scope belongs to the resolved Tenant and that scope inputs do not conflict. | Resolved Tenant and Institution candidate. | Validated hierarchical isolation scope or rejection. | A04 canonical hierarchy | Cross-Tenant Institution association, ambiguity or conflict fails closed. | C-002; AD-001. |
| Operational Eligibility Gate | Logical control point | Consumes authoritative Tenant and Institution lifecycle facts relevant to normal execution and determines whether the requested protected execution is eligible to proceed. It does not perform lifecycle transitions. | Validated isolation scope plus authoritative lifecycle facts. | Eligible / ineligible technical execution result. | Platform & Tenant owner; Institution owner | Non-operational Tenant or applicable Institution position blocks new normal protected execution. | C-003/C-004/C-005. |
| Isolation Boundary Guard | Enforcement boundary | Admits only execution carrying a validated, eligible isolation scope into the protected technical capability. | Validated isolation scope and eligibility result. | Governed handoff to the owning capability / A04-002 propagation path. | A03, A04-002, D02, B02 as applicable | No fallback to unscoped, caller-trusted or default-Tenant execution is permitted. | AD-001. |
| Peer Architecture Handoff | Governed boundary | Supplies the validated isolation semantics to peer L5 owners without redefining their mechanics. | Validated Tenant/Institution scope and isolation decision. | A04-002 propagation; D02 authorization context; B02 persistence path; D04 evidence context. | Peer L5 owner | Peer failure shall not be bypassed by weakening or reconstructing scope locally. | C-007. |

### 5.3 Primary Protected-Execution Flow

```text
Protected operation arrives
        ↓
Protected Execution Entry
        ↓
Resolve exactly one authoritative Tenant
        ↓
Tenant unresolved / ambiguous / conflicting?
        ├── Yes → reject before protected execution
        └── No
              ↓
      Institution scope required?
        ├── No → Tenant-scoped path
        └── Yes
              ↓
      Resolve Institution association
              ↓
      Institution belongs to Tenant?
        ├── No → reject as isolation mismatch
        └── Yes
              ↓
      Consume authoritative operational eligibility
              ↓
      Tenant / Institution ineligible for normal execution?
        ├── Yes → reject normal protected execution
        └── No
              ↓
      Validated eligible isolation scope
              ↓
      Isolation Boundary Guard
              ↓
      Owning capability executes
              ↓
      A04-002 / D02 / B02 / C01-C02 / D04
      apply their own governed responsibilities
```

The isolation result does not grant business authority. An operation that is correctly scoped may still be denied by D02 authorization or rejected by the owning business capability.

### 5.4 Isolation Semantics by Lifecycle Condition

This table consumes upstream lifecycle meaning; it does not create new lifecycle states.

| Scope | Authoritative condition | A04-001 technical consequence |
|---|---|---|
| Tenant | `Provisioning` | Normal downstream business execution shall not be admitted. |
| Tenant | `Provisioning Failed` | Tenant shall not be exposed as operationally available. |
| Tenant | `Active` | Tenant may be technically eligible for protected execution subject to applicable Institution, authorization and business rules. |
| Tenant | `Suspended` | Normal business execution for that Tenant shall be blocked while identity/history/isolation remain preserved. |
| Tenant | `Archived` | Normal business execution and normal modification shall be blocked; historical resolution remains possible according to upstream rules. |
| Institution | `Active` | Institution-scoped execution may be technically eligible inside its authoritative Tenant subject to authorization/business rules. |
| Institution | `Suspended` | New normal Institution operational activity shall be blocked while permitted historical/reporting access remains governed upstream. |
| Institution | `Archived` | New operational activity and normal modification shall be blocked; Tenant association/history remain resolvable. |

A04-001 shall not invent an operational outcome for any lifecycle condition not established by authoritative upstream architecture.

### 5.5 Cross-Tenant Failure Model

The following conditions are material isolation failures:

| Failure Class | Required Outcome | Forbidden Outcome | Delegated / Collaborating Owner |
|---|---|---|---|
| Tenant cannot be resolved exactly once | Fail closed before protected execution. | Defaulting to a Tenant, guessing from local state or accepting an unverified caller claim. | A04-002/C01/D01 as applicable for context carriage/evidence. |
| Institution resolves under another Tenant | Fail closed as a Tenant/Institution consistency violation. | Widening the Tenant boundary, switching Tenant implicitly or continuing with Institution-only scope. | A04-002 and owning Institution context source. |
| Conflicting scope evidence across technical boundaries | Reject/re-resolve through authoritative sources before continuation. | Selecting whichever scope value is most convenient or locally available. | A04-002/C01/C02. |
| Authoritative Tenant is non-operational for normal execution | Block normal protected execution. | Continuing because authentication succeeded or a module is technically reachable. | Platform & Tenant lifecycle owner; D02 remains separate. |
| Applicable Institution is non-operational for normal execution | Block new Institution operational execution. | Treating Active Tenant as permission to bypass Institution suspension/archive semantics. | Institution lifecycle owner. |
| Shared runtime or co-located module attempts direct cross-Tenant access | Reject as an isolation-boundary violation. | Treating process/database/network co-location as authority to cross scope. | A03/A04-002/B02/D03/E04 as applicable. |

### 5.6 Shared Runtime and Selective Isolation

A04 isolation is logical and mandatory regardless of physical/runtime placement.

```text
Same process / same host / shared infrastructure
        ≠
shared Tenant authority

Separate process / isolated execution unit
        ≠
new business ownership
```

The A01 modular-core/selective-isolation model may place capabilities differently, but every protected path remains subject to the same Tenant-rooted isolation semantics. D03/E04 may require stronger trust or physical separation for a specific capability; A04-001 does not require universal per-Tenant infrastructure.

### 5.7 Context, Persistence and Authorization Boundary

A04-001 defines the validated isolation semantics only.

- `L5-A-04-002` shall define how Tenant, Institution and lower-scope context is propagated and how isolation is enforced across execution and persistence boundaries.
- `L5-B-02-*` shall own persistence/data-placement and transaction mechanisms.
- `L5-D-02-*` shall own authorization decisions and enforcement.
- `L5-C-01-*` / `L5-C-02-*` shall own contract/message representation and transport.

No component may compensate for a missing A04 isolation result by treating successful authentication, authorization, contract validation or database connectivity as proof of Tenant/Institution scope.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-001` | Canonical Tenant/Institution isolation pattern | **Provisional Draft direction:** layered, fail-closed isolation with exactly one validated Tenant root and optional Institution scope valid only within that Tenant. | Frozen A04 responsibility; L2-A01; L2-B01; L4-A-001–003; L4-B-001–003; Lead Architect selection on 2026-09-10. | Enforces the locked business hierarchy without relying on every downstream component to independently infer scope and without requiring universal physical isolation. | **Required — pilot candidate `ADR-C1`; no canonical ADR ID allocated. See `OD-001`.** |
| `AD-002` | Trust treatment of supplied scope identifiers | Tenant/Institution identifiers supplied by a caller, request, job or message are scope claims/evidence until resolved and validated; protected execution consumes only the validated isolation result. | Technical derivation necessary to enforce C-001/C-002 and fail closed. | Prevents caller-controlled or stale identifiers from becoming isolation authority. | Expected to be governed within ADR-C1 scope; do not allocate separately unless ADR scoping determines otherwise. |
| `AD-003` | Physical isolation default | A04-001 does not require one physical runtime/database/infrastructure instance per Tenant. Logical isolation is mandatory; stronger physical/trust isolation may be required by canonical D03/E04/B02 architecture for specific cases. | L5 Governance §8; A01 boundary. | Preserves strong isolation without prematurely owning deployment/persistence topology. | Expected to be governed within ADR-C1 scope where material. |

No database, framework, IAM product, broker, cloud provider, orchestration platform or deployment technology is selected by this specification.

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaboration |
|---|---|---|
| Cross-Tenant confidentiality/integrity | Ambiguous or inconsistent Tenant/Institution scope fails closed before protected execution. | A04-001 with A04-002/B02/D03. |
| Authorization separation | Correct isolation scope is necessary but not sufficient for authorization. | D02. |
| Shared-runtime safety | Co-location shall not weaken Tenant/Institution boundaries or permit cross-scope internal access. | A03/A04-002/B02/D03/E04. |
| Lifecycle restriction | Suspended/Archived Tenant or Institution conditions remain technically enforceable without transferring lifecycle ownership. | Upstream owner + A04-001. |
| Portability | Isolation semantics are independent of on-premises/cloud/runtime product choice. | E04. |
| Auditability | Isolation rejection and scope-validation evidence may be handed to D04 without A04 owning audit storage. | D04. |

No numeric security, latency, capacity, recovery or retention targets are invented here.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From |
|---|---|---|
| `CR-001` | Every protected execution shall resolve exactly one authoritative Tenant before protected Tenant-scoped business processing continues. | `AD-001`, C-001, §5.3 |
| `CR-002` | An Institution-scoped operation shall prove that the Institution belongs to the resolved Tenant; Institution scope shall never replace, widen or contradict Tenant scope. | C-002, `AD-001`, §5.1 |
| `CR-003` | Missing, ambiguous or conflicting Tenant scope shall cause fail-closed rejection rather than default-Tenant selection, scope guessing or unscoped continuation. | `AD-001`, `AD-002`, §5.5 |
| `CR-004` | Caller/request/job/message supplied Tenant or Institution identifiers shall not by themselves be treated as validated isolation authority. | `AD-002` |
| `CR-005` | Normal protected execution shall not proceed for a Tenant whose authoritative lifecycle position makes it non-operational under the applicable upstream behavior. | C-003, §5.4 |
| `CR-006` | Institution-scoped normal operational execution shall not proceed where the authoritative Institution lifecycle condition makes new operational activity unavailable. | C-004, §5.4 |
| `CR-007` | A04 isolation behavior shall not create, modify or publish Tenant/Institution business lifecycle transitions, decisions or canonical events. | C-005 |
| `CR-008` | Shared process, host, network or persistence infrastructure shall not be treated as permission to cross Tenant/Institution isolation boundaries. | `AD-003`, §5.6 |
| `CR-009` | A04-001 shall hand validated isolation semantics to A04-002/D02/B02/C01-C02/D04 as applicable without duplicating their canonical technical ownership. | C-007, §§4, 5.7 |
| `CR-010` | Runtime/deployment realization may strengthen isolation but shall not weaken the Tenant-rooted isolation invariant or redefine business ownership. | `AD-003`, A01, §5.6 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| `OD-001` | Canonical ADR for the Tenant/Institution isolation-enforcement pattern covering A04-001 semantics/control points and A04-002 propagation/data-isolation mechanics. | ADR / Shared-Family | Lead Architect through L5 ADR governance | **Yes — blocks Architecture Approved/Locked status** | Re-check ADR necessity/scope, allocate the next canonical ADR ID only in `L5_ADR_Register.md` on `main`, author/evaluate the ADR, then reference the approved decision here. |
| `OD-002` | Canonical repository evidence for approved controlled use of Draft/Review `L4-A-001/002/003` and `L4-B-001/002/003`. | Governance / Upstream | Lead Architect / repository governance | **Yes — blocks promotion unless upstream states advance or authorization is canonicalized** | Before Architecture Approved status, either approve applicable L4 sources or record the explicit 2026-09-10 Lead-Architect authorization in canonical governance. |
| `OD-003` | Exact A04-002 context carrier and persistence/data-isolation realization. | Peer L5 dependency | `L5-A-04-002` with B02/C01/C02 dependencies | No for this Draft; required before end-to-end isolation architecture is complete | Author `L5-A-04-002` under the same ADR-C1 scope without moving its mechanics into A04-001. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-001-BH-002/003/005/009/011/013` — Tenant environment/isolation and operational availability.
- `L4-A-002-BH-003/004/005/006/009` — Tenant suspension restriction and governed restoration.
- `L4-A-003-BH-003/004/005/006` — Tenant archival, terminal non-operational behavior and historical preservation.
- `L4-B-001-BH-001/006` — exactly one Tenant association and governed Institution activation.
- `L4-B-002-BH-002/003/005/006` — Institution suspension/restoration operational restrictions.
- `L4-B-003-BH-003/004/005/006/007` — Institution archival and permanent non-operational behavior.

### Higher-Level Verification

- `L2-A01_Platform_Tenant_Foundation_Architecture.md` — Tenant root execution/isolation boundary.
- `L2-B01_Institution_Workflow.md` — one Tenant may contain multiple Institutions; Institution identity/code is Tenant-scoped.
- Locked L1 §3.1 — institutional hierarchy exists within a single Tenant; multi-Tenant isolation remains Platform/Tenant-owned.

### Material L5 Dependencies

- `L5-A-01-001` — application/platform placement.
- `L5-A-04-002` — context propagation and data-isolation enforcement.
- `L5-B-02-*` — persistence architecture.
- `L5-C-01-*` / `L5-C-02-*` — synchronous/asynchronous contract mechanics.
- `L5-D-01-*` / `L5-D-02-*` / `L5-D-03-*` / `L5-D-04-*` — authentication, authorization, trust and evidence.
- `L5-E-04-*` — runtime/deployment realization.

### Decisions / Conformance / Blockers

- Architecture decisions: `AD-001`–`AD-003`.
- Conformance requirements: `CR-001`–`CR-010`.
- Blocking items for promotion: `OD-001`, `OD-002`.
- End-to-end A04 family completion dependency: `OD-003`.
