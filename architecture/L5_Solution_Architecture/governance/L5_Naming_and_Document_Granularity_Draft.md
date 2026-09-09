# SutraERP L5 — Naming and Document Granularity Specialization

**Status:** Architecture Approved  
**Architecture Level:** L5 — Solution Architecture Governance  
**Purpose:** Approved L5 specialization defining stable L5 family, major-area and specification identifiers, document granularity, filename rules and identifier immutability.

---

# 1. Canonical Identifier Structure

Use three levels of identity:

```text
Family        L5-A
Major Area    L5-A-07
Document      L5-A-07-001
```

The family identifies the technical architecture family. The two-digit area identifies one of the approved major L5 architecture areas within that family. The three-digit document number identifies a specific architecture specification within that major area.

---

# 2. Canonical Major Area Codes

| Code | Major L5 Area |
|---|---|
| L5-A-01 | Application & Platform Architecture |
| L5-A-02 | Technology Platform & Standards Architecture |
| L5-A-03 | Module, Dependency & Technical Boundary Architecture |
| L5-A-04 | Tenant & Institutional Isolation Architecture |
| L5-A-05 | Workflow & Orchestration Architecture |
| L5-A-06 | Configuration, Feature & Institutional Extensibility Architecture |
| L5-A-07 | Secure Computational Engine Architecture |
| L5-A-08 | Background Processing & Job Execution Architecture |
| L5-B-01 | Search & Indexing Architecture |
| L5-B-02 | Database & Persistence Architecture |
| L5-B-03 | Master & Reference Data Technical Architecture |
| L5-B-04 | Document, Media & Object Storage Architecture |
| L5-B-05 | Data Classification, Retention & Privacy Architecture |
| L5-B-06 | Reporting, Analytics & Data Projection Architecture |
| L5-C-01 | API & Contract Architecture |
| L5-C-02 | Event & Messaging Architecture |
| L5-C-03 | External Integration & Connector Architecture |
| L5-C-04 | Notification & Communication Architecture |
| L5-D-01 | IAM Architecture |
| L5-D-02 | Authorization — RBAC, Scope & Policy Architecture |
| L5-D-03 | Security, Cryptography, Secrets & Key Management Architecture |
| L5-D-04 | Audit & Evidence Architecture |
| L5-E-01 | Availability & Resilience Architecture |
| L5-E-02 | Performance & Capacity Architecture |
| L5-E-03 | Backup, Restore & Disaster Recovery Architecture |
| L5-E-04 | Runtime, Packaging & Deployment Architecture |
| L5-E-05 | Observability & Operations Architecture |
| L5-F-01 | CI/CD & Software Supply Chain Security Architecture |
| L5-F-02 | Verification & Testing Architecture |
| L5-G-01 | Client Platform Architecture — Web & Future Mobile |

---

# 3. Canonical Document Identity Rule

```text
L5-<Family>-<Area>-<Document>
```

Example:

```text
L5-A-07-001  Secure Computational Engine Architecture Baseline
L5-A-07-002  Finance Computation Engine Architecture
L5-A-07-003  Examination Result Computation Engine Architecture
L5-A-07-004  Payroll Computation Engine Architecture
L5-A-07-005  General Rules & Formula Engine Architecture
L5-A-07-006  Scheduling & Optimization Engine Architecture
```

---

# 4. Document Granularity Principle

One L5 document shall describe one coherent technical architecture responsibility that can be reviewed and changed independently without duplicating another document's decisions.

A major area may contain one or many architecture documents.

Split a specification where there is a materially different:

- trust or security boundary;
- architecture decision lifecycle;
- technical responsibility;
- operational or deployment concern;
- dependency set;
- scalability characteristic;
- compliance requirement; or
- review scope that would otherwise become materially unmanageable.

Do not split merely because multiple named concepts exist. Where concerns share the same architecture decisions, ownership and lifecycle, unnecessary splitting shall be avoided.

---

# 5. Baseline Document Convention

Where a major area requires multiple child specifications, document `001` should normally represent the baseline architecture for that major area.

Example:

```text
L5-C-03-001  External Integration & Connector Architecture Baseline
L5-C-03-002  Payment Gateway Integration Architecture
L5-C-03-003  Government Education Connector Architecture
```

A major area that requires only one specification may use `001` for the complete architecture without creating an artificial baseline document.

---

# 6. Canonical Filename Convention

```text
L5-<Family>-<Area>-<Document>_<Canonical_Solution_Slug>.md
```

Examples:

```text
L5-A-07-003_Examination_Result_Computation_Engine_Architecture.md
L5-D-02-005_Privileged_Access_Architecture.md
L5-E-03-001_Backup_Restore_and_Disaster_Recovery_Architecture.md
```

The identifier is permanent. A descriptive slug may change only through controlled governance where terminology legitimately changes.

---

# 7. Identifier Stability

Once an L5 major-area identifier or document identifier enters the canonical L5 register, it shall not be silently renumbered or reused.

If an area or document is retired, its identifier remains retired. Gaps are permitted.

Before formal register approval, planning-draft identifiers may still be reorganized. Register approval is the boundary at which canonical identifier immutability begins.

---

# 8. Author Independence

Architecture identifiers shall not contain author names or team-member identities.

Author assignment belongs in workload-allocation records and the L5 document register, not in the architecture ID itself.

---

# 9. L1 Ownership Independence

L5 family letters identify technical solution-architecture families only.

```text
L5-A != L1-A
L5-B != L1-B
```

L5 identifiers shall not create, transfer or imply L1/L2/L3/L4 business ownership.

---

# 10. Traceability Principle

L5 does not use a one-L4-to-one-L5 document model.

One reusable L5 architecture specification may realize obligations from many L4 System Behavior Specifications, and one L4 behavior may depend on multiple L5 architecture specifications.

Cross-level traceability shall be explicit without creating duplicate architectural ownership.

---

# 11. Corpus Status Rule

The approved naming specialization does not itself approve or freeze the candidate L5 document corpus.

The candidate corpus must pass full L4-to-L5 coverage, duplicate-ownership and completeness validation before its document IDs become canonical and immutable.
