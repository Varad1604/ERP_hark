# SutraERP L3 — Stage 1 Proposed Canonical Scenario Set Working Draft

**Purpose:** Phase 4 Stage 1 derivation only  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L3 — Business Scenario Architecture  
**Phase:** Phase 4 — Canonical Scenario Set Construction  
**Date:** 2026-08-18  
**Authoritative input:** `architecture/L3_Business_Scenario_Architecture/02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md`  
**Constraint:** No final L3 IDs are assigned in this draft.

---

## 1. Derivation Rule

The Phase 2 register contains 1,124 rows representing 1,096 active frozen Phase 1 candidates plus 28 historical absorbed Section G rows retained only for traceability.

For Stage 1 canonical-set construction:

- every active `KEEP` row carries forward as one proposed canonical scenario;
- active `MERGE` rows create no additional canonical scenario and are absorbed into their approved surviving parent;
- active `VARIANT` rows create no additional canonical scenario and remain governed variants of their approved surviving parent;
- active `SPLIT-REQUIRED` rows are replaced by their two already-approved child outcomes;
- historical Section G traceability rows do not create canonical scenarios;
- no final `L3-*` ID is assigned during Stage 1.

---

## 2. Canonical Set Count

| Item | Count |
|---|---:|
| Active frozen Phase 1 candidates | 1,096 |
| Active `KEEP` rows carried forward | 1,080 |
| Active non-KEEP rows requiring Stage 1 resolution | 16 |
| Active absorbed MERGE/VARIANT rows | 14 |
| Active SPLIT-REQUIRED parents | 2 |
| Approved split child outcomes created | 4 |
| **Proposed canonical scenarios** | **1,084** |

Formula:

```text
1,080 KEEP outputs
+ 4 approved split-child outputs
= 1,084 proposed canonical scenarios
```

---

## 3. Proposed Canonical Count by Final Owner Section

| Section | Active Phase 1 | Stage 1 effect | Proposed Canonical Count |
|---|---:|---|---:|
| A | 62 | 62 KEEP | 62 |
| B | 32 | 31 KEEP + B-021 split into 2 | 33 |
| C | 75 | 75 KEEP | 75 |
| D | 57 | 55 KEEP + D-027 merged + D-042 split into 2 | 57 |
| E | 75 | 75 KEEP | 75 |
| F | 85 | 85 KEEP | 85 |
| G | 322 | 320 KEEP + 2 active variants absorbed | 320 |
| H | 81 | 81 KEEP | 81 |
| I | 197 | 192 KEEP + 5 merges absorbed | 192 |
| J | 110 | 104 KEEP + 6 method variants absorbed | 104 |
| **Total** | **1,096** |  | **1,084** |

---

## 4. Active Non-KEEP Resolution Register

### Section B — Resolved Split

#### `P2W-B-0709 / CAN-L3-B-021`

Phase 2 parent: **Institutional Campus Decommission and Archive**  
Disposition: `SPLIT-REQUIRED`

Approved canonical children:

1. **Institutional Campus Decommissioning to Decommissioned Campus**
   - Trigger: an InstitutionalCampus permanently ceases authorized institutional operation and decommissioning is authorized.
   - Outcome: the InstitutionalCampus reaches the B03-defined Decommissioned condition, becomes unavailable for institutional operation, establishes the authoritative `CampusDecommissioned` fact, and preserves identity and lifecycle history.

2. **Decommissioned Institutional Campus to Archived Historical Campus**
   - Trigger: a Decommissioned InstitutionalCampus satisfies B03 archival conditions and a distinct archival action is initiated.
   - Outcome: the InstitutionalCampus reaches Archived, establishes the authoritative `CampusArchived` fact, remains preserved as historical institutional evidence, and is excluded from future operational use.

Boundary protection: G10 physical infrastructure lifecycle remains independently owned by Section G.

---

### Section D — Merge

#### `P2W-D-0747 / CAN-L3-D-027`

Phase 2 row: **Academic Monitoring Requirement to Activated Monitoring Case**  
Disposition: `MERGE` into `CAN-L3-D-028`.

Surviving parent: `CAN-L3-D-028` — **Activated Monitoring Cycle to Published Monitoring Outcome**.

Stage 1 canonical-title recommendation:

**Academic Monitoring Requirement to Published Monitoring Outcome**

Reason: activation is an intermediate state in the same Monitoring Case lifecycle; the merged canonical scenario should begin at the monitoring requirement and end at the independently consumed published monitoring outcome.

This is a Stage 1 title recommendation only; it does not change L2 ownership, lifecycle, or events.

---

### Section D — Resolved Split

#### `P2W-D-0762 / CAN-L3-D-042`

Phase 2 parent: **Learning Opportunity Suspension to Archived Historical Opportunity**  
Disposition: `SPLIT-REQUIRED`

Approved canonical children:

1. **Active Learning Opportunity to Governed Suspension**
   - Trigger: a governance condition requires an Active flexible-learning opportunity to cease availability temporarily or under controlled restriction.
   - Outcome: the Learning Opportunity reaches Suspended, becomes unavailable for new participation according to D06 governance, preserves existing valid participation, and retains authoritative suspension history.

2. **Archival-Eligible Learning Opportunity to Archived Historical Opportunity**
   - Trigger: a Learning Opportunity completes its applicable operational period and satisfies D06 archival conditions after relevant enrollment and recognition obligations are resolved.
   - Outcome: the Learning Opportunity reaches Archived, is removed from future operational availability, and remains immutable historical flexible-learning context.

Boundary protection: these children remain distinct from `CAN-L3-D-047` Flexible Learning Academic Session Archival.

---

### Section G — Active Variants

#### Current active `CAN-L3-G-002`

Scenario: **Examination Readiness Assessment**  
Disposition: `VARIANT` of `CAN-L3-G-001` — **Campus Readiness Assessment to Governance Readiness Decision**.

Canonical action: no separate canonical scenario. Examination Readiness remains a governed readiness-type variant inside the G00 Campus Readiness scenario. Examination planning, allocation, preparation, exception handling, and conduct remain Section I responsibilities.

#### Current active `CAN-L3-G-164`

Scenario: **First Aid Incident to Stabilised Patient Outcome**  
Disposition: `VARIANT` of `CAN-L3-G-163` — **Emergency Incident to Immediate Institutional Medical Response**.

Canonical action: no separate canonical scenario.

Stage 2-refined canonical-title recommendation for the surviving parent:

**Emergency or First-Aid Incident to Immediate Institutional Medical Response**

Reason: the parent must visibly encompass the approved First Aid variant without creating a separate First Aid aggregate, lifecycle, or event, while remaining distinct from the routine `Walk-in Consultation to Completed Medical Encounter` scenario.

This is a title/boundary wording recommendation only; G06 remains the canonical owner.

---

### Section I — Active Merges

#### Into `CAN-L3-I-072` — Examination Evaluation to Final Approved Marks

The following active rows are absorbed:

- `P2W-I-0928 / CAN-L3-I-076` — **Evaluation Discrepancy Before Approval to Valid Evaluated Marks**
- `P2W-I-0939 / CAN-L3-I-087` — **Evaluation Process Interruption to Resumed Custody-Preserving Evaluation**

Canonical action: both become governed paths within the canonical evaluation-to-Final-Approved-Marks scenario. During full scenario authoring, the discrepancy/correction path belongs under the appropriate alternate/correction flow, while process interruption belongs under `Failure, Interruption and Business Recovery`.

#### Into `CAN-L3-I-074` — Marks Submission Exception to Authoritative Marks Outcome

The following active rows are absorbed:

- `P2W-I-0933 / CAN-L3-I-081` — **Unavailable Theory Answer Script to Resolved Evaluation Outcome**
- `P2W-I-0934 / CAN-L3-I-082` — **Damaged Evaluation Evidence to Authoritative Evaluation Resolution**
- `P2W-I-0935 / CAN-L3-I-083` — **Candidate-to-Script Mapping Error to Correct Evaluation Record**

Canonical action: these are governed exception, correction, or recovery paths inside the broader authoritative-marks-exception outcome and create no separate canonical scenarios. Their exact placement must preserve the locked v2.0 distinction between alternate business flow and business failure/recovery.

---

### Section J — Active Variants

Surviving parent:

`P2W-J-0159 / CAN-L3-J-097` — **Authorised Standard Disposal Case to Reconciled Physical Disposal Closure**.

The following active rows are governed method variants and create no separate canonical scenario:

- `CAN-L3-J-099` — Destruction
- `CAN-L3-J-100` — Scrap
- `CAN-L3-J-101` — Sale or Auction
- `CAN-L3-J-102` — Donation
- `CAN-L3-J-103` — External Transfer-Out
- `CAN-L3-J-104` — Recycling or External Return

Stage 2-refined canonical-title recommendation for the surviving parent:

**Authorised Disposal Method Execution to Reconciled Physical Disposal Closure**

Reason: the title must encompass the approved configurable disposal-method variants while remaining execution-focused and distinct from separate canonical scenarios for partial disposal, audit-origin treatment, asset-linked treatment, emergency disposal, write-off, and disposal exceptions.

---

## 5. Historical Section G Rows

The Phase 2 register retains 28 historical absorbed Section G rows solely for consolidation traceability:

- 20 historical `MERGE` rows;
- 6 historical `VARIANT` rows;
- 2 historical `DUPLICATE` rows.

They are not active Phase 1 candidates and create no Stage 1 canonical scenarios.

The only active Section G `VARIANT` rows affecting the current frozen baseline are `CAN-L3-G-002` and `CAN-L3-G-164`.

---

## 6. Stage 1 Result

**Proposed canonical scenario set size: 1,084.**

Stage 1 makes no final-ID assignment and creates no repository scenario documents.

The proposed set consists of:

- 1,080 active Phase 2 `KEEP` outcomes carried forward;
- four approved child outcomes from the two resolved Phase 2 splits;
- no standalone output for the 14 active MERGE/VARIANT rows;
- no output from the 28 historical Section G traceability rows.

Three surviving parent titles are recommended for boundary clarity before the canonical register is frozen:

1. D-028 → **Academic Monitoring Requirement to Published Monitoring Outcome**
2. G-163 → **Emergency or First-Aid Incident to Immediate Institutional Medical Response**
3. J-097 → **Authorised Disposal Method Execution to Reconciled Physical Disposal Closure**

These recommendations do not change canonical ownership, state, lifecycle, business rule, event, or dependency decisions.

---

## 7. Next Gate

Stage 1 derivation is complete as a working set, but it is not yet repository-frozen.

Next: perform the Stage 2 repository-wide canonical-set audit against this 1,084-scenario derivation before assigning final L3 IDs or creating full scenario documents.
