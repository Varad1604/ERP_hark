# SutraERP Level 3 — Business Scenario Architecture

**Architecture Level:** L3 — Business Scenario Architecture
**Repository Area:** `architecture/L3_Business_Scenario_Architecture`
**Status:** Phase 1 — Final / Frozen; Phase 2 — Consolidation Active
**Current Phase:** Phase 2 — Repository-Wide Scenario Consolidation
**Lock Status:** Phase 1 candidate inventory is the final frozen discovery baseline; Phase 1 candidate numbering normalized and frozen (2026-08-17); final scenario ownership not locked

---

## 1. Purpose

This directory contains the Level 3 Business Scenario Architecture for SutraERP.

Level 3 describes complete institutional business scenarios. Each L3 scenario represents one end-to-end business outcome that may involve one or more Level 2 domain workflow documents.

L3 does not define implementation design.

This level does not include:

* API endpoints
* database schemas
* software classes
* screen designs
* UI journeys
* deployment architecture
* infrastructure configuration
* programming details

Those belong to later architecture levels.

---

## 2. Position in the Architecture Hierarchy

SutraERP follows this architecture hierarchy:

```text
L0 — Business Vision
↓
L1 — Business Domain Architecture
↓
L2 — Domain Workflow Architecture
↓
L3 — Business Scenario Architecture
↓
L4 — System Behavior Specification
↓
L5 — Solution Architecture
↓
L6 — UI / UX Architecture
```

L3 depends on L2. It must not redefine L1 or L2 ownership, states, events, business rules, aggregates, or dependencies.

---

## 3. What L3 Represents

An L3 scenario describes one complete business outcome.

Good L3 scenario examples:

```text
Approved Candidate to Active Student
Student Fee Payment and Receipt
Regular Examination to Result Publication
Purchase Requisition to Goods Receipt and Inventory
Attendance Capture to Attendance Finalization
Employee Leave Approval to Availability Update
```

Weak L3 scenario examples:

```text
Validate Payment
Generate Receipt
Send Notification
Check Eligibility
Update Status
```

These weak examples are workflow steps, not complete L3 business scenarios.

---

## 4. Current Repository Scope

Phase 1 (scenario discovery and candidate inventory creation) is final and frozen: all five team
inventories form the final discovery baseline, and candidate numbering was normalized repository-wide on 2026-08-17 (see
`00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`).

Phase 2 (repository-wide scenario consolidation) is active. Its working artifact is
`02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md`.

Phase 1/2 do not produce final L3 scenario documents.

Phase 1/2 do not lock final scenario ownership.

Phase 1/2 do not assign final L3 document numbers.

---

## 5. Current Folder Structure

```text
L3_Business_Scenario_Architecture/
│   README.md
│
├── 00_Governance/
│   ├── L3_Business_Scenario_Template_v1.0.md
│   ├── L3_Naming_Numbering_Standard.md
│   ├── L3_Phase1_Candidate_ID_Migration_2026-08-17.md
│   ├── L3_Phase1_Discovery_Rules.md
│   ├── L3_Phase1_Team_Instructions.md
│   ├── L3_Phase1_Workload_Distribution_Decision.md
│   ├── L3_Scenario_Type_and_Priority_Rules.md
│   ├── L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md
│   ├── L3_Section_A_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_C_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_F_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_G_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_H_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_I_Phase2_Consolidation_Watchlist.md
│   └── L3_Section_J_Phase2_Consolidation_Watchlist.md
│
├── 01_Phase1_Discovery/
│   ├── L3_Phase1_Inventory_Chetan.md
│   ├── L3_Phase1_Inventory_Devang.md
│   ├── L3_Phase1_Inventory_Dhruv.md
│   ├── L3_Phase1_Inventory_Pallav.md
│   └── L3_Phase1_Inventory_Varad.md
│
└── 02_Phase2_Consolidation/
    └── L3_Phase2_Proposed_Canonical_Scenario_Register.md
```

Additional folders may be created later when the corresponding phase begins.

---

## 6. Governance Documents

### `L3_Naming_Numbering_Standard.md`

Defines candidate numbering, final scenario numbering, file naming, and versioning rules.

### `L3_Phase1_Discovery_Rules.md`

Defines what Phase 1 is, what it produces, and what it must not include.

### `L3_Scenario_Type_and_Priority_Rules.md`

Defines the allowed scenario type and priority values.

### `L3_Phase1_Team_Instructions.md`

Defines how each team member should prepare and submit their Phase 1 inventory.

### `L3_Phase1_Workload_Distribution_Decision.md`

Records the locked Phase 1 discovery workload distribution, its rationale, and its non-impact on canonical L1/L2 ownership.

### `L3_Business_Scenario_Template_v1.0.md`

Defines the approved L3 scenario document template for pilot use.

### `L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md`

Records the application of the independent Section A Phase 1 verification to the repository: the conflicts recorded, the corrections applied, every candidate change with its evidence, the candidates deliberately not added and why, the remaining blockers, and the final Section A coverage audit. It is a change record, not a governance standard — it does not add or amend any Phase 1 rule. Its section 7 Phase 2 watchlist is superseded; see below.

The two formerly blocking repository conflicts recorded by that application—**INT-301** and **INT-302**—were approved and resolved on 2026-08-13. The locked L1/L2 sources now define the identity/authentication/authorization ownership split and the dynamic Capability Readiness versus persisted Operational Readiness Governance & Certification boundary. Affected Section A provisional markers were removed; see `generated/reviews/L3_Phase1_Audit_Dhruv_Approved_Decisions_Application_2026-08-13.md`.

### `L3_Section_A_Phase2_Consolidation_Watchlist.md`

The authoritative record of the eighteen Section A candidate relationship clusters and their resolution status. All eighteen now have `CLOSED` or `APPLIED` outcomes. The register preserves retained/absorbed/split mappings, including A039/A046, A047/A054, and the A052/A067/A068 certificate-lifecycle split.

Relationship analysis is held here rather than in an inventory file because `L3_Phase1_Discovery_Rules.md` section 3 requires each inventory to contain candidate scenario rows only.

### `L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md`

The consolidation and traceability register for Sections B (Institution & Structure), D (Academic Delivery) and E (People & Workforce), covering the 2026-08-14 correction cycle applied to `L3_Phase1_Inventory_Pallav.md`.

### `L3_Section_C_Phase2_Consolidation_Watchlist.md`

The equivalent record for Section C — Student Lifecycle, produced by the independent verification of 2026-08-09 (`generated/reviews/l3-phase1-section-c-verification-2026-08-09.md`). It holds four candidate relationship clusters (2 `CLOSED`, 2 `OPEN`) and nine repository conflicts surfaced during discovery (2 `DETERMINED`, 5 `OPEN`, 1 `RESOLVED`, plus one conflict-dependent wording constraint). The original CF-08 row is preserved as historical evidence and followed by its current resolution.

**Policy Management ownership — resolved.** `CF-08` recorded this as blocking (`DEPENDENCY_MAP` §7, then status Open). `DEPENDENCY_MAP.md` line 409/434 records **Resolved 2026-08-15 (RC-03): institutional policy is owned by `L2-B01`**, with `AcademicPolicyUpdated` published by `L2-C04` and `AlumniTransitionPolicyUpdated` published by `L2-C08`. Phase 1 finalization added `CAN-L3-C-075` for the distinct Academic Progression policy-applicability outcome. No separate Alumni Policy candidate was added; policy validation remains within `CAN-L3-C-072` under C08's Alumni Transition lifecycle ownership. The watchlist preserves the original CF-08 entry as historical evidence and records the current resolution immediately after it.

### `L3_Section_J_Phase2_Consolidation_Watchlist.md`

The consolidation and traceability register for Section J — Enterprise Resource Management, covering `L3_Phase1_Inventory_Dhruv.md`'s J-section discovery and correction history.

### `L3_Phase1_Candidate_ID_Migration_2026-08-17.md`

The authoritative old-ID → new-ID crosswalk for the 2026-08-17 repository-wide candidate numbering normalization (see section 8 below). Every active candidate's pre-migration and post-migration ID is recorded here, grouped by section, in original row order.

### `L3_Section_F_Phase2_Consolidation_Watchlist.md`

The traceability register for the Section F audit and its authorized consolidation from 1,858 step-level rows to 85 institutional-outcome candidates. It preserves former ranges and lower-level rules for L4 authoring.

### `L3_Section_G_Phase2_Consolidation_Watchlist.md`

The Section G boundary and overlap register. It records which clusters remain open and the resolved H02/H07/H09 cross-cutting ownership decisions.

### `L3_Section_H_Phase2_Consolidation_Watchlist.md`

The audit-correction register for Section H, including the former-to-current crosswalk from 893 malformed or fragmented rows to 81 outcome candidates and the accepted dashboard, credit-transfer, H09-completeness, and accreditation-readiness decisions.

### `L3_Section_I_Phase2_Consolidation_Watchlist.md`

The Section I consolidation and conflict register created by its independent Phase 1 audit and correction cycle.

---

## 7. Phase 1 Candidate Inventory Format

Each Phase 1 candidate inventory must use the following locked table format:

```markdown
| Candidate ID | Scenario Title | Trigger | Final Outcome | Primary Owner | Domains | Type | Priority |
|---|---|---|---|---|---|---|---|
```

Only these fields are mandatory during Phase 1.

Detailed fields such as source references, events, hand-offs, blockers, reviewers, and completion criteria will be handled during Phase 2 consolidation or full L3 scenario drafting.

---

## 8. Candidate Numbering

Phase 1 candidates use temporary IDs:

```text
CAN-L3-<AREA>-<NNN>
```

Examples:

```text
CAN-L3-C-001
CAN-L3-F-001
CAN-L3-I-001
CAN-L3-J-001
CAN-L3-X-001
```

`CAN` means candidate.

Candidate IDs are temporary and must not be used as final L3 document IDs.

Numbering starts at `001` in each area and is continuous, with no maximum digit width. Candidate IDs
were normalized repository-wide on 2026-08-17 and the resulting final Phase 1 baseline is frozen — see
`00_Governance/L3_Naming_Numbering_Standard.md` §7.1 and
`00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.

---

## 9. Final L3 Numbering

Final scenario IDs will be assigned only after Phase 2 consolidation.

Final format:

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>
```

Final file format:

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>_<Scenario_Slug>.md
```

Example:

```text
L3-C-001_Approved_Candidate_to_Active_Student.md
```

---

## 10. Ownership Rule

The Phase 1 discoverer is not automatically the final scenario owner.

Final ownership is based on the domain that owns the final business outcome.

L3 must preserve canonical ownership from L1 and L2.

---

## 11. Current Phase 1 Team Inventory Files

Each team member must submit one Markdown inventory file:

```text
L3_Phase1_Inventory_Dhruv.md
L3_Phase1_Inventory_Varad.md
L3_Phase1_Inventory_Pallav.md
L3_Phase1_Inventory_Devang.md
L3_Phase1_Inventory_Chetan.md
```

Each file must contain only candidate L3 scenarios using the locked Phase 1 inventory format.

---

## 12. Repository Rule

Do not create final L3 scenario documents in Phase 1.

Do not create final scenario IDs in Phase 1.

Do not change canonical L1 or L2 ownership in Phase 1.

Do not introduce APIs, database design, UI design, implementation technology, or deployment details in L3.

---

## 13. Current Status

```text
Phase 1 candidate inventory format: Locked
Phase 1 candidate inventory: Final / Frozen (1,096 candidates)
Phase 1 candidate numbering: Continuous by section and frozen; no Phase 2 disposition may renumber it
Phase 2 consolidation: Final / Closed (1,124 register rows; 0 BLOCKED; 0 REVIEW-REQUIRED; 2 resolved SPLIT-REQUIRED decisions)
Final L3 scenario list: Not locked
Final L3 scenario ownership: Not locked
Final L3 numbering: Not locked
L3 Template v1.0: Approved for pilot use
```
