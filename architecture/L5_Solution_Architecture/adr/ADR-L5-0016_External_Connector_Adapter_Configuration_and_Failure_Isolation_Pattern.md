# ADR-L5-0016 — External Connector Adapter, Configuration & Failure-Isolation Pattern

**ADR ID:** `ADR-L5-0016`  
**Title:** External Connector Adapter, Configuration & Failure-Isolation Pattern  
**Scope:** Shared / Family  
**Decision Owner:** L5-C-03 — External Integration & Connector Architecture  
**Assigned Author:** Pallav  
**Reviewers:** Controlled L5 Stage-5 architecture remediation review  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-16  
**Affected L5 Specifications:** `L5-C-03-001`–`L5-C-03-009`  
**Related ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0004`; `ADR-L5-0009`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0013`; `ADR-L5-0014`  
**Supersedes:** None  
**Superseded By:** None

---

## 1. Decision Context

SutraERP requires a reusable external-integration pattern across payment gateways, government/statutory portals, APAAR/DigiLocker/credit exchange, external education providers, banks/financial networks, bulk/file exchange and device/edge integrations. Without a shared pattern, each connector could independently define provider coupling, configuration authority, credential handling, retry/failure behavior or ownership semantics.

The frozen L5 corpus assigns these technical responsibilities to `L5-C-03`, while upstream business domains retain canonical business facts, lifecycle decisions, events and outcomes.

## 2. Scope & Affected Architecture

This ADR governs the shared C03 technical pattern for:

- provider-specific adapters behind a stable internal integration boundary;
- anti-corruption mapping between provider contracts and SutraERP contracts;
- effective connector configuration realization;
- D03-owned credential references;
- Tenant/Institution/environment scoping;
- correlation, idempotency and ambiguous-outcome handling;
- connector-local failure containment;
- E01-compatible health/degradation/recovery semantics;
- evidence handoff without business-ownership transfer.

It does not select a gateway, bank, LMS, cloud, ESB, API gateway, integration platform, device vendor or transport product.

## 3. Authoritative Constraints

1. Provider systems and connectors do not become canonical owners of SutraERP business facts.
2. Provider-specific schemas/protocols shall be contained behind C03 adapters/mappers.
3. Business-domain acceptance/approval remains with the canonical owner.
4. Connector configuration shall realize the authoritative upstream-approved effective configuration; C03 does not invent a competing business lifecycle.
5. Secret material and machine credentials remain D03-owned; C03 stores only governed references/bindings.
6. Non-human authorization consumes D02/D03 patterns and shall not fabricate Person/Role Assignment semantics.
7. Synchronous calls use C01 contract/idempotency/error semantics; asynchronous exchange uses C02 delivery/versioning semantics where applicable.
8. Background/bulk retry execution remains A08-owned where applicable.
9. Technical health uses E01 `Healthy`, `Degraded`, `Unavailable`; health is not a business lifecycle state.
10. One provider failure shall be contained to the smallest safe technical scope.
11. Ambiguous external outcomes shall remain unresolved until authoritative reconciliation; blind duplicate execution is prohibited where duplicate business effect is possible.
12. The pattern must remain compatible with on-premises-primary and cloud-capable deployment.

## 4. Decision Criteria

The shared pattern must preserve canonical ownership, isolate provider change, support multiple provider protocols without forcing a single vendor/platform, provide safe failure semantics, enable independent provider evolution, integrate with existing C01/C02/D03/E01 controls and avoid duplicated connector governance.

## 5. Alternatives Considered

### A — Direct domain-to-provider integrations
Rejected because provider schemas, credentials, failures and protocol semantics would leak into canonical business domains and duplicate integration logic.

### B — One universal canonical provider model for all integrations
Rejected because payment, banking, education, statutory and device integrations have materially different provider semantics and forcing one universal model would either lose meaning or create an oversized shared schema.

### C — Shared connector framework with provider-specific adapters and ownership-preserving handoff
Selected.

### D — Provider-specific connector applications with no shared architecture rules
Rejected because configuration, health, credential, failure and evidence handling would diverge across C03 specifications.

## 6. Decision

SutraERP shall use a shared connector framework in which provider-specific adapters isolate external contracts while canonical SutraERP owners remain authoritative.

### 6.1 Shared Connector Structure

Each materially integrated provider shall be represented through an adapter boundary containing only the provider-specific protocol/schema/authentication mapping necessary for exchange.

```text
Canonical owner request/fact
→ governed C01/C02 contract
→ C03 connector boundary
→ provider-specific adapter/mapping
→ external provider
→ provider response/event/file/observation
→ normalized technical evidence
→ canonical owner acceptance/reconciliation
```

### 6.2 Configuration Realization

C03 shall load only the effective configuration authorized by the applicable upstream governance. Configuration is version-addressable and environment/Tenant/Institution scoped. A rollback establishes a newly effective eligible configuration; historical configuration is not rewritten.

### 6.3 Credential Binding

Connectors reference D03-governed secrets/certificates/machine credentials. Secret material shall not be embedded in connector configuration, logs or provider evidence records.

### 6.4 Failure Isolation

Provider failure is contained at the provider/connector scope unless correctness requires broader withdrawal. C03 may expose E01 health state, bounded backpressure and retry eligibility, but shall not invent business outcomes.

### 6.5 Ambiguous External Effects

Where an external operation may have been accepted despite a timeout or transport failure, the connector shall preserve correlation and provider references and route the condition to authoritative status resolution/reconciliation before any replay that could duplicate business effect.

### 6.6 Evidence Boundary

Provider acknowledgement, status, callback, file receipt or device observation is technical/external evidence. The canonical owner determines whether that evidence establishes a business outcome.

## 7. Integration Boundaries

### C03 → C01
C01 owns synchronous contract shape, compatibility, errors and idempotency semantics. C03 applies them at provider boundaries.

### C03 → C02
C02 owns durable event delivery, ordering, replay and event-contract compatibility. C03 consumes those mechanisms for asynchronous integration.

### C03 → D02/D03
D02 owns authorization policy; D03 owns secrets, certificates, machine credentials and cryptographic controls. C03 owns only connector binding and use.

### C03 → E01/E05
E01 owns health/degradation/recovery semantics; E05 owns observability infrastructure. C03 emits connector/provider telemetry and evidence.

### C03 → A08
A08 owns durable background work/retry execution. C03 supplies provider-specific operation semantics and replay constraints.

## 8. Consequences

Positive: provider change is isolated, ownership remains correct, multi-provider integrations can evolve independently, credentials and failures are consistently governed, and the same architecture applies across C03 specializations.

Cost: every provider integration requires an explicit adapter/mapping boundary, correlation/evidence model and operational profile. Provider-specific certification/test environments must be maintained where required.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Adapter becomes second business domain | Keep business acceptance/decision with canonical owner and enforce CRs in C03 specs. |
| Provider schema leaks into internal models | Require anti-corruption mapping at C03 boundary. |
| Retry causes duplicate external effect | Require correlation, idempotency where available and reconciliation before replay of ambiguous operations. |
| Credential leakage | D03-owned references only; redact operational evidence. |
| One provider outage cascades broadly | Connector-local isolation plus E01 health/degradation rules. |
| Configuration lifecycle duplicated | C03 realizes authoritative upstream configuration; it does not create competing approval states/events. |

## 10. Migration / Exit Position

Individual providers may use different protocols, SDKs, file formats or devices if the shared ownership, adapter, configuration, credential, correlation, failure-isolation and evidence invariants remain intact. Replacing the shared connector pattern requires a superseding ADR.

## 11. Traceability

Primary realization: `L5-C-03-001` through `L5-C-03-009`.  
Material upstream examples include connector governance in `L4-H-073`, government publication/exchange in Section H, Finance/Treasury payment and banking behavior, bulk exchange behavior and device/telemetry behaviors. Exact L4→L5 relation validation remains governed by the locked traceability methodology and upstream document status.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** The shared provider-adapter/configuration/failure-isolation pattern is a mandatory Shared / Family ADR trigger under the locked L5 Technology Selection & ADR Methodology. The pattern preserves existing canonical ownership and does not select a vendor or product. User authorization to proceed with the Stage-5 governance closure was provided on 2026-09-16.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| C03 technical decision owner | Recommended | 2026-09-16 | Stage-5 C03 corpus remediation |
| L5 architecture reviewer | PASS | 2026-09-16 | Cross-specification ownership and failure-model review |
| Architecture remediation authorization | Approved | 2026-09-16 | Explicit user instruction to proceed |
