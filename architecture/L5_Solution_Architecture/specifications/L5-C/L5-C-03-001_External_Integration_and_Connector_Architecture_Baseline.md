# External Integration & Connector Architecture Baseline

## 1. Specification Metadata
**Specification ID:** `L5-C-03-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Common external integration/connector boundary and ownership-preserving principles.  
**Technical Architecture Owner:** `L5-C-03 — External Integration & Connector Architecture`  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-01`; `L5-C-02`; `L5-D-03`; `L5-E-04`; `L5-E-05`; `L5-A-04`; applicable source-domain L5 architecture  
**Referenced By:** `L5-C-03-002`–`L5-C-03-009`  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0004`; `ADR-L5-0009`; `ADR-L5-0014`; shared C03 connector-pattern ADR required but not yet canonically allocated  
**Governing Sources:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Candidate Document Corpus Register 105; L5 Technology Selection & ADR Methodology; L5 ADR Register; L5 Broad Authoring Authorization.

## 2. Purpose, Scope & Non-Goals
This specification defines the reusable technical boundary through which SutraERP communicates with external providers, government platforms, financial networks, education systems, file-exchange partners and physical/edge systems while preserving upstream canonical ownership.

**In scope:** connector/adaptor boundary, protocol abstraction, provider configuration, request/callback correlation, technical validation, provider-model mapping, tenant/institution context propagation, failure isolation, evidence handoff and protocol-selection posture.

**Out of scope:** ownership of external-facing business facts, Finance state, academic facts, identity facts, statutory business decisions, device-originated business decisions, provider-specific business policy, product/vendor selection, and numeric timeout/retry/rate-limit targets.

## 3. Upstream Applicability, Constraints & Assumptions
### 3.1 Canonical Upstream Reference Matrix
| Source | Exact Reference | What It Establishes | L5-C-03-001 Use |
|---|---|---|---|
| L5 corpus | `L5-C-03-001` boundary | Common connector architecture without ownership transfer. | Defines this specification's canonical scope. |
| Repository baseline | Hybrid communication and canonical-owner rules | Cross-domain interaction does not transfer source-domain ownership. | External systems remain technical peers, not canonical owners. |
| L5 peer | `L5-C-01` | Governed synchronous contract, compatibility, error and idempotency architecture. | Connector synchronous calls consume C01 rather than redefine it. |
| L5 peer | `L5-C-02` | Durable event/messaging and compatibility architecture. | Connector asynchronous handoffs consume C02 where applicable. |
| L5 peer | `L5-D-03` | Trust, cryptography, secrets, certificates and segmentation. | Connector trust controls consume D03. |
| ADR | `ADR-L5-0009` | Non-human principal authorization binding. | Connector/service identities shall be separately authorized. |
| ADR | `ADR-L5-0014` | Repository resilience, health, degradation and recovery. | Connector failure is isolated and externally degraded without collapsing unrelated capability. |

Exact applicable L4 BH allocation across all connector-consuming business scenarios is not fabricated here; it remains a review-gate item in `OD-001`.

### 3.2 Authoritative Constraints
- External providers, adapters and connectors shall not become canonical owners of SutraERP business facts.
- Provider-specific models shall be translated at the connector boundary rather than propagated into canonical internal models.
- Tenant and Institution context shall remain explicit where applicable and shall not be inferred from endpoint naming alone.
- Security, secrets, authorization and audit capabilities are consumed from their canonical L5 owners.
- No protocol, gateway or vendor may require direct modification of another technical owner's internal persistence.

## 4. Technical Ownership, Boundaries & Dependencies
### Owns
- Shared connector/adaptor execution boundary.
- Provider protocol adaptation and transport-level validation.
- Provider request/callback correlation and technical outcome classification.
- Provider schema-to-internal-contract mapping boundary.
- Common external-dependency health and isolation semantics.

### Consumes / Collaborates With
| L5 Specification | Capability | Boundary |
|---|---|---|
| `L5-C-01` | Synchronous contracts, versioning, technical error/idempotency rules | C03 invokes/serves governed contracts; C01 owns contract rules. |
| `L5-C-02` | Durable asynchronous messaging | C03 may publish/consume through C02 but does not redefine event semantics. |
| `L5-D-01/D-02/D-03` | Service identity, authorization, trust, secrets and transport protection | C03 owns no independent IAM/crypto store. |
| `L5-A-04` | Tenant/Institution isolation | Context is propagated and enforced at connector boundaries. |
| `L5-E-01/E-05` | Resilience and observability | C03 emits health/telemetry and applies declared degradation behavior. |
| `L5-E-04` | Runtime/deployment capability | Connector deployment remains compatible with supported on-prem/cloud-capable models. |

### Must Not Own
Source-domain business rules, canonical external-facing facts, financial/academic/identity lifecycles, business reconciliation outcomes, canonical business events, or repository-wide security/observability architecture.

## 5. Solution Architecture
### 5.1 Technical Realization Model
| Element | Responsibility | Input / Context | Output / Handoff | Failure Boundary |
|---|---|---|---|---|
| Connector Entry Boundary | Accepts approved internal invocation or trusted inbound exchange. | Contract/event context, Tenant/Institution, provider profile. | Validated connector work item. | Rejects unauthenticated, unauthorized or malformed technical input. |
| Provider Adapter | Encapsulates provider-specific protocol, endpoint and payload behavior. | Canonical connector request. | Provider request/normalized response. | Provider failures remain adapter-scoped. |
| Credential Resolver | Obtains D03-owned machine credential/secret reference. | Provider + tenant/institution scope. | Runtime credential material/reference. | Credential unavailability prevents call; no insecure fallback. |
| Mapping Boundary | Converts provider representations to/from internal integration contracts. | Provider payload/schema. | Normalized technical representation. | Schema mismatch is quarantined/rejected, not leaked into domain model. |
| Correlation & Evidence Hook | Preserves operation identity, provider reference, timestamps and technical outcome. | Request/callback lifecycle. | D04/E05-compatible evidence/telemetry handoff. | Evidence failure shall not fabricate business completion. |
| Dependency Health Guard | Applies timeout boundary, rate-limit awareness, circuit/failure isolation and health classification. | Provider health and call outcome. | Available/degraded/unavailable technical state for operations. | Prevents repeated provider failure from cascading. |

### 5.2 Primary Technical Flow
```text
Approved internal request / trusted external callback
→ Connector Entry Boundary
→ authenticate + authorize + resolve Tenant/Institution context
→ resolve provider profile + D03 credential
→ map canonical integration request to provider representation
→ invoke provider using approved protocol profile
→ classify technical outcome + correlate provider reference
→ map validated response/evidence
→ hand result to owning business capability
→ emit E05 telemetry / D04-compatible technical evidence
```

A browser redirect, device signal, provider callback or transport acknowledgement is never by itself promoted to authoritative business success unless the owning domain's approved behavior says so.

### 5.3 Material Failure Model
| Failure Class | Required Outcome | Forbidden Outcome | Recovery Owner |
|---|---|---|---|
| Authentication/authorization failure | Reject and record technical evidence. | Credential downgrade or anonymous fallback. | D01/D02/D03 + connector. |
| Timeout/transport ambiguity | Return unresolved technical outcome and preserve correlation. | Invent success/failure or duplicate non-idempotent action. | C03 + owning domain reconciliation. |
| Provider rate limit/unavailability | Isolate provider; honor provider-aware throttling/back-pressure. | Unbounded retry storm. | C03/E01. |
| Provider schema incompatibility | Reject/quarantine incompatible representation. | Mutate canonical internal model to match provider. | C03/C01 compatibility process. |
| Callback replay/duplication | Apply replay/idempotency controls where contract supports them. | Duplicate business effect. | C03/C01 + owning domain. |

### 5.4 Protocol & Standards Posture
When SutraERP controls the contract, HTTPS with JSON is the preferred synchronous representation. OAuth 2.x machine-to-machine authorization is preferred where supported; mTLS is supported where provider/risk requirements demand it. Secure file exchange, webhooks, messaging, provider SDKs and device protocols remain valid behind adapters when required by the external contract. No named gateway, cloud integration service or connector product is selected here.

## 6. Architecture & Technology Decisions
| ID | Decision | Selected Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Integration structure | Shared connector framework plus provider-specific adapters; business domains retain ownership. | Corpus boundary + ownership rules. | Required shared/family ADR; unallocated. |
| AD-002 | Provider model isolation | Anti-corruption mapping at connector boundary. | Defensible L5 derivation needed to preserve canonical models. | Covered by future C03 ADR if material. |
| AD-003 | Protocol posture | Standards-based portfolio; HTTPS/JSON preferred when SutraERP controls contract. | Interoperability without vendor lock-in. | None unless a repository-wide technology is selected. |
| AD-004 | Failure containment | Explicit bounded calls, provider-aware throttling and circuit/failure isolation without invented numeric values. | `ADR-L5-0014` + C03 responsibility. | `ADR-L5-0014`; C03 ADR may specialize. |
| AD-005 | Evidence boundary | Integration evidence is technical evidence, not canonical business audit truth. | Ownership preservation. | None. |

## 7. Security, Privacy, Quality & Operational Impact
Connector credentials are D03-owned and shall never be committed in source/configuration artifacts. Sensitive provider payloads are minimized and redacted from operational telemetry. Production/sandbox endpoints and credentials are isolated by environment. Capacity planning considers provider quotas, concurrent operations, backlog, payload size and recovery throughput without inventing numeric thresholds.

## 8. Architecture Conformance Requirements
| ID | Requirement |
|---|---|
| CR-001 | Every external provider integration shall enter through a governed connector/adaptor boundary rather than directly mutating another owner's persistence. |
| CR-002 | Provider-specific models shall not become canonical SutraERP domain models. |
| CR-003 | Connector identities/credentials shall use D01/D02/D03-governed mechanisms and least privilege. |
| CR-004 | Ambiguous external outcomes shall remain unresolved until governed reconciliation; they shall not be guessed. |
| CR-005 | Provider failure shall be isolated from unrelated ERP capability in accordance with E01 resilience semantics. |
| CR-006 | External callbacks shall be authenticated where provider capability permits and protected against replay/duplicate technical effect. |
| CR-007 | Product/vendor selection shall remain non-canonical until governed ADR/technology selection occurs. |

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Required Resolution |
|---|---|---|---|---|
| OD-001 | Exact L4 BH-to-C03-001 traceability set across connector-consuming scenarios. | Traceability | Yes for Architecture Approved; no for Draft authoring. | Complete repository BH extraction before approval. |
| OD-002 | Shared C03 connector architecture ADR allocation/approval. | ADR | Yes for Architecture Approved because AD-001 is reusable across the family. | Allocate next canonical ADR only through `L5_ADR_Register.md`, then evaluate/approve. |
| OD-003 | Named integration platform/gateway product. | Technology | No. | Keep unselected unless evidence requires a product; then use ADR methodology. |

## 10. Traceability Summary
**Corpus:** `L5-C-03-001` approved responsibility.  
**Material L5 dependencies:** C01 contracts; C02 messaging; D01/D02/D03 trust; A04 isolation; E01 resilience; E04 runtime; E05 observability.  
**Approved ADR constraints:** `ADR-L5-0001`, `0002`, `0004`, `0009`, `0014`.  
**Local decisions:** `AD-001`–`AD-005`.  
**Conformance:** `CR-001`–`CR-007`.  
**Blocking for approval:** `OD-001`, `OD-002`.
