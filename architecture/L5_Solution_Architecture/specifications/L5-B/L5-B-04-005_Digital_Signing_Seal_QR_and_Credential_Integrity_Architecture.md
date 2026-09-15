# L5-B-04-005: Digital Signing, Seal, QR, and Credential Integrity Architecture

## 1. Document Metadata & Control

| Attribute | Value |
| :--- | :--- |
| **Specification ID** | `L5-B-04-005` |
| **Version** | `1.0.3` |
| **Status** | `APPROVED` |
| **Classification** | `Internal Confidential` |
| **Primary Architectural Module** | `Credential Trust, Cryptographic Signing & Public Verification` |
| **Owning Solution Architect** | `Varad Deshpande` |
| **Cross-Cutting Domain** | `Cryptographic Services, Institutional Trust, Identity & Public Verification` |
| **Upstream Traceability** | `L4-H-002`, `L4-H-004`, `L4-H-005`, `L4-C-018` |
| **Downstream Implementation Target** | `crypto-signing-service`, `public-verification-portal`, `hsm-gateway` |
| **Effective Date** | `2026-09-15` |

---

## 2. Executive Summary & Architectural Scope

The **Digital Signing, Seal, QR, and Credential Integrity Architecture** (`L5-B-04-005`) defines the cryptographic trust framework, institutional digital signing pipeline, tamper-evident high-density QR encoding, and public verification architecture for SutraERP. Across academic degrees, transcripts, payroll disbursement slips, and official institutional communications, the platform establishes verifiable provenance and long-term non-repudiation.

The architecture integrates Hardware Security Modules (Cloud KMS / FIPS 140-2 Level 3 HSM) for institutional key custody, implements PAdES-LTV (PDF Advanced Electronic Signatures - Long Term Validation) via qualified RFC 3161 Time-Stamping Authorities, generates cryptographically signed offline-verifiable QR codes (ECDSA P-256 + CBOR / COSE), and provides a zero-knowledge public verification gateway.

```
+---------------------------------------------------------------------------------------------------+
|                        L5-B-04-005 CRYPTOGRAPHIC SIGNING & CREDENTIAL TRUST                       |
|                                                                                                   |
|  [ Ingestion Pipeline ]          [ Cryptographic Signing Engine ]        [ Key & Trust Services ] |
|   - Rendered PDF Artifact        - PAdES B-LTA Compliant Signer          - Cloud KMS / Cloud HSM  |
|   - Signing Request Metadata     - SHA-256 / SHA-384 Digest Compute      - FIPS 140-2 Level 3     |
|   - Verification Payload         - Detached PKCS#7 / CMS Engine          - Institutional Root PKI |
|             |                                |                                   |                |
|             v                                v                                   v                |
|  +----------------------+      +---------------------------+      +-----------------------------+ |
|  | Verifiable QR Engine | ---> | Time-Stamping Authority   | ---> | Public Verification Gateway | |
|  | - CBOR Payload Pack  |      | - RFC 3161 Qualified TSA  |      | - Air-Gapped Offline Scan   | |
|  | - COSE Sign (P-256)  |      | - Secure Timestamp Token  |      | - Online Portal Lookup      | |
|  | - Error Correct Lvl H|      | - Embedded OCSP / CRL     |      | - Merkle Proof Invalidation | |
|  +----------------------+      +---------------------------+      +-----------------------------+ |
|                                              |                                   |                |
|                                              v                                   v                |
|  [ Output Artifacts ]          [ Non-Repudiation Storage ]             [ Trust Federation ]       |
|   - PAdES-LTV PDF Document      - Cryptographic Audit Log               - National Depository     |
|   - Signed Verification Receipt - Tamper-Proof Signature Manifest         Integration (e.g., NAD) |
|   - Public Verification URL     - Long-Term Archival (30+ Years)        - W3C Verifiable Cred.    |
+---------------------------------------------------------------------------------------------------+
```

---

## 3. Upstream Traceability Matrix

This specification establishes direct, bidirectional traceability to L4 behavioral specifications as detailed below:

| L4 Specification ID | Behavioral Header | L4 Requirement Description | L5 Section Reference | Verification Method |
| :--- | :--- | :--- | :--- | :--- |
| `L4-H-002` | `L4-H-002-BH-001` | HSM key lifecycle, generation, rotation, and usage policy | Section 5.2 | Cloud HSM key policy audit & KMS rotation test |
| `L4-H-002` | `L4-H-002-BH-002` | Digital signature generation for institutional academic transcripts | Section 5.1, 7.1 | Apache PDFBox PAdES signature validation harness |
| `L4-H-002` | `L4-H-002-BH-003` | Institutional digital seal embedding and visual representation | Section 5.1 | PDF rendering visual inspection and layer test |
| `L4-H-002` | `L4-H-002-BH-004` | Audit logging of all cryptographic signing operations | Section 5.6 | Audit trail tamper-resistance test |
| `L4-H-004` | `L4-H-004-BH-001` | High-density tamper-evident QR code generation | Section 5.3, 7.2 | QR decoder and CBOR unpacking test suite |
| `L4-H-004` | `L4-H-004-BH-002` | Elliptic curve digital signature embedding within QR payloads | Section 5.3 | ECDSA secp256r1 signature verification test |
| `L4-H-004` | `L4-H-004-BH-003` | Offline verification support without network connectivity | Section 5.4 | Air-gapped mobile scanner verification harness |
| `L4-H-004` | `L4-H-004-BH-004` | Public credential verification portal and API endpoint | Section 5.4, 7.3 | Public verification gateway integration test |
| `L4-H-004` | `L4-H-004-BH-005` | Credential revocation, suspension, and status list checking | Section 5.5, 7.4 | Revocation status bit-vector validation test |
| `L4-H-005` | `L4-H-005-BH-001` | RFC 3161 compliant cryptographic timestamping integration | Section 5.1 | RFC 3161 timestamp parsing and cert chain test |
| `L4-H-005` | `L4-H-005-BH-002` | Long-Term Validation (PAdES-LTV) embedding (DSS & VRI) | Section 5.1 | 30-year archival simulation and LTV validation |
| `L4-H-005` | `L4-H-005-BH-003` | Integration with national academic depository (NAD/DigiLocker) | Section 5.5 | Depository schema and push adapter contract test |
| `L4-H-005` | `L4-H-005-BH-004` | Credential tamper detection and anomaly alerting | Section 5.4, 9.2 | Altered payload rejection and alert trigger test |
| `L4-C-018` | `L4-C-018-BH-004` | Coordination of signature pipeline with document issuance | Section 5.1 | End-to-end issuance-to-signature orchestration |
| `L4-C-018` | `L4-C-018-BH-005` | Watermark and institutional seal visual overlay controls | Section 5.1 | Visual layer separation inspection |

---

## 4. Architectural Principles & Constraints

1. **Hardware-Guaranteed Non-Repudiation**: Private keys used for institutional seals and credential signing must reside in FIPS 140-2 Level 3 certified Hardware Security Modules (Cloud KMS / HSM). Export of raw private keys is cryptographically impossible.
2. **Long-Term Validation (LTV) Self-Containment**: Signed PDF credentials must embed all certificate chains, revocation information (CRL/OCSP), and RFC 3161 timestamps into the Document Security Store (DSS). The document must remain verifiable decades into the future without depending on external PKI availability.
3. **Dual-Mode Verification (Online & Air-Gapped Offline)**: High-density QR codes printed on physical or digital certificates must contain sufficient compressed, digitally signed payload data to permit full authentication by third parties in air-gapped environments without database access.
4. **Instant Invalidation & Cryptographic Revocation**: When an academic degree or invoice is revoked, revocation status must immediately reflect across the public verification portal and external depository federations via cryptographically signed revocation lists.

---

## 5. Detailed Solution Architecture & Component Design

### 5.1 PAdES-LTV Cryptographic Signing Pipeline
The signing pipeline transforms standard rendered PDF/A-3 documents into legally enforceable, digitally signed credentials conforming to ETSI EN 319 142 (PAdES Part 3 & 4):
- **Digest Calculation**: A SHA-256 / SHA-384 cryptographic hash is computed over the byte ranges excluding the signature dictionary.
- **Detached Signature Generation**: The computed hash is dispatched to the HSM Gateway, which executes an ECDSA/RSA signature operation using the institution's certified private key.
- **RFC 3161 Trusted Timestamping**: The signature token is transmitted to a designated Qualified Time-Stamping Authority (Q-TSA) to obtain a cryptographically binding RFC 3161 timestamp token.
- **DSS & VRI Injection**: The complete certificate path, intermediate CA certificates, current OCSP responses, and CRLs are written into the PDF's `/DSS` (Document Security Store) and `/VRI` dictionaries, achieving PAdES B-LTA status.

### 5.2 Hardware Security Module (HSM) Gateway & Key Lifecycle
Key operations are isolated behind the HSM Gateway:
- **Tenant Isolation**: Each higher-education institution or corporate entity holds a dedicated cryptographic key ring with separate IAM roles and access control boundaries.
- **Key Rotation**: Root signing keys rotate every 24 months, while sub-keys for administrative signing rotate annually. Revoked or expired keys remain preserved in read-only HSM state to facilitate signature verification of historical documents.
- **Role-Based Activation**: High-impact batch signing (e.g., annual degree convocation) requires multi-custodian quorum activation (M-of-N administrative authorization).

### 5.3 Verifiable High-Density QR Code Generator
Documents stamped with physical or visual verification blocks embed an ISO/IEC 18004 compliant high-density QR code:
- **Payload Serialization**: Critical credential claims (Student Name, Roll Number, Degree Awarded, CGPA/Classification, Issue Date, Document Unique ID) are encoded using Concise Binary Object Representation (CBOR - RFC 8949).
- **Cryptographic Enclosure**: The CBOR payload is signed using COSE (CBOR Object Signing and Encryption - RFC 9052) via the institution's ECDSA P-256 public-private key pair.
- **Error Correction**: QR codes are generated with Level H (30% recovery capability), ensuring readability even if physically abraded, stamped, or folded.

### 5.4 Public Verification Gateway & Air-Gapped Verification
The verification architecture provides two operational paths:
1. **Air-Gapped Offline Scan**:
   - The third-party mobile or web scanner reads the QR code.
   - It parses the COSE structure and extracts the institutional public key from an embedded trust store.
   - It validates the ECDSA signature locally and renders the verified student/credential attributes without making any network calls.
2. **Online Portal Lookup**:
   - The user navigates to the public verification portal URL encoded in the QR.
   - The portal executes an SHA-256 integrity lookup against the institutional issuance ledger, returning the canonical document metadata, issuing officer, exact signing timestamp, and active revocation status.

### 5.5 Revocation Lists & Depository Federation
- **Status List Bit-Vectors**: High-efficiency W3C StatusList2021 bit-vectors track active vs revoked credentials. Revocation changes update the bitmask and generate an updated Merkle tree root hash.
- **National Depository Adapters**: For accredited academic institutions, issued degrees and transcripts are simultaneously dispatched to national academic depositories (e.g., India's NAD/DigiLocker) via secure mutual-TLS API connectors.

### 5.6 Cryptographic Audit Ledger
Every cryptographic operation (signing, timestamping, key access, verification attempt, revocation) records an immutable entry into the Cryptographic Audit Log containing the operator identity, key version, target document hash, and TSA receipt.

---

## 6. Architectural Decisions

| Decision ID | Decision Title | Status | Rationale | Alternatives Considered | Trade-offs |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `AD-L5-B04-005-01` | PAdES B-LTA Profile with Embedded DSS over Basic PKCS#7 | `ACCEPTED` | Basic signatures break when intermediate CA certificates expire or CRL endpoints disappear. PAdES B-LTA guarantees decades-long validation. | Basic PKCS#7 detached signatures; Adobe XML Forms Architecture (XFA). | Increased file size (~35 KB additional PKI/OCSP metadata) and higher processing latency during PDF finalization. |
| `AD-L5-B04-005-02` | Compressed CBOR/COSE Signatures in High-Density QR over Plain URLs | `ACCEPTED` | Enables tamper-proof verification in air-gapped environments without exposing private institutional databases to public scraping. | Plain URL redirect QR code; Raw uncompressed JSON QR. | Payload size is constrained; requires specialized client parsing libraries for CBOR/COSE verification. |
| `AD-L5-B04-005-03` | Cloud HSM (FIPS 140-2 Level 3) Managed Keys over Software PKCS#12 Keystores | `ACCEPTED` | Eliminates risk of key exfiltration from server disk or memory; complies with national statutory standards for institutional degree issuance. | File-based `.pfx` / `.p12` keystores protected by software passwords. | Incurs API-based cryptographic latency (40-80ms per signature) and recurring cloud HSM service costs. |
| `AD-L5-B04-005-04` | Dual RFC 3161 Qualified TSA Integration with Automatic Failover | `ACCEPTED` | Guarantees non-repudiation and prevents signing pipeline stalls during commercial TSA endpoint outages. | Single commercial TSA provider; Local uncertified system clock timestamps. | Requires maintaining commercial subscriptions and active health probes across two independent TSA providers. |

---

## 7. Conceptual Data Models & Schema Contracts

### 7.1 Cryptographic Signing Job Request Schema
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "CryptoSigningJobRequest",
  "type": "object",
  "required": ["request_id", "document_id", "tenant_id", "document_sha256", "signature_profile", "key_alias"],
  "properties": {
    "request_id": { "type": "string", "format": "uuid" },
    "document_id": { "type": "string", "pattern": "^doc_[a-zA-Z0-9_-]{20,36}$" },
    "tenant_id": { "type": "string" },
    "document_sha256": { "type": "string", "pattern": "^[a-f0-9]{64}$" },
    "signature_profile": { "type": "string", "enum": ["PADES_B_LTA", "PADES_B_T", "RAW_COSE_SIGN"] },
    "key_alias": { "type": "string", "description": "KMS / HSM key identifier" },
    "visible_seal_config": {
      "type": "object",
      "required": ["page_number", "x_coordinate", "y_coordinate", "width", "height"],
      "properties": {
        "page_number": { "type": "integer", "minimum": 1 },
        "x_coordinate": { "type": "number" },
        "y_coordinate": { "type": "number" },
        "width": { "type": "number" },
        "height": { "type": "number" },
        "seal_image_asset_id": { "type": "string" }
      }
    }
  }
}
```

### 7.2 Verifiable QR CBOR Payload Schema (COSE Sign1)
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "VerifiableQRPayload",
  "type": "object",
  "required": ["iss", "sub", "doc_id", "deg", "gpa", "iat", "exp", "sig"],
  "properties": {
    "iss": { "type": "string", "description": "Institutional Issuer DID or Domain" },
    "sub": { "type": "string", "description": "Subject Identifier (e.g., Student ID / Roll Number)" },
    "doc_id": { "type": "string", "description": "Unique Document Tracking ID" },
    "deg": { "type": "string", "description": "Degree or Credential Title" },
    "gpa": { "type": "string", "description": "CGPA, Division, or Percentage" },
    "iat": { "type": "integer", "description": "UNIX timestamp of issuance" },
    "exp": { "type": "integer", "description": "UNIX timestamp of expiration (or 0 for perpetual)" },
    "sig": { "type": "string", "description": "ECDSA P-256 signature over CBOR claims" }
  }
}
```

### 7.3 Public Verification Response Schema
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "PublicVerificationResponse",
  "type": "object",
  "required": ["verification_status", "document_id", "issuer_name", "issued_at", "cryptographic_validity"],
  "properties": {
    "verification_status": { "type": "string", "enum": ["VALID", "REVOKED", "EXPIRED", "TAMPERED", "NOT_FOUND"] },
    "document_id": { "type": "string" },
    "document_type": { "type": "string" },
    "issuer_name": { "type": "string" },
    "recipient_name": { "type": "string" },
    "issued_at": { "type": "string", "format": "date-time" },
    "tsa_timestamp": { "type": "string", "format": "date-time" },
    "cryptographic_validity": {
      "type": "object",
      "required": ["signature_intact", "certificate_valid", "timestamp_valid", "hash_matched"],
      "properties": {
        "signature_intact": { "type": "boolean" },
        "certificate_valid": { "type": "boolean" },
        "timestamp_valid": { "type": "boolean" },
        "hash_matched": { "type": "boolean" }
      }
    },
    "revocation_details": {
      "type": "object",
      "properties": {
        "revoked_at": { "type": "string", "format": "date-time" },
        "reason": { "type": "string" }
      }
    }
  }
}
```

---

## 8. Concrete Operational & Cross-Cutting Requirements

| Requirement ID | Category | Objective / Specification | Verification Target |
| :--- | :--- | :--- | :--- |
| `CR-B04-005-PERF-01` | Performance | End-to-end PAdES B-LTA signing latency | <= 180 ms (p95) including HSM signature and TSA stamping |
| `CR-B04-005-PERF-02` | Performance | Verifiable QR code generation and COSE signing | <= 25 ms per payload |
| `CR-B04-005-SEC-01` | Security | HSM Key Protection Standards | FIPS 140-2 Level 3 certified hardware compliance |
| `CR-B04-005-SEC-02` | Security | Air-gapped QR signature validation | 100% cryptographic detection of altered payload bytes |
| `CR-B04-005-REL-01` | Reliability | RFC 3161 TSA availability via dual-provider failover | >= 99.999% successful timestamp token acquisition |
| `CR-B04-005-COMP-01` | Compliance | Signature archival longevity | Valid PAdES-LTV verification for minimum 30 years |

---

## 9. Security, Governance & Failure Modes

### 9.1 Cryptographic Attack Vectors & Mitigations
- **Private Key Exfiltration**: Keys are stored exclusively inside cloud HSM enclosures with strict access control policies and audit trails; private key material never touches RAM or disk on application servers.
- **Signature Replay / Identity Spoofing**: Each digital signature encapsulates the unique document SHA-256 hash and an RFC 3161 timestamp, precluding replay of signatures across altered documents.
- **QR Code Tampering**: Any alteration of the CBOR data bytes causes immediate cryptographic signature verification failure during both online portal lookup and offline mobile scanning.

### 9.2 Failure Modes & Resilience Patterns
- **Primary TSA Outage**: If the primary RFC 3161 TSA fails to respond within 2500ms, the signing service automatically switches to the secondary qualified TSA without dropping the transaction.
- **KMS / HSM Rate Throttling**: The HSM gateway maintains persistent connection pools and token buckets to smooth out peak batch signing bursts and prevent API throttling.
- **OCSP Responder Degradation**: If an external CA's OCSP responder is slow, cached OCSP responses (within their authorized validity window) are utilized for DSS building.

---

## 10. Traceability Verification & Test Harness Specifications

### 10.1 PAdES Signature & Archival Validation Test Harness
Verification is executed against the Apache PDFBox and Adobe Acrobat validation models:
```bash
# Execute PAdES-LTV cryptographic signature integrity test suite
npm run test:crypto:pades -- --input=./test/fixtures/signed_degree_sample.pdf
```
Asserts:
- Signature format is ETSI PAdES B-LTA.
- Document has not been modified since signature application.
- RFC 3161 timestamp token matches institutional clock within +/- 1000ms.
- DSS contains complete certificate chain and valid revocation data.

### 10.2 Verifiable QR Air-Gapped Signature Harness
```bash
# Execute CBOR / COSE air-gapped signature and tamper test
python test/verify_qr_cose.py --sample=./test/fixtures/sample_degree_qr.cbor
```
Tests:
1. Normal payload: Asserts valid ECDSA P-256 signature and accurate field decoding.
2. 1-bit corrupted payload: Asserts cryptographic signature verification fails with error code `SIGNATURE_MISMATCH`.
