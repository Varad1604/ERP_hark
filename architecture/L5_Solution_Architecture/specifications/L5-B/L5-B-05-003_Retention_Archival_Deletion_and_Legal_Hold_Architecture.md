# L5-B-05-003 — Retention, Archival, Deletion & Legal Hold Architecture

## 1. Specification Metadata

**Specification ID:** L5-B-05-003  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Technical enforcement of retention, archival, governed deletion and legal-hold controls without redefining business policy  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.0  
**Status:** Draft  
**Last Updated:** 2026-09-12  
**Depends On:** L5-B-05-001 (Classification Baseline); L5-B-02 (Persistence); L5-A-08 (Background Processing); L5-D-03 (Security & Cryptography); L5-D-04 (Audit & Evidence)  
**Referenced By:** L5-B-04, L5-B-02, L5-E-03, L5-A-05  
**Applicable ADRs:** ADR-L5-0001 (Canonical Trust Boundary & Segmentation Pattern), ADR-L5-0002 (Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern), ADR-L5-0003 (Contract-Driven Modular Platform with Selective Isolation), ADR-L5-0004 (Tenant and Institution Isolation Enforcement Pattern), ADR-L5-0005 (Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern)  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable technical architecture for enforcing authoritative data retention schedules, cold-tier archival migrations, governed physical deletions, and non-destructive legal hold locks across SutraERP data holdings.

It establishes background sweep workers, immutable legal-hold junction models, two-phase deletion execution, and tamper-evident destruction certificates, ensuring that business history is preserved and records subject to legal preservation are never purged.

### 2.2 In Scope

- Automated background retention evaluation sweep engine coordinated with L5-A-08 job execution.
- Relational schema and concurrency mechanics for active Legal Hold locks (RecordHoldLock).
- Strict precedence enforcement: active legal hold overrides and blocks all archival migrations, soft-deletions, and physical purges.
- Two-phase deletion execution pipeline: Phase 1 (Governed Logical Deactivation / Tombstoning) and Phase 2 (Coordinated Physical Purge / Cryptographic Erasure).
- Archival tier migration mechanics to cold object storage while maintaining queryable index lineage.
- Audit evidence generation recording retention evaluations, hold placements, hold releases, and physical purge completions.

### 2.3 Explicit Non-Goals

This specification does not own or redefine:

- Institutional retention policies, statutory retention schedules, or tax/compliance retention durations (owned by upstream business governance).
- Legal hold declaration, litigation case management, or authorization to lift legal holds (owned by institutional governance).
- Database table DDL, indexing strategies, or tablespace partitioning (owned by L5-B-02).
- Physical backup execution, snapshot frequency, or disaster recovery replication (owned by L5-E-03).
- Object storage bucket provisioning or cloud storage infrastructure (owned by L5-B-04 / L5-E-04).

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Architectural Constraints

Subordinate to locked L1 domain architecture and approved L2/L3/L4 system behavior. L5 does not create or alter business retention rules or legal authority.

The canonical L5 Candidate Document Corpus Register defines L5-B-05-003 as the technical owner of retention, archival, governed deletion, and legal-hold execution.

### 3.2 Immutability and Precedence Rule

A legal hold is an absolute technical block. If a record has one or more active legal hold bindings, no automated retention policy, administrative cleanup task, or tenant deprovisioning workflow shall physically delete, truncate, or overwrite that record.

### 3.3 Operational Assumptions

- **AS-B05-005 — Cluster-Wide Clock Consistency:** Database engines, background workers (L5-A-08), and storage managers share a synchronized clock baseline to reliably evaluate retention schedules, calculate expiry dates, and verify quarantine expiration without temporal drift.

---

## 4. Technical Architecture

### 4.1 Retention Evaluation Sweep and Hold Architecture

Retention evaluation and legal-hold locking operate across two distinct operational pipelines:

```text
[ Authoritative Policy Event / Scheduled Trigger (L5-A-08) ]
                              |
                              v
+-------------------------------------------------------------------------+
|                  RetentionEvaluationSweepWorker                         |
|                                                                         |
|  1. Scan Entities by Tenant, Institution, and RetentionClass            |
|  2. Calculate Expiry Date = CreationDate + PolicyRetentionPeriod        |
|  3. Check Active Legal Holds in RecordHoldLock                          |
+-------------------------------------------------------------------------+
                              |
        +---------------------+---------------------+
        |                                           |
  [Hold Active]                               [No Active Hold]
        |                                           |
        v                                           v
+------------------------------------+   +------------------------------------+
| Block Disposal & Log Hold Event    |   | Evaluate Expiry Status             |
| (Preserve Record State)            |   +------------------------------------+
+------------------------------------+              |                 |
                                                    v                 v
                                            [Expired]         [Archival Eligible]
                                                    |                 |
                                                    v                 v
                                            +---------------+ +---------------+
                                            | Two-Phase     | | Cold Archival |
                                            | Deletion      | | Migration     |
                                            +---------------+ +---------------+
```

### 4.2 Legal Hold Schema and Locking Mechanics

Legal holds are realized via an immutable hold lock registry bound to canonical record identifiers:

```text
RecordHoldLock {
    lock_id: UUID
    tenant_id: UUID
    institution_id: UUID
    target_entity_type: STRING
    target_record_id: UUID
    hold_reference_id: STRING (Legal Case / Regulatory Investigation Ref)
    applied_by: UUID (Authoritative Officer)
    applied_timestamp: TIMESTAMP
    status: ENUM [ ACTIVE, RELEASED ]
    release_timestamp: TIMESTAMP (NULL while ACTIVE)
    released_by: UUID (NULL while ACTIVE)
    immutable_hash: STRING (Cryptographic integrity check)
}
```

**Locking Rule:** A query evaluating deletion eligibility must perform an atomic check:
`SELECT COUNT(*) FROM RecordHoldLock WHERE target_record_id = :id AND status = 'ACTIVE'`  
If count > 0, deletion is rejected immediately with a `RECORD_UNDER_LEGAL_HOLD` exception.

### 4.3 Architecture Decision: Dual-Phase Retention Evaluation and Precedence-Enforced Legal Hold

**Decision ID:** AD-B05-003  
**Title:** Dual-Phase Retention Evaluation and Precedence-Enforced Legal Hold Architecture  
**Status:** Approved  
**Decision:** All record disposals must follow a mandatory two-phase workflow: Phase 1 transitions records to GOVERNED_SOFT_DELETED and starts a mandatory quarantine window. Phase 2 executes irreversible physical deletion or cryptographic erasure only after verifying zero active RecordHoldLock bindings and confirming quarantine expiration. Active legal holds take strict precedence over any retention schedule, archival transition, or deletion command.  
**Rationale:** Prevents accidental or malicious spoliation of legal evidence and ensures catastrophic deletion mistakes can be intercepted during the quarantine period.

### 4.4 Deletion Pipeline Execution

1. **Phase 1: Governed Soft-Delete (Logical Tombstone):**
   - The record status transitions to SOFT_DELETED.
   - Access is restricted strictly to legal/compliance audit queries.
   - Quarantine period timer initiates (e.g. 30 days; exact quarantine duration is an institutionally configurable governance parameter resolved via L5-A-06).
2. **Phase 2: Coordinated Physical Purge:**
   - Background worker validates that the quarantine period has lapsed.
   - Worker verifies that no RecordHoldLock is active.
   - For structured relational data: physical row deletion or irreversible anonymization is executed.
   - For encrypted data: cryptographic erasure is executed via L5-D-03 by destroying the record's specific data encryption key (DEK).
   - An immutable DataPurgeCertificate is emitted to L5-D-04.

---

## 5. Dependencies & Integration

### 5.1 Peer Dependencies
- **L5-B-05-001:** Provides baseline classification tags and scope verification.
- **L5-B-02:** Executes database row soft-deletion, physical purging, and lock table queries.
- **L5-A-08:** Schedules periodic retention sweep jobs.
- **L5-D-03:** Executes cryptographic erasure via key destruction.
- **L5-D-04:** Records immutable audit logs for hold application, hold release, and physical deletion.

---

## 6. Technical Behavior & Failure Concerns

- **TB-001: Absolute Hold Precedence:** An active hold halts all automated and manual deletion workflows.
- **TB-002: Overlapping Holds:** When multiple holds apply to a record, release of one hold leaves all remaining active holds fully enforced.
- **TB-003: Sweep Failure Containment:** Failure during a batch sweep marks the batch for retry without skipping unexamined records.
- **TB-004: Audit Trail Preservation:** Purging an operational record must never delete the attributable audit trail in L5-D-04 documenting that record's lifecycle.

---

## 7. Security, Privacy & Trust Boundaries

- **Hold Authority:** Hold application and release require elevated legal/administrative roles verified via L5-D-02.
- **Audit Logging:** Every hold lifecycle event emits an immutable audit record containing officer identity, reference case ID, and target record UUID.
- **Cryptographic Erasure:** Ensures compliance with privacy deletion mandates even where storage media cannot be physically degaussed.

---

## 8. Conformance Requirements

- **CR-001:** The system shall require verified authoritative retention context before initiating any automated archival or deletion process.
- **CR-002:** Active legal holds recorded in RecordHoldLock shall strictly block all archival, soft-deletion, and physical purge actions.
- **CR-003:** Where multiple legal holds exist for a single record, releasing one hold shall not clear other active holds on that record.
- **CR-004:** Record deletion shall strictly follow a two-phase process (logical quarantine followed by physical purge or cryptographic erasure).
- **CR-005:** Technical archival migrations shall preserve record identity, historical meaning, and audit provenance.
- **CR-006:** Physical deletion of business data shall never delete or corrupt associated historical audit evidence in L5-D-04.
- **CR-007:** The retention evaluation engine shall maintain strict Tenant and Institution scope isolation during all batch sweep operations.
- **CR-008:** The system shall emit an attributable DataPurgeCertificate record to L5-D-04 upon successful completion of any Phase 2 physical deletion.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Candidate L5 Spec ID | Material Technical Contribution | Realization Status |
| :--- | :--- | :--- | :--- | :--- |
| L4-A-025 | L4-A-025-BH-001 | L5-B-05-003 | Resolves Tenant/Institution scope, classification, and applicable retention policy before evaluating disposal eligibility. | Verified Realization |
| L4-A-025 | L4-A-025-BH-002 | L5-B-05-003 | Enforces active legal hold locks (RecordHoldLock) to prevent premature release or deletion of protected evidence. | Verified Realization |
| L4-A-025 | L4-A-025-BH-003 | L5-B-05-003 | Implements immutable Retention Governance Decision recording (Under Retention, Archival Eligible, Retention Completed). | Verified Realization |
| L4-A-026 | L4-A-026-BH-006 | L5-B-05-003 | Enforces archival as non-destructive cold storage migration while preserving historical lineage and references. | Verified Realization |
| L4-A-053 | L4-A-053-BH-004 | L5-B-05-003 | Validates retention and dependency prerequisites before marking Person and Identity records eligible for archival. | Verified Realization |
| L4-A-053 | L4-A-053-BH-007 | L5-B-05-003 | Governs exceptional permanent deletion workflows, verifying security confirmations, retention clearance, and absence of holds. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Exact L4 BH Applicability:** Resolved. Atomic L4 BH relations mapped to L4-A-025-BH-001, L4-A-025-BH-002, L4-A-025-BH-003, L4-A-026-BH-006, L4-A-053-BH-004, and L4-A-053-BH-007.
- **OD-002 — L4/L5 Version Binding:** Resolved. Mapped against locked canonical L4 production specifications and L5-B-05-003 v1.0.0.
- **OD-003 — Batch Sweep Chunk Size:** Non-blocking. Sweep batch size (e.g. 5,000 records per transaction chunk) configurable via L5-A-06.
- **OD-004 — Cold Storage Protocol Adapter:** Non-blocking. Cold object storage archival tier adapters managed under L5-B-04 / L5-C-03.

**Current State:** Ready for Review
