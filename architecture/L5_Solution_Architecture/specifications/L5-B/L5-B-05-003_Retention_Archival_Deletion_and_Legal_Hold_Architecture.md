# L5-B-05-003 — Retention, Archival, Deletion & Legal Hold Architecture

## 1. Specification Metadata

**Specification ID:** L5-B-05-003  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Technical enforcement of retention, archival, governed deletion and legal-hold controls without redefining business policy  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.1  
**Status:** Locked  
**Last Updated:** 2026-09-12  
**Depends On:** L5-B-05-001; L5-B-02; L5-D-03; L5-D-04; applicable canonical L4 behavior  
**Referenced By:** L5-B-04, L5-A-08, L5-E-03, L5-A-05  
**Applicable ADRs:** ADR-L5-0001; ADR-L5-0002; ADR-L5-0004; ADR-L5-0005; ADR-L5-0006  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the technical control contract for enforcing authoritative retention, archival, deletion and legal-hold decisions. It determines whether a requested technical disposition may proceed and coordinates with the canonical owners of persistence, storage, cryptography, audit and future job execution.

It does not create a universal record-deletion lifecycle, retention period, quarantine state, hold status vocabulary, database schema or scheduler design.

### 2.2 In Scope

- Resolution of authoritative retention/disposition directives.
- Technical hold-precedence checks using authoritative hold decisions/references.
- Validation that disposition is allowed before delegating execution.
- Archival handoff to the canonical storage owner.
- Persistence deletion/anonymization handoff to L5-B-02.
- Cryptographic-erasure handoff to L5-D-03 only when the authoritative disposition and approved security architecture permit it.
- Audit evidence of evaluation and execution outcomes via L5-D-04.
- A future execution contract consumable by L5-A-08 without pre-designing A08 scheduling or worker topology.

### 2.3 Explicit Non-Goals

This specification does not own:

- Retention schedules, statutory durations, legal-hold declaration/release authority or business disposal approval.
- Canonical domain lifecycle states such as soft-deleted, quarantined or purge-pending.
- Database tables, DDL, SQL queries, indexes, transactions or concurrency mechanisms.
- Background scheduler, worker, retry or batch orchestration architecture.
- Object-store topology or archive infrastructure.
- Key-destruction mechanisms or cryptographic algorithms.
- Backup/restore and disaster-recovery policy.

---

## 3. Architectural Constraints & Ownership

### 3.1 Authoritative Disposition Input

Every disposition attempt must reference authoritative source decisions. L5-B-05-003 may evaluate and enforce those decisions but shall not derive a business retention period or deletion eligibility from locally invented rules.

### 3.2 Legal-Hold Precedence

Where authoritative upstream policy marks a record as subject to an active legal hold, technical disposal shall be blocked. The hold's lifecycle, authority, vocabulary and case semantics remain upstream-owned.

### 3.3 Canonical Peer Ownership

- **L5-B-02:** owns physical persistence model, schema, SQL, transaction and concurrency realization.
- **L5-B-04:** owns managed document/object storage realization used for archival content.
- **L5-D-03:** owns cryptographic erasure capability and key-management behavior.
- **L5-D-04:** owns audit/evidence recording.
- **L5-A-08:** when authored and dependency-ready, owns scheduling/background execution. This specification exposes an execution contract for A08 to consume; it does not depend on A08 to define its own architecture.
- **L5-C-02:** owns messaging delivery/retry semantics if disposition commands or completion notifications use events.

---

## 4. Technical Architecture

### 4.1 Policy-Neutral Disposition Evaluation

```text
Authoritative disposition request / governance trigger
                     |
                     v
Resolve retention + hold decision references
                     |
                     v
Validate Tenant / Institution / target record scope
                     |
              +------+------+
              |             |
        hold blocks      disposition permitted
              |             |
              v             v
          deny action    delegate execution
                            |
          +-----------------+------------------+
          |                 |                  |
       L5-B-02           L5-B-04            L5-D-03
   persistence action   archive action   approved crypto action
          |                 |                  |
          +-----------------+------------------+
                            v
                    evidence to L5-D-04
```

### 4.2 Disposition Control Contract

The contract carries references, not a new business lifecycle:

```text
DispositionControlContext {
    tenant_context_ref
    institution_context_ref
    target_record_ref
    authoritative_retention_decision_ref
    authoritative_hold_decision_refs[]
    authorized_disposition_ref
    requested_execution_ref
}
```

`authorized_disposition_ref` points to the action authorized by the source workflow/policy. This specification does not define a repository-wide enumeration of retention outcomes.

### 4.3 Execution Handoff

After validation:

- persistence mutation/deletion/anonymization is delegated to L5-B-02 under B02-owned transactional rules;
- archive movement/storage is delegated to L5-B-04;
- cryptographic erasure is requested from L5-D-03 only when approved security architecture supports it;
- evidence of the decision and returned execution result is sent to L5-D-04.

No SQL statement, table shape, lock table, universal tombstone field or deletion state is defined here.

### 4.4 Future Background Execution Boundary

L5-B-05-003 defines a scheduler-neutral `DispositionEvaluationRequest` / result contract. When L5-A-08 becomes dependency-ready, A08 may schedule or orchestrate evaluations using that contract. A08 remains sole owner of scheduling cadence, batching, worker topology, retry and execution timing.

---

## 5. Dependencies & Integration

- **L5-B-05-001:** authoritative control references and scope validation.
- **L5-B-02:** persistence execution and transaction/concurrency boundaries.
- **L5-B-04:** archive/object storage realization where applicable.
- **L5-D-03:** cryptographic mechanisms where approved.
- **L5-D-04:** audit/evidence recording.
- **L5-A-08:** downstream consumer of the scheduler-neutral contract after its own architecture is established; not an upstream dependency of this specification.

---

## 6. Technical Behavior & Failure Concerns

- **TB-001 — Hold Block:** authoritative active hold => disposition fails closed.
- **TB-002 — Missing Retention Decision:** no deletion/archive action is inferred.
- **TB-003 — Conflicting Decisions:** fail closed and require authoritative resolution; do not choose locally.
- **TB-004 — Execution Failure:** retain the authoritative request/result context and surface failure; retry mechanics remain with the canonical executor/transport owner.
- **TB-005 — Audit Continuity:** disposition of operational data shall not silently remove audit evidence whose retention is independently governed.
- **TB-006 — Tenant Isolation:** evaluation never crosses validated Tenant/Institution scope.

---

## 7. Security, Privacy & Trust Boundaries

- Authorization to initiate or approve a disposition is consumed from its authoritative owner/D02; this specification does not create administrative roles.
- D03 controls all cryptographic deletion mechanisms.
- B02 controls all direct persistence mutation.
- No protected data values are copied into disposition audit evidence unnecessarily.

---

## 8. Conformance Requirements

- **CR-001:** Every technical disposition shall be traceable to an authoritative retention/disposition decision.
- **CR-002:** An authoritative active legal hold shall block technical disposal.
- **CR-003:** L5-B-05-003 shall not define universal deletion lifecycle states, quarantine periods or soft-delete semantics.
- **CR-004:** L5-B-05-003 shall not define database schemas, SQL, DDL or transaction mechanics.
- **CR-005:** Persistence execution shall be delegated to L5-B-02.
- **CR-006:** Background scheduling/batch/worker behavior shall remain owned by L5-A-08 and shall not be pre-designed here.
- **CR-007:** Cryptographic erasure shall be delegated to L5-D-03 under approved security architecture.
- **CR-008:** Archival storage realization shall be delegated to L5-B-04.
- **CR-009:** Evaluation and execution outcomes shall be recorded through L5-D-04.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Material Technical Contribution | Realization Status |
|---|---|---|---|
| L4-A-025 | L4-A-025-BH-001 | Resolves authoritative scope/classification/retention references before disposition evaluation. | Verified Realization |
| L4-A-025 | L4-A-025-BH-002 | Enforces authoritative legal-hold blocking without defining hold lifecycle semantics. | Verified Realization |
| L4-A-025 | L4-A-025-BH-003 | Records the authoritative retention-governance decision reference and technical outcome without inventing new status values. | Verified Realization |
| L4-A-026 | L4-A-026-BH-006 | Delegates non-destructive archival realization while preserving identity and provenance. | Verified Realization |
| L4-A-053 | L4-A-053-BH-004 | Evaluates authoritative retention/dependency prerequisites for Person/Identity archival eligibility. | Verified Realization |
| L4-A-053 | L4-A-053-BH-007 | Enforces technical prerequisites for authorized exceptional permanent deletion without creating the approval rule. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Execution Scheduling:** Deferred to L5-A-08; no cadence, chunk size or worker topology is specified here.
- **OD-002 — Persistence Schema:** Delegated to L5-B-02; no `RecordHoldLock` table or SQL contract is owned here.
- **OD-003 — Disposition State Model:** No universal L5 state model is established. Any lifecycle state must come from the authoritative source domain/workflow.
- **OD-004 — Cryptographic Erasure:** Available only where L5-D-03 establishes an approved mechanism and the authoritative disposition permits it.

**Current State:** Locked
