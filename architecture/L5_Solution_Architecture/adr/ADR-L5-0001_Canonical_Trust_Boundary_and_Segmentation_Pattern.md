# ADR-L5-0001 — Canonical Trust Boundary & Segmentation Pattern

**ADR ID:** `ADR-L5-0001`  
**Title:** Canonical Trust Boundary & Segmentation Pattern  
**Scope:** Shared / Family  
**Decision Owner:** L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture  
**Assigned Author:** Architecture remediation for Varad Stage-1 corpus  
**Reviewers:** Independent architecture reviewer / Lead Architect required  
**Status:** Under Evaluation  
**Decision Date:** —  
**Affected L5 Specifications:** `L5-D-03-001`; `L5-D-03-005`  
**Related ADRs:** None  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

`L5-D-03-001` and `L5-D-03-005` establish reusable security architecture for trust boundaries, trust admission, contextual trust, non-transitive trust, logical segmentation, cross-boundary communication and compromise containment.

These are material Shared / Family security decisions because they define how multiple L5 specifications and runtime capabilities must cross protected technical boundaries. The locked ADR methodology therefore requires ADR governance.

The decision must remain implementation-independent and must not select a network product, cloud provider, orchestration platform, service mesh, firewall product, port scheme or physical topology.

---

## 2. Scope & Affected Architecture

This ADR governs the reusable D03 pattern for:

- explicit logical trust boundaries;
- default-deny protected-boundary crossing;
- explicit boundary admission;
- contextual and non-transitive trust;
- preservation of Tenant/Institution and applicable execution/security context;
- privileged-boundary isolation;
- service/application/network trust segmentation semantics;
- compromise containment between protected technical zones.

It does not own:

- authentication — `L5-D-01-*`;
- authorization policy evaluation — `L5-D-02-*`;
- Tenant/Institution isolation semantics — `L5-A-04-*`;
- cryptographic/key/secrets internals — `L5-D-03-002/003/004`;
- audit/evidence persistence — `L5-D-04-*`;
- physical deployment topology — `L5-E-04-*`.

---

## 3. Authoritative Constraints

1. Trust architecture must preserve canonical business and technical ownership rather than creating a new cross-domain owner.
2. Tenant/Institution context must never be silently widened when crossing a protected technical boundary.
3. Authentication alone must not imply authorization or broad technical trust.
4. Shared runtime or co-location must not collapse logical trust boundaries.
5. Security-control failure must fail closed where the protected operation requires that control.
6. Physical separation is not automatically required; logical segmentation must remain valid across allowed deployment patterns.
7. The architecture must remain provider-neutral and technology-neutral until governed technology-selection decisions are made.

---

## 4. Decision Criteria

The selected pattern must maximize:

- ownership preservation;
- fail-closed security behavior;
- least implicit trust;
- portability across on-premises and cloud-capable deployment;
- composability with IAM, authorization, Tenant isolation and audit architectures;
- testability of protected-boundary admission and rejection;
- compromise containment;
- avoidance of hidden network or deployment coupling.

---

## 5. Alternatives Considered

### Alternative A — One implicit trusted internal zone

All internal modules/services are treated as mutually trusted after entering the SutraERP environment.

**Rejected.** It creates transitive trust, weakens compromise containment and cannot safely support selectively isolated services or engines.

### Alternative B — Physical-separation-first security

Each protected capability receives separate physical infrastructure by default.

**Rejected as the canonical pattern.** It over-constrains deployment architecture and incorrectly equates logical trust with physical topology.

### Alternative C — Explicit logical trust boundaries with governed admission

Protected technical boundaries remain explicit. Crossings require validated context, applicable identity/authorization evidence and an allowed trust relationship. Trust does not automatically transit across subsequent boundaries.

**Selected.**

---

## 6. Evaluation

| Criterion | A — Implicit Internal Trust | B — Physical Separation First | C — Explicit Logical Boundaries |
|---|---|---|---|
| Ownership preservation | Weak | Acceptable | Strong |
| Least implicit trust | Weak | Strong | Strong |
| Portability | Strong | Weak | Strong |
| Compromise containment | Weak | Strong | Strong |
| Deployment neutrality | Strong | Weak | Strong |
| Cross-spec reuse | Weak | Acceptable | Strong |
| Testability | Weak | Acceptable | Strong |

Alternative C is the only option that satisfies the full set of security, portability and architecture-layer constraints without forcing physical topology.

---

## 7. Decision

SutraERP shall use **explicit logical trust boundaries with default-deny, governed boundary admission and non-transitive trust** as the canonical D03 trust pattern.

The pattern means:

1. every protected technical boundary is explicit;
2. crossing a protected boundary requires validated context and the applicable identity/authorization/trust evidence;
3. successful admission to one boundary does not automatically grant trust across another boundary;
4. Tenant/Institution and other authoritative execution scope must be preserved and validated across crossings;
5. required control failure does not degrade into an unprotected path;
6. privileged capabilities are isolated behind explicit control points;
7. segmentation is logical and mandatory even when deployment is co-located;
8. physical placement remains owned by deployment architecture and may strengthen, but not weaken, the logical trust model.

`L5-D-03-001` owns the cross-solution trust-boundary baseline. `L5-D-03-005` owns the application/service/network segmentation realization of this pattern.

---

## 8. Consequences

### Positive

- prevents implicit transitive trust;
- supports modular-core and selectively isolated execution;
- enables deterministic negative testing of boundary admission;
- improves compromise containment;
- preserves deployment portability;
- provides one shared D03 pattern instead of per-spec trust semantics.

### Negative / Cost

- more explicit boundary contracts and enforcement points are required;
- peer architectures must carry sufficient context for protected crossings;
- observability and evidence must distinguish admitted and rejected crossings;
- deployment convenience cannot override logical boundary enforcement.

---

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Teams interpret co-location as trusted equivalence | D03 conformance rules require the same logical boundary semantics regardless of placement. |
| Boundary logic duplicates D02 authorization | D03 validates trust-boundary admission; D02 remains owner of authorization decisions. |
| D03 absorbs A04 Tenant isolation ownership | D03 consumes validated scope and preserves it; A04 remains canonical owner of Tenant/Institution isolation semantics. |
| Network products become the architecture | Product choice remains a later governed implementation/technology decision. |
| Excessive micro-segmentation creates operational burden | Only architecture-significant protected boundaries are canonical; lower-level implementation segmentation remains L6. |

---

## 10. Migration / Exit Position

No vendor migration is implied because this ADR selects an architecture pattern, not a product.

A future replacement must preserve or explicitly re-evaluate:

- fail-closed admission;
- non-transitive trust;
- context preservation;
- compromise containment;
- separation from authorization and deployment ownership.

Replacement requires a new ADR and explicit supersession.

---

## 11. Traceability

Primary downstream realization:

- `L5-D-03-001_Security_Architecture_and_Trust_Boundary_Baseline.md`
- `L5-D-03-005_Application_Service_and_Network_Trust_Segmentation_Architecture.md`

Material collaborating architecture:

- `L5-A-04-*` Tenant/Institution isolation;
- `L5-D-01-*` IAM;
- `L5-D-02-*` authorization;
- `L5-D-04-*` audit/evidence;
- `L5-E-04-*` runtime/deployment.

This ADR does not create business behavior or replace exact L4-to-L5 BH traceability in the affected specifications.

---

## 12. Approval Record

**Current disposition:** Architecture decision fully evaluated and recommended for approval.  
**Formal status:** `Under Evaluation`.  
**Reason not marked Architecture Approved:** the locked ADR methodology requires an independent architecture reviewer/approver; the authoring/remediation action cannot self-approve the ADR.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| D03 technical decision owner | Recommended | 2026-09-11 | D03 Stage-1 architecture review/remediation |
| Independent architecture reviewer / Lead Architect | Pending | — | Required before `Architecture Approved` |
