# L3 Phase 1 Discovery Audit — All Sections

**Report type:** Independent adversarial audit of L3 Phase 1 candidate scenario discovery
**Audit scope:** `ALL` — Sections A–J, all five Phase 1 inventories
**Date:** 2026-08-15
**Mode:** Read-only. No architecture, governance or inventory file was modified. This report is the only file written.
**Repository state audited:** working tree at `F:/Pravaha/ERP_Planning`, branch `dhruv`

---

## 1. Audit Metadata

| Property | Value |
| --- | --- |
| Audit scope | ALL |
| Team member | N/A (all) |
| Section | N/A (all) |
| Inventories audited | 5 |
| Candidates checked | 1,123 |
| L2 documents in coverage map | 107 |
| Prior audits accounted for | `l3-phase1-audit-all-2026-08-14.md`, `l3-phase1-audit-all-2026-08-14-2.md`, `L3_Phase1_Tranche3_Application_Report_2026-08-15.md`, nine section watchlists |
| Conformance checker | `python check_l3_inventory.py` — PASS, 1,123 candidates, 0 errors, 0 warnings |

This is a re-audit. Two prior all-scope audits and their corrections have been applied. Findings already recorded and preserved in a section watchlist are **not** re-raised as inventory defects; they are counted only where this audit found the recorded principle was not carried through.

---

## 2. Scope Actually Audited

| Section | Discovery owner | Candidates | L2 docs | Inventory file |
| --- | --- | ---: | ---: | --- |
| A — Platform Foundation | Dhruv | 62 | 11 | `L3_Phase1_Inventory_Dhruv.md` |
| B — Institution & Structure | Pallav | 32 | 6 | `L3_Phase1_Inventory_Pallav.md` |
| C — Student Lifecycle | Chetan | 74 | 9 | `L3_Phase1_Inventory_Chetan.md` |
| D — Academic Delivery | Pallav | 57 | 8 | `L3_Phase1_Inventory_Pallav.md` |
| E — People & Workforce | Pallav | 75 | 8 | `L3_Phase1_Inventory_Pallav.md` |
| F — Finance | Varad | 85 | 15 | `L3_Phase1_Inventory_Varad.md` |
| G — Campus Operations | Varad | 350 | 11 | `L3_Phase1_Inventory_Varad.md` |
| H — Institutional Services & Intelligence | Varad | 81 | 11 | `L3_Phase1_Inventory_Varad.md` |
| I — Examination | Devang | 197 | 17 | `L3_Phase1_Inventory_Devang.md` |
| J — Enterprise Resource Management | Dhruv | 110 | 11 | `L3_Phase1_Inventory_Dhruv.md` |
| **Total** | | **1,123** | **107** | 5 files |

**Distribution split verified against governance.** `L3_Phase1_Workload_Distribution_Decision.md` §2 matches the split given in the audit instruction exactly. No cross-domain `X`-area candidates exist in any inventory; `X` remains unused, which is consistent with `L3_Naming_Numbering_Standard.md` §5 (use `X` only when the primary owner cannot be identified).

---

## 3. Sources Consulted

**Governance:** `L3_Phase1_Discovery_Rules.md`; `L3_Scenario_Type_and_Priority_Rules.md`; `L3_Naming_Numbering_Standard.md`; `L3_Phase1_Workload_Distribution_Decision.md`; nine `L3_Section_*_Phase2_Consolidation_Watchlist.md` files.

**L2 sources opened for targeted verification:** `L2-G08`, `L2-J10`, `L2-C05`, `L2-C07`, `L2-I14`, `L2-F11`, `L2-H10`, `L2-E06`, `L2-E07`.

**Repository infrastructure:** `check_l3_inventory.py`; `DEPENDENCY_MAP.md` (via watchlist citation).

**Prior generated reviews:** consulted only to establish what had already been found and applied, never as evidence for a material finding.

---

## 4. Executive Verdict

The inventory set is in materially better condition than its size suggests. Candidate IDs are unique and conformant, no implementation architecture has leaked into any row, no fabricated state or event was found, and the repository's conflict-preservation discipline is genuinely strong — Sections C, D and I record upstream contradictions rather than resolving them, which is the correct Phase 1 behaviour.

Three problems prevent a clean pass.

**First, consolidation principles are still being applied cluster-by-cluster rather than swept.** The Section G watchlist named this exact failure mode on 2026-08-15 ("Recording a principle as `APPLIED` should in future require a section-wide sweep for the construct"). This audit found the same failure again, in three new places: two intact step-chains in G08 Housekeeping that no cluster reached, a six-row Exit-Type enumeration in Section C, and a six-row disposal-method enumeration in Section J. All three are the same construct class as clusters already recorded `APPLIED` elsewhere.

**Second, Priority is not comparable across inventories.** P0 density ranges from 12% (Section C) to 65% (Section F). Campus Operations carries 44% P0 while Examination carries 14%. P0 exists to cut one first release; a P0 flag that means different things in different files cannot perform that function, and Phase 2 consolidation is the last point at which this can be fixed cheaply.

**Third, two proven coverage gaps.** `L2-H10` §4.4 names seven supported business scenarios; two of them — Student Mobility and Faculty Collaboration — have zero representation across all 1,123 candidates in all five inventories.

Two genuine repository blockers remain, both of the same class (a capability that locked L2 documents depend on but no document owns): question paper governance (`I-RC-07`) and Policy Management (`CF-08`). Both are correctly preserved by the inventories and are repository defects, not discovery defects.

**Overall score: 8.9 / 10.0. Verdict: NOT READY — MATERIAL CORRECTIONS REQUIRED.**

---

## 5. Section Scorecard

| Section | Coverage 2.5 | Granularity 2.0 | Ownership/Domains 2.0 | Fields incl. Type/Priority 1.5 | Conflict handling 1.0 | Governance 1.0 | **Total** |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| A | 2.4 | 1.9 | 1.9 | 1.3 | 1.0 | 1.0 | **9.5** |
| B | 2.3 | 1.8 | 1.9 | 1.2 | 1.0 | 1.0 | **9.2** |
| C | 2.3 | 1.4 | 1.9 | 1.4 | 1.0 | 1.0 | **9.0** |
| D | 2.3 | 1.8 | 1.8 | 1.4 | 1.0 | 1.0 | **9.3** |
| E | 2.3 | 1.8 | 1.4 | 1.2 | 1.0 | 1.0 | **8.7** |
| F | 2.1 | 1.9 | 1.9 | 0.9 | 1.0 | 1.0 | **8.8** |
| G | 2.4 | 1.1 | 1.5 | 1.1 | 1.0 | 1.0 | **8.1** |
| H | 1.8 | 1.9 | 2.0 | 1.2 | 1.0 | 1.0 | **8.9** |
| I | 2.4 | 1.5 | 1.9 | 1.0 | 1.0 | 0.9 | **8.7** |
| J | 2.4 | 1.5 | 2.0 | 1.2 | 1.0 | 1.0 | **9.1** |

No section was scored on candidate count. Sections C, D and I were not penalised for upstream repository conflicts they correctly preserved.

---

## 6. Findings Register

---

### FIND-01 — G08 Housekeeping: two complete L2 workflows still rendered as consecutive-step candidates

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | G — `L2-G08_Housekeeping_Operations_and_Service_Governance_Workflows.md` |
| **Affected candidates** | Chain 1: `G-310`, `G-311`, `G-312`, `G-313`, `G-315`, `G-336`. Chain 2: `G-321`, `G-323`, `G-324`, `G-326` |
| **Disposition** | MERGE (two clusters) |
| **Evidence class** | PROVEN |
| **Confidence** | HIGH |

**Problem.** `L2-G08` defines the Scheduled Cleaning Workflow as one sequence and the Complaint Workflow as one sequence. Each has been converted step-for-step into standalone candidates. Four of the ten rows have Final Outcomes that are readiness states, not institutional results:

- `G-311` — "The schedule becomes active and is **available for operational task generation**"
- `G-312` — "**An executable cleaning task is generated**"
- `G-313` — "An authorised housekeeping operator or team is assigned and **the task becomes ready for execution**"
- `G-323` — "**Priority is assigned** according to configured criticality…"

**Evidence.**
1. `architecture/L2_Domain_Workflow_Architecture/section-g/L2-G08_…Workflows.md:982` — *Scheduled Cleaning Workflow*: `Cleaning Schedule → Task Generation → Operational Assignment → Cleaning Performed → Quality Inspection → Completed`. One workflow, six steps. The inventory carries six candidates over the same span.
2. `…L2-G08_…Workflows.md:1085` — *Complaint Workflow*: `Complaint Registered → Validation → Priority Assignment → Operational Assignment → Cleaning Completed → Inspection → Closure`. One workflow. The inventory carries four candidates over the same span.
3. `L3_Phase1_Discovery_Rules.md` §8 disqualifies "Approve Request", "Update Student Status" and "Check Eligibility" as standalone candidates — `G-311`, `G-312`, `G-313` and `G-323` are the same construct.

**Repository impact.** These are the identical construct class as `G-FR-04` (G07 single-step movement, 26 rows absorbed) and `G-FR-13` (G01 work-order internal steps), both recorded `APPLIED`. Neither sweep reached G08's two core operational workflows. Ten rows currently mis-state four workflow steps as institutional outcomes; Phase 2 would otherwise author four L3 documents that describe no institutional result.

**Recommendation.** Two merge clusters — see MG-01 and MG-02 in section 8. Then execute a section-wide sweep for the construct across all eleven G sub-areas before closing Phase 1, as the Section G watchlist §10 root-cause note already requires.

---

### FIND-02 — Section C: six candidates enumerate one configurable Exit Type attribute

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | C — `L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md` |
| **Affected candidates** | `C-039`, `C-040`, `C-041`, `C-042`, `C-044`, `C-045` |
| **Disposition** | MERGE |
| **Evidence class** | PROVEN |
| **Confidence** | HIGH |

**Problem.** Six candidates share one trigger form ("an authorised exit of type X is established"), one Final Outcome form ("Student Context reaches the *X* terminal outcome and the institutional academic relationship is concluded with history preserved"), one owner, and — for five of the six — identical Domains `C, F, G`. They differ only in which value of the Exit Type attribute they name.

**Evidence.**
1. `…/section-c/L2-C05_…Workflows.md:615` — "**Exit Type** — The classification describing how a student's institutional relationship concludes." Enumerated values: Qualification Completion, Withdrawal, Transfer, Discontinuation, Administrative Exit, Termination, Deceased. This is an attribute definition, not a set of workflows.
2. `…L2-C05_…Workflows.md:1697` — "Exit types shall support **institution-defined extensions** while preserving canonical exit semantics." A candidate set built one-row-per-value cannot be stable: an institution adding an exit type would require a new L3 scenario document.
3. `…L2-C05_…Workflows.md:1253–1257` — the state machine routes `Withdrawal`, `Discontinuation` and `Termination` all to the single terminal state `StudentExitCompleted`.

**Repository impact.** This is the same construct class as `G-FR-10` (six G10 rows enumerating attribute classes, retired) and `J-TV-01` (see FIND-03). Corroborating signal of mechanical generation: `C-045` (Deceased) is `P2` while its five siblings are `P1`, despite being the same enumerated family.

**Recommendation.** Merge to one candidate — *Authorised Student Exit to Concluded Institutional Relationship* — carrying Exit Type as a governed variant, with `C-043` (Exit Clearance) retained separately as a genuine precondition outcome and `C-036`/`C-037` retained as the completion pathway. If Phase 2 judges Termination materially distinct on disciplinary-evidence grounds, retain that one and merge the remaining five; the current six-way split is not supportable.

---

### FIND-03 — Section J: six candidates enumerate one configurable Disposal Method; `J-TV-01` is now decidable

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | J — `L2-J10_Disposal_Scrap_and_Write_Off.md` |
| **Affected candidates** | `J-117`, `J-118`, `J-119`, `J-120`, `J-121`, `J-122` against parent `J-115` |
| **Disposition** | MERGE |
| **Evidence class** | PROVEN |
| **Confidence** | HIGH |

**Problem.** The Section J watchlist records this as `J-TV-01` `OPEN` and sets an explicit test: *"Do not merge without checking `L2-J10` for distinct approval authorities."* That test has now been run against the source and it resolves **against** retaining the split.

**Evidence.**
1. `…/section-j/L2-J10_Disposal_Scrap_and_Write_Off.md:677–693` — §6.8 *Disposal Method* — "Representative methods include: Scrap, Destruction, Sale, Auction, Donation, External Transfer, Authorized Recycling, Return to Authorized External Party, Other… **The permitted methods shall be governed by institutional configuration and policy.**" A configurable, extensible attribute.
2. `…L2-J10…:1877` — "Approval authority may depend on materiality, category, quantity, **disposal method**, asset class, or institutional policy." Disposal method is *one of six* inputs to a configurable authority rule, alongside materiality and quantity — neither of which anyone proposes as a scenario boundary. This directly answers the watchlist's stated test.
3. `…L2-J10…:2096` and `:2163` — "Sale proceeds or accounting consequences **remain owned by Finance**"; "J10 **shall not** post sale proceeds." The strongest argument for retaining Sale/Auction separately — that it produces revenue — describes a Finance-owned outcome, already carried by `F-071`. Within J's ownership, all six methods reach the same result.
4. `…L2-J10…:755–775` — §6.12 defines a single Execution sequence (Physical Verification → Authorized Handover/Destruction → Witness Confirmation → Supporting Evidence → Execution Confirmation) applying to all methods.

**Repository impact.** Corroborating signal of mechanical generation: the Domains field is internally inconsistent across the family — `J-118`, `J-119`, `J-122` list `F, J` while `J-117`, `J-120`, `J-121` list `J` only, though every method produces the same class of financial consequence.

**Recommendation.** Merge the six method rows into `J-115`, restated as *Authorised Disposal Case to Reconciled Physical Disposal Closure* with Disposal Method as a governed variant. Retain `J-116` (partial disposal), `J-123` (loss write-off) and `J-126` (emergency safety disposal) — these differ by trigger and authority, not by method. Update `J-TV-01` from `OPEN` to `DETERMINED`.

---

### FIND-04 — Section F: two of six governed classification tiers unused across the entire domain

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | F — all 15 L2 documents |
| **Affected candidates** | All 85 |
| **Disposition** | RECLASSIFY |
| **Evidence class** | PROVEN |
| **Confidence** | HIGH |

**Problem.** Section F carries **79 of 85 `Critical`, 0 `Simple`, 0 `P2`, and 55 of 85 `P0` (65%)**. Two of the six governed values are unused across an 85-row, 15-document domain.

**Evidence.**
1. `L3_Scenario_Type_and_Priority_Rules.md` §14 Mistake 2 names this pattern verbatim: "All Finance scenarios = Critical" is listed as incorrect; "classification depends on the specific business outcome."
2. §10 lists **"Refund Processing"** as a canonical `P1` example. `F-014`, `F-015`, `F-017` and `F-018` are all `P0`.
3. §11 lists "Rare Historical Financial Correction" as a canonical `P2` example. `F-019` (Refund Closure to Historical Reconstruction), `F-032` (Closed Ledger Evidence to Auditable Financial History), `F-055` (Closed Financial Period to Governed Archival) and `F-079` (Financial Evidence to Audit and Historical Reconstruction) all sit at `P1` or higher with no `P2` anywhere in the section.

**Repository impact.** Recorded as `OPEN` in `L3_Section_F_Phase2_Consolidation_Watchlist.md` §13 on 2026-08-14 and explicitly **not** re-derived. Because the inventory records rather than hides the problem, Section F's conflict-handling score is not reduced — but the defect is unchanged in the inventory and Type/Priority cannot be used to sequence L3 authoring for Finance.

**Recommendation.** Re-derive Section F Type against §7 and Priority against §12 before Phase 2 sequencing. This is a Lead Architect judgment per §15, not a mechanical fix.

---

### FIND-05 — Priority is not comparable across sections; P0 cannot cut a first release

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | Cross-section — all ten |
| **Affected candidates** | All 1,123 |
| **Disposition** | RECLASSIFY (governance action) |
| **Evidence class** | PROVEN |
| **Confidence** | HIGH |

**Problem.** P0 density by section:

| Section | C | I | A | D | J | B | E | G | H | F |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| % P0 | 12% | 14% | 23% | 23% | 32% | 34% | 41% | 44% | 54% | 65% |

Campus Operations (G) carries a P0 share three times that of Examination (I), and Finance (F) five times. `L3_Scenario_Type_and_Priority_Rules.md` §9 defines P0 as "mandatory for the first L3 release" — a single institutional cut line. Under the current data, 655 candidates are P0 or P1 in one section's calibration and would be P1 or P2 in another's.

**Evidence.** Computed from all 1,123 rows across the five inventories (Priority field, locked eight-field format). Section watchlists confirm the tiers were re-counted per section but never calibrated against each other: `L3_Section_G…Watchlist.md` `G-OI-02` (`APPLIED`, G-local), `L3_Section_I…Watchlist.md` §8.3 (`OPEN`, explicitly deferred), `L3_Section_F…Watchlist.md` §13 (`OPEN`, explicitly deferred).

**Repository impact.** Producing a coherent first-release scope is the primary deliverable of Phase 2 consolidation. A per-section Priority scale cannot produce it. Every section has reviewed its own distribution; no one has reviewed them together.

**Recommendation.** Before Phase 2 authoring sequence is set, run one cross-section Priority calibration pass: fix the institutional first-release definition, then apply it to all ten sections in one sitting. This is a single Lead Architect exercise, not ten section exercises.

---

### FIND-06 — Section E: Section H listed as a participating domain without business participation

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | E — `L2-E06_Payroll_Workflows.md`, `L2-E07_Employee_Profile_Architecture.md` |
| **Affected candidates** | `E-064`, `E-080`, `E-081`, `E-084` |
| **Disposition** | KEEP WITH EDIT (Domains; Type follows) |
| **Evidence class** | SUPPORTED INFERENCE |
| **Confidence** | MEDIUM-HIGH |

**Problem.** Four Section E candidates list `H` as a participating domain. In each case the only H-side touchpoint the L2 source establishes is notification delivery, analytics, enterprise search or document storage — consumption, not production of the outcome. Three of the four (`E-080`, `E-081`, `E-084`) are typed `Cross-domain` **solely** because of the H entry; remove it and they are single-domain.

**Evidence.**
1. `…/section-e/L2-E06_Payroll_Workflows.md:681–697` — §4.4 *Notification Domain Dependency*, followed by "Notification owns delivery" at line 3564. `E-064`'s Final Outcome (immutable payroll results, Finance handoff, payslips, closure) requires no act by Section H.
2. `…/section-e/L2-E07_Employee_Profile_Architecture.md:320–334` — E07's cross-domain touchpoints are *Notification Services, Analytics Platform, Enterprise Search, Master Data Services*, and its External Systems list is *DigiLocker, Government Identity Services, Document Storage, Enterprise Notification Platform*. DigiLocker appears as an **external system**, not as Section H business participation; `L2-H01` governs APAAR/DigiLocker for student credentials, not employee profile verification.
3. `L3_Phase1_Discovery_Rules.md` §11 — "A domain that merely consumes the final event, query, certificate, or capability is not listed unless it materially participates in producing the scenario outcome." `L3_Scenario_Type_and_Priority_Rules.md` §5 — "Do not use `Cross-domain` only because a notification, audit record, or authorization check exists."

**Repository impact.** This is the same defect the Section G watchlist §4A carries as an `OPEN` residual-participation review, appearing in a second inventory that has no such review. It inflates cross-domain classification and will misdirect Phase 2 hand-off analysis.

**Recommendation.** Narrow Domains on all four rows; re-derive Type for `E-080`, `E-081`, `E-084`. Extend the participation review recorded for Section G to Sections B/D/E.

---

### FIND-07 — Section I: three enumerated branch outcomes of one L2 workflow carried as standalone candidates

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | I — `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` |
| **Affected candidates** | `I-283`, `I-284`, `I-285` against parent `I-282` |
| **Disposition** | MERGE |
| **Evidence class** | PROVEN |
| **Confidence** | HIGH |

**Problem.** `I-282`'s own Final Outcome reads: "The malpractice decision is **affirmed, modified, or overturned** through the governed review process." `I-283` (Uphold), `I-284` (Modify), `I-285` (Overturn) are those three words rendered as three further candidates. All four share owner, domains `C, I`, Type `Critical` and Priority `P1`.

**Evidence.**
1. `…/section-i/L2-I14_…Workflows.md:992–1017` — MW-16 *Review Appeal*, one workflow, whose **§Possible Outcomes** enumerates: "Decision upheld. Penalty modified. Penalty reduced. Penalty revoked. Case reopened. Appeal rejected." These are branches of one workflow's outcome, not six workflows.
2. `L3_Phase1_Discovery_Rules.md` §8 — outcome branches internal to one governed decision are not standalone L3 candidates.

**Repository impact.** This is distinct from the ruling already recorded in `L3_Section_I…Watchlist.md` `I-FR-10`, which retained the appeal family `I-281`…`I-288` as "post-terminal decisions with distinct consequences." That ruling correctly separates the *appeal* from the *adjudication* (`I-305`). It does not justify carrying `I-282` **and** its three enumerated branches. The finding is subsumption within the appeal family, not a challenge to the family's retention.

**Recommendation.** Merge `I-283`, `I-284`, `I-285` into `I-282`. `I-286` (Overturned Decision to Restored Examination Outcome) partially overlaps — `I-285` already states "affected examination outcomes are restored or recomputed as required" — but restoration reaches into downstream result and completion processes and is defensibly separate; retain pending Phase 2 judgment.

---

### FIND-08 — Notification-terminal candidates surviving in Section I after the same construct was retired in G

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | I — `L2-I15`, `L2-I03` |
| **Affected candidates** | `I-296`, `I-028` |
| **Disposition** | REMOVE (`I-296`) / KEEP WITH EDIT (`I-028`) |
| **Evidence class** | PROVEN |
| **Confidence** | HIGH |

**Problem.** `I-296` *Communicate Examination Contingency Decision to Affected Stakeholders* has the Final Outcome "The approved contingency outcome is **made available to affected stakeholders through the appropriate communication capability**." That is delivery, not an institutional result. `I-028` *Timetable Withdrawal to Notified Stakeholders* names notification in its title and terminates its Final Outcome at "…and stakeholders are notified", though it does carry a real outcome (authoritative withdrawal) ahead of it.

**Evidence.**
1. `L3_Phase1_Discovery_Rules.md` §8 lists "Send Notification" among the constructs that "should not become separate L3 scenario documents."
2. The same construct was retired at scale in Section G as cluster `G-FR-08` (39 rows, `APPLIED`) and inside Section I itself as part of `I-FR-10`, where `I-277` "Notify Candidate" was retired as "the §8 *Send Notification* example verbatim".
3. `L3_Section_I…Watchlist.md` `I-OB-03` treats `I-296` as the home for `L2-I15` EW-07 stakeholder notification — the row exists to hold a workflow step, which is the defect rather than its justification.

**Repository impact.** `L2-H02` and `CAN-L3-H-010` own governed communication delivery. `I-296` duplicates that ownership from the Examination side. EW-07 belongs inside the parent contingency candidates (`I-291`, `I-293`, `I-309`) as a step, exactly as G resolved it.

**Recommendation.** Retire `I-296`; fold stakeholder communication into the parent contingency outcomes. Retitle `I-028` to its real outcome (*Published Examination Timetable to Authoritative Withdrawal*) and end the Final Outcome at the withdrawal.

---

### FIND-09 — Section I: 21 command-form titles

| | |
| --- | --- |
| **Severity** | LOW |
| **Section / L2 area** | I — all 17 L2 areas |
| **Affected candidates** | `I-019`, `I-026`, `I-054`, `I-061`, `I-068`, `I-138`, `I-162`, `I-177`, `I-180`, `I-184`, `I-282`…`I-285`, `I-290`, `I-293`, `I-294`, `I-296`, `I-308`, `I-309`, `I-314` |
| **Disposition** | KEEP WITH EDIT |
| **Evidence class** | PROVEN |
| **Confidence** | HIGH |

**Problem.** 21 titles are imperative commands ("Plan and Approve an Examination Schedule", "Publish Official Examination Results", "Recompute Result Before Institutional Approval"). `L3_Phase1_Discovery_Rules.md` §5 requires "Outcome-based business scenario name". Section I is the only section with this pattern; the other nine sections have zero.

**Note.** Most of these describe genuine complete outcomes; the defect is title form, not scenario validity. The `check_l3_inventory.py` verb-initial warning did not catch them — its verb list does not include *plan, detect, recompute, republish, moderate, uphold, overturn, postpone, communicate*. The prior audit of 2026-08-14 recorded 208 verb-initial titles and applied only a §8 bright line; 21 remain.

**Recommendation.** Restate as outcomes during Phase 2 wording pass. Consider extending the checker's verb list so the residue is caught mechanically rather than by audit.

---

### FIND-10 — Section G: clearance split into request and publication; origin-variant duplicate

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | G — `L2-G05_Library_Management_Workflows.md` |
| **Affected candidates** | `G-182`/`G-183`; `G-173`/`G-181` |
| **Disposition** | MERGE (two pairs) |
| **Evidence class** | SUPPORTED INFERENCE |
| **Confidence** | MEDIUM-HIGH |

**Problem.** `G-183`'s Final Outcome is "Clearance completion status is **published to** the applicable student exit, employee exit, qualification completion, or transfer process" — a publication hand-off downstream of `G-182`, which already ends at "clearance is approved or rejected". `G-181` (Damaged Resource During Stock Verification) and `G-173` (Damaged Library Resource) differ only in the origin of the damage finding.

**Evidence.** `L3_Section_G…Watchlist.md` `G-FR-12` retired `G-523`, `G-524`, `G-525`, `G-538` on the basis that "consumer-side event delivery, read-model reconciliation and a query contract are mechanism, not institutional outcomes" — `G-183` is the publisher-side twin of the same construct. `G-OV-02` retired the emergency-cleaning origin variant on exactly the reasoning that applies to `G-181`/`G-173`.

**Recommendation.** Merge `G-183` into `G-182`; merge `G-181` into `G-173` with stock verification as an alternate trigger. Both are instances of the section-wide sweep FIND-01 requires.

---

### FIND-11 — Section B: `Cross-domain` tier unused; Section J effectively likewise

| | |
| --- | --- |
| **Severity** | LOW |
| **Section / L2 area** | B, J |
| **Affected candidates** | Section B (32 rows, 0 `Cross-domain`); Section J (110 rows, 2 `Cross-domain`) |
| **Disposition** | RECLASSIFY (review only) |
| **Evidence class** | SUPPORTED INFERENCE |
| **Confidence** | MEDIUM |

**Problem.** Section B has zero `Cross-domain` rows while eight of its 32 candidates list two or more participating domains (`B-001`, `B-007`, `B-011`, `B-013`, `B-017`, `B-991`…). Section J has two across 110 rows while roughly twenty list `F` or `G`.

This is **defensible** under `L3_Scenario_Type_and_Priority_Rules.md` §7, whose selection order puts `Critical` ahead of `Cross-domain` — a high-risk multi-domain scenario is correctly `Critical`. It is recorded as LOW rather than dismissed because complete absence of a tier across a whole section is the same signal that `check_l3_inventory.py` flags as "Type is fully predicted by…" and warrants one confirming pass rather than an assumption.

**Recommendation.** Confirm during the Phase 2 Type review; no change required if §7 precedence is the actual reason.

---

## 7. Missing Coverage Register

Only evidence-supported outcome families are listed. Three entries.

---

### MC-01 — Faculty Collaboration / academic exchange with external institutions

| | |
| --- | --- |
| **Section** | H (likely primary), with E participation |
| **L2 source** | `…/section-h/L2-H10_External_Education_Integration_Workflows.md:1299` — §4.4 *Faculty Collaboration* |
| **Confidence** | HIGH |

**Missing outcome.** `L2-H10` §4.4 names Faculty Collaboration as one of seven **supported business scenarios** of the domain, comprising visiting faculty, joint teaching, collaborative supervision and academic exchange, and states "Faculty ownership remains within HR."

**Why it qualifies.** It has a clear trigger (an approved academic collaboration arrangement with an external provider), a clear terminal institutional result (an active, governed collaboration engagement that later closes with its academic contribution recorded), and a determinable owner split — H10 owns the external relationship and exchange, E owns the workforce record.

**Closest existing candidate.** None. A keyword sweep of all 1,123 rows across all five inventories returns zero matches for *visiting*, *joint teaching*, *academic exchange* or *partner institution*. `H-076`…`H-085` are entirely student-facing. `E-006` *Workforce Deputation Lifecycle* is explicitly internal — its Final Outcome restores "the normal engagement context" and its Domains field is `E` alone; `L2-E01` WF-010/WF-011 define deputation as internal organisational movement, not external institutional collaboration.

**Likely owner.** External Education Integration (H10) for the collaboration arrangement; People & Workforce (E) for the resulting engagement. Split to be confirmed in Phase 2.

---

### MC-02 — Student Mobility (exchange, visiting, joint programme, semester/international mobility)

| | |
| --- | --- |
| **Section** | H (likely primary), with C and D participation |
| **L2 source** | `…/section-h/L2-H10_…Workflows.md:1229` — §4.4 *Student Mobility* |
| **Confidence** | MEDIUM |

**Missing outcome.** §4.4 names Student Mobility as a supported business scenario comprising exchange programs, visiting students, joint programs, semester mobility and international mobility. Zero matches for *mobility*, *exchange program*, *joint program* or *visiting student* across all 1,123 rows.

**Why it qualifies.** The end-to-end journey — approved mobility arrangement, outbound placement, external academic period, return with recognised standing — is a single institutional outcome with distinct governance from any of its parts.

**Closest existing candidates.** `H-080` (external enrollment exchange), `H-081` (external learning synchronisation), `H-082` (verified external credit). Each covers one leg. Confidence is MEDIUM rather than HIGH precisely because a defensible reading holds the journey to be the composition of those three plus `H-076`; Phase 2 should decide whether the composed journey is an independent L3 outcome or an orchestration of existing ones. Unlike MC-01, this is not a bare gap.

---

### MC-03 — Sponsor registration and sponsorship arrangement lifecycle

| | |
| --- | --- |
| **Section** | F |
| **L2 source** | `…/section-f/L2-F11_Scholarship_and_Financial_Aid.md:940–960` — §12 *Sponsorship Management* |
| **Confidence** | MEDIUM |

**Missing outcome.** `L2-F11` §12 defines Sponsorship Management with responsibilities "sponsor **registration**, sponsorship lifecycle, sponsor allocation, sponsor monitoring, sponsorship renewal", over sponsor types Government, Corporate, Trust, Alumni, NGO. `Sponsorship` is a named internal entity of the Financial Aid Aggregate (§14).

**Why it qualifies.** Sponsor registration establishes an external funding counterparty with its own eligibility, lifecycle and monitoring — structurally the same class of outcome as `J-020`/`J-021` (supplier registration and approval), which the inventory does cover.

**Closest existing candidates.** `F-062` begins at "Approved Aid Decision", downstream of the sponsor relationship. `F-066` covers funding and utilisation reconciliation. `F-007` mentions "Student or Sponsor Payment" but is a payment-receipt outcome. None covers establishing or governing the sponsorship arrangement itself.

**Note on Waiver and Concession.** `L2-F11` §12 also defines Waiver Management and Concession Management. These are **not** reported as missing: the approval path is covered by `F-062`'s "Approved Aid Decision" trigger and the liability effect by `F-002` and `F-020`. Zero inventory matches for "concession" is noted but not sufficient positive evidence of a missing complete outcome.

---

## 8. Fragmentation / Merge / Split / Duplicate Register

### Merge clusters

| ID | Section | Candidates | Proposed consolidated outcome | Basis |
| --- | --- | --- | --- | --- |
| `MG-01` | G | `G-310`, `G-311`, `G-312`, `G-313`, `G-315`, `G-336` | **Approved Cleaning Schedule to Verified Cleaning Completion** — one governed scheduled-cleaning outcome from schedule approval through task execution and quality verification | `L2-G08:982` one workflow, six steps; four rows have readiness-state outcomes |
| `MG-02` | G | `G-321`, `G-323`, `G-324`, `G-326` | **Cleaning Complaint to Verified Resolution** — registration, validation, prioritisation, assignment, execution, inspection, closure | `L2-G08:1085` one Complaint Workflow |
| `MG-03` | C | `C-039`, `C-040`, `C-041`, `C-042`, `C-044`, `C-045` | **Authorised Student Exit to Concluded Institutional Relationship**, Exit Type as governed variant | `L2-C05:615` attribute definition; `:1697` institution-extensible; `:1253-1257` single terminal state |
| `MG-04` | J | `J-117`, `J-118`, `J-119`, `J-120`, `J-121`, `J-122` into `J-115` | **Authorised Disposal Case to Reconciled Physical Disposal Closure**, Disposal Method as governed variant | `L2-J10:677` configurable method; `:1877` method is one of six authority inputs; `:2096` proceeds Finance-owned; `:755` single execution sequence |
| `MG-05` | I | `I-283`, `I-284`, `I-285` into `I-282` | **Malpractice Decision Appeal to Governed Review Outcome** (affirmed / modified / overturned) | `L2-I14:1008` MW-16 *Possible Outcomes* enumerates all three as branches of one workflow |
| `MG-06` | G | `G-183` into `G-182` | **Library Clearance Request to Approved Clearance**, publication to exit processes as the hand-off | `G-FR-12` precedent: publication is mechanism, not outcome |

### Splits

None recommended. No candidate was found combining independent terminal results without a unifying outcome. `I-305` and `H-025` are long but each resolves to one institutional result.

### True duplicates

| ID | Candidates | Assessment |
| --- | --- | --- |
| `DU-01` | `G-173` / `G-181` | Same damaged-resource outcome differing only by origin (return vs. stock verification). Same class as the already-retired `G-OV-02`. Merge, retaining stock verification as an alternate trigger. |

### Overlaps — retain, do not merge

| ID | Candidates | Why they stay separate |
| --- | --- | --- |
| `OV-01` | `I-285` / `I-286` | `I-285`'s outcome already mentions restoration, but `I-286` reaches into downstream result, publication and completion processes with a distinct institutional consequence. Retain both pending Phase 2. |
| `OV-02` | `B-006` / `G-450` | Institutional Campus (B03) and Physical Campus (G10) are distinct canonical entities. Both inventories state the reciprocal boundary. Confirmed sound. |
| `OV-03` | `C-059`…`C-068` / `F-062`…`F-066` | Correctly split by INT-005 (2026-08-05): C07 owns eligibility policy and the eligibility record; F11 owns financial approval, allocation and disbursement. `C-068` is the explicit hand-off. Confirmed sound. |
| `OV-04` | `A-006` / `J-035`, `E-096` | Duplicate-identity reconciliation for Person, Supplier and Employee Profile respectively — three different canonical aggregates. Not duplicates. |

### Cross-inventory duplicate scan

A pairwise title-similarity scan across all 1,123 candidates in all five inventories, restricted to cross-section pairs, returned **11 pairs above a 0.5 Jaccard threshold and zero true duplicates**. Every high-scoring pair was verified as a distinct subject (e.g. `A-006` Person vs `J-035` Supplier; `B-002` Institution vs `E-004` Workforce Engagement suspension). No business outcome is claimed by two sections.

---

## 9. Repository Conflict Register

These are **repository defects, not inventory defects**. In every case the inventory recorded and preserved the conflict rather than resolving it; no section was penalised for them.

| ID | Sources | Conflicting positions | Affected candidates | Authority analysis | Phase 2 blocked? | Escalate to |
| --- | --- | --- | --- | --- | --- | --- |
| `CF-08` (Section C) | `DEPENDENCY_MAP` §7 (status Open); `L2-C04` §2.6; `L2-C08` §5 | **Policy Management has no owner.** `AlumniTransitionPolicyUpdated` and `AcademicPolicyUpdated` have no publisher anywhere in the repository. `L2-C04` §2.6 records the gap in-document. | Two candidates *cannot* be discovered — the academic-progression and alumni-transition policy lifecycle analogues of `C-060` | Not resolvable by hierarchy — the capability is absent, not contested. `DEPENDENCY_MAP` §7 states owner assignment requires Lead Architect sign-off | **Yes, scoped** — two outcome families cannot be created | Lead Architect / EA Board |
| `I-RC-07` (Section I) | `L2-I07` §2 and §9.5; `L2-I14` §9.2; `L2-I15` §9.2 | **Question paper governance has no owner.** `L2-I07` owns only Question Paper *Distribution* and explicitly excludes creation, moderation and printing; I14 and I15 both disclaim it. | `I-111`, `I-112`, `I-113` — real institutional outcomes resting on an unassigned capability | Cannot be resolved by ruling between existing documents; needs a new owner or a new L2 document | **Yes, scoped** — three candidates cannot be assigned an owner | Lead Architect / EA Board |
| `CF-04` (Section C) | `L2-C02` WF-C02-13 vs `L2-C06` §2, §5 | Guardian capability claimed by two documents; C06 declares exclusive ownership | `C-048`…`C-058` (11 rows) | C06's exclusivity language is the defensible read and was applied — **recorded, not silent** | No | EA Board |
| `CF-05` (Section C) | `L2-C04` §1.5 vs `L2-C05` §5/§11.10 vs `L2-C08` §5.2 | Alumni Transition ownership stated three ways | `C-069` | `L2-C08` bounded context applied; `L2-C04` §1.5 needs source correction | No | Source correction |
| `CF-01`, `CF-02`, `CF-09` (Section C) | `L2-C01` §6 vs §10.3 | Two incompatible admission state vocabularies; `Rejected` published but not in any canonical state list | All C01 candidates | Conflict-dependent wording; §6 adopted as working vocabulary and recorded | No | Source correction |
| `FIND-D-01`…`D-08` (Section D) | `L2-D00` vs `L2-D03`/`D04`/`D07` | Aggregate ownership lists, terminology mapping and command-matrix coverage differ between sources | `D-016`, `D-017` and D05/D06/D07 rows | Preserved; no reconciliation attempted | No | Source correction |
| `BDE-B-01` (Section B) | `L2-B02` WF08 vs `L2-B03` §111/§112 | Decommission and archive: one workflow or two | `B-032`, `B-009` | Registration/activation convention deliberately not extended pending reconciliation | No | Source correction |
| `RC-01` (A/E/G) | `L2-A04` vs `L2-E02` | Delegation ownership | `A-012`, `E-103`, `G-362` | **RESOLVED 2026-08-15** — A04/A03 scoped to Authorization Delegation, E02 to Assignment Delegation | No | Closed |

**Systemic observation.** `CF-08` and `I-RC-07` are the same defect class in two sections: a capability that multiple locked L2 documents depend on, that no L2 document owns. Both were found by discovery, not by L2 review. It is worth asking whether a repository-wide sweep for unpublished-event / unowned-capability references would surface more before Phase 2 authoring begins.

---

## 10. Cross-Section Findings

| # | Finding | Sections | Detail |
| --- | --- | --- | --- |
| `XS-01` | Priority incomparable across sections | All | FIND-05. P0 density 12%–65%. Blocks first-release scoping. |
| `XS-02` | Enumerated-attribute-value fragmentation not swept | C, J (G already applied) | The `G-FR-10` principle — one row per configurable attribute value is not a scenario set — was applied in G and never carried to Section C Exit Type (MG-03) or Section J Disposal Method (MG-04). |
| `XS-03` | Notification-as-outcome not swept | I (G, H already applied) | `G-FR-08` retired 39 rows; H02 consolidated 86 rows into 7. `I-296` and `I-028` survive the same test. |
| `XS-04` | Domain-participation review incomplete | E, G | The Section G `OPEN` residual participation review (watchlist §4A) has an unrecorded twin in Section E (FIND-06). B and D were not reviewed for it at all. |
| `XS-05` | Consolidation applied cluster-by-cluster, not swept | G, and now C, I, J | The Section G watchlist §10 root-cause note of 2026-08-15 identified this failure mode. This audit found three further instances in three different sections, confirming it is a process defect rather than a Section G defect. |
| `XS-06` | No canonical-ownership duplication anywhere | All | Verified. The B03/G10 campus boundary, F/J monetary boundary, I result ownership, H07 dashboard boundary (`H-DEC-01`) and C07/F11 scholarship split (INT-005) are all explicitly stated on both sides. |

---

## 11. Clean-Coverage Summary

Areas where the audit found no material exception. Reported as counts only.

| Section | Candidates checked | L2 areas covered | Result |
| --- | ---: | ---: | --- |
| A — Platform Foundation | 62 | 11 / 11 | No material exception. `VAR:<role>` notation used correctly per §11 throughout; all 18 watchlist items `CLOSED` or `APPLIED`; no `OPEN` or `BLOCKED` cluster. |
| B — Institution & Structure | 32 | 6 / 6 | No material exception beyond FIND-11 (LOW). Position Definition → B02 / Position Assignment → E02 re-ownership verified correct against source. |
| D — Academic Delivery | 57 | 8 / 8 | No material exception. Best Type discrimination in the repository (49% Critical). Eight repository conflicts preserved without reconciliation — exemplary. |
| F — Finance | 85 | 15 / 15 | Granularity and ownership sound; least fragmented large section. Exceptions are FIND-04 (classification) and MC-03 (sponsorship) only. |
| H — Institutional Services | 81 | 11 / 11 | Ownership handling is the strongest in the repository — `H-DEC-01`…`H-DEC-04` state four boundaries explicitly and apply them to both sides. Exceptions are MC-01 and MC-02 only. |
| Repository-wide | 1,123 | 107 / 107 | Every L2 document in scope is represented by at least one candidate. Candidate IDs unique across all five inventories. No implementation architecture (API, schema, class, screen, deployment, framework) found in any row. No fabricated state, event, aggregate or business rule found. No candidate criticised for a field Phase 1 does not require. |

---

## 12. Overall Score

Arithmetic mean of the ten section scores, unweighted by candidate count:

`(9.5 + 9.2 + 9.0 + 9.3 + 8.7 + 8.8 + 8.1 + 8.9 + 8.7 + 9.1) / 10 = 89.3 / 10`

**Overall score: 8.9 / 10.0**

---

## 13. Phase 2 Readiness Verdict

## NOT READY — MATERIAL CORRECTIONS REQUIRED

The numerical score reflects genuinely good discovery work. The verdict is driven by three things the score does not capture:

1. **Priority cannot currently produce a first-release cut** (FIND-05). This is the primary output Phase 2 consolidation exists to produce, and no per-section review can fix it.
2. **Three unswept fragmentation clusters across three sections** (MG-01…MG-06), each an instance of a principle the repository has already accepted and applied elsewhere. Consolidating on top of them would carry the fragments into the L3 document set.
3. **Two proven coverage gaps** (MC-01, MC-03) with positive L2 evidence and zero existing coverage anywhere in 1,123 candidates.

Two repository conflicts (`CF-08`, `I-RC-07`) block five candidates specifically. They do not block Phase 2 as a whole, and the inventories handle them correctly — but they need a Lead Architect / EA Board ruling before the affected outcomes can be authored.

---

## 14. Prioritized Correction Recommendations

| # | Action | Severity | Owner | Effort |
| --- | --- | --- | --- | --- |
| 1 | Run one **cross-section Priority calibration** against a fixed first-release definition, all ten sections in one pass (FIND-05) | HIGH | Lead Architect | One sitting, 1,123 rows |
| 2 | Execute merge clusters `MG-01`…`MG-06` and duplicate `DU-01` — ~20 rows retired, 6 rewritten | HIGH | Discovery owners (Varad, Chetan, Dhruv, Devang) | Small |
| 3 | Run a **section-wide sweep** for step-chain, enumerated-attribute and notification-terminal constructs across all ten sections before closing Phase 1 — do not repeat the cluster-by-cluster pattern (`XS-05`) | HIGH | All discovery owners | Medium |
| 4 | Re-derive Section F Type against §7 and Priority against §12; the `Simple` and `P2` tiers are unused across 85 rows (FIND-04) | HIGH | Lead Architect + Varad | Medium |
| 5 | Add the MC-01 Faculty Collaboration outcome family; decide MC-02 Student Mobility and MC-03 Sponsorship | HIGH / MEDIUM | Varad (H, F) | Small |
| 6 | Narrow Domains on `E-064`, `E-080`, `E-081`, `E-084`; re-derive their Type. Extend the Section G residual participation review to B, D and E (FIND-06, `XS-04`) | MEDIUM | Pallav | Small |
| 7 | Retire `I-296`; retitle `I-028`; restate the 21 command-form Section I titles as outcomes (FIND-08, FIND-09) | MEDIUM / LOW | Devang | Small |
| 8 | Escalate `CF-08` (Policy Management) and `I-RC-07` (question paper governance) for owner assignment; consider a repository-wide sweep for other unowned capabilities | MEDIUM | EA Board | Decision |
| 9 | Confirm Section B / J `Cross-domain` absence is §7 precedence rather than under-classification (FIND-11) | LOW | Pallav, Dhruv | Small |
| 10 | Extend `check_l3_inventory.py`'s verb-initial list (*plan, detect, recompute, republish, moderate, uphold, overturn, postpone, communicate*) so the residue is caught mechanically | LOW | Repository maintainer | Small |

---

## 15. Audit Limitations / Items Not Verified

**Verified by targeted source reading.** All findings citing an L2 line number were verified against the original locked L2 document, not against a Canonical AI Context summary or a prior generated review.

**Not independently re-verified — accepted as previously audited.** The `APPLIED` consolidation decisions recorded in the nine section watchlists (approximately 1,500 retired rows across Sections G, H and I) were not re-derived from source. This audit checked whether the *principles* behind them had been swept, not whether each individual retirement was correct.

**Sampled, not exhaustively verified.**
- Section G's 350 candidates: G00, G05, G08 and G10 were read in full; G01–G04, G06, G07, G09 were assessed from title, trigger, outcome, owner, domains, type and priority via structured extraction, plus targeted greps. A full row-by-row source verification of Section G was not performed and further fragmentation may exist there.
- Sections A, B, D, E, J: full field-level review of every row; targeted L2 verification only where a finding was raised.

**Not verified.**
- Whether MC-02 Student Mobility is an independent L3 outcome or the composition of `H-076`, `H-080`, `H-081` and `H-082`. Recorded at MEDIUM confidence for that reason.
- The internal consistency of `L3_Section_C_Phase2_Consolidation_Watchlist.md`'s own header count. Line 34 states "1 `BLOCKED`" among repository conflicts; the only `BLOCKED` row in the document body is `CF-08` in §4.3, which is consistent, but the `DETERMINED`/`OPEN` counts were not re-tallied.
- `DEPENDENCY_MAP.md` was consulted only through watchlist citations. It carries uncommitted modifications in the working tree and was not read in full.
- Nine of the sixteen files in the working tree carry uncommitted modifications at audit time. This audit reflects the working-tree state, not the last commit.

**Explicitly out of scope.** No final L3 scenario IDs, ownership decisions or scenario documents were created. No repository conflict was resolved. No architecture, governance or inventory file was modified.

---

*End of report.*
