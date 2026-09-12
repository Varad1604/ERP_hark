# L5-D-04-001 — Audit & Evidence Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-D-04-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-04 — Audit & Evidence Architecture  
**Canonical Technical Responsibility:** Define the shared technical audit and evidence capability used to capture, qualify, correlate and make available evidence of material system behavior, while source domains retain ownership of business meaning, business evidence requirements and institutional authority.  
**Technical Architecture Owner:** L5-D-04-001 — Audit & Evidence Architecture Baseline  
**Assigned Author:** Varad  
**Version:** `1.1.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-12`  
**Depends On:** `L5-D-03-001`; `L5-A-04-001`; `L5-A-04-002`; `L5-B-02-003`; `L5-C-02-001`; `L4-A-022`; `L4-A-023`; `ADR-L5-0001`; `ADR-L5-0004`; `ADR-L5-0007`; `ADR-L5-0010`; `ADR-L5-0011`.  
**Referenced By:** `L5-D-04-002`; `L5-D-04-003`; cross-cutting application, workflow, integration, security and computational engine specifications.  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0004`; `ADR-L5-0007`; `ADR-L5-0010`; `ADR-L5-0011`.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the foundational technical architecture for capturing, qualifying, enriching and making audit evidence available across SutraERP. It ensures that material system behavior produces attributable technical evidence without transferring business authority, event ownership or record ownership away from originating domains.

### 2.2 In Scope

- Evidence capture boundaries for synchronous application behavior, asynchronous flows and computational engines.
- Evidence qualification separating mandatory conformance evidence from advisory operational observations.
- Context enrichment binding authoritative Tenant, Institution, Session, Workflow, Channel and Actor references.
- Atomic evidence-intent handoff under ADR-L5-0007.
- Fail-closed behavior where authoritative upstream behavior makes evidence creation a completion condition.
- Read-only evidence retrieval gateway for investigation use.
- Secret/credential exclusion at evidence-admission boundaries.

### 2.3 Explicit Non-Goals

This specification does not own:

- Detailed provenance/causality contracts (`L5-D-04-002`).
- Tamper-evident physical storage, sealing and evidence export (`L5-D-04-003`).
- Business audit policy, business events, business lifecycle states or institutional authority.
- Authentication (`L5-D-01`) or authorization policy (`L5-D-02`).
- Event transport reliability, retry, ordering or dead-letter behavior (`L5-C-02`).
- Database engine/product selection or persistence topology (`L5-B-02`).
- Operational telemetry/APM (`L5-E-05`).

---

## 3. Authoritative Constraints

- **C-001 — Mandatory Evidence Fail-Closed:** where canonical L4 behavior makes audit evidence a completion precondition, evidence-intent failure aborts the host operation.
- **C-002 — Evidence Immutability:** generated audit evidence is append-only; this baseline shall not mutate historical evidence.
- **C-003 — Credential Exclusion:** secrets, private keys, raw bearer tokens and equivalent credentials must not enter evidence payloads.
- **C-004 — Tenant/Institution Scope:** evidence capture and retrieval preserve validated scope under ADR-L5-0004.
- **C-005 — Business Ownership Invariance:** audit records technical evidence of business behavior but does not own, approve or alter the business fact.
- **C-006 — Messaging Ownership:** once an evidence publication intent is handed to L5-C-02, delivery, retry, ordering and dead-letter behavior are exclusively C02-owned.

---

## 4. Technical Ownership & Dependencies

### 4.1 Owns

- Shared audit capture interfaces/interceptors.
- Evidence admission and qualification.
- Context enrichment.
- Mandatory evidence completion guard.
- Evidence deduplication/idempotency at the audit-ingestion boundary.
- Read-only retrieval gateway.
- Publication-intent creation for canonical audit notifications after evidence persistence eligibility is satisfied.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Collaboration | Boundary |
|---|---|---|
| `L5-D-04-002` | Provenance & evidence model | D04-001 consumes conceptual evidence/provenance contracts. |
| `L5-D-04-003` | Tamper-evident storage/export | D04-001 hands off validated evidence; D04-003 owns physical storage/sealing/export. |
| `L5-D-01` | Identity | Supplies authenticated actor/workload references. |
| `L5-D-02` | Authorization | Supplies retrieval/admission authorization decisions. |
| `L5-D-03` | Security/trust | Supplies trust-boundary and secret-handling controls. |
| `L5-A-04` | Tenant/Institution isolation | Supplies validated scope context. |
| `L5-B-02-003` | Transaction/persistence | Realizes atomic local persistence of mandatory evidence intent under ADR-L5-0007. |
| `L5-C-02` | Events & messaging | Owns transport of audit notifications and any delivery retry/order/dead-letter semantics. |

### 4.3 Must Not Own

- Business event publishers or canonical business facts.
- Authorization roles/permissions.
- Durable event delivery mechanics, retry policies, ordering guarantees or dead-letter queues.
- Physical audit storage technology.
- Cross-domain telemetry/monitoring.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Responsibility | Output / Handoff | Failure Boundary | Canonical Owner of Recovery |
|---|---|---|---|---|
| `EvidenceCaptureInterceptor` | Capture a technical observation at a defined execution boundary. | Observation candidate. | Local capture boundary. | D04-001 |
| `AuditEvidenceAdmissionFilter` | Confirm evidence eligibility and remove prohibited secret material. | Admitted evidence candidate. | Reject/drop according to authoritative evidence requirement. | D04-001 / D03 for security containment |
| `AuditContextEnricher` | Bind authoritative Tenant, Institution, actor, session, workflow and correlation references. | Enriched evidence candidate. | Reject if mandatory scope is unresolved. | D04-001 / A04 |
| `MandatoryEvidenceGuard` | Commit mandatory evidence intent atomically with host state change. | Local persistence/outbox intent. | Abort host transaction on failure. | B02 transaction boundary under ADR-L5-0007 |
| `AdvisoryEvidenceBuffer` | Temporarily stage non-conformance-critical observations without converting them into business truth. | Candidate for audit ingestion. | Saturation handled as operational concern; no unbounded growth. | D04-001 with E05 observability |
| `AuditIngestionPipeline` | Deduplicate and construct the D04-002 evidence envelope; coordinate storage handoff. | Validated evidence to D04-003; publication intent after required persistence milestone. | Do not discard mandatory evidence; retain/return failure according to the owning persistence boundary. | D04-003/B02 for persistence, not C02 transport |
| `AuditPublicationHandoff` | Hand canonical `Audit Evidence Generated` publication intent to C02 after required evidence persistence milestone. | Publication request to L5-C-02. | If handoff not accepted, preserve the publication intent under the approved atomic-handoff pattern. | **L5-C-02 owns delivery retry, ordering, recovery and dead-letter behavior after acceptance.** |
| `AuditRetrievalGateway` | Provide governed read-only evidence retrieval. | Scoped evidence/read model response. | Deny on authorization/scope failure. | D02/A04/D04-003 |

### 5.2 Primary Flow

```text
System behavior
    |
    v
EvidenceCaptureInterceptor
    |
    v
AuditEvidenceAdmissionFilter
    |
    v
AuditContextEnricher
    |
    +--> mandatory evidence --> MandatoryEvidenceGuard --> B02 atomic intent
    |
    +--> advisory evidence  --> AdvisoryEvidenceBuffer
    |
    v
AuditIngestionPipeline
    |
    +--> D04-003 storage/sealing
    |
    +--> AuditPublicationHandoff --> C02 transport
```

### 5.3 Messaging Boundary

`AuditPublicationHandoff` owns only the creation and submission of an audit-notification publication intent. It shall not define:

- retry counts or backoff strategy;
- ordering partitions/keys;
- durable subscription semantics;
- dead-letter queues;
- poison-message policy;
- broker recovery behavior.

Those are owned by L5-C-02 and governed by ADR-L5-0010 / ADR-L5-0011 where applicable.

### 5.4 Failure Model

| Failure Class | Required Outcome | Forbidden Outcome |
|---|---|---|
| Mandatory evidence-intent write failure | Abort the host transaction where evidence is a completion condition. | Commit the business change while silently losing mandatory evidence. |
| Prohibited secret detected | Remove/reject prohibited secret material and create minimized security evidence as appropriate. | Persist raw credentials in audit evidence. |
| Tenant context unresolved | Fail closed for mandatory evidence and protected retrieval. | Create orphan/default-tenant evidence. |
| D04-003 storage handoff failure | Preserve the evidence intent/state according to D04-003/B02 persistence architecture and surface failure. | Discard mandatory evidence or report success falsely. |
| C02 publication transport failure after accepted handoff | C02 applies its approved delivery/recovery architecture. | D04-001 invents a parallel retry/dead-letter subsystem. |
| Unauthorized retrieval | Deny and evidence the denied attempt as applicable. | Cross-tenant or unauthorized disclosure. |

---

## 6. Architecture Decisions

- **AD-001 — Mandatory vs Advisory Evidence Separation:** conformance-critical evidence uses an atomic persistence intent; advisory observations may use bounded asynchronous staging.
- **AD-002 — Evidence Immutability:** generated evidence is append-only.
- **AD-003 — Mandatory Evidence Fail-Closed:** mandatory evidence-intent failure aborts the owning operation where L4 makes evidence a completion condition.
- **AD-004 — Capture/Storage Separation:** D04-001 owns capture/admission; D04-003 owns physical tamper-evident storage/export.
- **AD-005 — Credential Redaction:** secret material is excluded before evidence persistence.
- **AD-006 — Messaging Recovery Delegation:** D04-001 creates publication intent; C02 exclusively owns durable event transport, retry, ordering and dead-letter semantics.

---

## 7. Security, Privacy & Trust Boundaries

- Tenant and Institution context is mandatory under ADR-L5-0004.
- Trust-boundary admission follows ADR-L5-0001.
- Credential/key material is never stored in audit evidence.
- Authorization decisions are consumed from D02, not authored by D04.
- Event transport is delegated to C02 rather than duplicated inside D04.

---

## 8. Architecture Conformance Requirements

- **CR-001:** Audit capture shall occur at defined execution boundaries without changing business ownership.
- **CR-002:** Mandatory and advisory evidence shall remain technically distinguishable.
- **CR-003:** Evidence shall bind authoritative Tenant, Institution and actor context.
- **CR-004:** Mandatory evidence shall fail closed when required scope/evidence intent cannot be established.
- **CR-005:** Mandatory evidence intent shall commit atomically with host state change where ADR-L5-0007 applies.
- **CR-006:** D04-001 shall not mutate previously generated evidence.
- **CR-007:** D04-001 shall emit the canonical `Audit Evidence Generated` notification only after the required evidence persistence milestone.
- **CR-008:** D04-001 shall not own retry, ordering, durable subscription or dead-letter mechanics for that notification; these remain L5-C-02 responsibilities.
- **CR-009:** Evidence retrieval shall be read-only and authorization/scope governed.
- **CR-010:** Secret credentials and private key material shall be excluded from evidence payloads.
- **CR-011:** Long-term tamper-evident storage/sealing/export shall remain owned by L5-D-04-003.

---

## 9. Traceability

| L4 / ADR Reference | Technical Realization |
|---|---|
| `L4-A-022-BH-001` | Evidence eligibility and mandatory completion guard. |
| `L4-A-022-BH-002` | Authoritative Tenant/Institution/workflow/session/channel context enrichment. |
| `L4-A-022-BH-006` | Consumes governed audit classification without authorization-policy ownership transfer. |
| `L4-A-022-BH-007` | Immutable evidence creation and deduplication. |
| `L4-A-022-BH-008` | Publication intent for canonical `Audit Evidence Generated` after evidence creation/persistence milestone. |
| `L4-A-022-BH-009` | Makes evidence available for authorized investigation; physical read-model materialization remains D04-003. |
| `L4-A-023-BH-001` | Authorized investigation admission. |
| `L4-A-023-BH-002` | Scoped evidence lookup by reference. |
| `L4-A-023-BH-007` | Read-only investigation view. |
| `ADR-L5-0001` | Trust-boundary admission and secret handling. |
| `ADR-L5-0004` | Tenant/Institution isolation. |
| `ADR-L5-0007` | Atomic state-to-publication intent handoff. |
| `ADR-L5-0010` | C02-owned durable event delivery/recovery. |
| `ADR-L5-0011` | C02-owned event compatibility/versioning where applicable. |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — D04-003 Storage Technology:** remains owned by Stage-3 D04-003/ADR governance.
- **OD-002 — Advisory Capacity/Sizing:** deployment/operations concern; no numeric threshold is invented here.
- **OD-003 — Deduplication Retention Horizon:** remains a governed operational sizing decision; this specification does not invent a duration.
- **OD-004 — Event Delivery Recovery:** closed as an ownership question: L5-C-02 owns retries, ordering and dead-letter handling. D04-001 owns only publication intent and handoff.

**Current State:** Ready for Re-Review
