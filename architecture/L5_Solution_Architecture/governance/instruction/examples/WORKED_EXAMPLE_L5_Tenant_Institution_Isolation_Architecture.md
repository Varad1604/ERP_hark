# SutraERP L5 — Worked Authoring Example — Tenant & Institutional Isolation

**Training Status:** Non-canonical example only  
**Purpose:** Show the expected shape, depth and reasoning of a mass-authored L5 specification  
**Modelled On:** `L5-A-04-001` — Tenant & Institutional Isolation Architecture  
**Part of Frozen 105 Production Corpus:** No  
**May Be Submitted as Canonical L5-A-04-001:** No  

> This file is intentionally an annotated training example. It demonstrates how a real L5 production specification should be structured and how much technical architecture is expected. It does not replace the canonical `L5-A-04-001` production specification. The real production file must be authored under `architecture/L5_Solution_Architecture/specifications/L5-A/` using the locked template and current authoritative sources.

---

# 1. Specification Metadata

**Specification ID:** Training example modelled on `L5-A-04-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-04 — Tenant & Institutional Isolation Architecture  
**Canonical Technical Responsibility:** Technical Tenant/Institution isolation while preserving the locked business model  
**Technical Architecture Owner:** L5-A-04 technical responsibility  
**Assigned Author:** Training Example  
**Version:** 0.0.1-example  
**Status:** Training Example — Non-canonical  
**Depends On:** L5 Governance and Architecture Standard; L5 Specification Template; L5 Validation Standard; applicable L4 behavior; collaborating L5 architecture  
**Applicable ADRs:** Exact canonical isolation-enforcement pattern remains unresolved; pilot ADR candidate `ADR-C1` exists but no canonical ADR ID is allocated by that candidate record  

### Why this section exists

Before architecture is written, the document must establish exactly what technical responsibility is being described. The assigned author is not the canonical owner. The specification ID, family and responsibility come from the frozen L5 corpus, not from the author.

---

# 2. Purpose, Scope & Non-Goals

## Purpose

Define the technical architecture required to prevent Tenant-scoped execution from crossing an unauthorized Tenant boundary and to keep Institution context valid only within the resolved Tenant context.

The architecture must support downstream application, authorization, persistence, integration and client components without transferring their technical ownership into L5-A-04.

## In Scope

This example treats the following as L5-A-04 responsibilities:

- explicit Tenant/Institution execution-scope representation;
- technical context resolution and validation boundaries;
- Tenant/Institution consistency checks;
- isolation enforcement expectations at cross-component boundaries;
- failure behavior when required scope cannot be resolved or is inconsistent;
- propagation requirements that collaborating L5 specifications must consume without redefining Tenant business semantics.

## Out of Scope / Non-Goals

This example does not own:

- Tenant business lifecycle or business rules;
- Workspace, Role, Permission or Authorization Decision business meaning;
- IAM account or authentication architecture;
- authorization policy evaluation internals;
- physical database technology or schema design;
- UI navigation, screen layout or user journey design;
- exact source code, framework, database, broker, cloud provider or orchestration platform selection.

### Why this section exists

A strong L5 document says both **what it owns** and **what it must not own**. This prevents duplicate architecture. For example, Tenant isolation may require authorization and persistence, but that does not allow A04 to redesign D02 Authorization or B02 Persistence.

---

# 3. Upstream Applicability, Constraints & Assumptions

## 3.1 Illustrative L4 Applicability

This training example uses an illustrative subset of `L4-A-050 — Workspace Definition to Governed Resolvable Experience Context` only to demonstrate traceability. That L4 specification is Draft, so it may support controlled planning but cannot by itself make a real L5 specification Architecture Approved.

| L4 Specification | Applicable BH(s) | Technical Obligation for this Example | L5-A-04 Role |
|---|---|---|---|
| `L4-A-050` | `BH-001` | Tenant, Institution and applicable configuration context must be resolvable before governed experience context is established. | Supports technical scope-context resolution boundary. |
| `L4-A-050` | `BH-003` | Tenant isolation and Institution configuration consistency must be validated. | Owns technical isolation/context-validation expectations. |
| `L4-A-050` | `BH-004` | A Workspace mapping must not become an authorization bypass. | Ensures isolation context does not substitute for authorization. |
| `L4-A-050` | `BH-006` | Applicable context is dynamically resolved from authoritative inputs. | Provides technical context-resolution/propagation support. |
| `L4-A-050` | `BH-008` | Context changes require revalidation without changing authorization merely because context changes. | Requires revalidation of Tenant/Institution context across context changes. |

This is only an illustrative subset. A real production `L5-A-04-001` must use the repository-level traceability methodology to identify all materially applicable L4 behavior.

## 3.2 Higher-Level Verification

The example preserves these established meanings:

- Tenant isolation is mandatory.
- An Institution exists within a Tenant context; Institution context must not be treated as a substitute for Tenant context.
- Authorization remains a separate responsibility from Tenant/Institution isolation.
- L5 does not create missing business authority, lifecycle states or canonical events.

## 3.3 Authoritative Constraints

- Cross-Tenant execution must not occur through accidental context reuse, caller-supplied scope claims or unscoped downstream access.
- Institution context must be validated against the resolved Tenant boundary.
- Technical isolation must remain compatible with the supported on-premises-primary and cloud-capable deployment direction.
- No Kubernetes, cloud-provider or database-product assumption is introduced here.

## 3.4 Assumptions

| ID | Assumption | Why Needed | Validation / Resolution |
|---|---|---|---|
| `AS-001` | Collaborating request/contract architecture can carry an explicit execution-scope context. | A04 requires scope to cross technical boundaries. | Reconcile with `L5-C-01` contract architecture. |
| `AS-002` | Persistence architecture can enforce access using validated Tenant context without A04 owning the physical persistence design. | Isolation must reach storage boundaries. | Reconcile with `L5-B-02`. |

### Why this section exists

L5 must begin from approved behavior and established constraints. It must not manufacture technical requirements with no upstream basis. Assumptions remain visible so they cannot quietly become repository facts.

---

# 4. Technical Ownership, Boundaries & Dependencies

## Owns

- Tenant/Institution execution-scope semantics at the technical boundary level.
- Required isolation control points.
- Context-resolution and context-consistency validation expectations.
- Fail-closed behavior for unresolved or inconsistent scope.
- Technical propagation obligation for validated scope across collaborating L5 boundaries.

## Consumes / Depends On

| L5 Specification / Area | Capability Consumed | Boundary |
|---|---|---|
| `L5-A-01-001` | Overall application/platform responsibility boundaries | A04 operates within the platform decomposition rather than redefining it. |
| `L5-A-03-001/002` | Module/dependency boundary rules | Isolation context must cross only governed technical dependencies. |
| `L5-B-02` | Persistence and transaction architecture | B02 owns physical persistence mechanics; A04 supplies required isolation context and invariants. |
| `L5-C-01` | Synchronous contract architecture | C01 owns request/response contract mechanics; A04 defines the required scope-context semantics that must be carried. |
| `L5-D-01` | Authentication/session identity context | D01 establishes authenticated identity/session mechanics; A04 does not authenticate users. |
| `L5-D-02` | Authorization decision/enforcement | D02 decides/enforces authority; A04 ensures the resource/execution context is correctly Tenant/Institution scoped. |
| `L5-D-04` | Audit/evidence capability | D04 owns audit architecture; A04 supplies relevant isolation decision/evidence context when required. |

## Must Not Own

- IAM credential validation;
- authorization policy evaluation;
- persistence technology selection;
- canonical business-event publication;
- Workspace/Role/Permission business ownership;
- Institution or Tenant lifecycle decisions.

### Why this section exists

Dependencies are not permission to duplicate architecture. A04 may require D02 authorization and B02 persistence, but it must state the contract between responsibilities instead of copying their internal architecture.

---

# 5. Solution Architecture

## 5.1 Technical Realization Model

| Element | Type | Responsibility | State / Data | Depends On | Failure Boundary |
|---|---|---|---|---|---|
| Execution Entry Boundary | Control Point | Receive an operation together with available technical identity, Tenant and Institution context inputs. | Request/execution metadata only. | C01 / A01 | Does not allow protected execution to continue if required scope is absent. |
| Scope Context Resolver | Component / Control Point | Resolve canonical technical Tenant scope and applicable Institution scope from authoritative context sources. | Resolved scope references and resolution evidence. | A01 / D01 / collaborating authoritative source | Resolution failure produces no valid execution scope. |
| Scope Consistency Validator | Control Point | Verify that Institution context is valid within the resolved Tenant and that context does not conflict across inputs. | Validation result and reason. | A04 canonical rules | Mismatch causes fail-closed rejection. |
| Validated Execution Scope Context | Technical Context | Carry validated Tenant and applicable Institution references through the execution path. | Tenant reference, optional Institution reference, correlation/evidence references. | A04 | Unvalidated context must never be promoted to this form. |
| Isolation Boundary Guard | Control Point | Require validated scope before a protected component crosses an isolation-sensitive boundary. | Validation decision; no business state. | A03 / A04 | Reject before protected state mutation or cross-Tenant handoff. |
| Scoped Persistence Handoff | Governed Contract | Supply validated scope to B02-owned persistence mechanisms. | Scope context plus owner-specific operation intent. | B02 | A04 does not perform unscoped direct persistence access. |
| Audit Evidence Handoff | Governed Handoff | Supply isolation decision/context evidence to D04 when evidence is required. | Actor/context/correlation/failure evidence. | D04 | Audit failure treatment is owned by D04; A04 does not redefine audit storage. |

## 5.2 Primary Technical Flow — Real-World Example

### Example situation

A user operating inside **Tenant T1** selects a governed Workspace for **Institution I1**. The application receives the request and must ensure that the technical execution remains inside T1/I1 scope before any protected processing occurs.

```text
User / Client Request
        ↓
C01-governed request boundary
        ↓
Execution Entry Boundary
        ↓
Resolve Tenant T1
        ↓
Resolve / validate Institution I1 within T1
        ↓
Create Validated Execution Scope Context
        ↓
Isolation Boundary Guard
        ↓
D02 authorization consumes the validated resource/scope context
        ↓
Owning application component executes
        ↓
B02 persistence consumes validated Tenant/Institution scope
        ↓
Result returned through governed contract
```

Important ownership rule:

```text
A04 validates and carries execution scope.
D02 still owns authorization decision/enforcement.
B02 still owns persistence realization.
The business domain still owns the business fact being changed.
```

## 5.3 Failure Scenario — Cross-Tenant or Inconsistent Scope

### Example situation

An authenticated session associated with valid context for **Tenant T1** submits or receives a request containing an Institution reference that resolves under **Tenant T2**, or the required Tenant scope cannot be authoritatively resolved.

```text
Request arrives
     ↓
Scope Context Resolver
     ↓
Tenant / Institution mismatch detected
     ↓
NO validated execution context created
     ↓
Isolation Boundary Guard rejects execution
     ↓
NO protected domain mutation
     ↓
NO canonical business event is invented or published by A04
     ↓
Relevant technical evidence handed to D04 as applicable
```

The architecture is intentionally fail-closed because continuing with ambiguous or conflicting scope would create a cross-Tenant isolation risk.

## 5.4 Context Contract — Conceptual Architecture View

This is a conceptual architecture contract, not an API wire schema or software class:

```text
ValidatedExecutionScopeContext

- TenantReference          REQUIRED
- InstitutionReference     OPTIONAL when operation is Tenant-only
- ContextValidationStatus  VALIDATED
- CorrelationReference     REQUIRED where cross-boundary correlation applies
- ResolutionEvidenceRef    OPTIONAL / as required by audit architecture
```

Rules:

- a caller-provided Tenant/Institution claim is input, not automatically trusted context;
- only the validated context may cross protected technical boundaries;
- Institution scope must never be treated as globally unique outside its resolved Tenant boundary unless separately guaranteed by canonical architecture;
- collaborating components consume the validated context; they do not reconstruct Tenant scope from unrelated local data where doing so could create inconsistent isolation semantics.

## 5.5 Persistence / Consistency Interaction

A04 does not select the database or define physical schemas.

A04 requires that B02-owned persistence architecture provide an enforceable way for Tenant-scoped operations to remain constrained by validated execution scope.

A protected persistence operation must therefore be capable of demonstrating:

```text
Validated Tenant Context
        +
Owned Persistence Operation
        ↓
B02 Isolation / Ownership Enforcement
        ↓
Only data inside authorized technical scope is addressable
```

The exact physical isolation strategy is a material architecture decision and is not invented in this example.

## 5.6 Security / Trust Interaction

- Context supplied by a client or external caller is not accepted as authoritative solely because it was present in a request.
- Authentication and session validation remain D01-owned.
- Permission/policy evaluation remains D02-owned.
- A04 ensures those controls receive internally validated Tenant/Institution resource context.
- Cross-service propagation must preserve context integrity and correlation; exact transport representation belongs to the collaborating contract/runtime architecture.

## 5.7 Runtime / Deployment Implication

Every runtime boundary that participates in Tenant-scoped execution must be able to receive and preserve the validated execution-scope context required by A04.

This requirement is product-independent. It does not imply Kubernetes, a specific service mesh, a cloud provider or separate physical servers.

### Why this section exists

This is the core of L5. A production document cannot stop at statements such as “the system shall isolate Tenant data.” It must identify the technical elements, their responsibilities, boundaries, interactions, primary flow, failure flow and relevant persistence/security/runtime implications with enough precision that implementation does not have to invent major architecture.

---

# 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Rationale | Constraint / Traceability | ADR |
|---|---|---|---|---|---|
| `AD-001` | Execution scope representation | Tenant/Institution scope shall be explicit in a validated technical execution context. | Hidden/implicit scope increases inconsistent enforcement and cross-Tenant leakage risk. | L4-A-050 BH-001/BH-003/BH-006; A04 responsibility. | Candidate ADR-C1 may affect exact realization. |
| `AD-002` | Invalid or inconsistent scope | Isolation validation shall fail closed before protected processing. | Continuing with ambiguous scope violates the isolation objective. | L4-A-050 BH-003/BH-008. | None for principle; exact mechanism may fall under ADR-C1. |
| `AD-003` | Authorization separation | Tenant/Institution isolation context shall support but never replace D02 authorization decisions. | Workspace/context availability is not an access grant. | L4-A-050 BH-004/BH-008. | None. |
| `AD-004` | Persistence boundary | A04 defines isolation context/invariants; B02 owns physical persistence and consistency realization. | Preserves single technical ownership and prevents duplicate database architecture. | L5 corpus ownership boundaries. | ADR-C1 / persistence ADR candidate may affect implementation pattern. |

### Why this section exists

`AD-*` records make local architecture choices explicit. They are not canonical repository ADR numbers. A material choice that needs broader governance must go through the ADR methodology rather than being hidden in prose.

---

# 7. Security, Privacy, Quality & Operational Impact

| Attribute | Architectural Requirement / Position | Rationale |
|---|---|---|
| Security | Protected Tenant-scoped processing requires validated scope before crossing the isolation boundary. | Prevent cross-Tenant access caused by untrusted or inconsistent context. |
| Authorization | A04 provides validated context; D02 remains decision/enforcement owner. | Prevents isolation logic from becoming a second authorization system. |
| Persistence | Scoped context must reach B02-controlled persistence enforcement. | Isolation cannot end at the application boundary. |
| Auditability | Scope resolution/rejection evidence must be available to D04 where required. | Supports investigation without making logs canonical business truth. |
| Portability | Isolation requirements must work across supported on-premises and cloud-capable deployments. | Deployment model must not weaken architecture. |
| Testability | Valid, missing, conflicting and cross-Tenant scope cases must be objectively verifiable. | Isolation is a high-integrity technical boundary. |

No unsupported numeric latency, availability or capacity target is invented in this example.

---

# 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From |
|---|---|---|
| `CR-001` | Every protected Tenant-scoped execution path shall possess validated Tenant context before isolation-sensitive processing occurs. | AD-001 / Section 5. |
| `CR-002` | When Institution context is present, its consistency with the resolved Tenant shall be validated before protected processing. | AD-001 / AD-002. |
| `CR-003` | Missing, unresolved or conflicting required scope shall prevent protected domain mutation. | AD-002 / failure flow. |
| `CR-004` | Tenant/Institution context shall not itself grant permission or replace D02 authorization. | AD-003. |
| `CR-005` | A04-consuming persistence operations shall provide validated isolation context to the B02-owned persistence boundary rather than performing ungoverned unscoped access. | AD-004. |
| `CR-006` | Cross-boundary propagation of validated scope shall preserve enough context integrity and correlation for the receiving component to apply the same isolation semantics. | Section 5.4 / 5.6. |
| `CR-007` | A rejected scope validation shall not cause A04 to invent or publish a canonical business event. | Upstream event-ownership rule. |

### Why this section exists

These are objective technical conditions that downstream implementation and verification can check. They are **not new business behaviors** and do not replace L4 BHs.

---

# 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| `OD-001` | Exact canonical Tenant/Institution isolation-enforcement realization across application/runtime/persistence boundaries. | ADR | Lead Architect / canonical L5 decision owner | Yes before final Architecture Approved status if ADR remains required | Complete formal ADR necessity/scope review for pilot candidate `ADR-C1`; allocate canonical ADR only through the ADR Register. |
| `OD-002` | Exact wire-level representation of validated execution-scope context across synchronous contracts. | Cross-L5 / implementation detail boundary | L5-C-01 with A04 constraint | Potentially | C01 shall define contract realization without changing A04 scope semantics. |

### Why this section exists

If architecture is unresolved, the author must say so. A Draft may proceed with visible unresolved decisions; it must not silently choose a technology or architecture pattern just to make the document look complete.

---

# 10. Traceability Summary

### Illustrative Primary L4 Applicability

- `L4-A-050` — `BH-001`, `BH-003`, `BH-004`, `BH-006`, `BH-008`.

### Material Higher-Level Constraints

- Tenant isolation remains mandatory.
- Authorization & Policy retains authorization semantics; technical context does not grant access by itself.
- No missing business lifecycle, authority or canonical event is invented at L5.

### Owned L5 Responsibility

- Tenant/Institution isolation control points, validated scope context and cross-boundary isolation invariants.

### Material L5 Dependencies

- `L5-A-01-001`
- `L5-A-03-001/002`
- `L5-B-02`
- `L5-C-01`
- `L5-D-01`
- `L5-D-02`
- `L5-D-04`

### Architecture Decisions

- `AD-001` — explicit validated execution scope.
- `AD-002` — fail closed on invalid/inconsistent scope.
- `AD-003` — isolation context does not replace authorization.
- `AD-004` — A04/B02 persistence-ownership boundary.

### Conformance Requirements

- `CR-001` through `CR-007`.

### Open Decisions

- `OD-001` — canonical isolation-enforcement ADR decision.
- `OD-002` — C01 contract representation boundary.

---

# What Makes This an L5 Document Rather Than an L4 Document?

L4 would state observable behavior such as:

```text
Validate Tenant isolation and Institution configuration consistency.
Reject an invalid context or authorization bypass.
```

L5 goes further and answers:

```text
Where is scope resolved?
What technical context is created?
Where is it validated?
Which control point rejects failure?
What is allowed to execute after validation?
How is the context passed to authorization?
How does persistence consume the context?
What happens on mismatch?
Which responsibility belongs to A04 versus B02/D02/C01?
What decisions are explicit?
What remains unresolved?
What can implementation objectively verify?
```

That difference is the expected L5 technical depth.

---

# What Would Be Too Weak for L5?

The following is insufficient by itself:

```text
The system shall ensure secure Tenant isolation.
The solution shall be scalable and robust.
Tenant information shall be validated.
Industry best practices shall be followed.
A suitable database and security technology shall be used.
```

Those statements do not establish enough architecture for implementation.

A mass-authored L5 document must instead make responsibilities, technical elements, control points, flows, failure behavior, dependencies, architecture decisions, conformance requirements and unresolved decisions explicit where materially applicable.

---

# Production Author Checklist Derived from This Example

Before submitting a real L5 Draft, confirm:

1. The exact ID/title comes from the frozen 105 register.
2. The author assignment is correct.
3. Required dependencies are stable enough to consume.
4. Applicable L4 behavior is identified without rewriting or changing it.
5. `Owns` and `Must Not Own` are explicit.
6. The document contains real technical architecture, not only principles.
7. A primary execution flow is defined where runtime behavior exists.
8. Material failure behavior is defined.
9. Persistence/security/async/runtime/operations implications are addressed where applicable.
10. Local `AD-*` decisions are visible.
11. Material ADR needs are raised rather than silently decided.
12. Objective `CR-*` conformance requirements are present.
13. Unresolved `OD-*` items are explicit.
14. The traceability summary is complete.
15. The document can guide implementation without requiring the implementation team to invent major architecture.

---

**Training Example Only — Not Canonical Production Architecture**
