# ADR-L5-0008 — Expand-Migrate-Transition-Contract Schema Evolution Pattern

**ADR ID:** `ADR-L5-0008`  
**Title:** Expand-Migrate-Transition-Contract Schema Evolution Pattern  
**Scope:** Shared / Family  
**Decision Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Architecture remediation for Dhruv Stage-1 corpus  
**Reviewers:** SAOS architecture review; promotion explicitly authorized in controlled architecture session  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-11  
**Affected L5 Specifications:** `L5-B-02-004`  
**Related ADRs:** `ADR-L5-0005`; `ADR-L5-0006`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

SutraERP requires a reusable schema-evolution pattern that supports safe forward change, mixed-version compatibility where required, explicit migration execution, immutable migration history and safe retirement of obsolete structures. `L5-B-02-004` already defines the product-neutral Expand → Migrate → Transition → Contract pattern. Because it is reusable across owner persistence boundaries and materially affects release/recovery behavior, ADR governance is mandatory.

## 2. Scope & Affected Architecture

This ADR governs structural schema evolution and internal data migration mechanics inside B02 persistence ownership. It does not define business import/export, business compensation, backup/restore/DR, release deployment strategy, database product or migration-tool selection.

## 3. Authoritative Constraints

1. Schema evolution shall preserve canonical owner boundaries and Tenant isolation.
2. Destructive contraction shall not occur before successor structures and consumers are ready and required migration has completed.
3. Applied migration history shall be immutable and attributable.
4. Migration execution shall be separated from ordinary application runtime execution and remain governed by B02 controls.
5. Failed or partially applied change shall not be hidden by rewriting migration history.
6. Corrective evolution shall normally move forward through a new migration.
7. Recovery/restore remains E03-owned and release sequencing remains E04-owned.

## 4. Decision Criteria

Compatibility, recoverability, auditability, ownership preservation, portability, rollback/fix-forward clarity, operational safety and independence from a specific database/migration tool.

## 5. Alternatives Considered

### A — In-place destructive schema replacement
Rejected because it weakens mixed-version compatibility, rollback boundaries and partial-failure handling.

### B — Rewrite previously applied migration history when an error occurs
Rejected because it destroys trustworthy environment history.

### C — Expand → Migrate → Transition → Contract with immutable history
Selected.

## 6. Evaluation

| Criterion | A | B | C |
|---|---|---|---|
| Compatibility | Weak | Weak | Strong |
| Auditability | Acceptable | Weak | Strong |
| Failure recovery | Weak | Weak | Strong |
| Tool neutrality | Strong | Acceptable | Strong |
| Release coordination | Weak | Weak | Strong |

## 7. Decision

SutraERP shall use the **Expand → Migrate → Transition → Contract** pattern for material schema evolution.

1. **Expand:** add compatible successor structures without prematurely removing required predecessors.
2. **Migrate:** move or backfill authoritative data under the canonical owner using governed migration execution.
3. **Transition:** move runtime usage to the successor shape after compatibility and readiness gates pass.
4. **Contract:** remove obsolete structures only after no governed consumer/runtime path still requires them and recovery/rollback implications are understood.
5. Applied migration history shall be immutable; errors are corrected through new forward migrations.
6. Migration execution shall remain separated from ordinary application runtime execution.
7. Runtime-schema compatibility evidence is required where old/new runtime versions may coexist.

## 8. Consequences

Positive: safer upgrades, explicit compatibility windows, reliable migration evidence and clear fix-forward behavior.  
Cost: some releases temporarily carry duplicate or compatibility structures and require additional migration coordination and evidence.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Compatibility structures remain indefinitely | Contract phase requires governed retirement once consumers are clear. |
| Migration execution gains unnecessary scope | Keep execution limited to the governed migration operation and attributable. |
| Teams rewrite migration history | Immutable applied-history rule; corrective migration only. |
| B02 absorbs release/recovery ownership | E04 sequences release; E03 owns restore/DR; B02 owns schema/data migration mechanics. |

## 10. Migration / Exit Position

No migration framework or database product is selected. Tooling may change while retaining the four-phase pattern, immutable history and separated migration execution. Replacing the pattern requires explicit ADR supersession.

## 11. Traceability

Primary realization: `L5-B-02-004_Schema_Evolution_and_Data_Migration_Architecture.md`.  
Material collaboration: `L5-E-04-005` release sequencing and `L5-E-03-*` recovery boundaries; these are collaborators or downstream consumers, not prerequisites for B02-004 ownership.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** The pattern is reusable, product-neutral, ownership-preserving and correctly bounded from E03/E04. Controlled remediation was explicitly authorized on 2026-09-11. This approval does not imply `Locked` status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| B02 technical decision owner | Recommended | 2026-09-10 | B02-004 architecture |
| SAOS architecture reviewer | PASS | 2026-09-11 | Stage-1 production architecture review |
| Architecture promotion authorization | Approved | 2026-09-11 | Explicit remediation authorization |
