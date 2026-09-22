# Observability & Operations Architecture Baseline

## 1. Specification Metadata
**Specification ID:** `L5-E-05-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-05 — Observability & Operations Architecture  
**Canonical Technical Responsibility:** Operational observability/service operations without treating telemetry as business audit truth.  
**Technical Architecture Owner:** L5-E-05 — Observability & Operations Architecture  
**Assigned Author:** Pallav  
**Version:** `1.1.0`  
**Status:** `Review`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; Candidate Document Corpus Register 105; L5 Technology Selection & ADR Methodology; L5 ADR Register; `L5-E-04-*`; `L5-E-01-*`; `L5-E-02-*`; `L5-C-01-*`; `L5-C-02-*`; `L5-D-03-*`; `L5-D-04-*`; `L5-A-04-*` where Tenant/Institution execution context is consumed.  
**Referenced By:** `L5-E-05-002`, `L5-E-05-003`, `L5-E-05-004`, `L5-F-01-*`, and downstream operational-readiness consumers.  
**Applicable ADRs:** `ADR-L5-0014`; `ADR-L5-0028`; `ADR-L5-0029`.

## 2. Purpose, Scope & Non-Goals
Defines the reusable L5 technical architecture for collecting, transporting, correlating, protecting and consuming operational telemetry across SutraERP runtime components.

**In scope:** instrumentation boundaries; telemetry collection; signal routing; correlation; redaction/filtering; buffering/backpressure; backend abstraction; operational ownership metadata; service topology; operational evidence handoff.

**Out of scope:** canonical business audit truth (`L5-D-04-*`); business KPIs/lifecycles; deployment topology ownership (`L5-E-04-*`); resilience policy (`L5-E-01-*`); business-readiness certification; vendor selection.

## 3. Upstream Applicability, Constraints & Assumptions
The approved Candidate Corpus Register establishes this reusable technical responsibility. Traceability is many-to-many: this specification shall not invent one-to-one BH mappings merely to populate a table. Exact BH links are recorded only when an L4 behavior materially requires an E05 contribution.

### 3.1 Dependency-readiness assessment
The locked dependency-gated execution model requires the consumed peer responsibility to be architecturally stable enough for downstream use; it does not require every peer file to have reached final approval status. For the E05 responsibility used here, the consumed E04 surface is limited to runtime/deployment identity, environment/profile context and deployment-owned boundaries. Those responsibilities are explicit in the current E04 architecture and are not replaced by E05. D03/D04 are consumed only through their canonical security/audit boundaries and approved ADR-backed responsibility assignments.

Accordingly, peer `Draft` status is not by itself a blocker for continued Stage 3 authoring/review. E05 does not consume unresolved product/topology choices from those peers. A material change to any consumed boundary remains a revalidation trigger under L5 change governance.

### 3.2 Material L4 applicability
The following current L4 obligations provide defensible material applicability for the operational-evidence contribution supplied by this specification while preserving L2-A10 governance ownership. Their current L4 status is `Draft`; therefore these relations are **Specified**, not `Validated`, and do not authorize L5 Architecture Approved status until the upstream behavioral authority is approved/authorized under L5 governance.

| L4 Specification | BH ID | E05-001 Technical Contribution | L5 Evidence | Relation State |
|---|---|---|---|---|
| `L4-A-042` | `L4-A-042-BH-005` | Supplies governed operational telemetry evidence references and explicit loss/unavailability semantics when observability evidence is an applicable readiness input; it does not own the governance Assessment or evidence decision. | §§4.1, 5.1, 5.4; CR-001, CR-003, CR-008 | Specified |
| `L4-A-045` | `L4-A-045-BH-001` | Makes attributable operational evidence references resolvable for remediation verification without converting telemetry into audit truth. | §§4.1, 5.1, 5.3; CR-001, CR-010 | Specified |
| `L4-A-045` | `L4-A-045-BH-002` | Provides evidence quality, availability and correlation facts needed to determine whether operational evidence is usable for verification; governance still evaluates closure eligibility. | §§5.3–5.5, 7; CR-003, CR-004, CR-008 | Specified |

No other BH coverage is claimed by this file merely because telemetry may be useful operationally.

## 4. Technical Ownership, Boundaries & Dependencies
### 4.1 Owned technical elements
- **Instrumentation Boundary:** common logical point at which runtime components emit operational signals.
- **Telemetry Context Carrier:** propagates technical correlation, bounded business/workflow correlation and safe Tenant/Institution context.
- **Collection Boundary:** receives local process/runtime signals without coupling producing components to a concrete backend.
- **Telemetry Processor:** performs validation, redaction, enrichment, filtering, sampling/routing and bounded buffering.
- **Signal Router:** separates logs, metrics, traces, health and event-flow diagnostic streams according to signal-specific policy.
- **Backend Abstraction:** isolates producers from product-specific storage/query implementation.
- **Operational Topology Registry View:** correlates component identity, runtime identity, version, environment and declared dependency relationships without owning deployment configuration.
- **Operational Evidence Adapter:** exposes governed references to telemetry-derived evidence for `L5-E-05-004` and other authorized consumers.

### 4.2 Consumed responsibilities
`L5-E-04-*` supplies runtime/deployment identity; `L5-E-01-*` resilience semantics; `L5-E-02-*` capacity/performance dimensions; `L5-C-01-*` and `L5-C-02-*` request/event correlation surfaces; `L5-D-03-*` security controls; `L5-D-04-*` audit boundary; `L5-A-04-*` Tenant/Institution context where authorized.

### 4.3 Boundary rules
E05 never becomes the source of business truth, deployment truth, authorization truth or audit truth. Observability may reference those authorities but cannot redefine them.

## 5. Solution Architecture
### 5.1 Primary execution flow
```text
API / worker / scheduler / broker adapter / persistence adapter / connector
→ instrumentation boundary
→ telemetry context carrier
→ local collection boundary
→ bounded queue/buffer
→ validation + redaction + enrichment + filtering/sampling
→ signal router
→ backend abstraction
→ operational query / dashboard / alert / diagnostic / readiness-evidence consumer
```

The producing component must remain functional when the general telemetry path is unavailable unless a separately owned security/audit control explicitly requires fail-closed behavior.

### 5.2 Telemetry envelope
A signal may carry: timestamp; component/service identity; runtime instance identity where material; version/build identity; environment; severity or signal type; stable event/error code; technical trace/span identifiers; bounded business/workflow correlation identifier where available; safe Tenant/Institution scope; dependency identity; outcome/status; duration/count/value as appropriate. Payload fields are signal-specific and must obey classification/redaction policy.

### 5.3 Correlation model
Technical trace/span IDs are not business identifiers. Business/workflow correlation may bridge multiple bounded traces and asynchronous operations. Tenant/Institution identifiers are included only when required for diagnosis and permitted by data-classification rules. High-cardinality identifiers do not become default metric dimensions.

### 5.4 Failure and backpressure model
1. Producer emits telemetry without synchronously depending on backend availability.
2. Collection boundary uses bounded buffering; buffers cannot grow without limit.
3. Under pressure, configured signal policy determines drop/sample/spool behavior by signal class and criticality.
4. Dropped or delayed telemetry is itself surfaced through operational counters/health where feasible.
5. Backend outage must not silently convert general telemetry into transaction-critical infrastructure.
6. Mandatory audit/security evidence remains governed by `L5-D-04-*` / `L5-D-03-*`, not by this general telemetry failure model.

### 5.5 Trust and isolation model
Production and non-production telemetry are logically isolated. Write paths from application runtime to telemetry collection are separate from privileged query/administration paths. Operational, security and sensitive-data views use least-privilege access. Secrets, credentials, private keys and unrestricted payloads are prohibited from general telemetry.

### 5.6 Runtime/deployment handoff
E05 consumes runtime identity and environment placement from E04. It may require an instrumentation/collector presence or reachable collection boundary, but it does not prescribe packaging, orchestration, node topology or deployment product.

## 6. Architecture & Technology Decisions
- **AD-001 — Multi-signal model:** SutraERP uses coordinated logs, metrics, traces, health and event-flow diagnostics rather than a single-signal observability model.
- **AD-002 — Decoupled collection:** producers emit through a logical collection boundary so application code is not coupled to a concrete telemetry backend.
- **AD-003 — Correlation separation:** technical trace/span IDs remain distinct from business/workflow correlation identifiers.
- **AD-004 — Non-blocking general telemetry:** general observability failure normally cannot fail business processing.
- **AD-005 — Bounded pressure handling:** telemetry buffering is bounded and overload behavior is explicit by signal class.
- **AD-006 — Standards-oriented interoperability:** OpenTelemetry-compatible approaches may be evaluated; no implementation standard is canonical without ADR approval.
- **AD-007 — Locked observability standard:** OpenTelemetry/OTLP + Collector, Prometheus metrics, Loki logs, Tempo traces and Grafana visualization; producers remain backend-decoupled.

## 7. Security, Privacy, Quality & Operations
Telemetry is classified by signal and field. Sensitive values are redacted before leaving the producing trust boundary where feasible. Access is least privilege, privileged queries are attributable, and telemetry administration cannot grant business authorization. Timestamp sources must be sufficiently consistent for cross-component correlation. Signal retention is class-specific and delegated to governed operational/security requirements; this document invents no duration.

## 8. Architecture Conformance Requirements
- **CR-001:** General telemetry shall never be treated as canonical business audit truth.
- **CR-002:** Runtime producers shall not require direct knowledge of a concrete observability backend.
- **CR-003:** Telemetry buffering/backpressure shall be bounded and failure behavior explicit.
- **CR-004:** Technical and business/workflow correlation identifiers shall remain semantically distinct.
- **CR-005:** Secrets, credentials and private keys shall not enter general telemetry.
- **CR-006:** Production/non-production telemetry shall be logically isolated.
- **CR-007:** Tenant/Institution context shall be emitted only where diagnostically necessary and authorized.
- **CR-008:** General telemetry backend failure shall normally not fail business operations.
- **CR-009:** Backend/product selection shall require the applicable ADR/governance decision.
- **CR-010:** E05 shall consume, not redefine, E04 deployment identity, D03 security controls or D04 audit truth.

## 9. Open Decisions & Deferred Design
- **OD-001:** **Resolved:** OpenTelemetry + Prometheus/Loki/Tempo/Grafana under `ADR-L5-0029`.
- **OD-002:** Mandatory repository-wide telemetry implementation profile — requires ADR if standardized; non-blocking while the architecture remains interoperable and product-neutral.
- **OD-003:** Numeric sampling, retention, SLO and capacity targets — delegated to canonical owners/configuration profiles; no universal values are required at Review stage.
- **OD-004:** Current direct BH applicability is specified in §3.2. Relation validation remains blocked by the Draft status of those L4 sources and must be performed only after upstream approval/authorization.
- **OD-005:** Peer dependency drift — current consumed E04/D03/D04 responsibility surfaces are sufficiently stable for Stage 3 Review; any material change to those surfaces requires impact revalidation.

## 10. Traceability Summary
Candidate Corpus Register 105 establishes the canonical E05 responsibility. Technical dependency traceability is explicit to E04/E01/E02/C01/C02/D03/D04/A04.

Specified material L4 contribution relations for this version are:

- (`L4-A-042-BH-005`, `L5-E-05-001`) — Specified
- (`L4-A-045-BH-001`, `L5-E-05-001`) — Specified
- (`L4-A-045-BH-002`, `L5-E-05-001`) — Specified

These relations provide operational-evidence architecture only. They do not transfer L2-A10 readiness-governance ownership, D04 audit ownership, or any L4 behavioral ownership to E05. Because the cited L4 specifications are currently Draft, this L5 file remains `Review`; it shall not claim `Architecture Approved` on their authority.


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0029`  
OpenTelemetry/OTLP with Collector is canonical; Prometheus/Loki/Tempo/Grafana are default backends. D04 audit/evidence remains separate.
