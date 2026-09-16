# ADR-L5-0017 — Recovery Classification, Protected Copy, Restore & Disaster-Recovery Validation Pattern

**ADR ID:** `ADR-L5-0017`  
**Title:** Recovery Classification, Protected Copy, Restore & Disaster-Recovery Validation Pattern  
**Scope:** Repository  
**Decision Owner:** L5-E-03 — Backup, Restore & Disaster Recovery Architecture  
**Assigned Author:** Pallav  
**Reviewers:** Controlled L5 Stage-5 architecture remediation review  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-16  
**Affected L5 Specifications:** `L5-E-03-001`; `L5-E-03-002`; `L5-E-03-003`; constrains recovery-aware L5 consumers  
**Related ADRs:** `ADR-L5-0002`; `ADR-L5-0005`; `ADR-L5-0010`; `ADR-L5-0013`; `ADR-L5-0014`  
**Supersedes:** None  
**Superseded By:** None

---

## 1. Decision Context

SutraERP requires one repository-level recovery architecture that distinguishes ordinary resilience/service recovery from data backup, restore, point-in-time recovery, disaster recovery and proof of recoverability. Without a shared decision, durable stores and runtime capabilities could define inconsistent backup expectations, unsafe restore practices, incompatible failover authority or unverifiable recovery claims.

`ADR-L5-0014` already governs health, degradation, containment and controlled return to service. This ADR extends the repository with the separate E03-owned recovery strategy for protected copies, restore/PITR, authority transfer and disaster-recovery validation.

## 2. Scope & Affected Architecture

This ADR governs:

- recovery classification by technical/business criticality without inventing unsupported numeric targets;
- binding of approved RPO/RTO values when authoritative values exist;
- protected recovery-copy requirements;
- backup-set/catalog integrity and recovery-point eligibility;
- isolated restoration and point-in-time recovery where supported/required;
- cross-store recovery grouping for compatible authoritative state;
- provider-neutral DR topology classes;
- authoritative-writer fencing during failover/failback;
- dependency readiness and restoration validation;
- disaster-recovery exercises and durable proof of recoverability.

It does not define business continuity policy, universal RPO/RTO values, business-record retention, database technology, backup products, cloud providers, region counts or implementation commands.

## 3. Authoritative Constraints

1. E03 owns technical recovery architecture, not canonical business recovery decisions.
2. B02 retains persistence/concurrency ownership; recovery cannot bypass B02 consistency rules.
3. E01 retains health/degradation/recovery-state semantics; E03 supplies backup/restore/DR mechanisms and evidence.
4. E04 retains runtime packaging/placement/deployment topology ownership.
5. D03 owns encryption, key custody, secrets and emergency credential handling.
6. C02 owns messaging delivery/replay recovery; E03 validates messaging dependency readiness but does not redefine C02 semantics.
7. Recovery success requires validated state integrity and dependency readiness, not merely process startup or backup-job success.
8. A protected copy shall not share every material failure/security domain with the authoritative source when the assigned recovery class requires isolation.
9. Backup retention is distinct from business-record retention.
10. Restored or failed-over state shall not introduce competing authoritative writers.
11. RPO/RTO and retention values shall be bound from authoritative profiles; E03 shall not invent repository-wide numbers.
12. The architecture shall remain provider-neutral and support on-premises-primary deployment with cloud-capable options.

## 4. Decision Criteria

The model must preserve authoritative state ownership, prove recoverability, support heterogeneous durable stores, prevent split brain, support cyber-recovery concerns, remain product-neutral, align with E01 restoration gates and avoid forcing one DR topology or numeric objective on every capability.

## 5. Alternatives Considered

### A — One universal backup schedule and DR topology
Rejected because different capabilities have different criticality, persistence characteristics and business impact, and the repository has no authoritative universal RPO/RTO values.

### B — Infrastructure snapshots as sufficient proof of recovery
Rejected because snapshot completion does not prove application-consistent restoration, dependency readiness or authoritative state integrity.

### C — Recovery-class-driven protected copies, validated restore/PITR and governed provider-neutral DR
Selected.

### D — Each store/service defines independent recovery rules
Rejected because cross-store consistency, authority transfer, dependency sequencing and recovery evidence would become inconsistent.

## 6. Decision

SutraERP shall use a recovery-class-driven architecture with protected recovery copies, explicit restore validation and governed DR authority transfer.

### 6.1 Recovery Classification

Every governed durable store or stateful capability shall be assigned either:

- an explicit recovery class; or
- an explicit approved rebuild-only designation where authoritative reconstruction is safe.

A recovery class defines the required recovery capabilities and binds approved RPO/RTO values when those values exist. E03 does not invent numeric targets.

### 6.2 Protected Copy Model

Depending on the assigned class, recoverability may require:

- full/base recoverable copies;
- incremental/change-log material;
- point-in-time recovery capability;
- independent copy isolation;
- catalogued recovery-point metadata;
- encryption and access controls;
- scheduled restoration validation.

At least one required copy for a critical class shall be isolated from the primary failure/security domain to the degree required by the approved recovery profile.

### 6.3 Restore & PITR

```text
Approved recovery request
→ choose eligible validated recovery point
→ provision isolated recovery target
→ restore base state
→ apply eligible change/PITR material
→ validate technical + application consistency
→ satisfy E01 restoration gate
→ controlled authority/traffic handoff
```

Corrupt, incomplete or unverified recovery copies are not eligible recovery points.

### 6.4 Cross-Store Recovery Groups

Stores whose authoritative relationship requires a compatible recovery point shall be grouped for coordinated restoration. Recovery shall not silently combine incompatible points that create impossible business state.

### 6.5 Disaster Recovery & Authority Transfer

DR topology shall be selected by recovery class and supported deployment model rather than by provider preference. Cold, warm, pre-provisioned or equivalent topology classes may be used where appropriate.

Where more than one site/runtime could become authoritative, failover/failback shall use fencing, quorum or equivalent authority-transfer control to prevent simultaneous writers and split brain.

### 6.6 Recovery Validation

A recovery is successful only when all applicable evidence passes:

- recovered data integrity;
- required application consistency;
- E01 health/restoration checks;
- dependency readiness;
- credential/trust validity;
- absence of unresolved recovery work that makes normal operation unsafe.

### 6.7 Cyber Recovery

After suspected compromise, recovery shall use an eligible trusted recovery point and isolated clean restoration. Credentials/keys known or suspected to be compromised shall be rotated/re-established through D03 before reconnecting affected trust boundaries.

## 7. Integration Boundaries

### E03 → B02
B02 supplies authoritative persistence topology, durability, consistency and concurrency rules. E03 defines protected copies and recovery mechanics compatible with them.

### E03 → E01
E01 defines health, degradation and controlled-return criteria. E03 supplies restored state and recoverability evidence required to satisfy those gates.

### E03 → E04
E04 owns concrete deployment/runtime placement. E03 supplies recovery topology requirements and recovery-environment needs.

### E03 → D03
D03 owns backup encryption/key custody, credential rotation and trust recovery.

### E03 → C02/C03
C02 supplies messaging recovery; C03 supplies external-provider reconciliation boundaries. E03 verifies dependency readiness and preserves ambiguous external outcomes during recovery.

## 8. Consequences

Positive: recovery claims become evidence-based, RPO/RTO can vary by approved class, split-brain risk is explicitly governed, cyber recovery is safer, and provider neutrality is preserved.

Cost: protected-copy isolation, restore testing, recovery catalogs, cross-store grouping and DR exercises require operational investment and recurring verification.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Backup succeeds but restore fails | Treat backup completion and restore validation as separate evidence. |
| Split brain during failover/failback | Mandatory fencing/quorum/equivalent authority-transfer control. |
| Recovery reintroduces compromised credentials | D03-governed credential/key rotation before reconnection. |
| Inconsistent cross-store state | Use recovery groups and compatible recovery points. |
| Overengineering low-value data | Permit governed rebuild-only designation and class-driven requirements. |
| Provider lock-in | Keep topology/provider/product selection outside this ADR unless separately governed. |

## 10. Migration / Exit Position

Different stores, runtimes and deployment models may use different recovery technologies if they preserve recovery classification, protected-copy eligibility, restore validation, authority-transfer, ownership and evidence invariants. A materially different repository recovery strategy requires a superseding ADR.

## 11. Traceability

Primary realization: `L5-E-03-001`, `L5-E-03-002`, `L5-E-03-003`.  
Relevant upstream behavioral evidence includes controlled restoration requirements such as `L4-A-057-BH-006` through `BH-008`, while exact L4→L5 validation remains governed by upstream document status and the locked traceability methodology.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** A repository-wide backup/restore/DR strategy is a mandatory ADR trigger under the locked L5 Technology Selection & ADR Methodology. This decision is distinct from, and consistent with, `ADR-L5-0014`: E01 owns health/degradation/controlled return; E03 owns protected copies, restore/PITR, DR authority transfer and recoverability proof. The decision does not select a provider, product or numeric RPO/RTO. User authorization to proceed with the Stage-5 governance closure was provided on 2026-09-16.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| E03 technical decision owner | Recommended | 2026-09-16 | Stage-5 E03 corpus remediation |
| L5 architecture reviewer | PASS | 2026-09-16 | Recovery-boundary and cross-family consistency review |
| Architecture remediation authorization | Approved | 2026-09-16 | Explicit user instruction to proceed |
