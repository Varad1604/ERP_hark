# L5-B-04-003 — Document Ingestion, Validation & Verification Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-04-003 |
| Specification Title | Document Ingestion, Validation & Verification Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-04 — Document, Media & Object Platform Architecture |
| Canonical Technical Responsibility | Technical ingestion, file/content validation, evidence and verification mechanisms; owning domains retain semantic acceptance authority. |
| Technical Architecture Owner | Document Ingestion & Verification Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-04-001; L5-B-04-002; L5-B-02-001; L5-B-05-001; L5-B-05-002; L5-D-03-001; L5-D-04-001; L4-C-007; L4-D-043; L4-E-060 |
| Referenced By | Student Admission Ingestion; Faculty Qualification Registration; External Credit Transfer workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0005; ADR-L5-0014 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution architecture for technical document ingestion, multi-layer file validation, malware scanning, and integrity verification across SutraERP. The architecture provides a hardened technical intake pipeline that admits external file uploads, verifies structural file integrity, scans for malicious payloads, extracts verified metadata, and classifies verification findings, while strictly ensuring that the owning business domain retains sole authority for semantic document acceptance, applicant eligibility, or qualification approval.

### In Scope

- Multi-stage upload admission pipeline (rate limiting, chunked streaming, file size bounding).
- Deep structural file validation (magic byte inspection, MIME-type conformance, PDF/image parsing safety).
- Automated anti-malware and active content scanning (detecting macros, embedded scripts, zip slips).
- Technical verification of document integrity (SHA-256 hashing, digital signature verification on submitted files).
- Automated quarantine isolation for corrupted, malformed, or suspicious uploads.
- Structured technical verification report generation delivered to calling business workflows.

### Out of Scope / Non-Goals

- Granting business approval to submitted applicant documents (owned by Admissions Committee in L4-C-007).
- Deciding academic course credit equivalence from external certificates (owned by Faculty Committee in L4-D-043).
- Validating employee professional credentials against external government portals (owned by HR in L4-E-060).
- UI/UX upload drop-zones, progress bars, or client-side image cropping widgets (owned by L6).
- Permanent storage management (delegated to L5-B-04-002).

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-C-007` | Approved / Locked | `L4-C-007-BH-001` | Candidate document requirement profile resolution | Ingestion profile admission binding |
| L4 | `L4-C-007` | Approved / Locked | `L4-C-007-BH-002` | Upload submission handling and candidate document intake | Stream ingestion and chunking pipeline |
| L4 | `L4-C-007` | Approved / Locked | `L4-C-007-BH-003` | Technical file verification to structured document assessment | Structural/integrity validation pipeline |
| L4 | `L4-C-007` | Approved / Locked | `L4-C-007-BH-004` | Verification anomaly detection to flagged verification finding | Anomaly detection and quarantine routing |
| L4 | `L4-C-007` | Approved / Locked | `L4-C-007-BH-007` | Deficient submission remediation request to corrected upload window | Re-ingestion and version replacement |
| L4 | `L4-D-043` | Approved / Locked | `L4-D-043-BH-001` | Submission intake to verifiable certificate context | External certificate ingestion binding |
| L4 | `L4-D-043` | Approved / Locked | `L4-D-043-BH-002` | Issuer and certificate authenticity verification | Cryptographic authenticity assessment |
| L4 | `L4-E-060` | Approved / Locked | `L4-E-060-BH-002` | Uploaded qualification evidence to technical verification pipeline | Employee credential technical validation |
| L4 | `L4-E-060` | Approved / Locked | `L4-E-060-BH-003` | Integrity and tamper verification of submitted academic credential | Tamper analysis and hash verification |
| Decision | Corpus Register | Locked | `L5-B-04-003` | Ingestion, validation, verification; domains retain acceptance authority | Canonical technical responsibility |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | File extension names must never be trusted; true content types must be verified via magic byte inspection. | Application Security Baseline | Parser inspects binary file headers; mismatched extension and magic bytes trigger immediate rejection. |
| C-002 | All uploaded files must be scanned in an isolated quarantine sandbox before admission to the main registry. | Threat Modeling Baseline | Direct storage to production document buckets prior to security clearance is strictly prohibited. |
| C-003 | Technical verification reports shall not grant business acceptance. | L5 Candidate Register §3.1 | Pipeline returns `VERIFIED_VALID` or `FLAGGED_ANOMALY`; business officers make admission decisions. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Anti-malware scanning engine is provisioned as an isolated service in L5-E-04. | Protects host workers against zero-day file parsing exploits. | Scanning failure causes uploads to be held in quarantine. | Integrates with scanning sidecar via ADR-L5-0001 trust boundary. |
| AS-002 | Document upload profiles specify allowed MIME types and maximum file size per document category. | Prevents denial of service and invalid format submissions. | Rejection occurs at admission gate if file exceeds profile bounds. | Profiles resolved from L4-C-007-BH-001. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Streaming file ingestion gateways and chunked upload assembly.
- Magic byte detection and deep MIME-type structural validation.
- File integrity checks (SHA-256 computation, corrupt header detection, PDF syntax validation).
- Anti-malware scanning orchestration and malicious active-content detection.
- Temporary quarantine sandbox storage and automated quarantine cleanup.
- Generation of structured technical `DocumentValidationReport` records.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-B-04-001` | Document Platform Baseline | Architecture baseline | Inherits common object identity and storage gateway abstractions. |
| `L5-B-04-002` | Document Registry & Retention | Storage handoff | Registers cleared documents into permanent catalog and version tree. |
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Verification persistence | Persists verification findings and anomaly reports in database (ADR-L5-0005). |
| `L5-D-03-001` | Canonical Trust Boundary & Segmentation | Security boundary | Enforces complete isolation between unverified uploads and internal networks. |
| `L5-D-04-001` | Audit & Evidence Architecture Baseline | Audit trail | Emits tamper-evident evidence of document ingestion and verification outcomes. |

### 4.3 Must Not Own

- Deciding whether a candidate's high school transcript satisfies academic minimum eligibility.
- Deciding whether an applicant's caste or domicile certificate is legally genuine.
- Modifying admission application states or changing student enrollment statuses.
- Generating official university certificates or applying digital signatures (owned by L5-B-04-004/005).

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `UploadAdmissionGate` | Control Point | Validates upload authorization, tenant scope, file size limits, and allowed MIME profile. | `DocumentUploadRequest` | Ephemeral stream | Admitted upload stream | `L5-D-01`, `L5-A-04` | Rejects unauthenticated uploads or oversized files. | L4-C-007-BH-002 |
| `QuarantineBufferStore` | Storage Component | Stores incoming upload stream in an isolated, encrypted quarantine storage zone. | Upload binary stream | Quarantine bucket | Quarantine locator key | `L5-D-03-002` | Quarantined blobs cannot be read by standard application users. | C-002, ADR-L5-0001 |
| `StructuralFormatValidator` | Component | Reads magic bytes, inspects PDF/image internal structural tree, detects corrupt headers. | Quarantine blob stream | Stateless | Structural validation report | Format parsers | Fails on malformed headers, invalid compression, or truncated EOF. | C-001, L4-C-007-BH-003 |
| `MalwareScanningEngine` | Component | Orchestrates virus scanning, macro detection, and malicious PDF JavaScript extraction. | Quarantine blob stream | Sandbox runtime | Security clearance result | `L5-E-04-001` | Flags infected or suspicious files immediately. | C-002, L4-C-007-BH-004 |
| `VerificationReportEmitter` | Component | Assembles structural, security, and integrity findings into a structured report; registers valid doc. | Validation results & hash | Stateless | `DocumentValidationReport` | `L5-B-04-002`, `L5-D-04` | Transitions file from Quarantine to Registry if valid. | L4-C-007-BH-003, C-003 |

### 5.2 Primary Technical Flow

```text
Applicant / Employee File Upload (L4-C-007 / L4-E-060)
         ↓
UploadAdmissionGate (validate Tenant, Session Token, Allowed Extension, Max Size)
         ↓
QuarantineBufferStore (stream file into isolated, encrypted Quarantine Storage Zone)
         ↓
StructuralFormatValidator (inspect magic bytes: verify actual file matches claimed MIME type)
  [Mismatch] → Reject upload (ERR_MIME_SPOOFING_DETECTED)
         ↓
MalwareScanningEngine (scan for viruses, embedded macros, executable scripts, zip bombs)
  [Infection] → Lock in Quarantine, alert security, emit ANOMALY_MALWARE_DETECTED
         ↓
IntegrityHashVerifier (compute SHA-256 digest, verify embedded digital signatures if PDF)
         ↓
[All Checks Passed] → Move blob from Quarantine to Permanent Storage (L5-B-04-002)
                    → Register Document UUID and Metadata in Registry
         ↓
VerificationReportEmitter (emit DocumentValidationReport with Status: VERIFIED_VALID)
         ↓
Return Report to Calling Domain Workflow (Admissions / HR evaluates semantic acceptance)
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| MIME-Type Spoofing | `StructuralFormatValidator` | Immediate rejection (`ERR_MIME_SPOOF_REJECTED`); delete quarantine blob. | Allowing executable binaries disguised as `.pdf` or `.png`. | Candidate must upload genuine PDF/image format. | C-001 |
| Malware / Macro Detected | `MalwareScanningEngine` | Quarantine lock (`ERR_MALICIOUS_PAYLOAD_DETECTED`); emit security incident alert. | Processing or rendering file in application workers. | Security operations reviews incident; file purged after 7d. | C-002, L4-C-007-BH-004 |
| Corrupt File Structure | `StructuralFormatValidator` | Rejection with diagnostic code (`ERR_FILE_CORRUPTED_TRUNCATED`). | Storing unreadable images or half-downloaded PDFs. | Candidate is prompted to re-upload via remediation window. | L4-C-007-BH-007 |
| Unresolved Tamper Signs | `IntegrityHashVerifier` | Flag upload (`FLAG_DIGITAL_SIGNATURE_INVALID`). | Silently accepting modified or invalidly signed credentials. | Admissions officer reviews flagged finding in verification queue. | L4-E-060-BH-003 |

### 5.4 Conceptual Contracts

#### `DocumentUploadRequest`
```text
DocumentUploadRequest {
  TenantId: UUID
  InstitutionId: UUID
  UploadCorrelationId: UUID
  UploaderPrincipalId: UUID
  TargetRequirementId: String (from L4-C-007 profile)
  DeclaredFileName: String
  DeclaredMimeType: String
  ContentStream: BinaryStream
}
```

#### `DocumentValidationReport`
```text
DocumentValidationReport {
  ValidationReportId: UUID
  UploadCorrelationId: UUID
  DocumentId: Optional<UUID> (assigned if valid)
  VerifiedMimeType: String
  ContentSha256Hash: String
  FileSizeBytes: Long
  StructuralIntegrityStatus: Enum [VALID, CORRUPTED, INVALID_FORMAT]
  MalwareScanStatus: Enum [CLEAN, INFECTED, SCAN_TIMEOUT]
  ActiveContentDetected: Boolean
  DigitalSignatureStatus: Enum [NO_SIGNATURE, VALID_SIGNATURE, INVALID_TAMPERED]
  OverallVerificationOutcome: Enum [VERIFIED_VALID, FLAGGED_ANOMALY, REJECTED_UNSAFE]
  AnomalyFlags: List<String>
  ValidatedAtUtc: Timestamp
}
```

### 5.5 State, Persistence & Consistency Boundaries

The ingestion pipeline is **stateless**. Incoming uploads are temporarily held in an encrypted quarantine storage bucket with an automated 24-hour time-to-live (TTL). Cleared documents are atomically promoted to permanent storage and registered in `L5-B-04-002`. Technical validation reports are persisted in `L5-B-02` relational persistence.

### 5.6 Asynchronous & Job Realization

- Uploads under 10MB (standard transcripts, ID proofs) are validated synchronously within 3 seconds.
- Multi-file batch uploads or large portfolios (> 10MB) are ingested into quarantine and validated asynchronously via `L5-A-08-001` work-units under `ADR-L5-0013`.

### 5.7 Security, Trust & Isolation

- **Quarantine Zone Isolation:** The quarantine storage bucket is segmented with read-deny policies for standard application users (`ADR-L5-0001`).
- **Safe Parsing Libraries:** Parsers operate in isolated sandbox containers (`L5-E-04-001`) with strict CPU and memory ceilings to eliminate buffer overflow risks.
- **Decompression Bomb Protection:** Zip and tar extractions strictly limit uncompressed byte expansion ratios (max 10:1) to prevent disk exhaustion.

### 5.8 Runtime & Deployment Implications

Validation workers deploy in isolated DMZ subnets (`L5-D-03-005`), separated from core transactional application databases.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-B04-030` | Mandatory Magic-Byte Structural Verification | File content validation inspects binary signatures and internal structural tags; file extensions are ignored. | C-001; Threat modeling | Eliminates file extension spoofing attacks where malicious executables are disguised as PDFs. | None (Local L5 Architecture Decision) |
| `AD-B04-031` | Isolated Quarantine Storage Zone | Uploaded files stream directly to an isolated quarantine bucket; promoted only after multi-layer security clearance. | C-002; ADR-L5-0001 | Prevents unverified binary payloads from ever reaching active document stores or web gateways. | Governed by ADR-L5-0001 |
| `AD-B04-032` | Non-Blocking Anomaly Flagging | Structural flaws that do not pose security threats (e.g. invalid embedded certificate) result in `FLAGGED_ANOMALY` rather than hard crash. | L4-C-007-BH-004 | Allows human admissions officers to review edge-case documents rather than locking out candidates. | None (Local L5 Architecture Decision) |
| `AD-B04-033` | Separation of Technical Validation from Business Acceptance | Engine emits objective verification metrics (`VERIFIED_VALID`); business domain workflows retain approval authority. | Corpus Register L5-B-04-003; C-003 | Enforces canonical ownership: document engine checks file validity; admissions officers approve student qualifications. | Governed by ADR-L5-0005 |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Malware Immunity | Multi-layer scanning and quarantine isolation prevent viral payload propagation across campus networks. | Cybersecurity baseline | `L5-B-04-003`, `L5-D-03-001` |
| Ingestion Latency | Synchronous validation of standard documents completes in p95 < 2.5 seconds. | Candidate experience target | `L5-E-02-001` |
| Tamper-Proof Audit Trail | Every uploaded file's SHA-256 hash and scan outcome is logged immutably in L5-D-04. | Legal compliance | `L5-D-04-001` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | The ingestion pipeline shall inspect binary magic bytes for every upload, rejecting files where magic bytes contradict claimed MIME type. | §5.1, `AD-B04-030` | C-001 |
| `CR-002` | Incoming uploads shall be written to an isolated quarantine storage zone prior to security clearance. | §5.1, `AD-B04-031` | C-002, ADR-L5-0001 |
| `CR-003` | All uploads shall undergo automated anti-malware and active-script scanning before promotion to permanent storage. | §5.1, §5.3 | `L4-C-007-BH-004` |
| `CR-004` | The engine shall compute and verify a cryptographic SHA-256 digest across the complete uploaded stream. | §5.1 | `L4-E-060-BH-003` |
| `CR-005` | Verified documents shall be promoted to permanent storage and registered in `L5-B-04-002` atomically. | §5.2 | `L5-B-04-002` |
| `CR-006` | Files identified as infected or containing malicious scripts shall be locked in quarantine and trigger security incident alerts. | §5.3 | C-002 |
| `CR-007` | The pipeline shall emit a structured `DocumentValidationReport` and shall not autonomously approve student admission or qualification. | §5.1, `AD-B04-033` | C-003, `L4-C-007-BH-003` |
| `CR-008` | In re-ingestion scenarios following remediation, the pipeline shall register corrected files as new versions linking prior uploads. | §5.1 | `L4-C-007-BH-007` |
| `CR-009` | Archive and zip uploads shall enforce maximum decompression expansion ratios of 10:1 to prevent zip-bomb denial of service. | §5.7 | ADR-L5-0014 |
| `CR-010` | Quarantine blobs that fail validation or remain unpromoted after 24 hours shall be automatically purged. | §5.5 | `L5-B-04-001` |
| `CR-011` | Ingestion endpoints shall enforce rate limits and file size ceilings defined in the active admission profile. | §5.1 | `L4-C-007-BH-001` |
| `CR-012` | Validation findings shall be persisted in `L5-B-02` relational persistence for admissions officer review. | §5.5 | `L4-C-007-BH-003` |
| `CR-013` | Technical verification evidence shall be emitted immutably to `L5-D-04` for forensic auditability. | §5.1 | `L5-D-04-001` |
| `CR-014` | The engine shall handle external certificates by verifying issuer digital signatures against trusted root anchors. | §5.1 | `L4-D-043-BH-002` |
| `CR-015` | The implementation shall trap file parsing exceptions cleanly without crashing worker processes. | §5.3 | ADR-L5-0014 |
| `CR-016` | File processing workers shall operate in sandboxed containers with zero access to internal enterprise databases. | §5.7 | `L5-D-03-005` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Antivirus Scanner Daemon Technology | Technology / Infrastructure | ClamAV sidecar vs Commercial Cloud Antivirus API | Infrastructure Architect / L5-E-04 | No | ClamAV containerized daemon selected as default baseline in Stage 6. |
| `OD-002` | Automated OCR Text Extraction Pipeline | Feature / Technology | Tesseract vs Cloud OCR for document text extraction | Architecture Board | No | OCR capabilities deferred to Stage 5 external integrations / analytics. |
| `OD-003` | DigiLocker Automated Fetch Connector | External Integration | Direct API ingestion from National Academic Depository / DigiLocker | L5-C-03 Owner | No | Connector architecture governed under L5-C-03 in Stage 5. |

---

## 10. Traceability Summary

- `L4-C-007-BH-001` — Realized by `UploadAdmissionGate` resolving candidate document requirement profiles (§5.1, `CR-011`).
- `L4-C-007-BH-002` — Realized by stream ingestion and chunked upload handling (§5.1, `CR-002`).
- `L4-C-007-BH-003` — Realized by `StructuralFormatValidator` and `VerificationReportEmitter` producing structured assessments (§5.1, `CR-001`, `CR-007`).
- `L4-C-007-BH-004` — Realized by `MalwareScanningEngine` detecting anomalies and routing to quarantine (§5.1, `CR-003`, `CR-006`).
- `L4-C-007-BH-007` — Realized by re-ingestion pipelines handling corrected upload windows (§5.1, `CR-008`).
- `L4-D-043-BH-001` — Realized by external certificate ingestion context binding (§5.1).
- `L4-D-043-BH-002` — Realized by issuer digital signature authenticity verification (§5.1, `CR-014`).
- `L4-E-060-BH-002` — Realized by employee qualification upload verification pipeline (§5.1, `CR-004`).
- `L4-E-060-BH-003` — Realized by tamper detection and SHA-256 integrity verification of submitted credentials (§5.1, `CR-004`).
