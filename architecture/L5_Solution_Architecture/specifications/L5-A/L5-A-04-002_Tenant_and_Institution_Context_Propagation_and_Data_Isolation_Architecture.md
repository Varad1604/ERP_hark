# Tenant and Institution Context Propagation & Data Isolation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-04-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-04 — Tenant & Institutional Isolation Architecture  
**Canonical Technical Responsibility:** Propagation/enforcement of Tenant, Institution and lower-scope context across execution and persistence.  
**Technical Architecture Owner:** L5-A-04 — Tenant & Institutional Isolation Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-10`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Pilot ADR Candidate Disposition v1.0.0; `L5-A-04-001` v1.0.0 Draft; `L5-A-01-001` v1.0.0 Draft; `L4-A-001` v1.0.0 Draft; `L4-A-007` v1.0.0 Draft; `L4-A-015` v1.0.0 Draft; `L4-A-019` v1.0.0 Draft; `L4-A-050` v1.0.0 Draft; `L4-B-001` v1.0.0 Review; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `L2-B01_Institution_Workflow.md`; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** None yet  
**Applicable ADRs:** Required but unresolved — pilot candidate `ADR-C1`, Canonical Tenant/Institution isolation-enforcement pattern. No canonical ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the technical architecture required to preserve the validated Tenant-rooted isolation scope established by `L5-A-04-001` as execution crosses technical boundaries and reaches persistence.

This specification ensures that Tenant, applicable Institution and authoritative lower-scope context cannot be silently omitted, widened, substituted, reconstructed from untrusted input or bypassed during synchronous execution, asynchronous execution, workflow/background processing or persistence access.

### In Scope

- Technical representation of validated Tenant/Institution execution scope.
- Propagation of validated scope across synchronous and asynchronous technical boundaries.
- Scope narrowing and scope-change revalidation.
- Consumption of Tenant/Institution scope by workflow and background execution.
- Persistence-boundary scope enforcement requirements.
- Read and write isolation expectations for Tenant-scoped information.
- Detection and fail-closed handling of lost, inconsistent, widened or substituted scope.
- Requirements imposed on collaborating L5 architectures without duplicating their internal mechanics.

### Out of Scope / Non-Goals

- Defining Tenant/Institution isolation semantics or lifecycle eligibility — `L5-A-04-001`.
- Authentication/session establishment — L5-D01.
- Authorization policy evaluation or Authorization Decision ownership — L5-D02.
- Physical database, schema, transaction or concurrency architecture — L5-B02.
- Exact API/request fields or transport protocols — L5-C01.
- Event/message schema and broker/delivery mechanics — L5-C02.
- Workflow-engine internals — L5-A05.
- Background worker/job infrastructure — L5-A08.
- Cryptographic or network trust controls — L5-D03.
- Runtime/deployment topology — L5-E04.
- UI context-selection behavior — L6.
- Creation of new lower-scope business hierarchy.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-A-04-002 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-A-04-001`, `L5-A-04-002` | A04-001 owns isolation semantics/control points; A04-002 owns propagation/enforcement across execution and persistence. | Establishes canonical ownership split. |
| L5 Governance | `L5_Pilot_ADR_Candidate_Disposition_2026-09-09.md` | v1.0.0 / Locked | `ADR-C1` | Tenant/Institution isolation-enforcement pattern is a material Shared/Family ADR candidate covering A04-001 and A04-002 without merging their responsibilities. | Keeps propagation/enforcement architecture provisional pending canonical ADR handling. |
| L5 Peer | `L5-A-04-001_Tenant_and_Institutional_Isolation_Architecture.md` | v1.0.0 / Draft | §§5.1–5.7; `AD-001` | Exactly one Tenant is the root isolation scope; Institution may only narrow that Tenant; ambiguous/conflicting scope fails closed. | Supplies the isolation semantics that A04-002 propagates and enforces. |
| L4 | `L4-A-001_Tenant_Provisioning_to_Active_Tenant_Availability.md` | v1.0.0 / Draft — authorized for controlled A04 use | `BH-003`, `BH-005`, `BH-009` | Tenant isolation is established during provisioning and authoritative Active Tenant context becomes available to permitted downstream consumers. | Requires technical downstream propagation without ownership transfer. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.0 / Draft — authorized for controlled A04 use | `BH-001`, `BH-007`, `BH-008`, `BH-009` | Authentication occurs within exactly one Tenant and makes authenticated context available to Authorization without granting business authority. | Authentication/session context is an input to scope propagation but shall not substitute for A04 validation. |
| L4 | `L4-A-015_Workflow_Initiation_to_Completed_Orchestrated_Execution.md` | v1.0.0 / Draft — authorized for controlled A04 use | `BH-001`, `BH-002`, `BH-004`, `BH-005`, `BH-011` | Workflow execution occurs within one Tenant using valid execution context/correlation while preserving domain ownership. | Requires Tenant scope to survive workflow initiation and downstream coordinated execution. |
| L4 | `L4-A-019_Committed_Business_Fact_to_Reliable_Consumer_Delivery.md` | v1.0.0 / Draft — authorized for controlled A04 use | `BH-002`, `BH-005`, `BH-007`, `BH-010` | Event publication validates Tenant context and routes only to compatible subscriptions within authorized Tenant scope. | Requires scope preservation across asynchronous publication/delivery boundaries. |
| L4 | `L4-A-050_Workspace_Definition_to_Governed_Resolvable_Experience_Context.md` | v1.0.0 / Draft — authorized for controlled A04 use | `BH-001`, `BH-003`, `BH-006`, `BH-008` | Workspace resolution operates within resolvable Tenant/Institution context and context change requires revalidation. | Supports scope-change revalidation without converting context selection into authorization. |
| L4 | `L4-B-001_Institution_Establishment_to_Final_Activation_Decision.md` | v1.0.0 / Review — authorized for controlled A04 use | `BH-001` | Every Institution resolves to exactly one authoritative Tenant association. | Constrains Institution propagation to its parent Tenant. |
| L1 | `SutraERP_L1_Business_Domain_Architecture_Locked.md` | v2.1 / Locked | §§2.1, 2.4; §3.1 | Canonical business ownership is singular; direct cross-domain authoritative persistence bypass is prohibited; institutional hierarchy exists inside Tenant context. | Prevents context propagation/data isolation from becoming shared ownership or cross-domain persistence authority. |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Source | L5 Effect |
|---|---|---|---|
| `C-001` | Validated Tenant scope is mandatory for protected Tenant-scoped execution. | A04-001; L2-A01 | Every protected technical boundary must preserve a resolvable Tenant context. |
| `C-002` | Institution context is valid only inside its authoritative Tenant. | A04-001; L4-B-001-BH-001 | Propagation shall preserve parent-child scope consistency. |
| `C-003` | Authentication does not itself grant authorization and does not replace A04 isolation validation. | L4-A-007-BH-008 | Authenticated session context may contribute evidence but cannot become the sole isolation authority. |
| `C-004` | Workflow execution is Tenant-scoped. | L4-A-015-BH-001/011 | Workflow instances and downstream actions require preserved Tenant context. |
| `C-005` | Event publication/routing is Tenant-scoped. | L4-A-019-BH-002/005/010 | Asynchronous propagation shall not lose or switch Tenant scope. |
| `C-006` | Context changes require revalidation. | L4-A-050-BH-008 | Subsequent execution after a scope/context change requires a newly validated applicable scope. |
| `C-007` | Persistence ownership remains with the canonical business/technical owner. | L1 §§2.1/2.4; L5 Governance | A04-002 constrains access using scope but does not become data owner. |
| `C-008` | Physical implementation details belong to B02/C01/C02/A05/A08/D01/D02/D03/E04 as applicable. | Frozen L5 corpus | A04-002 defines enforcement obligations and handoffs, not peer internals. |

### 3.3 Assumptions

No product, protocol, schema, database or framework assumption is introduced.

Exact carrier formats and peer-specific implementation mechanisms remain dependent on the respective canonical L5 specifications and are recorded under `OD-002`.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The architecture concept of a **Validated Execution Scope Context** carrying Tenant, applicable Institution and authoritative lower-scope references.
- The requirement to propagate validated scope across every protected technical handoff.
- Scope narrowing rules and revalidation requirements when effective scope changes.
- Detection and fail-closed handling of scope omission, substitution, widening or contradiction.
- The technical requirement that Tenant-scoped persistence access consume validated scope.
- The prohibition on unscoped Tenant-data access paths.
- Context inheritance requirements for workflow/background/asynchronous continuation.
- Isolation requirements for returned/result data relative to the execution scope.

### Consumes / Collaborates With

| L5 Architecture | Consumed Responsibility | A04-002 Boundary |
|---|---|---|
| `L5-A-04-001` | Isolation hierarchy, scope validity and execution eligibility | A04-002 carries/enforces those semantics; it does not redefine them. |
| `L5-A-01-001` | Modular-core/selective-isolation application structure | Scope must survive either form of placement. |
| `L5-A-03-001/002` | Technical module/dependency boundaries | Every cross-module boundary shall preserve A04 scope without A04 owning dependency enforcement. |
| `L5-A-05-*` | Workflow/orchestration mechanics | Workflow execution consumes propagated scope; A04 does not own workflow state. |
| `L5-A-08-*` | Background processing/job mechanics | Jobs consume captured validated scope; A04 does not own queues/retries/workers. |
| `L5-B-02-*` | Persistence, transactions and concurrency | B02 implements persistence mechanics subject to A04 scope-enforcement requirements. |
| `L5-C-01-*` | Synchronous contracts | C01 carries required scope information in governed technical contracts. |
| `L5-C-02-*` | Event/message contracts and delivery | C02 carries scope through asynchronous boundaries. |
| `L5-D-01-*` | Authentication/session architecture | D01 supplies authenticated identity/session context; A04 independently validates execution scope. |
| `L5-D-02-*` | Authorization | D02 consumes valid scope when making/enforcing Authorization Decisions. |
| `L5-D-03-*` | Trust/security controls | D03 determines protection/integrity mechanisms around context crossing trust boundaries. |
| `L5-D-04-*` | Audit/evidence | D04 preserves required isolation/context evidence. |
| `L5-E-04-*` | Runtime/deployment | Scope semantics remain invariant across process/deployment boundaries. |

### Must Not Own

- Tenant/Institution business ownership or lifecycle.
- Authentication Session ownership.
- Role/Permission/Authorization Decision ownership.
- Business data ownership.
- Database technology or schema.
- API/event schema implementation.
- Workflow/job lifecycle.
- Deployment topology.

---

## 5. Solution Architecture

### 5.1 Validated Execution Scope Context

A04-002 defines a conceptual technical context representing scope that has successfully passed A04-001 isolation validation.

It shall contain only architecture-relevant scope information:

```text
Validated Execution Scope Context

Tenant Reference                     REQUIRED

Institution Reference                CONDITIONAL
                                     required when the operation is
                                     Institution-scoped

Lower-Scope Reference Chain          CONDITIONAL
                                     only for scopes defined by
                                     authoritative owning architecture

Validation Provenance                REQUIRED in a form sufficient
                                     for the receiving boundary to
                                     distinguish validated scope from
                                     caller-supplied scope claims
```

This is an architecture contract, not an API payload, database schema or software class.

The context does not contain or create business authorization.

### 5.2 Scope Propagation Invariant

For a protected execution path:

```text
Validated Scope S
      ↓
Technical Boundary 1
      ↓
Technical Boundary 2
      ↓
Workflow / Message / Job
      ↓
Persistence
```

every downstream protected operation shall operate under:

```text
same scope S

OR

a valid narrower scope derived from S
and revalidated against authoritative hierarchy
```

The following is prohibited:

```text
Tenant T1
   ↓
boundary
   ↓
Tenant missing

Tenant T1
   ↓
boundary
   ↓
Tenant T2

Tenant T1 / Institution I1
   ↓
boundary
   ↓
Institution I2 under another Tenant

Tenant T1
   ↓
boundary
   ↓
unscoped persistence access
```

### 5.3 Scope Narrowing and Scope Change

A downstream operation may narrow scope where authoritative architecture requires it.

Example:

```text
Tenant T1
      ↓
Tenant T1 / Institution I1
      ↓
Tenant T1 / Institution I1 / authorised lower scope
```

A child scope shall never widen or replace the Tenant root.

A material effective-context change, including selection of a different Institution or another governed experience context affecting scope, requires revalidation before subsequent protected execution.

The architecture shall not silently mutate the isolation scope of already-running protected work.

### 5.4 Synchronous Propagation

For a protected synchronous interaction:

```text
Validated Execution Scope
        ↓
C01-governed technical contract boundary
        ↓
Receiving execution boundary
        ↓
Scope presence / consistency check
        ↓
Protected target capability
```

C01 owns the actual contract representation.

A04-002 requires that the contract provide sufficient information to preserve the applicable validated Tenant/Institution scope.

A receiving capability shall not reconstruct missing Tenant scope from:

- a default Tenant;
- global mutable state;
- unrelated local configuration;
- an Institution identifier without validating its Tenant;
- successful authentication alone.

### 5.5 Workflow and Coordinated Execution

A workflow initiated for Tenant `T1` shall remain associated with `T1` throughout its governed execution.

```text
Workflow initiation
      ↓
Validated Tenant / Institution scope
      ↓
Workflow Instance
      ↓
step
      ↓
domain-owned action
      ↓
wait / resume
      ↓
later step
```

Continuation, waiting, suspension or resumption shall not cause the isolation scope to disappear or change implicitly.

Where a workflow step legitimately targets a narrower authoritative scope, that scope must remain a descendant of the workflow's Tenant root and be validated before execution.

A05 owns workflow state and orchestration mechanics.

### 5.6 Asynchronous Event / Message Propagation

When execution crosses an asynchronous boundary:

```text
Publisher execution
      ↓
Validated Tenant scope
      ↓
C02-governed Event / Message boundary
      ↓
Tenant-scoped routing
      ↓
Consumer receives
      ↓
Scope validation
      ↓
Consumer protected execution
```

An event or message associated with Tenant-scoped execution shall not become an unscoped consumer instruction.

A consumer shall reject protected execution when the required Tenant context is absent, conflicting or incompatible with the target resource context.

This requirement does not transfer event-publisher ownership to A04.

### 5.7 Background / Deferred Work

A deferred or background operation derived from protected execution shall preserve the validated scope required to continue the work.

```text
Foreground protected execution
        ↓
validated scope
        ↓
background work requested
        ↓
A08-owned job execution
        ↓
scope restored / validated
        ↓
protected work continues
```

The worker shall not infer Tenant scope from whichever data happens to be loaded first.

Retries, schedules, queues, workers and recovery mechanics remain A08-owned.

### 5.8 Persistence Isolation Boundary

For information governed as Tenant-scoped, every protected persistence operation shall receive and enforce the validated Tenant scope.

Where the data is Institution-scoped, the applicable Institution shall additionally remain constrained to the validated Tenant.

Conceptually:

```text
Protected capability
      ↓
Validated Execution Scope
      ↓
B02-owned scoped persistence boundary
      ↓
READ / WRITE
      ↓
only information valid for that execution scope
```

A04-002 does not prescribe the database mechanism.

B02 may realize the requirement through architecture it canonically owns, but the outcome shall satisfy these invariants:

```text
Tenant-scoped READ
→ cannot expose another Tenant's protected information

Tenant-scoped WRITE
→ cannot modify another Tenant's authoritative information

Institution-scoped READ/WRITE
→ Institution must belong to the validated Tenant

Missing validated Tenant scope
→ no protected Tenant-scoped persistence operation
```

### 5.9 Query / Result Isolation

Isolation applies to results as well as input filters.

A technically valid persistence operation is non-conformant if it can return Tenant-scoped information outside the validated execution scope.

Therefore:

```text
request scope
      ↓
persistence enforcement
      ↓
result set
      ↓
result scope must remain
within validated execution scope
```

The existence of a caller-supplied Tenant filter is not sufficient proof of isolation.

### 5.10 Cross-Module and Isolated-Execution Propagation

The same A04 context semantics apply whether the next capability is:

```text
inside the same modular application runtime

OR

inside another technical module

OR

inside a selectively isolated execution unit
```

Co-location does not permit omission of required isolation enforcement.

Physical separation does not create a new Tenant identity or new business ownership.

### 5.11 Context Validation at Trust / Ownership Boundaries

When scope crosses a material technical boundary, the receiving side shall establish that the propagated scope is present and consistent with the resource or operation it is about to process.

A04-002 defines this validation obligation.

D03 owns any cryptographic, network, credential or trust mechanism used to protect the context.

### 5.12 Material Failure Model

| Failure | Required Outcome | Forbidden Outcome |
|---|---|---|
| Required Tenant context missing | Reject protected continuation. | Default or infer Tenant. |
| Tenant context changes between hops without governed revalidation | Reject continuation. | Accept new Tenant silently. |
| Institution does not belong to propagated Tenant | Reject as scope inconsistency. | Continue with Institution-only scope. |
| Lower scope is not valid under parent hierarchy | Reject affected protected operation. | Widen parent context. |
| Asynchronous message lacks required scope | Do not execute protected consumer action. | Use consumer's local/default Tenant. |
| Background work cannot restore required scope | Do not execute protected work. | Query data first to discover Tenant. |
| Persistence access lacks validated Tenant context | Deny protected Tenant-scoped read/write. | Execute an unscoped query/write. |
| Persistence result contains another Tenant's protected information | Treat as isolation failure. | Return/filter only at a later client/UI layer. |
| Context conflicts with protected resource ownership | Reject/re-resolve using authoritative context. | Prefer caller context over resource ownership. |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Rationale | ADR |
|---|---|---|---|---|
| `AD-001` | Context propagation model | **Provisional Draft direction:** one validated Tenant-rooted execution-scope context shall accompany protected execution across technical boundaries; context loss or contradiction fails closed. | Direct realization of A04-001 layered isolation without requiring downstream components to independently infer Tenant scope. | Part of unresolved `ADR-C1`. |
| `AD-002` | Scope evolution | Scope may remain unchanged or narrow through authoritative child scopes. Widening, Tenant replacement or material context change requires new validation. | Preserves Tenant as root boundary and supports L4-A-050 revalidation semantics. | Expected within `ADR-C1`. |
| `AD-003` | Persistence isolation contract | Tenant-scoped persistence operations shall consume validated scope; unscoped Tenant-data reads/writes are non-conformant. B02 owns the physical enforcement mechanism. | Prevents isolation from depending solely on application filters while preserving B02 ownership. | Joint A04/B02 boundary; exact mechanism unresolved until B02 architecture stabilizes. |
| `AD-004` | Asynchronous/deferred propagation | Workflow, event/message and background continuation shall preserve the required validated Tenant root rather than rediscovering it from arbitrary local data. | Necessary because L4 workflow and event behavior is explicitly Tenant-scoped. | Part of A04 isolation pattern; peer mechanics remain separately owned. |

No specific programming framework, context library, database, ORM, message broker, workflow engine, queue, IAM product, cloud platform or deployment technology is selected.

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Collaboration |
|---|---|---|
| Cross-Tenant confidentiality | Reads/results outside validated Tenant scope are prohibited. | B02/D03 |
| Cross-Tenant integrity | Writes outside validated Tenant scope are prohibited. | B02 |
| Context substitution | Caller-supplied or downstream-reconstructed identifiers cannot silently replace validated scope. | C01/C02/D01 |
| Async isolation | Tenant scope survives event/message/job boundaries. | C02/A08 |
| Workflow isolation | Tenant scope survives workflow continuation/wait/resume. | A05 |
| Authorization separation | Valid scope is an input to authorization, not an Authorization Decision. | D02 |
| Deployment independence | Propagation/isolation semantics remain valid under co-located or isolated runtime placement. | E04 |
| Auditability | Isolation failures and context decisions may provide evidence to D04 without A04 owning evidence storage. | D04 |

No numeric performance, retention, recovery or capacity targets are established.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From |
|---|---|---|
| `CR-001` | Every protected Tenant-scoped execution path shall retain a resolvable validated Tenant root through every material technical handoff. | `AD-001` |
| `CR-002` | Institution and authoritative lower-scope context shall only narrow the validated Tenant hierarchy and shall not replace or widen the Tenant root. | `AD-002` |
| `CR-003` | A boundary receiving missing, ambiguous or conflicting required scope shall fail closed before protected continuation. | `AD-001`, §5.12 |
| `CR-004` | Context supplied by a caller, request, session, message or job shall not automatically be treated as validated isolation authority. | A04-001; §5.4 |
| `CR-005` | A material scope change shall require revalidation before subsequent protected execution. | `AD-002`; L4-A-050-BH-008 |
| `CR-006` | Workflow execution shall retain its Tenant-rooted scope across initiation, steps, waits and governed continuation. | §5.5; L4-A-015 |
| `CR-007` | Tenant-scoped asynchronous delivery shall preserve sufficient scope for the consumer to reject incompatible or unscoped protected execution. | §5.6; L4-A-019 |
| `CR-008` | Background/deferred protected work shall preserve or restore its validated Tenant-rooted context before execution and shall not infer Tenant from arbitrary work data. | §5.7 |
| `CR-009` | Tenant-scoped persistence reads and writes shall be rejected when validated Tenant scope is absent. | `AD-003`; §5.8 |
| `CR-010` | Institution-scoped persistence access shall prove that the Institution belongs to the validated Tenant. | `AD-003`; C-002 |
| `CR-011` | Persistence results for Tenant-scoped protected operations shall not expose protected information outside the validated Tenant scope. | §5.9 |
| `CR-012` | Shared runtime, shared database technology or technical co-location shall not be treated as authority to bypass A04 scope propagation/enforcement. | §5.10 |
| `CR-013` | Authentication success shall not substitute for Tenant/Institution scope validation or authorization. | C-003 |
| `CR-014` | A04-002 shall not create Tenant/Institution lifecycle transitions, business authorization decisions, database ownership or business events while enforcing technical scope. | §§2,4 |
| `CR-015` | Peer L5 implementations shall preserve A04 scope semantics without requiring A04-002 to duplicate their internal contract, persistence, workflow, security or runtime architecture. | C-008 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| `OD-001` | Canonical ADR for the complete Tenant/Institution isolation-enforcement pattern covering A04-001 and A04-002. | ADR / Shared-Family | Lead Architect through L5 ADR governance | **Yes — blocks Architecture Approved/Locked** | Scope `ADR-C1`, allocate the next canonical ADR ID on `main`, evaluate and approve before promotion. |
| `OD-002` | Exact context-carrier representations and enforcement mechanics in C01, C02, A05, A08, B02, D01/D02/D03. | Peer L5 dependency | Respective canonical L5 technical owners | **Yes for final cross-spec approval; not for this Draft** | Reconcile A04-002 conformance requirements with stable peer specifications without transferring ownership. |
| `OD-003` | Exact physical persistence mechanism satisfying scoped read/write/result isolation. | Peer L5 / ADR boundary | B02 with A04 constraints | **Yes for final persistence realization; not for this Draft** | B02 shall select/define the physical persistence isolation architecture while conforming to `AD-003`/CR-009–CR-011. |
| `OD-004` | Canonical repository evidence for controlled Draft/Review L4 use. | Governance / Upstream | Lead Architect / repository governance | **Yes — blocks promotion unless upstream is approved or authorization is canonicalized** | Approve applicable L4 sources or record the 2026-09-10 A04 Draft/Review usage authorization in canonical governance. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-001-BH-003/005/009` — canonical Tenant isolation and downstream Active Tenant context.
- `L4-A-007-BH-001/007/008/009` — exactly-one-Tenant authentication/session context and downstream authenticated context.
- `L4-A-015-BH-001/002/004/005/011` — one-Tenant workflow execution and preserved execution context.
- `L4-A-019-BH-002/005/007/010` — Tenant validation, Tenant-scoped subscription routing and delivery traceability.
- `L4-A-050-BH-001/003/006/008` — Tenant/Institution context resolution and revalidation on context change.
- `L4-B-001-BH-001` — exactly one authoritative Tenant association for Institution.

### Material Higher-Level Constraints

- L2-A01 — Tenant is the root execution/isolation boundary.
- L2-B01 — Institution exists inside a Tenant context.
- Locked L1 §§2.1/2.4 — technical sharing shall not create cross-owner persistence authority.

### Material L5 Dependencies

- `L5-A-04-001` — isolation semantics and fail-closed eligibility.
- `L5-A-01-001` — application placement.
- `L5-A-03-*` — module/dependency boundaries.
- `L5-A-05-*` / `L5-A-08-*` — workflow and deferred execution.
- `L5-B-02-*` — persistence realization.
- `L5-C-01-*` / `L5-C-02-*` — synchronous/asynchronous transport.
- `L5-D-01-*` / `D02-*` / `D03-*` / `D04-*` — identity, authorization, trust and evidence.
- `L5-E-04-*` — runtime/deployment.

### Architecture State

Architecture decisions: `AD-001`–`AD-004`.  
Conformance requirements: `CR-001`–`CR-015`.  
Blocking promotion items: `OD-001`, `OD-002`, `OD-003`, `OD-004`.

**Draft conclusion:** The specification makes Tenant/Institution scope propagation and persistence isolation technically explicit while preserving A04-001 isolation semantics and canonical ownership of persistence, authorization, contracts, workflow, security and runtime architecture.
