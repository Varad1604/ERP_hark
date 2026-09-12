# L5-B-05-002 — Sensitive Data Handling & Masking Architecture

## 1. Specification Metadata

**Specification ID:** L5-B-05-002  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Technical masking, controlled display/export and handling of classified sensitive information  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.0  
**Status:** Draft  
**Last Updated:** 2026-09-12  
**Depends On:** L5-B-05-001 (Classification Baseline); L5-B-02 (Persistence); L5-D-01/D-02 (IAM & Authorization); L5-D-03 (Security & Trust); L5-D-04 (Audit & Evidence)  
**Referenced By:** L5-B-05-004, L5-B-04, L5-B-06, L5-C-01, L5-C-03, L5-G-01  
**Applicable ADRs:** ADR-L5-0001 (Canonical Trust Boundary & Segmentation Pattern), ADR-L5-0002 (Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern), ADR-L5-0003 (Contract-Driven Modular Platform with Selective Isolation), ADR-L5-0004 (Tenant and Institution Isolation Enforcement Pattern), ADR-L5-0005 (Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern)  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable technical architecture for protecting sensitive and personally identifiable information (PII) during retrieval, display, transformation, reporting projection, and external export.

It establishes dynamic data masking, deterministic redaction, field-level tokenization, and controlled unmasking gates derived from authoritative classification context. The architecture ensures that sensitive attributes are never exposed to unauthorized technical actors, display surfaces, or logs, while preserving canonical data integrity in authoritative storage.

### 2.2 In Scope

- Technical masking, redaction, and obfuscation pipelines operating at service egress and projection boundaries.
- Transformation catalog: partial character masking, deterministic pseudonyms, format-preserving tokens, and nullification.
- Separation of canonical source values in persistence from dynamic masked projections delivered to callers.
- Controlled unmasking gateway requiring verified privileged authorization tokens and mandatory audit logging via L5-D-04.
- Enforcement across UI API endpoints, background reporting extractors, document generation pipelines, and external connectors.
- Fail-closed handling: missing or ambiguous sensitivity context defaults to maximum masking/redaction.

### 2.3 Explicit Non-Goals

This specification does not own or redefine:

- Canonical business entity schemas or primary persistent records (owned by originating business domains).
- Business classification taxonomy or statutory privacy rules (owned by upstream governance).
- Aadhaar Vault architecture, surrogate key tokenization, or regulatory Aadhaar constraints (owned by L5-B-05-004).
- Physical encryption algorithms, key generation, HSM custody, or digital certificates (owned by L5-D-03).
- Authorization policy evaluation and user role assignments (owned by L5-D-02).
- Client UI layouts, widget rendering, or CSS formatting (owned by L6 / client applications).

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Architectural Constraints

This specification is subordinate to locked L1 architecture and approved L2/L3/L4 system behavior. L5 shall not alter upstream business meanings, data models, or lifecycle definitions.

The canonical L5 Candidate Document Corpus Register defines L5-B-05-002 as the technical masking and sensitive handling architecture.

### 3.2 Canonical Value Preservation

Masking, redaction, or tokenization must never overwrite, mutate, or corrupt the authoritative underlying record in persistent storage. All masking transformations are applied dynamically during projection, query execution, or serialization, unless an explicit upstream business lifecycle command (such as permanent anonymization upon institutional exit) instructs canonical data mutation.

### 3.3 Operational Assumptions

- **AS-B05-003 — Projection-Layer Interception:** Query projection pipelines, ORM mapping layers, and response serializers provide interceptor hooks that apply transformations dynamically before records leave the service boundary.
- **AS-B05-004 — Context-Bearing Authorization Tokens:** Caller tokens verified by L5-D-01/D-02 reliably communicate active Tenant/Institution scope and privileged unmask entitlements without requiring direct identity table queries by masking interceptors.

---

## 4. Technical Architecture

### 4.1 Masking Pipeline Execution Model

Sensitive data masking operates as a dynamic egress interceptor on all query projections and API responses:

```text
+-------------------------------------------------------------------------+
|                  Database Persistence Boundary (L5-B-02)                |
|                    [ Canonical Cleartext / Ciphertext ]                 |
+-------------------------------------------------------------------------+
                                    |
                                    v
+-------------------------------------------------------------------------+
|                    Data Projection & Retrieval Layer                    |
|                        (Domain Service Entity)                          |
+-------------------------------------------------------------------------+
                                    |
                                    v
+-------------------------------------------------------------------------+
|                 SensitiveDataMaskingInterceptor (Gate)                  |
|                                                                         |
|  1. Inspect Caller Security Context (L5-D-02 Authorization Token)       |
|  2. Check for PrivilegedUnmaskPermission                                |
|  3. Apply Transformation Rules (Mask, Redact, Tokenize, Nullify)        |
+-------------------------------------------------------------------------+
           |                                             |
    [Unmask Granted]                              [Standard Access]
           |                                             |
           v                                             v
+-----------------------------+           +-------------------------------+
| Audit Event Dispatched      |           | Masked Representation Emitted |
| (Caller, Scope, Field ID)   |           | (e.g. 'XXXX-XXXX-1234')       |
+-----------------------------+           +-------------------------------+
           |                                             |
           +----------------------+----------------------+
                                  |
                                  v
                    [ Client / API / File Egress ]
```

### 4.2 Transformation Catalog

The technical architecture standardizes four discrete transformation categories:

1. **Partial Masking (Pattern-Preserving):** Preserves designated trailing or leading characters while obfuscating the remainder with a fixed masking character (e.g., Bank Account: XXXXXXXX1234, Email: j***@example.com).
2. **Format-Preserving Tokenization:** Generates a deterministic or random surrogate string maintaining the data type and length of the original value for operational correlation without exposing cleartext.
3. **Nullification / Value Suppression:** Completely replaces the sensitive field with a null representation or fixed sentinel ([REDACTED]).
4. **One-Way Irreversible Redaction:** Applied when projecting data to non-production, reporting, or external audit channels where recovery of the cleartext is prohibited.

### 4.3 Architecture Decision: Dynamic Egress-Layer Data Masking

**Decision ID:** AD-B05-002  
**Title:** Dynamic Egress-Layer Data Masking and Unmasking Authorization Gate  
**Status:** Approved  
**Decision:** All sensitive data masking shall be executed dynamically at the service and query egress boundary (SensitiveDataMaskingInterceptor), rather than mutating data in storage. Unmasking requests must present an explicit PrivilegedUnmaskPermission scoped to the exact field and Tenant/Institution. Every unmasking operation must emit a synchronous audit record to L5-D-04 containing actor identity, justification context, timestamp, and target record reference.  
**Rationale:** Preserves canonical data integrity in the database while eliminating the risk of cleartext exposure in service caches, API responses, and presentation tiers.

### 4.4 Controlled Unmasking Path

When a business process legitimately requires unmasked data (e.g., payroll verification by an authorized officer):
1. The requestor must present a valid session token bearing the specific DATA_UNMASK_PRIVILEGED scope.
2. The SensitiveDataMaskingInterceptor validates the token against L5-D-02 authorization policies.
3. If valid, cleartext is passed through the egress gate, and an immutable SensitiveDataUnmaskedEvent is synchronously emitted to L5-D-04.
4. If invalid or missing, the interceptor falls back to default masked output without halting the overall request.

---

## 5. Dependencies & Integration

### 5.1 Peer Dependencies
- **L5-B-05-001:** Supplies classification tags and privacy directives.
- **L5-B-02:** Database persistence layer supplies query projection streams.
- **L5-D-02:** Authorization engine verifies unmask permissions.
- **L5-D-04:** Audit infrastructure stores unmasking access evidence.
- **L5-C-01 / L5-C-03:** Inbound and outbound contract serialization layers enforce masking rules before wire transfer.

---

## 6. Technical Behavior & Failure Concerns

- **TB-001: Default-Mask Behavior:** If masking metadata for a classified field cannot be determined, the system must apply complete nullification.
- **TB-002: Export Containment:** Bulk exports, CSV/PDF generation, and reporting queries must strictly inherit masking rules; unmasking during bulk operations requires dedicated administrative approval.
- **TB-003: Memory Scrubbing:** Ephemeral memory buffers holding cleartext during masking transformations must be freed immediately.
- **TB-004: Anti-Inference:** Masking patterns must prevent length-based or frequency-based inference of underlying values.

---

## 7. Security, Privacy & Trust Boundaries

- **Trust Boundaries:** Cleartext is strictly bounded within domain service boundaries and isolated from presentation tiers.
- **Privilege Separation:** Standard read permissions grant access only to masked representations; unmasking requires separate role elevation.
- **Audit Logging:** Audit events record the fact of unmasking but never log the cleartext value itself.

---

## 8. Conformance Requirements

- **CR-001:** The system shall apply dynamic masking to all classified sensitive data at the egress boundary of synchronous and asynchronous endpoints.
- **CR-002:** The system shall maintain canonical storage values unaltered during all masking and projection operations.
- **CR-003:** The system shall require an explicit, verified PrivilegedUnmaskPermission before returning unmasked values.
- **CR-004:** Every unmasking event shall emit an attributable audit evidence record to L5-D-04 recording actor, scope, and target field.
- **CR-005:** The system shall fail closed by applying maximum redaction if handling directives or caller credentials are unresolved.
- **CR-006:** Masked representations shall be visually and structurally distinguishable from canonical cleartext values.
- **CR-007:** Masking transformations shall preserve Tenant and Institution scope invariants.
- **CR-008:** Audit logs, application diagnostics, and system traces shall never log cleartext sensitive values.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Candidate L5 Spec ID | Material Technical Contribution | Realization Status |
| :--- | :--- | :--- | :--- | :--- |
| L4-A-053 | L4-A-053-BH-006 | L5-B-05-002 | Executes dynamic and permanent masking of personal data upon approved institutional exit and anonymization directives while maintaining audit references. | Verified Realization |
| L4-H-041 | L4-H-041-BH-003 | L5-B-05-002 | Enforces privacy-controlled display and attribute-level masking for directory visibility and external personal profiles. | Verified Realization |
| L4-A-025 | L4-A-025-BH-001 | L5-B-05-002 | Consumes classification dimensions to determine appropriate masking patterns and unmasking privilege tiers. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Exact L4 BH Applicability:** Resolved. Mapped to L4-A-053-BH-006, L4-H-041-BH-003, and L4-A-025-BH-001.
- **OD-002 — L4/L5 Version Binding:** Resolved. Mapped against locked canonical L4 production specifications and L5-B-05-002 v1.0.0.
- **OD-003 — UI Component Masking Optimization:** Non-blocking. Client-side vs. server-side projection mechanics for interactive forms deferred to L6 / client platform.
- **OD-004 — High-Volume Export Streaming:** Non-blocking. Stream-based tokenization filters for bulk file exports deferred to implementation.

**Current State:** Ready for Review
