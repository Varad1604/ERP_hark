# SutraERP L5 — L4-to-L5 Coverage Audit Decision — 105 Candidate Specifications — 2026-09-09

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Decision Date:** 2026-09-09  
**Previous Candidate Corpus:** 99 specifications  
**Revised Candidate Corpus:** 105 specifications  

---

# 1. Decision

The previous 99-specification SutraERP L5 candidate corpus is insufficient as the active L5 planning baseline.

A repository-backed L4-to-L5 coverage and corpus-integrity review confirmed four material technical coverage gaps. The smallest coherent reusable remediation adds six net candidate specifications, producing a revised candidate corpus of **105 specifications**.

The 99-specification register remains historical planning evidence and shall not be deleted.

This decision approves the revised 105-specification candidate population for the next controlled L5 governance/methodology phase. It does not authorize mass authoring, technology selection, implementation or final L5 lock.

---

# 2. Authority and Architecture Boundary

The audit preserves the repository dependency direction:

```text
L0 → L1 → L2 → L3 → L4 → L5 → L6
```

L5 implements approved system behavior and shall not redefine:

- business-domain ownership;
- canonical business entities;
- business rules;
- business lifecycle states/transitions;
- canonical business events or publishers;
- institutional authority;
- L3 outcomes;
- L4 behavioral obligations.

The audit uses reusable technical ownership rather than one-L4-file-to-one-L5-file mapping.

---

# 3. Live-State Findings

At audit time the live L5 governance baseline contained:

- the Architecture Approved 30-major-area baseline;
- the Architecture Approved seven-family register;
- the Locked L5 naming-specialization approval decision;
- the Architecture Approved L5 naming/document-granularity specialization;
- the Proposed 99-specification candidate register.

The hierarchical L5 identifier model was already approved. Search & Indexing is correctly assigned to L5-B — Data & Information Architecture and remains there.

---

# 4. Confirmed Gap 1 — Document Platform

## Previous state

Area 12 was named:

```text
Document, Media & Object Storage Architecture
```

and the 99-register contained only:

```text
L5-B-04-001 Document, Media & Object Storage Architecture
L5-B-04-002 Managed Document & Object Lifecycle, Access & Integrity Architecture
```

## Finding

Upstream architecture requires a broader reusable technical responsibility including document registry, versioning, storage, ingestion, validation, verification, generation, rendering, issuance, templates, digital signing application, seals, QR/barcode credentials, archival and integrity.

The business domain remains the owner of document eligibility, acceptance, approval, issuance decision and related business lifecycle. L5-B-04 owns only the reusable technical document platform. Cryptographic primitives, certificates, secrets and key management remain under L5-D-03.

## Decision

Area 12 is clarified as:

```text
Document, Media & Object Platform Architecture
```

The revised B04 candidate set is:

```text
L5-B-04-001 Document, Media & Object Platform Architecture Baseline
L5-B-04-002 Document Registry, Versioning, Storage & Retention Architecture
L5-B-04-003 Document Ingestion, Validation & Verification Architecture
L5-B-04-004 Document Generation, Rendering, Issuance & Template Architecture
L5-B-04-005 Digital Signing, Seal, QR & Credential Integrity Architecture
```

**Net candidate change: +3.**

---

# 5. Confirmed Gap 2 — Governed Bulk Data Import / Export

## Finding

L4 requires governed operational import/export behavior including authorized source/consumer scope, structural validation, semantic-validation handoff, duplicate treatment, governed acceptance/rejection, export scope limitation, delivery integrity and audit evidence.

Database schema migration is not the owner of operational import/export, and generic connector architecture alone does not provide a sufficiently explicit reusable owner for controlled file/bulk exchange.

## Decision

Add:

```text
L5-C-03-008 Bulk Data Import, Export & File Exchange Architecture
```

The specification owns the technical exchange mechanism only. Source and destination business domains retain ownership of their data and semantic business rules.

**Net candidate change: +1.**

---

# 6. Confirmed Gap 3 — Device / Edge / Physical-System Integration

## Finding

Upstream scenarios and L4 behavior explicitly defer device/physical-system implementation mechanisms including scanners/readers, RFID/NFC, biometrics, ANPR, GPS, telemetry, IoT sensors, surveillance/evidence feeds and physical controllers.

These mechanisms recur across Campus Operations, Transport, Security, Examination, Infrastructure and Enterprise Resource Management and therefore require one reusable integration architecture rather than device-specific L5 documents.

## Decision

Add:

```text
L5-C-03-009 Device, Edge, Telemetry & Physical-System Integration Architecture
```

Mandatory ownership principle:

> A device supplies observations or executes controlled technical commands. It does not become the canonical owner of the business decision or business lifecycle.

**Net candidate change: +1.**

---

# 7. Confirmed Gap 4 — Consent / Purpose-Limitation Technical Enforcement

## Finding

Existing privacy candidates cover classification, masking, retention and Aadhaar but do not provide a sufficiently explicit technical owner for cross-channel enforcement of authoritative consent state, withdrawal, approved purpose, minimization and disclosure constraints.

## Decision

Add:

```text
L5-B-05-005 Consent, Purpose-Limitation & Privacy-Control Architecture
```

The originating business domain retains canonical ownership of consent/purpose facts. The L5 specification enforces those authoritative facts through APIs, integrations, reporting, document access, notifications and client access where required.

**Net candidate change: +1.**

---

# 8. Revised Family Counts

| Family | Previous | Revised |
|---|---:|---:|
| L5-A — Core Platform & Application | 20 | 20 |
| L5-B — Data & Information | 17 | 21 |
| L5-C — Contracts, Events & Integration | 16 | 18 |
| L5-D — Identity, Security & Trust | 19 | 19 |
| L5-E — Reliability, Deployment & Operations | 16 | 16 |
| L5-F — Engineering Delivery & Quality | 7 | 7 |
| L5-G — Client Platform | 4 | 4 |
| **TOTAL** | **99** | **105** |

Arithmetic:

```text
99
+3 Document Platform refinement
+1 Consent / Purpose Limitation
+1 Bulk Import / Export
+1 Device / Edge Integration
=105
```

---

# 9. Duplicate-Ownership and Granularity Review

The revised 105 corpus passes the candidate-level integrity review on the following basis:

- no new L5 family is required;
- no new major area is required beyond the controlled Area-12 clarification;
- the four gap remedies represent reusable technical capabilities rather than one-off business specializations;
- no new canonical business owner is created;
- cryptography remains L5-D-03 owned even where document-platform capabilities consume signing/key services;
- governed bulk exchange remains distinct from internal database/schema migration;
- device integration remains distinct from business authorization/identity/asset/trip/examination ownership;
- consent enforcement remains distinct from the source business consent fact;
- Search & Indexing remains L5-B as already approved;
- no evidence requires standalone L5 specifications for LMS, webhooks, localization, ETL, enterprise calendar, recruitment integration, individual device products or individual payment gateway vendors.

No additional candidate specification is justified by the reviewed evidence at this checkpoint.

---

# 10. L4 Maturity Qualification

The 105 candidate corpus is a solution-architecture planning/governance decision. Detailed L5 specification traceability must continue to respect the current approved L4 behavioral baseline.

Where relevant live L4 files remain Draft or later change through governed review, downstream L5 traceability shall use the subsequently approved/current L4 state and must not treat Draft wording as an immutable upstream decision.

This qualification does not invalidate the reusable technical-responsibility coverage findings recorded in this audit.

---

# 11. Items Not Promoted to Separate L5 Specifications

No new standalone specification is currently approved for:

- LMS as a separate major area;
- webhooks;
- localization;
- ETL separate from Reporting/Analytics/Data Projection;
- centralized identifier/number-sequence business ownership;
- enterprise calendar business ownership;
- recruitment-specific integration;
- individual hardware product types;
- individual payment-gateway vendors;
- Aadhaar as a peer architecture family;
- AI-agent Git/branch/PR governance.

Future authoritative requirements may reopen any of these conclusions through controlled architecture review.

---

# 12. Next Governance Sequence

After this decision, the recommended controlled sequence is:

1. synchronize L5 governance/navigation to the 105 candidate baseline;
2. define and lock the L5 Governance & Architecture Standard;
3. define the L5 Solution Architecture Specification Template;
4. define the L5 Validation Standard and scoring methodology;
5. define technology-selection and ADR methodology;
6. author a small representative L5 pilot set;
7. validate the methodology against the pilot;
8. lock the L5 authoring methodology;
9. finalize/lock the canonical L5 specification register;
10. score workload complexity and approve five-person allocation;
11. authorize controlled broad L5 authoring.

---

# 13. Decision Statement

**PREVIOUS CANDIDATE CORPUS:** **99**  
**CONFIRMED MATERIAL COVERAGE GAPS:** **4**  
**NET CANDIDATE ADDITIONS:** **6**  
**REVISED CANDIDATE CORPUS:** **105**  
**DUPLICATE TECHNICAL OWNERSHIP FOUND IN REVISED CORPUS:** **0 MATERIAL FINDINGS**  
**MASS L5 AUTHORING:** **NOT AUTHORIZED**  
**FINAL L5 CORPUS LOCK:** **NOT AUTHORIZED**  
**STATUS:** **ARCHITECTURE APPROVED — ACTIVE L5 COVERAGE DECISION**
