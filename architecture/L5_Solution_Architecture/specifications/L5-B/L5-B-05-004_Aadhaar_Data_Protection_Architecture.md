# L5-B-05-004 — Aadhaar Data Protection Architecture

## 1. Specification Metadata

**Specification ID:** L5-B-05-004  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Specialized technical protection architecture for Aadhaar-related information within authoritative privacy, security and access constraints  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.0  
**Status:** Draft  
**Last Updated:** 2026-09-12  
**Depends On:** L5-B-05-001 (Classification Baseline); L5-B-05-002 (Masking); L5-B-02 (Persistence); L5-D-02 (Authorization); L5-D-03 (Security & Cryptography); L5-D-04 (Audit & Evidence)  
**Referenced By:** L5-B-04, L5-C-01, L5-C-03, L5-G-01  
**Applicable ADRs:** ADR-L5-0001 (Canonical Trust Boundary & Segmentation Pattern), ADR-L5-0002 (Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern), ADR-L5-0003 (Contract-Driven Modular Platform with Selective Isolation), ADR-L5-0004 (Tenant and Institution Isolation Enforcement Pattern), ADR-L5-0005 (Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern)  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the specialized technical architecture for protecting Aadhaar numbers and associated identity documents in strict compliance with statutory regulations (Aadhaar Act and UIDAI data protection regulations).

The architecture enforces an **Aadhaar Vault** pattern, ensuring that 12-digit Aadhaar numbers are never stored in operational transactional databases, never displayed in cleartext in standard application interfaces, never transmitted in cleartext across service boundaries, and never logged in system diagnostics. Operational tables store strictly synthetic, random surrogate tokens (AadhaarReferenceKey).

### 2.2 In Scope

- Technical specification of the **Aadhaar Vault** boundary, isolated from general application persistence.
- Surrogate UUID tokenization pipeline converting 12-digit Aadhaar numbers to synthetic reference tokens (AadhaarReferenceKey).
- Field-level authenticated envelope encryption with master key custody managed by L5-D-03 HSM/KMS infrastructure (using authenticated ciphers, with AES-256-GCM as regulatory reference standard pending final technology-selection ADR confirmation).
- Mandatory display masking rules: only the last 4 digits may ever be displayed (XXXX-XXXX-1234).
- Strict prohibition of core biometric storage (fingerprints, iris scans, raw face templates).
- Attributable, tamper-evident audit logging of all vault token generation, verification, and unmasking requests via L5-D-04.

### 2.3 Explicit Non-Goals

This specification does not own or redefine:

- Institutional eligibility policies regarding whether Aadhaar collection is mandatory, voluntary, or prohibited.
- AUA/KUA registration status, UIDAI business agreements, or official authentication provider agreements.
- Canonical Person or Identity aggregate ownership (owned by Person / HR / Student domains).
- General PII masking and redaction mechanics (owned by L5-B-05-002).
- Key generation algorithms, key rotation schedules, or HSM physical security (owned by L5-D-03).
- Verification workflows for non-Aadhaar identity proofs (Voter ID, Passport, PAN) owned by originating domains.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Architectural Constraints

Subordinate to locked L1 architecture and approved L2/L3/L4 system behavior. L5 does not create legal policies or establish statutory authority.

The canonical L5 Candidate Document Corpus Register defines L5-B-05-004 as the specialized Aadhaar Data Protection Architecture.

### 3.2 Statutory Security Invariant

Storage of raw, unencrypted 12-digit Aadhaar numbers in operational business tables, application caches, browser local storage, or audit trails is strictly prohibited. Core biometrics shall never be captured or stored by SutraERP under any circumstance.

### 3.3 Operational Assumptions

- **AS-B05-006 — Dedicated Secure Storage Isolation:** The database persistence layer (L5-B-02) provides capability to logically or physically isolate the Aadhaar Vault table into a restricted tablespace with strict database privilege controls and encrypted storage separation.

---

## 4. Technical Architecture

### 4.1 Aadhaar Vault & Tokenization Execution Model

The architecture implements an isolated Aadhaar Vault micro-boundary separating operational systems from sensitive identity data:

```text
+-------------------------------------------------------------------------+
|                  Operational Business Modules (HR / Admissions)         |
|         Stores strictly synthetic surrogate 'AadhaarReferenceKey'       |
|                  (e.g., UUID: 'e7b9c1d2-4f8a-4b6e-...')                  |
+-------------------------------------------------------------------------+
                                    |
                    [ AadhaarVaultGateway (L5-B-05-004) ]
                                    |
                                    v
+-------------------------------------------------------------------------+
|                          Aadhaar Vault Boundary                         |
|                                                                         |
|  1. Ingress Tokenizer: 12-Digit Input -> Hash Check -> Generate UUID    |
|  2. Encryption Engine: Encrypt using DEK (authenticated envelope cipher)|
|  3. Key Envelope: DEK wrapped with KEK managed by L5-D-03 HSM/KMS       |
|  4. Isolated Persistence: Secure Vault Table (Strict Logical Partition)  |
+-------------------------------------------------------------------------+
                                    |
                                    v
+-------------------------------------------------------------------------+
|                             Vault Output Gate                           |
|                                                                         |
|  - Standard Consumers: Return ONLY Masked Value ('XXXX-XXXX-1234')      |
|  - Storage Consumers:  Return ONLY 'AadhaarReferenceKey' (UUID)         |
|  - Privileged Egress:  Require MFA + Signed Security Token (L5-D-02)    |
|  - Audit Dispatcher:   Synchronously log access event to L5-D-04        |
+-------------------------------------------------------------------------+
```

### 4.2 Aadhaar Vault Data Contract

The internal vault entity is strictly isolated from general application querying:

```text
AadhaarVaultRecord {
    reference_key: UUID (Primary Key, synthetic surrogate)
    tenant_id: UUID
    institution_id: UUID
    encrypted_aadhaar_payload: BYTEA (Authenticated ciphertext payload, AES-256-GCM reference baseline)
    initialization_vector: BYTEA (Initialization vector / nonce)
    authentication_tag: BYTEA (Authentication tag for integrity verification)
    key_version: INTEGER (L5-D-03 Key Reference)
    masked_representation: STRING(14) (Fixed format: 'XXXX-XXXX-1234')
    created_timestamp: TIMESTAMP
    last_accessed_timestamp: TIMESTAMP
}
```

### 4.3 Architecture Decision: Isolated Aadhaar Vault Architecture

**Decision ID:** AD-B05-004  
**Title:** Isolated Aadhaar Vault Architecture with Surrogate Key Tokenization  
**Status:** Approved  
**Decision:** All Aadhaar numbers collected by SutraERP must be immediately tokenized upon ingestion. Transactional modules, reporting tables, APIs, and logs shall reference the individual solely through a surrogate UUID (AadhaarReferenceKey). Cleartext Aadhaar numbers and their encrypted representations shall reside exclusively within a dedicated, restricted-access Aadhaar Vault table. Querying the cleartext requires explicit step-up authorization, emits an immediate audit record, and is forbidden in standard bulk operations.  
**Rationale:** Satisfies statutory UIDAI regulations, minimizes data breach blast radius, and ensures that operational database compromises cannot leak citizen identity numbers.

### 4.4 Mandatory Masking and Export Constraints

- **Display Formatting:** All UI forms, document previews, and operational screens must present only masked_representation (XXXX-XXXX-1234).
- **Bulk Export Prohibition:** Bulk exports, reporting projections, and data warehouse ETL pipelines shall never decrypt the vault. If identity verification evidence is required in reports, only the masked string or reference key may be exported.
- **Biometric Exclusion:** If an external e-KYC integration returns biometric scores or templates, the biometric data must be discarded in memory immediately after transaction validation; persisting biometrics is a hard fail.

---

## 5. Dependencies & Integration

### 5.1 Peer Dependencies
- **L5-B-05-001:** Enforces baseline classification and Tenant/Institution scope checks.
- **L5-B-05-002:** Enforces general masking standards on non-Aadhaar PII.
- **L5-B-02:** Manages the isolated, encrypted relational table hosting the vault.
- **L5-D-02:** Validates elevated authorization tokens for vault access.
- **L5-D-03:** Manages Data Encryption Keys (DEKs) and Key Encryption Keys (KEKs) in HSM/KMS.
- **L5-D-04:** Records immutable audit events for every vault token generation and access.

---

## 6. Technical Behavior & Failure Concerns

- **TB-001: Token Generation Idempotency:** Providing the same Aadhaar within the same Tenant/Institution returns the existing surrogate reference key without creating duplicate vault entries.
- **TB-002: Fail-Closed on Key Unavailable:** If L5-D-03 cryptographic services are unavailable, vault ingestion and resolution must fail immediately; cleartext fallback is prohibited.
- **TB-003: Tamper Detection:** Any failure of the cryptographic authentication tag during decryption triggers an immediate high-severity security alert in L5-D-04.
- **TB-004: Diagnostic Sanitization:** Application logs, exception messages, and traces must scrub all 12-digit continuous numeric patterns.

---

## 7. Security, Privacy & Trust Boundaries

- **Network / Service Isolation:** The Aadhaar Vault service must execute within an isolated trust boundary, accessible only via internal secure RPC.
- **Zero Cleartext Exposure:** Cleartext is transiently held in memory only during initial tokenization and encryption, followed by immediate zeroization of memory buffers.
- **Audit Attributability:** Every query to resolve a reference key to cleartext logs the caller identity, purpose code, and timestamp to L5-D-04.

---

## 8. Conformance Requirements

- **CR-001:** Operational domain modules shall store only synthetic surrogate tokens (AadhaarReferenceKey), never raw 12-digit Aadhaar numbers.
- **CR-002:** Cleartext Aadhaar numbers shall be stored exclusively inside the dedicated, encrypted Aadhaar Vault table.
- **CR-003:** The Aadhaar Vault shall encrypt payloads using authenticated envelope encryption with keys managed under L5-D-03 (with AES-256-GCM representing the statutory reference implementation baseline pending final technology-selection ADR confirmation).
- **CR-004:** Standard application interfaces and exports shall display only the masked representation (XXXX-XXXX-1234).
- **CR-005:** The system shall under no circumstances persist or store core biometric data (fingerprints, iris scans, face templates).
- **CR-006:** Resolving cleartext Aadhaar from a reference key shall require verified step-up authorization and emit an audit record to L5-D-04.
- **CR-007:** Application diagnostic logs, stack traces, and monitoring metrics shall never capture raw Aadhaar numbers.
- **CR-008:** The Aadhaar Vault shall strictly enforce Tenant and Institution scope isolation; reference keys from Tenant A shall not resolve in Tenant B.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Candidate L5 Spec ID | Material Technical Contribution | Realization Status |
| :--- | :--- | :--- | :--- | :--- |
| L4-A-053 | L4-A-053-BH-006 | L5-B-05-004 | Enforces mandatory identity masking and archival anonymization for national identity records upon institutional closure. | Verified Realization |
| L4-H-001 | L4-H-001-BH-001 | L5-B-05-004 | Supplies synthetic reference tokens to external government linking connectors while preventing raw citizen identifier disclosure. | Verified Realization |
| L4-A-025 | L4-A-025-BH-001 | L5-B-05-004 | Binds specialized Aadhaar classification constraints and statutory protection directives to identity domain assets. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Exact L4 BH Applicability:** Resolved. Atomic L4 BH relations mapped to L4-A-053-BH-006, L4-H-001-BH-001, and L4-A-025-BH-001.
- **OD-002 — L4/L5 Version Binding:** Resolved. Mapped against locked canonical L4 production specifications and L5-B-05-004 v1.0.0.
- **OD-003 — HSM Provider Interface:** Non-blocking. HSM vendor PKCS#11 vs. cloud KMS driver interfaces governed by L5-D-03.
- **OD-004 — Vault High-Availability Replication:** Non-blocking. Read-replica synchronization for the encrypted vault governed by L5-B-02 / L5-E-01.
- **OD-005 — Cryptographic Cipher Suite Allocation:** Non-blocking. Formal technology-selection ADR for the field-level authenticated encryption cipher suite (confirming the AES-256-GCM baseline under L5-D-03 governance).

**Current State:** Ready for Review
