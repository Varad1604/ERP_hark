# ADR-L5-0043 — Deployment Toolchain, Environment & Stateful Platform Profile

**ADR ID:** `ADR-L5-0043`  
**Title:** Deployment Toolchain, Environment & Stateful Platform Profile  
**Scope:** Repository  
**Decision Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-E-04-001` through `L5-E-04-005`; constrains F01/E03/E05 and stateful supporting-platform deployment  
**Related ADRs:** `ADR-L5-0028`; `ADR-L5-0030`; `ADR-L5-0040`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

ADR-L5-0028 already establishes OCI packaging, Kubernetes for clustered production, a simpler non-clustered OCI profile, and provider-neutral cloud deployment. Backend readiness requires a concrete deployment toolchain/profile without selecting one cloud provider or one Kubernetes distribution.

## 2. Decision

1. **Kubernetes distribution:** canonical architecture targets upstream Kubernetes APIs and remains distribution-neutral. RKE2, managed Kubernetes offerings and other conformant distributions are deployment-profile choices rather than repository-wide mandates.
2. **Small/non-clustered on-premises runtime:** the default non-clustered OCI profile uses **Podman** and Compose-compatible/container-unit orchestration where appropriate. Kubernetes is not required for a small installation solely for architectural uniformity.
3. **Cluster ingress/gateway:** clustered deployments use **Kubernetes Gateway API** with **Envoy Gateway** as the default gateway implementation. It owns transport routing/TLS/cross-cutting edge mechanics only; it does not own business routing/authorization semantics.
4. **Infrastructure as Code:** **OpenTofu** is the canonical provider-neutral Infrastructure-as-Code tool for managed infrastructure/environment provisioning. Provider modules may differ by deployment profile.
5. **Kubernetes application packaging:** **Helm charts** are the canonical Kubernetes deployable-configuration/package mechanism. Chart configuration does not replace E04 typed configuration, OpenBao secret authority or F01 artifact identity.
6. **GitOps:** no Argo CD, Flux or other GitOps controller is mandatory initially. F01-controlled promotion/deployment pipelines remain authoritative until an approved operational requirement justifies a dedicated GitOps controller.
7. **Configuration:** runtime configuration is external to immutable application artifacts and is represented as versioned typed configuration with environment/profile overrides. Secret values are never stored in ordinary configuration and are resolved/injected through the D03/OpenBao boundary.
8. **Environment model:** the default governed environment progression is **Local/Development → Integration/Test → Staging/Pre-production → Production**. Production is isolated from non-production credentials/data/authority. CI may create ephemeral verification environments without redefining these governance stages.
9. **PostgreSQL operation:** PostgreSQL is an independently managed stateful platform service, never embedded inside application containers. For Kubernetes HA/clustered on-premises operation, **CloudNativePG** is the default PostgreSQL operator. Managed cloud PostgreSQL may substitute within an approved deployment profile while preserving B02 architecture.
10. **NATS operation:** NATS/JetStream is independently operated from application processes; durable messaging profiles use persistent JetStream storage and C02 recovery/observability controls.
11. **Supporting platforms:** Keycloak, Flowable, OpenBao, OpenSearch, observability backends, Ceph RGW where used, and other selected supporting platforms run as independently managed runtime components behind their canonical contracts; they are not bundled into the Rust application image.
12. **Provider neutrality:** cloud-provider-specific managed services are optional deployment realizations and shall not become mandatory repository dependencies without a separate ADR.

## 3. Conformance Rules

- Deployment topology shall not create new business/domain ownership.
- A Kubernetes namespace, Helm chart, Podman unit or cloud resource is not a canonical domain boundary.
- Environment configuration changes are attributable/versioned and secrets remain D03-owned.
- Stateful supporting services have independent backup/recovery requirements coordinated by E03.
- Managed-service substitution must preserve applicable product contract/behavior and recovery/security obligations.
- Envoy Gateway policy shall not become a second D02 authorization engine.
- Infrastructure state/backends for OpenTofu are protected operational assets and use D03/D02 access controls.

## 4. Consequences

The repository has one deployable operations baseline while retaining on-premises and multi-cloud portability. Smaller installations avoid unnecessary Kubernetes complexity; clustered installations share Kubernetes/Gateway/Helm conventions.

## 5. External Evidence

- OpenTofu documentation: https://opentofu.org/docs/
- Envoy Gateway / Kubernetes Gateway API: https://gateway.envoyproxy.io/docs/concepts/gateway-api/
- Podman documentation: https://docs.podman.io/
- Helm documentation: https://helm.sh/docs/
- CloudNativePG documentation: https://cloudnative-pg.io/documentation/

## 6. Approval Record

**Decision selected:** Kubernetes distribution-neutral baseline; Podman non-clustered on-prem profile; Envoy Gateway + Gateway API; OpenTofu; Helm; GitOps optional; typed external config/OpenBao secrets; four-stage governed environment progression; independently operated PostgreSQL/NATS/supporting platforms; CloudNativePG as default on-prem Kubernetes PostgreSQL operator.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
