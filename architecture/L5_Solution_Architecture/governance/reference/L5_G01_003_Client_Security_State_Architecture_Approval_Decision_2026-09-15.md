# SutraERP L5 — G01-003 Client Security, Session, State & Contract Consumption Architecture Approval Decision

**Decision Date:** 2026-09-15  
**Architecture Level:** L5 — Solution Architecture Governance  
**Decision Type:** Independent architecture validation and approval  
**Status:** Architecture Approved  
**Scope:** `L5-G-01-003 — Client Security, Session, State & Contract Consumption Architecture`

---

## 1. Decision

`L5-G-01-003` is approved as the canonical client security/session/state/safe-contract-consumption architecture for SutraERP G01.

The specification is architecture-valid and ready for downstream consumption by `L5-G-01-004` and future L6/implementation work.

This approval does not lock the specification and does not alter the repository status of C01, D01, A04 or D03 peer specifications.

---

## 2. Governing Basis

The review applies:

- L5 Governance and Architecture Standard;
- L5 Solution Architecture Specification Template v1.0.3;
- L5 Validation Standard;
- frozen L5 Candidate Document Corpus Register 105;
- L5 Technology Selection & ADR Methodology;
- G01 upstream dependency-readiness and approval decision dated 2026-09-15;
- `L5-G-01-001` Architecture Approved;
- `L5-G-01-002` Architecture Approved;
- `ADR-L5-0001` — Canonical Trust Boundary & Segmentation Pattern;
- `ADR-L5-0003` — Contract-Driven Modular Platform with Selective Isolation;
- `ADR-L5-0004` — Tenant and Institution Isolation Enforcement Pattern;
- `ADR-L5-0015` — Web Client Platform, Rendering & Deployment Pattern;
- stable consumed boundaries of C01, D01, D02 and A04.

---

## 3. Canonical Ownership Result

G01-003 correctly owns only client-local technical controls:

- session/authentication projection handling;
- protected client-state partitioning;
- context invalidation and stale-response containment;
- server-state projection caching;
- safe contract-consumption coordination;
- client mutation-attempt/indeterminate-state handling;
- browser persistence admission;
- client reconciliation after session/scope/contract changes.

It does not transfer canonical ownership from:

- D01 — Authentication Session, Credential, continuation and authentication mechanics;
- D02 — Authorization Decision and enforcement;
- A04 — Tenant/Institution validation and isolation authority;
- C01 — contract versioning, compatibility, technical error and idempotency semantics;
- D03 — trust/security/secret-protection architecture;
- business domains — canonical business state and recovery.

---

## 4. Security Architecture Decision

The following G01-003 client-security rule is approved:

```text
Script-readable durable browser storage
        !=
Authentication/session/continuation secret store
```

The web client shall not persist authentication/session secrets, bearer-equivalent credentials, refresh/continuation secrets, Credential/MFA secret material, private keys or machine credentials in `localStorage`, `sessionStorage`, IndexedDB or equivalent script-readable durable browser storage.

This rule does not select an authentication protocol or token format. D01 remains the canonical owner of the session-carrier mechanism.

A D01-approved browser mechanism that is not exposed to application JavaScript may be consumed by G01 without changing ownership.

---

## 5. State Architecture Result

The following client-state architecture is accepted:

```text
Client Context Epoch
        +
Protected State Partition
        +
Non-authoritative Server-State Projection Cache
        +
Safe Contract Client
        +
Mutation Coordinator
```

### Client Context Epoch

A local technical generation marker changes when material protected context changes, including session/principal, Tenant/Institution, logout/invalidation or incompatible client/contract state.

Late asynchronous results from an older epoch may not populate the new active protected state partition.

This is a client-local race/isolation control and is not a business lifecycle state.

### Protected State Partition

Protected client state is partitioned by applicable Tenant/Institution and authenticated context. A context change makes the prior partition ineligible for active reuse until separately revalidated.

### Server-State Projection Cache

Protected business data in the client remains a non-authoritative projection. For the immediate web architecture it is memory-scoped by default; durable protected caching is not approved by this decision.

---

## 6. Contract and Mutation Safety Result

G01-003 correctly consumes rather than redefines C01 semantics.

The approved client mutation outcome model is:

```text
Authoritative Success
Authoritative Rejection
Technical / Transport Ambiguity -> Indeterminate
```

`Indeterminate` is client technical state only. It prevents the client from falsely declaring success or blindly repeating a potentially completed business mutation.

Mutation retry is not automatically safe. Repeat execution is allowed only where C01 contract semantics provide the required retry/idempotency safety, including preservation of logical-operation identity where applicable.

No retry count, timeout, backoff or idempotency retention period is invented.

---

## 7. State-Management Technology Position

No additional mandatory state/cache framework is required for G01-003 approval.

The approved G01-002 React + React Router platform may implement the G01-003 architecture using its existing route data/action/revalidation and React state mechanisms behind the canonical G01 control plane.

Redux, Zustand, TanStack Query or another state/cache product may not become a repository mandate merely through local implementation adoption.

A future shared mandatory client state/cache technology shall undergo applicable technology/ADR review where the L5 methodology requires it.

---

## 8. Offline Position

The immediate client architecture remains online-first.

No durable offline mutation queue, offline canonical business state or disconnected business-success authority is approved.

A future offline-first architecture would materially affect consistency, reconciliation, security, Tenant isolation and contract semantics and therefore requires architecture review and likely ADR governance.

---

## 9. Hard-Fail Review

| Hard Fail | Result | Reason |
|---|---|---|
| HF-01 Invalid Behavioral Authority | PASS | G01-003 consumes approved G01 parents and governance-authorized peer boundaries; no unsupported direct L4 authority is claimed. |
| HF-02 Orphan Architecture | PASS | `L5-G-01-003` exists in the frozen 105-specification corpus. |
| HF-03 Upstream Business Semantics Corruption | PASS | No business rule, lifecycle, event or recovery semantics are created. |
| HF-04 Canonical Ownership Transfer | PASS | D01/D02/A04/C01/D03/business ownership remains intact. |
| HF-05 Silent Upstream Repair | PASS | Unresolved D01 protocol/token/session-carrier decisions remain explicit. |
| HF-06 Duplicate Technical Ownership | PASS | Client-local state/consumption controls do not duplicate server-side peer ownership. |
| HF-07 Cross-Boundary Architecture Bypass | PASS | Safe Contract Client explicitly prohibits private protected-endpoint bypasses. |
| HF-08 Claimed L4 Realization Failure | PASS | No unsupported L4 realization claim is used as the approval basis. |
| HF-09 Ungoverned Material Decision | PASS | No new major framework/product/protocol is selected. Existing React/React Router selection is governed by ADR-L5-0015. |
| HF-10 Unsupported Constraint / Hidden Assumption | PASS | Peer stability, authentication carrier and cache-product positions are explicit. |
| HF-11 Material Architecture Unimplementability | PASS | State partitions, context epochs, mutation outcomes, persistence gate and safe contract boundary are implementable with the approved platform. |
| HF-12 Security / Privacy / Trust Violation | PASS | Browser-secret storage is fail-safe; client authority remains non-canonical; scope isolation is explicit. |
| HF-13 L6 Boundary Violation | PASS | No screen/navigation/interaction design is prescribed. |
| HF-14 Blocking Unresolved Architecture | PASS | D01 protocol/token choice is implementation dependency, not a blocker to G01-003 product-neutral architecture. |
| HF-15 Invalid Identity / Metadata | PASS | Canonical ID, family, title and production path are valid. |

**Blocking hard failures:** `0`.

---

## 10. Approval Result

```text
L5-G-01-003
    = canonical responsibility valid
    = client security/state boundary complete
    = D01 session authority preserved
    = D02 authorization authority preserved
    = A04 isolation authority preserved
    = C01 contract/idempotency ownership preserved
    = no new technology ADR required
    = 0 blocking hard failures
    = Architecture Approved
    = ready for L5-G-01-004 and downstream L6/implementation consumption
    != Locked
```

---

## 11. Metadata Synchronization Rule

The approval established by this decision is authoritative over the initial authoring-time `Draft` metadata in `L5-G-01-003`.

When the specification file is next normalized or materially edited:

1. set `Status: Architecture Approved`;
2. record this approval decision in its dependency/governance references;
3. retain the open D01 authentication protocol/session-carrier decision as non-blocking for G01-003 architecture;
4. do not introduce a mandatory state/cache product without governed review.

Until that mechanical synchronization occurs, this decision governs current architectural status and downstream readiness.

---

## 12. Lock Boundary

Architecture Approved does not equal Locked.

Final lock requires a distinct explicit final-governance authorization and impact review if G01-001, G01-002, C01, D01, D02, A04, D03 or applicable ADRs materially change before lock.
