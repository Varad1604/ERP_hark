# Tenant and Institution Context Propagation & Data Isolation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-04-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-04 — Tenant & Institutional Isolation Architecture  
**Canonical Technical Responsibility:** Propagation/enforcement of Tenant, Institution and lower-scope context across execution and persistence.  
**Technical Architecture Owner:** L5-A-04 — Tenant & Institutional Isolation Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.1.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Pilot ADR Candidate Disposition v1.0.0; `L5-A-04-001` v1.1.0 Architecture Approved; `L5-A-01-001` v1.1.0 Architecture Approved; `L4-A-001` v1.0.1 Architecture Approved; `L4-A-007` v1.0.1 Architecture Approved; `L4-A-015` v1.0.1 Architecture Approved; `L4-A-019` v1.0.1 Architecture Approved; `L4-A-050` v1.0.1 Architecture Approved; `L4-B-001` v1.0.1 Architecture Approved; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `L2-B01_Institution_Workflow.md`; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** Peer L5 execution, contract, persistence, security and runtime architectures carrying protected execution scope.  
**Applicable ADRs:** `ADR-L5-0004` — Tenant and Institution Isolation Enforcement Pattern (`Architecture Approved`).

---

## 2. Purpose, Scope & Non-Goals

This specification defines how validated Tenant, Institution and applicable lower-scope context is carried and enforced across execution and persistence boundaries. It realizes the isolation semantics established by `L5-A-04-001` and `ADR-L5-0004` without redefining business ownership or peer technical internals.

### In Scope

- Validated Execution Scope Context.
- Scope propagation across synchronous, asynchronous, workflow, background/deferred and persistence boundaries.
- Scope narrowing and validation rules.
- Fail-closed handling for missing, switched, widened or incompatible scope.
- Data-isolation requirements consumed by B02 and other stateful peers.

### Out of Scope

- Tenant/Institution lifecycle.
- Authentication and authorization decision ownership.
- API/event payload schema design.
- Persistence topology/database product.
- Workflow internals.
- Runtime/deployment topology.

---

## 3. Authoritative Basis & Constraints

| Source | L5-A-04-002 Effect |
|---|---|
| `L5-A-04-001` | Defines Tenant-rooted isolation semantics and admission. |
| `ADR-L5-0004` | Canonically approves the Tenant/Institution isolation and propagation pattern. |
| applicable L4 A/B behavior | Requires Tenant/Institution context to remain correct across business/execution handoffs. |
| L1 v2.1 + applicable amendment | Preserves canonical ownership and Tenant-scoped enterprise boundaries. |
| pilot `ADR-C1` | Historical candidate provenance. |

### Constraints

1. Tenant scope is mandatory for protected Tenant-scoped execution.
2. Institution/lower scopes shall never widen the Tenant root.
3. Scope shall not be inferred from mutable global state or an implicit default Tenant.
4. Every material execution/persistence handoff must preserve sufficient validated scope to prevent cross-Tenant ambiguity.
5. Scope may be narrowed only when the target operation permits the narrower context; it shall not be widened without authoritative revalidation.
6. A04-002 defines architecture semantics, not concrete DTO/class/header/table schemas.

---

## 4. Technical Ownership & Boundaries

### Owns

- **Validated Execution Scope Context** semantics.
- Propagation and revalidation requirements.
- Fail-closed rules for scope loss, mismatch, switch or widening.
- Technical data-isolation context requirements handed to B02.

### Consumes / Collaborates With

- C01 synchronous contracts.
- C02 messaging/events.
- A05/A08 workflow/background execution where applicable.
- B02 persistence.
- D01 identity, D02 authorization, D03 trust, D04 evidence.
- E04 runtime/deployment.

### Must Not Own

Business lifecycle/rules, authentication, authorization decisions, contract schemas, event semantics, database ownership, workflow semantics, audit ownership or deployment topology.

---

## 5. Solution Architecture

### 5.1 Validated Execution Scope Context

The architecture context contains the minimum validated scope required for protected execution:

```text
Tenant scope        — mandatory
Institution scope   — conditional, always subordinate to Tenant
Lower scope         — conditional, owned/defined by applicable architecture
Validation provenance / authoritative basis — sufficient for enforcement/evidence
```

This is an architecture contract. It does not prescribe a programming class, API header, message envelope or database column.

### 5.2 Propagation Invariant

Validated scope shall survive every material technical handoff:

```text
request / job / workflow / message
        ↓
validate Tenant root and applicable subordinate scope
        ↓
execute / hand off
        ↓
receiving boundary preserves and revalidates scope as required
        ↓
persistence / downstream execution under same or validly narrowed scope
```

### 5.3 Synchronous Boundaries

C01 implementations shall carry sufficient validated scope to the receiving execution boundary. The target shall not silently substitute server-local/default Tenant context.

### 5.4 Asynchronous / Event Boundaries

C02 implementations shall preserve sufficient scope correlation for protected consumption without changing canonical event meaning or publisher ownership. Consumer execution revalidates the applicable scope before mutable action.

### 5.5 Workflow / Background / Deferred Execution

Long-lived, scheduled or deferred execution shall capture/reference authoritative scope deliberately. Resumption shall not depend on whichever user/session/Tenant happens to be active later.

### 5.6 Persistence Boundary

B02 shall receive the validated scope needed to restrict authoritative persistence access. Shared physical storage does not permit scope widening. Owner and Tenant isolation are both enforced according to their canonical owners.

### 5.7 Scope Change Rules

- **Narrowing:** permitted only when consistent with the operation and authoritative hierarchy.
- **Switching Tenant:** requires a new validated execution context; cannot occur implicitly mid-operation.
- **Widening:** prohibited unless an authoritative boundary explicitly establishes and validates broader scope.
- **Missing scope:** fail closed for protected execution.

### 5.8 Failure Model

| Failure | Required Outcome |
|---|---|
| Tenant context absent | Fail closed. |
| Institution does not belong to Tenant | Fail closed. |
| Context dropped across async/background handoff | Do not execute protected mutable action. |
| Context switches or widens unexpectedly | Reject and surface evidence. |
| Persistence receives ambiguous scope | Reject protected persistence operation. |
| Peer cannot preserve required scope | Stop at peer boundary; no fallback/global Tenant. |

---

## 6. Architecture Decisions

| Decision ID | Decision | Status | ADR |
|---|---|---|---|
| `AD-001` | Protected execution uses a validated Tenant-rooted Execution Scope Context. | Architecture Approved | `ADR-L5-0004` |
| `AD-002` | Scope is propagated across synchronous, asynchronous, workflow, background/deferred and persistence boundaries and revalidated where required. | Architecture Approved | `ADR-L5-0004` |
| `AD-003` | Missing, switched, widened or incompatible protected scope fails closed; no default/global Tenant fallback is allowed. | Architecture Approved | `ADR-L5-0004` |

---

## 7. Security, Privacy & Operational Impact

- Cross-Tenant leakage/execution becomes a fail-closed condition.
- Background and integration paths are held to the same isolation semantics as interactive execution.
- Peer architectures remain free to choose product-specific context carriers later, provided semantics are preserved.
- Evidence should distinguish scope-validation failures without exposing unnecessary sensitive data.

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Protected execution shall carry one validated Tenant root. |
| `CR-002` | Institution/lower scope shall be compatible with and subordinate to the Tenant root. |
| `CR-003` | Synchronous handoffs shall preserve required validated scope. |
| `CR-004` | Asynchronous/event consumption shall recover and revalidate applicable protected scope before mutable execution. |
| `CR-005` | Workflow/background/deferred execution shall not depend on ambient user/Tenant state. |
| `CR-006` | Persistence shall receive sufficient validated scope to enforce B02 isolation. |
| `CR-007` | Missing, ambiguous, switched, widened or incompatible scope shall fail closed. |
| `CR-008` | No default/global Tenant fallback shall authorize protected mutable execution. |
| `CR-009` | Scope carrier representation shall remain peer-owned and must preserve A04 semantics. |
| `CR-010` | A04-002 shall not create business lifecycle transitions, business authorization decisions, persistence ownership or business events. |

---

## 9. Open Decisions & Deferred Design

`ADR-L5-0004` resolves the former blocking isolation-pattern ADR item.

| ID | Item | Blocking Status | Resolution Path |
|---|---|---|---|
| `OD-002` | Exact context-carrier representations and enforcement mechanics in C01, C02, A05, A08, B02, D01/D02/D03. | Not blocking for this product-neutral Draft; required for final peer integration/implementation standardization. | Respective canonical L5 owners reconcile to A04 semantics. |

---

## 10. Traceability Summary

Canonical decision: `AD-001`–`AD-003` → `ADR-L5-0004`.

Historical candidate provenance: `ADR-C1`.

Primary peers: C01/C02, A05/A08, B02, D01/D02/D03/D04 and E04.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
