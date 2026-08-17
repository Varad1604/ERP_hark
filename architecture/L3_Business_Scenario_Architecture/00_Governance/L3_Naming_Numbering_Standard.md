# SutraERP L3 Naming and Numbering Standard

**Document:** L3 Naming and Numbering Standard
**Architecture Level:** L3 — Business Scenario Architecture
**Version:** 1.0
**Status:** Approved for Phase 1 Use
**Applies To:** L3 candidate inventory, final L3 scenario IDs, and L3 scenario file names

---

## 1. Purpose

This document defines the naming and numbering rules for Level 3 Business Scenario Architecture.

The purpose is to ensure that L3 scenario identifiers remain:

* clear;
* stable;
* traceable;
* independent of L2 numbering;
* aligned with canonical ownership;
* easy to review during Phase 1 and Phase 2.

---

## 2. Core Rule

L3 numbering is independent of L2 numbering.

An L3 scenario may derive from one or many L2 workflow documents.

Therefore, L3 scenario numbers must not copy L2 workflow numbers.

Example:

```text
Correct:
L3-C-001_Approved_Candidate_to_Active_Student.md

Incorrect:
L3-C01-001_Approved_Candidate_to_Active_Student.md
```

`C01` is an L2 document identifier. It must not be embedded into the final L3 scenario number.

---

## 3. Identifier Types Used in L3

The L3 repository uses different identifiers for different purposes.

| Identifier Type             | Format                             | Used For                                            |
| --------------------------- | ---------------------------------- | --------------------------------------------------- |
| Phase 1 candidate ID        | `CAN-L3-<AREA>-<NNN>`              | Temporary candidate scenarios discovered in Phase 1 |
| Final L3 scenario ID        | `L3-<PRIMARY_OWNER_SECTION>-<NNN>` | Approved L3 scenarios after Phase 2 consolidation   |
| L3 issue ID                 | `L3-ISS-<NNN>`                     | Open L3 architecture or repository issues           |
| L3 architecture decision ID | `L3-ADR-<NNN>`                     | L3 architecture decisions                           |

Only `CAN-L3` is used during Phase 1 discovery.

Final scenario IDs are assigned only after Phase 2 consolidation.

---

## 4. Phase 1 Candidate ID Format

Phase 1 candidate scenarios use this format:

```text
CAN-L3-<AREA>-<NNN>
```

Example:

```text
CAN-L3-C-001
```

Meaning:

| Segment  | Meaning                                               |
| -------- | ----------------------------------------------------- |
| `CAN`    | Candidate                                             |
| `L3`     | Level 3 Business Scenario Architecture                |
| `<AREA>` | Discovery area                                        |
| `<NNN>`  | Running number within that discovery area, minimum three digits, unbounded upward (`001` … `999` … `1000` … `1001` …) |

Candidate IDs are temporary.

They do not represent final scenario ownership.

They do not represent final scenario numbering.

They do not guarantee that the candidate will become a final L3 document.

---

## 5. Discovery Area Codes

Use the following area codes during Phase 1:

| Code | Discovery Area                         |
| ---- | -------------------------------------- |
| `A`  | Platform Foundation                    |
| `B`  | Institution & Structure                |
| `C`  | Student Lifecycle                      |
| `D`  | Academic Delivery                      |
| `E`  | People & Workforce                     |
| `F`  | Finance                                |
| `G`  | Campus Operations                      |
| `H`  | Institutional Services & Intelligence  |
| `I`  | Examination                            |
| `J`  | Enterprise Resource Management         |
| `X`  | Cross-domain or uncertain primary area |

Use `X` only when the likely primary owner cannot be safely identified during discovery.

No final L3 scenario should remain under `X`.

`X` is an identifier-area discovery code, not a wildcard for the inventory `Domains` field. A genuinely variable participant in that field uses the governed `VAR:<role>` notation defined by `L3_Phase1_Discovery_Rules.md` section 11.

---

## 6. Candidate ID Examples

```text
CAN-L3-C-001 — Approved Candidate to Active Student
CAN-L3-F-001 — Student Fee Payment and Receipt
CAN-L3-I-001 — Regular Examination to Result Publication
CAN-L3-J-001 — Purchase Requisition to Goods Receipt and Inventory
CAN-L3-X-001 — Cross-domain Scenario Requiring Ownership Review
```

---

## 7. Candidate ID Assignment Rules

Each team member assigns candidate IDs only within the discovery areas they are working on.

Candidate numbering starts at `001` for each area and is continuous within that area's active
inventory — no gaps between `001` and the last active number.

Examples:

```text
CAN-L3-C-001
CAN-L3-C-002
CAN-L3-C-003
```

and separately:

```text
CAN-L3-F-001
CAN-L3-F-002
CAN-L3-F-003
```

Candidate numbers do not need to match final L3 scenario numbers.

If a candidate is later merged, split, deferred, or rejected, its original candidate ID remains as a historical discovery reference and is never reused. Its removal from the active inventory therefore does not create a gap under the continuity rule above — continuity is measured against the active inventory only.

### 7.1 One-Time Continuous Numbering Migration (2026-08-17)

On 2026-08-17 every active section's candidate numbering was normalized to start at `001` and run
continuously, in a controlled, one-time, repository-wide migration. Before that date, sections had
accumulated non-`001` starting points through ordinary Phase 1 discovery and consolidation activity.
The complete old-ID → new-ID mapping, section by section, is the authoritative record in
`L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.

**This migration does not repeat.** After 2026-08-17, active candidate IDs are frozen again:

* Future merges, splits, duplicate resolutions, or rejections retire the affected candidate ID(s)
  permanently. They do **not** trigger a renumbering of the remaining active candidates, and they do
  **not** need to be back-filled to keep the sequence gap-free.
* `check_l3_inventory.py` enforces continuity as a submission quality gate against the numbering as it
  stands today, immediately after the 2026-08-17 migration. Whether a future retirement's resulting gap
  is closed by an explicit, separately authorized micro-renumbering of just the affected area, or left
  as a recorded gap, is a Phase 2 / Lead Architect governance decision at that time — it is not an
  automatic consequence of the retirement itself, and it is not this document's decision to make in
  advance.
* A new full-repository renumbering may only happen again under the same kind of explicit, one-time,
  documented authorization this one had — never as a routine consequence of Phase 2 consolidation.

---

## 8. Final L3 Scenario ID Format

Final L3 scenarios use this format:

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>
```

Example:

```text
L3-C-001
```

Meaning:

| Segment                   | Meaning                                              |
| ------------------------- | ---------------------------------------------------- |
| `L3`                      | Level 3 Business Scenario Architecture               |
| `<PRIMARY_OWNER_SECTION>` | Section that owns the final business outcome         |
| `<NNN>`                   | Three-digit running number within that owner section |

Final IDs are assigned only after Phase 2 consolidation.

---

## 9. Final L3 File Name Format

Final L3 scenario files use this format:

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>_<Scenario_Slug>.md
```

Examples:

```text
L3-C-001_Approved_Candidate_to_Active_Student.md
L3-F-001_Student_Fee_Payment_and_Receipt.md
L3-I-001_Regular_Examination_to_Result_Publication.md
L3-J-001_Purchase_Requisition_to_Goods_Receipt_and_Inventory.md
```

---

## 10. Scenario Slug Rules

The scenario slug must:

* use clear business words;
* use title-style words separated by underscores;
* avoid implementation terms;
* avoid abbreviations unless already canonical;
* describe the business outcome.

Correct examples:

```text
Approved_Candidate_to_Active_Student
Student_Fee_Payment_and_Receipt
Regular_Examination_to_Result_Publication
Purchase_Requisition_to_Goods_Receipt_and_Inventory
```

Incorrect examples:

```text
Validate_Payment
Generate_Receipt
Update_Status
API_Student_Activation
DB_Fee_Posting
Screen_Result_Publish
```

---

## 11. Scenario Title Rules

Scenario titles must be outcome-based.

A good title usually follows one of these patterns:

```text
<Starting Business State> to <Final Business Outcome>
```

or:

```text
<Business Outcome Noun Phrase>
```

Examples:

```text
Approved Candidate to Active Student
Student Fee Payment and Receipt
Regular Examination to Result Publication
Purchase Requisition to Goods Receipt and Inventory
Employee Leave Approval to Availability Update
Attendance Capture to Attendance Finalization
```

Avoid small workflow-action titles:

```text
Validate Payment
Generate Receipt
Check Eligibility
Send Notification
Update Student Status
```

These are steps inside L2/L3 flows, not standalone L3 business scenarios.

---

## 12. Primary Owner Numbering Rule

The final owner section is based on the domain that owns the final business outcome.

The final owner is not necessarily:

* the person who discovered the scenario;
* the first domain in the flow;
* the domain with the most steps;
* the domain most visible to users.

Examples:

| Scenario                                            | Likely Final Owner                         | Reason                                                       |
| --------------------------------------------------- | ------------------------------------------ | ------------------------------------------------------------ |
| Approved Candidate to Active Student                | Section C — Student Lifecycle              | Final outcome is Active Student Context                      |
| Student Fee Payment and Receipt                     | Section F — Finance                        | Final outcome is financial settlement and receipt            |
| Regular Examination to Result Publication           | Section I — Examination                    | Final outcome is official published result                   |
| Purchase Requisition to Goods Receipt and Inventory | Section J — Enterprise Resource Management | Final outcome is goods receipt and inventory/resource update |

Final owner confirmation occurs during Phase 2 consolidation.

---

## 13. Candidate to Final ID Traceability

When a Phase 1 candidate becomes an approved L3 scenario, the final document must preserve the candidate trace.

Example:

```text
Candidate ID:
CAN-L3-C-001

Final Scenario ID:
L3-C-001

Final File:
L3-C-001_Approved_Candidate_to_Active_Student.md
```

The final L3 document should include the original candidate ID in its metadata.

---

## 14. Merge and Split Traceability

If candidates are merged:

```text
CAN-L3-F-001 + CAN-L3-F-002 → L3-F-001
```

If a candidate is split:

```text
CAN-L3-X-001 → L3-G-001 + L3-F-002
```

The Phase 2 Consolidation Register must preserve this traceability when that register is created.

---

## 15. Deferred or Rejected Candidates

Deferred, rejected, duplicate, merge, split, and blocked candidates remain inventory statuses during Phase 1 and Phase 2.

Separate folders or files are not required at the start of L3.

They may be introduced later only if the volume justifies it.

---

## 16. Versioning Rules

L3 template and governance documents use version numbers.

| Version | Meaning                                          |
| ------- | ------------------------------------------------ |
| `v1.0`  | First approved version for pilot and Phase 1 use |
| `v1.1`  | Minor clarification or correction                |
| `v1.2`  | Additional minor improvement                     |
| `v2.0`  | Major structural redesign                        |

A `v2.0` version should be created only if pilot validation proves that the v1.x structure is insufficient.

---

## 17. Prohibited Numbering Patterns

Do not use L2 workflow document numbers as final L3 numbers.

Incorrect:

```text
L3-C01-001
L3-F02-001
L3-I10-001
```

Do not use candidate IDs as final document file names.

Incorrect:

```text
CAN-L3-C-001_Approved_Candidate_to_Active_Student.md
```

Do not use unclear or implementation-based names.

Incorrect:

```text
L3-C-001_Student_API.md
L3-F-001_Payment_DB_Update.md
L3-I-001_Result_Screen.md
```

---

## 18. Lock Statement

The following rules are locked for Phase 1:

```text
Phase 1 candidate IDs use CAN-L3-<AREA>-<NNN>.
Candidate numbering starts at 001 in every area and is continuous, with no maximum width.
Final L3 scenario IDs use L3-<PRIMARY_OWNER_SECTION>-<NNN>.
Final L3 file names use L3-<PRIMARY_OWNER_SECTION>-<NNN>_<Scenario_Slug>.md.
CAN-L3 is temporary and must not be used for final L3 documents.
L3 numbering is independent of L2 numbering.
Final scenario ownership is confirmed during Phase 2 consolidation.
Candidate IDs were normalized once, repository-wide, on 2026-08-17
  (see L3_Phase1_Candidate_ID_Migration_2026-08-17.md). This does not repeat as a
  routine event -- see section 7.1.
```
