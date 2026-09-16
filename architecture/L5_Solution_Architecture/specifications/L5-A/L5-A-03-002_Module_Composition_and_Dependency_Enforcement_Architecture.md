# Module Composition & Dependency Enforcement Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-03-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-03 — Module, Dependency & Technical Boundary Architecture  
**Canonical Technical Responsibility:** Enforceable technical dependency/composition rules and prevention of unauthorized coupling.  
**Technical Architecture Owner:** L5-A-03 — Module, Dependency & Technical Boundary Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-11`  
**Depends On:** `L5-A-03-001` v1.0.1 Draft; `L5-A-01-001` v1.0.1 Draft; L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L2-A08_Module_Registry_Configuration_Workflows.md` v1.0.0 Locked; `L4-A-032_Module_Dependency_Change_to_Validated_Architectural_Dependency_Outcome.md` v1.0.0 Draft; `DEPENDENCY_MAP.md`.  
**Referenced By:** repository/build validation architecture; implementation composition rules derived from A03.  
**Applicable ADRs:** `ADR-L5-0003` — Contract-Driven Modular Platform with Selective Isolation (`Architecture Approved`).

---

## 2. Purpose, Scope & Non-Goals

This specification defines how the technical-module boundaries and dependency rules established by `L5-A-03-001` are declared, validated and enforced so implementation composition cannot silently violate canonical Module ownership, dependency direction, public-boundary rules or repository architecture.

### In Scope

- Machine-readable Technical Composition Declaration.
- Parent Module / Module Version binding validation.
- Same-parent and cross-parent dependency validation.
- Dependency direction and cycle checks.
- Declared-versus-actual implementation dependency checks.
- Repository, CI/build, package/composition and conditional runtime enforcement.
- Explicit, narrow, time-bounded Architecture Waivers.
- Attributable conformance evidence.

### Out of Scope

- Definition of Technical Modules — `L5-A-03-001`.
- Module Registry lifecycle/dependency ownership.
- General CI/CD architecture — `L5-F-01-*`.
- Verification strategy — `L5-F-02-*`.
- API/event mechanics — `L5-C-*`.
- Persistence ownership — `L5-B-02-*`.
- Tenant isolation — `L5-A-04-*`.
- Deployment topology — `L5-E-04-*`.
- Source language, build tool, package manager, framework or policy-engine selection.

---

## 3. Authoritative Basis & Constraints

| Source | Exact Reference | L5-A-03-002 Effect |
|---|---|---|
| `L5-A-03-001` | Technical Module model and `CR-001`–`CR-010` | Supplies the rules A03-002 enforces. |
| `L2-A08_Module_Registry_Configuration_Workflows.md` | Module Registry ownership | Technical declarations reference but do not replace Module Registry facts. |
| `L4-A-032` | `BH-001`–`BH-008`, especially `BH-003`–`BH-006` | Invalid dependency/cycle outcomes shall not enter governed architecture. |
| `DEPENDENCY_MAP.md` | §7A and Runtime Contract/Event register | Runtime consumption is distinct from foundational dependency. |
| `ADR-L5-0003` | Decision §§7–11 | Establishes the reusable modular composition and boundary pattern. |

### Constraints

1. A03-002 enforces A03-001; it does not redefine module ownership.
2. Module Registry remains sole canonical owner of Module, Module Version and canonical Module Dependency metadata.
3. Prohibited cycles and unauthorized cross-parent coupling are hard failures.
4. Runtime consumption shall not be misclassified as foundational dependency.
5. No waiver can legalize cross-owner persistence access, Tenant/isolation bypass, security/trust bypass, ownership transfer or prohibited cycles.
6. Co-location does not make a dependency valid.

---

## 4. Technical Ownership & Boundaries

### Owns

- Technical Composition Declaration model.
- Validation/enforcement order and semantics.
- Declared-versus-actual dependency drift detection.
- Architecture Waiver semantics, expiry and revalidation.
- Conformance evidence for A03 rules.

### Consumes / Collaborates With

- A03-001 Technical Module model.
- A01 platform composition.
- A04 isolation/context.
- B02 persistence boundaries.
- C01/C02 public collaboration boundaries.
- D-family security/trust constraints.
- E04 package/runtime realization.
- F01 may execute checks; F02 may verify evidence. A03 remains owner of the rules.

### Must Not Own

Canonical Module graph mutation, business ownership/rules, contract/event schemas, persistence internals, CI/CD orchestration, deployment topology or generic governance exceptions.

---

## 5. Solution Architecture

### 5.1 Enforcement Sequence

```text
Technical Composition Declaration
        ↓
Declaration validation
        ↓
Repository architecture validation
        ↓
CI/build dependency validation
        ↓
Package/composition validation
        ↓
Runtime guard only where dynamic composition requires it
        ↓
Conformance evidence
```

A violation shall be rejected at the earliest deterministic layer. Later enforcement is not justification for omitting an earlier check.

### 5.2 Technical Composition Declaration

Each Technical Module shall declare enough architecture information to validate:

- Technical Module identity;
- canonical parent Module;
- applicable Module Version context;
- direct technical dependencies;
- dependency direction;
- same-parent or cross-parent classification;
- governed public boundary for cross-parent collaboration where required;
- applicable Architecture Waiver, if any.

The declaration is architecture evidence, not a competing Module Registry. Exact serialization format remains downstream standardization.

### 5.3 Validation Rules

Before a technical dependency is admitted, validation shall confirm:

1. source and target Technical Module identity where applicable;
2. valid single-parent bindings;
3. allowed same-parent/cross-parent form;
4. no direct peer-internal cross-parent coupling;
5. no prohibited dependency-direction contradiction;
6. no prohibited cycle;
7. no contradiction with applicable canonical Module Dependency metadata;
8. runtime consumption is not incorrectly promoted to foundational dependency;
9. no owner, Tenant, persistence, security or trust boundary is bypassed.

### 5.4 Declared vs Actual Dependency Evidence

```text
declared allowed architecture
        +
actual implementation dependency evidence
        ↓
comparison
        ↓
within declaration → pass
outside declaration → block or evaluate valid waiver
```

Implementation may realize less coupling than allowed. It shall not introduce material coupling outside the declared architecture. Build/package tooling is evidence, not the source of architecture truth.

### 5.5 Package / Dynamic Composition

Package/runtime assembly shall be checked for undeclared modules, bundled peer internals, unauthorized dependencies and incompatible composition. Runtime guards are required only where material composition can be introduced dynamically and cannot be fully determined earlier.

### 5.6 Architecture Waivers

A waiver shall identify exact rule, scope, rationale, owner, approver, expiry and compensating controls/evidence. It shall be narrow and temporary.

A waiver cannot override:

- canonical ownership;
- prohibited dependency cycles/direction;
- cross-owner direct persistence restrictions;
- Tenant/Institution isolation;
- security/trust requirements;
- locked repository governance.

Expired waivers are invalid and must fail validation.

### 5.7 Conformance Evidence

Evidence shall identify the evaluated declaration, dependency graph outcome, violations, waiver state and enforcement layer. Evidence is attributable and reproducible enough for F01/F02 consumption.

---

## 6. Architecture Decisions

| Decision ID | Decision | Status | ADR |
|---|---|---|---|
| `AD-001` | Technical composition shall be explicitly declared and treated as architecture authority rather than inferred solely from implementation tooling. | Architecture Approved | `ADR-L5-0003` |
| `AD-002` | Enforcement shall be layered and fail-fast, using runtime guards only for material dynamic composition not deterministically checkable earlier. | Architecture Approved | `ADR-L5-0003` |
| `AD-003` | Narrow time-bounded technical waivers are permitted only where they cannot override higher architecture authority or non-waivable ownership/isolation/cycle rules. | Architecture Approved | `ADR-L5-0003` |

No concrete declaration format, build system, dependency-analysis product, package manager or runtime policy engine is selected.

---

## 7. Failure & Security Model

| Failure | Required Outcome |
|---|---|
| Missing/invalid declaration | Block affected composition. |
| Undeclared material dependency | Block or evaluate an eligible valid waiver. |
| Prohibited cycle/direction | Hard fail; non-waivable. |
| Cross-parent peer-internal coupling | Hard fail. |
| Cross-owner persistence/isolation/security bypass | Hard fail; non-waivable. |
| Expired/invalid waiver | Treat dependency as unauthorized. |
| Dynamic composition cannot be validated | Do not activate the composition. |

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Every governed Technical Module shall have an explicit Technical Composition Declaration. |
| `CR-002` | Declaration parent bindings shall conform to A03-001. |
| `CR-003` | Unauthorized cross-parent peer-internal coupling shall be rejected. |
| `CR-004` | Prohibited cycles and dependency-direction contradictions shall hard fail. |
| `CR-005` | Runtime consumption shall not be promoted automatically into foundational dependency. |
| `CR-006` | Actual implementation dependencies shall not materially exceed the declared allowed architecture. |
| `CR-007` | Package/runtime composition shall remain A03-conformant regardless of E04 placement. |
| `CR-008` | Runtime guards shall be used where material dynamic composition cannot be validated earlier. |
| `CR-009` | Waivers shall be explicit, narrow, approved, time-bounded and revalidated. |
| `CR-010` | Waivers shall not override ownership, persistence, Tenant/isolation, security/trust or prohibited-cycle rules. |
| `CR-011` | A03 evidence shall be attributable to declaration, graph result, violations and waiver state. |
| `CR-012` | CI/build/package technology shall not become the architecture source of truth. |

---

## 9. Open Decisions & Deferred Design

`ADR-L5-0003` resolves the former blocking A03 enforcement-pattern ADR item.

| ID | Item | Blocking Status | Resolution Path |
|---|---|---|---|
| `OD-002` | Concrete serialization/schema and repository convention for the Technical Composition Declaration. | Does not block product-neutral Draft; required before implementation standardization. | Resolve with A02/F01 after applicable runtime/build/toolchain decisions are governed. |

---

## 10. Traceability Summary

Primary upstream applicability: `L4-A-032-BH-001`–`BH-008`, L2-A08, `DEPENDENCY_MAP.md`, A03-001 and A01.

Architecture decisions: `AD-001`–`AD-003` → `ADR-L5-0003`.

Execution/verification collaborators: `L5-F-01-*` and `L5-F-02-*`; they do not own A03 rules.
