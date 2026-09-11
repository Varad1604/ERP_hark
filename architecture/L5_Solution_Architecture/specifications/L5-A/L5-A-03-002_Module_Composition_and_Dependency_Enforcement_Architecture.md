# Module Composition & Dependency Enforcement Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-03-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-03 — Module, Dependency & Technical Boundary Architecture  
**Canonical Technical Responsibility:** Enforceable technical dependency/composition rules and prevention of unauthorized coupling.  
**Technical Architecture Owner:** L5-A-03 — Module, Dependency & Technical Boundary Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** `L5-A-03-001` v1.0.0 Draft; `L5-A-01-001` v1.0.0 Draft; L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L2-A08_Module_Registry_Configuration_Workflows.md` v1.0.0 Locked; `L4-A-032_Module_Dependency_Change_to_Validated_Architectural_Dependency_Outcome.md` v1.0.0 Draft; `DEPENDENCY_MAP.md`.  
**Referenced By:** `L5-A-03-001`; repository/build validation architecture; implementation composition rules derived from A03.  
**Applicable ADRs:** Required but unresolved — repository-wide/family module-composition and dependency-enforcement pattern shall undergo formal ADR scope review; no canonical ADR is currently allocated for this A03 decision.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines how the technical-module boundaries and dependency rules established by `L5-A-03-001` are declared, validated and enforced so that implementation composition cannot silently violate canonical Module ownership, dependency direction, public-boundary rules or repository architecture.

The architecture is fail-fast by design: invalid composition is rejected at the earliest layer where it can be determined, with runtime enforcement used only where dynamic composition cannot be fully validated earlier.

### In Scope

- Product-neutral representation of the approved technical-module composition and dependency graph.
- Explicit machine-readable technical dependency declaration for each Technical Module.
- Validation of canonical parent Module and governing Module Version binding inherited from `L5-A-03-001`.
- Validation of allowed same-parent and cross-parent technical dependencies.
- Prevention of direct cross-parent coupling to peer internal implementation boundaries.
- Technical dependency-direction and cycle validation.
- Alignment checks between technical dependencies and canonical Module Dependency metadata where that upstream dependency is applicable.
- Layered repository, CI/build, composition/package and conditional runtime enforcement.
- Comparison of declared architecture with actual implementation dependency evidence.
- Hard-fail handling for unauthorized dependency/coupling.
- Explicit, time-bounded architecture waivers for narrowly scoped technical exceptions that do not violate higher authority.
- Conformance evidence sufficient for repository/build verification.

### Out of Scope / Non-Goals

- Definition of Technical Module boundaries, parent mapping or dependency categories — `L5-A-03-001`.
- Canonical Module, Module Version, Module lifecycle or Module Dependency ownership — Module Registry upstream architecture.
- Approval/change of the canonical Module dependency graph — Module Registry governance and `L4-A-032` behavior.
- General CI/CD pipeline architecture — `L5-F-01-*`.
- Verification strategy beyond A03 conformance evidence — `L5-F-02-*`.
- API/event contract mechanics — `L5-C-01-*` / `L5-C-02-*`.
- Persistence ownership or cross-owner write authority — `L5-B-02-*`.
- Tenant/Institution isolation — `L5-A-04-*`.
- Runtime/deployment topology — `L5-E-04-*`.
- Authorization/security policy implementation — L5-D.
- Selection of source language, build tool, dependency-analysis product, package manager, runtime framework or policy engine.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-A-03-002 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-A-03-002` row | A03-002 owns enforceable technical dependency/composition rules and prevention of unauthorized coupling. | Establishes canonical responsibility. |
| L5 Peer | `L5-A-03-001_Module_Dependency_and_Technical_Boundary_Architecture.md` | v1.0.0 / Draft | §§2, 4–5; `C-001`–`C-008` | Defines Technical Modules, parent binding, dependency classes, cross-parent public-boundary rules and invalid dependency forms; explicitly delegates enforcement to A03-002. | Supplies the architecture rules that this specification enforces without redefining them. |
| L5 Peer | `L5-A-01-001_Application_and_Platform_Architecture.md` | v1.0.0 / Draft | A03 collaboration boundary | Overall application structure is modular-core/selective-isolation direction and delegates enforceable dependency/composition rules to A03. | Provides platform context without transferring deployment ownership. |
| L2 | `L2-A08_Module_Registry_Configuration_Workflows.md` | v1.0.0 / Locked | Module Registry ownership and dependency governance | Canonical Module/Module Version/Module Dependency metadata belongs to Module Registry and a Module is not an implementation artifact. | Technical declaration may reference but shall not replace canonical Module metadata. |
| L4 | `L4-A-032_Module_Dependency_Change_to_Validated_Architectural_Dependency_Outcome.md` | v1.0.0 / Draft — explicitly authorized for controlled A03 authoring through the existing A03 baseline | `BH-001`–`BH-008`, especially `BH-003`–`BH-006` | Dependencies are validated for eligibility/compatibility/circularity; invalid or unapproved dependency changes leave the canonical graph unchanged. | Requires technical realization to reject invalid dependency structure rather than accepting architecture drift. |
| Repository Governance | `DEPENDENCY_MAP.md` | Current canonical `main` | §§1, 7A and Runtime Contract/Event register | Architectural dependency direction remains acyclic; runtime contract/event/read-model consumption does not automatically become foundational dependency or reverse ownership. | Prevents technical dependency detection from misclassifying runtime interaction as upstream ownership/dependency. |
| L5 Governance | `L5_Technology_Selection_and_ADR_Methodology.md` | v1.0.1 / Locked | §§1–5, especially mandatory ADR triggers | Reusable cross-specification patterns and material module/service-boundary architecture require ADR governance. | Makes the enforcement/declaration pattern ADR-required before Architecture Approval. |
| L5 Governance | `L5_ADR_Register.md` | v1.0.0 / Locked, current `main` | canonical allocation table/rules | ADR IDs are allocated only through the canonical register; currently allocated ADRs do not govern A03 enforcement. | Prohibits inventing an A03 ADR number in this Draft. |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| `C-001` | A03-002 enforces rules defined by A03-001; it shall not redefine Technical Module ownership/boundaries. | A03-001 scope/boundary | Validators consume the A03-001 model as authoritative input. |
| `C-002` | Module Registry remains sole canonical owner of Module/Module Version/Module Dependency metadata. | L2-A08; A03-001 `C-003` | Technical dependency declaration is an L5 realization artifact, not a competing Module Registry. |
| `C-003` | Canonical dependency graph shall reject prohibited cycles and invalid/unapproved dependency changes. | L4-A-032 `BH-003`–`BH-006` | Dependency/cycle violations are blocking failures rather than warnings. |
| `C-004` | Runtime contract/event/read-model consumption does not automatically create foundational dependency or reverse ownership. | `DEPENDENCY_MAP.md` §7A | Enforcement must distinguish architectural dependency from runtime consumption. |
| `C-005` | Cross-owner direct persistence writes are prohibited. | L1/A03-001/B02 | No architecture waiver can legalize a dependency path whose effect is unauthorized cross-owner persistence modification. |
| `C-006` | Tenant/Institution isolation and security/trust constraints cannot be weakened by composition. | A04/L5-D; L5 Validation HF-12 | A03 waiver/enforcement cannot override isolation/trust requirements. |
| `C-007` | Shared runtime/package/deployment placement does not transfer ownership or authorize internal coupling. | A03-001; E04 boundary | Co-location is not proof that a dependency is valid. |
| `C-008` | Material reusable module/dependency enforcement pattern requires ADR governance. | L5 ADR methodology §3 | AD-001–AD-003 remain Draft directions until formal ADR scoping/approval. |

### 3.3 Assumptions

No implementation technology is assumed.

The Lead Architect approved the following controlled Draft directions on 2026-09-11, subordinate to repository authority:

- layered fail-fast enforcement across repository, CI/build, composition/package and runtime only where dynamic composition requires it;
- an explicit machine-readable technical dependency declaration that is verified against actual implementation dependencies;
- explicit, time-bounded architecture waivers for narrowly scoped technical exceptions, with no waiver permitted to override upstream ownership, dependency-graph integrity, persistence-isolation, Tenant/Institution isolation or other locked architecture.

These choices establish a reusable A03 technical standard and therefore remain provisional pending ADR scope review.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The **Technical Composition Declaration** model for A03 technical-module realization.
- Dependency-validation control points and enforcement order.
- Validation of declared source/target Technical Module identities and canonical parent bindings supplied by A03-001.
- Validation of same-parent/cross-parent dependency form, direction and cycle constraints.
- Detection of unauthorized peer-internal coupling.
- Detection of declared-versus-actual implementation dependency drift.
- Technical conformance outcome: conformant, blocked by violation, or allowed only by a valid technical waiver.
- The **Architecture Waiver** model for narrowly scoped temporary technical exceptions to A03 enforcement rules.
- Expiry/revalidation semantics for waivers.

### Consumes / Collaborates With

| Exact L5 Architecture | Consumed Responsibility | A03-002 Boundary |
|---|---|---|
| `L5-A-03-001` | Technical Module model, parent mapping, valid/invalid dependency forms | A03-002 enforces; it does not redefine. |
| `L5-A-01-001` | Overall application/platform structure | Enforcement remains compatible with modular-core/selective-isolation structure. |
| `L5-A-04-*` | Tenant/Institution isolation/context | Composition validation must not permit isolation bypass. |
| `L5-B-02-*` | Persistence ownership/isolation | A dependency may not be waived into cross-owner direct persistence authority. |
| `L5-C-01-*` / `L5-C-02-*` | Governed synchronous/event boundaries | Cross-parent implementation dependencies must route through approved public boundaries where applicable. |
| `L5-D-*` | Security/authorization/trust rules | A03 enforcement consumes those constraints rather than redefining them. |
| `L5-E-04-*` | Packaging/runtime/deployment realization | A03 supplies composition constraints; E04 decides placement. |
| `L5-F-01-*` | CI/build/release pipeline | F01 may execute A03 checks; A03 owns the dependency rules and validation semantics. |
| `L5-F-02-*` | Verification/test architecture | F02 may verify A03 conformance; A03 owns the rules being verified. |

### Must Not Own

- Canonical Module Registry graph approval or mutation.
- Business-domain ownership, business dependency semantics or upstream lifecycle decisions.
- Technical Module definition or parent/dependency taxonomy already owned by A03-001.
- Source-code layout, package names, class names, build-tool syntax or repository implementation conventions not yet governed.
- Contract/event schemas, transport mechanisms or persistence internals.
- CI/CD orchestration or deployment topology.
- A generic exception mechanism that overrides repository governance.

---

## 5. Solution Architecture

### 5.1 Enforcement Architecture

```text
Technical Composition Declaration
        ↓
Declaration validation
        ↓
Repository architecture validation
        ↓
CI / build dependency validation
        ↓
Package / composition validation
        ↓
Runtime guard where composition is dynamic
        ↓
Conformance evidence
```

A violation shall be rejected at the earliest layer capable of determining it. A later enforcement layer shall not be used as justification for omitting an earlier deterministic check.

### 5.2 Technical Composition Declaration

Each Technical Module shall have an explicit machine-readable declaration sufficient to establish its allowed architecture without inferring canonical design solely from implementation artifacts.

The declaration shall represent, at minimum, architecture concepts necessary to validate:

- Technical Module identity;
- canonical parent Module identity;
- applicable governing Module Version context where required by A03-001;
- declared direct Technical Module dependencies;
- dependency direction;
- classification as same-parent or cross-parent;
- applicable public technical boundary/reference for cross-parent collaboration where such a boundary is required;
- applicable temporary Architecture Waiver reference, if any.

The declaration shall not duplicate the canonical Module Registry. Canonical Module and Module Dependency facts remain upstream-owned and are referenced for alignment.

Exact serialization format, repository filename, schema language and tooling are deferred to implementation/technology standardization.

### 5.3 Declaration Admission Validation

Before a technical dependency can be treated as allowed composition, A03-002 shall validate:

1. source Technical Module exists in the governed technical-module model;
2. target Technical Module exists where a direct technical relation is declared;
3. both have valid canonical parent bindings from A03-001;
4. the declared relation matches the allowed same-parent/cross-parent form;
5. a cross-parent relation does not target another parent's internal implementation boundary;
6. the dependency does not create a prohibited technical/canonical dependency-direction contradiction;
7. applicable canonical Module Dependency metadata is not contradicted;
8. runtime contract/event/read-model consumption is not incorrectly promoted into a foundational dependency;
9. no applicable owner/Tenant/security/persistence boundary is bypassed.

Failure of a mandatory check blocks admission unless the failed rule is explicitly categorized as technically waivable and a valid waiver exists.

### 5.4 Dependency Graph Validation

A03-002 maintains a technical realization graph derived from the declarations.

Conceptually:

```text
Technical Module A
      ↓ declared dependency
Technical Module B
      ↓
Technical Module C
```

Validation shall detect both direct and transitive violations that are material to architecture, including:

- undeclared direct technical dependency;
- dependency direction contrary to A03-001 or applicable canonical Module Dependency direction;
- prohibited direct or indirect cycle;
- cross-parent peer-internal dependency;
- multi-parent Technical Module realization;
- dependency on another technical owner's internal state/persistence;
- dependency that converts runtime consumption into false foundational ownership/dependency.

A technical cycle that would contradict the canonical Module dependency graph or repository dependency direction is non-waivable.

### 5.5 Repository Architecture Validation

Repository validation checks the declared composition itself before implementation-specific dependency analysis.

It shall reject at least:

- missing/ambiguous parent binding;
- duplicate/conflicting Technical Module identity;
- invalid cross-parent dependency declaration;
- prohibited graph cycle;
- declaration contradicting canonical Module/Module Version/dependency metadata;
- expired or invalid waiver reference.

Repository validation produces deterministic architecture-conformance evidence but does not mutate upstream Module Registry data.

### 5.6 CI / Build Dependency Validation

CI/build validation compares implementation dependency evidence against the Technical Composition Declaration.

```text
declared allowed dependency graph
        +
actual implementation dependency evidence
        ↓
comparison
        ↓
within declaration → pass
outside declaration → hard failure / valid waiver evaluation
```

Actual implementation dependency evidence may come from source imports, package/module references, generated dependency metadata or another product-specific mechanism once selected. No single mechanism is made canonical here.

The key invariant is:

> Implementation may realize less coupling than the declaration permits, but shall not introduce material coupling outside the declared and valid architecture.

A build tool or package manager is evidence of implementation composition; it is not the canonical source of architecture authority.

### 5.7 Package / Composition Validation

Where technical modules are assembled into deployable/runtime composition, the resulting composition shall be checked against A03 declarations before release/deployment proceeds.

This layer detects architecture violations that may not appear as direct source dependency, such as:

- prohibited bundled peer internals;
- undeclared plugin/module attachment;
- composition that creates an unauthorized technical dependency;
- incompatible technical-module versions where such compatibility is governed by the declaration/upstream Module Version context.

This validation does not decide deployment topology; it checks that E04's realized package/runtime composition remains A03-conformant.

### 5.8 Runtime Dynamic Composition Guard

Runtime enforcement is required only where a material dependency can be introduced or resolved dynamically and therefore cannot be fully proven during repository/build/package validation.

Examples of architecture categories include dynamically discovered module/plugin/capability binding. No specific framework or loading technology is selected.

The runtime guard shall:

- resolve the requesting and target Technical Module identities;
- validate the dependency against the effective declared composition/waiver state;
- reject unauthorized dynamic binding before it becomes an active dependency;
- not infer that a runtime-discoverable capability is automatically a valid architectural dependency.

Runtime enforcement supplements earlier validation. It shall not become the normal mechanism for discovering static architecture violations after deployment.

### 5.9 Unauthorized Coupling Failure Model

| Violation | Required Outcome |
|---|---|
| Undeclared implementation dependency | Hard fail the applicable validation gate unless a valid waivable exception exactly covers it. |
| Cross-parent dependency on peer internal implementation | Hard fail; require governed public boundary or architecture correction. |
| Prohibited canonical/architectural cycle | Hard fail; no waiver when it contradicts upstream dependency integrity. |
| Dependency requires cross-owner direct persistence write | Hard fail; non-waivable. |
| Missing/invalid canonical parent binding | Hard fail; correct A03-001 mapping rather than infer ownership. |
| Runtime consumption misclassified as foundational dependency | Hard fail architecture declaration and correct classification. |
| Expired waiver | Treat underlying violation as unwaived and fail. |
| Dynamic unauthorized module binding | Reject binding at runtime and surface technical conformance failure. |

Warnings may accompany diagnostics, but a mandatory architecture violation shall not be converted into a warning-only result.

### 5.10 Architecture Waiver

A waiver is a controlled temporary exception to a specifically identified **technical A03 enforcement rule**. It is not a new architecture authority and does not modify canonical ownership or upstream dependency metadata.

A valid waiver shall identify:

- exact source Technical Module;
- exact target Technical Module or affected composition boundary;
- exact A03 rule being excepted;
- technical rationale;
- known architecture risk/consequence;
- accountable technical owner;
- approval reference required by architecture governance;
- expiry date or objectively verifiable expiry/remediation condition;
- remediation target;
- affected declaration/version scope.

A waiver shall be narrowly scoped. Broad statements such as “legacy code may bypass module rules” are invalid.

### 5.11 Non-Waivable Violations

A03-002 shall never use a waiver to authorize:

- transfer or duplication of canonical business ownership;
- contradiction of canonical Module/Module Version authority;
- a dependency cycle prohibited by the canonical Module dependency graph/repository dependency direction;
- cross-owner direct persistence writes or modification of another technical owner's internal state;
- Tenant/Institution isolation bypass;
- security/trust violation prohibited by authoritative L5-D architecture;
- fabricated canonical event/contract/business behavior;
- any exception that requires changing L1–L4 meaning without upstream governance.

Where the desired exception requires any of the above, the architecture itself must be changed through the proper owning governance path; A03 cannot waive it.

### 5.12 Waiver Expiry and Remediation

```text
violation detected
      ↓
valid waiver?
   no     yes
   ↓       ↓
 block   temporary allow
           ↓
       expiry/remediation check
           ↓
      completed → remove waiver
      expired   → violation becomes hard fail
```

A waiver shall not silently renew itself. Renewal is a new architecture-governance decision and shall preserve history of the prior exception.

Repeated waivers for the same structural problem shall trigger architecture review of whether the declared module/dependency design remains valid rather than normalizing permanent exception debt.

### 5.13 Conformance Evidence

A03-002 enforcement shall make evidence available sufficient to establish:

- declaration version/reference evaluated;
- Technical Module identities and parent bindings evaluated;
- dependency graph validation outcome;
- declared-versus-actual dependency comparison outcome where applicable;
- waiver identity/status where one affected the result;
- failed rule/relationship when non-conformant;
- validation layer that detected the issue.

Exact evidence persistence and audit-grade storage remain owned by B02/D04 as applicable.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Status | Basis | Consequence | ADR |
|---|---|---|---|---|---|
| `AD-001` | Dependency/composition rules use layered fail-fast enforcement: repository → CI/build → package/composition → runtime only where dynamic composition requires it. | **Provisional Draft direction** | Frozen A03-002 responsibility; L4-A-032 invalid-dependency rejection; Lead Architect selection 2026-09-11. | Violations are blocked as early as determinable and cannot rely on production-time discovery. | **ADR required — reusable module/dependency enforcement pattern; scope unresolved.** |
| `AD-002` | Technical architecture is declared explicitly in a machine-readable Technical Composition Declaration and verified against actual implementation dependencies. | **Provisional Draft direction** | Need for deterministic enforcement without making implementation tooling architecture authority; Lead Architect selection 2026-09-11. | Architecture becomes declarative and objectively enforceable while Module Registry remains canonical for Module metadata. | **ADR required/scope review with AD-001.** |
| `AD-003` | Narrow technical exceptions require explicit, approved, time-bounded Architecture Waivers with owner, rationale, risk, expiry and remediation target. | **Provisional Draft direction** | Need for controlled transition/refactoring without warning-only architecture; Lead Architect selection 2026-09-11. | Temporary debt remains visible and expires automatically; non-waivable higher-authority violations stay blocked. | **ADR scope review with A03 enforcement pattern.** |
| `AD-004` | Mandatory architecture violations are hard failures, not warning-only conditions. | Defensible L5 derivation | Frozen “enforceable” responsibility; L4-A-032 rejection semantics; L5 Validation integrity gates. | Implementation cannot normalize known unauthorized coupling. | Covered by A03 enforcement ADR scope. |

No programming language, build system, dependency scanner, policy engine, module framework or runtime product is selected.

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaboration |
|---|---|---|
| Ownership integrity | Prevents implementation coupling from becoming de facto ownership transfer. | A03-001/A03-002; upstream owners |
| Tenant/security isolation | Invalid composition cannot bypass A04/D03 boundaries; such violations are non-waivable. | A04, D03 |
| Persistence integrity | Direct cross-owner persistence dependency is blocked regardless of shared infrastructure. | B02 |
| Maintainability | Declared graph and actual-dependency comparison make architecture drift visible. | A03-002 |
| Testability | Dependency rules are deterministically verifiable before deployment. | A03-002, F02 |
| Delivery | F01 can consume A03 conformance as a pipeline gate without owning the rules. | F01 |
| Operability | Dynamic composition failures surface at the runtime guard only when they could not be determined earlier. | A03-002, E05 where telemetry is required |
| Portability | Architecture is independent of source language/build/runtime product. | A03-002 / future A02 standards |

---

## 8. Architecture Conformance Requirements

| CR ID | Conformance Requirement | Evidence Expected |
|---|---|---|
| `CR-001` | Every governed Technical Module shall have one valid Technical Composition Declaration bound to the A03-001 Technical Module identity/parent model. | Declaration and A03-001 mapping validation. |
| `CR-002` | The declaration shall identify direct technical dependencies sufficiently for deterministic graph validation. | Machine-readable dependency declaration. |
| `CR-003` | Cross-parent technical dependencies shall not target another parent's internal implementation boundary. | Graph/public-boundary validation. |
| `CR-004` | Applicable repository dependency direction and canonical Module Dependency metadata shall not be contradicted by the technical graph. | Dependency-alignment validation. |
| `CR-005` | Prohibited direct/indirect dependency cycles shall fail the architecture gate. | Graph-cycle validation evidence. |
| `CR-006` | Runtime contract/event/read-model consumption shall not be automatically treated as a foundational architectural dependency. | Dependency classification evidence. |
| `CR-007` | Actual implementation dependencies shall be checked against the declared allowed technical graph during CI/build or equivalent pre-deployment validation. | Declared-versus-actual comparison evidence. |
| `CR-008` | A material undeclared/unauthorized dependency shall be a hard failure unless exactly covered by a valid waivable exception. | Gate result and optional waiver reference. |
| `CR-009` | Dynamic composition capable of creating dependencies not knowable earlier shall be guarded at runtime against the effective declaration. | Runtime composition validation evidence. |
| `CR-010` | A waiver shall identify exact scope, rule, owner, rationale, risk, approval, expiry/remediation condition and affected declaration scope. | Waiver record. |
| `CR-011` | Expired waiver shall no longer suppress the underlying architecture violation. | Expiry/revalidation evidence. |
| `CR-012` | No waiver shall authorize canonical ownership transfer, prohibited canonical dependency cycles, cross-owner persistence writes, Tenant isolation bypass or authoritative security/trust violation. | Waiver admission negative tests/review. |
| `CR-013` | Shared runtime/package/deployment placement shall not by itself authorize a technical dependency or peer-internal access. | Composition validation evidence. |
| `CR-014` | Build/package/runtime technology shall not become the source of architecture truth merely by exposing observed dependencies. | Architecture declaration remains authoritative and comparison is one-way conformance. |
| `CR-015` | Enforcement shall provide attributable evidence identifying the evaluated declaration, graph outcome, violations and any waiver affecting the result. | Conformance evidence output. |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Owner | Blocking Status | Resolution Path |
|---|---|---|---|---|
| `OD-001` | Formal ADR scope for the A03 module/dependency enforcement standard and its relationship to the unresolved A01/A03-001 decomposition decision. | L5-A-03 / Lead Architect | **Blocks Architecture Approved for AD-001–AD-003.** | Determine whether one Shared/Family ADR governs the A01/A03 module pattern or whether enforcement needs a distinct ADR; then allocate only through the canonical ADR Register. |
| `OD-002` | Concrete serialization/schema for the Technical Composition Declaration. | L5-A-03 with A02/F01 implementation standards | Does not block product-neutral Draft; blocks implementation standardization. | Resolve after applicable runtime/build/toolchain decisions are governed. |
| `OD-003` | Concrete implementation-dependency extraction/analysis mechanisms for each approved language/build/runtime stack. | A02/F01 with A03 conformance | Does not block architecture Draft. | Define in technology/engineering standards after stack ADRs exist. |
| `OD-004` | Exact repository location/lifecycle mechanism for technical Architecture Waiver records. | L5 architecture governance | Does not block Draft; required before production enforcement. | Governance placement/approval decision without creating a competing architecture register. |

No new canonical ADR identifier is allocated by this specification. The current ADR Register on `main` contains ADR-L5-0001 and ADR-L5-0002 for D03 concerns; neither governs A03.

---

## 10. Traceability Summary

### Canonical Responsibility

`L5-A-03-002` realizes the frozen corpus responsibility to make technical dependency/composition rules enforceable and prevent unauthorized coupling.

### Material Upstream Contribution

| L4 Behavior | A03-002 Technical Contribution | Evidence |
|---|---|---|
| `L4-A-032-BH-003` | Enforces technical dependency compatibility/alignment inputs without inventing Module Registry decisions. | §§5.2–5.6; `CR-001`–`CR-004` |
| `L4-A-032-BH-004` | Detects and blocks prohibited dependency cycles at technical realization layers. | §§5.3–5.6; `CR-005` |
| `L4-A-032-BH-005` | Ensures failed dependency validation cannot silently become valid implementation composition. | §§5.5–5.9; `CR-007`–`CR-008` |
| `L4-A-032-BH-006` | Keeps technical composition subordinate to governed dependency approval rather than allowing implementation to create canonical dependency authority. | §§5.2–5.5; `CR-004`, `CR-014` |

These relations remain Draft/Specified architecture evidence only; this file does not self-declare repository-level L4→L5 relations `Validated` while the cited L4 source remains Draft.

### Enforcement Model

```text
A03-001 boundary rules
        ↓
Technical Composition Declaration
        ↓
repository validation
        ↓
CI/build implementation comparison
        ↓
package/composition validation
        ↓
conditional runtime guard
        ↓
conformance evidence

Violation
  ↓
non-waivable? → block
  ↓ no
valid unexpired technical waiver? → temporary allow
  ↓ no
block
```

This specification remains `Draft` while `OD-001` is unresolved and shall not be treated as an Architecture Approved repository-wide module-enforcement standard until the required ADR governance is completed.