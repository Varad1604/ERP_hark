# SutraERP L5 — G01 Upstream Dependency Readiness & Architecture Approval Decision

**Decision Date:** 2026-09-15  
**Architecture Level:** L5 — Solution Architecture Governance  
**Decision Type:** Dependency remediation, upstream-boundary correction and architecture approval  
**Status:** Architecture Approved  
**Scope:** `L5-G-01-001` — Client Platform Architecture Baseline only

---

## 1. Decision

`L5-G-01-001 — Client Platform Architecture Baseline` is approved to advance from `Draft` to `Architecture Approved` after the dependency corrections and readiness determinations in this decision are applied.

This decision does not lock `L5-G-01-001` and does not change the repository status of any upstream L4 or peer L5 specification.

---

## 2. Governing Basis

This decision applies:

- the locked L5 Governance and Architecture Standard;
- the locked L5 Validation Standard;
- the frozen 105-specification corpus register;
- the dependency-gated execution timeline;
- the L5 L4-to-L5 Traceability Methodology;
- the L5 Technology Selection & ADR Methodology;
- the L5 ADR Register;
- `ADR-L5-0003` — Contract-Driven Modular Platform with Selective Isolation;
- `ADR-L5-0004` — Tenant and Institution Isolation Enforcement Pattern;
- current canonical peer architecture in `L5-C-01`, `L5-D-01`, `L5-D-02` and `L5-A-04`.

The dependency timeline requires `C01 + D01 + D02 + A04` to be sufficiently stable before G01 depends on them. It does not require every consumed peer specification to be Locked.

---

## 3. Upstream L4 Dependency Correction

### 3.1 `L4-A-050` is not a valid direct approval dependency for G01

`L3-A-050 — Workspace Definition to Governed Resolvable Experience Context` is `Locked` but explicitly classified `BLOCKED` because L2-A04 does not define the Workspace lifecycle, commands, canonical events/publishers or approval authority. Its child `L4-A-050` therefore remains unsuitable as an approval-level direct behavioral authority.

G01 does not canonically own Workspace business behavior. Workspace is an Authorization & Policy concept, and client-platform architecture needs only the peer-owned authorization/scope result, not Workspace lifecycle authority.

**Decision:** remove `L4-A-050` from direct G01 applicability and remove Workspace-specific behavior from the G01 baseline. No Workspace state, command, event, publisher or authority is invented.

### 3.2 `L4-A-007` is not required as a direct G01 authority

`L3-A-007` retains unresolved actor, User Account terminology, generic authentication-event publisher and Authentication Session lifecycle wording. Those gaps are correctly preserved upstream.

G01 does not own authentication behavior or Authentication Session lifecycle. It consumes the D01 technical authenticated-context boundary and the D02 authorization boundary.

**Decision:** remove direct `L4-A-007` BH dependency from G01. Authentication behavior remains owned by D01/upstream Authentication architecture; G01 consumes only the technical context exposed by the canonical peer.

### 3.3 Traceability consequence

G01 is a reusable shared client-platform architecture whose canonical technical purpose is established by the approved L5 corpus boundary and whose protected-execution constraints are supplied by peer L5 architecture and approved ADRs. It therefore requires no invented direct L4 BH mapping merely to satisfy a documentation pattern.

This decision does not alter complete repository-level BH-to-L5 traceability where later validated client-specific mappings are established.

---

## 4. Minimum Peer Dependency Readiness

The following dependency boundaries are declared **stable enough for `L5-G-01-001` consumption** under the dependency-gated execution rule.

| Dependency | Current Repository Status | G01-Consumed Boundary | Readiness Decision |
|---|---|---|---|
| `L5-C-01-001` | Draft | Governed contract boundary; released-contract consumption; ownership-preserving synchronous interaction | Stable for G01 consumption. Transport/style/framework/product choices remain peer-owned and are not required by G01. |
| `L5-D-01-001` | Draft | Authenticated technical context separated from authorization | Stable for G01 consumption. G01 does not depend on unresolved provider/protocol/credential/session representation choices. |
| `L5-D-02-001` | Architecture Approved | Authoritative authorization decision/enforcement boundary | Stable and approved. |
| `L5-A-04-001` / `L5-A-04-002` | Draft | Tenant-rooted isolation, fail-closed scope semantics and validated scope propagation | Stable for G01 consumption because the material isolation pattern is governed by Architecture Approved `ADR-L5-0004`; G01 does not own carrier representation or persistence enforcement. |

This scoped readiness determination does not promote C01, D01 or A04, does not close their unrelated open decisions, and does not authorize another specification to consume more than the boundary stated above without its own dependency review.

---

## 5. Technology Decision Position

G01 remains product-neutral.

No client framework, language, browser rendering pattern, state-management product, mobile framework, authentication protocol or API transport is selected by this decision.

A future shared client platform/framework selection remains subject to the L5 Technology Selection & ADR Methodology.

The absence of that selection does not block the product-independent G01 baseline.

---

## 6. Hard-Fail Revalidation

| Hard Fail | Result | Reason |
|---|---|---|
| HF-01 Invalid Behavioral Authority | PASS | Blocked/unapproved L4 material is no longer claimed as direct G01 approval authority. |
| HF-02 Orphan Architecture | PASS | G01 responsibility exists in the approved frozen 105-specification corpus. |
| HF-03 Business-Semantics Corruption | PASS | No business rule, lifecycle, event, publisher or authority is created. |
| HF-04 Canonical Ownership Transfer | PASS | Client remains non-authoritative; peer/domain ownership is preserved. |
| HF-05 Silent Upstream Repair | PASS | A050/A007 gaps remain upstream; G01 no longer attempts to depend on or repair them. |
| HF-06 Duplicate Technical Ownership | PASS | Contract, IAM, authorization and isolation responsibilities remain with C01, D01, D02 and A04. |
| HF-07 Cross-Boundary Bypass | PASS | Client contract consumption does not permit direct peer persistence/internal-state access. |
| HF-08 Claimed L4 Realization Failure | PASS | G01 makes no unsupported direct L4 realization claim. |
| HF-09 Ungoverned Material Decision | PASS | No shared client technology is selected. |
| HF-10 Hidden Material Assumption | PASS | Scoped peer-readiness determination and technology deferral are explicit. |
| HF-11 Material Architecture Unimplementability | PASS | Logical client host, context coordination, contract boundary, trust boundary and failure behavior are defined. |
| HF-12 Security / Privacy / Trust Violation | PASS | Tenant isolation and server-side authorization remain authoritative and fail closed. |
| HF-13 L6 Boundary Violation | PASS | UI/UX, navigation and journeys remain excluded. |
| HF-14 Blocking Unresolved Architecture | PASS | No G01-owned blocking decision remains. |
| HF-15 Canonical Identity / Metadata | PASS | Canonical ID, family, title and production placement remain valid. |

---

## 7. Required Specification Synchronization

The canonical `L5-G-01-001` file shall be synchronized as follows:

1. advance status to `Architecture Approved`;
2. remove `L4-A-007` and `L4-A-050` as direct applicability/approval dependencies;
3. remove Workspace-specific context-change behavior from G01;
4. retain C01, D01, D02 and A04 as the minimum peer dependency set;
5. record this governance decision as the scoped peer-readiness authority;
6. retain shared client technology selection as a future ADR-governed decision;
7. retain all existing non-authoritative-client, contract-boundary, Tenant/isolation, authorization and L6-separation conformance requirements.

---

## 8. Final Position

```text
L5-G-01-001
    = canonical identity valid
    = false A050 direct dependency removed
    = unnecessary A007 direct dependency removed
    = C01 boundary stable for G01 consumption
    = D01 boundary stable for G01 consumption
    = D02 Architecture Approved
    = A04 isolation boundary stable for G01 consumption
    = no G01-owned blocking decision remains
    = Architecture Approved
    = ready for L5-G-01-002 / 003 / 004 dependency consumption
    != Locked
```

Any material change to the consumed C01, D01, D02 or A04 boundary requires G01 impact assessment before lock.
