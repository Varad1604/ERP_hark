# Backup, Restore & Point-in-Time Recovery Architecture

## 1. Specification Metadata
**Specification ID:** `L5-E-03-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-E — Reliability, Deployment & Operations Architecture`  
**Major Area:** `L5-E-03 — Backup, Restore & Disaster Recovery Architecture`  
**Canonical Technical Responsibility:** Backup sets, restoration and point-in-time recovery across governed data stores.  
**Technical Architecture Owner:** `L5-E-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-E-03-001`; `L5-B-02`; `L5-D-03`; `L5-E-04`; `L5-E-05`  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0005`; `ADR-L5-0014`; repository recovery-strategy ADR required but not yet canonically allocated

## 2. Purpose, Scope & Non-Goals
Defines how governed SutraERP data stores produce recoverable backup sets, execute restores and support point-in-time recovery where the selected store and recovery class require it. It does not select a database/storage vendor or invent business recovery semantics.

## 3. Upstream Applicability & Constraints
E03-001 supplies recovery classification. B02 remains owner of store/persistence topology. D03 governs encryption/key custody. Exact recovery-sensitive L4 BH traceability remains `OD-001`.

## 4. Technical Ownership & Dependencies
**Owns:** backup-set composition, recoverable base-copy/change-log relationship, restore orchestration boundary, PITR target selection mechanics, restore validation and recovery-copy lifecycle.  
**Must not own:** database product internals, business retention, source-domain acceptance criteria or availability SLA definition.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility |
|---|---|
| Backup Coordinator | Schedules/initiates backups according to recovery class and approved objectives. |
| Base Recovery Copy | Provides a recoverable full/base state appropriate to the store technology. |
| Change Capture / Recovery Log | Preserves sufficiently granular change history for PITR where supported/required. |
| Recovery Catalog | Tracks backup identity, scope, integrity, encryption/key reference, recovery window and validation evidence. |
| Restore Orchestrator | Restores an isolated target from a selected recovery set/point. |
| Integrity Validator | Verifies readability, structural integrity and required cross-store/application consistency before release. |

### 5.2 Backup Flow
```text
Recovery class + store capability
→ create protected base copy
→ preserve required change-log/PITR material
→ verify integrity/catalog evidence
→ replicate/isolate per E03-001
→ retention lifecycle
```

### 5.3 Restore / PITR Flow
```text
Approved recovery request
→ choose validated recovery point
→ provision isolated restore target
→ restore base copy
→ replay/apply change material to target point where applicable
→ validate store + application consistency
→ authorize handoff to E03-003/service recovery process
```

### 5.4 Failure Rules
A corrupt/incomplete backup is excluded from eligible recovery points and recorded as a recoverability defect. Restore occurs into an isolated target unless an approved recovery procedure explicitly provides another safe boundary. PITR shall not create a concurrent competing authoritative writer.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | PITR pattern | Base recoverable copy plus sufficiently granular change capture where required/supported. |
| AD-002 | Validation | Restore validation includes technical integrity and required application consistency. |
| AD-003 | Isolation | Restore is validated in an isolated recovery target before authoritative service return. |
| AD-004 | Derived data | Rebuild caches/indexes from authoritative stores when safe instead of mandating backup. |
| AD-005 | Retention | Technical backup retention is policy-driven and distinct from business-record retention. |

## 7. Security, Privacy & Operations
Backups are encrypted in transit/at rest according to D03. Key custody remains separated from backup storage sufficiently to preserve compromise isolation and recoverability. Backup deletion/expiry and restore initiation are privileged, auditable operations.

## 8. Architecture Conformance Requirements
- `CR-001`: Backup sets shall be catalogued with sufficient evidence to identify scope, integrity and eligible recovery point.
- `CR-002`: PITR shall only be claimed where the store/recovery design actually preserves required change history.
- `CR-003`: Restore validation shall occur before the recovered state is returned to authoritative service.
- `CR-004`: Corrupt or unverified copies shall not be treated as proven recovery points.
- `CR-005`: Recovery shall not introduce concurrent authoritative writers or silently merge divergent states.
- `CR-006`: Backup retention/deletion shall be governed and attributable.

## 9. Open Decisions & Deferred Design
| ID | Item | Blocking? | Resolution |
|---|---|---|---|
| OD-001 | Exact L4 BH mappings for recovery-sensitive data stores. | Yes for Architecture Approved. | Complete traceability extraction. |
| OD-002 | Store-specific backup/PITR commands and products. | No. | Define after governed persistence/storage selections. |
| OD-003 | Numeric backup frequency, recovery windows and retention. | No. | Bind to approved recovery profiles only. |

## 10. Traceability Summary
**Corpus:** `L5-E-03-002`. **Peers:** E03-001, B02, D03, E04, E05. **Decisions:** AD-001–005. **Conformance:** CR-001–006.
