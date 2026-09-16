# Connector Lifecycle & Failure Isolation Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Connector configuration/lifecycle, credentials, health and failure isolation.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.1`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-01`; `L5-D-03`; `L5-E-01`; `L5-E-05`; `L5-A-04`; applicable upstream connector-governance behavior  
**Referenced By:** `L5-C-03-003`–`L5-C-03-009`  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0004`; `ADR-L5-0009`; `ADR-L5-0014`  
**Governing Sources:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Candidate Document Corpus Register 105; L5 Technology Selection & ADR Methodology; L5 ADR Register.

## 2. Purpose, Scope & Non-Goals
Defines technical realization for governed connector configuration, effective runtime activation, credential binding, health assessment, failure isolation, recovery and retirement consequences. It does not create a second business lifecycle for a connector configuration and does not own provider business policy or source-domain recovery outcomes.

## 3. Upstream Applicability, Constraints & Assumptions
### 3.1 Canonical Upstream Reference Matrix
| Source | Version / Status | Exact Reference | L5-C-03-002 Contribution |
|---|---|---|---|
| L5 corpus | Approved candidate baseline | `L5-C-03-002` | Assigns connector configuration/lifecycle, credentials, health and failure isolation to this specification. |
| L4 | 1.0.1 / Draft | `L4-H-073-BH-001`–`BH-008` | Requires governed configuration/version progression, activation, suspension/restoration, rollback and retirement without rewriting history. C03 realizes the technical configuration/effective-runtime consequences only. |
| L4 | 1.0.1 / Draft | `L4-H-073-BH-010` | Authentication Provider/Credential lifecycle remains upstream-owned; C03 consumes credential references. |
| L4 | 1.0.1 / Draft | `L4-H-073-BH-011`–`BH-012` | Unauthorized lifecycle actions leave authoritative state unchanged and configuration/decision history remains reconstructable. |
| ADR | Architecture Approved | `ADR-L5-0014` | Supplies canonical technical health/degradation/recovery semantics. |
| L5 peer | current | `L5-D-03` | Owns secrets, certificates and machine-credential controls. |

The cited L4 file is presently `Draft`. These mappings are therefore `Specified` planning evidence, not `Validated` L4→L5 relations under the locked traceability methodology. This remains an approval blocker unless governance authorizes approval-level use of that L4 behavior.

### 3.2 Authoritative Constraints
- H10-owned connector configuration states/events shall not be duplicated as C03 technical lifecycle states.
- C03 runtime enablement shall follow the currently effective upstream-approved connector configuration; it shall not approve, suspend, restore, roll back or retire the business configuration itself.
- `Healthy`, `Degraded` and `Unavailable` are E01 technical health states and shall not be used as substitutes for upstream connector-governance states.
- Credentials remain D03/A03-owned references; secret material is not owned by connector configuration.

## 4. Technical Ownership, Boundaries & Dependencies
**Owns:** connector technical profile realization, provider endpoint/environment binding, effective configuration loading, credential-reference binding, scoped health evaluation, connector-local failure containment, runtime withdrawal/re-entry after authoritative configuration or health changes.  
**Consumes:** upstream connector-governance facts/decisions; A04 Tenant/Institution isolation; D03 credentials; E01 health/degradation/recovery; E05 telemetry; C01 compatibility/idempotency.  
**Must not own:** upstream connector approval/suspension/rollback/retirement decisions or events, provider commercial policy, canonical source-domain state, secrets-store internals, or repository-wide observability.

## 5. Solution Architecture
### 5.1 Technical Elements
| Element | Responsibility | Failure / Ownership Boundary |
|---|---|---|
| Connector Profile Registry | Holds technical configuration references, provider/environment scope and the effective upstream-approved configuration version. | Does not create upstream `Draft`/`Approved`/`Active`/`Suspended`/`Retired` facts. |
| Credential Binding | Resolves D03-managed credential/certificate references without persisting secret material in connector configuration. | Credential failure prevents operation; no weaker fallback is permitted. |
| Effective-Configuration Loader | Activates only the configuration version made effective by authoritative upstream governance. | Cannot self-approve, roll back or retire a configuration. |
| Health Evaluator | Classifies connector/provider runtime health as E01 `Healthy`, `Degraded` or `Unavailable` from technical signals. | Health is operational evidence, not a business lifecycle state. |
| Isolation Guard | Applies bounded invocation, provider-aware pressure handling and failure containment. | One provider failure is contained to the smallest safe scope. |
| Runtime Withdrawal / Re-entry Control | Stops new connector work when authoritative configuration is suspended/retired or when safe operation is unavailable; re-entry requires authoritative eligibility plus E01 health validation. | Does not manufacture a business restoration decision. |

### 5.2 Configuration-to-Runtime Flow
```text
Authoritative upstream connector-governance decision
→ resolve effective approved configuration/version
→ validate environment + Tenant/Institution binding
→ resolve D03 credential reference
→ load provider adapter configuration
→ evaluate E01 technical health
→ admit or withhold connector runtime work
→ emit E05 health/configuration evidence
```

Upstream rollback establishes which eligible prior configuration becomes newly effective; C03 loads that governed effective version without rewriting historical versions.

### 5.3 Material Failure Model
| Failure Class | Required Technical Outcome | Forbidden Outcome |
|---|---|---|
| Missing/invalid effective configuration | Withhold new work and expose technical unavailability. | Auto-approve or invent a replacement configuration. |
| Credential resolution failure | Withhold provider call and emit protected evidence. | Credential downgrade or embedded-secret fallback. |
| Provider degradation/unavailability | Apply E01-compatible isolation and declared degraded/unavailable behavior. | Unbounded retry storm or repository-wide outage. |
| Ambiguous in-flight effect | Preserve correlation for owning-domain/provider reconciliation. | Blind duplicate non-idempotent execution. |
| Upstream suspension/retirement | Withdraw new runtime work and preserve in-flight/evidence disposition according to governed contracts. | Continue new work solely because local health is `Healthy`. |

## 6. Architecture & Technology Decisions
| ID | Decision | Selected Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Configuration realization | Environment-scoped, version-addressable runtime configuration derived from the effective upstream-approved version. | `L4-H-073-BH-001`–`BH-008`; corpus boundary. | Shared C03 ADR required before approval if the reusable family pattern is retained. |
| AD-002 | Credential model | Reference D03-owned secrets/certificates; never embed secret material in connector configuration. | D03 ownership; `L4-H-073-BH-010`. | `ADR-L5-0002` |
| AD-003 | Health model | Reuse E01 `Healthy` / `Degraded` / `Unavailable`; do not create a parallel connector business lifecycle. | `ADR-L5-0014`. | `ADR-L5-0014` |
| AD-004 | Recovery | Runtime re-entry requires authoritative configuration eligibility plus health validation. | `ADR-L5-0014`; ownership preservation. | Shared C03 ADR may specialize. |

## 7. Security, Privacy, Quality & Operational Impact
Configuration/effective-version changes and credential-binding changes require upstream authority plus D02/D03 controls where applicable. Telemetry exposes effective configuration reference, health, last contact, failure class, backlog/pressure and recovery progress without leaking credentials or sensitive provider payloads.

## 8. Architecture Conformance Requirements
- `CR-001`: C03 shall not create a second connector business lifecycle; upstream connector-governance states/events remain authoritative.
- `CR-002`: Runtime execution shall use only an authoritative eligible/effective connector configuration.
- `CR-003`: Secret material shall not be stored in connector configuration.
- `CR-004`: Runtime health shall use E01 health semantics and remain distinct from upstream connector-governance state.
- `CR-005`: One provider failure shall not force unrelated connectors or ERP capabilities unavailable.
- `CR-006`: Re-entry after failure shall validate both authoritative configuration eligibility and dependency health.
- `CR-007`: Rollback realization shall load the newly governed effective configuration without rewriting historical configuration versions.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Required Resolution |
|---|---|---|---|---|
| OD-001 | `L4-H-073` is Draft, so its BH mappings cannot yet be `Validated`. | Upstream | Yes for Architecture Approved absent explicit governance authorization. | Approve/review L4-H-073 or record governance authorization for approval-level use. |
| OD-002 | Shared C03 connector architecture ADR. | ADR | Yes for Architecture Approved if reusable family decisions remain. | Allocate/evaluate through the canonical L5 ADR Register; do not invent an ADR number locally. |
| OD-003 | Numeric timeout/retry/circuit thresholds. | Operations | No. | Bind only from approved provider/operational profiles. |

## 10. Traceability Summary
**Specified L4 applicability:** `L4-H-073-BH-001`–`008`, `010`–`012`.  
**Peers:** C03-001, C01, D03, E01, E05, A04.  
**Decisions:** AD-001–004. **Conformance:** CR-001–007.  
**Blocking:** OD-001, OD-002.
