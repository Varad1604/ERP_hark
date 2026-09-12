# L5-B-05-005 — Consent, Purpose-Limitation & Privacy-Control Architecture

## 1. Specification Metadata

**Specification ID:** L5-B-05-005  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Technical enforcement of authoritative consent state/withdrawal, approved purpose, minimization and disclosure constraints; never canonical consent owner  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.0  
**Status:** Draft  
**Last Updated:** 2026-09-12  
**Depends On:** L5-B-05-001 (Classification Baseline); L5-B-05-002 (Masking); L5-B-02 (Persistence); L5-C-02 (Events & Messaging); L5-D-02 (Authorization); L5-D-04 (Audit & Evidence)  
**Referenced By:** L5-C-01, L5-C-03, L5-C-04, L5-B-04, L5-B-06, L5-G-01  
**Applicable ADRs:** ADR-L5-0001 (Canonical Trust Boundary & Segmentation Pattern), ADR-L5-0002 (Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern), ADR-L5-0003 (Contract-Driven Modular Platform with Selective Isolation), ADR-L5-0004 (Tenant and Institution Isolation Enforcement Pattern), ADR-L5-0005 (Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern)  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable technical architecture for enforcing authoritative consent states, purpose limitations, data minimization directives, and disclosure restrictions across all SutraERP data egress channels.

The architecture provides a cross-channel enforcement gateway that intercepts data access across APIs, integration connectors, notifications, document downloads, and reporting queries. It guarantees that personal data is processed solely for verified, approved purposes and that any consent withdrawal immediately halts downstream processing without altering canonical business records.

### 2.2 In Scope

- Cross-channel privacy enforcement interceptor pipeline operating across APIs, event handlers, reporting extractors, and connectors.
- Standardized PrivacyEvaluationContext data contract binding subject identity, consent token, approved purpose, and permitted attributes.
- Real-time consent withdrawal propagation model utilizing event-driven cache invalidation across channel gateways.
- Data minimization filter stripping attributes from payloads that are not explicitly permitted for the declared purpose.
- Fail-closed privacy evaluation: missing, expired, or withdrawn consent strictly blocks data disclosure.
- Audit evidence generation recording consent evaluations, purpose validations, and withdrawal enforcements via L5-D-04.

### 2.3 Explicit Non-Goals

This specification does not own or redefine:

- Canonical consent records, consent wording, or consent legal validity (owned by originating business domains like Student, Person, Employee).
- Institutional privacy policies, terms of service agreements, or statutory privacy rules (owned by upstream governance).
- User interface presentation of consent banners, checkboxes, or preference centers (owned by L6 / client applications).
- Role-based authorization policy evaluation (owned by L5-D-02; privacy permission is orthogonal to authorization).
- Cryptographic keys or digital signature verification for consent receipts (owned by L5-D-03).

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Architectural Constraints

Subordinate to locked L1 architecture and approved L2/L3/L4 system behavior. L5 enforces supplied consent facts but does not own or originate them.

The canonical L5 Candidate Document Corpus Register defines L5-B-05-005 as the technical privacy and purpose-limitation enforcement engine.

### 3.2 Authorization vs. Privacy Independence

Technical authorization is necessary but never sufficient to access personal data. A caller with full administrative authorization (L5-D-02) must still be blocked by L5-B-05-005 if the individual has withdrawn consent or if the request lacks an approved purpose under applicable privacy governance.

### 3.3 Operational Assumptions

- **AS-B05-007 — Distributed Event Delivery Infrastructure:** The underlying event messaging backbone (L5-C-02) provides low-latency publish-subscribe delivery for cache invalidation events across all active gateway and worker instances.

---

## 4. Technical Architecture

### 4.1 Cross-Channel Interceptor Pipeline

Privacy and purpose enforcement operates as an interceptor gateway across all egress channels:

```text
[ Incoming Request (API / Connector / Reporting / Notification) ]
                                |
                                v
+-------------------------------------------------------------------------+
|                  PrivacyEnforcementInterceptor (Gate)                   |
|                                                                         |
|  1. Resolve Subject & Purpose (Inspect PrivacyEvaluationContext)        |
|  2. Check Local Consent Cache (Low-Latency Cache)                       |
|  3. Validate Active Consent State (PENDING / GRANTED / WITHDRAWN)       |
|  4. Filter Attributes by Purpose (Apply Data Minimization Schema)       |
+-------------------------------------------------------------------------+
                                |
        +-----------------------+-----------------------+
        |                                               |
  [Consent Granted & Valid]              [Withdrawn / Expired / Invalid]
        |                                               |
        v                                               v
+-------------------------------+               +-------------------------------+
| Apply Data Minimization Mask  |               | Block Request & Log Privacy   |
| & Deliver Permitted Payload   |               | Violation Event to L5-D-04    |
+-------------------------------+               +-------------------------------+
```

### 4.2 Privacy Evaluation Context Contract

Every request touching governed personal data carries or resolves an immutable privacy evaluation context:

```text
PrivacyEvaluationContext {
    tenant_id: UUID
    institution_id: UUID
    subject_person_id: UUID
    declared_purpose_category: ENUM [ ADMISSIONS, ACADEMIC_RECORDS, PAYROLL, ALUMNI_RELATIONS, STATUTORY_REPORTING ]
    target_channel: ENUM [ REST_API, EXTERNAL_CONNECTOR, BULK_EXPORT, NOTIFICATION, CLIENT_VIEW ]
    active_consent_token_ref: UUID
    permitted_attribute_set: LIST [ STRING ]
    consent_status: ENUM [ GRANTED, WITHDRAWN, EXPIRED, NOT_REQUIRED ]
    effective_until: TIMESTAMP
}
```

### 4.3 Architecture Decision: Cross-Channel Purpose Enforcement and Real-Time Invalidation

**Decision ID:** AD-B05-005  
**Title:** Cross-Channel Interceptor-Enforced Purpose Limitation and Immediate Withdrawal Invalidation  
**Status:** Approved  
**Decision:** Privacy enforcement shall be implemented as a unified cross-channel interceptor (PrivacyEnforcementInterceptor) validating both purpose legitimacy and active consent. Gateways may cache consent states locally for high-throughput reads, but upon publishing a ConsentWithdrawn or ConsentExpired domain event, an immediate distributed cache invalidation broadcast must clear all cached tokens across API gateways, background workers, and connectors in near real-time (sub-second propagation governed by operational SLA and L5-C-02 transport guarantees).  
**Rationale:** Guarantees instantaneous compliance with citizen consent withdrawals across all technical communication and projection surfaces.

### 4.4 Data Minimization Mechanics

Even where valid consent exists, the interceptor enforces strict attribute filtering:
- The declared purpose defines an authoritative allow-list of attributes.
- Non-essential attributes present in the domain entity are stripped from the response payload before egress.
- If a purpose permits contact information for notifications but not home address, the home address field is suppressed.

---

## 5. Dependencies & Integration

### 5.1 Peer Dependencies
- **L5-B-05-001:** Provides baseline classification tags and scope verification.
- **L5-B-05-002:** Executes masking on attributes permitted for display.
- **L5-C-02:** Event broker delivers ConsentWithdrawn events triggering cache invalidation.
- **L5-D-02:** Verifies caller authorization prior to privacy evaluation.
- **L5-D-04:** Records immutable audit logs for consent checks and blocked disclosures.
- **L5-C-01 / L5-C-03 / L5-C-04:** Channels consuming the privacy interceptor.

---

## 6. Technical Behavior & Failure Concerns

- **TB-001: Immediate Withdrawal Effect:** A consent withdrawal event immediately invalidates local gateway caches and prevents subsequent data disclosure.
- **TB-002: Fail-Closed on Unverified Purpose:** Processing data without an approved purpose category is rejected immediately with a PURPOSE_NOT_AUTHORIZED exception.
- **TB-003: Purpose Non-Expansion:** Data delivered for an approved purpose cannot be internally forwarded to another internal service for a different purpose without re-evaluating the privacy gate.
- **TB-004: Audit Event Emitted on Block:** Blocked disclosures generate an attributable audit event in L5-D-04 detailing requesting channel, declared purpose, and reason for denial.

---

## 7. Security, Privacy & Trust Boundaries

- **Separation of Concerns:** Business domains record consent agreements; L5-B-05-005 enforces technical gating.
- **Least Disclosure:** Payloads crossing external boundaries contain only the strictly minimized subset required for the verified purpose.
- **Evidence Integrity:** Audit records capture purpose code and consent reference token without replicating sensitive subject data.

---

## 8. Conformance Requirements

- **CR-001:** The system shall verify active, valid consent prior to releasing personal data where consent is required by policy.
- **CR-002:** The system shall immediately block data disclosure upon receiving an authoritative consent withdrawal event.
- **CR-003:** The system shall enforce data minimization, stripping attributes not permitted for the verified purpose category.
- **CR-004:** Privacy enforcement shall apply consistently across all egress channels (REST APIs, asynchronous events, notifications, reports, exports).
- **CR-005:** Technical authorization privileges shall not override or bypass a consent restriction or withdrawal block.
- **CR-006:** All blocked data access attempts resulting from consent or purpose violations shall emit an audit record to L5-D-04.
- **CR-007:** The system shall strictly preserve Tenant and Institution scope during all privacy and consent evaluations.
- **CR-008:** Distributed consent cache invalidation shall execute across all operational nodes in near real-time (sub-second propagation governed by operational SLA and event delivery guarantees under L5-C-02) following withdrawal publication.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Candidate L5 Spec ID | Material Technical Contribution | Realization Status |
| :--- | :--- | :--- | :--- | :--- |
| L4-C-054 | L4-C-054-BH-007 | L5-B-05-005 | Ingests GuardianConsentWithdrawn events and executes immediate cross-channel cache invalidation and disclosure halting. | Verified Realization |
| L4-C-054 | L4-C-054-BH-008 | L5-B-05-005 | Distributes verified consent states to technical consumers across channels without altering guardian business ownership. | Verified Realization |
| L4-C-055 | L4-C-055-BH-002 | L5-B-05-005 | Intercepts notification and communication channels (L5-C-04) to evaluate active consent before dispatching messages. | Verified Realization |
| L4-H-001 | L4-H-001-BH-005 | L5-B-05-005 | Enforces consent lifecycle state checking (Pending, Granted, Withdrawn, Expired) on external linking requests. | Verified Realization |
| L4-H-003 | L4-H-003-BH-003 | L5-B-05-005 | Enforces locked consent states and transitions across government credential publication interfaces. | Verified Realization |
| L4-H-003 | L4-H-003-BH-004 | L5-B-05-005 | Guarantees that Withdrawn or Expired consent strictly blocks future external credential publications. | Verified Realization |
| L4-H-041 | L4-H-041-BH-003 | L5-B-05-005 | Enforces purpose limitation and directory visibility constraints on alumni profiles and personal preferences. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Exact L4 BH Applicability:** Resolved. Atomic L4 BH relations mapped to L4-C-054-BH-007, L4-C-054-BH-008, L4-C-055-BH-002, L4-H-001-BH-005, L4-H-003-BH-003, L4-H-003-BH-004, and L4-H-041-BH-003.
- **OD-002 — L4/L5 Version Binding:** Resolved. Mapped against locked canonical L4 production specifications and L5-B-05-005 v1.0.0.
- **OD-003 — Distributed Invalidation Protocol:** Non-blocking. Distributed publish-subscribe event broker wire protocol for cache invalidation governed under L5-C-02.
- **OD-004 — Dynamic Purpose Allow-Lists:** Non-blocking. Institutional configuration schema for purpose-to-attribute mapping managed under L5-A-06.

**Current State:** Ready for Review
