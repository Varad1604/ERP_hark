# Security, Performance, Resilience & Recovery Testing Architecture

## 1. Specification Metadata

**Specification ID:** `L5-F-02-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-F — Engineering Delivery & Quality Architecture  
**Major Area:** L5-F-02 — Verification & Testing Architecture  
**Canonical Technical Responsibility:** Non-functional verification for security, capacity, failure isolation and disaster recovery.  
**Technical Architecture Owner:** L5-F-02 — Verification & Testing Architecture  
**Assigned Author:** Chetan  
**Version:** `1.0.1`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-F-02-001_Verification_and_Testing_Architecture_Baseline.md`; `L5-D-03-001_Security_Architecture_and_Trust_Boundary_Baseline.md`; `L5-D-03-002_Cryptography_and_Data_Protection_Architecture.md`; `L5-D-03-004_Secrets_Certificates_and_Machine_Credential_Architecture.md`; `L5-D-03-005_Application_Service_and_Network_Trust_Segmentation_Architecture.md`; applicable `L5-D-01-*`, `L5-D-02-*`, `L5-D-03-003`, `L5-D-04-*`; `L5-E-01-001_Availability_and_Resilience_Architecture.md`; `L5-E-01-002_Failure_Isolation_Graceful_Degradation_and_Service_Recovery_Architecture.md`; `L5-E-02-001_Performance_and_Capacity_Architecture.md`; `L5-E-02-002_Caching_Throughput_and_Capacity_Management_Architecture.md`; `L5-E-03-001_Backup_Restore_and_Disaster_Recovery_Architecture_Baseline.md`; `L5-E-03-002_Backup_Restore_and_Point_in_Time_Recovery_Architecture.md`; `L5-E-03-003_Disaster_Recovery_Failover_and_Recovery_Validation_Architecture.md`; applicable `L5-E-04-*`, `L5-E-05-*`, `L5-A-02-001_Technology_Platform_and_Standards_Architecture.md`; L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; current exact L4 Behavioral Obligations and L5 Architecture Conformance Requirements routed by F02-001.  
**Referenced By:** `L5-F-02-001`; `L5-F-01-*`; architecture-review, operational-readiness and release-evidence consumers.  
**Applicable ADRs:** No ADR is currently allocated specifically to `L5-F-02-004`. Verification obligations may target architecture governed by applicable approved ADRs, including `ADR-L5-0001`, `ADR-L5-0002`, `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0005`, `ADR-L5-0006`, `ADR-L5-0007`, `ADR-L5-0009`, `ADR-L5-0010`, `ADR-L5-0013`, `ADR-L5-0014`, `ADR-L5-0017`, and `ADR-L5-0018` where relevant. F02-004 consumes those decisions without reinterpreting or owning them.  

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines how SutraERP verifies non-functional technical conformance for security, performance/capacity, resilience/failure isolation, and backup/restore/disaster-recovery behavior.

It specializes the common `L5-F-02-001` Verification Obligation, Result, Evidence, Currentness and Gate model for concerns that require deliberate security-negative-path verification, controlled workload/pressure verification, controlled fault/degradation verification, or recoverability exercises.

F02-004 produces evidence about whether the implementation conforms to already-defined D-family and E-family architecture. It does not create security policy, performance objectives, resilience profiles, recovery objectives or disaster-recovery topology.

### In Scope

- security-control verification at explicit trust boundaries;
- fail-closed and no-silent-downgrade verification;
- Tenant/Institution and security-context integrity under protected execution;
- cryptographic-protection boundary verification where exact architecture requires it;
- secret/certificate/machine-credential handling verification where exact architecture requires it;
- trust-segmentation and compromise-containment verification;
- performance/capacity verification against approved workload/capacity profiles;
- throughput/admission/backpressure verification;
- cache/freshness behavior under declared pressure/failure conditions where E02 applies;
- resilience and failure-domain containment verification;
- declared graceful-degradation verification;
- recovery-validation and controlled-return verification;
- backup/restore/PITR verification;
- disaster-recovery/failover/failback exercise verification;
- split-brain/fencing safety verification where required;
- recovery-set integrity and dependency-readiness verification;
- attributable result/evidence handoff to F02-001.

### Out of Scope / Non-Goals

F02-004 does not own:

- component/unit/domain verification mechanics — `L5-F-02-002`;
- contract/event/connector/end-to-end interoperability mechanics except where they are exercised as part of a non-functional test — `L5-F-02-003`;
- security architecture, trust-zone definitions, cryptographic policy, key custody, credential lifecycle or authorization policy;
- business SLA/SLO, RPO/RTO, retention, availability promises or business-continuity policy;
- E01 health/degradation/recovery semantics;
- E02 capacity envelope, cache policy or admission semantics;
- E03 recovery classes, backup/restore topology, DR topology or failover authority;
- E04 runtime/deployment topology;
- business rules, business recovery, compensation, lifecycle state or canonical event ownership;
- named security scanner, load-test tool, fault-injection product, backup product, cloud service, orchestration platform or testing framework;
- universal numeric security/performance/recovery targets.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Status | Exact Responsibility / Requirement | F02-004 Use |
|---|---|---|---|
| L5 Candidate Document Corpus Register 105 | Architecture Approved | `L5-F-02-004` = non-functional verification for security, capacity, failure isolation and disaster recovery | Freezes canonical responsibility. |
| `L5-F-02-001` | Stage-6 consolidated Draft authoring baseline | common obligation/result/evidence/currentness/gate semantics | Parent verification contract. |
| `L5-D-03-001` | Draft v1.0.1 | explicit trust boundaries, contextual/non-transitive trust, fail-closed protected crossings | Security verification source. |
| `L5-D-03-002` | Draft v1.0.1 | mandatory cryptographic protection, no silent downgrade, protected representation handling | Cryptographic boundary verification source. |
| `L5-D-03-004` | Draft v1.0.1 | controlled credential provisioning/use, scope binding, revocation and compromise containment | Credential-handling verification source. |
| `L5-D-03-005` | Draft v1.0.1 | logical segmentation, explicit-allow protected crossing, compromise containment | Segmentation verification source. |
| `L5-E-01-001` | Draft v1.0.1 | scoped `Healthy`/`Degraded`/`Unavailable`, degradation profiles, restoration gate | Resilience verification source. |
| `L5-E-01-002` | Draft v1.0.1 | failure classification, containment, degradation gate, delegated recovery, validated return | Failure-isolation/recovery verification source. |
| `L5-E-02-001` | Draft v1.0.1 | capacity dimensions/envelope, pressure states, admission signal | Performance/capacity verification source. |
| `L5-E-02-002` | Draft v1.0.1 | cache validity, throughput gate, backpressure, no-silent-drop behavior | Throughput/cache-pressure verification source. |
| `L5-E-03-001` | Locked v1.0.2 | recovery profiles/sets; backup success alone is insufficient proof | Recoverability verification source. |
| `L5-E-03-002` | Locked v1.0.3 | backup sets, protected copies, restore/PITR, post-restore validation | Restore/PITR verification source. |
| `L5-E-03-003` | Locked v1.0.3 | DR topology, fencing, failover/failback and recovery validation | DR exercise verification source. |
| `L5-A-02-001` | Architecture Approved v1.0.0 | Rust primary server platform; approved standards and governed exceptions | Tested implementation must conform to approved technology standards; F02-004 selects no test product. |

### 3.2 Authoritative Constraints

| ID | Constraint | Effect |
|---|---|---|
| `C-001` | Every F02-004 result binds to an exact F02-001 obligation, exact architecture requirement and exact technical target/version/scope. | No generic “security/performance/DR passed” claim. |
| `C-002` | F02-004 tests architecture; it does not create security policy, capacity limits, degradation profiles or recovery objectives. | Preserves canonical ownership. |
| `C-003` | Missing numeric targets remain missing; F02-004 shall not invent them. | No fabricated latency, throughput, RPO/RTO, availability, retry, coverage or security thresholds. |
| `C-004` | Security tests shall not weaken or bypass mandatory security controls merely to create a test path. | Test mode is not a privileged security bypass. |
| `C-005` | Security test evidence shall not disclose secrets, key material, protected plaintext or sensitive business payloads unless explicitly and authoritatively required. | Preserves D03/D04/B05 boundaries. |
| `C-006` | Performance testing uses approved workload/capacity profiles or measured characterization; absence of an approved limit cannot be converted into a pass/fail threshold. | Separates measurement from invented requirements. |
| `C-007` | Resilience testing verifies E01-defined failure containment/degradation/restoration semantics; it shall not invent business recovery. | Preserves E01 boundary. |
| `C-008` | Recovery testing follows E03 recovery profile/topology and shall not invent universal backup/DR architecture. | Preserves E03 ownership. |
| `C-009` | Backup completion alone cannot establish recoverability. | Restore and validation evidence are mandatory where recoverability is claimed. |
| `C-010` | DR infrastructure becoming reachable/running alone cannot establish restored service. | Integrity, dependencies, trust and critical-path validation remain required. |
| `C-011` | Fault injection/recovery exercise shall be scoped so the test mechanism itself does not create ungoverned business effects. | Controlled test boundary. |
| `C-012` | Tool/harness/infrastructure failure maps to `Verification Not Established` unless attributable target evidence proves non-conformance. | Aligns with F02-001. |

### 3.3 Non-Functional Verification Classes

F02-004 uses four scope classes:

1. **Security Verification**  
   Tests required trust, isolation, protection, credential and fail-closed behavior against exact D-family requirements.

2. **Performance & Capacity Verification**  
   Measures target behavior under declared workload dimensions and verifies E02 admission/backpressure/capacity requirements against approved profiles.

3. **Resilience Verification**  
   Introduces or reproduces controlled technical failure/pressure conditions and verifies E01 containment, degradation and validated return behavior.

4. **Recovery & DR Verification**  
   Exercises E03 backup/restore/PITR/failover/failback/recovery validation against exact recovery profiles and supported deployment architecture.

These are verification classifications, not new architecture ownership domains.

### 3.4 Assumptions

No test framework, penetration-testing tool, workload generator, chaos/fault-injection tool, backup product, database technology, cloud provider, orchestration product, benchmark suite or scanner is assumed.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

F02-004 owns:

- non-functional Verification Target characterization;
- controlled security-negative-path execution semantics;
- approved-workload performance test execution semantics;
- pressure/saturation observation semantics;
- controlled fault/degradation test execution semantics;
- recoverability exercise verification semantics;
- non-functional evidence normalization for F02-001;
- distinction between target non-conformance and test-environment inability;
- explicit disclosure of tested profile/environment/failure/recovery scope.

### Consumes / Collaborates With

| Architecture | Canonical Responsibility | F02-004 Role |
|---|---|---|
| F02-001 | common verification/gate semantics | Receives obligations; receives F02-004 results/evidence. |
| D01/D02 | identity/authentication and authorization | Security testing exercises these controls only where exact obligation requires; does not redefine policy. |
| D03-001 | trust-boundary baseline | Tests explicit admission, non-transitive trust and fail-closed behavior. |
| D03-002 | cryptographic protection | Tests required protection/no-downgrade behavior. |
| D03-003 | key management | Tests only exact exposed conformance obligations; key lifecycle ownership remains D03-003. |
| D03-004 | secrets/certificates/machine credentials | Tests scope/retrieval/revocation/containment behavior where required. |
| D03-005 | trust segmentation | Tests explicit-allow/default-deny and compromise containment. |
| D04 | audit/evidence | Receives security/recovery evidence where required; F02 does not invent D04 storage. |
| E01 | health/degradation/recovery | Tests defined failure-containment/degradation/restoration behavior. |
| E02 | capacity/cache/backpressure | Tests declared capacity dimensions/profile behavior. |
| E03 | recovery/DR | Tests recoverability against locked profile/restore/DR architecture. |
| E04 | runtime/deployment | Supplies actual failure/recovery topology and controlled test realization. |
| E05 | observability/operations | Supplies operational measurements/diagnostics; telemetry is not automatically proof of conformance. |
| A02 | technology standards | Constrains implementation target; F02 selects no testing technology. |
| F01 | build/release | May execute/consume F02-004 evidence; retains release authority. |

### Must Not Own

F02-004 shall not:

- approve security exceptions;
- define authorization or trust policy;
- create cryptographic algorithms/parameters;
- create capacity or SLO targets;
- create degradation profiles;
- create recovery classes/RPO/RTO;
- select DR topology;
- mutate business state directly for fault/recovery testing;
- claim release/architecture approval.

---

## 5. Solution Architecture

### 5.1 Common Non-Functional Verification Flow

```text
F02-001 Verification Obligation
        ↓
Resolve exact D/E architecture requirement
        ↓
Resolve test profile + target/version + environment
        ↓
Establish controlled test prerequisites
        ↓
Execute security / workload / failure / recovery scenario
        ↓
Collect target + dependency + operational evidence
        ↓
Evaluate exact requirement
        ↓
Produce F02-004 specialized result
        ↓
F02-001 Result Normalizer / Evidence Binder
```

### 5.2 Non-Functional Verification Target

Conceptually:

```text
NonFunctionalVerificationTarget
{
  exact obligation reference
  exact technical target/version
  verification class
  deployment/environment reference
  Tenant/Institution scope where applicable
  approved workload/security/failure/recovery profile reference
  dependency set
  test-control boundary
  expected observable evidence
}
```

This is an architecture model, not a mandatory file/schema format.

### 5.3 Security Verification Architecture

Security verification is derived from exact security CRs/TBs rather than a generic attack checklist.

Applicable test categories include only what the source architecture establishes:

- protected-boundary admission;
- identity/authorization separation;
- Tenant/Institution context preservation;
- invalid/missing context rejection;
- contextual/non-transitive trust;
- fail-closed mandatory controls;
- external/untrusted context handling;
- required cryptographic protection;
- no silent crypto downgrade;
- protected plaintext exposure boundary;
- credential scope/retrieval/revocation behavior;
- secret/credential non-disclosure in logs/evidence;
- explicit trust-zone allow/deny behavior;
- privileged/restricted path isolation;
- compromise containment and controlled restoration.

A security verification result does not create security policy.

### 5.4 Security Negative-Path Verification

Where the architecture requires fail-closed behavior, F02-004 shall deliberately verify the relevant failure case.

Examples supported by D03 include:

- identity unavailable where required;
- authorization denied/unavailable;
- invalid/missing Tenant/Institution scope;
- invalid trust relationship;
- required cryptographic protection unavailable;
- integrity/authenticity verification failure;
- invalid/revoked credential;
- prohibited trust-zone crossing;
- compromised component/credential relationship.

The expected result is the exact architecture-defined rejection/containment behavior, not a new business failure state.

### 5.5 Security Evidence Handling

Security evidence shall identify as applicable:

- target/version;
- boundary/control under test;
- test precondition;
- technical actor/workload context;
- Tenant/Institution scope;
- allowed/denied/contained outcome;
- evidence reference;
- correlation/provenance.

Evidence shall not record secret values, private key material, protected plaintext or sensitive payloads merely for verification completeness.

### 5.6 Performance / Capacity Test Profile

A performance test shall reference an approved workload/capacity profile when a normative pass/fail claim is made.

A profile may define, where authoritatively available:

- technical scope;
- workload dimensions;
- demand shape;
- concurrency/throughput dimensions;
- dataset/volume class;
- dependency profile;
- governed capacity/latency/service objective references;
- required observation interval/conditions;
- admission/backpressure expectations.

If no approved numeric target exists, F02-004 may produce characterization evidence but shall not invent a pass/fail threshold.

### 5.7 Performance Measurement Model

F02-004 may measure only relevant dimensions, such as:

- request throughput/latency distribution;
- transaction/query concurrency;
- persistence pressure;
- background-work parallelism;
- event/message ingress/delivery/backlog;
- external integration concurrency;
- computation density;
- cache pressure/hit/validity behavior;
- resource/queue saturation signals.

The measurement model does not itself create capacity requirements.

### 5.8 Capacity / Saturation Verification

Where E02 applies, verify:

```text
declared workload
        ↓
Demand Meter / observed pressure
        ↓
E02 Capacity Evaluator
        ↓
Normal | Pressured | Saturated
        ↓
Accept | Defer | Throttle | RejectTechnical
        ↓
owning capability preserves work / applies allowed response
```

F02-004 verifies that:

- the capacity signal is attributable to the correct scope/dimension;
- saturation is explicit;
- unsafe additional work is not silently admitted/dropped;
- owner boundaries remain intact;
- E01 health mapping is used if saturation affects availability.

### 5.9 Cache / Throughput Pressure Verification

Where E02-002 applies, verify only source-defined behavior such as:

- cache validity/provenance is preserved;
- Tenant/Institution scope is not collapsed;
- stale/invalid data is rejected or handled only by declared bounded-stale policy;
- freshness-critical operation revalidates/bypasses cache as required;
- downstream pressure produces explicit backpressure;
- durable A08/C02 work is not silently dropped;
- cache failure uses safe owner-path fallback or E01-governed degradation/unavailability.

No universal TTL, rate or queue depth is invented.

### 5.10 Resilience Test Scenario

A resilience test declares:

```text
affected scope
+ exact failure class
+ injection/reproduction method
+ declared E01 degradation profile, if any
+ expected containment boundary
+ recovery-action owner
+ restoration criteria
```

F02-004 owns the verification scenario, not the recovery mechanism itself.

### 5.11 Failure Injection / Reproduction Boundary

A controlled resilience test may reproduce conditions such as:

- local transient/persistent technical failure;
- dependency failure;
- capacity exhaustion;
- state-integrity uncertainty;
- security/trust uncertainty;
- component/runtime loss;
- C02 consumer-path failure;
- external dependency unavailability.

The injection mechanism shall not:

- directly rewrite another owner's authoritative state;
- create ungoverned business events;
- bypass authorization/isolation;
- broaden failure beyond the declared safe test scope.

### 5.12 Failure Isolation Verification

Verify, where required:

- failure is classified and attributable;
- affected scope is correctly identified;
- unrelated scopes remain available where architecture permits;
- unsafe paths are contained;
- no undeclared degradation is enabled;
- mandatory dependency failure results in the defined unavailable/degraded behavior;
- recovery is delegated to the canonical owner;
- business recovery is not invented by E01/F02.

### 5.13 Graceful Degradation Verification

A degraded-mode pass requires a declared E01 degradation profile.

Verify:

- tested failure is covered by that profile;
- only permitted operations remain available;
- blocked operations remain blocked;
- data authority/freshness limits are honored;
- caller-visible degradation signal is present where required;
- no success/freshness/business completion is fabricated;
- profile exit occurs only after its required recovery evidence.

If no applicable profile exists, degraded operation is not a valid pass condition.

### 5.14 Restoration / Return-to-Service Verification

A successful resilience recovery test requires positive evidence that applicable E01 restoration prerequisites are satisfied.

Verify:

1. failure source is removed or isolated;
2. owner-state integrity is confirmed where applicable;
3. mandatory dependencies are healthy enough;
4. required security/context authority is valid;
5. unresolved replay/retry/recovery does not make normal service unsafe;
6. restrictions/degradation are removed in controlled order;
7. resulting `Healthy` status has attributable evidence.

Disappearance of the original error alone is insufficient.

### 5.15 Recoverability Verification Baseline

E03 establishes that backup success alone is not proof of recoverability.

Therefore F02-004 recoverability verification requires:

```text
approved recovery profile
→ trusted recovery point / recovery set
→ authorized recovery action
→ restore / reconstruction
→ integrity + compatibility validation
→ dependency + security readiness
→ critical path verification where required
→ E01 controlled return
→ evidence
```

### 5.16 Backup / Protected-Copy Verification

Where required by the recovery profile, F02-004 may verify:

- backup/recovery set is attributable to the correct asset/profile;
- required protected-copy isolation/protection property exists;
- recovery-point metadata is usable;
- corrupt/untrusted recovery material is rejected;
- required credentials/keys remain governed;
- derived/rebuildable stores are handled according to their profile;
- backup completion is not reported as full recovery success.

### 5.17 Restore / PITR Verification

Where E03-002 and selected storage capability support it, verify:

- trusted target recovery point is selected;
- dependent stores recover to a compatible recovery-set point;
- required keys/credentials resolve through governed paths;
- restored representation is readable/integral;
- owner-specific consistency validation passes;
- application/dependency readiness passes;
- service is not returned before E01 controlled return.

No PITR granularity, backup frequency or retention value is invented.

### 5.18 Disaster-Recovery / Failover Verification

A DR test follows the exact E03-003 plan/topology selected from an approved recovery profile.

Verify as applicable:

- material outage is correctly qualified;
- recovery plan/profile is resolved;
- trusted recovery point/state is established;
- authoritative write role is fenced/transferred safely where required;
- concurrent authoritative writers are prevented;
- runtime/data/configuration/dependencies recover;
- C02 durable work is handled by C02 semantics;
- C03 ambiguous external effects are not blindly replayed;
- integrity/security/dependency/critical paths validate;
- E01 controlled return occurs only after successful validation.

### 5.19 Failback Verification

Failback is a separate controlled verification concern.

Verify:

- former primary reachability alone does not trigger automatic authoritative return;
- platform/security health is validated;
- state is synchronized/reconciled;
- write-role/ownership safety is established;
- dependencies and critical paths are validated;
- traffic/write-role transition is authorized according to E03;
- failback evidence is preserved.

### 5.20 Recovery Exercise Safety

A recovery/DR exercise shall explicitly identify whether it uses:

- isolated recovery environment;
- non-production recovery target;
- provider sandbox/safe external dependency;
- governed simulation for dependencies that cannot safely be exercised.

Simulation may prove only the behavior it actually exercises.

A mocked external dependency cannot prove a real provider failover behavior that it does not implement.

### 5.21 Mixed Non-Functional Scenarios

One scenario may exercise more than one F02-004 concern, for example:

- capacity pressure causing E01 degradation;
- security credential failure during recovery;
- DR failover followed by performance characterization.

Evidence shall remain decomposable by obligation so one successful dimension does not hide another failed/unverified dimension.

### 5.22 Result / Evidence Handoff

F02-004 returns:

```text
NonFunctionalVerificationResult
{
  obligation reference
  target/version/environment
  verification class
  profile/reference set
  induced/observed condition
  execution reference
  measurement/observation references
  recovery/security evidence references
  conformance outcome
  unresolved requirement/profile references
}
```

This maps into the F02-001 common result/evidence semantics.

---

## 6. Architecture & Technology Decisions

| ID | Decision | Position | Basis |
|---|---|---|---|
| `AD-001` | Security verification source | Security testing derives from exact D-family CR/TB obligations rather than a generic attack catalogue. | D03 ownership + F02 responsibility |
| `AD-002` | Fail-closed verification | Architecture-defined mandatory-control failures are explicit negative test cases. | D03-001/002/004/005 |
| `AD-003` | Evidence protection | Security evidence excludes secrets/key material/protected plaintext unless separately authorized. | D03/D04/B05 |
| `AD-004` | Performance thresholds | Normative pass/fail uses approved workload/target profiles; absent targets are characterized, not invented. | E02 + no-guessing governance |
| `AD-005` | Capacity behavior | Saturation testing verifies E02 admission/backpressure and no-silent-drop semantics. | E02-001/002 |
| `AD-006` | Resilience testing | Fault/degradation tests verify E01-defined containment and declared degradation profiles without owning recovery mechanisms. | E01 + `ADR-L5-0014` |
| `AD-007` | Restoration proof | Return to `Healthy` requires positive state/dependency/security evidence, not disappearance of error. | E01 |
| `AD-008` | Recoverability proof | Backup success is insufficient; restore plus validation evidence is required. | E03-001/002 + `ADR-L5-0017` |
| `AD-009` | DR proof | Infrastructure running is insufficient; fencing/ownership, integrity, dependency, trust and critical-path validation are required. | E03-003 |
| `AD-010` | Technology neutrality | A02 standards constrain implementation targets, but F02-004 selects no security/load/chaos/recovery testing product. | A02 / `ADR-L5-0018` |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Required Position |
|---|---|
| Test authorization | Security/fault/recovery tests run only within authorized technical scope and do not grant additional business authority. |
| Tenant/Institution isolation | Test scenarios preserve explicit scope; cross-Tenant test leakage is non-conformant. |
| Secrets / keys | Never place secret/key material into ordinary test fixtures, logs or evidence. |
| Production safety | Destructive/fault/recovery tests require a governed safe environment/scope; no uncontrolled production disruption is authorized by this document. |
| Sensitive data | Use B05-governed handling; production-derived data is not unrestricted test data. |
| Performance evidence | Measurements include enough profile/environment context to be comparable and attributable. |
| Resilience evidence | Failure condition, affected scope, containment and restoration evidence are explicit. |
| Recovery evidence | Recovery point/set, method, validation and disposition are reconstructable. |
| Observability | E05 telemetry supports measurement/diagnosis; telemetry alone does not prove business/security truth. |
| Release use | F01 may consume F02-004 gates but retains release authority. |

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Every F02-004 verification result shall bind an exact F02-001 obligation to an exact technical target/version/environment and applicable security/workload/failure/recovery profile. |
| `CR-002` | F02-004 shall not create or modify D/E architecture requirements in order to make a test executable or passable. |
| `CR-003` | Mandatory security-control failure cases defined by the governing architecture shall be verified as fail-closed where material to the obligation. |
| `CR-004` | Security testing shall preserve Tenant/Institution scope and shall not use implicit/default Tenant authority. |
| `CR-005` | Security evidence shall not expose secrets, private key material, protected plaintext or credential values merely for verification completeness. |
| `CR-006` | Required cryptographic protection shall not be tested using an unprotected fallback that the governing architecture prohibits. |
| `CR-007` | Credential/revocation/segmentation verification shall preserve D03 ownership and shall not convert possession/reachability into business authority. |
| `CR-008` | Normative performance/capacity pass/fail claims shall use approved targets/profile references; absent numeric targets shall not be invented. |
| `CR-009` | Performance/capacity evidence shall identify the tested scope, workload dimensions, target/environment and applicable dependencies. |
| `CR-010` | Capacity saturation verification shall demonstrate explicit E02 admission/backpressure behavior and shall not treat silent work loss as conformant. |
| `CR-011` | Cache/throughput testing shall not allow cache state to become unauthorized source of truth or cross-owner write authority. |
| `CR-012` | Resilience fault scenarios shall be explicitly scoped and shall not directly mutate another owner's authoritative state merely to create failure. |
| `CR-013` | `Degraded` verification shall require an applicable declared E01 degradation profile; undeclared degradation shall not be treated as conformant. |
| `CR-014` | Degraded operation shall not fabricate authoritative freshness, success or business completion. |
| `CR-015` | Restoration to `Healthy` shall require positive E01-defined state/dependency/security validation evidence. |
| `CR-016` | Recovery action ownership shall remain with the applicable canonical architecture; F02-004 verifies the behavior but does not absorb A08/C02/B02/E04/D03 recovery mechanisms. |
| `CR-017` | Backup completion alone shall not establish recoverability. |
| `CR-018` | Restore/PITR verification shall use a trusted recovery point/set and shall validate integrity/compatibility before E01 controlled return. |
| `CR-019` | Missing/corrupt/untrusted recovery material or required key/credential material shall block recovery promotion rather than trigger an insecure bypass. |
| `CR-020` | DR failover verification shall preserve canonical ownership and prevent competing authoritative writers where E03 requires single-writer safety. |
| `CR-021` | Automatic failover shall not be represented as conformant unless E03-required failure qualification and consistency/fencing conditions are demonstrated. |
| `CR-022` | Ambiguous in-flight external effects shall not be blindly replayed during recovery testing. |
| `CR-023` | Failback verification shall validate state, trust, dependencies and authoritative-role safety before return. |
| `CR-024` | DR/recovery success shall require integrity, dependency, security/trust and required critical-path validation, not only running infrastructure. |
| `CR-025` | Fault/recovery-test harness failure shall map to `Verification Not Established` unless target evidence independently proves non-conformance. |
| `CR-026` | F02-004 shall not introduce universal latency, throughput, availability, security, RPO/RTO, retention, backup-frequency, retry or recovery-time numbers. |
| `CR-027` | F02-004 shall remain independent of a specific security scanner, load generator, fault-injection tool, backup product, cloud provider or orchestration technology unless separately governed. |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Classification | Blocking? | Required Resolution |
|---|---|---|---|---|
| `OD-001` | The four-file F02 Draft authoring package is now family-reconciled locally but is not yet synchronized into the canonical repository. | Repository synchronization | No architectural blocker; procedural prerequisite to repository submission | Submit the reconciled F02-001/002/003/004 Drafts under their exact canonical paths, then re-run repository-level reference/status validation. |
| `OD-002` | E01/E02 specifications remain Draft while E03 is Locked. | Peer maturity | No for Stage-6 Draft; may affect approval-level claims derived from E01/E02 | Re-check E01/E02 status before F02 family promotion. |
| `OD-003` | D03 security specifications used here remain Draft although governing ADRs 0001/0002 are approved. | Peer maturity | No for Draft; may affect final approval claims | Re-check D03 governance status and exact CRs before final validation. |
| `OD-004` | Numeric performance/capacity/security/recovery objectives are not universally established. | Requirement / operations | No | Use only approved workload/service/recovery/security profile values when available. |
| `OD-005` | Exact testing/scanning/load/fault-injection/recovery products are not selected. | Technology | No | Select only through A02/canonical owner/ADR governance if made normative. |
| `OD-006` | Exact safe environments for destructive security/fault/DR exercises vary by deployment/integration. | Operations / deployment | No | E04/F01/E03 and applicable provider profiles establish safe execution context. |
| `OD-007` | Some external provider failover/recovery paths may not be exercisable against real provider infrastructure. | External dependency | No | Evidence shall disclose simulation/sandbox boundaries and leave unexercised real-provider behavior unresolved. |

---

## 10. Traceability Summary

### Canonical Responsibility

`L5-F-02-004` — **Non-functional verification for security, capacity, failure isolation and disaster recovery.**

### Parent Verification Architecture

`L5-F-02-001` owns the common Verification Obligation, Result, Evidence, Currentness and Gate model.

F02-004 supplies specialized non-functional results into that common model.

### Security Verification Sources

| Source | Material F02-004 Scope |
|---|---|
| `L5-D-03-001` | explicit trust boundaries, fail-closed admission, non-transitive trust, context integrity, compromise containment |
| `L5-D-03-002` | required transport/stored protection, no silent downgrade, protected representation/integrity validation |
| `L5-D-03-004` | controlled secret/certificate/machine-credential retrieval/use, scope binding, revocation/exposure containment |
| `L5-D-03-005` | explicit-allow/default-deny trust segmentation, privileged-path restriction, containment |
| applicable D01/D02/D03-003/D04/B05 | exact identity, authorization, key, evidence and sensitive-data requirements routed by the obligation |

### Performance / Capacity Verification Sources

| Source | Material F02-004 Scope |
|---|---|
| `L5-E-02-001` | workload dimensions, capacity envelope, pressure state, admission/backpressure behavior |
| `L5-E-02-002` | cache validity/freshness, throughput gate, pressure propagation, no silent drop |
| `L5-E-01-*` | capacity-induced health/degradation behavior |
| B02/A08/C02/E04/E05 | owner-specific capacity, backlog, runtime and observation boundaries |

### Resilience Verification Sources

| Source | Material F02-004 Scope |
|---|---|
| `L5-E-01-001` | scoped health, declared degradation, failure-domain isolation, restoration gate |
| `L5-E-01-002` | failure classification, containment, delegated recovery, positive validation, controlled return |
| `ADR-L5-0014` | repository resilience/health/degradation/recovery pattern |

### Recovery / DR Verification Sources

| Source | Material F02-004 Scope |
|---|---|
| `L5-E-03-001` Locked | recovery class/profile/set and restore-validation requirement |
| `L5-E-03-002` Locked | protected copies, restore/PITR, compatible recovery sets, post-restore validation |
| `L5-E-03-003` Locked | DR plan/topology, fencing, failover/failback, split-brain prevention and recovery validation |
| `ADR-L5-0017` | recovery classification/protected-copy/restore/DR-validation pattern |
| E01/E04/B02/C02/C03/D03/E05 | controlled return, runtime, consistency, durable messaging, external ambiguity, trust and telemetry boundaries |

### Boundary to F02-002

F02-002 owns lower-scope functional/component/domain verification.

F02-004 owns the specialized non-functional test when proof requires security-negative-path, workload/capacity, fault/degradation or recovery/DR conditions.

### Boundary to F02-003

F02-003 owns ordinary real-boundary interoperability.

F02-004 may exercise real contracts/events/connectors while testing non-functional behavior, but C01/C02/C03 interoperability semantics remain owned by F02-003 and their canonical architecture owners.

### Technology Standards

`L5-A-02-001` establishes Rust as the primary server platform with governed exceptions.

F02-004 verifies the actual approved implementation target but does not select security/load/fault/recovery testing products or peer-owned infrastructure products.

### Architecture Decisions

`AD-001`–`AD-010`

### Conformance Requirements

`CR-001`–`CR-027`

### Current Status

**Draft — Stage-6 F02 final child specification.**

With this file authored, the four-spec F02 family has a complete Draft authoring set:

- `L5-F-02-001` — Verification & Testing Architecture Baseline;
- `L5-F-02-002` — Component, Unit & Domain Verification Architecture;
- `L5-F-02-003` — Integration, Contract & End-to-End Verification Architecture;
- `L5-F-02-004` — Security, Performance, Resilience & Recovery Testing Architecture.

The four-spec F02 Draft authoring set is now internally reconciled. Final repository synchronization, governed validation, approval and lock remain outside author authority.
