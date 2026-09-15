# SutraERP L5 — G01-002 Web Application Architecture Approval Decision

**Decision Date:** 2026-09-15  
**Architecture Level:** L5 — Solution Architecture Governance  
**Decision Type:** Architecture validation and approval  
**Status:** Architecture Approved  
**Scope:** `L5-G-01-002 — Web Application Architecture`; `ADR-L5-0015`

---

## 1. Decision

`L5-G-01-002 — Web Application Architecture` and its governing `ADR-L5-0015 — Web Client Platform, Rendering & Deployment Pattern` are approved for downstream use.

This decision does not lock either artifact.

---

## 2. Validation Basis

Validation applied the locked L5 governance, specification template, validation standard, technology-selection/ADR methodology, approved G01-001 parent baseline, dependency-readiness decision, approved ADR-L5-0003/0004, and the current C01/D01/D02/A04 boundaries authorized for G01 consumption.

External framework documentation was used only as technology-feasibility evidence and not as repository authority.

---

## 3. Hard-Fail Review

| Hard Fail | Result | Reason |
|---|---|---|
| HF-01 Invalid Behavioral Authority | PASS | G01-002 specializes approved G01-001 and ADR authority; it does not claim unsupported direct L4 behavior. |
| HF-02 Orphan Architecture | PASS | G01-002 exists in the frozen 105-spec corpus. |
| HF-03 Upstream Business Semantics Corruption | PASS | No business rule, state, lifecycle, event or authority is introduced. |
| HF-04 Canonical Ownership Transfer | PASS | Business truth, scope, authentication and authorization remain with canonical server/peer owners. |
| HF-05 Silent Upstream Repair | PASS | No upstream gap is repaired inside web architecture. |
| HF-06 Duplicate Technical Ownership | PASS | G01-002 owns web runtime/platform structure; G01-003 retains session/state/security/safe-consumption mechanics. |
| HF-07 Cross-Boundary Architecture Bypass | PASS | Web features use G01/C01 contracts; no direct persistence/backend-internal access. |
| HF-08 Claimed L4 Realization Failure | PASS | No unsupported direct L4 realization claim. |
| HF-09 Ungoverned Material Decision | PASS | React/TypeScript/React Router/rendering/deployment pattern is governed by ADR-L5-0015. |
| HF-10 Unsupported Constraint / Hidden Assumption | PASS | Online-first, SPA-first and single-app topology are explicit governed decisions. |
| HF-11 Material Architecture Unimplementability | PASS | Runtime structure, bootstrap, module, hosting, failure and boundary architecture are implementable. |
| HF-12 Security / Privacy / Trust Violation | PASS | Browser is non-authoritative; secrets excluded; server authorization and Tenant validation preserved. |
| HF-13 L6 Boundary Violation | PASS | Screens, visual design, journeys and navigation semantics remain excluded. |
| HF-14 Blocking Unresolved Architecture | PASS | No G01-002-owned blocker remains; G01-003 details are a governed downstream dependency, not missing ownership. |
| HF-15 Invalid Identity / Metadata | PASS | Canonical ID, title, family and production path are valid. |

---

## 4. Technical Feasibility Result

The selected architecture is technically feasible because:

- React Router Framework Mode supports SPA rendering with runtime SSR disabled and static output;
- TypeScript compiles to JavaScript and provides static type tooling suitable for the selected browser runtime;
- a static browser artifact preserves on-premises/cloud-neutral hosting options;
- the selected topology does not require a new authoritative server-side business boundary;
- optional later server rendering remains an explicit re-evaluation path rather than an immediate dependency.

No numeric performance, availability or capacity target is invented.

---

## 5. Approval Result

```text
L5-G-01-002
    = canonical responsibility valid
    = technical stack governed by ADR-L5-0015
    = G01-001 inheritance preserved
    = C01 / D01 / D02 / A04 ownership preserved
    = G01-003 ownership preserved
    = no L6 leakage
    = no unresolved G01-002-owned blocking finding
    = Architecture Approved
    = ready for implementation planning and downstream L6 consumption
    != Locked
```
