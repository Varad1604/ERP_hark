# ADR-L5-0046 — Recovery Class, RPO/RTO, Retention & Validation Governance Standard

**ADR ID:** `ADR-L5-0046`  
**Title:** Recovery Class, RPO/RTO, Retention & Validation Governance Standard  
**Scope:** Repository  
**Decision Owner:** L5-E-03 — Backup, Restore & Disaster Recovery Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-E-03-001`; `L5-E-03-002`; `L5-E-03-003`; constrains B02/B04/B06/B01/E04/F02 recovery-sensitive implementation  
**Related ADRs:** `ADR-L5-0014`; `ADR-L5-0017`; `ADR-L5-0044`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

E03 already requires recovery classification, protected copies, restore validation and provider neutrality. Backend readiness requires a canonical classification/governance rule for RPO/RTO and retention without fabricating one numeric target for all SutraERP data.

## 2. Decision

1. **Recovery classes:** recoverable assets are assigned a governed recovery class. The default conceptual classes are:
   - **Critical** — highest business/dependency impact;
   - **Important** — significant operational impact;
   - **Standard** — recoverable within normal operational tolerance;
   - **Reconstructible** — derived state that may be rebuilt from authoritative sources where reconstruction meets the approved objective.
   A later profile may refine names/subclasses without changing this classification principle.
2. **Class assignment evidence:** recovery class is determined from authoritative business impact, legal/regulatory obligations, dependency criticality, data-loss consequence and whether the asset can be safely reconstructed.
3. **No universal RPO/RTO:** numeric RPO/RTO targets bind from approved class/profile requirements and authoritative source obligations. E03 shall not invent a repository-wide number.
4. **Retention authority:** business-record retention/legal hold/privacy requirements remain source-owner policy. Backup-copy retention is an E03 recovery policy and shall not silently extend, shorten or override canonical record-retention authority.
5. **Derived/rebuildable data:** OpenSearch indexes, B06 projections/read models, Valkey cache and other non-authoritative state should be reconstructed from authoritative sources where the assigned recovery objective can be met; backup is used only where recovery economics/objectives justify it.
6. **PostgreSQL recovery:** PostgreSQL recovery profiles support governed base/full backup plus WAL/PITR capability where required by the assigned recovery class. Exact backup frequency, WAL retention and restore point granularity remain profile values.
7. **Object/platform recovery:** object stores and other stateful supporting platforms receive recovery treatment appropriate to their authoritative/reconstructible role and assigned class.
8. **Recovery proof:** backup-job success does not prove recoverability. Approved restore plus integrity/application/dependency/security validation is required.
9. **DR exercises:** disaster-recovery/failover exercises are mandatory on a governed cadence appropriate to the recovery class. This ADR intentionally does not invent universal cadence values.
10. **Fencing/authority:** restored or failed-over copies shall not create competing authoritative writers; canonical ownership and Tenant/Institution isolation survive recovery.
11. **Recovery acceptance:** normal service is declared restored only after required E03/E01 validation evidence passes.

## 3. Conformance Rules

- Recovery class shall be traceable to its authoritative rationale.
- Business/legal retention obligations cannot be inferred from backup technology defaults.
- Reconstructible classification requires a verified reconstruction source/path.
- A failed restore validation blocks claims of recoverability regardless of backup-job success.
- Recovery testing shall include dependency/security/context validation where material, not data-file presence alone.
- Deployment-specific managed backup products do not become canonical recovery owners.

## 4. Consequences

SutraERP can implement backup/DR immediately using class/profile governance while postponing numeric RPO/RTO/retention values until authoritative evidence exists.

## 5. Approval Record

**Decision selected:** Critical/Important/Standard/Reconstructible recovery classification; evidence-based class assignment; no universal RPO/RTO/retention values; PITR where class requires it; derived data reconstruction preferred where objectives allow; restore validation and class-based DR exercises mandatory.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
