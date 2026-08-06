# SutraERP L3 Phase 1 Team Instructions

**Document:** L3 Phase 1 Team Instructions
**Architecture Level:** L3 — Business Scenario Architecture
**Version:** 1.0
**Status:** Approved for Phase 1 Use
**Applies To:** All team members preparing Phase 1 candidate scenario inventories

---

## 1. Purpose

This document explains how each team member must perform Phase 1 scenario discovery for SutraERP Level 3 Business Scenario Architecture.

Phase 1 is a discovery phase.

The goal is to identify candidate L3 business scenarios from assigned L2 areas and submit them in a consistent inventory format.

---

## 2. Phase 1 Output

Each team member must submit one Markdown inventory file.

File name format:

```text
L3_Phase1_Inventory_<Name>.md
```

Required files:

```text
L3_Phase1_Inventory_Dhruv.md
L3_Phase1_Inventory_Varad.md
L3_Phase1_Inventory_Pallav.md
L3_Phase1_Inventory_Devang.md
L3_Phase1_Inventory_Chetan.md
```

Each file must contain only Phase 1 candidate scenario rows.

No full L3 scenario documents shall be created in Phase 1.

---

## 3. Locked Inventory Format

Each team member must use this exact table format:

```markdown
| Candidate ID | Scenario Title | Trigger | Final Outcome | Primary Owner | Domains | Type | Priority |
|---|---|---|---|---|---|---|---|
```

No additional mandatory columns shall be added during Phase 1.

If extra notes are required, they may be kept separately by the author, but they should not change the locked Phase 1 submission format.

---

## 4. Field Rules

| Field          | Rule                                                  |
| -------------- | ----------------------------------------------------- |
| Candidate ID   | Use `CAN-L3-<AREA>-<NNN>`                             |
| Scenario Title | Use an outcome-based scenario name                    |
| Trigger        | State what starts the scenario                        |
| Final Outcome  | State the business result that completes the scenario |
| Primary Owner  | State the likely owner of the final business outcome  |
| Domains        | List participating sections/domains                   |
| Type           | Use only `Simple`, `Cross-domain`, or `Critical`      |
| Priority       | Use only `P0`, `P1`, or `P2`                          |

---

## 5. Team Responsibility Split

The following split is locked for Phase 1 discovery only.

It does not change L1 or L2 canonical ownership.

It does not lock final L3 scenario ownership.

| Team Member | Phase 1 Discovery Responsibility                                                                                    |
| ----------- | ------------------------------------------------------------------------------------------------------------------- |
| Dhruv       | Platform Foundation, Section H, governance, cross-domain scenario candidates, pilot framing                         |
| Varad       | Student Lifecycle, Academic Progression, Qualification Completion, Workforce, HR, Leave, Payroll-related candidates |
| Pallav      | Institution, Organisation, Curriculum, Academic Calendar, Academic Delivery, Timetable, Teaching, Attendance        |
| Devang      | Selected Examination and selected Campus Operations candidates                                                      |
| Chetan      | Selected Finance, Procurement, Inventory, Asset, Payables/Receivables candidates                                    |

Final scenario ownership, final scenario creator, and mandatory reviewers will be decided during Phase 2 consolidation.

---

## 6. Work Method for Each Team Member

Each team member should follow this sequence:

```text
1. Review assigned L2 documents and relevant Canonical AI Context files.
2. Identify complete business outcomes.
3. Avoid small workflow actions.
4. Write one candidate row for each possible L3 scenario.
5. Use temporary CAN-L3 numbering.
6. Assign provisional Type and Priority.
7. Submit the Markdown inventory file for review.
```

---

## 7. How to Use the SutraERP ChatGPT Project

Team members may use the SutraERP ChatGPT project as an architecture assistant.

Allowed uses:

* understand assigned L2 workflow areas;
* identify possible L3 scenario candidates;
* improve scenario title wording;
* clarify triggers and final outcomes;
* check whether a candidate is too small or too broad;
* classify Type and Priority provisionally;
* identify likely participating domains.

Not allowed:

* invent new domain ownership;
* rename canonical entities;
* create new lifecycle states;
* create new business events;
* resolve repository conflicts silently;
* add API endpoints;
* add database schemas;
* add UI or screen details;
* add technology or deployment decisions;
* treat AI output as more authoritative than repository documents.

Repository sources remain authoritative.

---

## 8. What Counts as a Good Candidate

A good candidate has:

* clear trigger;
* clear final outcome;
* likely primary owner;
* one complete business result;
* outcome-based title;
* correct domain participation;
* no implementation detail.

Good examples:

```text
Approved Candidate to Active Student
Student Fee Payment and Receipt
Regular Examination to Result Publication
Purchase Requisition to Goods Receipt and Inventory
Attendance Capture to Attendance Finalization
Employee Leave Approval to Availability Update
```

---

## 9. What Must Not Be Submitted as a Candidate

Do not submit action fragments such as:

```text
Validate Payment
Generate Receipt
Send Notification
Check Eligibility
Update Status
Approve Request
Save Record
```

These are workflow steps, not complete L3 scenarios.

Do not submit oversized candidates such as:

```text
Complete Student Lifecycle
Manage Finance
Process Examination
Handle Procurement
Run Campus Operations
```

These are too broad and must be split into outcome-based scenarios.

---

## 10. Candidate ID Instructions

Use the following format:

```text
CAN-L3-<AREA>-<NNN>
```

Examples:

```text
CAN-L3-C-001
CAN-L3-C-002
CAN-L3-F-001
CAN-L3-I-001
CAN-L3-X-001
```

Use `X` only when the likely primary owner cannot be safely identified during Phase 1.

Do not use final L3 numbering in Phase 1.

Incorrect:

```text
L3-C-001
```

Correct for Phase 1:

```text
CAN-L3-C-001
```

---

## 11. Type Instructions

Use only:

```text
Simple
Cross-domain
Critical
```

Use `Simple` when the scenario is mainly one domain.

Use `Cross-domain` when multiple domains collaborate to complete the outcome.

Use `Critical` when the scenario affects high-risk academic, financial, legal, compliance, payroll, lifecycle, resource custody, or historical outcomes.

---

## 12. Priority Instructions

Use only:

```text
P0
P1
P2
```

Use `P0` when the scenario is mandatory for the first L3 release.

Use `P1` when the scenario is important for operational completeness.

Use `P2` when the scenario is rare, future, optional, or lower-frequency.

Do not mark everything as P0.

---

## 13. Submission Header

Each team inventory file should begin with this header:

```markdown
# L3 Phase 1 Scenario Inventory — <Name>

**Prepared By:** <Name>  
**Phase:** Phase 1 — Scenario Discovery  
**Status:** Submitted for Review  

---
```

Then include the locked table.

---

## 14. Standard Submission Template

Each team member should use this structure:

```markdown
# L3 Phase 1 Scenario Inventory — <Name>

**Prepared By:** <Name>  
**Phase:** Phase 1 — Scenario Discovery  
**Status:** Submitted for Review  

---

| Candidate ID | Scenario Title | Trigger | Final Outcome | Primary Owner | Domains | Type | Priority |
|---|---|---|---|---|---|---|---|
```

---

## 15. Example Rows

```markdown
| Candidate ID | Scenario Title | Trigger | Final Outcome | Primary Owner | Domains | Type | Priority |
|---|---|---|---|---|---|---|---|
| CAN-L3-C-001 | Approved Candidate to Active Student | Admission is approved and activation prerequisites are available | Student Context becomes Active | Student Lifecycle | A, B, C, F | Cross-domain | P0 |
| CAN-L3-F-001 | Student Fee Payment and Receipt | Student or payer initiates payment against an outstanding charge | Payment is recorded, allocated, and receipt is available | Finance | C, F, H | Critical | P0 |
| CAN-L3-I-001 | Regular Examination to Result Publication | Evaluation is completed and result processing begins | Official result is published | Examination | C, D, F, I, H | Critical | P0 |
```

---

## 16. Review After Submission

After all five team inventory files are submitted:

1. The Lead Architect / Plus reviewer will review all candidate rows.
2. Duplicate candidates will be identified.
3. Small action-level candidates will be merged or rejected.
4. Overbroad candidates will be split.
5. Primary owners will be validated.
6. Type and Priority will be corrected where needed.
7. Phase 2 consolidation will begin.

---

## 17. Phase 1 Completion Condition

Phase 1 is complete when:

* all five inventory files are submitted;
* all rows use the locked table format;
* all candidate IDs follow `CAN-L3-<AREA>-<NNN>`;
* all rows have Trigger and Final Outcome;
* all rows have Type and Priority;
* the Lead Architect accepts the inventory set for Phase 2 consolidation.

---

## 18. Lock Statement

The following instructions are locked for Phase 1:

```text
Each team member submits one Markdown inventory file.
The locked file format is L3_Phase1_Inventory_<Name>.md.
The locked table format is Candidate ID | Scenario Title | Trigger | Final Outcome | Primary Owner | Domains | Type | Priority.
Candidate IDs use CAN-L3-<AREA>-<NNN>.
Type values are Simple, Cross-domain, Critical.
Priority values are P0, P1, P2.
No full L3 scenario documents are created in Phase 1.
```
