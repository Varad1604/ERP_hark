# Connector Lifecycle & Failure Isolation Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Connector configuration/lifecycle, credentials, health and failure isolation.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-01`; `L5-D-03`; `L5-E-01`; `L5-E-05`; `L5-A-04`  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0004`; `ADR-L5-0009`; `ADR-L5-0014`

## 2. Purpose, Scope & Non-Goals
Defines the technical lifecycle of a provider connector: environment-scoped configuration, provider/tenant binding, credential resolution, activation, health assessment, degradation, suspension, recovery and retirement. It does not own provider business policy, business state, or source-domain recovery outcomes.

## 3. Upstream Applicability & Constraints
The frozen corpus assigns connector lifecycle and failure isolation to this specification. C03-001 supplies the common connector boundary; D03 owns secrets/certificates; E01 owns repository resilience; E05 owns observability.

Exact L4 BH allocation is not guessed and remains `OD-001`.

## 4. Technical Ownership & Dependencies
**Owns:** connector technical profile, provider endpoint/environment binding, configuration version/effective state, health classification, connector-local isolation, lifecycle transitions needed only for technical operation.  
**Consumes:** A04 Tenant/Institution isolation; D03 credentials; E01 degradation/recovery; E05 telemetry; C01 compatibility/idempotency.  
**Must not own:** provider commercial policy, business approval, canonical source-domain state, secrets store internals, global observability platform.

## 5. Solution Architecture
### 5.1 Technical Elements
| Element | Responsibility |
|---|---|
| Connector Profile Registry | Stores governed technical configuration references, provider/environment scope and active version. |
| Credential Binding | Resolves D03-managed credential/certificate references without persisting secret material in connector config. |
| Health Evaluator | Classifies provider/connector reachability and dependency health from technical signals. |
| Isolation Guard | Applies bounded invocation, throttling awareness and circuit/failure containment. |
| Configuration Promotion Control | Separates sandbox/test/production configuration and requires attributable promotion. |
| Retirement Control | Prevents new work, drains/settles in-flight work, preserves evidence, then disables connector. |

### 5.2 Lifecycle
```text
Configured → Validated → Enabled → Healthy/Degraded/Unavailable → Recovering → Enabled
                              ↘ Suspended → Retired
```
These are connector technical states only; they do not redefine any business lifecycle.

### 5.3 Failure Rules
- No unbounded retries.
- No credential fallback to weaker authentication.
- Provider outage shall isolate that connector and preserve unrelated capability.
- Ambiguous in-flight effects remain correlated for reconciliation.
- Configuration changes are versioned and attributable; hardcoded production endpoints are prohibited.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Configuration model | Environment-scoped, versioned, provider + Tenant/Institution-aware configuration. |
| AD-002 | Credential model | Reference D03-owned secrets/certificates; never embed secrets in config. |
| AD-003 | Failure containment | Provider-aware bounded calls and circuit/isolation semantics, with numeric values deferred. |
| AD-004 | Recovery | Explicit health validation before re-enablement; no immediate blind recovery. |

## 7. Security, Privacy & Operations
Configuration changes, enable/disable actions and credential-binding changes require privileged authorization and auditable evidence. Telemetry exposes health, last contact, failure class, backlog/pressure and recovery progress without leaking sensitive payloads.

## 8. Architecture Conformance Requirements
- `CR-001`: Connector configuration shall be versioned, environment-scoped and attributable.
- `CR-002`: Secret material shall not be stored in connector configuration.
- `CR-003`: One provider failure shall not force unrelated connectors or ERP capabilities unavailable.
- `CR-004`: Connector health shall be observable through E05-compatible signals.
- `CR-005`: Recovery shall validate dependency health before returning the connector to normal service.
- `CR-006`: Retirement shall preserve evidence and prevent orphaned in-flight work from being silently discarded.

## 9. Open Decisions & Deferred Design
| ID | Item | Blocking? | Resolution |
|---|---|---|---|
| OD-001 | Exact L4 BH mappings. | Yes for Architecture Approved. | Complete traceability extraction. |
| OD-002 | Numeric timeout/retry/circuit thresholds. | No. | Define only from approved operational/provider profiles. |

## 10. Traceability Summary
**Corpus:** `L5-C-03-002`. **Peers:** C03-001, C01, D03, E01, E05, A04. **Decisions:** AD-001–004. **Conformance:** CR-001–006.
