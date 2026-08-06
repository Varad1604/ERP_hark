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
│   └── L3_Scenario_Type_and_Priority_Rules.md
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

### `L3_Business_Scenario_Template_v1.0.md`

Defines the approved L3 scenario document template for pilot use.

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
