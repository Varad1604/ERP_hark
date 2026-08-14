# SutraERP Level 3 — Business Scenario Architecture

**Architecture Level:** L3 — Business Scenario Architecture
**Repository Area:** `architecture/L3_Business_Scenario_Architecture`
**Status:** Phase 1 Active
**Current Phase:** Phase 1 — Scenario Discovery
**Lock Status:** Phase 1 candidate inventory format locked; final scenario ownership not locked

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

The current repository scope is Phase 1 only.

Phase 1 is for scenario discovery and candidate inventory creation.

Phase 1 does not produce final L3 scenario documents.

Phase 1 does not lock final scenario ownership.

Phase 1 does not assign final L3 document numbers.

---

## 5. Current Folder Structure

```text
L3_Business_Scenario_Architecture/
│   README.md
│
├── 00_Governance/
│   ├── L3_Business_Scenario_Template_v1.0.md
│   ├── L3_Naming_Numbering_Standard.md
│   ├── L3_Phase1_Discovery_Rules.md
│   ├── L3_Phase1_Team_Instructions.md
│   ├── L3_Phase1_Workload_Distribution_Decision.md
│   ├── L3_Scenario_Type_and_Priority_Rules.md
│   ├── L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md
│   ├── L3_Section_A_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_C_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_F_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_G_Phase2_Consolidation_Watchlist.md
│   ├── L3_Section_H_Phase2_Consolidation_Watchlist.md
│   └── L3_Section_I_Phase2_Consolidation_Watchlist.md
│
└── 01_Phase1_Discovery/
    ├── L3_Phase1_Inventory_Chetan.md
    ├── L3_Phase1_Inventory_Devang.md
    ├── L3_Phase1_Inventory_Dhruv.md
    ├── L3_Phase1_Inventory_Pallav.md
    └── L3_Phase1_Inventory_Varad.md
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

### `L3_Section_C_Phase2_Consolidation_Watchlist.md`

The equivalent record for Section C — Student Lifecycle, produced by the independent verification of 2026-08-09 (`generated/reviews/l3-phase1-section-c-verification-2026-08-09.md`). It holds four candidate relationship clusters (2 `CLOSED`, 2 `OPEN`) and nine repository conflicts surfaced during discovery (2 `DETERMINED`, 5 `OPEN`, 1 `BLOCKED`, plus one conflict-dependent wording constraint). Nothing in it has been consolidated.

One conflict is blocking: **Policy Management has no owner** (`DEPENDENCY_MAP` section 7, status Open; `L2-C04` §2.6). Until the Lead Architect assigns one, an academic-progression policy lifecycle candidate and an alumni-transition policy lifecycle candidate cannot be discovered. Their absence from `L3_Phase1_Inventory_Chetan.md` is correct, not an omission.

### `L3_Section_F_Phase2_Consolidation_Watchlist.md`

The traceability register for the Section F audit and its authorized consolidation from 1,858 step-level rows to 87 institutional-outcome candidates. It preserves former ranges and lower-level rules for L4 authoring.

### `L3_Section_G_Phase2_Consolidation_Watchlist.md`

The Section G boundary and overlap register. It records which clusters remain open and the resolved H02/H07/H09 cross-cutting ownership decisions.

### `L3_Section_H_Phase2_Consolidation_Watchlist.md`

The audit-correction register for Section H, including the former-to-current crosswalk from 893 malformed or fragmented rows to 85 outcome candidates and the accepted dashboard, credit-transfer, H09-completeness, and accreditation-readiness decisions.

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
Phase 1 team discovery files: Created
Final L3 scenario list: Not locked
Final L3 scenario ownership: Not locked
Final L3 numbering: Not locked
L3 Template v1.0: Approved for pilot use
```
