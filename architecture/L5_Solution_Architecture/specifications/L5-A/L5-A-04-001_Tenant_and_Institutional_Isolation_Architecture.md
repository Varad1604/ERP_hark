# Tenant & Institutional Isolation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-04-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-04 — Tenant & Institutional Isolation Architecture  
**Canonical Technical Responsibility:** Technical Tenant/Institution isolation while preserving the locked business model.  
**Technical Architecture Owner:** L5-A-04 — Tenant & Institutional Isolation Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.1.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Pilot ADR Candidate Disposition v1.0.0; `L5-A-01-001` v1.0.1 Draft; `L4-A-001` v1.0.0 Draft; `L4-A-002` v1.0.0 Draft; `L4-A-003` v1.0.0 Draft; `L4-B-001` v1.0.0 Review; `L4-B-002` v1.0.0 Review; `L4-B-003` v1.0.0 Review; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `L2-B01_Institution_Workflow.md`; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** `L5-A-04-002` and peer architectures requiring validated Tenant/Institution isolation semantics.  
**Applicable ADRs:** `ADR-L5-0004` — Tenant and Institution Isolation Enforcement Pattern (`Architecture Approved`).

---

## 2. Purpose, Scope & Non-Goals

This specification defines the technical isolation semantics and control points for Tenant and Institution scope. It consumes upstream Tenant/Institution lifecycle eligibility but does not own or redefine those business lifecycles.

### In Scope

- Tenant as mandatory root protected technical scope.
- Institution as subordinate scope inside a Tenant.
- Isolation admission and fail-closed behavior.
- Logical isolation independent of physical placement.
- Handoff of validated scope semantics to A04-002 and peer L5 architectures.

### Out of Scope

- Tenant/Institution business lifecycle or business authorization.
- Exact propagation representation — A04-002 and peer owners.
- Authentication — D01.
- Authorization decisions — D02.
- Physical persistence topology — B02.
- Security/trust internals — D03.
- Runtime/deployment topology — E04.

---

## 3. Authoritative Basis & Constraints

| Source | L5-A-04-001 Effect |
|---|---|
| `L2-A01_Platform_Tenant_Foundation_Architecture.md` | Establishes Tenant foundation and scope semantics consumed by L5. |
| `L2-B01_Institution_Workflow.md` | Establishes Institution as governed institutional scope without transferring Tenant ownership. |
| applicable `L4-A-001/002/003` and `L4-B-001/002/003` | Provide Tenant/Institution behavioral applicability used by isolation control points. |
| L1 v2.1 + applicable amendment | Preserves canonical ownership and Tenant-scoped enterprise model. |
| pilot `ADR-C1` | Historical candidate provenance for A04 isolation pattern. |
| `ADR-L5-0004` | Canonical approved isolation-enforcement decision. |

### Constraints

1. Tenant is the mandatory root protected scope for Tenant-scoped execution.
2. Institution is subordinate to Tenant and cannot replace/widen Tenant scope.
3. Missing, ambiguous, mismatched or cross-Tenant scope shall fail closed.
4. No default/global Tenant fallback is permitted for protected mutable operations.
5. Logical isolation remains mandatory even when infrastructure is physically shared.
6. A04 consumes lifecycle eligibility; it does not create lifecycle states/transitions.
7. Peer architectures may strengthen isolation but shall not weaken the A04 invariant.

---

## 4. Technical Ownership & Boundaries

### Owns

- Tenant-rooted isolation invariant.
- Valid isolation-scope relationship between Tenant and Institution.
- Technical admission/rejection semantics for protected execution.
- Requirement that validated scope be handed to A04-002 and peer boundaries.

### Consumes / Collaborates With

- A01 application placement.
- A04-002 propagation/data isolation.
- B02 persistence isolation.
- C01/C02 contract/event boundaries.
- D01 identity, D02 authorization, D03 trust, D04 audit/evidence.
- E04 runtime/deployment realization.

### Must Not Own

Tenant/Institution business lifecycle, authentication, authorization policy/decision, persistence ownership, trust implementation, deployment topology or business events.

---

## 5. Solution Architecture

### 5.1 Isolation Scope Model

```text
Tenant (mandatory protected root)
        ↓
Institution (optional/required according to business context; always subordinate)
        ↓
Lower scope(s) when governed by owning architecture
```

A protected operation shall execute only when its required scope is present, valid and compatible with the target resource/capability.

### 5.2 Isolation Admission

Before protected execution:

1. resolve the required Tenant scope from authoritative execution context;
2. validate that the target belongs to or is permitted within that Tenant scope;
3. where Institution is required, validate it as a subordinate scope of the same Tenant;
4. reject missing, ambiguous, mismatched, switched or widened scope;
5. hand the validated scope semantics to A04-002/peer execution and persistence controls.

### 5.3 Physical Placement Independence

Tenant isolation is a logical architecture invariant. Shared process, host, cluster, database technology or physical database does not grant cross-Tenant authority. Physical separation may be introduced by B02/D03/E04 where justified, but it is not the definition of Tenant isolation.

### 5.4 Lifecycle Boundary

A04-001 may check whether upstream Tenant/Institution state makes execution eligible. It shall not create, advance, suspend, restore or retire Tenant/Institution business state.

### 5.5 Failure Model

| Failure | Required Outcome |
|---|---|
| Tenant scope missing/ambiguous | Fail closed. |
| Institution belongs to another Tenant | Fail closed. |
| Scope is widened/switched during execution | Reject continuation and preserve evidence as applicable. |
| Peer boundary cannot preserve required scope | Do not execute protected operation. |
| Physical co-location creates apparent broad access | Logical A04/B02/D03 controls remain authoritative. |

---

## 6. Architecture Decisions

| Decision ID | Decision | Status | ADR |
|---|---|---|---|
| `AD-001` | Tenant is the mandatory root protected execution scope. | Architecture Approved | `ADR-L5-0004` |
| `AD-002` | Institution scope is subordinate to Tenant and shall never widen or replace it. | Architecture Approved | `ADR-L5-0004` |
| `AD-003` | Isolation is logically mandatory independent of physical placement; missing/invalid scope fails closed. | Architecture Approved | `ADR-L5-0004` |

---

## 7. Security, Privacy & Operational Impact

- Prevents cross-Tenant execution and data access by construction.
- Supports shared infrastructure without equating sharing with authorization.
- Requires negative tests for missing, mismatched and switched scope.
- Leaves authentication, authorization, cryptography/trust and audit internals with their canonical D-family owners.
- Introduces no numeric performance, availability or capacity target.

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Every protected Tenant-scoped execution shall resolve one unambiguous Tenant root scope. |
| `CR-002` | Institution scope, when present, shall be validated as subordinate to the same Tenant. |
| `CR-003` | Missing, ambiguous, mismatched, switched or widened scope shall fail closed. |
| `CR-004` | No default/global Tenant fallback shall authorize protected mutable execution. |
| `CR-005` | Shared physical placement shall not weaken logical Tenant/Institution isolation. |
| `CR-006` | A04-001 shall not create upstream Tenant/Institution lifecycle transitions. |
| `CR-007` | Validated isolation semantics shall be handed to A04-002 and applicable peer owners without duplicating their internals. |
| `CR-008` | Deployment realization may strengthen but shall not weaken the Tenant-rooted isolation invariant. |

---

## 9. Open Decisions & Deferred Design

`ADR-L5-0004` resolves the former blocking `ADR-C1` item.

| ID | Item | Blocking Status | Resolution Path |
|---|---|---|---|
| `OD-002` | Canonical repository evidence for controlled use of cited Draft/Review L4 sources. | Blocks formal promotion where repository governance requires upstream evidence; does not reopen the A04 isolation decision. | Resolve through upstream/governance promotion path. |

---

## 10. Traceability Summary

Canonical decision: `AD-001`–`AD-003` → `ADR-L5-0004`.

Historical candidate provenance: `ADR-C1` in the locked pilot disposition.

Primary collaboration: A04-002, B02, C01/C02, D01/D02/D03/D04 and E04.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
