# SutraERP L3 Phase 1 Discovery Rules

**Document:** L3 Phase 1 Discovery Rules
**Architecture Level:** L3 — Business Scenario Architecture
**Version:** 1.0
**Status:** Approved — Phase 1 Final / Frozen Baseline
**Applies To:** The final frozen Phase 1 candidate inventory and its historical discovery rules

---

## 1. Purpose

This document defines the rules for Phase 1 of SutraERP Level 3 Business Scenario Architecture.

Phase 1 exists to discover possible L3 business scenarios and capture them in a lightweight inventory format.

Phase 1 does not create final L3 scenario documents.

Phase 1 does not lock final scenario ownership.

Phase 1 does not assign final L3 scenario numbers.

---

## 2. Phase 1 Objective

The objective of Phase 1 is to identify real institutional business scenarios that may become Level 3 Business Scenario Architecture documents.

Each discovered candidate should represent one complete business outcome.

The Phase 1 output is a candidate inventory, not a final architecture document set.

---

## 3. Locked Phase 1 Output

Each team member must submit one inventory file using this format:

```text id="ck7f76"
L3_Phase1_Inventory_<Name>.md
```

Examples:

```text id="7bap21"
L3_Phase1_Inventory_Dhruv.md
L3_Phase1_Inventory_Varad.md
L3_Phase1_Inventory_Pallav.md
L3_Phase1_Inventory_Devang.md
L3_Phase1_Inventory_Chetan.md
```

Each inventory file must contain candidate scenario rows only.

No full L3 scenario document shall be created in Phase 1.

---

## 4. Locked Candidate Inventory Format

Each candidate row must use the following locked fields:

```markdown id="iez4s4"
| Candidate ID | Scenario Title | Trigger | Final Outcome | Primary Owner | Domains | Type | Priority |
|---|---|---|---|---|---|---|---|
```

Only these fields are mandatory for Phase 1.

Additional details such as source references, events, hand-offs, blockers, reviewers, completion criteria, failure paths, authorization, and audit will be handled during Phase 2 consolidation or full L3 scenario drafting.

---

## 5. Field Definitions

| Field          | Meaning                                                  |
| -------------- | -------------------------------------------------------- |
| Candidate ID   | Temporary Phase 1 identifier using `CAN-L3-<AREA>-<NNN>` |
| Scenario Title | Outcome-based business scenario name                     |
| Trigger        | Business event or condition that starts the scenario     |
| Final Outcome  | Business result that completes the scenario              |
| Primary Owner  | Likely domain owner of the final business outcome        |
| Domains        | Participating sections or domains                        |
| Type           | Simple, Cross-domain, or Critical                        |
| Priority       | P0, P1, or P2                                            |

---

## 6. Phase 1 Candidate ID Rule

Phase 1 candidate IDs use this format:

```text id="c7c7js"
CAN-L3-<AREA>-<NNN>
```

Examples:

```text id="ywdboa"
CAN-L3-C-001
CAN-L3-F-001
CAN-L3-I-001
CAN-L3-J-001
CAN-L3-X-001
```

`CAN` means candidate.

Candidate IDs are temporary.

They must not be used as final L3 scenario document IDs.

Numbering starts at `001` in each discovery area and is continuous, with no maximum digit width — see
`L3_Naming_Numbering_Standard.md` §4 and §7.1, and `L3_Phase1_Candidate_ID_Migration_2026-08-17.md` for
the one-time repository-wide renumbering applied on 2026-08-17.

---

## 7. What Counts as a Valid L3 Candidate

A valid candidate must describe one complete business outcome.

Valid examples:

```text id="bkqbtd"
Approved Candidate to Active Student
Student Fee Payment and Receipt
Regular Examination to Result Publication
Purchase Requisition to Goods Receipt and Inventory
Attendance Capture to Attendance Finalization
Employee Leave Approval to Availability Update
```

These examples are valid because each has a clear trigger and a clear final business outcome.

---

## 8. What Does Not Count as a Valid L3 Candidate

The following are not valid standalone L3 candidates:

```text id="30nu40"
Validate Payment
Generate Receipt
Send Notification
Check Eligibility
Update Student Status
Save Application
Approve Request
```

These are actions or workflow steps.

They may appear inside an L3 scenario, but they should not become separate L3 scenario documents.

---

## 9. Scenario Boundary Rule

Each Phase 1 candidate must be understandable from its title, trigger, and final outcome.

A candidate is weak if:

* the trigger is unclear;
* the final outcome is unclear;
* the title describes only a small action;
* the title is too broad;
* the title combines unrelated outcomes;
* the primary owner cannot be inferred at all.

Examples of weak candidates:

```text id="yzccwk"
Complete Student Lifecycle
Manage Finance
Process Examination
Handle Notifications
Do Procurement
```

These are too broad and must be split into smaller outcome-based candidates.

---

## 10. Primary Owner Rule

The Primary Owner field records the likely owner of the final business outcome.

The primary owner is not necessarily:

* the person who discovered the candidate;
* the first domain in the flow;
* the domain with the most steps;
* the domain most visible to the user.

Examples:

| Candidate                                           | Likely Primary Owner           | Reason                                                       |
| --------------------------------------------------- | ------------------------------ | ------------------------------------------------------------ |
| Approved Candidate to Active Student                | Student Lifecycle              | Final outcome is Active Student Context                      |
| Student Fee Payment and Receipt                     | Finance                        | Final outcome is financial settlement and receipt            |
| Regular Examination to Result Publication           | Examination                    | Final outcome is official result publication                 |
| Purchase Requisition to Goods Receipt and Inventory | Enterprise Resource Management | Final outcome is goods receipt and inventory/resource update |

Primary owner is still provisional in Phase 1 and will be confirmed during Phase 2 consolidation.

---

## 11. Domains Field Rule

The Domains field must list participating areas using section letters or governed variable-participant tokens.

Examples:

```text id="a67fgs"
A, C, F
C, D, I
F, J
G, F
A, H
```

Use only domains that genuinely participate in the scenario.

Do not list all domains by default.

### Variable participant notation

Use `VAR:<role>` only when all of the following are true:

* the candidate's Primary Owner is already known;
* a participant that varies by scenario instance materially initiates or co-produces the outcome;
* the exact section participant will be resolved for each authored scenario instance; and
* the role is precise, for example `VAR:Requesting Domain`, `VAR:Evidence-Owning Domain`, or `VAR:Task-Owning Domain`.

`VAR` is a Domains-field participation notation only. It shall never be used as a candidate ownership area, final L3 identifier area, or substitute for an owner decision. A domain that merely consumes the final event, query, certificate, or capability is not listed unless it materially participates in producing the scenario outcome.

The discovery-area code `X` remains reserved for an uncertain primary area under `L3_Naming_Numbering_Standard.md` section 5 and shall not be used as a Domains wildcard.

---

## 12. Scenario Type Rule

Allowed Type values:

```text id="sd0fc3"
Simple
Cross-domain
Critical
```

Use:

* `Simple` when the scenario mainly belongs to one domain with limited hand-offs.
* `Cross-domain` when multiple domains collaborate to complete the outcome.
* `Critical` when the scenario is academically, financially, legally, operationally, historically, or compliance sensitive.

When in doubt between `Cross-domain` and `Critical`, choose `Critical` only if the outcome has high institutional risk.

---

## 13. Priority Rule

Allowed Priority values:

```text id="u8m2pp"
P0
P1
P2
```

Use:

* `P0` for scenarios mandatory before safe implementation can proceed.
* `P1` for important operational baseline scenarios.
* `P2` for future, rare, optional, or lower-frequency scenarios.

Do not mark a scenario P0 only because the domain is important.

Priority must be based on implementation readiness, institutional risk, and architectural coverage.

---

## 14. What Phase 1 Must Not Do

Phase 1 must not:

* create full L3 scenario documents;
* assign final L3 scenario IDs;
* lock final scenario ownership;
* lock final scenario creators;
* lock final mandatory reviewers;
* define API endpoints;
* define database schemas;
* define UI screens;
* define button labels;
* define deployment architecture;
* create new business events;
* create new lifecycle states;
* rename canonical entities;
* change L1 or L2 ownership;
* silently resolve repository conflicts.

---

## 15. Use of ChatGPT Project During Phase 1

Team members may use the SutraERP ChatGPT project to:

* understand assigned L2 workflow areas;
* identify possible L3 candidate scenarios;
* test whether a candidate is outcome-based;
* improve trigger and final outcome wording;
* classify candidates by type and priority;
* identify likely participating domains.

Team members must not use ChatGPT output as repository authority.

Repository authority remains:

1. Locked Repository Decisions
2. Governance Standards
3. L0 Business Vision
4. Locked L1 Architecture
5. Dependency Map
6. L2 Workflow Documents
7. Canonical AI Context files
8. Current Phase 1 instructions

If ChatGPT suggests something that conflicts with L1 or L2, the repository source wins.

---

## 16. Phase 1 Submission Rule

Each team member must submit only one Markdown file.

Each row must use the locked Phase 1 inventory format.

The file should not include long explanations unless necessary.

The goal is a clean candidate list that can be reviewed and consolidated.

### 16.1 Conformance check before acceptance

Added 2026-08-14 following the Pallav all-assigned Phase 1 audit.

Every inventory must pass the repository conformance checker before it is accepted for review:

```text id="l3chk1"
python check_l3_inventory.py
```

The checker reads every file in `01_Phase1_Discovery/` by default, or specific files if given as arguments. It exits non-zero when any error is found. Warnings do not block acceptance but must be read and answered.

It enforces the mechanical parts of these rules:

| Level | Check |
| ----- | ----- |
| Error | Row does not carry exactly the eight locked fields (section 4) |
| Error | Candidate ID does not match `CAN-L3-<AREA>-<NNN>` (section 6) |
| Error | Duplicate candidate ID, within a file or across inventories |
| Error | Empty Scenario Title, Trigger, Final Outcome, Primary Owner or Domains |
| Error | Type or Priority outside the allowed values (Type and Priority Rules sections 3 and 8) |
| Error | Domains token is neither a section letter nor a `VAR:<role>` token (section 11) |
| Error | Distinct-outcome ratio below 85 percent |
| Warning | Distinct-outcome ratio below 97 percent, or candidates sharing an identical Final Outcome |
| Warning | Title begins with an action verb, or is too broad (sections 8 and 9) |
| Warning | Type is fully predicted by the number of participating domains |
| Warning | Type or Priority is near-uniform across the inventory |
| Warning | Domains or Primary Owner mixes conventions |

The distinct-outcome and derived-Type checks exist because a generated inventory can satisfy every field-level rule while still failing section 7. An inventory whose rows are path enumerations of the same outcome is not a discovery result, however many rows it holds.

A clean checker run is a floor, not a pass. It cannot judge whether a candidate is a complete institutional business outcome, whether ownership is correct, or whether an outcome is missing. Those remain review responsibilities under section 17.

---

## 17. Review After Phase 1

After all Phase 1 inventory files are submitted:

1. The Lead Architect / Plus reviewer will review the candidate lists.
2. Duplicate candidates will be identified.
3. Too-small candidates will be merged into larger outcome scenarios.
4. Too-broad candidates will be split.
5. Incorrect owners will be corrected.
6. Priority will be validated.
7. Final Phase 2 consolidation will begin.

---

## 18. Phase 1 Lock Statement

The following are locked for Phase 1:

```text id="rzjfv5"
Phase 1 creates candidate inventory only.
Each team member submits one Markdown inventory file.
The locked candidate row format is:
Candidate ID | Scenario Title | Trigger | Final Outcome | Primary Owner | Domains | Type | Priority.
Candidate IDs use CAN-L3-<AREA>-<NNN>.
No full L3 scenario documents are created in Phase 1.
Final ownership, final numbering, final creators, and final reviewers remain unlocked until Phase 2.
```

### 18.1 Final Freeze Rule — 2026-08-17

Phase 1 is now the final frozen discovery baseline. Its section ranges are continuous as frozen, including
`CAN-L3-C-075`, and Phase 2 remains active.

Phase 2 may confirm or correct Primary Owner, Type, and Priority where the governance standards permit,
but a Phase 2 disposition never removes or renumbers a Phase 1 candidate row. Merge, variant, duplicate,
split, deferral, rejection, and blocked relationships are recorded in the Phase 2 register while the Phase 1
candidate IDs and row presence remain unchanged. Final `L3-*` IDs are still assigned only after Phase 2
consolidation.
