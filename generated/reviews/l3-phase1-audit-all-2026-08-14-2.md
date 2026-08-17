# SutraERP L3 Phase 1 — Independent Adversarial Discovery Audit (All Scope)


> **Numbering note (2026-08-17).** Candidate IDs in this historical report reflect the numbering in force when it was written. Repository-wide candidate numbering was normalized on 2026-08-17; for current IDs see `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.
## 1. Audit Metadata

| | |
| --- | --- |
| **Audit type** | Independent adversarial L3 Phase 1 scenario discovery audit |
| **Scope** | `AUDIT_SCOPE = ALL` — Sections A–J, all five Phase 1 inventories |
| **Date** | 2026-08-14 |
| **Mode** | Read-only against all architecture and governance files; single new report written to `generated/reviews/` |
| **Branch** | `dhruv` |
| **Repository state** | Post-correction. `generated/reviews/L3_Phase1_Audit_All_Correction_Application_2026-08-14.md` applied the 2026-08-14 all-scope audit earlier the same day. This audit is an **independent re-audit of the resulting state**, not a verification of that application. |
| **Prior report preserved** | `generated/reviews/l3-phase1-audit-all-2026-08-14.md` was not modified. This report carries the `-2` suffix per the filename rule. |
| **Candidates checked** | **1,175** (every row in all five inventories, extracted mechanically and reviewed) |

---

## 2. Scope Actually Audited

Assignment confirmed against `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Workload_Distribution_Decision.md` §2 before use. The split supplied in the audit brief matches the locked governance decision exactly; no inferred assignment was used.

| Inventory | Sections | Candidates |
| --- | --- | ---: |
| `L3_Phase1_Inventory_Dhruv.md` | A (62), J (110) | 172 |
| `L3_Phase1_Inventory_Pallav.md` | B (32), D (57), E (75) | 164 |
| `L3_Phase1_Inventory_Chetan.md` | C (74) | 74 |
| `L3_Phase1_Inventory_Varad.md` | F (85), G (377), H (85) | 547 |
| `L3_Phase1_Inventory_Devang.md` | I (218) | 218 |
| **Total** | **A–J** | **1,175** |

No `CAN-L3-X-###` cross-domain candidates exist in any inventory. Cross-domain outcomes are carried inside owning sections with `VAR:<role>` participation notation, which `L3_Phase1_Discovery_Rules.md` §11 permits. This is a legitimate discovery choice, not a gap.

---

## 3. Sources Consulted

**Governance (read in full):** `L3_Phase1_Discovery_Rules.md`; `L3_Scenario_Type_and_Priority_Rules.md`; `L3_Phase1_Workload_Distribution_Decision.md`; `L3_Section_G_Phase2_Consolidation_Watchlist.md`; `L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md`.

**Governance (targeted):** `L3_Section_A_Phase2_Consolidation_Watchlist.md` §9; `L3_Section_I_Phase2_Consolidation_Watchlist.md` §4B; `check_l3_inventory.py` lines 63–76.

**L2 originals (targeted extraction):** `L2-J05_Goods_Receipt_Inspection_and_Acceptance.md` (§1, §5, §13, Canonical Section Crosswalk); `L2-D04_Teaching_Session_and_Attendance_Workflows.md` §31.2; `L2-G03_Hostel_Management_Workflows.md` §24 and Exit Clearance Lifecycle; `L2-F03_Refund_Management.md` structure; L2 document manifests for all ten sections.

**Tooling:** `python check_l3_inventory.py` — **PASS, 1,175 candidates, 0 errors, 8 warnings.**

**Method:** all 1,175 rows were extracted into a structured set and swept programmatically for §8-disqualified patterns, action-verb titles, read-model/notification/authorization outcomes, owner and domain conventions, and Type/Priority distributions. L2 originals were opened only to prove or disprove a specific finding. No finding in this report rests on a Canonical AI Context file.

---

## 4. Executive Verdict

The inventory is in **good condition and materially better than its size suggests**. Field conformance is clean, no implementation architecture has leaked into any of the 1,175 rows, and the three genuinely unresolvable repository conflicts are recorded with both positions preserved and no side chosen. I could not prove a single missing outcome family anywhere in Sections A–J — a strong result for a discovery phase.

The defect that matters is not coverage. It is **altitude consistency**.

Between 2026-08-11 and 2026-08-14 the repository accepted, recorded and executed a specific set of consolidation principles: validation-failure branches are not standalone candidates; read-model refresh, event delivery and query contracts are mechanism, not outcomes; notification delivery belongs to H02, not to each publishing domain. Those principles are correct and are written down as `APPLIED` in the Section G watchlist. **They were applied selectively.** Seven rejection-only candidates still stand in Section G, all in the two sub-domains the passes did not reach. Two read-model rows identical in kind to four retired four days ago still stand in G09. Section H still carries the query and projection rows that Section G retired. Section I still carries 149 imperative-verb titles that the conformance checker structurally cannot see.

The result is that a Phase 2 consolidation starting today would inherit ten sections authored at meaningfully different scenario altitudes — Section G at roughly 38 candidates per L2 workflow document against a 6–14 range everywhere else — with no rule in the repository to reconcile them. That is a consolidation problem, not a discovery problem, and it is correctable.

Separately, and independent of everything above: **three repository conflicts remain unresolved and Phase 2 cannot close over them.** RC-01 (Delegation ownership), RC-02 (question-paper governance) and RC-03 (Policy Management) each require a decision no existing source can supply. I verified RC-01 directly and confirm it is genuinely undecidable from the current sources, correctly preserved, and correctly blocking. I also found that its recorded affected-candidate list is incomplete.

---

## 5. Section Scorecard

Rubric: Coverage 2.5 · Granularity/boundary 2.0 · Ownership/dependency/domains 2.0 · Field quality incl. Type & Priority 1.5 · Conflict handling 1.0 · Governance compliance 1.0.

| Section | Cov 2.5 | Gran 2.0 | Own 2.0 | Fields 1.5 | Conf 1.0 | Comp 1.0 | **Total** |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| A — Platform Foundation | 2.4 | 1.9 | 1.9 | 1.4 | 1.0 | 0.8 | **9.4** |
| B — Institution & Structure | 2.1 | 1.8 | 2.0 | 1.3 | 1.0 | 1.0 | **9.2** |
| C — Student Lifecycle | 2.3 | 1.7 | 1.9 | 1.3 | 1.0 | 0.8 | **9.0** |
| D — Academic Delivery | 2.2 | 1.8 | 1.9 | 1.3 | 1.0 | 1.0 | **9.2** |
| E — People & Workforce | 2.3 | 1.8 | 1.8 | 1.2 | 1.0 | 1.0 | **9.1** |
| F — Finance | 2.3 | 1.7 | 1.9 | 0.8 | 1.0 | 1.0 | **8.7** |
| G — Campus Operations | 2.4 | 0.9 | 1.5 | 1.2 | 0.9 | 0.7 | **7.6** |
| H — Institutional Services | 2.3 | 1.5 | 1.6 | 1.2 | 1.0 | 0.9 | **8.5** |
| I — Examination | 2.4 | 1.3 | 1.8 | 0.9 | 1.0 | 0.8 | **8.2** |
| J — Enterprise Resource Mgmt | 2.1 | 1.5 | 2.0 | 1.4 | 1.0 | 1.0 | **9.0** |

Scores reward evidence, not volume. Sections G and I carry the largest inventories and the two lowest scores; Sections B and J carry among the smallest and score near the top. No section was penalised for an upstream repository conflict it recorded honestly.

---

## 6. Findings Register

### FIND-A01 — Accepted §8 disqualification principle applied selectively within Section G
**Severity:** HIGH · **Area:** G08, G09 · **Confidence:** PROVEN

**Affected:** `CAN-L3-G-322`, `G-371`, `G-372`, `G-382`, `G-388`, `G-398`, `G-446`

**Problem.** Seven candidates exist whose entire Final Outcome is that an invalid input was refused. Examples: `G-372` *Duplicate Mentorship Assignment to Controlled Relationship Prevention*; `G-446` *Invalid Mentorship State Transition to Controlled Rejection*; `G-388` *Incomplete Parent Interaction to Controlled Recording Rejection*. None produces an institutional business result. `L3_Phase1_Discovery_Rules.md` §8 disqualifies exactly this class ("Validate Payment", "Check Eligibility"), and §9 marks a candidate weak when "the title describes only a small action".

**Evidence.** `L3_Section_G_Phase2_Consolidation_Watchlist.md` §4 rows `G-FR-05` and `G-FR-03` record this reasoning as **`APPLIED`** — fourteen G10 "invalid X to controlled rejection" rows and seven authorization-denial rows were retired on that exact basis. A repository-wide sweep of all 1,175 candidates returns **seven** rows still matching the pattern, and **all seven are in Section G**, in G08 and G09 — the two sub-domains no pass reached. No other section carries one.

**Repository impact.** Phase 2 will consolidate a section that has two internally contradictory altitudes for the same construct.

**Recommendation.** Extend `G-FR-05` to G08 and G09. Absorb each row as the validation-failure branch of its parent, as was done for G10.

---

### FIND-A02 — Section G09 mentoring family systematically fragmented into workflow steps
**Severity:** HIGH · **Area:** G09 Teacher-Guardian Scheme (`L2-G09`) · **Confidence:** PROVEN

**Affected (representative, not exhaustive):** `G-381`, `G-383`, `G-384`, `G-385`; `G-403`, `G-404`, `G-405`, `G-407`, `G-408`; `G-421`, `G-422`, `G-423`, `G-424`; `G-438`, `G-439`

**Problem.** G09 holds **47 candidates** — more than all of Section B (32) and approaching Section D (57) — for one campus-operations sub-domain. The rows decompose single journeys into their recording steps:

* One mentoring session becomes four rows: `G-381` discussion record → `G-383` guidance record → `G-384` recommendation record → `G-385` no-further-action closure. Each terminates in "a record is recorded"; none is an institutional outcome on its own.
* One follow-up cycle becomes five rows: `G-403` creation → `G-404` review → `G-405` additional guidance → `G-407` further follow-up → `G-408` completion.
* Four rows (`G-421`–`G-424`) are access-permission decisions for student, parent, department and administration. `L3_Phase1_Discovery_Rules.md` §11 and `L3_Scenario_Type_and_Priority_Rules.md` §5 both exclude authorization from being the substance of a scenario.
* `G-438` *Mentorship Read Model Refresh to Current Student Context* and `G-439` *External Domain Context Change to Mentoring Context Reconciliation* are consumer-side mechanism.

**Evidence.** `L3_Section_G_Phase2_Consolidation_Watchlist.md` §4A row `G-FR-12` retired `G-523`, `G-524`, `G-525`, `G-538` on 2026-08-14 with the stated basis "Consumer-side event delivery, read-model reconciliation and a query contract are mechanism, not institutional outcomes. §8." `G-438` and `G-439` are the same construct in G09 and survived the same pass.

**Repository impact.** Section G stands at ~37.7 candidates per L2 workflow document against 6.1–13.6 for every other section. Consolidating G09 at Section D's altitude would reduce it by roughly half.

**Recommendation.** Run a G09 pass mirroring `G-FR-04` (which absorbed 26 G07 movement steps into eight journeys). Expected clusters: one mentoring-session outcome, one follow-up-cycle outcome, one mentoring-information-access outcome, and retirement of the two read-model rows.

---

### FIND-A03 — Section I title convention violates the locked field definition at scale, invisibly to the checker
**Severity:** HIGH · **Area:** Section I, all sixteen L2 documents · **Confidence:** PROVEN

**Affected:** 149 of 218 rows (68%). Representative: `I-133`–`I-137`, `I-148`–`I-153` (*Resolve …*), `I-093`–`I-110` (*Conduct …*), `I-177`–`I-192` (*Publish … / Withdraw … / Restore …*), `I-250`–`I-266` (*Evaluate … / Establish …*), `I-296` *Communicate Examination Contingency Decision to Affected Stakeholders*.

**Problem.** `L3_Phase1_Discovery_Rules.md` §5 defines Scenario Title as "**Outcome-based** business scenario name", and §8 lists action names as not valid standalone candidates. Two-thirds of Section I titles are imperative command phrases. `I-296` is the §8 "Send Notification" pattern verbatim.

This is primarily a **titling** defect rather than a boundary defect — most affected rows carry a genuine terminal result in the Final Outcome column, and Section I should not be scored as if all 149 were fragments. But it is a locked-format defect at scale, and it makes the inventory unreadable against the boundary rule in §9, which tests the *title* first.

**Evidence.** `check_l3_inventory.py` lines 65–69: `FRAGMENT_VERBS` contains eighteen verbs — `validate, generate, send, check, update, save, approve, verify, calculate, notify, display, view, browse, search, configure, record, submit, assign`. It contains none of *Resolve, Conduct, Publish, Withdraw, Evaluate, Compute, Establish, Correct, Restore, Issue, Cancel, Release, Deny, Amend, Finalize, Escalate, Apply, Register, Schedule, Allocate, Review*. The checker therefore reports 0 errors and flags only `H-021`/`H-023`. The tool's own documentation anticipates this: `L3_Phase1_Discovery_Rules.md` §16.1 states "A clean checker run is a floor, not a pass."

**Repository impact.** Section I titles cannot be assessed against §9 as written, and Phase 2 will need to retitle 149 rows before boundary review is meaningful.

**Recommendation.** Retitle Section I to the trigger→outcome form used by Sections A, F, G, H and J. Separately, extend `FRAGMENT_VERBS` so the defect is detectable next time — that change belongs to the tool owner, not to this audit.

---

### FIND-A04 — Section F Type and Priority distribution is the mistake the rules name explicitly
**Severity:** HIGH (mitigated by disclosure) · **Area:** Section F, `L2-F01`–`L2-F14` · **Confidence:** PROVEN

**Affected:** all 85 Section F rows.

**Problem.** Section F is **79 Critical / 6 Cross-domain / 0 Simple** and **55 P0 / 30 P1 / 0 P2**. Not one candidate in an 85-row Finance inventory is Simple, and not one is deferrable.

**Evidence.** `L3_Scenario_Type_and_Priority_Rules.md` §14 Mistake 2 states the incorrect pattern as "All Finance scenarios = Critical" and the correct one as "classification depends on the specific business outcome". §11 requires P2 for "rare, future, optional, institution-specific, or low-frequency" scenarios; Section F contains such rows — `F-019` *Refund Closure to Historical Reconstruction* and `F-055` *Closed Financial Period to Governed Archival* are both marked P1.

**Mitigation.** This is recorded as `OPEN` in `L3_Section_F_Phase2_Consolidation_Watchlist.md` §13 with four worked starting points, and `L3_Scenario_Type_and_Priority_Rules.md` §15 does reserve Type/Priority correction for Lead Architect review. The inventory has not hidden the problem. Severity is retained because the distribution is a direct hit on a named rule and remains uncorrected; the Fields sub-score reflects it, the Conflict-handling sub-score does not penalise it.

**Recommendation.** Lead Architect re-derives Section F Type and Priority under §7 and §12. This is a bounded ~85-row task.

---

### FIND-A05 — Section I Type distribution, same rule
**Severity:** MEDIUM (mitigated by disclosure) · **Area:** Section I · **Confidence:** PROVEN

197 of 218 rows (90%) are `Critical`; 189 (87%) are `P1`; one row is `P2`. `L3_Scenario_Type_and_Priority_Rules.md` §14 Mistake 2 names "All Examination scenarios = Critical" alongside Finance. Recorded `OPEN` in the Section I watchlist and surfaced by `check_l3_inventory.py`. Same recommendation and same bounded scope as FIND-A04.

---

### FIND-A06 — Primary Owner convention is split across inventories
**Severity:** MEDIUM · **Area:** cross-inventory · **Confidence:** PROVEN

**Affected:** 929 of 1,175 rows (79%) — all of B, D, E, F, G, H, I.

**Problem.** Sections B, D, E, F, G, H and I record Primary Owner as an **L2 document code** (`B02`, `D01`, `F05`, `G10`, `H09`, `I07`). Sections A, C and J record it as a **business-capability name** (`Person & Identity`, `Student Lifecycle`, `Vendor & Supplier Management`).

`L3_Phase1_Discovery_Rules.md` §5 defines the field as "Likely **domain** owner of the final business outcome", and every worked example in §10 is a business-domain name — *Student Lifecycle*, *Finance*, *Examination*, *Enterprise Resource Management*. A document code is a narrower construct than a domain: recording `F05` rather than `Finance` pre-commits the owner at sub-document granularity, which Phase 1 is explicitly not permitted to lock (§14, §18).

**Evidence.** `L3_Phase1_Audit_All_Correction_Application_2026-08-14.md` §2 FIND-016 unified this convention **for Dhruv's inventory only** ("Dhruv's Primary Owner convention unified on business-capability names"). The other four inventories were not brought along, so the repository now has a documented preferred convention followed by three sections out of ten.

**Repository impact.** Phase 2 owner confirmation must normalise 929 values before ownership can be compared across sections.

**Recommendation.** Normalise B, D, E, F, G, H and I to business-capability names, retaining the document code as a source reference outside the locked eight fields if it is useful. Do not renumber or re-own anything as part of this.

---

### FIND-A07 — RC-01 affected-candidate list is incomplete
**Severity:** MEDIUM · **Area:** Section G08 vs Sections A and E · **Confidence:** PROVEN (omission); SUPPORTED INFERENCE (materiality)

**Affected:** `CAN-L3-G-362` *Housekeeping Delegation to Temporary Operational Authority*, owner G08, `Critical`, `P1`, domains `G, A`.

**Problem.** RC-01 records the Delegation ownership conflict between `L2-A04` ("exclusively owns … Delegation") and `L2-E02` (`Assignment Delegation | Canonical`). Both `L3_Section_A_Phase2_Consolidation_Watchlist.md` §9 and `L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md` §4 list the affected candidates as exactly two: `CAN-L3-A-012` and `CAN-L3-E-103`.

`G-362` is a **third** candidate whose final outcome is a delegation of business authority, owned by neither A nor E. A search of all three watchlists returns no reference to it.

**Evidence.** `L3_Section_A_Phase2_Consolidation_Watchlist.md:175` and `L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md:52` both enumerate two affected candidates. `G-362` appears in no watchlist. Under `L2-A04`'s "exclusively owns" wording, a G08-owned delegation outcome is a third instance of the same unresolved boundary.

**Repository impact.** Whichever way RC-01 is decided, `G-362` is affected and is not currently tracked to receive that decision.

**Recommendation.** Add `G-362` to RC-01's affected-candidate list in both watchlists. Do not re-own it and do not choose a side — RC-01 remains `BLOCKED`.

---

### FIND-A08 — Section J is authored at two different altitudes
**Severity:** MEDIUM · **Area:** J02 vs J05 · **Confidence:** PROVEN

**Affected:** `J-020`–`J-037` (18 rows, J02) against `J-049`, `J-056` (2 rows, J05).

**Problem.** The same author gave `L2-J02` Vendor & Supplier Management **18** candidates, including six separate supplier-status outcomes (`J-029` Restriction, `J-030` Suspension, `J-031` Blacklist, `J-032` Remediation, `J-033` Inactive, `J-034` Closure) and three separate qualification outcomes (`J-022` grant, `J-023` renewal, `J-024` expiry) — and gave `L2-J05` **two**, of which one (`J-049`) absorbs the entire receipt → inspection → acceptance/rejection/return/variance → downstream-availability lifecycle.

`L2-J05` line 42 defines that document's scope as "delivery receipt, inspection, acceptance, rejection, variance recording, and supplier return handling", and line 246 states the lifecycle as "Issued Purchase Order → Delivery Arrival → Goods Receipt → Inspection → **Acceptance, or Rejection/Variance/Return**" — an explicit terminal branch point. `L3_Phase1_Discovery_Rules.md` §7 names "Purchase Requisition to Goods Receipt and Inventory" as a canonical complete outcome, so a single broad row is not wrong in principle. It is inconsistent with the granularity the same inventory applies one document earlier.

**Repository impact.** Both altitudes cannot be the correct L3 altitude for Section J. Phase 2 must pick one and apply it to both.

**Recommendation.** Decide Section J's altitude, then either split `J-049` at the acceptance/return branch or consolidate the J02 supplier-status rows into a supplier-eligibility-governance outcome family. Do not do both. See SPLIT-01 and MG-06.

---

### FIND-A09 — Section H domain participation inconsistent with its own 2026-08-14 correction
**Severity:** MEDIUM · **Area:** H03, H10 · **Confidence:** PROVEN

**Affected:** `H-018` (`A, B, C, D, E, G, H`), `H-085` (`A, B, C, D, E, H, I`).

**Problem.** `L3_Phase1_Audit_All_Correction_Application_2026-08-14.md` FIND-009 changed 19 Section H rows "from all-ten-domain listings to governed `VAR:<role>` participation". `H-018` *Entity Context to Related Enterprise Discovery* and `H-085` *Governed External Education Data Exchange* retain seven-domain enumerations. In both, the listed domains are **indexed or exchanged over**, not participants producing the outcome — `L3_Phase1_Discovery_Rules.md` §11 states "A domain that merely consumes the final event, query, certificate, or capability is not listed unless it materially participates in producing the scenario outcome."

**Recommendation.** Apply the same `VAR:<role>` treatment used on the other 19 rows.

---

### FIND-A10 — Section H carries the query, projection and dashboard-mechanism rows Section G retired
**Severity:** MEDIUM · **Area:** H03, H07 · **Confidence:** SUPPORTED INFERENCE

**Affected:** `H-017` *Authorized Enterprise Query to Search Result*; `H-019` *User Search Preference to Saved Discovery Workspace*; `H-020` *Authoritative Source Change to Current Search Projection*; `H-022` *New Domain Projection to Governed Search Availability*; `H-053` *User Personalization to Effective Personal Dashboard Layout*; `H-058` *Source Projection Change to Refreshed Dashboard or Controlled Degradation*.

**Problem.** These are query execution, saved preferences, projection refresh and personal layout. `G-FR-12` retired `G-523`–`G-525` and `G-538` for being "consumer-side event delivery, read-model reconciliation and a query contract … mechanism, not institutional outcomes". `H-053` in particular — a personal dashboard layout — sits close to the L6 boundary that `L3_Phase1_Discovery_Rules.md` §14 places outside Phase 1.

**Why SUPPORTED rather than PROVEN.** `L2-H03` and `L2-H07` establish search and dashboard composition as Section H's actual business capability, not merely as infrastructure it consumes. A domain whose product *is* discovery has a stronger claim to query-shaped outcomes than Section G did. The boundary needs a Phase 2 ruling rather than a mechanical retirement.

**Recommendation.** Rule on whether H03/H07 platform capability outcomes are L3 business scenarios. If the `G-FR-12` reasoning holds, retire the six rows; if Section H's capability ownership prevails, record that explicitly so the asymmetry with Section G is deliberate and documented.

---

### FIND-A11 — Section G watchlist contradicts itself and its own header
**Severity:** LOW · **Area:** governance hygiene · **Confidence:** PROVEN

`L3_Section_G_Phase2_Consolidation_Watchlist.md:4` states "391 current sparse candidates"; §4A line 78 states "Section G total: **391 → 377 candidates**". The header was not updated — the same class of defect as FIND-015 in the prior audit (Section I header 258 → 245). Actual count verified at **377**.

Line 169 states "No identified Section G audit decision remains open", while line 76 of the same document records an `OPEN` residual domain-participation review. **Recommendation:** correct the header to 377 and amend line 169 to except the §4A `OPEN` item.

---

### FIND-A12 — Section G information-delivery rows survive the `G-FR-08` principle
**Severity:** LOW · **Area:** G03, G04, G05 · **Confidence:** PROVEN

`G-113` *Hostel Occupancy Change to Finance Consumption* ("residency information is published or made available to Finance"), `G-114` *Hostel Occupancy Status to Security Operational Awareness* ("Security receives the relevant operational information"), `G-150` *Transport Allocation to Finance Consumption*, `G-170` *Library Fine to Finance Payment Status Consumption*. Each terminates in information reaching a consumer. `G-FR-08` retired 39 rows on the basis that publisher-side facts belong inside their source parent. `G-183` *Library Clearance Approval to Institutional Exit Consumption* is arguably distinguishable — clearance is a gating institutional decision — and is left out of this finding.

---

### FIND-A13 — `C-044` is an undifferentiated catch-all
**Severity:** LOW · **Area:** C05 · **Confidence:** SUPPORTED INFERENCE

`CAN-L3-C-044` *Administrative Exit Closure* — trigger "Institution authorizes an administrative exit under the applicable…", outcome "Student Context reaches the applicable administrative exit terminal outcome". Neither names a specific condition. This is the pattern `G-SP-03` retired as "vague catch-all" across nine Section G rows. The four named exits beside it (`C-039`–`C-042`) are specific and should be retained — see DUP/SEP-02.

---

## 7. Missing Coverage Register

**No missing outcome family met the evidentiary bar. The register is empty.**

This is a genuine finding, not an omission. Every L2 workflow document in Sections A–J carries candidate coverage, and each probe I ran for a plausibly absent outcome resolved to existing coverage or to insufficient source evidence.

Two investigations are recorded because they came close and a later reviewer will otherwise repeat them:

| Probe | Result | Evidence |
| --- | --- | --- |
| **Faculty Substitution** — named as a correct P1 example in `L3_Scenario_Type_and_Priority_Rules.md` §10; returns zero matches across all 1,175 candidates. | **Not raised.** `L2-D04` places Faculty Substitution at §**31.2**, inside "# 31. Business Rules", not among its workflows. It defines conceptual terms and constraints ("substitution never modifies the Faculty Academic Assignment aggregate") rather than an independent journey. `CAN-L3-D-019` covers the teaching-session outcome that substitution occurs inside. A governance *example* is not positive source evidence of a missing outcome. | `L2-D04` lines 1102, 1182–1197 |
| **Supplier return in Section J** — `L2-J05` names supplier return handling as in-scope; no candidate is titled for it. | **Not raised — covered semantically.** `J-049`'s Final Outcome reads "each quantity reaches authorised acceptance, rejection, **return**, replacement or exception disposition". Searching by business meaning rather than title resolves this. | `L2-J05` line 42; `J-049` |

`BDE-E-01` in the B/D/E watchlist reaches the same conclusion for the D/E-side examination-facing counterparts, on the same basis — no positive E-side source evidence. I independently agree with that disposition.

---

## 8. Fragmentation / Merge / Split / Duplicate Register

### Merge clusters

| ID | Candidates | Proposed consolidated outcome | Basis | Confidence |
| --- | --- | --- | --- | --- |
| `MG-01` | `G-322`, `G-371`, `G-372`, `G-382`, `G-388`, `G-398`, `G-446` | Absorb each as the validation-failure branch of its parent journey | `G-FR-05`, `G-FR-03` already `APPLIED` for the identical class in G10 and G07 | HIGH |
| `MG-02` | `G-381`, `G-383`, `G-384`, `G-385` (retain `G-381` rewritten) | A mentoring session is conducted and recorded with its guidance, recommendations and closure disposition | Four consecutive recording steps of one session; none is a terminal institutional result | HIGH |
| `MG-03` | `G-403`, `G-404`, `G-405`, `G-407`, `G-408` (retain `G-404` rewritten) | A mentoring follow-up cycle completes with reviewed progress and its continuation decision | One follow-up journey expressed as five transitions | HIGH |
| `MG-04` | `G-421`, `G-422`, `G-423`, `G-424` | Authorised mentoring information is released under institutional privacy policy to the requesting party | Four rows differing only by requester; authorization is excluded from being scenario substance by `L3_Scenario_Type_and_Priority_Rules.md` §5 | HIGH |
| `MG-05` | `G-438`, `G-439` — **retire, do not merge** | Absorb into publishing parents | Same construct as `G-523`–`G-525`, `G-538` retired under `G-FR-12` | HIGH |
| `MG-06` | `J-029`–`J-034`, and separately `J-022`–`J-024` | Supplier eligibility and standing reach their governed status under authorised decision | **Conditional on FIND-A08.** Apply only if Section J adopts the broader J05 altitude | MEDIUM |
| `MG-07` | `F-014`, `F-015` (retain `F-015` rewritten) | An approved refund reaches financial completion | `F-014`'s outcome is an approval decision; `L3_Phase1_Discovery_Rules.md` §8 disqualifies "Approve Request" as a standalone candidate. `F-016`–`F-018` are materially different terminal results and stay separate | MEDIUM |
| `MG-08` | `C-056`, `C-057` | A guardian relationship is suspended and restored under authorised decision | Consecutive states of one journey — the pattern consolidated as `G-OV-06` for `G-125`/`G-126`. `C-058` Termination is terminal and stays separate | MEDIUM |
| `MG-09` | `G-113`, `G-114`, `G-150`, `G-170` — **retire** | Absorb into source parents | `G-FR-08` | MEDIUM |
| `MG-10` | `G-100`, `G-101` (retain `G-100` rewritten) | A hostel visit is authorised, conducted and closed with recorded entry and exit | Two halves of one visit; `G-100`'s `Critical`/`P0` also warrants review under §6 | MEDIUM |
| `MG-11` | `G-097`, `G-098` (retain `G-097` rewritten) | A resident's approved leave completes with verified return | Departure and return are one leave journey | MEDIUM |

### Splits

| ID | Candidate | Hidden outcomes | Confidence |
| --- | --- | --- | --- |
| `SPLIT-01` | `J-049` *Supplier Delivery to Governed Receipt, Inspection and Disposition* | Acceptance → goods available to inventory/asset processing, versus rejection/return → goods leave institutional custody and the commitment is restored. `L2-J05` line 246 states these as an explicit terminal branch: "Acceptance, **or** Rejection/Variance/Return". The trigger separately enumerates six delivery types. **Conditional on FIND-A08** — do not split if Section J adopts the broad altitude | MEDIUM |

No other over-broad candidate met the bar. `D-016` (timetable planning through publication) and `C-036` (qualification completion and conferment) were examined and **rejected as splits**: each has one unifying terminal outcome, and `L3_Phase1_Discovery_Rules.md` §7 names "Timetable Publication" and `L3_Scenario_Type_and_Priority_Rules.md` §6 names "Qualification Completion and Student Exit" as complete outcomes at that altitude.

### Duplicates and subsumption

| ID | Candidates | Assessment | Confidence |
| --- | --- | --- | --- |
| `DUP-01` | `G-096`, `G-108`, `G-110` | **Subsumption.** `G-096` *Occupancy Lifecycle to Authorised Check-Out* — outcome "Exit clearance is completed and the resident's occupancy is formally closed" — is the concatenation of `G-108` ("…exit is approved") and `G-110` ("Checkout is completed and the … occupancy and residency lifecycle are formally closed"). Same owner G03, same `Critical`/`P0`, overlapping domains. Three candidates for one hostel-exit outcome. `G-OV-05` consolidated the *entry* side (`G-092`/`G-094`/`G-095`) and did not reach the exit side. **Retain `G-096` rewritten; retire `G-108` and `G-110`.** | PROVEN |
| `DUP-02` | `I-074`/`I-096`, `I-079`/`I-102`, `I-092`/`I-121` | Three pairs with byte-identical Final Outcomes, surfaced by `check_l3_inventory.py`. Practical-vs-theory conduct in different L2 documents (`L2-I06`, `L2-I07`), so not automatically duplicates — but they are currently indistinguishable by outcome. Sits inside `I-TV-03`; resolve with it | PROVEN (identity); OPEN (disposition) |

### Remain separate despite superficial similarity

| ID | Candidates | Why the distinction matters |
| --- | --- | --- |
| `SEP-01` | `A-011`, `A-012`, `A-013`, `A-014` | `L2-A04` §9 declares Delegation, Temporary Access Grant and Break-Glass Session as three distinct canonical aggregates with different lifecycles and approval models. Already recorded as `A-C03`; re-confirmed here |
| `SEP-02` | `C-039` Withdrawal, `C-040` Transfer, `C-041` Discontinuation, `C-042` Termination | Superficially a trigger-variant family — same owner, domains, Type and Priority, near-identical outcome phrasing. **Do not merge.** Each reaches a different terminal Student Context state with materially different institutional consequence, and `L3_Scenario_Type_and_Priority_Rules.md` §6 names "Student Termination" as a standalone Critical example. `C-044` is the exception — see FIND-A13 |
| `SEP-03` | `F-016`, `F-017`, `F-018` | Cancellation, failure recovery and post-completion reversal are three different terminal financial results, not steps |

### Deferred clusters independently reviewed, no change to disposition

`I-TV-01`, `I-TV-02`, `I-TV-03`, `I-TV-04` in `L3_Section_I_Phase2_Consolidation_Watchlist.md` §4B (≈90 rows) and the `OPEN` residual G01/G02 participation review in `L3_Section_G_Phase2_Consolidation_Watchlist.md` §4A. I reviewed the reasoning recorded for each and **agree these are genuine Phase 2 judgments, not deferred rule applications.** The §4B entry for `I-TV-04` is notably honest in recording that precedent exists both ways. These are not re-raised as findings.

---

## 9. Repository Conflict Register

Repository defects, separated from inventory defects. **None is resolved here.**

| ID | Sources | Positions | Affected candidates | Authority analysis | Phase 2 blocked? | Escalate to |
| --- | --- | --- | --- | --- | --- | --- |
| `RC-01` Delegation ownership | `L2-A04` line 114; `L2-E02` lines 359, 975, 2644, 2662 | **A:** A04 "exclusively owns Role, Role Assignment, **Delegation** and authorization-policy lifecycles", approved under INT-301. **B:** E02 declares `Assignment Delegation \| Canonical` with its own `DelegateAuthority` command and `PositionAssignmentDelegated` event | `A-012`, `E-103`, **and `G-362`** (FIND-A07) | **Unresolved by hierarchy.** Both are locked L2 documents at the same tier. A04's dated approval is suggestive but does not override E02's canonical declaration plus command and event ownership. A split reading exists but **no repository source states it**, and A04's exclusivity wording forecloses it | **Yes** | The authority that approved INT-301 |
| `RC-02` Question-paper governance ownership | Recorded in the Section I register | Needs a new owner or a new L2 document; cannot be ruled between existing sources | Section I question-paper candidates incl. `I-111`, `I-112`, `I-113` | Cannot be decided from current repository evidence | **Yes** | Lead Architect |
| `RC-03` Policy Management ownership | `DEPENDENCY_MAP` §7 | Needs Lead Architect sign-off | Section A/B policy candidates | Cannot be decided from current repository evidence | **Yes** | Lead Architect per `DEPENDENCY_MAP` §7 |

**Inventory handling of these conflicts is correct.** RC-01 is recorded in two watchlists with both positions quoted at line level, an explicit "**No side has been chosen**" statement, and no candidate silently re-owned. I verified this directly against `L3_Section_A_Phase2_Consolidation_Watchlist.md` §9 and `L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md` §4. No section was penalised for these.

The **only** inventory-side defect attaching to any conflict is FIND-A07 — RC-01's affected-candidate list omits `G-362`.

Preserved-and-unresolved section findings reviewed and confirmed as correctly held open, not silently reconciled: `FIND-D-01`…`D-08`, `BDE-B-01`, `CF-01` (admission rejection state), `CF-05` (alumni boundary), `G-OI-01`…`G-OI-03`.

---

## 10. Cross-Section Findings

**CS-01 — Consolidation altitude is not uniform across sections.** *Severity: HIGH. Confidence: PROVEN.*

Candidates per L2 workflow document:

| Section | Docs | Candidates | Per doc |
| --- | ---: | ---: | ---: |
| F | 14 | 85 | 6.1 |
| A | 10 | 62 | 6.2 |
| B | 5 | 32 | 6.4 |
| D | 7 | 57 | 8.1 |
| H | 10 | 85 | 8.5 |
| C | 8 | 74 | 9.3 |
| E | 7 | 75 | 10.7 |
| J | 10 | 110 | 11.0 |
| I | 16 | 218 | 13.6 |
| **G** | **10** | **377** | **37.7** |

The count is corroboration, not the finding — `L3_Phase1_Discovery_Rules.md` sets no target and a 377-row section may be correct. The finding is that the **qualitative** evidence points the same way: every one of the seven surviving §8-disqualified rows in the repository is in Section G (FIND-A01); the read-model class retired in G10 four days ago still stands in G09 (FIND-A02); and the entry side of the hostel journey was consolidated while the exit side was not (`DUP-01`). Section G was audited hardest and consolidated most, and still carries the most residue — because the passes were organised by cluster rather than swept across the section.

**Impact.** A Phase 2 consolidation starting now would produce an L3 set in which a Campus Operations scenario and a Finance scenario mean different things. No repository rule reconciles them.

**Recommendation.** Before Phase 2 consolidation begins, adopt a written altitude standard — the `G-FR-05`/`G-FR-08`/`G-FR-12` principles already have repository acceptance and would serve — and sweep all ten sections against it once, rather than cluster by cluster.

**CS-02 — Primary Owner convention split.** See FIND-A06. 929 of 1,175 rows.

**CS-03 — No duplicate canonical ownership found across sections.** Verified: `B-991`–`B-993` Position Definition (B02) against `E-015`–`E-017` Position Assignment (E02) — confirmed repository-correct against `L2-B02` "Approved Position Definition Ownership and Lifecycle Clarification" and `L2-E02` line 76, and correctly recorded as an ownership check that passed. `B-006`–`B-010` InstitutionalCampus (B03) against `G-450`–`G-453` Physical Campus (G10) — reciprocal statements, no overlap. Section F monetary ownership, Section I result ownership and Section J resource ownership are not duplicated elsewhere; `E-062`, `E-073`, `D-037`, `D-045`, `G-113`, `G-170` all stop at handoff. **No dependency reversal found.** Section H does not become a business owner for any source-domain outcome.

---

## 11. Clean-Coverage Summary

| Area | Checked | Result |
| --- | --- | --- |
| Section A | 62 candidates, `L2-A01`–`L2-A10` | No material exception. Strongest section: outcome-based titles throughout, `VAR:<role>` used precisely and only where §11 permits, five `Simple` rows present. The 74% `Critical` rate is rule-supported — §6 lists "institutional audit trail" and "legal/compliance evidence" explicitly |
| Section B | 32 candidates, `L2-B01`–`L2-B05` | No material exception. Ownership boundaries verified against source. `B-991`–`B-996` ID range is unusual but §6 permits any `NNN` and §18 makes Phase 1 IDs temporary |
| Section D | 57 candidates, `L2-D01`–`L2-D07` | No material exception. `D-020` matches the canonical attendance example in §9 |
| Section E | 75 candidates, `L2-E01`–`L2-E07` | No material exception beyond RC-01 and the Type distribution reflected in scoring |
| Section F | 85 candidates, `L2-F01`–`L2-F14` | Coverage complete across all fourteen documents. Sole material exception is FIND-A04 |
| Section I | 218 candidates, `L2-I01`–`L2-I16` | Coverage complete across all sixteen documents — the most thorough coverage in the repository. Exceptions are titling (FIND-A03) and classification (FIND-A05), not coverage |
| Section J | 110 candidates, `L2-J01`–`L2-J10` | Coverage complete. Sole material exception is altitude (FIND-A08) |
| Implementation leakage | all 1,175 rows | **None.** No API, schema, class, DTO, payload, framework, deployment, screen, button or navigation reference in any candidate. `H-053` (personal dashboard layout) is the closest approach and is flagged in FIND-A10, not as leakage |
| Invented architecture | all 1,175 rows | **None found.** No fabricated state, event, aggregate, owner or dependency detected in any row inspected |
| Candidate ID conformance | all 1,175 rows | 0 errors. All match `CAN-L3-<AREA>-<NNN>`; no duplicates within or across files; no final L3 ID assigned; sparse ranges are documented consolidation traces |
| Locked field format | all 1,175 rows | 0 errors. All eight fields present on every row; all Type and Priority values within the allowed sets |

---

## 12. Overall Score

Arithmetic mean of the ten section scores, unweighted by candidate count:

(9.4 + 9.2 + 9.0 + 9.2 + 9.1 + 8.7 + 7.6 + 8.5 + 8.2 + 9.0) ÷ 10 = 87.9 ÷ 10

## **8.8 / 10.0**

Interpretation: **good discovery requiring meaningful corrections.** The gap to 9.0+ is consolidation altitude and classification, not coverage or invention.

---

## 13. Phase 2 Readiness Verdict

## **BLOCKED — REPOSITORY DECISION REQUIRED**

Three repository conflicts — `RC-01`, `RC-02`, `RC-03` — cannot be resolved from any existing source and each determines the owner of real candidates. Phase 2 consolidation cannot close over them. The score of 8.8 does not override this; readiness and score are separate.

Sections **B, D, F, J and A** are individually ready for Phase 2 consolidation subject to their watchlist `OPEN` items, with A and B carrying RC-01 and RC-03 exposure. Sections **G, H and I** need a consolidation pass before they can be consolidated against the others. Sections **C and E** are ready subject to RC-01.

---

## 14. Prioritized Correction Recommendations

**Before Phase 2 consolidation begins**

1. **Escalate RC-01, RC-02 and RC-03.** RC-01 to the authority that approved INT-301; RC-02 and RC-03 to the Lead Architect. Nothing downstream is safe to lock until these return. *(Blocker)*
2. **Add `G-362` to RC-01's affected-candidate list** in both watchlists. Two lines. Do not choose a side. *(FIND-A07)*
3. **Adopt a written altitude standard and sweep all ten sections against it once.** The `G-FR-05`, `G-FR-08` and `G-FR-12` principles already have repository acceptance; write them down as a rule rather than as per-section watchlist rows, and apply them repository-wide in one pass. This is the single highest-value action in this report. *(CS-01)*

**Correction pass**

4. **Section G:** apply `MG-01` (7 rejection rows), `MG-05` (2 read-model rows), `DUP-01` (hostel exit subsumption), `MG-09`–`MG-11`. *(FIND-A01, FIND-A12, HIGH)*
5. **Section G09:** run the G09 consolidation pass — `MG-02`, `MG-03`, `MG-04`. Expect roughly a halving of the 47 rows. *(FIND-A02, HIGH)*
6. **Section I:** retitle the 149 imperative-verb rows to outcome form. Mechanical, bounded, and a precondition for meaningful §9 boundary review. *(FIND-A03, HIGH)*
7. **Sections F and I:** Lead Architect re-derives Type and Priority under §7 and §12. ~300 values, already scoped in both watchlists. *(FIND-A04, FIND-A05, HIGH/MEDIUM)*

**Consistency pass**

8. **Normalise Primary Owner** in B, D, E, F, G, H and I to business-capability names, matching the convention adopted for A, C and J. *(FIND-A06, MEDIUM)*
9. **Section J:** decide the altitude, then apply either `SPLIT-01` or `MG-06` — not both. *(FIND-A08, MEDIUM)*
10. **Section H:** apply `VAR:<role>` to `H-018` and `H-085`; rule on the H03/H07 query and projection rows and record the ruling either way. *(FIND-A09, FIND-A10, MEDIUM)*

**Hygiene**

11. Correct the Section G watchlist header (391 → 377) and amend its line 169 "no open decision" claim. *(FIND-A11, LOW)*
12. Rewrite or retire `C-044`. *(FIND-A13, LOW)*
13. **Tooling, not inventory:** extend `FRAGMENT_VERBS` in `check_l3_inventory.py` so the FIND-A03 class becomes detectable. Owner's call, outside this audit's scope.

---

## 15. Audit Limitations / Items Not Verified

Recorded so the report's reach is not overstated.

* **Sampling.** All 1,175 candidate rows were read and swept programmatically for every pattern reported here. **Deep per-candidate L2 verification was targeted, not exhaustive** — L2 originals were opened for Sections D, F, G and J where a specific finding required proof. Sections A, B, C, E, H and I were assessed against their candidate content, their watchlists and their L2 document manifests. A deeper source pass on those six could surface further findings.
* **Section G09 fragmentation is characterised, not enumerated.** FIND-A02 names 15 representative rows out of 47. The full cluster boundaries need the G09 pass itself to fix.
* **Type and Priority were not re-derived.** `L3_Scenario_Type_and_Priority_Rules.md` §15 reserves this for Lead Architect review. FIND-A04 and FIND-A05 assess the *distribution* against §14, which is a rule the distribution can be tested against; they do not assert a correct value for any individual row.
* **`L2-G09` was not opened.** FIND-A02 rests on the candidate rows' own content — outcomes reading "a record is recorded", "the transition is rejected", "the read model reflects" — and on the watchlist's own `G-FR-12` precedent. Whether `L2-G09` itself is authored at an unusually fine granularity was not checked, and would not change the L3 boundary conclusion in any case: `L3_Phase1_Discovery_Rules.md` §7 makes L2 workflows evidence sources, not automatic L3 boundaries.
* **RC-02 and RC-03 were taken from the existing registers, not independently re-derived.** RC-01 was verified directly against both watchlists' quoted `L2-A04` and `L2-E02` line references.
* **The 2026-08-14 correction application was not re-verified row by row.** This audit assesses the *current state* of the inventories. Where a correction is cited (FIND-A01, FIND-A02, FIND-A06, FIND-A09), the citation establishes that a principle was accepted — the finding itself rests on rows I read in the current files.
* **`L3_Phase1_Inventory_Pallav_PreConsolidation_2026-08-13.md`** (1,080 rows) and the deleted 954-row variant catalogue were not examined. Only the current 164-row inventory was audited.
* **No L0, L1, Dependency Map or Architecture Index review was performed** beyond what specific findings required. A full authority-hierarchy consistency audit is outside this scope.

---

**Files written by this audit:** this report only. No architecture file, inventory, governance document, watchlist, index, matrix or template was modified. No repository conflict was resolved. No candidate was added, removed, merged, split, renumbered or re-owned. No final L3 scenario ID was assigned. No L3 scenario document was created.
