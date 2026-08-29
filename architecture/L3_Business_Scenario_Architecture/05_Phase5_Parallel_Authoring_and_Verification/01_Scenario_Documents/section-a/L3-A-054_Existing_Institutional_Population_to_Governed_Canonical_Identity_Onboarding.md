# Existing Institutional Population to Governed Canonical Identity Onboarding

# 1. Scenario Metadata

**Scenario ID:** L3-A-054  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Person & Identity  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Tenant adoption, implementation and identity-consolidation programmes

---

# 2. Scenario Purpose

Onboard an institution's existing population from governed legacy or external sources into canonical Tenant-scoped Person records while preventing duplicates, preserving legacy mappings, supporting controlled recovery and retaining complete audit evidence.

---

# 3. Business Outcome

Source records are validated, duplicate-checked against canonical Person rules, mapped to their legacy identifiers and established as canonical Persons, with a governed rollback path and complete migration audit retained

Source assessment: `PASS WITH ISSUES`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An institution adopting SutraERP must establish canonical Person records for an existing population held in legacy systems or an external identity source.

## 4.2 End Boundary

Valid source records have canonical Persons and legacy mappings, rejected records have evidenced outcomes, migration verification and reporting are complete, and recovery remains governed by migration stage.

## 4.3 In Scope

- Authorized source and Tenant validation.
- Extract, transform, schema and business validation.
- Dry run or preview.
- Canonical duplicate detection and reconciliation decisions.
- Administrative migration approval.
- Batch execution and per-record canonical Person creation.
- Legacy identifier mapping, verification, reporting and audit.
- Stage-appropriate cancellation, batch rollback or corrective recovery.

## 4.4 Out of Scope

- Routine synchronization after onboarding.
- Authentication providers, credentials or sessions.
- Role and Permission assignment.
- Source-system implementation, ETL technology, database or UI design.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Uses | L3-A-004 — Person Establishment to Active Canonical Identity | Governs each accepted Person's canonical establishment. |
| Uses | L3-A-005 — Duplicate Person Identification to Canonical Identity Reconciliation | Governs duplicate outcomes found during migration. |
| Related | L3-A-053 — Person Institutional Exit to Governed Identity Closure | Governs later identity closure, not migration rollback. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Person & Identity / Section A | Owns migration validation, canonical Person creation, duplicate rules and legacy mapping. |
| Platform & Tenant / Section A | Supplies Tenant scope, configuration and authorization context. |
| Event Infrastructure / Section A | Distributes aggregate lifecycle events produced for successful records. |
| Audit Infrastructure / Section A | Preserves migration, approval and record-level evidence. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Authorized Migration Administrator | Submits, previews and executes the approved migration. | Logical role inferred only from L2-A02's “Administrative Approval”; exact canonical role name is undefined. |
| Identity Steward | Reviews validation and duplicate outcomes. | Cannot bypass Person invariants. |
| Source Custodian | Supplies authorized legacy records and identifier meaning. | Does not create canonical Persons directly. |

L2-A02 does not formally catalogue migration actors; these labels describe responsibilities and do not create new platform roles.

---

# 7. Preconditions and Entry Criteria

- One Tenant and institution scope are resolved.
- The legacy or external source is authorized and its schema is known.
- Source identifiers can be retained as external references.
- Mandatory Person attributes and transformation mappings are defined.
- Duplicate detection and Tenant business rules are available.
- Migration mode, batch policy, approval and recovery approach are established.

---

# 8. Trigger

An institution adopting SutraERP must establish canonical Person records for an existing population held in legacy systems or an external identity source

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register the authorized source, migration scope and Tenant. | Migration Administrator; Platform & Tenant | A governed onboarding scope is established. |
| 2 | Extract and transform legacy records into the canonical intake shape. | Source Custodian; Person & Identity | Source data is prepared without creating Persons. |
| 3 | Validate schema, types, mandatory attributes, references, Tenant ownership and identity quality. | Person & Identity | Invalid records receive evidenced validation results. |
| 4 | Perform canonical duplicate detection for every record. | Person & Identity | New, duplicate and ambiguous records are distinguished. |
| 5 | Route duplicates and ambiguity to governed reconciliation or correction. | Identity Steward; Person & Identity | Duplicate Person creation is prevented. |
| 6 | Produce a dry-run preview and migration impact report. | Person & Identity | Expected creations, mappings and rejections are reviewable. |
| 7 | Obtain administrative approval for execution and recovery boundaries. | Person & Identity | Migration is authorized; the exact actor contract is an upstream issue. |
| 8 | Create the controlled processing job and execute approved records in governed batches. | Person & Identity | Valid records progress independently under batch policy. |
| 9 | Establish each accepted canonical Person using normal Person invariants and publish its successful aggregate events. | Person & Identity | Canonical Person records exist without bypassing L3-A-004 rules. |
| 10 | Retain legacy Person, employee or student identifiers as mappings to the canonical PersonID and Business Identifier. | Person & Identity | Post-migration traceability is established. |
| 11 | Verify migrated outcomes, isolate rejected records and generate the completion report. | Identity Steward; Person & Identity | Migration outcome is evidenced. |
| 12 | Preserve source, transformations, approvals, batches, record outcomes, mappings and correlation evidence. | Person & Identity; Audit Infrastructure | Complete migration audit is retained. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome |
|---|---|---|---|
| Validate-only | Institution is not ready to persist. | Run all validation and duplicate checks without creation. | A correction report is produced. |
| Partial acceptance | Some records fail but fail-fast is not required. | Process valid records and isolate invalid records. | Successful records remain governed; failures are reported. |
| Existing canonical Person | A source record matches an existing Person. | Link the validated legacy identifier or use duplicate reconciliation. | No duplicate Person is created. |
| Cancel before execution | Approval is withdrawn before persistence. | Cancel the job. | No canonical record changes occur. |
| Batch recovery | A current batch fails. | Roll back the current batch or resume failed records according to approved policy. | Completed governed batches remain traceable. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Source or Tenant is invalid. | Migration cannot preserve ownership. | Reject the job. | Platform & Tenant; Person & Identity |
| Duplicate outcome is unresolved. | Canonical uniqueness is at risk. | Exclude the record pending correction or L3-A-005. | Person & Identity |
| Mandatory data is missing. | Person establishment cannot complete. | Reject the record with stage and correction evidence. | Person & Identity |
| Batch execution fails. | Some records may be incomplete. | Isolate the batch and apply the approved stage-specific recovery path. | Person & Identity |
| Completed migration requires reversal. | Direct database rollback would violate governance. | Use approved corrective workflows rather than rewriting completed history. | Person & Identity |

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|
| One canonical Person per Tenant. | Duplicate creation is prohibited. | L2-A02 — Person Business Rules |
| Migration does not bypass Person invariants. | Each accepted record follows canonical establishment rules. | L2-A02 — Migration Principles |
| Legacy identifiers may be retained as external references. | Source-to-canonical traceability is preserved. | L2-A02 — Legacy Identifier Mapping |
| Invalid records require correction before migration. | Failed validation cannot be ignored. | L2-A02 — Migration Validation |
| Completed migrations use corrective workflows, not direct rollback. | Historical facts remain immutable. | L2-A02 — Migration Rollback |
| Every accepted state change publishes aggregate events only after success. | Consumers receive completed facts. | L2-A02 — Event Reliability Rules |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Source record | Extracted | Validate, classify and map. | Accepted, rejected or linked to an existing Person. |
| Person | Absent for a valid new record | Create under canonical lifecycle. | Canonical Person established. |
| Identity | Created where required by the establishment scenario | Follow A02 activation sequencing. | Governed canonical Identity. |
| Legacy Identifier Mapping | Absent | Associate validated source identifier with PersonID. | Traceable mapping. |
| Migration processing job | Planned | Preview, approve, execute, verify and report. | Completed or governed recovery outcome; canonical job states are undefined. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Publisher | Condition |
|---|---|---|
| `PersonCreated` | Person aggregate | A new canonical Person is successfully created. |
| `IdentityCreated` | Identity aggregate | Associated Identity creation succeeds where required. |
| `PersonVerified`, `IdentityVerified`, `PersonActivated`, `IdentityActivated` | Owning aggregate | Normal establishment transitions succeed. |

L2-A02 defines no canonical migration-job started, completed, failed or rolled-back events; none are invented.

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|
| Authorized source extract | Source Custodian | Person & Identity | Legacy records and identifiers. |
| Tenant context | Platform & Tenant | Person & Identity | Isolation and configuration. |
| Duplicate resolution | Person & Identity | Migration processing | New, match or unresolved outcome. |
| Canonical Person reference | Person & Identity | Institutional consumers | PersonID and mapped legacy references. |
| Audit input | Person & Identity | Audit Infrastructure | Job, batch and record evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Source access, preview, execution and recovery require Tenant-scoped authorization.
- Migration does not grant authentication or authorization.

## 15.2 Approval and Separation of Duty

- Administrative approval precedes execution.
- L2-A02 does not name the approver or define migration-specific separation of duty.

## 15.3 Audit Obligations

Retain source, Tenant, schema, transformations, preview, approval, batch, record, duplicate, mapping, rejection, recovery, actor, timestamp and correlation evidence.

---

# 16. Completion Criteria and Postconditions

- Source and Tenant were authorized.
- Every record completed validation and duplicate detection.
- Only approved valid records created canonical Persons.
- Legacy identifiers map to canonical identifiers.
- Rejected records have actionable evidence.
- Verification and completion reporting are complete.
- Stage-appropriate recovery remains governed.
- Migration audit is retained.

Postconditions: canonical identifiers remain immutable; source systems do not own Person state; authentication and role assignment remain separate.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|
| A054-ISSUE-01 | Non-blocking | L2-A02 defines migration and bulk-processing workflows but no canonical Migration Job aggregate, lifecycle states, commands or job-level events. | Use the source-defined workflow and per-Person events; define authoritative job contracts before L4. |
| A054-ISSUE-02 | Non-blocking | Rollback options are stage-dependent, while completed migrations require unspecified approved corrective workflows. | Do not invent database rollback; require an approved corrective path and define it before locking operational recovery. |
| A054-ISSUE-03 | Non-blocking | Migration actors and administrative approver are not formally defined. | Treat actor labels as logical responsibilities and establish canonical roles upstream. |

No unsupported job state, event, actor, rollback mechanism or implementation detail is assumed.
