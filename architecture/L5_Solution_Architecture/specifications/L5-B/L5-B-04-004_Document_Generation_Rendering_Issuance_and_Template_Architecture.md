# L5-B-04-004: Document Generation, Rendering, Issuance, and Template Architecture

## 1. Document Metadata & Control

| Attribute | Value |
| :--- | :--- |
| **Specification ID** | `L5-B-04-004` |
| **Version** | `1.0.3` |
| **Status** | `APPROVED` |
| **Classification** | `Internal Confidential` |
| **Primary Architectural Module** | `Document Management Platform & Publishing Engine` |
| **Owning Solution Architect** | `Varad Deshpande` |
| **Cross-Cutting Domain** | `Enterprise Integration, Document Lifecycle, Templating & Publishing` |
| **Upstream Traceability** | `L4-C-018`, `L4-E-039` |
| **Downstream Implementation Target** | `core-document-engine`, `reporting-service`, `template-registry-service` |
| **Effective Date** | `2026-09-15` |

---

## 2. Executive Summary & Architectural Scope

The **Document Generation, Rendering, Issuance, and Template Architecture** (`L5-B-04-004`) specifies the enterprise-wide document compilation, dynamic compilation, high-fidelity rendering, and structured issuance platform for SutraERP. Across academic, financial, administrative, and payroll contexts, the platform must deterministically generate high-volume, pixel-perfect artifacts—including student transcripts, grade cards, tax invoices, fee receipts, employment offer letters, and institutional certificates.

The document subsystem enforces complete isolation between untrusted dynamic content and host environments through hermetic headless sandbox pools. It introduces an enterprise-grade Template Management Framework supporting syntax validation, semantic data hydration, internationalization (i18n), automated pagination, and PDF/A-3 archival compliance.

```
+---------------------------------------------------------------------------------------------------+
|                            L5-B-04-004 DOCUMENT GENERATION & RENDERING ENGINE                    |
|                                                                                                   |
|  [ Upstream Triggers ]          [ Ingestion & Queueing ]              [ Sandboxed Rendering ]     |
|   - Finance Fee Receipts        - Asynchronous Render Broker           - Chromium Headless Pool   |
|   - Exam Grade Cards/Transcripts- BullMQ / Redis Cluster               - Hermetic Worker Cgroups  |
|   - HR Payslips & Letters       - Priority Dispatcher                  - Zero-Network Sandbox     |
|             |                                |                                   |                |
|             v                                v                                   v                |
|  +----------------------+      +---------------------------+      +-----------------------------+ |
|  | Template Registry    | ---> | Data Hydration & Schema   | ---> | Virtual DOM & CSS Paged     | |
|  | - Handlebars / MJML  |      | - JSON Schema Validation  |      |   Media Engine              | |
|  | - Strict Linting     |      | - XSS/Injection Filter    |      | - Auto-Pagination Engine    | |
|  | - Versioned Git Store|      | - Currency/Locale Format  |      | - Vector Asset Inliner      | |
|  +----------------------+      +---------------------------+      +-----------------------------+ |
|                                                                                  |                |
|                                                                                  v                |
|  [ Downstream Issuance ]       [ Integrity & Archival ]               [ Artifact Transformation ] |
|   - Webhook Notifications       - Metadata Embedding (XMP)             - PDF/A-3 Format Engine    |
|   - Storage Tier Upload         - SHA-256 Checksum Compute             - Font Subset Inliner      |
|   - L5-B-04-005 Signing Gate    - Audit Issuance Ledger Entry          - CMYK / ICC Color Profile |
+---------------------------------------------------------------------------------------------------+
```

---

## 3. Upstream Traceability Matrix

This specification establishes direct, bidirectional traceability to L4 behavioral specifications as detailed below:

| L4 Specification ID | Behavioral Header | L4 Requirement Description | L5 Section Reference | Verification Method |
| :--- | :--- | :--- | :--- | :--- |
| `L4-C-018` | `L4-C-018-BH-001` | Dynamic template registration and compilation pipeline | Section 5.1 | Automated template compilation unit test |
| `L4-C-018` | `L4-C-018-BH-002` | Template schema validation and context hydration | Section 5.3, 7.1 | JSON Schema structural validation harness |
| `L4-C-018` | `L4-C-018-BH-003` | Isolated sandbox execution for untrusted document layouts | Section 5.2, 9.1 | Container security & network namespace audit |
| `L4-C-018` | `L4-C-018-BH-004` | Asynchronous high-throughput batch rendering dispatcher | Section 5.4, 8.1 | 5,000-job stress test in staging cluster |
| `L4-C-018` | `L4-C-018-BH-005` | Deterministic pagination, header/footer stamping & page numbering | Section 5.5 | PDF visual regression & pixel diff verification |
| `L4-C-018` | `L4-C-018-BH-006` | Issuance ledger recording and webhook distribution | Section 5.6, 7.3 | Event bus delivery and audit log verification |
| `L4-E-039` | `L4-E-039-BH-002` | Academic transcript and grade card multi-page rendering | Section 5.5 | Transcripts layout boundary test |
| `L4-E-039` | `L4-E-039-BH-003` | Financial invoice and fee receipt generation standards | Section 5.3, 8.1 | GST receipt formatting and precision verification |
| `L4-E-039` | `L4-E-039-BH-004` | PDF/A-3 long-term archival conversion and metadata embedding | Section 5.5, 8.4 | VeraPDF conformance validator harness |
| `L4-E-039` | `L4-E-039-BH-005` | Document revocation, invalidation, and issuance life-cycle management | Section 5.6, 7.4 | Lifecycle transition state machine verification |

---

## 4. Architectural Principles & Constraints

1. **Zero-Trust Hermetic Rendering**: No rendering process is permitted outbound network access. Remote images, stylesheets, or external scripts are strictly blocked; all referenced assets must be resolved, validated, and embedded as Base64/data URIs prior to sandbox handoff.
2. **Deterministic Output & Reproducibility**: Rendering identical data against an identical versioned template must yield bitwise equivalent or functionally indistinguishable visual artifacts, including font metrics, kerning, and line breaks.
3. **Decoupled Asynchronous Processing**: Synchronous rendering is restricted exclusively to single-page, real-time receipts (latency < 800ms). All multi-page, bulk, or batch requests are routed through asynchronous priority queues.
4. **Mandatory Archival Standards**: Official institution records, grade sheets, and diplomas must conform to ISO 19005-3 (PDF/A-3), embedding strict XML metadata (XMP), color profiles, and font subsets.

---

## 5. Detailed Solution Architecture & Component Design

### 5.1 Template Management & Registry Engine
The Template Registry operates as an immutable versioned repository for document definitions. Templates are structured into two distinct domains:
- **Layout Definition**: HTML5 / CSS3 Paged Media templates for PDF documents, or MJML for responsive notifications.
- **Data Binding Schema**: Strict JSON Schema definition specifying mandatory and optional attributes, data types, and formatting masks.

Templates are stored with semantic versioning (`MAJOR.MINOR.PATCH`). Modifications to variable bindings or page structures require a version bump. The engine compiles templates into cached abstract syntax trees (ASTs), disallowing arbitrary JavaScript execution (`eval`, `Function`, `process`) within template helper blocks.

### 5.2 Hermetic Headless Sandbox Rendering Pool
Rendering execution utilizes a worker pool of pre-warmed Chromium instances running inside restricted Linux cgroups and seccomp profiles:
- **Process Isolation**: Each worker runs under a distinct unprivileged system user with temporary, isolated `tmpfs` directories that are purged immediately upon job termination.
- **Resource Constraints**: Workers are constrained to 512 MB RAM, 1 vCPU, and a hard execution timeout of 15 seconds per job.
- **Network Blackholing**: Rendering processes run inside a dedicated network namespace with loopback-only traffic, eliminating Server-Side Request Forgery (SSRF) and local file inclusion (`file://`).

### 5.3 Data Hydration, Normalization & Sanitization Pipeline
Incoming document payloads undergo strict preprocessing prior to rendering:
1. **Schema Validation**: Payloads are validated against the template's target JSON Schema.
2. **Context Enrichment**: System metadata (tenant identifier, generation timestamp, operator ID, correlation hash) is stamped into the context.
3. **Text Sanitization**: String variables are HTML-escaped by default. Markdown or structured HTML fields must pass through an AST-level sanitizer that strips script tags, object embeds, and untrusted iframes.
4. **Localization Formatting**: Currencies, dates, timestamps, and numbers are pre-formatted according to the tenant's regional and institutional locale settings.

### 5.4 Asynchronous Render Broker & Priority Queueing
High-volume document generation requests are managed via an asynchronous worker cluster backed by Redis/BullMQ:
- **Priority Classes**:
  - `P0 (Urgent)`: Single-receipt checkout, one-time verification tokens.
  - `P1 (Standard)`: End-of-day billing invoices, individual transcript requests.
  - `P2 (Batch)`: Institution-wide payroll generation, semester examination grade cards.
- **Concurrency & Backpressure**: Worker concurrency scales dynamically based on CPU utilization and queue depth. If worker memory exceeds 80%, incoming batch jobs are throttled.

### 5.5 Artifact Transformation & PDF/A-3 Compliance Engine
Post-rendering, the artifact enters the transformation pipeline:
- **Font Subsetting**: All vector fonts (TrueType/OpenType) used in the document are subsetted to include only the glyphs present, reducing file size and ensuring immutable presentation.
- **Metadata Embedding**: Document title, creator, tenant ID, unique document tracking ID (UDTI), and XMP metadata are injected.
- **Archival Conversion**: The stream is validated for PDF/A-3 compliance, embedding an sRGB ICC color profile and disabling active scripting/encryption layers that breach ISO 19005-3.

### 5.6 Document Issuance, Distribution & Audit Ledger
Following successful compilation:
1. The artifact is written to encrypted object storage (`L5-B-02`) with an SHA-256 integrity digest.
2. An issuance entry is written to the transactional database recording document ID, recipient, template version, digest, and generation duration.
3. Downstream signing jobs are triggered via `L5-B-04-005` if the document metadata requires digital signatures or institutional seals.
4. Webhook events (`document.issued`, `document.failed`) are published to the enterprise event bus.

---

## 6. Architectural Decisions

| Decision ID | Decision Title | Status | Rationale | Alternatives Considered | Trade-offs |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `AD-L5-B04-004-01` | Headless Chromium Sandbox Isolation over In-Process Libraries | `ACCEPTED` | Ensures full support for modern CSS Paged Media, flexbox, and web typography while guaranteeing robust security isolation. | In-process PDF libraries (PDFKit, ReportLab, WeasyPrint). | Higher memory overhead per worker process, requiring aggressive pooling and lifecycle recycling. |
| `AD-L5-B04-004-02` | Strict Logic-Less Templating with Handlebars AST | `ACCEPTED` | Prevents arbitrary code execution in user-authored templates while providing extensible, sandboxed helpers for formatting. | Full-featured templating engines allowing embedded script execution (EJS, Jinja2). | Complex presentation logic must be computed in the data preparation layer before template hydration. |
| `AD-L5-B04-004-03` | Tiered Priority Queue Architecture for Render Jobs | `ACCEPTED` | Prevents bulk semester payroll and grade card runs from starving real-time student fee payment receipt generation. | First-In-First-Out (FIFO) single-queue processing. | Requires dedicated queue monitoring and dynamic resource partition management. |
| `AD-L5-B04-004-04` | Universal PDF/A-3 Conformance for Institutional Artifacts | `ACCEPTED` | Guarantees multi-decade readability and legal admissibility of academic records and tax invoices across global jurisdictions. | Standard PDF 1.7 / 2.0 without archival constraints. | File size increases by ~25-40 KB per document due to embedded font subsets and ICC color profiles. |

---

## 7. Conceptual Data Models & Schema Contracts

### 7.1 Template Entity Definition Contract
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "DocumentTemplateDefinition",
  "type": "object",
  "required": ["template_id", "tenant_id", "version", "engine", "content_type", "schema_definition", "layout_source"],
  "properties": {
    "template_id": { "type": "string", "pattern": "^tpl_[a-zA-Z0-9_-]{16,32}$" },
    "tenant_id": { "type": "string", "pattern": "^tnt_[a-zA-Z0-9_-]{8,16}$" },
    "version": { "type": "string", "pattern": "^[0-9]+\\.[0-9]+\\.[0-9]+$" },
    "engine": { "type": "string", "enum": ["CHROMIUM_PAGED_MEDIA", "MJML_EMAIL", "CSV_STREAM"] },
    "content_type": { "type": "string", "enum": ["application/pdf", "text/html", "text/csv"] },
    "schema_definition": { "type": "object", "description": "JSON Schema enforcing input payload structure" },
    "layout_source": { "type": "string", "description": "UTF-8 encoded HTML5/Handlebars or MJML template" },
    "requires_digital_seal": { "type": "boolean", "default": false },
    "retention_policy_days": { "type": "integer", "minimum": 1 }
  }
}
```

### 7.2 Render Job Dispatch Payload Schema
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "RenderJobDispatchPayload",
  "type": "object",
  "required": ["job_id", "template_id", "template_version", "priority", "payload_data", "output_destination"],
  "properties": {
    "job_id": { "type": "string", "format": "uuid" },
    "template_id": { "type": "string" },
    "template_version": { "type": "string" },
    "priority": { "type": "string", "enum": ["P0_URGENT", "P1_STANDARD", "P2_BATCH"] },
    "correlation_id": { "type": "string" },
    "payload_data": { "type": "object", "description": "Context variables conforming to template schema" },
    "output_destination": {
      "type": "object",
      "required": ["storage_bucket", "storage_key"],
      "properties": {
        "storage_bucket": { "type": "string" },
        "storage_key": { "type": "string" },
        "generate_presigned_url": { "type": "boolean", "default": false }
      }
    }
  }
}
```

### 7.3 Issuance Receipt & Verification Schema
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "DocumentIssuanceReceipt",
  "type": "object",
  "required": ["document_id", "job_id", "tenant_id", "sha256_digest", "byte_size", "issued_at", "status"],
  "properties": {
    "document_id": { "type": "string", "pattern": "^doc_[a-zA-Z0-9_-]{20,36}$" },
    "job_id": { "type": "string", "format": "uuid" },
    "tenant_id": { "type": "string" },
    "sha256_digest": { "type": "string", "pattern": "^[a-f0-9]{64}$" },
    "byte_size": { "type": "integer", "minimum": 1 },
    "page_count": { "type": "integer", "minimum": 1 },
    "render_duration_ms": { "type": "number" },
    "issued_at": { "type": "string", "format": "date-time" },
    "storage_uri": { "type": "string", "format": "uri" },
    "status": { "type": "string", "enum": ["ISSUED", "PENDING_SIGNATURE", "ARCHIVED", "REVOKED"] }
  }
}
```

---

## 8. Concrete Operational & Cross-Cutting Requirements

| Requirement ID | Category | Objective / Specification | Verification Target |
| :--- | :--- | :--- | :--- |
| `CR-B04-004-PERF-01` | Performance | Real-time P0 single-page document generation latency | <= 850 ms (p95) from receipt to storage commit |
| `CR-B04-004-PERF-02` | Throughput | Batch grade card/payslip throughput on multi-worker cluster | >= 120 pages per second aggregate throughput |
| `CR-B04-004-SEC-01` | Security | Headless worker network sandbox isolation | 100% outbound TCP/UDP requests blocked; 0 bytes leaked |
| `CR-B04-004-SEC-02` | Security | Sanitization of dynamic user variables | Zero unescaped HTML/JavaScript injection vulnerabilities |
| `CR-B04-004-REL-01` | Reliability | Error rate and automatic job recovery | < 0.01% failed jobs; max 3 automatic retries with exponential backoff |
| `CR-B04-004-COMP-01` | Compliance | Archival format conformance | 100% compliance with ISO 19005-3 (PDF/A-3) validated via VeraPDF |

---

## 9. Security, Governance & Failure Modes

### 9.1 Attack Vectors & Mitigations
- **Server-Side Request Forgery (SSRF)**: Sandboxed render workers execute with Linux network namespaces disabled (`netns` loopback only). DNS resolution within the worker process is completely eliminated.
- **Resource Exhaustion & Denial of Service**: Malicious CSS loops (`@page { size: 1px; }` generating millions of pages) are mitigated by hard worker memory caps (512 MB) and maximum page count ceilings (default: 500 pages per job).
- **Template Poisoning**: All template registrations require administrative dual-authorization. Code execution macros are compiled out by the Handlebars parser.

### 9.2 Failure Modes & Resilience Patterns
- **Headless Worker Crash**: If a Chromium process crashes due to an out-of-memory exception or segmentation fault, the supervisor process catches the exit code, logs a diagnostic stack trace, restarts the worker, and requeues the job into a dead-letter quarantine if it fails repeatedly.
- **Storage Subsystem Unavailability**: In the event that the primary object store (`L5-B-02`) experiences a brief outage, generated artifacts are temporarily buffered in encrypted local scratch storage and retried with backoff.

---

## 10. Traceability Verification & Test Harness Specifications

### 10.1 Deterministic Rendering & Visual Regression Test Harness
The testing harness maintains a baseline repository of golden-master PDFs:
```bash
# Execute visual regression test suite across all standard institutional templates
npm run test:visual-regression -- --tolerance=0.001 --suite=academic_transcripts
```
The harness performs pixel-by-pixel comparisons against established golden images, asserting a maximum diff delta of <= 0.05% to catch kerning regressions, font misalignment, or table wrapping defects.

### 10.2 Archival PDF/A-3 Conformance Automated Audit
Every template update triggers an automated build pipeline step running the VeraPDF validation engine:
```bash
verapdf --flavour 3b --format text ./artifacts/generated_sample_transcript.pdf
```
Asserts 100% PASS with zero validation profile failures regarding color spaces, embedded font dictionaries, or metadata tags.
