# Application Packaging & Runtime Isolation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-E-04-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Canonical Technical Responsibility:** Application packaging, runtime isolation and service-boundary realization without mandating a specific orchestrator.  
**Technical Architecture Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.1`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-E-04-001`, `L5-A-01-001`, `L5-A-03-001`, `L5-C-01-*`, `L5-D-03-005`, `L5-B-02-*`, `L4-A-028`, `L4-A-029`, `L4-A-032`.  
**Collaborates With / Referenced By:** `L5-C-02-*`, `L5-E-04-003/004/005`, `L5-E-01-*`, `L5-E-05-*`, `L5-F-01-*`.  
**Applicable ADRs:** `ADR-L5-0002`, `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0005`. Container/package format and orchestration product selection remain unresolved.

---

## 2. Purpose, Scope & Non-Goals

Define how SutraERP application capabilities are composed into deployable runtime units and when isolation is justified, without turning every domain/module into an independent service.

### In Scope

- Deployable artifact composition.
- Runtime-unit boundary and isolation criteria.
- Independent scale/restart/replace semantics.
- Co-location versus isolation constraints.
- Logical service addressing/discovery requirements.
- Runtime-local state restrictions.
- Startup, readiness and shutdown behavior.

### Out of Scope

- Business module lifecycle.
- API/event semantics.
- Orchestrator product selection.
- Network trust-policy internals.
- Persistence topology.

---

## 3. Authoritative Basis

| Source | Effect |
| --- | --- |
| `L4-A-028-BH-003/BH-007/BH-008/BH-009` | Module/version/capability governance remains distinct from runtime deployment. |
| `L4-A-029-BH-001/BH-003/BH-004/BH-006/BH-007` | Packaging may support governed compatible coexistence. |
| `L4-A-032-BH-001/BH-003/BH-004/BH-005/BH-006` | Isolation shall not bypass governed dependency compatibility. |
| `ADR-L5-0003` | Modular Application Core is the default; selective isolation only when justified by security/workload/resilience/integrity/operational needs. |
| `ADR-L5-0002` | Machine credentials and secrets remain externally governed. |
| `ADR-L5-0004` | Tenant/Institution isolation remains enforced independent of placement. |
| `ADR-L5-0005` | Runtime co-location does not alter persistence ownership. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

---

## 4. Technical Ownership & Boundaries

### Owns

- Package composition and runtime-unit identity.
- Isolation criteria.
- Independent restart/scale/replace characteristics.
- Logical addressing requirements.
- Runtime-local state constraints.
- Startup/readiness/shutdown requirements.

### Relationships

| Specification | Relationship | Boundary |
| --- | --- | --- |
| `L5-E-04-001` | Dependency | Supplies baseline deployment concepts. |
| `L5-A-01-001`, `L5-A-03-001` | Dependency | Packaging respects approved composition/dependency graph. |
| `L5-C-01-*` | Dependency | Isolated synchronous crossings use governed C01 contracts. |
| `L5-C-02-*` | Collaboration | If asynchronous crossing is required, C02 owns event/messaging semantics. E04 does not depend on later-stage C02 to define isolation. |
| `L5-D-03-005` | Dependency | Security-motivated isolation must align with trust segmentation. |
| `L5-B-02-*` | Dependency | Runtime-local state cannot replace authoritative persistence. |

---

## 5. Solution Architecture

### 5.1 Runtime Classes

| Runtime Class | Use |
| --- | --- |
| Modular Core Runtime | Default co-located execution preserving enforceable module boundaries. |
| Isolated Service/Worker Runtime | Separate execution only when an approved technical isolation reason exists. |
| Edge/Adapter Runtime | Controlled connector/adapter execution where trust or integration placement requires it. |

### 5.2 Isolation Decision Gate

A capability may move from the modular core into an isolated runtime only when at least one material reason is demonstrated and governed:

- security/trust separation;
- materially independent workload or scale;
- fault/resilience containment;
- integrity/high-risk execution boundary;
- operational lifecycle independence;
- integration/edge placement necessity.

Business-domain identity alone is not a valid isolation reason.

### 5.3 Technical Flow

```text
Approved application/module composition
→ evaluate isolation gate
→ default to modular core unless justified
→ package runtime unit
→ bind governed contracts/dependencies
→ deploy instance(s)
→ resolve isolated units by logical identity
→ health/readiness/lifecycle control.
```

### 5.4 Runtime State Rules

- Canonical business state shall not depend on process/node-local storage unless its canonical persistence owner explicitly defines that mechanism.
- Isolated units shall remain replaceable without ownership transfer.
- Machine credentials/secrets shall be resolved externally through D03-owned mechanisms.
- Tenant/Institution isolation shall remain enforced in every runtime class.

---

## 6. Architecture Decisions

| Decision ID | Decision | Position | Authority |
| --- | --- | --- | --- |
| AD-001 | Packaging topology | Modular core by default; selective isolation only by explicit technical need. | `ADR-L5-0003` |
| AD-002 | Isolated communication | Cross-runtime interaction uses governed C01/C02 mechanisms; isolation does not create hidden internal bypasses. | `ADR-L5-0003` + C01/C02 ownership |
| AD-003 | Credential handling | No embedded runtime secrets/credentials. | `ADR-L5-0002` |
| AD-004 | Persistence ownership | Co-location/isolation does not change persistence ownership. | `ADR-L5-0005` |
| AD-005 | Tenant isolation | Placement changes do not weaken Tenant/Institution isolation. | `ADR-L5-0004` |
| AD-006 | Package/orchestrator technology | Unresolved; OCI/Kubernetes remain candidates only. | ADR required if standardized |

---

## 7. Conformance Requirements

| Requirement ID | Requirement |
| --- | --- |
| CR-001 | A domain/module shall not receive a separate runtime solely because it is a domain/module. |
| CR-002 | Every isolated runtime shall record its material isolation rationale before becoming normative. |
| CR-003 | Isolated units shall use governed logical addressing/contracts rather than code-hardcoded machine endpoints. |
| CR-004 | Runtime-local state shall not become canonical business persistence absent canonical owner authority. |
| CR-005 | Startup/readiness/shutdown shall protect mandatory dependencies and accepted in-flight work. |
| CR-006 | C02 is a collaborator for asynchronous realization, not a prerequisite dependency for E04-002. |

---

## 8. Open Decisions

| Item ID | Item | Blocking? | Resolution |
| --- | --- | --- | --- |
| OD-001 | Package/container standard | No for architecture pattern; Yes before normative technology mandate | Govern through ADR if a shared standard is selected. |
| OD-002 | Orchestration platform | No for architecture pattern; Yes before normative platform mandate | Evaluate Kubernetes and alternatives through ADR governance. |
| OD-003 | Material L4 parents remain Draft | Yes for Architecture Approved | Resolve through L4 governance. |

---

## 9. Traceability Summary

**Material Dependencies:** `L5-E-04-001`, `L5-A-01-001`, `L5-A-03-001`, `L5-C-01-*`, `L5-D-03-005`, `L5-B-02-*`, `L4-A-028`, `L4-A-029`, `L4-A-032`  
**Material Collaborator:** `L5-C-02-*`  
**Applicable Approved ADRs:** `ADR-L5-0002`, `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0005`
