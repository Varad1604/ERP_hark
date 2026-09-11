# API & Contract Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-C-01-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-C — Contracts, Events & Integration Architecture  
**Major Area:** L5-C-01 — API & Contract Architecture  
**Canonical Technical Responsibility:** Technical API/contract principles and ownership-preserving synchronous boundaries.  
**Technical Architecture Owner:** L5-C-01 — API & Contract Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.1`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-A-01-001`, `L5-A-03-001`, `L5-A-04-002`, `L5-B-02-001/002/003`, `L5-D-01-001`, `L5-D-03-001`, `L4-A-001`, `L4-A-015`, `L4-A-028`, `L4-A-050`.  
**Referenced By:** `L5-C-01-002`, `L5-C-01-003`, `L5-C-02-*`, `L5-C-03-*`.  
**Applicable ADRs:** `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0006`. Exact API transport/style, API-management product and primary runtime/framework remain unresolved technology/shared-pattern decisions.

---

## 2. Purpose, Scope & Non-Goals

Define the reusable synchronous contract boundary through which SutraERP capabilities collaborate without transferring canonical business ownership, bypassing Tenant/Institution scope, or exposing persistence internals.

### In Scope

- API/contract ownership and publication boundary.
- Internal versus external contract classification.
- Conceptual request/response context for identity, Tenant/Institution scope, correlation and authority.
- Contract-first definition, catalogue and lifecycle handoff.
- External API-management/ingress boundary at architecture level.
- Contract semantics independent of runtime placement.

### Out of Scope

- Business rules/lifecycles/canonical events.
- Authentication/authorization/cryptography/secrets internals — L5-D.
- Persistence/transaction/concurrency — L5-B-02.
- Event/messaging — L5-C-02.
- Exact language/framework/gateway/protocol/version encoding.

---

## 3. Authoritative Basis

| Source | L5 Effect |
| --- | --- |
| `L4-A-001-BH-003/BH-005/BH-009` | Contracts carry validated Tenant/platform scope context. |
| `L4-A-015-BH-001/BH-004/BH-005/BH-008/BH-010/BH-011` | Cross-capability calls preserve owner authority and explicit outcomes. |
| `L4-A-028-BH-003/BH-007/BH-008/BH-009` | Contracts bind to governed capabilities, not physical placement. |
| `L4-A-050-BH-001/BH-003/BH-006/BH-008` | Scope context is resolvable/revalidated independently of authorization. |
| `ADR-L5-0003` | Contract-driven modular platform with selective isolation; business-domain identity alone does not create a remote boundary. |
| `ADR-L5-0004` | Tenant/Institution isolation must be enforced across technical boundaries. |
| `ADR-L5-0006` | Owner-local transaction/concurrency boundaries shall not be propagated across owners. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

---

## 4. Technical Ownership & Boundaries

### Owns

- Synchronous Contract Boundary.
- Internal/external contract classification.
- Contract catalogue ownership/discovery metadata.
- Conceptual request/response context categories.
- External API-management boundary.
- Compatibility handoff to C01-003.

### Relationships

| Specification | Boundary |
| --- | --- |
| `L5-A-01-001` | Contract semantics survive modular-core vs selectively isolated placement. |
| `L5-A-04-002` | C01 carries scope context; A04 defines it. |
| `L5-B-02-001/002/003` | Provider processing remains owner-local; C01 exposes no persistence bypass. |
| `L5-D-01/02/03` | Identity/authorization/trust are enforced by D-family owners. |
| `L5-C-01-002` | Owns synchronous admission/execution semantics. |
| `L5-C-01-003` | Owns versioning, compatibility, technical error and idempotency semantics. |

---

## 5. Solution Architecture

### 5.1 Technical Elements

| Element | Responsibility |
| --- | --- |
| Contract Definition | Canonical technical request/response definition for one governed capability boundary. |
| Contract Catalogue | Discoverable ownership, version and lifecycle metadata. |
| Synchronous Contract Boundary | Preserve scope, ownership and technical context across invocation. |
| External API Management Boundary | Expose only released external contracts and apply technical ingress policy without business logic ownership. |
| Contract Telemetry Hook | Emit correlation/contract evidence without becoming the observability owner. |

### 5.2 Primary Flow

```text
Consumer
→ resolve released contract
→ establish validated identity/scope/correlation context
→ external API-management boundary when applicable
→ owning capability
→ authorization handoff
→ owner-local processing
→ governed response/rejection
→ telemetry handoff.
```

### 5.3 Failure Rules

- Missing/invalid scope or authority context fails closed.
- Unreleased/retired contracts are rejected unless a governed compatibility path exists.
- Provider unavailability/overload returns explicit technical failure; no business success is fabricated.
- Internal errors shall not leak stack, storage or secret internals.

---

## 6. Architecture Decisions

| Decision ID | Decision | Position | Authority |
| --- | --- | --- | --- |
| AD-001 | Contract model | Capability-oriented contract boundary preserving canonical ownership. | C01 responsibility + L4-A-015/A028 |
| AD-002 | Placement independence | Contract semantics do not change between modular-core and selectively isolated placement. | `ADR-L5-0003` |
| AD-003 | Tenant/Institution context | Required scope is explicit and fail-closed. | `ADR-L5-0004` |
| AD-004 | Transaction ownership | Synchronous contract crossing does not propagate a caller transaction across owners. | `ADR-L5-0006` |
| AD-005 | External exposure | External contracts pass through a governed technical ingress/API-management boundary that owns no business logic. | Local architecture decision; product selection deferred |
| AD-006 | API transport/style | Unresolved; REST/HTTP/OpenAPI, gRPC or other styles are candidates only until governed. | ADR if shared/repository standard selected |
| AD-007 | Runtime/framework | Unresolved; no language/framework is canonical from this file. | ADR required for normative selection |

---

## 7. Conformance Requirements

| Requirement ID | Requirement |
| --- | --- |
| CR-001 | Cross-owner synchronous integration shall invoke the owning capability through a governed contract and shall not directly modify another owner's persistence/internal state. |
| CR-002 | Protected requests shall carry validated Tenant/Institution scope where required and fail closed if missing/inconsistent. |
| CR-003 | Contract semantics shall remain independent of local versus selectively isolated runtime placement. |
| CR-004 | Contract catalogue metadata shall identify ownership/status/version without exposing persistence/framework internals. |
| CR-005 | External ingress/API-management components shall not become business-rule or canonical business-state owners. |

---

## 8. Open Decisions

| Item ID | Item | Blocking? | Resolution |
| --- | --- | --- | --- |
| OD-001 | Shared synchronous API transport/style | No for technology-neutral architecture; Yes before normative shared style mandate | Govern through ADR if repository/shared standard is required. |
| OD-002 | Primary runtime/language/framework | No for C01 architecture; Yes before normative technology mandate | Govern through canonical technology selection. |
| OD-003 | API-management product | No for boundary architecture; Yes before normative product mandate | Evaluate through ADR if a shared platform is selected. |
| OD-004 | Material L4 parents remain Draft | Yes for Architecture Approved | Resolve through L4 governance. |

---

## 9. Traceability Summary

**Material Dependencies:** `L5-A-01-001`, `L5-A-03-001`, `L5-A-04-002`, `L5-B-02-001/002/003`, `L5-D-01-001`, `L5-D-03-001`, `L4-A-001`, `L4-A-015`, `L4-A-028`, `L4-A-050`  
**Applicable Approved ADRs:** `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0006`
