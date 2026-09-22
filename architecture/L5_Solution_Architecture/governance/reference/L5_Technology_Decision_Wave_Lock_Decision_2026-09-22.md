# SutraERP L5 — Technology Decision Wave Lock Decision — 2026-09-22

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architecture Level:** L5 — Solution Architecture Governance  
**Decision Date:** 2026-09-22

## 1. Decision
The Lead Architect authorizes `ADR-L5-0019` through `ADR-L5-0033` as canonical **Locked** L5 technology decisions. They change implementation architecture only and preserve all L1–L4 business architecture.

## 2. Locked Positions
Axum+Tokio; PostgreSQL 18.x+SQLx; HTTPS REST/JSON+OpenAPI 3.2; NATS JetStream; BPMN 2.x+Flowable OSS 8.x; Keycloak 26.x; OpenSearch; S3-compatible storage+Ceph RGW reference; OpenBao+rustls; OCI+Kubernetes clustered profile with simple non-clustered OCI profile; OpenTelemetry+Prometheus/Loki/Tempo/Grafana; GitHub Actions+GHCR+Cosign/Sigstore; CEL; rust_decimal; external payment/banking/communication provider pluggability with no repository-wide vendor default.

## 3. Controlled Amendment Rule
Already-Locked specifications affected by these decisions may receive a controlled minor-version synchronization and remain Locked under this decision. Draft/Review specifications retain their existing lifecycle status; this decision locks the ADR, not the full corpus.

## 4. Explicitly Deferred
The following remain open: scheduling/optimization solver; universal latency/throughput/concurrency/capacity numbers; RPO/RTO and retention durations; mobile client platform; cache product where still open; exact Kubernetes distribution/cloud provider/non-clustered OCI runtime; institution-specific external vendors; Final L5 Corpus Lock.

## 5. Integrity Rules
Technology selection never transfers canonical business ownership. Flowable is a governed non-Rust runtime exception, not a second business platform. Search and telemetry stay non-authoritative. D04 audit evidence stays separate from E05 telemetry. External providers stay behind C03/C04 adapters.

## 6. Lock Statement
**NEW LOCKED ADRS:** 15  
**ADR RANGE:** `ADR-L5-0019` … `ADR-L5-0033`  
**FINAL L5 CORPUS LOCK:** NOT AUTHORIZED  
**STATUS:** LOCKED — ACTIVE
