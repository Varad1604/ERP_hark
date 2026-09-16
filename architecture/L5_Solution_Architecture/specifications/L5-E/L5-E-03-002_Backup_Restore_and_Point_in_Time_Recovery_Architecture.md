# Backup, Restore & Point-in-Time Recovery Architecture

## 1. Specification Metadata
**Specification ID:** `L5-E-03-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-E — Reliability, Deployment & Operations Architecture`  
**Major Area:** `L5-E-03 — Backup, Restore & Disaster Recovery Architecture`  
**Canonical Technical Responsibility:** Backup sets, restoration and point-in-time recovery across governed data stores.  
**Technical Architecture Owner:** `L5-E-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.3`  
**Status:** `Locked`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-E-03-001`; `L5-B-02`; `L5-D-03`; `L5-E-01`; `L5-E-05`; applicable data-store owners  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0005`; `ADR-L5-0014`; `ADR-L5-0017`  
**Governance Authorization:** `L5_Pallav_Stage5_Architecture_Approval_Decision_2026-09-16.md`  
**Governance Lock:** `L5_Pallav_Stage5_Final_Lock_Decision_2026-09-16.md`

## 2. Purpose, Scope & Non-Goals
Defines backup-set construction, protected recovery copies, restore execution and point-in-time recovery mechanics across governed data stores. It does not choose a database/storage product, invent RPO/RTO/retention values or redefine business-record retention.

## 3. Upstream Applicability & Constraints
| Source | Status | Exact Reference | E03-002 Use |
|---|---|---|---|
| `L5-E-03-001` | Locked | recovery class/profile and recovery-set model | Supplies class/objective boundary consumed here. |
| `L5-B-02-*` | Draft/current | persistence ownership/topology/transaction capabilities | Supplies store ownership and consistency characteristics; E03 does not redesign persistence. |
| `ADR-L5-0014` | Architecture Approved | validation + controlled return | Restoration does not imply service readiness. |
| `ADR-L5-0017` | Architecture Approved | recovery classification/protected-copy/restore/DR validation | Governs protected-copy, restore/PITR and recovery-validation pattern. |
| `L4-A-057` | Draft | `BH-005`–`BH-009` | Supporting requirement to preserve data/config/history and validate integrity before restored availability. |

`L4-A-057` remains Draft, but its cited behavior is explicitly governance-authorized for this Stage-5 approval. The relation remains `Specified`, not `Validated`, until normal traceability source-status conditions are met. The B02 boundary is declared stable enough for scoped E03 consumption by the Stage-5 approval decision.

### Authoritative Constraints
- Backup method/frequency must satisfy an approved recovery profile; E03 shall not invent numeric objectives.
- Point-in-time recovery is used only where supported by the selected persistence technology and required by the recovery profile.
- Recovery copies/keys shall be protected against the primary failure/security domain according to the approved class.
- Restore shall target a known recovery point and preserve multi-store compatibility where a recovery set spans dependent stores.
- Rebuildable derived stores may be reconstructed from canonical sources when explicitly classified as such.

## 4. Technical Ownership & Dependencies
**Owns:** backup-set coordination, recovery-point catalog, restore/PITR orchestration, protected-copy verification, multi-store recovery-set ordering/compatibility checks and restore-validation evidence.  
**Consumes:** B02 store-specific backup/log/snapshot capabilities, D03 encryption/key controls, E01 health/controlled return, E05 telemetry.  
**Must not own:** business data retention, database technology, canonical data ownership, application business reconciliation or DR failover topology (`L5-E-03-003`).

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility | Boundary |
|---|---|---|
| Recovery-Point Catalog | Records available backup/base/change-log recovery points and protected-copy status. | Metadata only; not canonical business data. |
| Backup Set Coordinator | Ensures class-appropriate backup components form a recoverable set. | Uses B02/storage-native mechanisms; does not prescribe product. |
| Protected Copy Verifier | Confirms required isolation/immutability/protection properties for the profile. | Does not select vendor/WORM technology. |
| Restore Planner | Resolves target recovery point, dependent stores, ordering and required credentials/keys. | No business-state mutation beyond restoration. |
| Restore Executor | Invokes selected technology's restore/PITR mechanism. | Store-specific mechanics remain implementation/technology-specific. |
| Post-Restore Validator | Verifies integrity, compatibility and required application/dependency readiness before E01 controlled return. | Business truth remains with canonical owners. |

### 5.2 Backup / PITR Model
For transactional stores requiring fine-grained recovery, the architecture supports a recoverable base plus continuous or sufficiently granular change history where the selected store provides such capability. This is a capability requirement, not a mandate for a specific WAL/log technology. Snapshot-only protection is acceptable only where the recovery profile and consistency requirements support it.

### 5.3 Restore Flow
```text
Authorized restore request + approved recovery profile
→ select trusted recovery point / recovery set
→ resolve D03 keys/credentials
→ isolate target recovery context
→ restore base + applicable change history / selected store mechanism
→ reconcile dependent-store compatible point
→ integrity + application/dependency validation
→ E01 controlled return
→ preserve recovery evidence
```

### 5.4 Material Failure Rules
Unreadable/corrupt/untrusted backup components are not promoted. Missing key material is a recovery blocker rather than justification for bypassing encryption. A restored store that is technically readable but inconsistent with its recovery set is not ready for return. Backup deletion/expiry follows governed retention/privilege controls and evidence requirements.

## 6. Architecture Decisions
| ID | Decision | Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | PITR posture | Support PITR/change-history recovery only where profile requires it and selected store supports it. | E03 scope + product neutrality. | `ADR-L5-0017` |
| AD-002 | Protected copies | Class-appropriate recovery copies may require isolation/immutability/geographic separation, expressed as capabilities rather than vendor products. | Recovery-risk derivation + D03/E04 boundaries. | `ADR-L5-0017` |
| AD-003 | Restore proof | Restore success requires integrity/compatibility/application readiness, not mere file/database readability. | A-057 BH006–009 + ADR0014. | `ADR-L5-0017` |
| AD-004 | Derived data | Safely reconstructable derived stores may be rebuilt from canonical sources instead of backed up, when classified in the recovery profile. | Ownership + recovery efficiency. | `ADR-L5-0017` |

## 7. Security, Privacy & Operations
Backup data and transfer consume D03 protection. Key custody remains separated from backup media according to D03. Privileged restore/delete operations require authorization/evidence. Telemetry includes backup age/status, copy-verification result, restore duration stages, validation disposition and failures; no unsupported numeric thresholds are fixed.

## 8. Architecture Conformance Requirements
- `CR-001`: Every backup/restore method shall trace to an approved recovery profile and B02-supported store capability.
- `CR-002`: E03 shall not invent RPO/RTO, retention, backup frequency or PITR granularity values.
- `CR-003`: Protected recovery copies shall satisfy class-required isolation/protection without assuming a specific vendor.
- `CR-004`: Multi-store recovery shall restore to a compatible recovery-set point before return to service.
- `CR-005`: Missing/corrupt/untrusted recovery material shall block promotion rather than bypass integrity/security.
- `CR-006`: Post-restore validation shall precede E01 controlled return.
- `CR-007`: Business-record retention remains owned outside E03.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| OD-001 | Named database/storage backup technology and numeric profile values. | Technology / Operations | No. | Decide only through governed technology selection and approved requirements. |
| OD-002 | Promotion of supporting `L4-A-057` relation from `Specified` to `Validated`. | Traceability | No for this Architecture Approved specification. | Revalidate after applicable L4 approval. |

## 10. Traceability Summary
**Primary:** `L5-E-03-001`, B02, `ADR-L5-0014`, `ADR-L5-0017`, Stage-5 approval decision. **Governance-authorized / Specified supporting L4:** `L4-A-057-BH-005`–`009`. **Peers:** D03, E01, E05. **Blocking:** none.
