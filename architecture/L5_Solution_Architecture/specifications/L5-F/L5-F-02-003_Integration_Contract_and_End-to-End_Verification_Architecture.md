# Integration, Contract & End-to-End Verification Architecture

## 1. Specification Metadata

**Specification ID:** `L5-F-02-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-F — Engineering Delivery & Quality Architecture  
**Major Area:** L5-F-02 — Verification & Testing Architecture  
**Canonical Technical Responsibility:** Contract/event/connector/end-to-end verification across technical boundaries.  
**Technical Architecture Owner:** L5-F-02 — Verification & Testing Architecture  
**Assigned Author:** Chetan  
**Version:** `1.1.0`  
**Status:** `Locked`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-F-02-001_Verification_and_Testing_Architecture_Baseline.md`; `L5-C-01-002_Synchronous_Service_Contract_Architecture.md`; `L5-C-01-003_Contract_Versioning_Compatibility_Error_and_Idempotency_Architecture.md`; `L5-C-02-001_Event_and_Messaging_Architecture_Baseline.md`; `L5-C-02-002_Durable_Event_Delivery_Ordering_and_Failure_Handling_Architecture.md`; `L5-C-02-003_Event_Schema_Versioning_and_Consumer_Compatibility_Architecture.md`; `L5-C-03-001_External_Integration_and_Connector_Architecture_Baseline.md`; `L5-C-03-002_Connector_Lifecycle_and_Failure_Isolation_Architecture.md`; `L5-A-04-002_Tenant_and_Institution_Context_Propagation_and_Data_Isolation_Architecture.md`; `L5-A-02-001_Technology_Platform_and_Standards_Architecture.md`; L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; current applicable L4 Behavioral Obligations and L5 Architecture Conformance Requirements.  
**Referenced By:** `L5-F-02-001`; `L5-F-01-*`; cross-boundary architecture-review/release evidence consumers.  
**Applicable ADRs:** F02-003 consumes the applicable locked profiles including `ADR-L5-0021`, `ADR-L5-0022`, `ADR-L5-0034`, `ADR-L5-0035`, plus the existing contract/event/integration ADRs; it does not reinterpret or own them.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines how SutraERP proves technical conformance across actual architectural boundaries rather than only inside isolated components.

It realizes the `L5-F-02-001` common verification model for:

- synchronous service contracts;
- contract-version compatibility and idempotency;
- event publication, delivery, ordering, retry/replay and consumer compatibility;
- external connector/provider integration;
- multi-boundary end-to-end technical paths.

F02-003 verifies that independently owned technical boundaries interoperate correctly while preserving canonical ownership, Tenant/Institution scope, contract meaning, event publisher authority and source-domain business responsibility.

### In Scope

- real producer/consumer or caller/provider contract verification;
- request/response admission/outcome/correlation verification;
- compatible/incompatible contract-version verification;
- operation idempotency and duplicate-request boundary verification;
- event publisher-to-transport-to-consumer verification;
- durable event delivery/redelivery/acknowledgement/replay verification;
- event-contract version compatibility across producers/consumers;
- connector/adaptor/provider-boundary verification;
- inbound callback/exchange correlation verification;
- ambiguous external-outcome evidence verification;
- multi-component technical end-to-end verification across governed boundaries;
- preservation of Tenant/Institution and identity/authorization context across handoffs;
- distributed evidence correlation and completion proof;
- result handoff to the F02-001 common result/evidence model.

### Out of Scope / Non-Goals

F02-003 does not own:

- unit/component/owner-local verification mechanics — `L5-F-02-002`;
- security-control effectiveness/adversarial testing — `L5-F-02-004`;
- performance/capacity testing — `L5-F-02-004`;
- resilience/fault-injection/DR/recovery validation mechanics — `L5-F-02-004`;
- C01 contract semantics;
- C02 messaging semantics;
- C03 connector lifecycle/provider adaptation semantics;
- A04 Tenant/Institution isolation semantics;
- business outcome/rule/lifecycle/event/publisher ownership;
- release approval;
- protocol/broker/gateway/provider/test-tool selection;
- L6 user-journey/UI testing architecture.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Status | Exact Responsibility | F02-003 Use |
|---|---|---|---|
| Candidate Document Corpus Register 105 | Architecture Approved | `L5-F-02-003` = contract/event/connector/end-to-end verification across technical boundaries | Freezes canonical responsibility. |
| `L5-F-02-001` | Stage-6 closed / Review baseline | common Verification Obligation, Result, Evidence, Currentness and Gate semantics | Parent verification contract. |
| `L5-C-01-002` | current / Architecture Approved | authoritative synchronous request/response execution, unknown outcome, overload, correlation | Synchronous boundary verification target. |
| `L5-C-01-003` | current / Architecture Approved | compatibility, version coexistence, technical errors and idempotency | Contract evolution/interoperability verification target. |
| `L5-C-02-001` | Architecture Approved v1.0.3 | event publication/routing/subscription/delivery baseline | Event-path verification target. |
| `L5-C-02-002` | Architecture Approved v1.0.3 | durable delivery, retry/redelivery, ordering, replay, failure isolation | Delivery/recovery-path verification target. |
| `L5-C-02-003` | Architecture Approved v1.0.3 | event schema/version/consumer compatibility | Producer/consumer compatibility verification target. |
| `L5-C-03-001` | Locked v1.0.2 | common connector/adaptor/provider integration boundary | External integration verification target. |
| `L5-C-03-002` | Locked v1.0.2 | connector technical configuration, health and failure isolation | Connector readiness/failure-boundary verification target. |
| `L5-A-04-002` | current / Architecture Approved | Tenant/Institution scope propagation/revalidation | Scope-continuity verification target across real handoffs. |
| `L5-A-02-001` | Architecture Approved v1.0.0 | approved technology standards, including Rust primary server platform | Implementation targets must conform to approved platform standards; F02-003 still owns no testing framework. |

### 3.2 Authoritative Constraints

| ID | Constraint | Effect |
|---|---|---|
| `C-001` | Every F02-003 verification uses an exact F02-001 obligation bound to exact source requirement and technical target versions. | No generic “integration passed” claim. |
| `C-002` | A real boundary must be exercised where the evidence claims interoperability. | Substitutes cannot prove actual integration. |
| `C-003` | Cross-owner verification shall not combine owners into one transaction or persistence boundary. | Preserves B02/C01/C02 ownership. |
| `C-004` | Synchronous timeout/disconnect is not proof of non-execution. | Unknown outcomes must remain explicit. |
| `C-005` | Event verification shall preserve canonical event publisher/meaning and C02 delivery semantics. | F02-003 cannot synthesize publisher authority. |
| `C-006` | Event delivery may be duplicate/at-least-once-compatible; tests shall not assume platform-wide exactly-once. | Aligns with C02. |
| `C-007` | Ordering is verified only for declared scopes; no global order assumption. | Aligns with C02-002. |
| `C-008` | Connector/provider acknowledgements or callbacks are technical evidence unless authoritative business architecture defines them as business outcomes. | Prevents provider ownership transfer. |
| `C-009` | End-to-end verification shall prove only the exact declared path and obligations included in scope. | No universal “system works” claim. |
| `C-010` | Security/performance/resilience/recovery evidence requiring specialized methods is delegated to F02-004. | Prevents scope overlap. |
| `C-011` | A02 implementation standards apply to implementation targets, but F02-003 shall not infer a testing framework/runtime product from A02. | Preserves A02/F02 ownership. |
| `C-012` | No numeric timeout, retry count, latency, throughput or coverage target is invented. | Zero guessing. |

### 3.3 Verification Scope Classes

F02-003 uses four technical verification classes:

1. **Contract Verification**  
   Proves an actual caller/provider or producer/consumer boundary respects the released technical contract.

2. **Integration Verification**  
   Proves two or more independently owned technical elements collaborate through the approved boundary without bypassing ownership.

3. **Connector Verification**  
   Proves the C03 connector/adaptor/provider boundary performs the governed technical exchange and preserves correlation/evidence.

4. **End-to-End Technical Verification**  
   Proves a declared chain of real technical boundaries from entry condition to observable completion/evidence without requiring that one component own the entire chain.

These are testing-scope classifications, not new architecture levels or business lifecycles.

### 3.4 Assumptions

No contract-test framework, API runner, event-test harness, broker, gateway, service-virtualization platform, container technology or provider sandbox is assumed unless already governed by its canonical owner.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

F02-003 owns:

- cross-boundary Verification Target topology;
- real-boundary execution semantics;
- contract interoperability verification;
- producer/consumer compatibility verification;
- event-path verification semantics;
- connector/provider verification semantics;
- distributed correlation/evidence assembly;
- end-to-end technical completion evaluation;
- cross-boundary failure classification for verification;
- specialized result production for F02-001.

### Consumes / Collaborates With

| Owner / Spec | Owns | F02-003 Verification Role |
|---|---|---|
| F02-001 | common result/evidence/gate semantics | Receives obligations; normalizes F02-003 results. |
| C01-002 | synchronous execution/outcome semantics | F02-003 verifies actual call-boundary behavior. |
| C01-003 | contract compatibility/error/idempotency | F02-003 verifies producer/consumer compatibility and replay-safe request behavior. |
| C02-001 | event publication/routing/subscriptions | F02-003 verifies the actual event path. |
| C02-002 | durable delivery/retry/order/replay | F02-003 verifies actual delivery/recovery semantics. |
| C02-003 | event schema/version compatibility | F02-003 verifies producer/consumer version interoperability. |
| C03-001/002 | external integration, connector profiles, correlation and failure isolation | F02-003 verifies actual governed provider boundary behavior. |
| A04 | scope propagation | F02-003 verifies scope survives/revalidates across actual boundaries. |
| D01/D02/D03 | identity, authorization, trust/secrets | Used during real integration verification; security-effectiveness tests remain F02-004. |
| D04/E05 | evidence/telemetry | Supplies attributable distributed evidence/diagnostics. |
| F01 | release pipeline | May execute/consume F02-003 verification; F01 retains release authority. |

### Must Not Own

F02-003 shall not:

- redefine contract/event/connector semantics;
- create synthetic business events;
- assign publisher authority;
- mutate peer persistence directly for test convenience;
- claim a provider technical acknowledgement as a business outcome without upstream authority;
- treat an emulated integration as proof of the production integration path;
- define security/performance/resilience/DR testing methods;
- approve architecture or releases.

---

## 5. Solution Architecture

### 5.1 Cross-Boundary Verification Flow

```text
F02-001 Verification Obligation
        ↓
Boundary Topology Resolver
        ↓
Exact Contract / Event / Connector Versions
        ↓
Verification Environment / Real Boundary Admission
        ↓
Execute Cross-Boundary Interaction
        ↓
Collect Distributed Correlation + Evidence
        ↓
Evaluate Each Required Boundary
        ↓
Evaluate Declared End-to-End Completion
        ↓
Specialized F02-003 Result
        ↓
F02-001 Result Normalizer / Evidence Binder
```

### 5.2 Boundary Topology Descriptor

Conceptually:

```text
CrossBoundaryVerificationTarget
{
  obligation reference
  participating technical owner/spec references
  exact target versions
  contract/event/connector contract versions
  Tenant/Institution scope requirement
  correlation/causation basis
  declared entry boundary
  declared completion boundary
  required intermediate evidence points
}
```

This is an architecture model, not a mandatory serialization.

### 5.3 Synchronous Contract Verification

F02-003 shall verify, where applicable:

- released/expected contract version is used;
- structural and required context admission occurs;
- Tenant/Institution scope is preserved/revalidated;
- authorization occurs before protected mutation;
- provider executes within its owner-local boundary;
- success/rejection/conflict/overload/error outcomes match C01-defined categories;
- deadline exhaustion/disconnect does not get misreported as definite non-execution;
- ambiguous outcomes remain unresolved until authoritative reconciliation;
- request/response correlation is attributable.

### 5.4 Contract Compatibility Verification

For C01-003-governed contracts, F02-003 may verify:

- supported predecessor/successor coexistence;
- compatible change acceptance;
- breaking-change rejection or governed successor handling;
- stable technical error-category consumption;
- consumer migration compatibility;
- idempotency identity behavior;
- same idempotency identity with materially different request content is rejected;
- duplicate safe replay where the contract permits it.

F02-003 verifies compatibility behavior; C01-003 remains owner of compatibility rules.

### 5.5 Event Publication & Delivery Verification

For an event-path obligation:

```text
authoritative publisher state commits
        +
B02 Durable Publication Intent
        ↓
C02 publication
        ↓
active event contract/version
        ↓
routing / eligible subscription
        ↓
consumer delivery
        ↓
owned consumer processing
        ↓
acknowledgement
```

F02-003 shall verify only the actual stages required by the obligation.

It shall preserve:

- publisher identity/authority;
- event identity;
- event contract/version;
- Tenant/Institution context;
- correlation/causation where applicable;
- delivery attempt/evidence;
- consumer processing attribution.

### 5.6 Duplicate Delivery & Idempotent Consumer Verification

Because C02 critical delivery is at-least-once-compatible:

- duplicate delivery shall be an expected technical verification case where relevant;
- consumer behavior shall remain idempotent according to its owned semantics;
- duplicate delivery shall not create an ungoverned second business effect;
- acknowledgement shall not occur before the required consumer processing success;
- a repeated delivery does not become a new canonical event.

F02-003 shall not impose platform-wide exactly-once behavior.

### 5.7 Ordering Verification

Where architecture declares an ordering scope, F02-003 shall verify:

- order is preserved within the exact governed scope;
- independent scopes need not be globally serialized;
- timestamps alone are not assumed to establish causality;
- replay/redelivery does not violate the declared ordering contract beyond allowed C02 semantics.

Where no ordering scope is defined, F02-003 shall not invent one.

### 5.8 Event Schema / Version Compatibility Verification

For C02-003-governed event contracts, F02-003 may verify:

- producer emits an explicit governed version;
- consumer declares/supports the tested version;
- additive-compatible evolution remains consumable where allowed;
- unsupported breaking versions fail safely/quarantine;
- predecessor/successor coexistence works where required;
- historical replay preserves original event-contract version;
- registry/catalog validation does not transfer event semantic ownership.

### 5.9 Connector / External Provider Verification

F02-003 verifies C03 technical exchange through the actual governed connector boundary.

Where applicable, verify:

- correct effective connector profile/version is used;
- Tenant/Institution/environment binding is correct;
- credential reference resolves through D03-owned controls;
- provider representation is mapped at the C03 boundary;
- source-domain model is not rewritten around provider shape;
- request/provider-reference/callback correlation is preserved;
- inbound callback/exchange authenticity path is exercised according to governing architecture;
- provider duplicate/replay controls are honored where defined;
- ambiguous provider outcome remains unresolved for owning-domain reconciliation;
- connector evidence does not fabricate business success.

### 5.10 Provider Test Endpoint / Sandbox Boundary

A provider-approved non-production endpoint or governed simulation may be used when it exercises the same declared integration contract sufficiently for the obligation.

The evidence shall identify whether the run used:

- actual production-equivalent provider interface;
- provider sandbox/test endpoint;
- governed technical simulator.

A simulator shall not be represented as evidence of provider behavior it does not implement.

The repository does not mandate one provider-simulation product.

### 5.11 End-to-End Technical Verification

End-to-end verification begins with an exact declared entry condition and ends at an exact declared observable completion/evidence boundary.

Conceptually:

```text
declared entry
→ component/owner A
→ governed contract/event
→ component/owner B
→ optional connector/provider boundary
→ downstream owned processing
→ declared completion/evidence
```

A passing end-to-end result requires:

1. every mandatory boundary in the declared path was exercised;
2. exact contract/event/connector versions were attributable;
3. scope/identity/correlation were preserved as required;
4. each owner produced its own authoritative result;
5. no owner was bypassed through direct persistence/test manipulation;
6. completion evidence matches the exact upstream obligation.

F02-003 does not declare a business outcome beyond the authoritative source.

### 5.12 Partial / Mixed Boundary Execution

If a verification run exercises some real boundaries and substitutes others:

- evidence shall identify which boundaries were real;
- substituted boundaries cannot be counted as real interoperability evidence;
- F02-001 gate coverage records the unresolved real-boundary obligations;
- a partial-path result may demonstrate only the subset actually exercised.

### 5.13 Failure Classification

F02-003 distinguishes:

| Situation | Verification Outcome |
|---|---|
| Target returns an architecture-defined non-conformant result | `Detected Non-Conformance` |
| Contract/version mismatch prevents required interoperability | `Detected Non-Conformance` where attributable to target |
| Test harness/environment unavailable | `Verification Not Established` |
| Provider sandbox unavailable | `Verification Not Established` unless target evidence independently proves non-conformance |
| Synchronous timeout with uncertain provider execution | Preserve unknown technical outcome; do not claim definite target failure without evidence |
| Event delivery duplicate occurs and consumer remains idempotent | May be conformant to C02 semantics |
| Provider technical acknowledgement received but business completion not established | Do not claim business completion |

### 5.14 Distributed Evidence Correlation

A cross-boundary evidence set shall be reconstructable enough to associate:

```text
obligation
+ entry request/event identity
+ Tenant/Institution context where applicable
+ contract/event/connector versions
+ participant owner references
+ correlation/causation/provider references
+ boundary outcomes
+ declared completion evidence
```

F02-003 owns the verification association, while D04/E05 retain their canonical evidence/telemetry responsibilities.

### 5.15 Technology Standard Boundary

A02 currently establishes Rust as the primary server platform and governed exceptions for server-side implementation.

F02-003 shall therefore verify the actual approved implementation target, whether Rust-standard or a governed exception, but:

- shall not select a Rust testing framework;
- shall not require web/mobile clients to use Rust;
- shall not select a broker, API framework, connector gateway or integration-test product;
- shall preserve C01/C02/C03 semantics regardless of implementation technology.

---

## 6. Architecture & Technology Decisions

| ID | Decision | Position | Basis |
|---|---|---|---|
| `AD-001` | Real-boundary evidence | A claim of integration/interoperability requires exercising the real governed boundary represented by the claim. | Frozen F02-003 responsibility |
| `AD-002` | Contract verification | C01 contracts are verified against exact versions, context and outcome semantics; F02 does not redefine them. | C01-002/003 |
| `AD-003` | Event verification | C02 publisher/delivery/ordering/replay/version rules are verified without changing event ownership. | C02-001/002/003 |
| `AD-004` | Delivery semantics | Critical event tests are at-least-once-compatible; duplicates and idempotent consumers are first-class verification cases. | `ADR-L5-0010` |
| `AD-005` | Ordering | Only declared ordering scopes are verified; no global ordering assumption. | C02-002 / `ADR-L5-0010` |
| `AD-006` | Connector verification | C03 connector/provider exchange is verified through the C03 boundary; provider evidence is not automatically business truth. | C03-001/002 / `ADR-L5-0016` |
| `AD-007` | Ambiguous outcomes | Unknown synchronous/provider outcomes remain unknown until authoritative reconciliation. | C01/C03 |
| `AD-008` | End-to-end scope | E2E verification proves only the exact declared chain and completion conditions. | F02-001 + ownership rules |
| `AD-009` | Partial-path honesty | Mixed real/substituted runs disclose boundary coverage and cannot overclaim interoperability. | Evidence integrity |
| `AD-010` | Technology standards | F02-003 consumes A02 implementation standards but selects no testing framework or integration product. | A02 / `ADR-L5-0018` |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Required Position |
|---|---|
| Identity / authorization | Real boundary verification uses canonical D01/D02 paths; test credentials do not grant extra business authority. |
| Trust / secrets | D03 controls credentials, certificates and protected transport; secrets are not verification evidence. |
| Tenant isolation | A04 scope is preserved/revalidated across actual boundaries. |
| Sensitive payloads | Verification data/evidence follows B05/D04 requirements where applicable. |
| Event ownership | Broker/test harness never becomes publisher of business truth unless it is explicitly exercising the canonical publisher implementation. |
| External providers | Provider sandbox/test endpoint usage is explicitly identified in evidence. |
| Telemetry | E05 diagnostics support investigation but do not substitute for target conformance evidence. |
| Release use | F01 may consume results but retains release/promotion authority. |
| Numeric targets | No latency/throughput/retry/timeout/coverage numbers are invented here. |

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Every F02-003 result used for conformance shall identify the exact F02-001 obligation and exact participating target/contract/event/connector versions. |
| `CR-002` | An interoperability claim shall not be based solely on a substituted peer when the claim requires the actual governed boundary. |
| `CR-003` | Synchronous verification shall preserve C01 admission, owner-local execution and outcome semantics. |
| `CR-004` | Timeout/disconnect shall not be treated as proof of non-execution where C01 classifies the outcome as potentially unknown. |
| `CR-005` | Contract-version/idempotency verification shall conform to C01-003 and shall not invent compatibility or replay semantics. |
| `CR-006` | Event-path verification shall preserve canonical publisher, event identity, contract version and consumer attribution. |
| `CR-007` | Critical event verification shall tolerate duplicate delivery according to C02 at-least-once-compatible semantics and shall verify applicable consumer idempotency. |
| `CR-008` | Event acknowledgement shall not be represented as conformant before the C02-defined successful consumer processing boundary where that acknowledgement is required. |
| `CR-009` | Ordering verification shall be limited to explicitly declared ordering scopes; no global ordering shall be assumed. |
| `CR-010` | Event version compatibility verification shall preserve C02-003 version and breaking-change semantics. |
| `CR-011` | Connector/provider verification shall enter through the governed C03 boundary and shall not directly modify another owner's persistence. |
| `CR-012` | Provider-specific payloads shall be validated/mapped at the C03 boundary and shall not become canonical domain-model authority through testing. |
| `CR-013` | Provider acknowledgement/callback evidence shall not be reported as business completion unless authoritative upstream behavior defines that meaning. |
| `CR-014` | Ambiguous external outcomes shall remain unresolved until authoritative reconciliation rather than being guessed by the verifier. |
| `CR-015` | End-to-end verification shall declare exact entry, participating boundaries, completion condition and required evidence. |
| `CR-016` | A partially substituted end-to-end run shall disclose unresolved real-boundary coverage and shall not be reported as complete real-path interoperability. |
| `CR-017` | Cross-boundary verification shall preserve required Tenant/Institution context and shall not use implicit/default Tenant authority. |
| `CR-018` | Distributed evidence shall preserve sufficient correlation to reconstruct the tested interaction path. |
| `CR-019` | F02-003 shall not redefine C01, C02, C03, A04, D-family or source-domain technical/business ownership. |
| `CR-020` | F02-003 shall consume A02 technology standards for implementation targets without selecting an ungoverned testing framework, broker, gateway or provider technology. |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Classification | Blocking? | Required Resolution |
|---|---|---|---|---|
| `OD-001` | **Resolved:** the four-file F02 family is canonical on `main` and Stage-6 is closed for downstream consumption. | Repository synchronization | Closed | Maintain impact review. |
| `OD-002` | C01-002/003 are Architecture Approved after R6; C02 remains Architecture Approved and C03 Locked. | Peer maturity | Non-blocking for verification; relevant to lifecycle-promotion evidence | Preserve governed peer status. |
| `OD-003` | **Resolved dependencies:** HTTPS REST/JSON + OpenAPI 3.2 and NATS JetStream are canonical under the C01/C02 ADRs. | Technology | Closed | Verification targets the locked profiles without owning them. |
| `OD-004` | Exact provider sandbox/simulation capability varies by integration. | External dependency | No | Each C03 specialization/provider profile establishes available governed verification endpoint/evidence. |
| `OD-005` | Exact end-to-end verification environment topology is not repository-wide standardized. | Runtime/quality | No | Preserve E04/A02/F01 boundaries and declare actual environment per verification scope. |
| `OD-006` | No repository-wide numeric integration-test pass-rate, latency, timeout or retry threshold exists. | Quality/operations | No | Numeric targets require explicit canonical authority; do not invent locally. |

---

## 10. Traceability Summary

### Canonical Responsibility

`L5-F-02-003` — **Contract/event/connector/end-to-end verification across technical boundaries.**

### Parent Verification Architecture

`L5-F-02-001` owns the common Verification Obligation, Result, Evidence, Currentness and Gate model.

F02-003 returns specialized cross-boundary evidence into that model.

### Primary Technical Verification Targets

| Target | Material Verification Scope |
|---|---|
| `L5-C-01-002` | actual synchronous request admission, context, owner-local execution, explicit outcomes and unknown-outcome handling |
| `L5-C-01-003` | contract compatibility, version coexistence, errors and idempotency |
| `L5-C-02-001` | actual publication/routing/subscription/consumer-delivery path |
| `L5-C-02-002` | acknowledgement, duplicate delivery, retry/redelivery, ordering, replay and consumer isolation |
| `L5-C-02-003` | explicit event version/schema compatibility and consumer support |
| `L5-C-03-001` | actual connector/adaptor/provider exchange, correlation and evidence handoff |
| `L5-C-03-002` | effective connector configuration, credential binding, health/failure-boundary effects |
| `L5-A-04-002` | actual cross-boundary Tenant/Institution context preservation and revalidation |
| `L5-A-02-001` | implementation target conforms to approved technology standards without transferring peer technology ownership |

### Boundary to F02-002

F02-002 owns isolated/lower-scope component, unit and owner-local verification.

F02-003 is required when the verification claim depends on the actual governed interaction between independently owned technical boundaries.

### Boundary to F02-004

F02-004 owns specialized security, performance, resilience and recovery testing.

F02-003 may observe ordinary contract/integration failures, but adversarial security verification, load/capacity testing, fault-injection and disaster-recovery validation belong to F02-004.

### Applicable ADR Context

- `ADR-L5-0003` — modular platform / governed boundaries;
- `ADR-L5-0004` — Tenant/Institution isolation;
- `ADR-L5-0006` — owner-local transaction boundaries;
- `ADR-L5-0007` — state-to-publication handoff;
- `ADR-L5-0008` — compatible evolution;
- `ADR-L5-0010` — durable event delivery/subscription/ordering/recovery;
- `ADR-L5-0011` — event contract versioning/compatibility;
- `ADR-L5-0014` — health/degradation/recovery semantics;
- `ADR-L5-0016` — shared external connector pattern;
- `ADR-L5-0018` — Rust primary server platform and governed implementation exceptions.

### Architecture Decisions

`AD-001`–`AD-010`

### Conformance Requirements

`CR-001`–`CR-020`

### Current Status

**Architecture Approved — post-R8 lifecycle closure.**

F02-003 is reconciled with F02-001/002/004 on `main`. Stage-6 downstream consumption is approved; Architecture Approved/lock status remains separate governance.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Reconciled integration-test dependency wording with the locked API/event transport profiles.


## 2026-09-23 Post-R8 Lifecycle Closure

This specification is promoted from Review to Architecture Approved after current peer maturity and upstream L4 authority were revalidated. F02-001's selective BH mappings are separately materialized in the canonical L4→L5 store; child specifications retain their established verification responsibilities without creating new business ownership or release authority. Final corpus lock remains separate governance.
