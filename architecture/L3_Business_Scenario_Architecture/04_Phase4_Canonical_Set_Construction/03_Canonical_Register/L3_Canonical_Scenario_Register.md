# SutraERP L3 — Canonical Scenario Register

**Architecture Level:** L3 — Business Scenario Architecture  
**Phase:** Phase 4 — Stage 3 Canonical Register Construction  
**Status:** Draft  
**Owner:** Lead Architect  
**Last Updated:** 2026-08-18  
**Canonical Scenario Count:** 1,084  
**Final L3 IDs Assigned:** No  
**Depends On:** Phase 2 Final/Closed Scenario Register; Phase 4 Stage 1 Proposed Canonical Set; Phase 4 Stage 2 Canonical Set Audit; locked L3 v2.0 template and naming standard  

---

## 1. Purpose

This register is the canonical construction control plane for the proposed final L3 scenario set before final `L3-<SECTION>-<NNN>` numbering and full scenario authoring.

It does not replace Phase 1 or Phase 2 history. Phase 1 candidate IDs and Phase 2 working IDs remain traceability keys in their authoritative source artifacts.

No final L3 scenario ID is assigned by this register while its status is Draft.

---

## 2. Canonical Scenario Count

| Section | Canonical Scenarios |
|---|---:|
| A — Platform Foundation | 62 |
| B — Institution & Structure | 33 |
| C — Student Lifecycle | 75 |
| D — Academic Delivery | 57 |
| E — People & Workforce | 75 |
| F — Finance | 85 |
| G — Campus Operations | 320 |
| H — Institutional Services & Intelligence | 81 |
| I — Examination | 192 |
| J — Enterprise Resource Management | 104 |
| **Total** | **1,084** |

The total is consistent with the approved Stage 1 derivation:

```text
1,080 Phase 2 KEEP outputs
+ 4 approved split-child outputs
= 1,084 canonical scenarios
```

---

## 3. Register Files

| Section | Register Artifact | Canonical Rows |
|---|---|---:|
| A | `Section_A_Canonical_Register.md` | 62 |
| B | `Section_B_Canonical_Register.md` | 33 |
| C | `Section_C_Canonical_Register.md` | 75 |
| D | `Section_D_Canonical_Register.md` | 57 |
| E | `Section_E_Canonical_Register.md` | 75 |
| F | `Section_F_Canonical_Register.md` | 85 |
| G | `Section_G_001_080_Canonical_Register.md` | 79 |
| G | `Section_G_081_160_Canonical_Register.md` | 80 |
| G | `Section_G_161_240_Canonical_Register.md` | 79 |
| G | `Section_G_241_322_Canonical_Register.md` | 82 |
| H | `Section_H_Canonical_Register.md` | 81 |
| I | `Section_I_001_070_Canonical_Register.md` | 70 |
| I | `Section_I_071_140_Canonical_Register.md` | 65 |
| I | `Section_I_141_197_Canonical_Register.md` | 57 |
| J | `Section_J_Canonical_Register.md` | 104 |
| **Total** | **15 register shards** | **1,084** |

G and I are sharded only for repository maintainability and reviewability. They remain one logical Section G register and one logical Section I register.

---

## 4. Applied Canonical Transformations

Only transformations already approved by the Final/Closed Phase 2 register and validated by the Stage 2 audit are applied here.

### 4.1 Split Outcomes

`CAN-L3-B-021` is replaced by two canonical outputs:

1. **Institutional Campus Decommissioning to Decommissioned Campus**
2. **Decommissioned Institutional Campus to Archived Historical Campus**

`CAN-L3-D-042` is replaced by two canonical outputs:

1. **Active Learning Opportunity to Governed Suspension**
2. **Archival-Eligible Learning Opportunity to Archived Historical Opportunity**

No new Phase 1 candidate IDs were created for these child outcomes.

### 4.2 Merge Survivors

- `CAN-L3-D-027` is absorbed into `CAN-L3-D-028`, producing **Academic Monitoring Requirement to Published Monitoring Outcome**.
- `CAN-L3-I-076` and `CAN-L3-I-087` are absorbed into `CAN-L3-I-072`, **Examination Evaluation to Final Approved Marks**.
- `CAN-L3-I-081`, `CAN-L3-I-082`, and `CAN-L3-I-083` are absorbed into `CAN-L3-I-074`, **Marks Submission Exception to Authoritative Marks Outcome**.

### 4.3 Variant Survivors

- `CAN-L3-G-002` is governed inside `CAN-L3-G-001`, **Campus Readiness Assessment to Governance Readiness Decision**.
- `CAN-L3-G-164` is governed inside `CAN-L3-G-163`, refined to **Emergency or First-Aid Incident to Immediate Institutional Medical Response**.
- `CAN-L3-J-099` through `CAN-L3-J-104` are governed disposal-method variants inside `CAN-L3-J-097`, refined to **Authorised Disposal Method Execution to Reconciled Physical Disposal Closure**.

---

## 5. Lineage Governance

For Sections A–F, source Phase 2 and Phase 1 lineage is carried directly in the section register where practical.

For Sections G–J, the frozen Phase 1 candidate ID is used as the compact stable register key, while the Phase 2 Final/Closed register remains the authoritative source for P2W identity and detailed disposition evidence.

Section G requires this treatment because the Phase 2 register intentionally retains 28 historical absorbed G rows whose point-in-time candidate text may collide with post-normalization active IDs. Those historical rows do not create canonical outputs.

This centralized register satisfies construction traceability without requiring Phase 1/2 lineage metadata to be duplicated in future individual final L3 scenario documents.

---

## 6. Construction Status

Every canonical row currently has:

```text
Construction Status = Not Authored
```

No full L3 scenario document has been created from this register yet.

No final `L3-*` numbering has been assigned.

---

## 7. Next Governance Gate

Before final IDs are assigned, this register must pass the Stage 3 mechanical and architecture-control validation for:

- exact 1,084-row total;
- section count reconciliation;
- split/merge/variant transformation integrity;
- lineage preservation;
- no duplicate canonical output introduced by construction;
- no final ID assigned prematurely;
- no ownership, type, priority, or architecture-boundary drift introduced during materialization.

Only after that validation and Lead Architect approval may the register be frozen and final `L3-<PRIMARY_OWNER_SECTION>-<NNN>` IDs assigned.
