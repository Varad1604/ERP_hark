# SutraERP L5 — Workload Complexity and Five-Person Allocation Decision

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Decision Date:** 2026-09-09  
**Depends On:** L5 Authoring Population Freeze Decision 105; L5 Candidate Document Corpus Register 105 v1.1.0; L5 Governance and Architecture Standard v0.3.1; L5 Validation Standard v1.0.1  

---

## 1. Decision

The frozen 105-specification L5 authoring population is allocated across five authors using the following locked workload order:

```text
Varad
  ↓
Pallav
  ↓
Dhruv
  ↓
Devang
  ↓
Chetan
```

The allocation intentionally does not equalize file counts.

It balances:

- total volume;
- technical difficulty;
- architectural coherence;
- author capability focus;
- cross-specification dependency concentration;
- Dhruv's parallel L6 UI/UX responsibility.

---

## 2. Locked Workload Summary

| Author | Assigned L5 Specifications | Share of 105 | Overall Difficulty | Complexity Level | Workload Position |
|---|---:|---:|---|---:|---|
| Varad | 31 | 29.5% | Very High | 5/5 | Highest |
| Pallav | 30 | 28.6% | Very High | 5/5 | Second |
| Dhruv | 20 | 19.0% | Very High | 5/5 | Third — deliberately capped because L6 begins in parallel |
| Devang | 14 | 13.3% | High logical complexity | 4/5 | Fourth |
| Chetan | 10 | 9.5% | Moderate relative to the L5 corpus | 3/5 | Lowest |
| **Total** | **105** | **100%** |  |  |  |

File count is not treated as a complete measure of effort.

```text
Number of specifications
        ≠
Technical difficulty
```

---

## 3. Varad — 31 Specifications

### Assigned Architecture

- `L5-A-07-001` to `L5-A-07-006` — Secure Computational Engine family
- `L5-A-08-001` to `L5-A-08-002` — Background Processing & Job Execution
- `L5-B-01-001` to `L5-B-01-002` — Enterprise Search & Indexing
- `L5-B-04-001` to `L5-B-04-005` — Document, Media & Object Platform
- `L5-B-05-001` to `L5-B-05-005` — Data Classification, Retention & Privacy
- `L5-D-03-001` to `L5-D-03-005` — Security, Cryptography & Trust
- `L5-D-04-001` to `L5-D-04-002` — Audit & Evidence baseline/provenance
- `L5-E-01-001` to `L5-E-01-002` — Availability & Resilience
- `L5-E-02-001` to `L5-E-02-002` — Performance & Capacity

### Work Character

Varad owns the largest workload and a very high technical-difficulty concentration spanning secure computation, privacy, cryptography, audit, resilience and performance.

**Difficulty:** Very High  
**Complexity:** 5/5

---

## 4. Pallav — 30 Specifications

### Assigned Architecture

- `L5-C-01-001` to `L5-C-01-003` — API & Contract Architecture
- `L5-C-02-001` to `L5-C-02-003` — Event & Messaging Architecture
- `L5-C-03-001` to `L5-C-03-009` — External Integration & Connector Architecture
- `L5-E-03-001` to `L5-E-03-003` — Backup, Restore & Disaster Recovery
- `L5-E-04-001` to `L5-E-04-005` — Runtime, Packaging & Deployment
- `L5-E-05-001` to `L5-E-05-004` — Observability & Operations
- `L5-F-01-001` to `L5-F-01-003` — CI/CD & Software Supply Chain Security

### Work Character

Pallav owns the principal API, integration, runtime, deployment and DevOps-oriented architecture cluster.

```text
API / Contracts
        ↓
Events / Messaging
        ↓
External Integrations
        ↓
Runtime / Deployment
        ↓
Backup / DR
        ↓
Observability / Operations
        ↓
CI/CD / Supply Chain
```

**Difficulty:** Very High  
**Complexity:** 5/5

---

## 5. Dhruv — 20 Specifications

### Assigned Architecture

- `L5-A-01-001` — Application & Platform Architecture
- `L5-A-02-001` — Technology Platform & Standards Architecture
- `L5-A-03-001` to `L5-A-03-002` — Module / Dependency Architecture
- `L5-A-04-001` to `L5-A-04-002` — Tenant & Institutional Isolation
- `L5-A-06-001` to `L5-A-06-003` — Configuration & Institutional Extensibility
- `L5-B-02-001` to `L5-B-02-004` — Database & Persistence
- `L5-B-03-001` to `L5-B-03-002` — Master & Reference Data Technical Architecture
- `L5-D-04-003` — Tamper-Evident Audit Storage, Access & Export Architecture
- `L5-G-01-001` to `L5-G-01-004` — Client Platform Architecture

### Work Character

Dhruv owns the core platform, database/persistence, Tenant/Institution isolation and client-platform bridge architecture.

The assignment deliberately includes technically hard work while capping overall L5 volume because Dhruv will begin L6 UI/UX in parallel.

The `L5-D-04-003` assignment is an intentional cross-area exception because its integrity-protected storage/access/export responsibility is strongly coupled to Dhruv's database/persistence architecture.

**Difficulty:** Very High  
**Complexity:** 5/5

---

## 6. Devang — 14 Specifications

### Assigned Architecture

- `L5-A-05-001` to `L5-A-05-003` — Workflow & Orchestration
- `L5-D-01-001` to `L5-D-01-004` — IAM
- `L5-D-02-001` to `L5-D-02-007` — Authorization / RBAC / Permission / Scope / Delegation / Privileged Access / SoD

### Work Character

Devang receives lower total volume but important logical and security-sensitive architecture.

All IAM and authorization/RBAC architecture is intentionally concentrated with Devang to avoid fragmented policy/security ownership.

**Difficulty:** High logical complexity  
**Complexity:** 4/5

---

## 7. Chetan — 10 Specifications

### Assigned Architecture

- `L5-B-06-001` to `L5-B-06-003` — Reporting, Analytics & Data Projection
- `L5-C-04-001` to `L5-C-04-003` — Notification & Communication
- `L5-F-02-001` to `L5-F-02-004` — Verification & Testing

### Work Character

Chetan receives the lowest total workload and the comparatively least technically intensive assignment available within the frozen L5 corpus, while still retaining meaningful L5 architecture responsibilities.

**Difficulty:** Moderate relative to the L5 corpus  
**Complexity:** 3/5

---

## 8. Coverage Reconciliation

The allocation covers the entire frozen 105-specification population exactly once.

```text
Varad   31
Pallav  30
Dhruv   20
Devang  14
Chetan  10
-------------
Total  105
```

No specification is intentionally unassigned.

No specification is intentionally assigned to more than one author.

The only deliberate split of a major-area family is `L5-D-04`:

- `L5-D-04-001` and `L5-D-04-002` → Varad
- `L5-D-04-003` → Dhruv

This is an authoring assignment split only. It does not change canonical technical ownership or family placement.

---

## 9. Authoring Governance

This allocation determines assigned authorship only.

It does not transfer canonical technical ownership.

Each author shall:

- use the locked L5 governance and template;
- preserve upstream L1-L4 authority;
- preserve the frozen 105 specification identities and responsibility boundaries;
- identify and respect cross-L5 dependencies;
- use ADR governance for material decisions;
- not independently allocate ADR numbers;
- not self-approve or self-lock authored specifications.

---

## 10. Broad-Authoring Boundary

Locking this workload allocation completes the workload/author-allocation prerequisite.

It does **not by itself** authorize broad L5 authoring.

A separate explicit broad-authoring authorization remains required.

---

**Status:** Locked  
**105-Specification Allocation:** Complete  
**Workload Order:** Varad > Pallav > Dhruv > Devang > Chetan  
**Dhruv Parallel L6 Responsibility:** Accounted For  
**Broad L5 Authoring:** Not Authorized by this decision
