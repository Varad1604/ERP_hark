# External Integration & Connector Architecture Baseline

## 1. Specification Metadata
**Specification ID:** `L5-C-03-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Common external integration/connector boundary and ownership-preserving principles.  
**Technical Architecture Owner:** `L5-C-03 — External Integration & Connector Architecture`  
**Assigned Author:** Pallav  
**Version:** `1.0.2`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-01`; `L5-C-02`; `L5-D-03`; `L5-E-04`; `L5-E-05`; `L5-A-04`; applicable source-domain L5 architecture  
**Referenced By:** `L5-C-03-002`–`L5-C-03-009`  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0004`; `ADR-L5-0009`; `ADR-L5-0014`; `ADR-L5-0016`  
**Governance Authorization:** `L5_Pallav_Stage5_Architecture_Approval_Decision_2026-09-16.md`  
**Governing Sources:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Candidate Document Corpus Register 105; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology; L5 ADR Register; L5 Broad Authoring Authorization.

## 2. Purpose, Scope & Non-Goals
Defines the reusable technical boundary through which SutraERP communicates with external providers, government platforms, financial networks, education systems, file-exchange partners and physical/edge systems while preserving canonical ownership.

**In scope:** connector/adaptor boundary, provider protocol adaptation, technical validation, provider-model mapping, request/callback correlation, Tenant/Institution context propagation, failure isolation, evidence handoff and provider-profile isolation.

**Out of scope:** source-domain business facts/rules/lifecycles, Finance state, academic facts, statutory business decisions, device-originated business decisions, provider commercial policy, named integration products, and unsupported numeric timeout/retry/rate-limit targets.

## 3. Upstream Applicability, Constraints & Assumptions
### 3.1 Canonical Upstream Reference Matrix
| Source | Version / Status | Exact Reference | What It Establishes | L5-C-03-001 Use |
|---|---|---|---|---|
| L5 coverage decision | 1.0.0 / Architecture Approved | §5, §6, §9 | Reusable integration boundary; bulk/file and device integration are distinct reusable technical responsibilities; no canonical business ownership transfer. | Establishes the family purpose and ownership-preserving integration boundary. |
| L5 corpus | current approved population | `L5-C-03-001`–`009` | Frozen C03 responsibilities. | Fixes family decomposition and prevents split/merge drift. |
| `ADR-L5-0016` | Architecture Approved | shared C03 connector pattern | Shared adapter, configuration, credential, correlation and failure-isolation pattern. | Governs reusable C03 technical structure. |
| L4 | 1.0.1 / Draft | `L4-H-073-BH-001`–`012` | Governed external connector configuration/effective history while authentication facts remain upstream-owned. | Requires a connector realization boundary without duplicating H10 business lifecycle. |
| L4 | 1.0.1 / Draft | `L4-H-059-BH-003`–`008` | External submission, acknowledgement/outcome and evidence must preserve reporting ownership. | Requires transport/correlation/evidence handoff without owning regulatory completion. |
| L4 | 1.0.0 / Draft | `L4-A-036-BH-001`–`008` | Governed bulk import requires authorization, structure/schema validation, semantic-owner handoff and evidence. | Requires technical intake/validation/staging boundary specialized by C03-008. |
| L4 | 1.0.0 / Draft | `L4-F-061-BH-003`–`015` | External banking evidence may be failed/returned/uncertain; Treasury remains authoritative. | Requires ambiguous external outcomes to remain evidence for owner reconciliation. |
| ADR | Architecture Approved | `ADR-L5-0014` | Shared health, containment and validated recovery semantics. | Constrains provider failure isolation and return to service. |

The cited L4 specifications remain `Draft`. Their identified behavior is explicitly governance-authorized for this Stage-5 approval by `L5_Pallav_Stage5_Architecture_Approval_Decision_2026-09-16.md`. Canonical L4→L5 relation state therefore remains `Specified`, not `Validated`, until the locked traceability methodology's normal source-status conditions are met.

### 3.2 Authoritative Constraints
- External providers, adapters and connectors shall not become canonical owners of SutraERP business facts.
- Provider-specific representations shall be isolated at the connector/mapping boundary rather than force source-domain model changes.
- Tenant and Institution context remains explicit where applicable and consumes A04 controls.
- Identity, authorization, secrets, cryptography, audit and observability remain owned by their canonical peer specifications.
- External integration shall not directly modify another technical owner's persistence.
- Technical health/degradation shall use E01 semantics and shall not be represented as a business lifecycle state.

## 4. Technical Ownership, Boundaries & Dependencies
### Owns
- Shared connector/adaptor execution boundary.
- Provider protocol and representation adaptation.
- Request/callback correlation and technical outcome classification.
- Provider schema-to-internal integration-contract mapping boundary.
- Common external-dependency health and failure-containment integration points.

### Consumes / Collaborates With
| L5 Specification | Capability | Boundary |
|---|---|---|
| `L5-C-01-*` | Synchronous contracts, compatibility, technical error/idempotency | C03 consumes contract rules; it does not redefine them. |
| `L5-C-02-*` | Durable asynchronous messaging | C03 consumes messaging where applicable; canonical business events/publishers remain upstream-owned. |
| `L5-D-01-*`, `L5-D-02-*`, `L5-D-03-*` | Service identity, authorization, trust, secrets and transport protection | C03 owns no parallel IAM/crypto/secrets capability. |
| `L5-A-04-*` | Tenant/Institution isolation | Context is carried/enforced at connector boundaries. |
| `L5-E-01-*`, `L5-E-05-*` | Resilience and observability | C03 exposes dependency health/telemetry and follows declared degradation/recovery semantics. |
| `L5-E-04-*` | Runtime/deployment | Connector realization must remain compatible with supported on-premises-primary and cloud-capable deployment. |

### Must Not Own
Source-domain business rules/facts/lifecycles, canonical financial/academic/identity outcomes, canonical business events, regulatory meaning, or repository-wide security/observability/deployment policy.

## 5. Solution Architecture
### 5.1 Technical Realization Model
| Element | Responsibility | Input / Context | Output / Handoff | Failure Boundary |
|---|---|---|---|---|
| Connector Entry Boundary | Accepts an authorized internal invocation or trusted inbound exchange. | Contract/event context, Tenant/Institution, provider profile. | Validated connector work item. | Rejects unsupported, unauthenticated or unauthorized technical input. |
| Provider Adapter | Encapsulates provider-specific protocol, endpoint and payload behavior. | Governed internal integration request. | Provider request / normalized response. | Provider failure remains adapter-scoped. |
| Credential Resolver | Obtains D03-owned credential/certificate reference/material at runtime. | Provider + applicable scope. | Authorized provider call context. | No insecure fallback when unavailable. |
| Mapping Boundary | Converts provider representation to/from governed integration contracts. | Provider schema/payload. | Normalized technical representation. | Incompatibility is rejected/quarantined; canonical models are not rewritten. |
| Correlation & Evidence Hook | Preserves operation identity, provider reference, time and technical outcome. | Request/callback lifecycle. | D04/E05-compatible evidence/telemetry handoff. | Evidence loss never fabricates business completion. |
| Dependency Health Guard | Integrates bounded-call, provider-pressure and failure-isolation behavior with E01. | Dependency health/call outcome. | Scoped technical health/degradation signal. | Prevents provider failure from cascading into unrelated capabilities. |

### 5.2 Primary Technical Flow
```text
Approved internal request / trusted external input
→ Connector Entry Boundary
→ authenticate + authorize + resolve required Tenant/Institution context
→ resolve authoritative provider/effective connector profile + D03 credential
→ adapt governed integration contract to provider representation
→ invoke/receive using the provider-authorized transport
→ classify technical outcome + correlate provider reference
→ validate/normalize response or callback evidence
→ hand evidence/result to the canonical owning capability
→ emit E05 telemetry / D04-compatible technical evidence
```

A redirect, transport acknowledgement, device signal or provider callback is technical evidence only unless the applicable authoritative business behavior defines the corresponding business outcome.

### 5.3 Material Failure Model
| Failure Class | Required Technical Outcome | Forbidden Outcome | Recovery / Owner |
|---|---|---|---|
| Authentication/authorization failure | Reject and preserve protected technical evidence. | Credential downgrade or anonymous fallback. | D01/D02/D03 + C03. |
| Timeout/ambiguous external outcome | Preserve correlation and return unresolved technical disposition. | Invent business success/failure or blindly duplicate non-idempotent effect. | C03 + canonical owner reconciliation. |
| Provider rate/availability pressure | Apply provider-aware containment and E01 degradation/unavailability semantics. | Unbounded retry storm. | C03/E01. |
| Provider schema incompatibility | Reject/quarantine incompatible representation. | Change canonical domain model to fit provider. | C03/C01 compatibility governance. |
| Callback/replay duplication | Apply contract-supported authenticity/replay/idempotency controls. | Duplicate technical/business effect. | C03/C01 + owner. |

### 5.4 Protocol / Provider Profile Boundary
C03 does not define one universal protocol. Each connector profile shall use only provider-authorized and repository-governed transport/authentication mechanisms. Where upstream architecture explicitly supports REST APIs, webhooks, secure file exchange, events or OAuth2/OIDC/mTLS-style trust mechanisms, C03 realizes them behind provider adapters without converting those examples into a repository-wide mandatory product/protocol selection. Any shared mandatory product, runtime, gateway or protocol choice follows ADR governance.

## 6. Architecture & Technology Decisions
| ID | Decision | Selected Position | Basis / Derivation | ADR |
|---|---|---|---|---|
| AD-001 | Integration structure | Shared C03 boundary with provider-specific adapters; source domains retain business ownership. | Approved coverage/corpus boundary + exact L4 exemplars. | `ADR-L5-0016` |
| AD-002 | Provider-model isolation | Mapping/anti-corruption boundary prevents provider schemas from becoming canonical domain models. | Ownership preservation; L4-A-036/H-059/H-073 obligations. | `ADR-L5-0016` |
| AD-003 | Protocol posture | Provider-authorized governed portfolio; no single protocol/product selected here. | Upstream variability + technology-selection governance. | `ADR-L5-0016` for shared pattern; separate ADR only if a material product is selected. |
| AD-004 | Failure containment | Ambiguous outcomes remain unresolved; dependency failures use E01 containment/health semantics. | F-061 + `ADR-L5-0014`. | `ADR-L5-0014`; `ADR-L5-0016` |
| AD-005 | Evidence boundary | Connector evidence supports traceability but does not become canonical business truth. | H-059/F-061 ownership constraints. | `ADR-L5-0016` |

## 7. Security, Privacy, Quality & Operational Impact
Connector credentials consume D03 and shall not be committed in source-controlled configuration. Sensitive provider payloads are minimized in telemetry. Production and non-production provider profiles/credentials remain isolated where applicable. Capacity concerns include provider quotas, concurrency, backlog, payload size and recovery throughput; unsupported numeric values are not fixed here.

## 8. Architecture Conformance Requirements
- `CR-001`: Every C03 external integration shall enter through a governed connector/adaptor boundary rather than directly modifying another owner's persistence.
- `CR-002`: Provider-specific representations shall not become canonical SutraERP domain models.
- `CR-003`: Connector identities/credentials shall consume D-family controls and least-privilege authorization.
- `CR-004`: Ambiguous external outcomes shall remain unresolved until authoritative reconciliation; C03 shall not guess a business outcome.
- `CR-005`: Provider failure shall be contained according to E01 semantics and shall not automatically collapse unrelated ERP capability.
- `CR-006`: Inbound callbacks/exchanges shall use provider-supported authenticity/replay/duplicate controls where applicable.
- `CR-007`: Named product/vendor/shared protocol selection remains non-canonical until governed selection/ADR approval.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Required Resolution |
|---|---|---|---|---|
| OD-001 | Draft-L4 relation promotion from `Specified` to `Validated`. | Traceability | No for this Architecture Approved specification. | Revalidate after applicable L4 approval under the locked traceability methodology. |
| OD-002 | Named integration gateway/platform/product or repository-wide mandatory protocol profile. | Technology | No until such selection is required. | Use Technology Selection & ADR Methodology if proposed. |

## 10. Traceability Summary
**Approved purpose authority:** L5 Coverage Audit Decision §5/§6/§9; frozen `L5-C-03-001` responsibility; `ADR-L5-0016`; Stage-5 approval decision 2026-09-16.  
**Governance-authorized / Specified L4 exemplars:** `L4-H-073-BH-001`–`012`; `L4-H-059-BH-003`–`008`; `L4-A-036-BH-001`–`008`; `L4-F-061-BH-003`–`015`.  
**Material peers:** C01, C02, D01/D02/D03, A04, E01, E04, E05.  
**Decisions:** AD-001–005. **Conformance:** CR-001–007. **Blocking:** none.
