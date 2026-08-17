# L3 Section I — Phase 2 Consolidation Watchlist

**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section I — Examination, currently 197 active candidates in continuous range `CAN-L3-I-001` … `CAN-L3-I-197` (renumbered 2026-08-17; see "ID Renumbering — 2026-08-17" near the end of this document and `L3_Phase1_Candidate_ID_Migration_2026-08-17.md`). This header described 245 sparse candidates across `CAN-L3-I-001`…`CAN-L3-I-313` before the 2026-08-15 `I-TV` consolidation and is corrected here as a live governance fact; §1–§16 below remain unmodified historical narrative, using the numbering that existed when each was written.
**Status:** Active — Phase 2 input
**Created:** 2026-08-12
**Source:** independent adversarial audit of Section I, 2026-08-12 — `generated/reviews/L3_Phase1_Audit_Devang_ALL_ASSIGNED.md`
**Conformance applied:** 2026-08-12, recorded in section 8 below
**Consolidation executed:** 2026-08-12 under Lead Architect authorization, recorded in section 15 below

---

## 1. Purpose

This document holds the candidate relationships, boundary questions and deferred decisions identified during the independent Section I audit of 2026-08-12, and records for each one whether it is settled or still requires a Phase 2 decision.

It exists as a separate document because `L3_Phase1_Discovery_Rules.md` section 3 requires each Phase 1 inventory file to contain candidate scenario rows only, and section 16 requires the inventory to avoid long explanations.

**Consolidation was executed on 2026-08-12 under explicit Lead Architect authorization**, ahead of the ordinary `L3_Phase1_Discovery_Rules.md` section 17 sequence. Section 15 records exactly what was executed. Items still marked `OPEN` or `BLOCKED` below were **not** executed and remain Phase 2 work.

This document is not a governance standard. It adds and amends no Phase 1 rule and resolves no architectural conflict.

It follows the structure established by `L3_Section_A_Phase2_Consolidation_Watchlist.md`, `L3_Section_C_Phase2_Consolidation_Watchlist.md`, `L3_Section_F_Phase2_Consolidation_Watchlist.md` and `L3_Section_G_Phase2_Consolidation_Watchlist.md`.

---

## 2. Scope Statement

Section I is the only section assigned to `L3_Phase1_Inventory_Devang.md` under `L3_Phase1_Workload_Distribution_Decision.md` section 2. The assignment is complete and correctly scoped — no scope or governance violation was found.

Section I carries **17 L2 documents and 27,542 lines of source**, the largest section in the repository, against 15 documents for Finance and 11 for Platform Foundation. All 16 workflow documents (`L2-I01` … `L2-I16`) have candidate coverage. `L2-I00` is a control review document that defines no workflows and correctly has none.

---

## 3. Status Values

| Status | Meaning | Phase 2 action |
| --- | --- | --- |
| `CLOSED` | Repository evidence establishes the candidates are distinct or the question is settled. | None. Do not re-raise. |
| `EXECUTED` | Repository evidence was one-sided; the decision was executed on 2026-08-12. | None. Verify only. |
| `OPEN` | A genuine Phase 2 judgment is required. | Decide, then execute. |
| `BLOCKED` | Cannot be decided from current repository evidence. | Resolve the blocker first. |

---

## 4. Scale Statement

Section I now carries **245 candidates**, down from 311 at audit and 304 as originally submitted.

**The original estimate of 90–110 material outcomes is withdrawn.** It was a first-pass inference and it was too aggressive. Working through all sixteen sub-domains against their L2 workflow catalogues gives a defensible target of **200–230**: `L2-I14` alone specifies 18 canonical workflows, `L2-I15` 14 and `L2-I16` 12, and the conduct, evaluation and result documents each carry a comparable lifecycle plus a substantial exception set that the L2 sources define explicitly rather than leave implicit.

At 245 the section is close to that range. Every evidenced fragmentation cluster has been executed; what remains is not excess so much as the genuine breadth of a sixteen-document domain. Further reduction would require merging outcomes the L2 sources keep separate, and is not supported by repository evidence.

For comparison, Section F carries 1,858 candidates at a ratio near 20:1 and Section G carries 541. **Section I is the least fragmented large section discovered so far**, and the consolidation task here is correspondingly smaller and more surgical.

The estimate is audit inference from the L2 lifecycle, workflow, ownership and state-model sections. It is a consolidation target, not an approved number.

---

## 4A. Second-Pass Consolidation — 2026-08-14

The all-scope Phase 1 audit of 2026-08-14 found 208 of 245 Section I titles verb-initial, and identified candidates matching the examples `L3_Phase1_Discovery_Rules.md` §8 explicitly disqualifies as standalone L3 scenarios ("Send Notification", "Approve Request", "Save Application"). The 2026-08-12 consolidation had worked against the L2 workflow catalogues; §8 is a different and stricter test, and it had not been applied.

**A deliberate bright line was used.** Only rows that §8 disqualifies outright — procedural steps, notifications, receipt of a response, an approval treated as its own scenario — were retired. **Trigger-variant merges were not executed** (see 4B). Applied under user authorization on 2026-08-14.

| ID | Cluster | Retired | Absorbed into | Basis |
| --- | --- | --- | --- | --- |
| `I-FR-10` | I14 malpractice procedure rendered as consecutive steps | `I-270`, `I-271`, `I-272`, `I-273`, `I-274`, `I-275`, `I-276`, `I-277`, `I-278`, `I-279`, `I-280` | `I-305`, rewritten as **Suspected Examination Malpractice to Adjudicated Penalty Determination** | One institutional journey — report, register, preserve evidence, refer, notify, receive response, hear, decide — split across twelve rows. `I-277` ("Notify Candidate") is the §8 *Send Notification* example verbatim; `I-278` ("Receive Candidate Response") and `I-273` ("Preserve Evidence") are process steps with no independent institutional outcome. Appeal and review outcomes (`I-281`…`I-288`, `I-286`) were **retained** — they are post-terminal decisions with distinct consequences. |
| `I-FR-11` | I08 evaluation assignment, approval and pre-approval correction steps | `I-130`, `I-131`, `I-132`, `I-140`, `I-141`, `I-143`, `I-144`, `I-154`, `I-155` | `I-126`, rewritten as **Examination Evaluation to Final Approved Marks** and extended to `D, E, I` | `I-141` ("Approve Evaluated Marks") is the §8 *Approve Request* pattern and carries the journey's real terminal result, so the parent was extended to end at Final Approved Marks rather than at "evaluated marks". Evaluator assignment, reassignment, substitution and pre-approval correction are guards inside that outcome. `I-142` (withheld approval) **retained** as a genuine alternate terminal outcome. |
| `I-FR-12` | I07 theory-conduct material handling steps | `I-114`, `I-115`, `I-118` | `I-093` Conduct a Scheduled Theory Examination | Question-paper distribution, script collection and custody transfer are spans inside conducting the examination. The script *exception* rows `I-116` and `I-117` were **retained** — a missing or misidentified script is a distinct recovery outcome. |
| `I-FR-13` | I02 examination-session lifecycle transitions | `I-013`, `I-014`, `I-015`, `I-017` | `I-011` Establish an Examination Session | Activate, suspend, reopen and reject-and-return are lifecycle transitions of one aggregate. `L3_Phase1_Discovery_Rules.md` §8 lists "Update Student Status" as the analogous disqualified pattern. |

Section I total: **245 → 218 candidates.** Retired IDs must not be reused. All three §8 checker warnings for this file are now clear.

---

## 4B. Deferred — Trigger-Variant Merges

`OPEN`. These were analysed on 2026-08-14 and **deliberately not executed.** Each is a set of rows that reach the *same* terminal institutional outcome from different originating circumstances. Merging them is defensible but it is a judgment about outcome identity, not a rule application, and executing all of them would have retired roughly 90 further rows in a single unreviewed pass.

| ID | Cluster | Candidates | Shared terminal outcome | Argument against merging |
| --- | --- | --- | --- | --- |
| `I-TV-01` | Hall Ticket issuance variants | `I-058`, `I-059`, `I-060`, `I-063`, `I-065`, `I-066` around `I-055` | An eligible candidate holds valid examination entry authorization | Supplementary and special-arrangement issuance may carry distinct eligibility authority. |
| `I-TV-02` | Result publication state transitions and trigger variants | `I-178`…`I-182`, `I-185`…`I-192` around `I-177` | An official result is published | Withhold/release and delay/resume have real institutional consequence for candidates; post-revaluation and post-malpractice publication have distinct upstream authority. |
| `I-TV-03` | Theory and practical conduct variants | `I-094`, `I-095`, `I-099`, `I-100`, `I-102`, `I-106`…`I-110` around `I-093`; `I-076`…`I-087` around `I-069` | The examination is conducted and participation outcomes established | Multi-centre and special-arrangement conduct carry materially different control obligations. |
| `I-TV-04` | I08 marks validation-failure branches | `I-133`…`I-137`, `I-139`, `I-148`…`I-153`, `I-156` | Authoritative evaluated marks exist | Precedent exists both ways: Section G retired its equivalent `G-FR-05` validation-failure rows, but several of these carry independent policy authority (second/adjudicating evaluation). |

Phase 2 should decide each cluster explicitly. The `I-074`/`I-096`, `I-079`/`I-102` and `I-092`/`I-121` identical-Final-Outcome pairs flagged by `check_l3_inventory.py` sit inside `I-TV-03` and should be resolved with it.

---

## 5. Fragmentation Clusters

Twelve clusters were confirmed by testing candidate **groups** against the L2 lifecycle that governs them, not by reviewing rows individually.

| ID | Candidates | Recommended boundary | Evidence | Status |
| --- | --- | --- | --- | --- |
| `I-FR-01` | I-001 … I-009 | **Examination Policy Definition to Institutional Activation** — one scenario parameterised by policy category | `L2-I01` §8.1–8.4 define a single Create→Review→Approve→Activate lifecycle applied to all categories; §15 Configurable Policy Matrix lists the nine as *configurable categories* of one framework; §20 defines one Examination Policy State Model | `EXECUTED` |
| `I-FR-02` | I-157, I-158, I-174 | **Final Approved Marks to Institutionally Approved Result** | `L2-I09` §5.15 defines one 14-step lifecycle "concluding with Institutionally Approved Results ready for publication"; §14 scopes the domain "from Final Approved Marks until Institutionally Approved Results are ready for publication" | `EXECUTED` |
| `I-FR-03` | I-122 ← I-123, I-124, I-125 | I-122 absorbs the three per-component collection rows | `L2-I08` §5.7 Marks Collection is one capability; §8.7 one lifecycle stage. I-122's own trigger already spans practical, viva and theory | `EXECUTED` |
| `I-FR-04` | I-126, I-127, I-128 | One evaluation scenario alongside `I-FR-03` | `L2-I08` §5.5 Academic Evaluation is one capability; §8.6 one stage | `EXECUTED` |
| `I-FR-05` | I-029, I-040 … I-043, I-051 | **Candidate Examination Registration to Confirmed Registration**, with eligibility, finance and academic-context preconditions as governed paths inside it | `L2-I04` §20 Registration State Model defines one path Draft→Pending Validation→Pending Review→Approved→Confirmed; §8.3 and §10.4 are stages and exceptions *within* it | `EXECUTED` |
| `I-FR-06` | I-035 ← I-036 | One scenario with two governed termination paths | `L2-I04` §20 defines a single `Withdrawn` state; no `Cancelled` registration state exists; §5.11 covers "candidate **or institutional** withdrawal"; and §19 glosses Registration Withdrawal as "Controlled registration **cancellation**". Executed — I-036 retired into I-035, and I-037, I-048 and I-061 re-worded to `Withdrawn`. Closes `I-LS-01` | `EXECUTED` |
| `I-FR-13` | I-069 ← I-070, I-071; I-072 ← I-073; I-076 ← I-077 | **Non-theory assessment conduct is one workflow, not per-type workflows** | **Found 2026-08-12 during this round.** `L2-I06` §4.3 *Unified Assessment Conduct Framework*: "All non-theory examination activities shall follow a **common business workflow regardless of assessment type**." §7.2 lists twelve assessment types — practical, laboratory, viva voce, project demonstration, clinical, studio, jury, skill, performance, field, internship — as configurable variants of that one workflow. Splitting practical from viva reproduced a distinction the L2 explicitly collapses | `EXECUTED` |
| `I-FR-07` | I-055 ← I-056 | Bulk issuance is a volume variant, not a distinct outcome | `L2-I05` §8.3 and §20 make no single/bulk distinction; both end in `Published`/`Active` | `EXECUTED` |
| `I-FR-08` | I-058 ← I-062 | One scenario: Hall Ticket issuance after a blocking condition clears | `L2-I05` §10.1 treats these as one exception family | `EXECUTED` |
| `I-FR-09` | I-063 ← I-064 | Near-exact duplicate — a schedule change *is* a Hall Ticket-relevant context change | `L2-I05` §10.2 treats both as one exception | `EXECUTED` |
| `I-FR-10` | I-194, I-196, I-198, I-202, I-208 / mirror I-195, I-197, I-199, I-204, I-207 | **Revaluation Request to Revised Marks Outcome** and **Rechecking Request to Verified Marks Outcome** — two scenarios | `L2-I11` §8.2–8.11 is one sequential lifecycle. §4.3 justifies separating revaluation from rechecking, not splitting each into five | `EXECUTED` |
| `I-FR-11` | I-217 … I-222 | **Subsequent Examination Registration**, with fee clearance as a governed precondition | `L2-I12` §4.2 **Unified Supplementary Examination Framework**; §5.7 one Examination Registration capability; §8.4 Fee Verification is a *stage* of the single §8 lifecycle | `EXECUTED` |
| `I-FR-12` | I-266 … I-269 | One scenario: completion blocked pending an authoritative upstream examination outcome | `L2-I13` §10.1–10.4 treat these as exception variants of one completion-eligibility evaluation; all four reach the same governed state | `EXECUTED` |

Cluster `I-FR-11` should be sequenced together with the per-type triplication noted in section 7 (`I-DU-02`, `I-DU-03`, `I-DU-06`), which shares the same root cause.

---

## 6. Over-Broad Candidates — Split Review

| ID | Candidate | Recommended split | Evidence | Status |
| --- | --- | --- | --- | --- |
| `I-OB-01` | I-291 | suspend/resume · terminate/cancel · postpone/reschedule | Its Final Outcome reads "resumed, suspended, terminated, relocated, or scheduled for re-conduct" — five terminal outcomes. `L2-I15` §11.1 defines these as five separate canonical workflows (EW-09 … EW-12, with EW-08). Violates `L3_Phase1_Discovery_Rules.md` §9 ("the title combines unrelated outcomes") | `EXECUTED` |
| `I-OB-02` | I-301 | **Examination Session Closure and Finalization** · **Examination Record Archival** | `L2-I16` §2.1 separates Session Closure from Archive Management as distinct in-scope groups; §11.2 assigns SW-01…SW-06 to closure and SW-07…SW-12 to archival; §21.3 makes `Session Finalized` and `Archived` distinct states | `EXECUTED` |
| `I-OB-03` | I-290 | classification · impact assessment · contingency planning and execution | `L2-I15` EW-02 … EW-07 are six separate workflows carried by one candidate. Executed — I-290 narrowed to classification and operational impact assessment (EW-02, EW-03); new `CAN-L3-I-314` covers contingency plan preparation and emergency approval (EW-04, EW-05). Plan *execution* (EW-06) is the specific continuation action already carried by I-291, I-292, I-293 and I-309; stakeholder notification (EW-07) by I-296 | `EXECUTED` |
| `I-OB-04` | I-019 | Narrow to plan-and-approve; leave I-025, I-026, I-027 separate | I-019 claimed the full `L2-I03` §8.2–8.8 span while I-021, I-025, I-026 and I-027 separately claimed stages inside it. Executed — I-019 narrowed to planning, timetable preparation, administrative review and schedule approval (§8.2, §8.3, §8.6, §8.7), ending "ready for resource allocation, conflict validation and publication". `L3_Scenario_Type_and_Priority_Rules.md` §9 names "Timetable Publication" as a canonical P0, which confirms keeping I-025 separate | `EXECUTED` |

---

## 7. Duplication Findings

| ID | Candidates | Assessment | Status |
| --- | --- | --- | --- |
| `I-DU-01` | I-157 / I-158 | Overlapping. Conformance rewrote I-158 to the candidate-level computation defined by `L2-I09` §5.5–5.11 so it is no longer a placeholder, but it remains inside cluster `I-FR-02` | `OPEN` |
| `I-DU-02` | I-065 / I-230, I-231, I-232 | Duplicate. I-065 already covers Hall Ticket issuance for subsequent opportunities. Ownership corrected to `I05` — see section 8 row 3 | `EXECUTED` |
| `I-DU-03` | I-083, I-107 / I-233 … I-238 | Duplicate. I-083 and I-107 already cover subsequent-opportunity conduct. Ownership corrected to `I06`/`I07` | `EXECUTED` |
| `I-DU-04` | I-063 / I-064 | Near-exact duplicate — see `I-FR-09` | `EXECUTED` |
| `I-DU-05` | I-080, I-081, I-082 / I-103, I-104, I-105 / I-291, I-292, I-293 | Resolved by the `I-RC-02` ruling. I-080, I-081, I-103 and I-104 retired into I-291 and I-292; I-082 and I-105 retired into I-293. Suspension, resumption and cancellation of an interrupted examination now sit solely in the I15 block | `EXECUTED` |
| `I-DU-06` | I-145 / I-241, I-242, I-243 | Duplicate. I-145 already covers evaluation of subsequent opportunities. Ownership corrected to `I08` | `EXECUTED` |
| `I-DU-07` | I-047 / I-217 | Resolved by the `I-RC-03` ruling. `I-047` is retained as the registration outcome under owner `I04`; `I-217` re-framed to **Subsequent Examination Application, Eligibility and Attempt Authorization** under owner `I12`, ending at registration initiation. I-223, I-224 and I-225 retired — amendment, withdrawal and reinstatement of a registration are `L2-I04` lifecycle transitions already carried by I-033, I-035 and I-037 | `EXECUTED` |
| `I-DU-08` | I-016 / I-301 | Duplicate reflecting repository conflict `I-RC-01` | `BLOCKED` |
| `I-DU-09` | I-122 / I-123, I-124, I-125 | Superset/subset overlap — see `I-FR-03` | `EXECUTED` |
| `I-DU-10` | I-092 / I-121 | **Distinct — not a duplicate.** `L2-I06` and `L2-I07` are separate canonical owners with materially different conduct records: `L2-I06` §14 owns assessment batches, examiner assignment and assessment attendance; `L2-I07` §14 owns seating plans, seat assignment, invigilator management and answer book collection. A correctable error in one record has no counterpart in the other. Both retained | `CLOSED` |
| `I-DU-11` | I-155 / I-206 | **Distinct — not a duplicate.** `L2-I11` §14 "Business Capabilities Not Owned" explicitly excludes *Original Evaluation*, and owns its own Review Request Approval and answer-script retrieval. A conflict of interest at original evaluation (I08) and at revaluation (I11) are governed by different domains under different policies. Both retained | `CLOSED` |

---

## 8. Conformance Corrections Applied 2026-08-12

These were defects with a single correct resolution derivable from locked L2 sources or existing Phase 1 rules. They were applied to `L3_Phase1_Inventory_Devang.md`. They are recorded here for traceability. **No candidate was removed, merged or split.**

| # | Defect | Rows affected | Authority | Resolution applied |
| --- | --- | ---: | --- | --- |
| 1 | Seven rows were unedited template output — Trigger read `An authorized requirement triggers <title>` and Final Outcome read `The workflow <title> is successfully completed`, supplying neither a business trigger nor a business result | 6 of 7 | `L3_Phase1_Discovery_Rules.md` §5, §9 | Real triggers and final outcomes authored from the L2 source for I-158 (`L2-I09` §5.5–5.10), I-177 (`L2-I10` §5.1, §5.4, §5.6), I-290 (`L2-I15` EW-02…EW-07), I-301 (`L2-I16` SW-01…SW-08), I-303 (`L2-I16` SW-10, §21.3 `Retrieval / Restoration`), I-304 (`L2-I16` SW-09, §21.4 `Apply Retention Policy`). **I-302 was deliberately not rewritten — see section 8.1** |
| 2 | `Primary Owner` read the flat value `Examination` on all 304 rows, carrying no discriminating information and providing nothing for Phase 2 consolidation | 304 | `L3_Phase1_Discovery_Rules.md` §10 | Normalised to canonical `I01`…`I16` codes, matching the `F00`…`F14` and `G00`…`G10` conventions already in use. Derived from the owning L2 document per `ARCHITECTURE_INDEX.md` Section I |
| 3 | Twelve I12 rows asserted final outcomes that `L2-I12` §14 explicitly disclaims — Hall Ticket **Generation**, Practical and Theory **Conduct**, **Marks Collection** and **Academic Evaluation** — where `L2-I12` owns only Hall Ticket *Initiation* (§5.9), Conduct *Coordination* (§5.10) and Evaluation *Initiation* (§5.11) | 12 | `L2-I12` §14; `L2-I05` §14; `L2-I06` §14; `L2-I07` §14; `L2-I08` §14; `L3_Phase1_Discovery_Rules.md` §10 | `Primary Owner` set to the canonical owner of the final outcome: I-230/231/232 → `I05`; I-233/235/237 → `I07`; I-234/236/238 → `I06`; I-241/242/243 → `I08`. **The ownership attribution is corrected; the resulting duplication against I-065, I-083, I-107 and I-145 is a merge decision and remains open under `I-DU-02`, `I-DU-03` and `I-DU-06`** |
| 4 | I-255 claimed a CGPA outcome for the completion domain — "The candidate's **CGPA** and qualification-completion eligibility are re-evaluated" | 1 | `L2-I13` §14.3 ("shall never own … CGPA Calculation"); §2.2 (CGPA Calculation → `L2-I09`); `L2-I09` §14 | Re-worded so the candidate consumes the CGPA recomputed by I09 rather than producing it |
| 5 | Three rows carried a `Type` inconsistent with identical siblings — I-060 `Cross-domain` against I-055/I-059/I-061/I-062 `Critical` (same `C, I` domains, same aggregate, same lifecycle); I-086 against I-087; I-261 against I-260 | 3 | `L3_Scenario_Type_and_Priority_Rules.md` §7 | Aligned to the sibling value. **This is a local consistency fix only — the section-wide Type distribution remains unreviewed, see section 8.3** |
| 6 | Seven materially distinct institutional outcomes defined in locked L2 sources had no candidate | +7 added | see section 9 | Added as `CAN-L3-I-305` … `CAN-L3-I-311`, each placed in its own L2 block |
| 7 | Blocks I04–I16 carried no `**Source:**` traceability metadata, which blocks I01–I03 did carry; the file had no document header, unlike `L3_Phase1_Inventory_Chetan.md` and `L3_Phase1_Inventory_Dhruv.md` | 13 blocks | Repository convention; `L3_Phase1_Discovery_Rules.md` §3 | Source, Discovery Area and Phase metadata added to each block; document header and scope block added |

Post-conformance integrity: **311 candidates, 311 unique IDs, all rows carry the full eight-column locked format.**

### 8.1 Deliberate non-correction — I-302

`CAN-L3-I-302 Reopen Closed Examination Session for Authorized Correction` **remains an unedited placeholder row.** It was not rewritten because no supported trigger or outcome could be authored for it without inventing architecture.

```text
Status: RESOLVED 2026-08-12 — candidate removed. L2 contradiction I-RC-06 remains open.
```

**Repository Fact.** `L2-I16` §21.4 Valid State Transitions defines no transition out of `Session Finalized` back to an editable state. §9.3 Ownership Principles requires "Immutable finalized examination sessions." §21.5 Invalid State Transitions lists "Reopening a closed archive lifecycle" as prohibited.

**Repository Fact.** `L2-I16` §13 Business Rules nevertheless states: "Closed archive lifecycles shall not be reopened **unless permitted by institutional policy**."

**Repository Fact.** `L2-I02` §5.6, §8.9 and §20 permit reopening only of **suspended** sessions — already covered by `CAN-L3-I-015`.

The §13 conditional permission and the §21.5 prohibition contradict each other inside one locked document. This is recorded as `I-RC-06`.

**Resolution applied 2026-08-12.** `CAN-L3-I-302` was **removed** from the inventory. `L2-I16` defines no workflow and no state transition supporting it, and reopening a *suspended* session is already covered by `CAN-L3-I-015` under `L2-I02` §5.6. The candidate was unsupported either way.

**`I-RC-06` was not resolved.** The `L2-I16` §13 versus §21.5 contradiction is an upstream document-integrity defect and was left for separate correction — the Lead Architect ruling covered closure *ownership*, not this business rule's wording. Removing the candidate does not depend on which way §13 is settled.

### 8.2 What was deliberately not done

- **No merging or splitting** *at the time of the conformance pass*. This was subsequently authorized and executed — see section 15.
- **No candidate removed** *at the time of the conformance pass*. Superseded by section 15.
- **No `Priority` re-derivation.** See section 8.3. Still outstanding.
- **No repository conflict resolved.** See section 10.
- **No other inventory file modified.** `L3_Phase1_Inventory_Chetan.md`, `_Varad.md`, `_Pallav.md` and `_Dhruv.md` are untouched by this work.
- **No L2, L1 or governance file modified.**

### 8.3 Type and Priority remain provisional

`L3_Scenario_Type_and_Priority_Rules.md` §15 states that Phase 1 classification is provisional and that Lead Architect review may correct Type and Priority during Phase 2. Two distributions warrant that review, and **neither was corrected** — unlike Section F, Section I uses only permitted vocabulary, so this is a question of judgment about institutional risk rather than a mechanical rule violation.

| Observation | Detail | Status |
| --- | --- | --- |
| `Type` re-derived section-wide | **Executed 2026-08-12** — see section 15.3. Final distribution across 258 rows: **233 `Critical`, 18 `Cross-domain`, 7 `Simple`**. The `Critical` share stays high (90%) and that is the correct outcome, not a failure to discriminate: `L3_Scenario_Type_and_Priority_Rules.md` §6 names *official academic outcome*, *examination result*, *qualification completion*, *legal/compliance evidence* and *institutional audit trail* as Critical triggers, and Section I candidates hit at least one of these by construction. What the re-derivation extracted is the 25 genuinely operational rows — session governance, planning, scheduling, seating, venue and personnel logistics, stakeholder communication — that produce no academic outcome | `EXECUTED` |
| `P2` tier effectively unused | 42 `P0`, 215 `P1`, 1 `P2`. **Not re-derived** — priority is a sequencing judgment about institutional readiness, not a rule application, and `L3_Scenario_Type_and_Priority_Rules.md` §15 reserves it for Lead Architect review. Candidates such as I-247, I-249, I-286, I-300 and I-303 read as `P2`; Chetan applies the tier where warranted (C-045, C-047) | `OPEN` |

`P0` assignment is **sound** and needs no review: the 42 `P0` rows align with the worked examples in `L3_Scenario_Type_and_Priority_Rules.md` §9, and no candidate is `P0` merely because the domain is important.

---

## 9. Candidates Added 2026-08-12

Seven materially distinct outcomes were defined in locked L2 sources with no candidate. Adding them completes Phase 1 discovery; it does not pre-empt Phase 2 consolidation.

| ID | Title | L2 authority | Reason |
| --- | --- | --- | --- |
| `CAN-L3-I-305` | Malpractice Case Adjudication to Penalty Determination | `L2-I14` §11.1 (MW-11, MW-12, MW-13); §12 MW-12 postconditions ("Official decision finalized. Decision becomes immutable."); §21.2 `Decision Pending → Decision Recorded → Penalty Assigned`; §21.4 | **The authoritative outcome of the entire I14 domain.** I-280 stopped at "ready for an authoritative decision" and I-281 resumed after finalization. Five candidates — I-170, I-188, I-256, I-268, I-286 — consume this decision as their *trigger*; none produced it |
| `CAN-L3-I-306` | Malpractice Case Closure and Archival | `L2-I14` §11.1 (MW-17, MW-18); §21.2; §21.6 Terminal States; §21.5 ("Archive before case closure" prohibited) | The I14 aggregate had no terminal outcome. `Archived` is its sole terminal state |
| `CAN-L3-I-307` | Examination Exception Case Resolution to Closure | `L2-I15` §11.1 (EW-13, EW-14); §12 EW-13; §9.1 Owns ("Exception resolution", "Exception closure") | I-289 opened the exception case; nothing concluded it. I-299 and I-300 presupposed a finalized outcome that was never produced |
| `CAN-L3-I-308` | Retire an Examination Policy with Historical Preservation | `L2-I01` §8.6 Policy Retirement Workflow; §10 EX-005; §20 Examination Policy State Model | Distinct from I-010, which covers §8.5 supersession. Retirement is an irreversible governance transition with deferral semantics and historical-integrity obligations |
| `CAN-L3-I-309` | Postpone and Reschedule an Examination Under Authorized Contingency | `L2-I15` §11.1 (EW-08, EW-09); §12 EW-08 postconditions ("Examination postponed. Rescheduling initiated.") | I-291 listed re-conduct as one of five alternatives; I-239 covers only subsequent examinations. Regular-examination postponement was uncovered |
| `CAN-L3-I-310` | Authorized Disposal of Archived Examination Records After Retention Expiry | `L2-I16` §11.2 SW-11; §9.2 ("Disposal Authorization"); §21.3 `Retention Expired → Pending Disposal → Disposed`; §9.1 ("No other bounded context may … dispose") | The most irreversible outcome in I16 and entirely unrepresented |
| `CAN-L3-I-311` | Apply and Release Legal Hold on Examination Records | `L2-I16` §9.2 ("Legal Hold Record"); §2.1 Governance ("Legal hold support"); §21.4 `Retention Active ↔ Legal Hold`; §14.4 Retention Exceptions | A legally consequential suspension of the retention lifecycle. Distinct from retention (SW-09) and disposal (SW-11) |

### 9.1 Candidate considered and deliberately not added

**Academic Standing and Classification Determination** — `L2-I09` §5.11 produces Distinction, First Class, **Academic Probation** and **Promotion Eligibility**, and §14 lists "Academic Standing Determination" as an owned capability. Promotion Eligibility and Academic Probation are consumed outside Examination by Sections C and D.

```text
Status: OPEN — may be a distinct outcome, or a step inside the consolidated
        result-processing scenario of cluster I-FR-02
```

Not added, because the audit could not determine from repository evidence alone whether this is an independently meaningful institutional outcome or an internal step of result computation. Deciding it is Phase 2 work.

---

## 10. Repository Conflicts — Lead Architect Decision Required

These are **upstream defects in locked L2 documents**, not authoring errors. None was resolved. Both positions are preserved in each case; no compromise was invented.

| ID | Conflict | Positions | Status |
| --- | --- | --- | --- |
| `I-RC-01` | Examination Session closure ownership | **RESOLVED 2026-08-12 by Lead Architect ruling: `L2-I16` owns Examination Session closure and archival.** `L2-I02` was amended — 17 edits moving the ownership declaration, removing Session Closure from its owned lists (§2, §4, §14, §22), redirecting §5.7, §8.10, §8.11, §9.7 and §10.9 to L2-I16 as consuming context, replacing the two closure events in §11, and truncating the §20 state model at `Reopened` with the hand-off to `L2-I16` §21 recorded. An Ownership Resolution note was added to `L2-I02`. Unblocked `I-DU-08`, which was then executed | `CLOSED` |
| `I-RC-02` | Suspend/resume of an in-progress examination | **RESOLVED 2026-08-12: `L2-I15` owns the continuation decision and its execution.** The deciding evidence is `L2-I06` §10.7 and `L2-I07` §10.7 themselves, which enumerate possible outcomes and then defer — *"Institutional authorities shall govern continuation decisions"* — without defining a workflow, preconditions or postconditions, while `L2-I15` §11.1 specifies EW-09 to EW-12 in full and §9.1 reserves exception cases exclusively to that domain. `L2-I06` and `L2-I07` amended: Ownership Resolution note added, §10.7 redirected, §14 non-ownership tables extended. The conduct domains detect and report; conducting a rescheduled or re-conducted examination stays with them. Unblocked and closed `I-DU-05` | `CLOSED` |
| `I-RC-03` | Subsequent-examination registration ownership | **RESOLVED 2026-08-12: `L2-I04` owns the Examination Registration aggregate and its lifecycle for every category.** The deciding evidence is `L2-I04` §7.3, which enumerates Regular, Backlog, Supplementary, Improvement and Special as *categories of one registration mechanism*, and `L2-I04` §20, which defines a single Registration State Model governing "Every Examination Registration". `L2-I12` §5.7 language — "Approved applications are **converted into** authorised registrations" — is the same initiation verb it already uses for Hall Tickets (§5.9), evaluation (§5.11) and result processing (§5.12), and §4.4 limits it to "supplementary examination **administration**". `L2-I12` amended in six places: Ownership Resolution note, §2, §5.7 retitled to Examination Registration Initiation, §14 owned and non-owned lists, §18 catalogue, §22. Unblocked and closed `I-DU-07` | `CLOSED` |
| `I-RC-04` | `L2-I13` title contradicts §14.3 | **RESOLVED 2026-08-12 by clarification.** A note added at `L2-I13` §14 records that "CGPA" in the document title denotes CGPA **consumption** for completion verification and degree classification, not calculation, which `L2-I09` owns. The title is retained because `ARCHITECTURE_INDEX.md` and cross-references in `L2-I09`, `L2-I11` and `L2-I12` depend on it | `CLOSED` |
| ~~`I-RC-05`~~ | ~~`L2-I00` cites a non-existent document~~ | **RETRACTED 2026-08-12 — audit error, no defect exists.** `architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md` is present (11,763 bytes). The original finding came from a search run with a drifted working directory. `L2-I00`'s dependency reference resolves correctly and no change is required | `CLOSED` |
| `I-RC-06` | `L2-I16` self-contradiction on reopening | **RESOLVED 2026-08-12 by clarification.** §13 read "Closed archive lifecycles shall not be reopened unless permitted by institutional policy", contradicting §21.5 (reopening is an invalid transition) and §9.3 (immutable finalized sessions). §13 amended: reopening is prohibited outright, and policy-permitted post-closure access is served by authorised retrieval and restoration under §21.4, which does not reopen the lifecycle. `CAN-L3-I-302` had already been removed as unsupported | `CLOSED` |
| `I-RC-07` | Question-paper scope consistency in `L2-I07` | **RESOLVED 2026-08-17 as a deterministic correction to stale L2 text.** The approved 2026-08-15 scope decision already assigns Question Paper Intake, system Custody, Timed Release and Distribution to `L2-I07`, including direct Examination Controller authoring or document upload. External paper setting, moderation boards, printing and physical custody chains remain outside SutraERP. Stale disclaimers in §9.5, §23 and Appendix A were aligned to that decision without extending I07. Current candidates `CAN-L3-I-063`, `-064` and `-065` are unblocked. | `CLOSED` |

---

## 11. Cross-Inventory Findings

Targeted collision check only, per audit scope. **No cross-inventory architecture error was found on the Section I side.**

| ID | Overlap | Assessment | Status |
| --- | --- | --- | --- |
| `I-XI-01` | Chetan `C-036 Qualification Completion and Degree Conferment` vs I-250, I-258 | **Not an error.** `L2-I13` §2.2 assigns Final Qualification Completion Decision and Degree Award Authorization to `L2-C05`, and retains Completion Eligibility, Degree Award *Recommendation* and Completion Evidence for I13. Both sides respect the boundary. Valid Phase 1 discovery overlap requiring Phase 2 sequencing confirmation | `CLOSED` |
| `I-XI-02` | Chetan `C-047 Conferred Qualification Correction or Revocation` vs I-262, I-263; Chetan `C-046` vs I-266 … I-269 | Adjacent outcomes with no defined ordering. Phase 2 must establish which triggers which | `OPEN` |
| `I-XI-03` | Varad `G-543 Examination Readiness Assessment` vs I-021, I-085, I-094, I-095, I-099 | Genuine two-sided discovery. `L2-I03` §13.5 confirms Campus & Infrastructure supplies examination centres, halls, laboratories and seating capacity as a dependency. Boundary confirmation required | `OPEN` |
| `I-XI-04` | Varad Finance `F-0030`, `F-0145`, `F-0302`, `F-0132`, `F-0254`, `F-0303`, `F-0341`, `F-0436` vs I-009, I-041, I-042, I-196, I-197, I-220 … I-222 | **Not an error — boundary held correctly on both sides.** Section I candidates consistently condition on "authoritative Finance-owned clearance" without claiming the monetary transaction, matching `L2-I01` §14 ("does not own Fee Collection, Accounting, Payments") and `L2-I11` §14. This discipline should be preserved through consolidation | `CLOSED` |
| `I-XI-05` | Sections D and E have no examination-facing counterpart candidates | Section I lists `E` as a participating domain in ~15 candidates (invigilator, examiner and evaluator assignment) and `D` in ~30. `L3_Phase1_Inventory_Pallav.md` returns **zero** matches for "examination", "invigilat" or "examiner". The workforce-side outcome is unclaimed anywhere in Phase 1. **Not a Section I defect** — raise with the Section D/E discovery owner | `OPEN` |

---

## 12. Lifecycle and State Findings

| ID | Finding | Evidence | Status |
| --- | --- | --- | --- |
| `I-LS-01` | `Cancelled` is not a canonical Examination Registration state | **Resolved 2026-08-12: "cancelled" was loose wording for `Withdrawn`.** `L2-I04` §20 defines no `Cancelled` state; §5.11 covers "candidate **or institutional** withdrawal", so institutional termination needs no separate state; and `L2-I04` §19 glosses Registration Withdrawal as "Controlled registration **cancellation**", equating the two terms outright. I-036 retired into I-035; I-037 and I-048 re-worded to `Withdrawn`. I-061's trigger re-worded to registration *withdrawal* while its own `Cancelled` outcome is retained — `L2-I05` §20 does define `Cancelled` for Hall Tickets | `CLOSED` |
| `I-LS-02` | Terminal states now covered | `L2-I14` §21.6 `Archived`, `L2-I15` EW-13/EW-14 and `L2-I01` §20 `Retired` had no candidate. Closed by I-306, I-307 and I-308 | `CLOSED` |

---

## 13. Recommended Phase 2 Sequence

1. **Resolve `I-RC-01`, `I-RC-02`, `I-RC-03` and `I-RC-06`.** Four clusters and three duplication findings are `BLOCKED` behind them, and `I-RC-06` blocks the last placeholder row in the inventory.
2. **Execute the eight `EXECUTED` merges** (`I-FR-01` … `I-FR-05`, `I-FR-07` … `I-FR-12`) and the two `EXECUTED` splits (`I-OB-01`, `I-OB-02`).
3. **Decide the twelve re-owned I12 rows** — now that ownership is correct, choose between removing them in favour of I-065, I-083, I-107 and I-145, or re-scoping them to the initiation and coordination outcomes `L2-I12` actually owns.
4. **Run the Lead Architect Type and Priority review** (section 8.3) once consolidation has reduced the row count.
5. **Confirm the cross-inventory boundaries** in section 11 with the Section C, D, E and G discovery owners.
6. **Decide section 9.1** — whether academic standing and classification is a distinct outcome.

---

## 14. Open Question for the Lead Architect

Section I is the largest section in the repository and the least fragmented large section discovered so far. Twelve of its remaining defects are `BLOCKED` behind six upstream conflicts inside locked L2 documents — not behind authoring work.

**Should the `L2-I02`/`L2-I16` and `L2-I04`/`L2-I12` ownership conflicts be resolved before Phase 2 consolidation begins, or should consolidation proceed on the sections that are unblocked and revisit these afterwards?**

The audit recommends resolving them first: both conflicts sit on the session and registration aggregates, which the rest of Section I depends on, and consolidating around an unresolved ownership boundary would have to be redone.

---

## 15. Consolidation Executed 2026-08-12

Executed under explicit Lead Architect authorization, ahead of the ordinary `L3_Phase1_Discovery_Rules.md` §17 sequence. **Section I moves from 311 to 258 candidates.**

Candidate IDs were **not** renumbered. IDs are sparse after consolidation (`CAN-L3-I-001` … `CAN-L3-I-313` with gaps). `L3_Phase1_Discovery_Rules.md` §6 states candidate IDs are temporary; renumbering should happen once, at Phase 2 close, not at each consolidation step.

### 15.1 Merges executed — 11 clusters, 45 rows retired

| Cluster | Retired | Retained and rewritten |
| --- | --- | --- |
| `I-FR-01` | I-002 … I-009 | `I-001` Examination Policy Definition to Institutional Activation — domains widened to `A, B, C, D, F, I` |
| `I-FR-02` | I-158, I-174 | `I-157` Final Approved Marks to Institutionally Approved Result — domains widened to `C, D, I` |
| `I-FR-03` | I-123, I-124, I-125 | `I-122` Marks Collection for Completed Examination Activities |
| `I-FR-04` | I-127, I-128 | `I-126` Academic Evaluation of Examination Performance |
| `I-FR-05` | I-040 … I-043, I-051 | `I-029` Candidate Examination Registration to Confirmed Registration |
| `I-FR-07` | I-056 | `I-055` Hall Ticket Issuance to Eligible Registered Candidates |
| `I-FR-08` | I-062 | `I-058` Hall Ticket Issuance After a Withholding Condition Is Resolved |
| `I-FR-09` | I-064 | `I-063` Regenerate Hall Ticket After an Authoritative Examination Context Change |
| `I-FR-10` | I-196, I-198, I-202, I-208 / I-197, I-199, I-204, I-207 | `I-194` Revaluation Request to Revised Marks Outcome; `I-195` Rechecking Request to Verified Marks Outcome |
| `I-FR-11` | I-218 … I-222 | `I-217` Subsequent Examination Registration for a Supplementary, Backlog or Improvement Opportunity |
| `I-FR-12` | I-267, I-268, I-269 | `I-266` Resolve Academic Completion Block Pending an Upstream Examination Outcome |
| per-type scheduling | I-227, I-228 | `I-226` Schedule a Supplementary, Backlog or Improvement Examination |
| `I-DU-08` (unblocked by `I-RC-01`) | I-016 | `I-301` — closure now sits solely in the L2-I16 block |

### 15.2 Splits, removals and additions

| Action | Detail |
| --- | --- |
| `I-OB-01` split | `I-291` **narrowed** to the EW-10 suspension outcome. It was not split into new rows because its other four outcomes already had candidates — resume `I-292`, terminate `I-293`, re-conduct `I-294`/`I-295`, postpone/reschedule `I-309`, relocate `I-078`/`I-099`. **`I-291` remains inside the `I-RC-02` blocked set** |
| `I-OB-02` split | `I-301` narrowed to **Examination Session Closure and Finalization** (SW-01…SW-06); new `CAN-L3-I-312` **Examination Record Archival** (SW-07…SW-08) added |
| `UN-01` removal | `CAN-L3-I-302` removed as unsupported — see section 8.1 |
| `OW-01` removal | I-230 … I-238 and I-241 … I-243 removed (12 rows). Their outcomes are carried by `I-065`, `I-083`, `I-107` and `I-145` under their canonical owners. New `CAN-L3-I-313` **Subsequent Examination Execution Coordination** added for the initiation and coordination outcomes `L2-I12` §5.9–5.12 actually owns |
| `MS-08` | **Not added as a separate candidate.** Folded into `I-157`, whose consolidated outcome now names classification and academic standing explicitly and whose domains include `C, D` for the progression hand-off. Adding a separate row after `I-FR-02` would have duplicated it. Closes section 9.1 |

### 15.3 Type re-derived section-wide

Applied `L3_Scenario_Type_and_Priority_Rules.md` §7 selection order to all 258 rows: high institutional risk per §6 → `Critical`; else two or more participating business domains → `Cross-domain`; else `Simple`.

| Type | Count | Share |
| --- | ---: | ---: |
| `Critical` | 233 | 90.3% |
| `Cross-domain` | 18 | 7.0% |
| `Simple` | 7 | 2.7% |

`Simple` — `I-013`, `I-014`, `I-015`, `I-017`, `I-018` (session governance transitions), `I-026`, `I-027` (scheduling conflict and seating plan).

`Cross-domain` — `I-011`, `I-012`, `I-019` … `I-025`, `I-028`, `I-085`, `I-086`, `I-098`, `I-101`, `I-226`, `I-229`, `I-296`, `I-297`.

The high `Critical` share is the correct result, not a failure to discriminate. §6 names *official academic outcome*, *examination result*, *qualification completion*, *legal/compliance evidence* and *institutional audit trail* as Critical triggers, and Section I candidates hit at least one by construction. The re-derivation extracted the 25 rows that produce no academic outcome: session governance, planning, scheduling, seating, venue and personnel logistics, and stakeholder communication.

**Type remains provisional** under `L3_Scenario_Type_and_Priority_Rules.md` §15 and is not locked by this consolidation.

### 15.4 What was still not done

- **`Priority` was not re-derived.** 42 `P0`, 215 `P1`, 1 `P2`. See section 8.3.
- **`I-FR-06`, `I-OB-03`, `I-OB-04`, `I-DU-10`, `I-DU-11`, `I-LS-01`, `I-XI-02`, `I-XI-03`, `I-XI-05`** remain `OPEN` — each needs a judgment repository evidence does not settle.
- **`I-RC-02`, `I-RC-03`, `I-RC-06`** remain unresolved, and `I-DU-05`, `I-DU-07` remain `BLOCKED` behind them.
- **`I-RC-03` is the significant one.** It was raised for ruling alongside `I-RC-01` and was not answered. Until it is, `L2-I04` and `L2-I12` both claim supplementary, backlog and improvement registration, and `I-047` versus the consolidated `I-217` cannot be resolved.
- **No candidate renumbering.** See the note at the head of this section.

### 15.5 Files changed

| File | Change |
| --- | --- |
| `architecture/L3_Business_Scenario_Architecture/01_Phase1_Discovery/L3_Phase1_Inventory_Devang.md` | 311 → 258 candidates; 15 rows rewritten; 21 rows retyped; 2 rows added |
| `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I02_Examination_Session_Management_Workflows.md` | 17 edits implementing the `I-RC-01` ruling. **The only L2 file changed** |
| `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_I_Phase2_Consolidation_Watchlist.md` | this document |
| `generated/reviews/L3_Phase1_Audit_Devang_ALL_ASSIGNED.md` | re-rating recorded in Part III |

No other inventory, no other L2 document, and no governance standard was modified.

---

## 16. Round 2 — Conflicts Resolved and Remaining Items Closed, 2026-08-12

Executed under Lead Architect direction to resolve duplication, fragmentation, boundary quality, ownership and `I-RC-03` immediately. **Section I moves from 258 to 245 candidates.**

### 16.1 Ownership rulings issued

| Conflict | Ruling | L2 amended |
| --- | --- | --- |
| `I-RC-02` | `L2-I15` owns suspension, resumption, rescheduling and cancellation of an interrupted in-progress examination. The conduct domains detect and report | `L2-I06`, `L2-I07` — Ownership Resolution note, §10.7 redirect, §14 non-ownership row |
| `I-RC-03` | `L2-I04` owns the Examination Registration aggregate and lifecycle for **every** category. `L2-I12` owns application, eligibility, fee verification, session allocation and attempt lifecycle, and **initiates** registration in `L2-I04` | `L2-I12` — Ownership Resolution note, §2, §5.7 retitled, §14 owned and non-owned lists, §18 catalogue, §22 |
| `I-RC-04` | "CGPA" in the `L2-I13` title denotes consumption, not calculation | `L2-I13` — clarifying note at §14 |
| `I-RC-06` | Reopening a closed archive lifecycle is prohibited outright; policy-permitted post-closure access is served by retrieval and restoration under §21.4 | `L2-I16` — §13 rule re-worded |

Both rulings follow the same discriminator the `OW-01` correction used: where one document *specifies* a workflow with preconditions and postconditions and another merely *enumerates outcomes and defers*, the specifying document owns it.

### 16.2 Retraction

`I-RC-05` — "L2-I00 cites a non-existent L2 Domain Workflow Matrix" — is **withdrawn as an audit error**. `architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md` exists (11,763 bytes). The original finding came from a search run with a drifted shell working directory. No defect, no change required. Also corrected in the audit report §3.

### 16.3 New conflict found

At the time of this 2026-08-12 round, `I-RC-07` was recorded as an unowned question-paper capability affecting then-current candidates `CAN-L3-I-111`, `-112` and `-113`. **Superseded 2026-08-17:** the approved 2026-08-15 scope decision assigns system intake, custody, timed release and distribution to `L2-I07`; the remaining contradiction was stale text and is now corrected. Current candidates `CAN-L3-I-063`, `-064` and `-065` are unblocked.

### 16.4 Consolidation executed

| Action | Rows | Basis |
| --- | ---: | --- |
| `I-DU-05` — I-080, I-081, I-103, I-104 retired into I-291 / I-292; I-082, I-105 into I-293 | −6 | `I-RC-02` ruling |
| `I-DU-07` — I-223, I-224, I-225 retired into I-033 / I-035 / I-037; I-047 and I-217 re-scoped | −3 | `I-RC-03` ruling |
| `I-FR-06` / `I-LS-01` — I-036 retired into I-035; I-037, I-048, I-061 re-worded | −1 | `L2-I04` §19, §20, §5.11 |
| `I-FR-13` — I-070, I-071 retired into I-069; I-073 into I-072; I-077 into I-076 | −4 | `L2-I06` §4.3 Unified Assessment Conduct Framework |
| `I-OB-03` — I-290 narrowed; new `CAN-L3-I-314` Prepare and Authorize an Examination Contingency Plan | +1 | `L2-I15` EW-02 … EW-05 |
| `I-OB-04` — I-019 narrowed to plan-and-approve | 0 | `L2-I03` §8.2, §8.3, §8.6, §8.7 |
| `I-DU-10`, `I-DU-11` — examined and **closed as distinct**, both retained | 0 | `L2-I06`/`L2-I07` §14; `L2-I11` §14 |

Final: **245 candidates · 245 unique IDs · 0 placeholders · 0 unsupported candidates · Type 220 Critical / 18 Cross-domain / 7 Simple.**

### 16.5 What is still open

| Item | Status | Why |
| --- | --- | --- |
| `I-RC-07` question-paper scope consistency | `CLOSED 2026-08-17` | Approved 2026-08-15 scope applied consistently to `L2-I07`; no new owner or capability introduced |
| `Priority` re-derivation | `OPEN` | 41 `P0`, 203 `P1`, 1 `P2`. A readiness judgment, not a rule application; `L3_Scenario_Type_and_Priority_Rules.md` §15 reserves it for Lead Architect review |
| `I-XI-02`, `I-XI-03`, `I-XI-05` | `OPEN` | Cross-inventory boundaries with Sections C, G and D/E. Need the counterpart discovery owners, not a Section I decision |
| Candidate renumbering | `OPEN` | IDs sparse (`I-001` … `I-314` with gaps). Deferred to Phase 2 close per `L3_Phase1_Discovery_Rules.md` §6 |

**Every duplication, fragmentation, boundary-quality and ownership item raised by the audit is now closed.** `I-RC-07`, the last item that remained open after the 2026-08-12 round, was closed on 2026-08-17 by aligning stale `L2-I07` text to the approved 2026-08-15 scope decision.

---

**End of Watchlist**


---

# 2026-08-15 Audit Application: I-TV Cluster Rulings and Title Conformance

Applied under Lead Architect approval. Record:
`generated/reviews/L3_Phase1_Tranche3_Application_Report_2026-08-15.md`.
Section I: 218 -> 197 candidates. Retired IDs must not be reused.

| Cluster | Ruling | Retired | Retained |
|---|---|---|---|
| `I-TV-01` Hall ticket variants | **MERGE** into `I-055`. Terminal outcome is identical; differing eligibility authority is upstream input, not a distinct outcome | `I-058`, `I-059`, `I-060`, `I-063`, `I-065` | `I-066` — an approved special arrangement changes the Hall Ticket's own content obligation, not just its trigger |
| `I-TV-02` Result publication | **KEEP SEPARATE.** Discovery Rules section 6 and section 10 name regular publication and revised result as distinct worked examples; withhold and release carry real candidate consequence | none | all |
| `I-TV-03` Conduct variants | **MERGE logistics antecedents** into `I-093` (theory) and `I-069` (non-theory); merge accommodation variants into `I-108` | `I-094`, `I-099`, `I-100`, `I-102`, `I-107`, `I-109`, `I-110`, `I-076`, `I-078`, `I-079`, `I-083`, `I-087` | `I-095` multi-centre, `I-106` and `I-084` re-conduct decisions, `I-085` and `I-086` contingency resolutions |
| `I-TV-04` Marks validation | **SPLIT.** Merge validity-failure branches into `I-133`; retain rows carrying independent policy authority | `I-134`, `I-135`, `I-136`, `I-137` | `I-151`, `I-152`, `I-153` |

**Identical-outcome pairs — differentiated, not merged.** `I-074`/`I-096` and `I-092`/`I-121`
are the practical and theory forms of the same situation, owned by `L2-I06` and `L2-I07`
respectively. Section I mirrors the two conduct modes deliberately throughout. The duplication
was in the Final Outcome *wording*, not in the scenarios; both pairs were re-worded to name
their conduct mode. The `I-079`/`I-102` pair was resolved by the `I-TV-03` merge.

**Title conformance.** 146 Section I titles were converted from imperative action-fragment form
("Resolve X", "Conduct X") to the canonical `<condition> to <outcome>` form used across the rest
of the repository, per Discovery Rules section 8. Every replacement was derived from the row's
own recorded Final Outcome; no outcome, state, trigger or business rule was changed.

**Still open for Phase 2.** `I-139`, `I-148`, `I-149`, `I-150` and `I-156` sit in the `I-TV-04`
range but were outside the approved merge scope and remain undecided. Type and Priority for
Section I were deliberately left unchanged — see the application report.

---

## ID Renumbering — 2026-08-17 (disjoint-start; superseded later the same day, see below)

**Authorization:** explicit user request to apply consistent, continuous numbering to all Phase 1
discovery sections, with explicit override of the "Phase 1 Candidate IDs must never be renumbered" rule.
Same treatment already applied to Section G — see `L3_Section_G_Phase2_Consolidation_Watchlist.md` §14 for
the full rationale (collision avoidance, why historical narrative stays frozen).

**What changed.** The 197 active Section I candidates (previously sparse across `CAN-L3-I-001`…`CAN-L3-I-313`)
were renumbered to a single continuous block **`CAN-L3-I-315`…`CAN-L3-I-511`**, in the same order they
already held in `L3_Phase1_Inventory_Devang.md`. Applied to the Phase 1 inventory, the Phase 2 register,
and the repository-wide review.

**All content above this section is left exactly as originally written**, using the pre-2026-08-17 numbering
— it documents decisions made under the ID scheme that existed at the time, and many of those passages
describe a range of IDs (e.g. `I-217…I-222`) where only some members are still active; mechanically
renumbering a shared endpoint would corrupt the range. Resolve any old ID encountered above through the
`L3_Phase1_Candidate_ID_Migration_2026-08-17.md` crosswalk (for `315`–`511` IDs) or through this section
(for pre-`315` IDs).

**Why the range started at 315, not 1.** Section I's historical maximum ever allocated is `CAN-L3-I-314`
(prior to the 2026-08-15 `I-TV-01`…`I-TV-04` consolidation). Starting the new continuous range at 315 made
it provably disjoint from every ID ever allocated in Section I, so no historical reference anywhere could
become ambiguous.

### Superseded — Repository-Wide `001`-Start Renumbering — 2026-08-17

Later the same day, the repository-wide `001`-start migration (`L3_Phase1_Candidate_ID_Migration_2026-08-17.md`)
renumbered every section, including this one, a second time: the 197 active Section I candidates (disjoint
range `CAN-L3-I-315`…`CAN-L3-I-511` above) became the continuous block **`CAN-L3-I-001`…`CAN-L3-I-197`**,
in the same order, which is the range currently active in `L3_Phase1_Inventory_Devang.md`. This resolves the
same collision concern described above through an explicit crosswalk document instead of a disjoint numeric
offset. Full per-candidate old→new pairs (both the `315`-start disjoint IDs and their final `001`-start IDs)
are recorded in `L3_Phase1_Candidate_ID_Migration_2026-08-17.md` and, per-row, in the Phase 2 register's
Section I "Phase 1 ID" column.

## 17. Wave 2 Cross-Domain Boundary Resolution — 2026-08-17

This is the current-state disposition of the two Wave 2 Section I cross-inventory findings. It supersedes the `OPEN` status in section 11 without rewriting that point-in-time audit narrative.

### 17.1 I-XI-03 — RESOLVED

**Classification:** `STALE_L3_BLOCKER`

**Affected current candidates:** `CAN-L3-G-002`, `CAN-L3-I-009`, `CAN-L3-I-048`, `CAN-L3-I-055`, `CAN-L3-I-056`.

Locked upstream architecture separates the outcomes. `L2-G10` section 9A.249 and sections 44A.177-199 make G the authoritative owner and publisher of physical examination facilities, capacity, accessibility, lifecycle, and operational availability. `L2-I03` owns Examination venue planning and allocation using those facts. `L2-I06` owns examination-specific practical preparation and alternative arrangements after a laboratory, equipment, or venue problem. `L2-I07` consumes the existing `Examination Centre Available` fact, explicitly keeps infrastructure ownership external, and owns examination-centre operations, hall preparation, seating, and theory-examination conduct.

Accordingly, `CAN-L3-I-009`, `CAN-L3-I-048`, `CAN-L3-I-055`, and `CAN-L3-I-056` are `KEEP`. Their outcomes are Examination decisions or conduct outcomes, not physical-infrastructure lifecycle outcomes. The G-side `CAN-L3-G-002` is `VARIANT` into `CAN-L3-G-001` under the already-evidenced SG-01 consolidation because `L2-G00` defines Examination Readiness as a configurable type inside the single Campus Readiness Workflow. This G00 consolidation does not transfer any Examination decision to G.

### 17.2 I-XI-02 — RESOLVED BY LEAD ARCHITECT DECISION

**Approval:** the recommended origin-sensitive hand-off was approved on 2026-08-17.

**Classification:** `DETERMINISTIC_FIX`

**Affected current candidates:** `CAN-L3-C-050`, `CAN-L3-I-162`, `CAN-L3-I-163`.

The approved ordering is:

1. **Examination-originating change:** I13 consumes the authoritative revised Examination fact, first withdraws, restores, or re-establishes its own completion evidence as required, and publishes the superseding current evidence position through the existing `ExaminationCompletionEvidencePrepared` event. C05 then alone decides whether the qualification requires correction, revocation, or no change.
2. **Non-Examination qualification-governance change:** C05 first makes the qualification decision. A qualification-only correction does not alter I13 evidence unless an Examination outcome also changes. Where an approved revocation requires evidence review, C05 publishes the existing `QualificationAwardRevoked` event; I13 reviews and dispositions only its own evidence, without reinterpreting the qualification decision.

`L2-C05` and `L2-I13` now record this decision. The hand-off reuses `ExaminationResultPublished`, `RevaluationCompleted`, `ExaminationCompletionEvidencePrepared`, and `QualificationAwardRevoked`; no new event or owner was introduced.

The three candidates produce distinct institutional outcomes owned by different domains and are all `KEEP`. Qualification ownership remains exclusively with C05, and completion-evidence ownership remains exclusively with I13. Phase 1 remains frozen and unchanged.

---

## 18. Wave 3 I-TV-04 Residual Resolution — 2026-08-17

The five residual rows are resolved without reopening the approved 2026-08-15 `I-TV-04` rulings or changing `CAN-L3-I-084` through `CAN-L3-I-086`.

`L2-I08` sections 4.7, 8, 17 and 20 establish one authoritative domain result: Final Approved Marks. Its sections 10.1-10.7 treat missing/compromised evidence, discrepancy, validation failure, evaluator unavailability and continuity as exception or correction handling inside that lifecycle; they define no separate terminal business fact for the five causes.

| Candidate | Disposition | Target | Evidence-based reason |
| --- | --- | --- | --- |
| `CAN-L3-I-076` | `MERGE` | `CAN-L3-I-072` | A validation discrepancy returns for traceable pre-approval correction; the parent already spans correction through Final Approved Marks. |
| `CAN-L3-I-081` | `MERGE` | `CAN-L3-I-074` | An unavailable script is a missing-evidence cause that pauses evaluation and converges on authoritative resolution or routing. |
| `CAN-L3-I-082` | `MERGE` | `CAN-L3-I-074` | Compromised evidence suspends evaluation for institutional determination; damage does not create a different downstream fact. |
| `CAN-L3-I-083` | `MERGE` | `CAN-L3-I-074` | Script association is an integrity/validation correction before an authoritative exception outcome, not a separate lifecycle. |
| `CAN-L3-I-087` | `MERGE` | `CAN-L3-I-072` | Resumption or authorized reassignment preserves the continuing evaluation; the parent already includes governed reassignment and reaches Final Approved Marks. |

The retained policy-required second and adjudicating evaluation outcomes remain unchanged because they exercise independent policy authority rather than merely describing exception causes.
