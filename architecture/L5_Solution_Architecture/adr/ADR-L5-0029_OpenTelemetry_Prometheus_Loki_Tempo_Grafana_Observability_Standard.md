# ADR-L5-0029 — OpenTelemetry, Prometheus, Loki, Tempo & Grafana Observability Standard

**ADR ID:** `ADR-L5-0029`  
**Title:** OpenTelemetry, Prometheus, Loki, Tempo & Grafana Observability Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-E-05 — Observability & Operations Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-E-05-001`…`L5-E-05-004`  
**Related ADRs:** `ADR-L5-0014`; `ADR-L5-0028`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Open standards, Rust support, self-hosting, backend interoperability, operations and provider neutrality.

## 5. Alternatives Considered
OTel+Grafana ecosystem; proprietary APM default; direct vendor instrumentation; remain unselected.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**OpenTelemetry/OTLP + Prometheus/Loki/Tempo/Grafana is the canonical observability stack.**

Runtime instrumentation/transport uses OpenTelemetry/OTLP with Collector. Prometheus is the default metrics backend, Loki logs, Tempo traces and Grafana operational visualization. D04 audit evidence remains separate from telemetry.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-E-05-001`…`L5-E-05-004`. Related authority: `ADR-L5-0014`; `ADR-L5-0028`.  
External evidence: https://opentelemetry.io/docs/collector/ ; https://prometheus.io/docs/ ; https://grafana.com/docs/

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
