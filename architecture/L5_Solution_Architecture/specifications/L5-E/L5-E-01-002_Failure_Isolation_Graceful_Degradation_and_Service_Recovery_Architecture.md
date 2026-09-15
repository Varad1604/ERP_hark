# L5-E-01-002 — Failure Isolation, Graceful Degradation & Service Recovery Architecture

## 1. Specification Metadata

**Specification ID:** `L5-E-01-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-01 — Availability & Resilience Architecture  
**Canonical Technical Responsibility:** Failure containment, declared graceful degradation and validated technical recovery/return-to-service without taking ownership of business recovery or peer-family runtime mechanisms.  
**Technical Architecture Owner:** L5-E-01 — Availability & Resilience Architecture  
**Assigned Author:** Varad  
**Version:** `1.0.1`  
**Status:** `Draft`  
**Last Updated:** `2026-09-15`  
**Depends On:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Technology Selection & ADR Methodology; L5 ADR Register; `L5-E-01-001`; `L5-A-01-001`; `L5-E-04-001`; `L5-E-02-*`; `L5-A-08-*`; applicable B02/C02/D03/D04 architecture; `L4-A-015`; `L4-A-016`; `L4-A-057`.  
**Referenced By:** L5 capabilities that declare degradation/recovery profiles.  
**Applicable ADRs:** `ADR-L5-0014`, `ADR-L5-0013`, `ADR-L5-0010`.  

---

## 2. Purpose, Scope & Non-Goals

This specification realizes the E01 shared resilience decision as concrete failure-containment, graceful-degradation and recovery-control architecture. It defines the technical objects, control points and handoffs required to move from detected failure through isolation and validated return to service.

It does not define business recovery/compensation, workflow transitions, A08 retry mechanics, C02 delivery recovery, B02 persistence internals, E04 deployment/orchestration technology, or numeric SLO targets.

---

## 3. Constraints & Dependency Gate

1. `L5-E-01-001` and `ADR-L5-0014` define the canonical health vocabulary and resilience policy.
2. Stage-3 dependencies A01 + E04 are explicitly consumed.
3. Recovery may coordinate peer mechanisms but never takes over their ownership.
4. Any mutation of authoritative owner state occurs through the owner contract.
5. `Degraded` requires an approved degradation profile; otherwise fail closed.
6. Restoration requires positive validation of state and dependencies.
7. A08 technical retry and C02 message recovery remain owned by those families.

---

## 4. Technical Architecture

### 4.1 Recovery Control Record

Each material recovery episode shall have a durable/observable control identity containing at minimum:

- `recovery_id`;
- affected technical scope;
- Tenant/Institution scope where applicable;
- detected failure class and evidence reference;
- current health state;
- containment action/status;
- selected recovery action owner;
- validation status;
- return-to-service status;
- correlation/causation reference.

This record is technical evidence, not a business entity.

### 4.2 Logical Control Points

| Control point | Responsibility |
|---|---|
| Failure Classifier | classifies observed technical condition and affected scope |
| Containment Controller | blocks/isolates unsafe paths and limits blast radius |
| Degradation Gate | permits only declared degraded capabilities |
| Recovery Coordinator | invokes the owning recovery mechanism/contract |
| Recovery Validator | verifies integrity, dependency readiness and safe operation |
| Return-to-Service Controller | removes restrictions in controlled order after validation |

### 4.3 Failure Classification

At minimum classify:

- local transient technical failure;
- local persistent technical failure;
- dependency failure;
- data/state-integrity uncertainty;
- security/trust uncertainty;
- capacity exhaustion;
- unresolved/unknown.

Classification determines containment/recovery eligibility but does not establish business meaning.

### 4.4 Containment

Containment may:

- stop new admission to an affected scope;
- pause technical release of A08 work via A08 contract;
- stop routing to an unhealthy execution unit via E04/runtime controls;
- block a read/write operation whose authoritative prerequisite is unavailable;
- isolate one consumer/subscription through C02-owned recovery;
- mark the scope `Unavailable` or `Degraded`.

Containment must not directly rewrite business state or another owner’s persistence.

### 4.5 Degradation Contract

For each permitted degraded mode, the owner declares:

- `profile_id` and affected capability;
- operations still allowed;
- operations forbidden;
- data-authority/freshness constraints;
- dependency substitutions/bypasses, if any;
- caller-visible degradation signal;
- evidence required to activate and exit the profile.

The Degradation Gate rejects activation when the profile does not cover the observed failure.

### 4.6 Recovery Handoff

Recovery action is delegated to the owning architecture:

- A08: job/unit/attempt retry/recovery mechanics;
- C02: event delivery/subscription/replay recovery;
- B02: persistence consistency/recovery primitives;
- E04: process/container/host restart/failover/deployment operations;
- security family: trust/credential/key recovery;
- owning business/workflow domain: business recovery/compensation.

E01 coordinates evidence and return criteria; it does not duplicate these mechanisms.

### 4.7 Recovery Validation

Before controlled return, verify applicable:

1. failure source is removed or safely isolated;
2. owner state consistency/integrity is confirmed;
3. mandatory dependencies satisfy required health contract;
4. security/context authority is valid;
5. in-flight recovery/replay/retry does not make normal operation unsafe;
6. degradation limits are no longer required or remain intentionally declared;
7. observability/evidence is available.

### 4.8 Controlled Return

Return occurs in a deliberate sequence:

```text
RecoveryActionComplete
 -> ValidationPassed
 -> LimitedAdmission/Probe if applicable
 -> RestrictionsRemoved
 -> Health = Healthy
```

If validation fails, the scope remains `Unavailable`/`Degraded`; failure is not represented as restored service.

### 4.9 Duplicate / Concurrent Recovery Protection

For a single affected scope, competing recovery controllers shall use a durable claim/version or semantically equivalent coordination mechanism. A stale recovery controller cannot remove restrictions or mark Healthy after losing authority.

---

## 5. Cross-Family Handoffs

### E01 ↔ A08
E01 may request pause/release restriction for an affected execution scope. A08 owns actual Job/Unit/Attempt transitions and retry. E01 consumes resulting status/evidence.

### E01 ↔ C02
E01 may classify messaging capability health and request isolation of affected delivery paths. C02 owns retry/dead-message/replay semantics.

### E01 ↔ B02
E01 consumes persistence health/integrity evidence. Any repair or authoritative state mutation is B02/owner governed.

### E01 ↔ E04
E01 declares containment/restart/failover eligibility and restoration criteria; E04 performs runtime/deployment actions.

---

## 6. Failure Concerns

| Failure | Required response |
|---|---|
| unknown failure | isolate affected scope; no undeclared degradation; require diagnosis |
| state integrity uncertain | mark unsafe/unavailable; no restoration until owner validation |
| recovery action fails | remain contained; record evidence; select next governed owner action if one exists |
| duplicate recovery controllers | one valid claim; stale controller cannot return service |
| dependency remains degraded | only declared compatible degradation may continue |
| observability/evidence unavailable | do not claim fully validated restoration when evidence is required |

---

## 7. Conformance Requirements

| ID | Requirement |
|---|---|
| CR-001 | Every material recovery episode has an attributable affected scope and recovery identity/evidence. |
| CR-002 | Failure classification is explicit and cannot silently become business recovery policy. |
| CR-003 | Containment does not directly mutate another owner’s authoritative state. |
| CR-004 | Degraded operation is enabled only by a matching declared degradation profile. |
| CR-005 | Unknown/unresolved conditions fail closed for the unsafe operation. |
| CR-006 | Recovery action is delegated to the owning peer architecture. |
| CR-007 | Return to service requires positive validation of applicable state, dependency and security prerequisites. |
| CR-008 | Competing recovery controllers cannot both authorize restoration of the same scope. |
| CR-009 | Failed validation cannot be represented as Healthy/restored. |
| CR-010 | Stage-3 A01+E04 dependencies and `ADR-L5-0014` are explicitly consumed. |

---

## 8. L4 Traceability

| L4 BH ID | Relationship | L5 contribution / boundary |
|---|---|---|
| `L4-A-015-BH-010` | Supporting | technical containment preserves failure rather than presenting execution as completed. |
| `L4-A-016-BH-001` | Supporting | technical failure classification supports controlled recovery without ownership transfer. |
| `L4-A-016-BH-003` | Supporting | Tenant/execution/correlation context is preserved throughout recovery. |
| `L4-A-016-BH-004` | Supporting | containment prevents unauthorized progression while recovery conditions are unmet. |
| `L4-A-016-BH-005` | Supporting | recovery execution requires valid authority before protected recovery action. |
| `L4-A-016-BH-006` | Supporting | continuation/recovery eligibility is validated before action. |
| `L4-A-016-BH-007` | Supporting | validated recovery can permit return to normal execution while workflow state remains upstream-owned. |
| `L4-A-016-BH-010` | Direct | recovery identity/classification/action/validation evidence supports traceability. |
| `L4-A-057-BH-006` | Supporting | post-change integrity validation is part of recovery validation. |
| `L4-A-057-BH-008` | Direct | unresolved failed change cannot be represented as restored availability. |

---

## 9. Architecture Decisions

**AD-001:** `ADR-L5-0014` governs health, degradation, containment and recovery semantics.  
**AD-002:** `ADR-L5-0013` remains authoritative for A08 job/retry execution when E01 coordinates a background-work recovery.  
**AD-003:** `ADR-L5-0010` remains authoritative for C02 delivery recovery.

---

## 10. Open / Deferred Design

**OD-001:** Capability-specific degradation profiles are separate owner-authored controlled artifacts/sections under this architecture.  
**OD-002:** Concrete runtime failover/orchestration mechanisms remain E04 implementation/technology governance.  
**OD-003:** Numeric recovery thresholds/timers remain governed operational configuration.