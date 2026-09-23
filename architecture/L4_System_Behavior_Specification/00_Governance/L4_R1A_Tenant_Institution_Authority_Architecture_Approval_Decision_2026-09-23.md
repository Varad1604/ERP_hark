# SutraERP L4 R1A — Tenant / Institution Authority — Architecture Approval Decision — 2026-09-23

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Architecture Level:** L4 — System Behavior Specification Governance  
**Decision Type:** Post-L5-validation upstream blocker remediation / controlled L4 parent-set approval  
**Scope:** `L4-A-001` through `L4-A-003`; `L4-B-001` through `L4-B-003`  
**Downstream Impact:** L5 A01/A04 and dependent persistence/API/deployment blocker remediation; no automatic L5 promotion  

---

## 1. Decision

The following six canonical L4 System Behavior Specifications have completed scoped validation against the Locked L4 governance and validation standards and are approved to advance to `Architecture Approved`:

- `L4-A-001` — Tenant Provisioning to Active Tenant Availability;
- `L4-A-002` — Active Tenant Governance Concern to Controlled Restriction and Restored Availability;
- `L4-A-003` — Active Tenant to Governed Archival;
- `L4-B-001` — Institution Establishment to Final Activation Decision;
- `L4-B-002` — Institution Suspension and Operational Restoration;
- `L4-B-003` — Institution Archival.

**Verdict:** `PASS / Architecture Approved` for the scoped R1A parent set.

This decision does not Lock these specifications and does not authorize Final L4 or L5 corpus Lock.

---

## 2. Authority and Validation Basis

Validation used the current canonical `main` baseline at the start of R1A:

`78b26bc0316d60be07e92508b423575b3be56f15`

and the following authority:

- `L4_Governance_and_Architecture_Standard.md` — Locked;
- `L4_Validation_Standard.md` — Locked;
- `L4_System_Behavior_Specification_Template_v1.0.md` — Locked;
- `L4_Full_Corpus_Parent_Readiness_Baseline_1084_2026-08-29.md` — Locked;
- Locked parents `L3-A-001…003` and `L3-B-001…003`;
- Locked L1 Tenant/Institution lifecycle and ownership authority;
- Locked `L2-A01` plus `L2-A01_Tenant_Provisioning_Authority_and_Event_Amendment_2026-08-26.md`;
- Locked `L2-B01` plus `L2-B01_Institution_Activation_Authority_Amendment_2026-08-24.md`;
- `DEPENDENCY_MAP.md`.

No L5 specification was used as authority to repair or reinterpret L4 behavior.

---

## 3. Hard-Fail Validation Result

No unresolved L4 hard failure remains in the scoped specifications.

The review verified:

- every BH has defensible Locked-L3 provenance;
- no business rule was invented or modified;
- Tenant and Institution lifecycle states/transitions remain source-defined;
- exact canonical Institution events retain the Institution publisher;
- Tenant event behavior is used only where defined upstream;
- no new Tenant suspension/restoration/archival event is invented;
- canonical business ownership remains with Platform & Tenant or Institution as applicable;
- no unsupported human role is created where upstream authority names are absent;
- no material L3 omission is silently repaired from L2;
- no API, database, service/class, queue/broker, infrastructure, technical IAM/RBAC or UI design leaks into L4;
- parent outcomes and scenario boundaries are preserved;
- BHs remain observable and testable;
- metadata, identity and provenance remain canonical.

---

## 4. Bounded Upstream Authority Conditions

### 4.1 Tenant suspension / restoration

`L3-A-002` records that the exact named business actor for Tenant suspension/restoration is not defined upstream.

`L4-A-002` therefore requires an authoritative source decision but intentionally does not invent an actor or role.

This is accepted as a bounded authority condition for L4 system behavior. It does not authorize later technical RBAC or business-role invention.

### 4.2 Tenant archival

`L3-A-003` requires an approved permanent-retirement decision but does not name the approving business role.

`L4-A-003` therefore requires that approved decision without assigning unsupported authority.

This is accepted as a bounded authority condition for the approved system behavior.

### 4.3 Institution suspension / restoration and archival

`L4-B-002` and `L4-B-003` preserve source-authoritative decision requirements without fabricating a universal local role.

Institution lifecycle and event semantics remain owned by the Institution domain.

### 4.4 Institution archival entry position

The L4 archival specification does not invent additional archival entry states. It admits archival only from an entry position supported by authoritative Institution lifecycle semantics.

Any future change to those semantics requires controlled impact assessment.

---

## 5. Repository Changes Authorized by This Decision

The six scoped specifications may carry:

`Status: Architecture Approved`

with a patch-version metadata synchronization reflecting this controlled approval.

No behavioral redesign is required by this decision.

This decision does not authorize:

- L4 `Locked` status;
- new business actors, approval roles or authorization semantics;
- new lifecycle states or transitions;
- new canonical business events;
- L5 lifecycle promotion;
- automatic L4→L5 relation validation;
- Final L5 Corpus Lock.

---

## 6. Downstream Effect on the L5 Blocker Programme

R1A removes the Draft/Review L4 lifecycle blocker for these six parent specifications only.

Affected L5 architecture must still be handled under the Locked L4→L5 traceability methodology.

The next valid downstream sequence is:

```text
R1A L4 approval
→ validate exact affected L4 BH → L5 relations
→ update only current-bound traceability evidence
→ revalidate affected L5 specifications
→ reconcile the canonical 105-row register
→ consider L5 lifecycle promotion separately
```

This approval does not by itself change any Batch-9 L5 disposition.

---

## 7. Final Decision

```text
R1A scoped L4 parents                 6
Hard-fail validation                  PASS
Architecture Approved                 6 / 6
L4 Locked                             0
L5 specifications changed             0
L5 blocker dispositions changed       0
New L5 ADRs                           0
Final L5 Corpus Lock                  NOT AUTHORIZED
```

**Final Decision:** `ARCHITECTURE APPROVED — PROCEED TO NEXT R1 AUTHORITY SET AND LATER CONTROLLED L4→L5 TRACEABILITY REVALIDATION`.
