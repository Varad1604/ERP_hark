# Component, Unit & Domain Verification Architecture

## 1. Specification Metadata

**Specification ID:** `L5-F-02-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-F — Engineering Delivery & Quality Architecture  
**Major Area:** L5-F-02 — Verification & Testing Architecture  
**Canonical Technical Responsibility:** Lower-scope verification of technical components/domain implementations against architecture contracts.  
**Technical Architecture Owner:** L5-F-02 — Verification & Testing Architecture  
**Assigned Author:** Chetan  
**Version:** `1.0.2`  
**Status:** `Review`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-F-02-001_Verification_and_Testing_Architecture_Baseline.md`; L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; `L5-A-03-001` current Review; `L5-A-03-002` current Review; `L5-B-02-003` current Review; `L5-A-04-002` current Review; applicable current L5 Architecture Conformance Requirements and L4 Behavioral Obligations routed by F02-001.  
**Referenced By:** `L5-F-02-001`; `L5-F-01-*`; architecture-review and implementation-verification consumers.  
**Applicable ADRs:** No ADR is allocated specifically to F02-002. Verification obligations may target architecture governed by applicable approved ADRs such as `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0006`, and `ADR-L5-0007`; F02-002 does not reinterpret those decisions.  

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the lower-scope verification architecture used to demonstrate that SutraERP implementation units, technical components and owner-local/domain implementations conform to their exact L5 Architecture Conformance Requirements and applicable L4 Behavioral Obligations.

It realizes the `L5-F-02-001` common obligation/result/evidence model for verification that can be established without requiring a live cross-boundary integration path or a specialized security, performance, resilience or recovery test.

### In Scope

- verification-target scoping for implementation units and technical components;
- Technical Module / owner-local boundary verification;
- component and unit verification against exact L5 CRs;
- owner-local/domain behavior verification against routed L4 BHs;
- invariant, validation and negative-path verification inside one canonical owner boundary;
- local transaction/concurrency verification where required by B02-owned architecture;
- local publication-intent verification up to the B02→C02 handoff boundary;
- Technical Composition Declaration and declared-versus-actual dependency verification;
- controlled collaborator substitution for lower-scope verification;
- test fixture/data-state control without acquiring source-data ownership;
- repeatability, isolation and attributable evidence;
- result handoff into the F02-001 common VerificationResult model.

### Out of Scope / Non-Goals

This specification does not own:

- real synchronous contract compatibility or service-to-service verification — `L5-F-02-003`;
- actual event routing/delivery/subscription verification — `L5-F-02-003`;
- connector/provider/external-system verification — `L5-F-02-003`;
- full end-to-end business-path verification — `L5-F-02-003`;
- security-control effectiveness, adversarial security verification or privileged-access testing mechanics — `L5-F-02-004`;
- load, throughput, capacity or performance testing — `L5-F-02-004`;
- fault-injection/resilience/DR/recovery validation mechanics — `L5-F-02-004`;
- upstream business rules, states, events, publishers or authority;
- A03 module/dependency rules, B02 transaction semantics, A04 isolation semantics or peer architecture ownership;
- testing framework, language, runner, database, container, CI/CD or other implementation-product selection.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Version / Status | Exact Reference | What It Establishes | F02-002 Use |
|---|---|---|---|---|
| L5 Candidate Document Corpus Register 105 | v1.1.0 / Architecture Approved | `L5-F-02-002` | Lower-scope verification of technical components/domain implementations against architecture contracts. | Freezes canonical responsibility. |
| `L5-F-02-001` | Stage-6 closed / Review | common Verification Obligation, Result, Evidence, Scope and Gate semantics | F02-002 is one specialization supplying lower-scope results into the common baseline. | Parent verification contract. |
| `L5-A-03-001` | current / Review | `CR-001`–`CR-010` | Technical Module ownership/boundaries, one parent, public-boundary and persistence restrictions. | Provides structural/component verification targets. |
| `L5-A-03-002` | current / Review | §§5.1–5.7; `CR-001`–`CR-012` | Technical Composition Declaration, dependency drift detection, fail-fast enforcement and attributable evidence. | Provides composition/dependency verification targets. |
| `L5-B-02-003` | current / Review | §§5.1–5.5; `CR-001`–`CR-011` | Owner-local transaction, concurrency conflict and state-to-publication-intent semantics. | Provides owner-local consistency verification targets. |
| `L5-A-04-002` | current / Review | §§5.1–5.8; `CR-001`–`CR-010` | Validated Tenant/Institution scope propagation and fail-closed isolation rules. | Provides lower-scope isolation verification targets when applicable. |
| L5 Validation Standard | v1.0.1 / Locked | objective CR verification and hard-fail integrity | Verification claims must be attributable, implementable and non-invented. | Governs result/evidence integrity. |
| L5 L4-to-L5 Traceability Methodology | v1.0.0 / Locked | version-bound BH/L5 evidence | Lower-scope behavior verification cannot silently create canonical relation validation. | Governs traceability status. |

A03/B02/A04 are Architecture Approved after R6. F02-002 may verify their explicit CRs for governed implementation evidence but does not promote or lock those peer specifications.

### 3.2 Verification-Level Definitions

For F02-002:

**Implementation Unit**  
The smallest implementation scope intentionally verified in isolation. It is not a new canonical architecture entity and may vary by implementation technology.

**Technical Component**  
A coherent technical realization element with an architecture responsibility or contract that can be verified independently enough to produce attributable evidence.

**Owner-Local / Domain Implementation Scope**  
The implementation of behavior inside one canonical owner/module boundary, including its invariants and owner-local persistence semantics, without claiming proof of actual cross-owner integration.

These terms define verification scope only. They do not change L1/L2/L3/L4 or A03 ownership.

### 3.3 Authoritative Constraints

| ID | Constraint | Effect |
|---|---|---|
| `C-001` | Every F02-002 result uses an F02-001 Verification Obligation bound to an exact BH/CR and exact target/version/scope. | No generic "unit tests passed" conformance claim. |
| `C-002` | Lower-scope verification may isolate collaborators, but isolated collaborator behavior does not prove real cross-boundary integration. | Prevents overlap with F02-003. |
| `C-003` | F02-002 may verify business-rule implementation only against exact upstream rule/BH authority; it shall not derive or rewrite business meaning. | Preserves ownership. |
| `C-004` | Owner-local verification shall not widen the transaction boundary beyond the canonical owner. | Preserves B02 semantics. |
| `C-005` | Local publication verification ends at creation/commit of the B02-governed publication handoff where applicable; actual C02 delivery belongs to F02-003. | Preserves B02/C02 boundary. |
| `C-006` | Technical composition checks verify A03 declarations/rules; F02-002 does not become owner of A03 rule definitions. | Preserves A03 ownership. |
| `C-007` | Tenant/Institution verification uses A04 semantics and does not invent a default/global Tenant. | Preserves isolation architecture. |
| `C-008` | No fixed code-coverage percentage, test count, mutation score, execution duration or similar metric is invented. | Avoids ungoverned numeric targets. |
| `C-009` | A controlled substitute cannot strengthen evidence beyond the boundary it actually exercises. | Evidence scope remains honest. |
| `C-010` | A lower-scope verifier/harness failure maps to `Verification Not Established` unless target evidence independently proves non-conformance. | Aligns with F02-001. |

### 3.4 Assumptions

No testing framework, mock/stub technology, language, package system, in-memory database, container runtime, test runner or CI product is assumed.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

F02-002 owns:

- lower-scope Verification Target selection and boundary declaration;
- component/unit verification execution semantics;
- owner-local/domain implementation verification semantics;
- controlled collaborator-substitution semantics;
- local fixture/state arrangement semantics;
- local assertion/observation semantics;
- lower-scope structural/dependency verification;
- owner-local invariant/transaction/concurrency verification;
- evidence production sufficient for F02-001 normalization.

### Consumes / Collaborates With

| Architecture | Supplies / Owns | F02-002 Use |
|---|---|---|
| F02-001 | common obligation/result/evidence/gate semantics | Receives obligation and returns specialized result/evidence. |
| A03-001 | Technical Module/boundary rules | Structural and owner-boundary verification targets. |
| A03-002 | composition/dependency declaration and enforcement rules | Declared-versus-actual dependency and waiver evidence verification. |
| B02 | persistence, transaction and concurrency semantics | Owner-local persistence verification target; F02-002 does not design the store. |
| A04 | Tenant/Institution isolation semantics | Scope/isolation verification where the target participates in protected execution. |
| D02/D03/B05 | authorization/trust/privacy controls | Constrain verifier execution and test/evidence data; security effectiveness testing remains F02-004. |
| D04 | evidence provenance/integrity | Receives or stores governed evidence where required. |
| F01 | build/release pipeline | May execute/consume lower-scope verification; does not own test meaning. |

### Must Not Own

- actual API/contract/event/connector interoperability;
- cross-owner orchestration correctness;
- security/performance/resilience/recovery testing;
- A03/B02/A04 rule definitions;
- business/domain ownership;
- release eligibility;
- physical persistence/test-runtime product choices.

---

## 5. Solution Architecture

### 5.1 Lower-Scope Verification Flow

```text
F02-001 Verification Obligation
        ↓
Lower-Scope Applicability Check
        ↓
Verification Target Boundary
        ↓
Fixture / Local Context Arrangement
        ↓
Controlled Collaborator Boundary
        ↓
Execute Target
        ↓
Observe local outputs / state / invariants / handoff intent
        ↓
Evaluate exact BH / CR
        ↓
Produce specialized result + evidence
        ↓
F02-001 Result Normalizer / Evidence Binder
```

If the obligation cannot be demonstrated without exercising a real cross-boundary contract/event/connector, the obligation or relevant portion is routed to F02-003 rather than simulated and overclaimed.

### 5.2 Verification Target Descriptor

Conceptually:

```text
LowerScopeVerificationTarget
{
  technical target identity/version
  canonical parent / owner reference where applicable
  technical-module/component boundary reference
  exact obligation reference
  local dependency set
  persistence participation, if applicable
  Tenant/Institution scope requirement, if applicable
}
```

This is a conceptual architecture contract, not a prescribed test-manifest schema.

### 5.3 Component Verification

Component verification demonstrates that a coherent technical component:

- satisfies its exact L5 CRs;
- preserves its owned boundary;
- rejects invalid local inputs where the governing architecture requires rejection;
- produces the required local technical outcome;
- does not invoke undeclared peer-private dependencies;
- preserves required scope/ownership metadata;
- produces attributable local evidence.

Component verification shall not treat an isolated peer substitute as proof of the peer's implementation.

### 5.4 Unit Verification

Unit verification targets the smallest intentional implementation scope needed to verify deterministic local behavior.

Appropriate unit-level obligations may include:

- transformation/decision logic that is already authoritatively defined upstream;
- invariant checks;
- validation logic;
- local state progression within the owner boundary;
- local failure classification;
- deterministic mapping of defined input/context to defined output.

Unit scope is implementation-specific and is not added to the canonical architecture corpus.

A unit result contributes to architecture conformance only when it is explicitly bound to an F02-001 obligation and target version.

### 5.5 Owner-Local / Domain Implementation Verification

Owner-local verification demonstrates behavior that belongs inside one canonical owner/module boundary.

Typical architecture concerns include:

- authoritative invariant enforcement;
- required validation before owner-local mutation;
- exact owner-local state result;
- prevention of prohibited mutation;
- persistence consistency inside the owner boundary;
- concurrency conflict behavior where B02 requires it;
- creation/commit of required Durable Publication Intent where applicable;
- preservation of canonical event ownership without claiming transport delivery.

The verification scope stops before C02 delivery or peer-domain execution.

### 5.6 Controlled Collaborator Substitution

A lower-scope verification may use a **Controlled Collaborator Substitute** to supply defined peer outcomes without executing the actual peer.

The substitute shall be bounded by the authoritative peer/public contract context needed for the local obligation.

It may be used to verify:

- how the target reacts to an allowed peer result;
- how the target reacts to a defined peer rejection/failure outcome;
- local branching/invariant behavior dependent on peer response categories.

It shall not be used to claim:

- real C01 contract compatibility;
- real C02 event routing/delivery;
- real connector/provider behavior;
- real peer authorization/security behavior;
- end-to-end completion.

Those claims require F02-003 and/or F02-004 evidence.

### 5.7 Fixture and Test-State Control

Test state shall be deliberately established and attributable to the target obligation.

Fixture architecture shall:

- use explicit Tenant/Institution scope when the target requires it;
- preserve canonical owner boundaries;
- avoid undeclared global/default Tenant context;
- avoid production-data assumptions;
- use B05-governed handling when sensitive/source-derived data is required;
- avoid direct mutation of peer canonical stores to make a local test pass;
- distinguish setup data from evidence of target behavior.

F02-002 does not prescribe fixture serialization, factories or data-generation products.

### 5.8 A03 Structural / Dependency Verification

For A03-governed targets, F02-002 may verify:

1. every Technical Module has the required single canonical parent binding;
2. Technical Module composition remains traceable to applicable Module Version context;
3. cross-parent dependencies target governed public boundaries;
4. cross-owner direct persistence is absent;
5. declared and actual implementation dependencies do not materially diverge;
6. prohibited cycles/direction contradictions are rejected;
7. applicable waivers are explicit, valid and unexpired;
8. evidence identifies declaration, graph outcome, violations and waiver state.

A03 remains owner of these rules. F02-002 only verifies their implementation.

### 5.9 Owner-Local Transaction & Concurrency Verification

Where an obligation covers B02-003:

```text
arrange owner-local authoritative state/version
        ↓
execute target mutation
        ↓
observe commit/conflict outcome
        ↓
verify:
- no silent lost update
- owner boundary preserved
- stronger locking only where architecture permits/requires
- state + publication intent atomicity where applicable
```

Cross-owner distributed ACID is not created for test convenience.

If the target produces a Durable Publication Intent, F02-002 may verify its owner-local creation/atomic commit. Actual downstream delivery/retry/consumer behavior is F02-003.

### 5.10 Tenant / Institution Isolation Verification

Where A04 applies, lower-scope verification shall exercise at least the applicable architecture conditions defined by the obligation, such as:

- valid Tenant-rooted context;
- compatible subordinate Institution/lower scope;
- missing scope;
- incompatible scope;
- unexpected widening/switching;
- absence of default/global Tenant fallback.

The test mechanics shall not invent new A04 states or scope rules.

Cross-boundary propagation through actual messaging/API/workflow paths belongs to F02-003; F02-002 verifies local boundary behavior and fail-closed reaction.

### 5.11 Negative-Path Verification

For every obligation whose architecture defines rejection/failure behavior, F02-002 shall include evidence for the relevant invalid condition instead of proving only the successful path.

Examples are source-driven, such as:

- invalid module composition;
- invariant violation;
- concurrency conflict;
- missing protected scope;
- prohibited local mutation.

This specification does not create a universal catalogue of business failure cases.

### 5.12 Repeatability and Isolation

A lower-scope result is reusable as conformance evidence only when the execution is attributable and its target/fixture/dependency context is sufficiently controlled to reproduce the architectural observation.

Repeatability does not require identical timestamps, identifiers or physical runtime details unless the obligation makes them material.

Test order shall not create hidden target state that changes the architectural result.

### 5.13 Result and Evidence Handoff

F02-002 returns to F02-001:

```text
LowerScopeVerificationResult
{
  obligation reference
  target/version/scope reference
  verification level: unit | component | owner-local/domain
  collaborator-boundary classification
  execution reference
  observation/assertion evidence references
  conformance outcome
  unresolved / excluded cross-boundary evidence references
}
```

The classification labels are technical verification metadata, not canonical business states.

---

## 6. Architecture & Technology Decisions

| ID | Decision | Position | Basis |
|---|---|---|---|
| `AD-001` | Scope | F02-002 verifies implementation inside a declared lower-scope/component/owner-local boundary. | Candidate corpus + F02-001 |
| `AD-002` | Contract binding | Every lower-scope result is bound to an exact BH/CR and exact target/version/scope. | F02-001 + traceability methodology |
| `AD-003` | Substitution | Controlled collaborator substitution may verify local reactions but cannot prove real integration. | Ownership split with F02-003 |
| `AD-004` | Domain ownership | Verification of business/domain behavior consumes upstream meaning and never redefines it. | L5 governance |
| `AD-005` | Structural verification | A03 technical composition/dependency rules are lower-scope verification targets; A03 remains rule owner. | A03-001/002 |
| `AD-006` | Local consistency | B02 owner-local transaction/concurrency/publication-intent semantics may be verified here; C02 delivery is excluded. | B02-003 |
| `AD-007` | Isolation | A04 local fail-closed/scope behavior may be verified here; actual cross-boundary propagation belongs to F02-003. | A04-002 |
| `AD-008` | Negative evidence | Where source architecture defines a rejection/failure condition, lower-scope conformance includes verification of that condition. | Objective verification requirement |
| `AD-009` | No numeric coverage invention | F02-002 does not establish universal code-coverage/test-count thresholds. | Zero-guessing / technology governance |
| `AD-010` | Technology neutrality | No test framework, runner, language, fixture tool or persistence product is selected. | L5 technology governance |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | F02-002 Position |
|---|---|
| Authorization | Test execution/evidence access consumes D02; F02-002 does not test security-control effectiveness unless routed to F02-004. |
| Privileged access | Privileged test actions require canonical safeguards and evidence. |
| Tenant isolation | Test context never uses implicit/default Tenant authorization. |
| Sensitive data | Test fixtures/evidence consume B05 controls; credentials/secrets are not ordinary evidence. |
| Persistence | Test execution shall not bypass B02 owner boundaries merely for convenience. |
| Test contamination | Setup/cleanup shall not modify peer canonical state outside governed boundaries. |
| Evidence quality | Evidence identifies exact obligation, target, scope, execution and observation. |
| Product neutrality | No testing tool becomes canonical architecture through this specification. |

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Every F02-002 verification execution used for conformance shall reference an exact F02-001 Verification Obligation and exact target/version/scope. |
| `CR-002` | Lower-scope verification shall declare whether its target is an implementation unit, technical component or owner-local/domain implementation scope. |
| `CR-003` | A controlled collaborator substitute shall not be represented as evidence of actual peer integration or interoperability. |
| `CR-004` | F02-002 shall not redefine upstream business rules, lifecycle semantics, canonical events, publishers or architecture CR meaning. |
| `CR-005` | Component verification shall preserve canonical A03 owner/module boundaries and shall not validate peer-private cross-parent coupling as conformant. |
| `CR-006` | Where A03 composition is in scope, declared-versus-actual dependency evidence and applicable waiver state shall be attributable. |
| `CR-007` | Owner-local persistence verification shall preserve the B02 transaction boundary and shall not introduce cross-owner distributed ACID. |
| `CR-008` | Where publication intent is in scope, F02-002 may verify owner-local atomic state-to-intent behavior but shall not claim C02 delivery verification. |
| `CR-009` | Where A04 scope is in scope, missing/incompatible/widened/switched scope shall be verified only according to canonical A04 semantics and shall not fall back to a default/global Tenant. |
| `CR-010` | Invalid/rejection paths defined by the governing architecture shall not be omitted when they are material to the exact obligation. |
| `CR-011` | Test fixture/setup state shall not acquire canonical ownership or mutate peer authoritative state outside governed boundaries. |
| `CR-012` | Sensitive/source-derived test data and verification evidence shall consume applicable B05/D04 controls. |
| `CR-013` | Harness/substitute/setup failure shall not be reported as target non-conformance unless attributable target evidence establishes that outcome. |
| `CR-014` | F02-002 results shall map into the F02-001 common result/evidence semantics. |
| `CR-015` | F02-002 shall not define a universal numeric code-coverage, mutation-score, pass-rate, test-count or execution-time threshold. |
| `CR-016` | F02-002 shall remain independent of a specific test framework, runner, programming language, database or CI/CD product unless separately governed. |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Classification | Blocking? | Required Resolution |
|---|---|---|---|---|
| `OD-001` | **Resolved:** the four-file F02 family is canonical on `main` and Stage-6 is closed for downstream consumption. | Repository synchronization | Closed | Maintain impact review. |
| `OD-002` | A03-001/002, B02-003 and A04-002 are Review peers and remain blocked from Architecture Approved by upstream authority. | Peer maturity | Non-blocking for verification; relevant to lifecycle-promotion evidence | Preserve their current governed status. |
| `OD-003` | Exact implementation-unit granularity is implementation-specific. | Implementation | No | Each target declares its own lower-scope boundary without creating a new canonical architecture level. |
| `OD-004` | Exact test framework, runner, fixture representation and automation technology are unselected. | Technology | No | Select only through governed technology standards if made canonical. |
| `OD-005` | No repository-wide numeric lower-scope coverage threshold is established by authoritative architecture. | Quality governance | No | Any mandatory numeric target requires explicit governed authority; do not invent locally. |

---

## 10. Traceability Summary

### Canonical Responsibility

`L5-F-02-002` — **Lower-scope verification of technical components/domain implementations against architecture contracts.**

### Primary Parent

`L5-F-02-001` supplies:

- Verification Obligation;
- exact requirement/target binding;
- common Verification Result;
- evidence binding/currentness;
- technical gate semantics.

F02-002 supplies lower-scope execution/evidence back to that baseline.

### Selective L5 Verification Applicability

| Target Architecture | Exact CR Examples | F02-002 Contribution |
|---|---|---|
| `L5-A-03-001` | `CR-001`, `CR-003`, `CR-004`, `CR-007`, `CR-008`, `CR-009` | Technical Module parent/version/boundary/dependency/persistence/placement verification. |
| `L5-A-03-002` | `CR-001`–`CR-012` as applicable | Declaration validity, dependency drift, cycle/direction, waiver and evidence verification. |
| `L5-B-02-003` | `CR-001`, `CR-003`, `CR-005`, `CR-006`, `CR-010` | Owner-local transaction, conflict, atomic publication-intent and no cross-owner persistence verification. |
| `L5-A-04-002` | `CR-001`, `CR-002`, `CR-006`, `CR-007`, `CR-008` | Local Tenant/Institution scope and fail-closed isolation behavior verification. |

These are selective Draft-stage mappings and do not self-declare canonical relation state or peer approval.

### Boundary to F02-003

F02-002 stops where proof requires the actual:

- C01 service contract boundary;
- C02 event transport/subscription/delivery path;
- external connector/provider;
- cross-owner orchestration path;
- end-to-end multi-component outcome.

Those are F02-003 verification responsibilities.

### Boundary to F02-004

F02-002 does not own specialized verification of:

- security-control effectiveness;
- performance/capacity;
- resilience/fault isolation;
- backup/restore/disaster recovery.

Those are F02-004 responsibilities.

### Architecture Decisions

`AD-001`–`AD-010`

### Conformance Requirements

`CR-001`–`CR-016`

### Current Status

**Review — Stage-6 F02 child specification; downstream consumption approved.**

F02-002 is reconciled with F02-001/003/004 on `main`. Stage-6 downstream consumption is approved; Architecture Approved/lock status remains separate governance.
