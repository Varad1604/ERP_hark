# L5-B-05-001 — Data Classification, Retention & Privacy Architecture Baseline

## 1. Specification Metadata

**Specification ID:** L5-B-05-001  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Technical privacy and data-classification control model derived from upstream policy and ownership  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.0  
**Status:** Draft  
**Last Updated:** 2026-09-12  
**Depends On:** Locked L1 Domain Architecture; Approved L2/L3 Workflows; Applicable L4 System Behavior Specifications; L5-D-03 (Security & Trust); L5-B-02 (Persistence Baseline)  
**Referenced By:** L5-B-05-002, L5-B-05-003, L5-B-05-004, L5-B-05-005, L5-B-04, L5-A-07, L5-C-01, L5-C-03  
**Applicable ADRs:** ADR-L5-0001 (Canonical Trust Boundary & Segmentation Pattern), ADR-L5-0002 (Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern), ADR-L5-0003 (Contract-Driven Modular Platform with Selective Isolation), ADR-L5-0004 (Tenant and Institution Isolation Enforcement Pattern), ADR-L5-0005 (Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern)  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable technical architecture baseline by which SutraERP represents, propagates, evaluates, and enforces data-classification and privacy-control constraints established by authoritative upstream business and governance sources.

The architecture provides a common, fail-closed technical control boundary for classified information, privacy-relevant handling directives, scope preservation, and attributable enforcement evidence. It consumes authoritative policy and ownership facts without creating or altering institutional governance rules.

### 2.2 In Scope

- Technical representation and propagation of authoritative data-classification metadata (ClassificationContext).
- Standardized technical control evaluation pipeline for data access, transformation, projection, and exchange operations.
- Interceptor-driven classification and handling constraint enforcement across synchronous services and asynchronous message dispatchers.
- Preservation of Tenant and Institution scope across all classification and privacy evaluation points.
- Fail-closed handling behavior when classification or privacy context is missing, malformed, or contradictory.
- Attributable control evidence generation via L5-D-04 audit infrastructure without exposing raw sensitive payload data.
- Foundational contract integration with specialized L5-B-05 specifications (masking, retention, Aadhaar protection, and consent).

### 2.3 Explicit Non-Goals

This specification does not own or redefine:

- Business ownership of canonical domain entities, records, or source-of-truth datastores.
- Business policy defining classification rules, legal retention periods, or consent requirements.
- Sensitive-data masking and presentation mechanics (canonically owned by L5-B-05-002).
- Retention sweep execution, archival storage, and legal-hold locking (canonically owned by L5-B-05-003).
- Specialized Aadhaar vault and tokenization architecture (canonically owned by L5-B-05-004).
- Consent collection lifecycle and cross-channel purpose enforcement (canonically owned by L5-B-05-005).
- Authentication, role-based access control, cryptographic key management, or machine credentials (owned by Family L5-D).

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Architectural Constraints

This specification is subordinate to the locked L1 architecture and approved L2/L3/L4 system behavior. L5 shall not redefine upstream business ownership, business rules, lifecycle states, canonical events, publishers, institutional authority, or L3/L4 outcomes.

The canonical L5 Candidate Document Corpus Register defines L5-B-05-001 as the technical privacy and data-classification control baseline.

### 3.2 Ownership Preservation

The architecture consumes authoritative classification, privacy, and regulatory decisions; it does not originate them. A technical control may enforce a supplied constraint or block non-compliant processing, but it shall not invent a classification tag, retention schedule, consent state, purpose grant, disclosure clearance, or business exception.

### 3.3 Dependency Boundaries

- **L5-D-01 / L5-D-02:** Consumes caller identity, account context, and authorization tokens for actor attribution and entitlement verification.
- **L5-D-03:** Consumes cryptographic primitives and trust boundary enforcement without managing cryptographic keys directly.
- **L5-D-04:** Dispatches structured audit and evidence records documenting control evaluation outcomes.
- **L5-B-02:** Cooperates with database persistence boundaries to ensure classified fields preserve associated metadata during persistence and query projection.

### 3.4 Operational Assumptions

- **AS-B05-001 — Host Interceptor Framework:** The host execution runtime, API gateways, and messaging dispatchers provide extensible in-process interceptor hooks capable of resolving ClassificationContext, evaluating directives, and halting unauthorized processing prior to business logic execution.
- **AS-B05-002 — Cluster Clock Synchronization:** Distributed cluster nodes, database engines, and gateways operate with synchronized system clocks (via standard NTP infrastructure) ensuring temporal consistency of context signatures and audit timestamps.

---

## 4. Technical Architecture

### 4.1 Control Pipeline and Interceptor Architecture

The baseline establishes an interceptor-driven technical control model embedded at service boundaries:

```text
[ Incoming Request / Event ]
           |
           v
+-------------------------------------------------------------+
|        ClassificationEnforcementInterceptor (Gate)          |
|                                                             |
|  1. Context Resolution (Resolve ClassificationContext)      |
|  2. Scope Verification (Assert Tenant & Institution Match)  |
|  3. Directive Evaluation (Evaluate Handling Constraints)    |
+-------------------------------------------------------------+
           |
     +-----+-----+
     |           |
 [Valid]     [Missing / Contradictory]
     |           |
     v           v
+-----------+  +----------------------------------------------+
| Controlled|  | Fail-Closed Rejection & Security Audit Event |
| Operation |  +----------------------------------------------+
+-----------+
     |
     v
+-------------------------------------------------------------+
| Evidence Handoff (Emit Audit Evidence via L5-D-04)          |
+-------------------------------------------------------------+
```

### 4.2 Classification Context Contract

Classification context is represented as an immutable data structure bound to domain requests, entity projections, and event payloads:

```text
ClassificationContext {
    tenant_id: UUID
    institution_id: UUID
    classification_tier: ENUM [ PUBLIC, INTERNAL, RESTRICTED, HIGHLY_CONFIDENTIAL ]
    handling_tags: LIST [ PII, FINANCIAL, STATUTORY, BIOMETRIC_DERIVED ]
    privacy_directives: LIST [ CONSENT_REQUIRED, MASKING_MANDATORY, RETENTION_GOVERNED ]
    originating_domain: STRING
    context_signature: STRING (HMAC/Digest for tamper-evidence)
}
```

### 4.3 Architecture Decision: Fail-Closed Interceptor Model

**Decision ID:** AD-B05-001  
**Title:** Fail-Closed Interceptor-Driven Classification Propagation Model  
**Status:** Approved  
**Decision:** All inter-module service invocations and asynchronous event dispatches carrying domain data must include an authoritative ClassificationContext. Technical handling components shall evaluate classification constraints via a common ClassificationEnforcementInterceptor. If context is missing, tampered with, or incompatible with the requested operation, the pipeline shall immediately reject the request (fail-closed) and log an attributable security event via L5-D-04.  
**Rationale:** Prevents unauthorized downgrade, data leakage, and implicit assumptions regarding data sensitivity across modular boundaries.

### 4.4 Scope Preservation & Boundary Isolation

Classification and privacy controls strictly enforce Tenant and Institution boundaries:
- A component processing classified data in Tenant A shall never evaluate, merge, or expose data against Tenant B context.
- Cross-institutional aggregation is prohibited unless an explicit cross-institutional contract is validated.
- Any attempt to process classified information without valid Tenant/Institution scope triggers an immediate SCOPE_ENFORCEMENT_FAILURE.

---

## 5. Dependencies & Integration

### 5.1 Upstream Dependencies
- Locked L1 Domain Architecture.
- Approved L2/L3 Business Scenarios.
- Locked L4 System Behavior Specifications.

### 5.2 Peer L5 Dependencies
- **L5-B-02:** Database persistence layer must enforce classification tag preservation on database entities.
- **L5-D-03:** Security architecture supplies cryptographic integrity verification for classification tokens.
- **L5-D-04:** Audit infrastructure consumes enforcement outcome evidence records.
- **L5-B-05-002 through L5-B-05-005:** Specialist specifications consume the baseline classification model and apply domain-specific transformations (masking, retention, vaulting, consent).

---

## 6. Technical Behavior & Failure Concerns

- **TB-001: Context Availability Check:** Required classification context must be present prior to executing data access or mutation.
- **TB-002: Immutability of Classification:** Downstream components shall not downgrade or mutate classification tiers.
- **TB-003: Fail-Closed on Conflict:** Contradictory tags (e.g., PUBLIC tier tagged with PII) cause immediate operation termination.
- **TB-004: Tenant Scope Isolation:** Context resolution failure immediately halts request dispatch.
- **TB-005: Audit Evidence Continuity:** Every failure to resolve classification generates an audit event containing caller identity, scope, and failed directive.

---

## 7. Security, Privacy & Trust Boundaries

- **Authentication & Authorization:** Relies on tokens validated by L5-D-01/D-02.
- **Least Authority:** Interceptors operate under minimal execution privileges and cannot modify business payloads.
- **Data Minimization:** Control evidence dispatches metadata only; raw payload content is never replicated into audit trails.

---

## 8. Conformance Requirements

- **CR-001:** The system shall resolve a valid ClassificationContext for all service requests and messaging events crossing modular boundaries.
- **CR-002:** The system shall fail closed and reject any operation where classification context is missing, invalid, or contradictory.
- **CR-003:** The system shall preserve Tenant and Institution scope invariants across all classification evaluation points.
- **CR-004:** Downstream technical components shall not alter, downgrade, or discard authoritative classification tiers.
- **CR-005:** Technical classification controls shall not invent, approve, or alter upstream business classification policies.
- **CR-006:** All rejected operations resulting from classification or privacy constraint violations shall emit attributable audit records via L5-D-04.
- **CR-007:** Reusable classification evaluation components shall remain stateless with respect to business entity state.
- **CR-008:** Classification context metadata shall be cryptographically verifiable to prevent in-transit tampering.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Candidate L5 Spec ID | Material Technical Contribution | Realization Status |
| :--- | :--- | :--- | :--- | :--- |
| L4-A-025 | L4-A-025-BH-001 | L5-B-05-001 | Establishes ClassificationContext resolution, scope dimensions (Tenant/Institution), and baseline handling directives before evaluating downstream data operations. | Verified Realization |
| L4-A-053 | L4-A-053-BH-006 | L5-B-05-001 | Provides classification tag validation and privacy directive propagation enforcing personal data protection across operational pipelines. | Verified Realization |
| L4-H-001 | L4-H-001-BH-001 | L5-B-05-001 | Preserves and verifies Tenant/Institution boundary context and data classification prerequisites across external connector invocation boundaries. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Exact L4 BH Applicability:** Resolved. Atomic L4 BH relations mapped to L4-A-025-BH-001, L4-A-053-BH-006, and L4-H-001-BH-001.
- **OD-002 — L4/L5 Version Binding:** Resolved. Mapped against locked canonical L4 production specifications and L5-B-05-001 v1.0.0.
- **OD-003 — Pipeline Interceptor Integration Hook:** Non-blocking. Detail implementation framework binding for HTTP/gRPC pipeline filters deferred to L6 / implementation.
- **OD-004 — High-Throughput Context Caching:** Non-blocking. Local ephemeral caching of verified classification tag templates permitted if cache invalidation integrates with configuration events.

**Current State:** Ready for Review
