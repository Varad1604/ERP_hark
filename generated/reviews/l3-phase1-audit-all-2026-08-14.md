# L3 Phase 1 Discovery Audit — All Sections


> **Numbering note (2026-08-17).** Candidate IDs in this historical report reflect the numbering in force when it was written. Repository-wide candidate numbering was normalized on 2026-08-17; for current IDs see `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.
**Audit type:** Independent adversarial audit of the L3 Phase 1 candidate scenario inventories
**Audit scope:** `ALL` — Sections A through J, all five Phase 1 inventories, cross-domain/`X` and governance candidates
**Audit date:** 2026-08-14
**Mode:** Read-only. No architecture file, governance file or inventory was modified. This report is the only write.
**Auditor role:** Senior Enterprise Architecture Auditor / Adversarial L3 Scenario Discovery Reviewer

---

## 1. Audit Metadata

| Item | Value |
| --- | --- |
| Scope input | `AUDIT_SCOPE = ALL`, `TEAM_MEMBER = N/A`, `SECTION = N/A` |
| Scope validation | Valid. The prompt's discovery split was confirmed against `L3_Phase1_Workload_Distribution_Decision.md` §2 and matches exactly. No contradiction; audit proceeded. |
| Inventories audited | 5 |
| Candidate rows checked | **1,215** |
| Sections audited | 10 (A–J) |
| L2 workflow documents in scope | 104 across sections A–J |
| Conformance checker | `check_l3_inventory.py` executed read-only: PASS, 0 errors, 221 warnings |
| Report path | `generated/reviews/l3-phase1-audit-all-2026-08-14.md` |

---

## 2. Scope Actually Audited

| Inventory | Discovery areas | Rows | Distinct outcomes |
| --- | --- | ---: | --- |
| `L3_Phase1_Inventory_Chetan.md` | C | 71 | 71 (100%) |
| `L3_Phase1_Inventory_Devang.md` | I | 245 | 242 (99%) |
| `L3_Phase1_Inventory_Dhruv.md` | A (62), J (110) | 172 | 172 (100%) |
| `L3_Phase1_Inventory_Pallav.md` | B (32), D (57), E (75) | 164 | 164 (100%) |
| `L3_Phase1_Inventory_Varad.md` | G (391), F (87), H (85) | 563 | 563 (100%) |

**Cross-domain / `X` area:** no candidate anywhere in the repository uses the `CAN-L3-X-nnn` area code. This is not a defect — `L3_Naming_Numbering_Standard.md` §5 reserves `X` for cases where the likely primary owner cannot be safely identified, and every discovered candidate names a provable or provisional owner. Cross-domain outcomes are instead carried under their owner's area with `VAR:<role>` participation notation (Dhruv, Section A) or explicit multi-section `Domains` values.

**Governance and pilot framing:** governance candidates appear as owner-side governance outcomes inside their sections (`A-047`…`A-055`, `A-062`, `G-000` family, `H-065`…`H-075`). The procurement-to-inventory pilot framing is recorded in `L3_Phase1_Dhruv_Revised_Submission_Verification_2026-08-13.md`, not as inventory rows. Both are consistent with Phase 1 rules.

---

## 3. Sources Consulted

**Governance and standards**
`architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Discovery_Rules.md` (§4–§14, §16.1, §17);
`.../L3_Scenario_Type_and_Priority_Rules.md` (§3–§14);
`.../L3_Naming_Numbering_Standard.md` (§4–§7);
`.../L3_Phase1_Workload_Distribution_Decision.md` (§2, §4);
`.../L3_Section_A_Phase2_Consolidation_Watchlist.md`, `.../L3_Section_C_...`, `.../L3_Section_F_...`, `.../L3_Section_G_...`, `.../L3_Section_H_...`, `.../L3_Section_I_Phase2_Consolidation_Watchlist.md`.

**L2 sources opened for targeted verification**
`L2-A04_Authorization_and_Policy_Workflows.md` (line 114, §10.5);
`L2-B02_Organisation_Workflows.md` (line 110 ff.);
`L2-C01_Admissions_Workflows.md` (§6.4, §6.5, §6.6, §7.3);
`L2-C02_Student_Lifecycle_and_Activation.md` (line 5406);
`L2-C03_Student_Academic_Registration_Workflows.md` (lines 947, 1212, 3018);
`L2-E02_Position_and_Assignment_Workflows.md` (lines 76, 359, 975, 2644, 2662);
`L2-E03_Faculty_Workforce_Workflows.md`;
`L2-I03_Examination_Planning_and_Scheduling_Workflows.md` (line 458);
`L2-I07_Theory_Examination_Conduct_Workflows.md` (lines 98, 835).

**Repository authority**
`DEPENDENCY_MAP.md` §7 (line 409); `architecture/L2_Domain_Workflow_Architecture/` section indexes A–J.

**Prior reviews consulted for traceability, not treated as authority**
`generated/reviews/l3-phase1-section-c-verification-2026-08-09.md`;
`generated/reviews/L3_Phase1_Audit_Pallav_Correction_Application_2026-08-14.md` (referenced by the inventory under audit).

---

## 4. Executive Verdict

The Phase 1 discovery effort is, on the whole, **substantially better than a first-pass discovery inventory**. Four of five inventories have already been through an independent audit and an authorised consolidation, and the evidence of that work is real: Section F went from 1,858 rows to 87 outcome-level candidates, Section H from 893 to 85, Section G from 541 to 391, Section I from 304 to 245, and Pallav's B/D/E from a 1,080-row submission to 164. Every one of the 104 L2 workflow documents in Sections A–J has candidate coverage. The mechanical conformance checker passes with zero errors across all 1,215 rows. Ownership discipline is genuinely strong: I found no case anywhere in the repository where two inventories claim the same canonical business outcome for two different owners, and the delicate boundaries the prompt flagged in advance — B03 institutional Campus versus G10 physical infrastructure, Section F monetary ownership, Section I result ownership, Section J resource ownership — are all handled correctly and explicitly.

Against that, four problems are material.

**First, Section C's inventory and its governance record disagree about what Section C actually contains.** `L3_Section_C_Phase2_Consolidation_Watchlist.md` §6 states Section C holds 74 candidates after application, names `CAN-L3-C-072`, `-073` and `-074` as added, and records a split of `C-011`. The inventory holds 71 rows, stops at `C-071`, and `C-011` still carries its pre-split title. Three evidence-backed institutional outcomes — including the entire admission initiation-to-approval decision span — exist in governance but not in the repository's candidate set. Phase 2 cannot consolidate a section whose authoritative row set is ambiguous.

**Second, a canonical ownership conflict over Delegation is live and unrecorded.** `L2-A04` line 114 asserts, under the approved INT-301 boundary, that A04 "exclusively owns Role, Role Assignment, Delegation and authorization-policy lifecycles". `L2-E02` lists `Assignment Delegation` as Canonical, owns a `DelegateAuthority` command and a `PositionAssignmentDelegated` event. `CAN-L3-A-012` and `CAN-L3-E-103` each proceed as though its own side owns delegation. Neither inventory flags the collision. This is exactly the class of defect the deferred A/J-versus-B/D/E peer comparison was supposed to catch.

**Third, granularity is uneven in a way the scores must reflect.** Sections B, C, D, E, F and H are at or near outcome level. Sections I and G are not. Section I carries 208 verb-initial titles out of 245 and contains candidates that map one-for-one onto the examples `L3_Phase1_Discovery_Rules.md` §8 explicitly disqualifies — `I-277 Notify Candidate of Malpractice Allegation`, `I-141 Approve Evaluated Marks for Result Processing`, `I-273 Preserve Examination Evidence`. Section G retains attribute-level CRUD rows and event-delivery/read-model rows in G10. Both sections have already been consolidated once; what remains is a second, smaller, but still real consolidation obligation.

**Fourth, Type and Priority are being applied as domain labels rather than as risk and release judgments.** Section F is 79 `Critical` out of 87 with zero `Simple` and zero `P2`. Section I is 220 `Critical` out of 245. Section J10 is 17 out of 17 `Critical`. `L3_Scenario_Type_and_Priority_Rules.md` §14 Mistake 2 names this precise pattern — "All Finance scenarios = Critical" — as incorrect. Separately, all 110 Section J rows use L2 subdomain codes (`J01`…`J10`) in the `Domains` field where `L3_Phase1_Discovery_Rules.md` §11 requires section letters, and that non-conforming convention has directly produced 11 false `Cross-domain` classifications from what is a single business domain.

None of these is a fatal flaw in the discovery work. Sections B and F–H in particular are strong. But the Section C divergence, the Delegation conflict, and two pre-existing unresolved ownership questions preserved from earlier audits (question-paper governance, Policy Management) together make the ownership or existence of material scenarios indeterminate, which is the definition of a blocker.

---

## 5. Section Scorecard

| Section | Owner | Rows | Coverage /2.5 | Granularity /2.0 | Ownership & domains /2.0 | Fields, Type, Priority /1.5 | Conflict handling /1.0 | Governance /1.0 | **Total /10.0** |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| A — Platform Foundation | Dhruv | 62 | 2.2 | 1.8 | 1.7 | 1.1 | 1.0 | 0.9 | **8.7** |
| B — Institution & Structure | Pallav | 32 | 2.3 | 1.8 | 1.9 | 1.2 | 1.0 | 0.9 | **9.1** |
| C — Student Lifecycle | Chetan | 71 | 1.3 | 1.9 | 1.8 | 1.0 | 0.8 | 0.6 | **7.4** |
| D — Academic Delivery | Pallav | 57 | 2.2 | 1.7 | 1.8 | 1.2 | 1.0 | 0.6 | **8.5** |
| E — People & Workforce | Pallav | 75 | 2.2 | 1.8 | 1.7 | 1.1 | 0.7 | 0.9 | **8.4** |
| F — Finance | Varad | 87 | 2.3 | 1.9 | 1.6 | 0.8 | 1.0 | 0.9 | **8.5** |
| G — Campus Operations | Varad | 391 | 2.4 | 0.9 | 1.4 | 0.8 | 1.0 | 0.8 | **7.3** |
| H — Institutional Services | Varad | 85 | 2.3 | 1.9 | 1.4 | 1.0 | 1.0 | 0.9 | **8.5** |
| I — Examination | Devang | 245 | 2.4 | 0.8 | 1.7 | 0.8 | 1.0 | 0.9 | **7.6** |
| J — Enterprise Resource Mgmt | Dhruv | 110 | 2.2 | 1.5 | 1.0 | 0.8 | 0.9 | 0.9 | **7.3** |

Scoring notes:

- No section was credited or penalised for candidate count. Section G scores 2.4 on coverage — the highest in the table — while scoring 0.9 on granularity. Section B scores highest overall on 32 rows.
- Sections C, F, G, H and I were **not** penalised for the upstream repository conflicts their watchlists correctly record and preserve (`CF-01`…`CF-09`, `I-RC-01`…`I-RC-07`, `F-DU-*`, `G-FR-*`). Section C was penalised on `Conflict handling` and `Governance` because the *inventory* diverges from the governance record of what was applied — a different defect from the conflicts themselves.
- Section E lost `Conflict handling` credit solely for the unflagged A04/E02 Delegation collision (FIND-002).
- Section D lost `Governance` credit for two self-contradictions inside its own file (FIND-010), not for its content.

---

## 6. Findings Register

### FIND-001 — Section C inventory contradicts its own approved governance record

| | |
| --- | --- |
| **Severity** | BLOCKER |
| **Section / L2 area** | C — `L2-C01`, `L2-C02`, `L2-C03` |
| **Affected candidates** | `CAN-L3-C-011`; absent `CAN-L3-C-072`, `CAN-L3-C-073`, `CAN-L3-C-074` |
| **Evidence** | `L3_Section_C_Phase2_Consolidation_Watchlist.md` §6 records "Candidates after application: **74**", "Added (`072`, `073`, `074`) +3"; its header scopes the section as `CAN-L3-C-001` … `CAN-L3-C-074`. `L3_Phase1_Inventory_Chetan.md` contains 71 rows with maximum ID `CAN-L3-C-071`. `C-011` retains its pre-split title "Intra-Institution Student Transfer" (line 30), whereas the source review executed a split into `C-011` (retitled *Intra-Institute Student Academic Reassignment*) + `C-074`. |
| **Problem** | The governance layer records three additions and one split as applied. None is present in the inventory. The watchlist further relies on the missing rows: `CF-01` states the admission rejection decision "is carried inside `CAN-L3-C-072`", and §5 cites `C-074` as a passed boundary check. Both statements are false against the current repository. |
| **Repository impact** | The authoritative Section C candidate set is indeterminate. `CF-01`'s recorded mitigation does not exist. Three institutional outcomes with cited L2 evidence are absent from Phase 1 (see §7). |
| **Recommendation** | Determine which artifact is authoritative — apply the three additions and the `C-011` split to the inventory, or withdraw the §6 application record and reopen `CF-01`. Do not leave both standing. Lead Architect decision. |
| **Confidence** | PROVEN |

### FIND-002 — Delegation ownership conflict between L2-A04 and L2-E02 is unrecorded in both inventories

| | |
| --- | --- |
| **Severity** | BLOCKER |
| **Section / L2 area** | A (`L2-A04`) and E (`L2-E02`) |
| **Affected candidates** | `CAN-L3-A-012` (Responsibility Delegation to Automatic Authority Reversion, owner *Authorization & Policy*); `CAN-L3-E-103` (Delegated Authority and Additional Charge Assignment, owner *E02*) |
| **Evidence** | `L2-A04_Authorization_and_Policy_Workflows.md` line 114: "**Canonical boundary approved 2026-08-13 under INT-301.** L2-A04 **exclusively owns** Role, Role Assignment, **Delegation** and authorization-policy lifecycles." §10.5: "Delegation represents temporary transfer of business responsibility from one authorized user to another." — `L2-E02_Position_and_Assignment_Workflows.md` line 359: `Assignment Delegation | Canonical`; line 975: `Delegation | L2-E02 | Operational delegation`; line 2644 command `DelegateAuthority`; line 2662 event `PositionAssignmentDelegated`. |
| **Problem** | Two locked L2 documents each assert canonical ownership of a lifecycle both call "Delegation". A04's wording is exclusionary and post-dates INT-301 approval; E02's is a canonical-concept declaration backed by its own command and event. A reading exists in which A04 owns authorization-layer delegation and E02 owns organisational-assignment delegation, but no repository source states that split, and A04's "exclusively owns" wording forecloses it. Each inventory proceeded on its own side's reading without recording the collision, which `L3_Phase1_Discovery_Rules.md` §14 forbids ("silently resolve repository conflicts"). |
| **Repository impact** | Ownership of a material scenario family is indeterminate. If A04's exclusivity is upheld, `CAN-L3-E-103` loses its owner; if E02's canonical claim is upheld, A04's INT-301 wording requires source correction. |
| **Recommendation** | Do **not** resolve inside Phase 1. Escalate to the authority that approved INT-301. Either amend `L2-A04` line 114 to scope its exclusivity to authorization delegation, or amend `L2-E02` to consume A04 delegation. Record the conflict in the Section A and a new Section E watchlist meanwhile. |
| **Confidence** | CONFLICTED — both positions preserved, no side chosen |

### FIND-003 — Section J `Domains` field violates §11 in every row, producing 11 false Cross-domain classifications

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | J — all of `L2-J01`…`L2-J10` |
| **Affected candidates** | All 110 `CAN-L3-J-*` rows. Type consequence: `J-004`, `J-005`, `J-007`, `J-008`, `J-009`, `J-011`, `J-012`, `J-013`, `J-014`, `J-036`, `J-044` |
| **Evidence** | `L3_Phase1_Discovery_Rules.md` §11: "The Domains field must list participating areas **using section letters** or governed variable-participant tokens… Examples: `A, C, F` / `F, J`". All 110 Section J rows instead carry L2 subdomain codes — e.g. `CAN-L3-J-004` Domains = `J01, J02, J03`. `check_l3_inventory.py` raises the same warning ("Domains uses subdomain codes rather than section letters"). |
| **Problem** | The convention breach is not cosmetic. `L3_Scenario_Type_and_Priority_Rules.md` §5 requires `Cross-domain` to mean collaboration between **multiple business domains**. The eleven candidates listed above are typed `Cross-domain` while every token in their `Domains` field resolves to Section J — one business domain. By §7 they should be `Simple` or `Critical`. Section A, written by the same author, uses section letters correctly, so the inventory is also internally inconsistent. |
| **Repository impact** | Phase 2 cannot compute cross-domain participation for Section J without re-deriving every row, and the 11 mistyped candidates will mis-sequence cross-domain consolidation work. |
| **Recommendation** | Normalise Section J `Domains` to section letters (`J`, plus `F`, `G` where genuinely present), then re-derive Type for the 11 affected candidates under §7. |
| **Confidence** | PROVEN |

### FIND-004 — Section I contains candidates that are workflow steps under the §8 prohibited-example list

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | I — `L2-I02`, `L2-I07`, `L2-I08`, `L2-I14` |
| **Affected candidates** | `I-013`, `I-017`, `I-114`, `I-115`, `I-118`, `I-130`, `I-141`, `I-142`, `I-143`, `I-273`, `I-276`, `I-277`, `I-278`, `I-279` (representative; 208 of 245 titles are verb-initial) |
| **Evidence** | `L3_Phase1_Discovery_Rules.md` §8 disqualifies "Send Notification", "Approve Request", "Save Application", "Update Student Status" as standalone L3 candidates. Direct matches in the inventory: `I-277 Notify Candidate of Malpractice Allegation`; `I-141 Approve Evaluated Marks for Result Processing`; `I-273 Preserve Examination Evidence for Malpractice Investigation`; `I-278 Receive Candidate Response to Malpractice Allegation`; `I-114 Distribute Authorized Question Papers`; `I-115 Collect Candidate Answer Scripts`. |
| **Problem** | These are consecutive steps of larger business journeys, not independent institutional outcomes. `I-270`→`I-271`→`I-272`→`I-273`→`I-274`→`I-276`→`I-277`→`I-278`→`I-279`→`I-280`→`I-305`→`I-306` is one malpractice journey rendered as twelve rows. `I-114`/`I-115`/`I-118` are three steps inside `I-093 Conduct a Scheduled Theory Examination`. `I-013`/`I-014`/`I-015`/`I-017` are lifecycle transitions of the `I-011` Examination Session. |
| **Repository impact** | Section I's 245 rows overstate the number of L3 documents by a material margin. Phase 2 consolidation for Section I is substantially larger than its watchlist §4 currently projects ("245 is close to the defensible target of 200–230"). |
| **Recommendation** | Run a second consolidation pass on Section I against §8 specifically, not against the L2 workflow catalogues. See the merge register in §8. Note the L2 sources define these as workflows — that is evidence of business activity, not proof of an independent L3 boundary (`L3_Phase1_Discovery_Rules.md` §7–§8). |
| **Confidence** | PROVEN |

### FIND-005 — Section G retains attribute-level and mechanism-level candidates after its first consolidation

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | G — `L2-G10`, `L2-G01` |
| **Affected candidates** | Attribute CRUD: `G-453`, `G-454`, `G-458`, `G-460`, `G-461`, `G-463`, `G-465`, `G-467`, `G-469`, `G-470`, `G-471`, `G-473`, `G-474`. Mechanism rows: `G-523`, `G-524`, `G-525`, `G-538`. Step rows: `G-027`, `G-051`, `G-052` |
| **Evidence** | `G-460 Building Attribute Change to Effective-Dated Update` — final outcome "The Building reflects the new effective attributes"; `G-465` is the identical pattern for Space; `G-454` for Physical Campus; `G-473` for Capacity; `G-524 Infrastructure Availability Change Event to Consumer Resource Reassessment` — final outcome "Consumers receive the authoritative event"; `G-525 Infrastructure Service Contract Request to Authorised Infrastructure Information` — a read contract; `G-027 Technician Assignment to Accepted Maintenance Work` — final outcome "a technician accepts the assignment and is ready to execute the work". |
| **Problem** | Effective-dated attribute maintenance repeated once per hierarchy level is one governed outcome per aggregate at most, not one per attribute class. `G-524` and `G-538` are event delivery and read-model reconciliation — the mechanism `L2-A06` owns, and the same class of row the section's own watchlist already retired under `G-FR-08` and `G-FR-09` for notifications and dashboards. `G-027` is a mid-journey step of `G-025 Facility Work Order to Verified Closure`, as are `G-051` (verification) and `G-052` (verification failure). |
| **Repository impact** | Section G remains the most fragmented section in the repository at 391 rows against 11 L2 documents, and its granularity is inconsistent with Sections F and H authored by the same discoverer. |
| **Recommendation** | Extend the `G-FR-08`/`G-FR-09` reasoning already accepted for notifications and dashboards to (a) G10 effective-dated attribute maintenance, (b) G10 event/read-model rows, (c) G01 work-order internal steps. |
| **Confidence** | PROVEN |

### FIND-006 — The admission initiation-to-approval span has no candidate in any inventory

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | C — `L2-C01` |
| **Affected candidates** | Gap between `CAN-L3-C-001` and the start of the admission lifecycle |
| **Evidence** | `L2-C01` §6.4 Primary Lifecycle: `Admission Initiated → Documents Submitted → Verification Completed → Admission Approved → Financial Clearance → Admission Confirmed → Student Created → Admission Completed`. §7.3 catalogue defines `WF-C01-02 Document Verification` and `WF-C01-03 Admission Approval` as canonical workflows. `CAN-L3-C-001` trigger reads "Admission **is approved** and the candidate satisfies the requirements for transition into the student lifecycle" — it begins after the span. `C-002`, `C-003`, `C-004`, `C-006` are alternate terminal outcomes only; `C-005` is readmission. A repository-wide grep for "document verification", "admission approval" and "verification completed" across all five inventories returns **0 matches**. |
| **Problem** | The institutional admission decision — verification of submitted evidence followed by an authority-chain approval, producing a published `AdmissionApproved` / `AdmissionRejected` fact — is a complete institutional business outcome with its own trigger, its own decision authority, and its own terminal result. It gates every downstream admission state and is not covered semantically by any existing candidate. |
| **Repository impact** | A P0-class outcome in the repository's flagship domain is undiscovered. Directly linked to FIND-001: the governance record shows this was identified and remedied as `CAN-L3-C-072`, but the remedy is not in the repository. |
| **Recommendation** | Restore or re-create the outcome. Proposed family: *Admission Document Verification and Approval Decision*, owner Admissions, participating `A, B, C`. Confirm against `L2-C01` §6.6 state definitions and `CF-02` (two incompatible admission state vocabularies) before drafting. |
| **Confidence** | PROVEN (positive source evidence + verified absence) |

### FIND-007 — Type is being assigned by domain rather than by institutional risk

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | F, I, J10 |
| **Affected candidates** | Section F: 79 of 87 `Critical`, **0** `Simple`, **0** `P2`. Section I: 220 of 245 `Critical`. Section J10: 17 of 17 `Critical` (`J-112`…`J-128`) |
| **Evidence** | `L3_Scenario_Type_and_Priority_Rules.md` §14 Mistake 2 states verbatim that "All Finance scenarios = Critical" and "All Examination scenarios = Critical" are **incorrect**, and that "classification depends on the specific business outcome". §7 gives the risk-first selection order. Counter-examples in the inventories: `F-075 Management Reporting Request to Governed Financial Report` and `F-076 Financial Definition Change to Approved KPI` carry no settlement, payroll or statutory consequence yet are typed `Critical`; `J-120 Authorised Inventory to Reconciled Donation Outcome` and `J-121 …External Transfer-Out…` are routine dispositions typed `Critical`. |
| **Problem** | Where a section's Type field is near-uniform, it carries no information and cannot be used to sequence Phase 2 or to size L3 authoring effort. Section F using zero `Simple` and zero `P2` values means two of the six governed tiers are unused across the entire domain. |
| **Repository impact** | Type and Priority cannot be relied on as Phase 2 inputs for F, I and J10. |
| **Recommendation** | Re-derive Type for Sections F, I and J10 against §7, outcome by outcome. Note that `L3_Scenario_Type_and_Priority_Rules.md` §15 reserves final Type/Priority for Lead Architect review — so this is a flagged re-derivation obligation, not an authoring error the discoverers must self-correct. The Section I watchlist already carries the Priority half of this as `OPEN`. |
| **Confidence** | PROVEN |

### FIND-008 — Section A/J cross-inventory verification rests on a premise that is now false

| | |
| --- | --- |
| **Severity** | HIGH |
| **Section / L2 area** | A, J versus B, D, E |
| **Affected candidates** | All 172 `CAN-L3-A-*` and `CAN-L3-J-*` rows |
| **Evidence** | `L3_Phase1_Inventory_Dhruv.md` line 28: "**The empty Pallav inventory** provides no B/D/E candidate rows to compare; that external availability limitation does not leave an A/J Phase 1 coverage defect." `L3_Section_A_Phase2_Consolidation_Watchlist.md` line 153 repeats it: "Pallav's B/D/E inventory remains empty, so those unavailable rows must be checked when populated." `L3_Phase1_Inventory_Pallav.md` currently contains **164 rows** across B, D and E, last revised 2026-08-14. |
| **Problem** | The A/J peer-collision check was explicitly deferred against 879 available peer rows and never re-run once the remaining 164 became available. FIND-002 (A-012 / E-103 Delegation) is a concrete collision that this deferred check would have been expected to surface. |
| **Repository impact** | A/J cross-inventory integrity is unverified against one third of the repository's candidate population. |
| **Recommendation** | Re-run the targeted peer comparison for all 172 A/J candidates against the 164 B/D/E rows and update both the inventory statement and the Section A watchlist. Priority pairs to check first: A-011/A-012 versus E-019…E-024, E-103; A-005/A-006 versus E-010, E-096; A-037…A-045 versus B-021, B-022. |
| **Confidence** | PROVEN |

### FIND-009 — Nineteen Section H candidates list all ten business domains

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | H — `L2-H03`, `L2-H07`, `L2-H08`, `L2-H09` |
| **Affected candidates** | `H-020`, `H-058`, `H-059`…`H-064`, `H-065`…`H-075` |
| **Evidence** | `L3_Phase1_Discovery_Rules.md` §11: "Use only domains that genuinely participate in the scenario. **Do not list all domains by default.**" Each of the nineteen rows carries `Domains = A, B, C, D, E, F, G, H, I, J`. |
| **Problem** | For H08 and H09 a case exists that regulatory reporting genuinely draws evidence from every domain — but §11 distinguishes participation in *producing the outcome* from *supplying a consumed reference*, and the candidates' own wording concedes the latter: `H-068` states evidence is "collected… **without copying canonical business ownership into H09**". A domain that supplies a referenced fact is a source, not a co-producer. |
| **Repository impact** | Cross-domain participation counts for Section H are unusable, and the blanket listing drives `Cross-domain` typing on rows such as `H-072`. |
| **Recommendation** | Replace the blanket list with the domains that materially co-produce each outcome, or adopt a governed `VAR:Evidence-Owning Domain` token as Section A already does. |
| **Confidence** | PROVEN |

### FIND-010 — Pallav's Section D notes contradict Section D's own rows

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | D |
| **Affected candidates** | Section D register as a whole; `CAN-L3-D-020` |
| **Evidence** | Line 62: "**The 38 candidates below** are the canonical Phase 1 Section D register." The table that follows contains **57** rows. — Line 128: "`CAN-L3-D-020` is recorded at its current boundary, **which ends at an authoritative attendance record**. Whether this candidate should be extended to Attendance Finalization is an open decision item, not a settled boundary." Line 234: "**Boundary extension.** `CAN-L3-D-020` **now ends at Attendance Finalization**". The row itself (line 89) is titled "…to Attendance Finalization" and its Final Outcome ends at "Finalized". |
| **Problem** | Two stale derivation notes were left in place after the 2026-08-14 corrections were applied. The "38" figure describes the pre-correction parent set; the D-020 note describes the pre-correction boundary. Both now read as current statements and contradict the register. |
| **Repository impact** | A Phase 2 reader taking the notes at face value would consolidate against the wrong row count and re-open a settled boundary. |
| **Recommendation** | Correct both notes to match the applied state, or move Section D's derivation history into a Section D watchlist as Sections A, C, F, G, H and I do. |
| **Confidence** | PROVEN |

### FIND-011 — Section G lists impacted domains as participating domains

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | G — `L2-G01`, `L2-G02` |
| **Affected candidates** | `G-022`, `G-023`, `G-024`, `G-026`, `G-032`, `G-033`, `G-034`, `G-044`, `G-045`, `G-046`, `G-047`, `G-051`, `G-052`, `G-055`, `G-058`, `G-076` (representative) |
| **Evidence** | `G-023 Duplicate Facility Complaints to Consolidated Resolution` carries `Domains = G, D, I` with a Final Outcome entirely internal to facilities: "Duplicate complaints are linked or merged while preserving historical references and one coordinated resolution path". `G-051 Facility Maintenance Verification to Closure` carries `G, D, I`. Neither Academic Delivery nor Examination performs any act in either outcome. |
| **Problem** | `D` and `I` appear to have been listed because academic and examination activity may be *affected* by a facility issue. `L3_Phase1_Discovery_Rules.md` §11 requires genuine participation; `L3_Scenario_Type_and_Priority_Rules.md` §5 warns against typing `Cross-domain` on the strength of incidental involvement. |
| **Repository impact** | Section G's cross-domain participation is overstated, inflating `Cross-domain` counts (141 of 391). |
| **Recommendation** | Apply the §11 participation test row by row across G01 and G02. Retain `D`/`I` only where the outcome cannot complete without an act by that domain. |
| **Confidence** | SUPPORTED INFERENCE — the outcome wording is the evidence; no L2-G01 statement asserts D/I participation in complaint consolidation |

### FIND-012 — Two Varad candidates are wholly subsumed by his own Section H candidates

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | F00 versus H02, H07 |
| **Affected candidates** | `CAN-L3-F-086` ⊂ `CAN-L3-H-010`; `CAN-L3-F-087` ⊂ `CAN-L3-H-050` |
| **Evidence** | `F-086 Financial Outcome to Governed Notification`, **Primary Owner `H02`**, outcome "H02 delivers the governed communication without taking ownership of the financial transaction". `H-010 Authoritative Business Outcome to Governed Communication Outcome`, owner `H02`, outcome "Eligible recipients receive the governed notification… without H02 owning the originating transaction". `F-087 Financial Reports and Projections to Governed Dashboard Composition`, **owner `H07`**, versus `H-050 Dashboard Requirement to Governed Published Dashboard`, owner `H07`. |
| **Problem** | Both rows carry a Section H primary owner while sitting under the F discovery area, and both describe the generic H02/H07 outcome specialised to a Finance source. `H-010` and `H-050` already cover any source domain. The publisher-side Finance outcomes are separately and correctly held by `F-077`. |
| **Repository impact** | Two candidates will resolve to the same final L3 documents as `H-010` and `H-050`. Note the same pattern was already retired for Section G under watchlist cluster `G-FR-08`/`G-FR-09` — Finance was not given the same treatment. |
| **Recommendation** | Retire `F-086` and `F-087` into `H-010` and `H-050`, consistent with the accepted `H-DEC-01` dashboard decision and the applied `G-FR-08`/`G-FR-09` clusters. |
| **Confidence** | PROVEN |

### FIND-013 — Alumni transition boundary overlaps between Sections C and H

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | C (`L2-C08`) versus H (`L2-H06`) |
| **Affected candidates** | `CAN-L3-C-069` versus `CAN-L3-H-041` |
| **Evidence** | `C-069 Completed or Eligible Exit Student to Alumni Transition`, owner *Alumni Transition*, trigger "qualification completion or another permitted student exit reaches the condition required for alumni transition", outcome "the resulting alumni engagement context is made available to the authoritative Alumni lifecycle". `H-041 Eligible Student Completion to Alumni Relationship`, owner `H06`, trigger "L2-C05 confirms completion or exit eligibility", outcome "A governed alumni relationship is established". |
| **Problem** | Same trigger condition, adjacent outcomes, two owners. Both are carefully worded to disclaim the other's ownership, which is good discipline, but the ordering and the hand-off point are not established anywhere. `L3_Section_C_Phase2_Consolidation_Watchlist.md` `CF-05` records that Alumni Transition ownership is already stated three ways across `L2-C04` §1.5, `L2-C05` §5/§11.10 and `L2-C08` §5.2. |
| **Repository impact** | Partial overlap, not a duplicate. Phase 2 must fix the boundary before either can be authored. |
| **Recommendation** | Record as a cross-inventory boundary item in both the Section C and a new Section H cross-reference, dependent on the `CF-05` resolution. Do not merge — the outcomes are genuinely distinct if the boundary is drawn. |
| **Confidence** | SUPPORTED INFERENCE |

### FIND-014 — Priority is compressed toward P0/P1 across the largest sections

| | |
| --- | --- |
| **Severity** | MEDIUM |
| **Section / L2 area** | F, G, I |
| **Affected candidates** | Section F: 55 `P0` / 32 `P1` / **0** `P2` of 87. Section G: 162 `P0` / 222 `P1` / 7 `P2` of 391. Section I: 41 `P0` / 203 `P1` / **1** `P2` of 245 |
| **Evidence** | `L3_Scenario_Type_and_Priority_Rules.md` §9: "Do not mark a scenario P0 only because the domain is important"; §11 defines `P2` for "rare, future, optional, institution-specific, or low-frequency scenarios". Candidates that read as `P2` but are not marked so: `G-461 Floor Registration to Building Hierarchy Extension` (`P0`), `F-079 Financial Evidence to Audit and Historical Reconstruction` (`P1`), `I-311 Apply and Release Legal Hold` is correctly `P2` — the only one in 245 rows. |
| **Problem** | 47 percent of Section G and 63 percent of Section F are marked first-release-blocking. If `P0` describes what the first L3 release cannot safely proceed without, these figures cannot all be true. |
| **Repository impact** | Priority cannot be used to sequence L3 authoring. |
| **Recommendation** | Re-derive Priority under §12 for F, G and I. The Section I watchlist already records this as `OPEN` and reserves it for Lead Architect review under §15; extend the same treatment to F and G rather than asking the discoverers to re-derive. |
| **Confidence** | PROVEN |

### FIND-015 — Section I watchlist header states a row count that does not match the inventory

| | |
| --- | --- |
| **Severity** | LOW |
| **Section / L2 area** | I |
| **Affected candidates** | None directly |
| **Evidence** | `L3_Section_I_Phase2_Consolidation_Watchlist.md` header: "candidates `CAN-L3-I-001` … `CAN-L3-I-313` (**258 live rows**…)". §4 of the same document: "Section I now carries **245 candidates**". Actual inventory count: **245**. |
| **Recommendation** | Correct the header to 245. |
| **Confidence** | PROVEN |

### FIND-016 — Dhruv's inventory mixes two Primary Owner conventions

| | |
| --- | --- |
| **Severity** | LOW |
| **Section / L2 area** | A versus J |
| **Affected candidates** | 62 Section A rows use business-capability names (`Platform & Tenant`, `Person & Identity`, `Audit Infrastructure`); 110 Section J rows use L2 document identifiers (`J01 — Procurement Planning & Sourcing`) |
| **Evidence** | `L3_Phase1_Discovery_Rules.md` §5 defines Primary Owner as "Likely domain owner of the final business outcome". `check_l3_inventory.py` warns: "Primary Owner mixes subdomain codes with free text; pick one convention". Pallav's inventory uses L2 codes consistently (`B01`, `D01`, `E02`); Chetan's uses capability names consistently. |
| **Recommendation** | Pick one convention per inventory. Either is defensible; the inconsistency is not. |
| **Confidence** | PROVEN |

### FIND-017 — Two inventories carry extended explanatory sections contrary to §16

| | |
| --- | --- |
| **Severity** | LOW |
| **Section / L2 area** | B/D/E and A/J |
| **Evidence** | `L3_Phase1_Discovery_Rules.md` §3: "Each inventory file must contain candidate scenario rows only"; §16: "The file should not include long explanations unless necessary." `L3_Phase1_Inventory_Pallav.md` carries ~45 lines of consolidation traceability, change notes, open items and preserved findings (lines 124–256). `L3_Phase1_Inventory_Dhruv.md` carries ~50 lines of governance and revision notes. |
| **Problem** | The watchlist pattern used by Sections A, C, F, G, H and I exists precisely to hold this material outside the inventory. Sections B, D, E and J have no watchlist, so the content had nowhere else to go. This is a structural gap, not authoring indiscipline — and FIND-010 shows the practical cost: notes inside the inventory went stale against the rows beside them. |
| **Recommendation** | Create `L3_Section_B_D_E` and `L3_Section_J` Phase 2 consolidation watchlists and move the narrative content there. |
| **Confidence** | PROVEN |

### FIND-018 — Section J10 disposal-method rows may be variants of one execution outcome

| | |
| --- | --- |
| **Severity** | LOW |
| **Section / L2 area** | J — `L2-J10` |
| **Affected candidates** | `J-115`, `J-117`, `J-118`, `J-119`, `J-120`, `J-121`, `J-122` |
| **Evidence** | All seven share the trigger form "*Inventory is authorised for \<method\>*" and the outcome form "*approved quantity leaves custody with evidence; inventory and financial consequences reconcile; the case closes*". `J-115` is explicitly defined as the residual method "not separately classified as destruction, scrap, sale, auction, donation, external transfer-out, recycling or external return" — i.e. the set is a method enumeration by construction. |
| **Problem** | Possible enumeration of one outcome across a fixed method list. Against this: destruction requires witnesses and a certificate, sale/auction produces proceeds and a buyer, donation produces an acknowledgement — genuinely different institutional evidence and different financial consequences. |
| **Repository impact** | Low. Either reading is defensible. |
| **Recommendation** | Phase 2 judgment. Do **not** merge without checking `L2-J10` for whether the methods carry distinct approval authorities and evidence obligations. Flagged rather than recommended. |
| **Confidence** | SUPPORTED INFERENCE — reported as a question, not a merge instruction |

---

## 7. Missing Coverage Register

Only evidence-supported missing outcome families are listed. No speculative additions were generated. All three below trace to FIND-001 — they were identified by the approved 2026-08-09 Section C verification, and I independently confirmed each against its L2 source.

| # | Section | L2 source | Missing business outcome | Why it is a complete L3 outcome | Closest existing candidate | Likely owner | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| MC-01 | C | `L2-C01` §6.4, §6.6, §7.3 (`WF-C01-02`, `WF-C01-03`) | **Admission Document Verification and Approval Decision** | Distinct institutional authority chain; produces its own published decision fact; gates every downstream admission state. `CAN-L3-C-001` begins after it by its own stated trigger. | `C-001` (starts after approval) | Admissions | HIGH |
| MC-02 | C | `L2-C03` §5.3 `Registration Hold — Full`; §6.4/§6.6 child entity `RegistrationHold` (line 1212); §12.8 `OverrideRegistrationHold()` (line 3018) | **Registration Hold Placement and Release** | Raised by a party outside the registration transaction, carries its own authorization and audit obligation, persists across registration attempts, and its release is a separately authorized act. Exists whether or not a registration is attempted. | `C-019`, `C-020` (registration transactions) | Academic Registration | HIGH |
| MC-03 | C | `L2-C02` line 5406: "the originating context becomes terminal; a new context opens at the receiving Institute" | **Inter-Institute Student Transfer with Student Context Re-establishment** | Ends in a terminal context **plus** a newly created context — materially different from `C-011`, whose outcome updates the existing context, and from `C-040` Transfer Exit, which ends the relationship with no receiving context. | `C-011`, `C-040` | Student Lifecycle | HIGH |

**Explicitly not reported as missing.** The following were considered and rejected for lack of positive evidence, and are recorded so they are not re-derived:

- *Workforce-side examination duty outcome (invigilator/examiner assignment) in Section D/E.* Pallav's inventory returns **0** matches for "invigilat" or "examiner", and `L2-I03` line 458 names "Faculty Administration — coordinates invigilator and supervisory resource planning". But `L2-E03` defines no invigilator or examiner duty capability at all, and Section I already owns `I-022`, `I-023`, `I-024`. There is no positive E-side source evidence for a missing outcome. Already correctly recorded as `I-XI-05` `OPEN` in the Section I watchlist — it is a boundary question, not a coverage gap.
- *Academic-progression and alumni-transition policy lifecycle scenarios.* Their absence is **correct**. `DEPENDENCY_MAP.md` line 409 records Policy Management as "**Open.** No owner assigned", and §7 requires Lead Architect sign-off before an owner exists. The Section C watchlist `CF-08` reaches the same conclusion. Creating these candidates now would invent an owner.
- *Question paper creation, moderation and security in Section I.* `L2-I07` line 98 excludes Question Paper Creation and line 835 states "Question paper governance remains outside this document". No Section I document owns it. Correctly held as `I-RC-07` `BLOCKED` — it needs a new owner or a new L2 document, and cannot be discovered into an existing one.

---

## 8. Fragmentation / Merge / Split / Duplicate Register

### 8.1 Merge clusters

| # | Section | Candidates | Proposed consolidated business outcome | Basis |
| --- | --- | --- | --- | --- |
| MG-01 | I | `I-270`, `I-271`, `I-272`, `I-273`, `I-274`, `I-275`, `I-276`, `I-277`, `I-278`, `I-279`, `I-280`, `I-305`, `I-306` | **Suspected Examination Malpractice to Adjudicated and Closed Case** | Twelve consecutive steps of one journey: report → register → preserve evidence → refer → notify → receive response → hearing → penalty → closure. Report/notify/receive/preserve are §8 fragments. Retain `I-281`…`I-286` (appeal and overturn) as genuinely distinct post-terminal outcomes. |
| MG-02 | I | `I-055`, `I-058`, `I-060`, `I-063`, `I-065`, `I-066` | **Hall Ticket Issuance to Valid Examination Entry Authorization** | Six rows differ only by originating circumstance (post-withholding, reissue, regeneration, supplementary, special arrangement) and share one final outcome: a valid Hall Ticket authorising entry. Retain `I-057` (withholding — a distinct negative outcome) and `I-061` (invalidation). |
| MG-03 | I | `I-172`, `I-173`, `I-178`, `I-179`, `I-180`, `I-181`, `I-182`, `I-185`, `I-186` | **Approved Result to Published Result, including governed withholding and release** | Nine withhold/release/delay/resume/cancel/withdraw/restore transitions around the single `I-177` publication outcome. Retain `I-183`/`I-184` (post-publication correction and republication) — those change the official record. |
| MG-04 | I | `I-130`, `I-141`, `I-142`, `I-143`, `I-144` | Absorb into **`I-126` Academic Evaluation of Examination Performance** | Assign material, approve marks, withhold approval, release, reopen are steps and guards inside evaluation. `I-141` is the §8 "Approve Request" pattern verbatim. |
| MG-05 | I | `I-114`, `I-115`, `I-116`, `I-117`, `I-118` | Absorb into **`I-093` Conduct a Scheduled Theory Examination** | Question-paper distribution, script collection, and custody transfer are conduct steps. Retain the missing-script and misidentified-script recoveries only if `L2-I07` gives them separate authority. |
| MG-06 | I | `I-013`, `I-014`, `I-015`, `I-017` | Absorb into **`I-011` Establish an Examination Session** | Activate / suspend / reopen / reject-and-return are lifecycle transitions of one aggregate under `L3_Phase1_Discovery_Rules.md` §8. |
| MG-07 | G | `G-453`, `G-454`, `G-460`, `G-465`, `G-467`, `G-469`, `G-470`, `G-471`, `G-473`, `G-474` | **Physical Infrastructure Profile to Effective-Dated Governed Update** (one per aggregate, not one per attribute class) | Ten rows repeat identical effective-dated attribute maintenance across Campus / Building / Space / classification / capability / capacity / availability. |
| MG-08 | G | `G-523`, `G-524`, `G-525`, `G-538` | Retire into their publishing parents | Event delivery, consumer refresh, read contract and read-model reconciliation — the same class the section's own `G-FR-08`/`G-FR-09` clusters already retired for notifications and dashboards. |
| MG-09 | G | `G-027`, `G-051`, `G-052` | Absorb into **`G-025` Facility Work Order to Verified Closure** | Technician acceptance, verification and verification failure are mid-journey steps; `G-027`'s outcome ("a technician… is ready to execute") is not an institutional result. |

### 8.2 Split recommendations

**None.** No candidate in any inventory was proven to hide multiple independent complete outcomes. The over-broad candidates identified by earlier audits (`G-SP-01`…`G-SP-06`, `I-OB-*`) have already been narrowed or retired, and I confirmed the applied narrowings in the current rows. Complexity and cross-domain breadth alone — e.g. `J-049`, `J-097`, `E-064`, `F-001`, `H-071` — were not treated as split grounds.

### 8.3 True duplicates

| # | Candidates | Type | Disposition |
| --- | --- | --- | --- |
| DU-01 | `F-086` ⊂ `H-010` | Specific subsumed by broader | REMOVE `F-086` (FIND-012) |
| DU-02 | `F-087` ⊂ `H-050` | Specific subsumed by broader | REMOVE `F-087` (FIND-012) |

### 8.4 Partial overlaps requiring a boundary decision, not a merge

| # | Candidates | Nature |
| --- | --- | --- |
| OV-01 | `C-069` versus `H-041` | Alumni transition hand-off point undefined; dependent on `CF-05` (FIND-013) |
| OV-02 | `A-012` versus `E-103` | Delegation ownership conflict (FIND-002) — do not merge until the L2 conflict is ruled |
| OV-03 | `I-074` / `I-096`; `I-079` / `I-102`; `I-092` / `I-121` | Practical-versus-theory mirror pairs carrying **identical** Final Outcome text (flagged by `check_l3_inventory.py`). May be legitimately distinct conduct modes; the identical outcome wording is the defect, not necessarily the pair. |

### 8.5 Candidates that must remain separate despite superficial similarity

Recorded so Phase 2 does not over-consolidate:

- `C-029` (Academic Progression decides credit recognition) / `D-032` (D06 requests posting) / `H-082` (H10 synchronises the reference). Three carefully bounded roles in one external-credit journey; each disclaims the others' ownership. **Correct as authored.**
- `B-006`/`B-007` (InstitutionalCampus, B03) versus `G-450`/`G-452`/`G-453` (Physical Campus, G10). The prompt named this as a risk boundary; both sides state the split explicitly and neither claims the other's aggregate. **Correct as authored.**
- `B-991`…`B-993` (Position Definition, B02) versus `E-015`…`E-017` (Position Assignment, E02). Verified against `L2-B02` line 110 ff. ("Approved Position Definition Ownership and Lifecycle Clarification"; E02 "consumes a Position Definition as an external structural reference and shall never create" one) and `L2-E02` line 76. Pallav's re-ownership of former `E-012`…`E-014` is **repository-correct**, not a silent resolution.
- `F-062` (Finance allocation) versus `C-068` (Scholarship eligibility hand-off). Clean owner-side/consumer-side pair.
- `E-098` (Archived → Restored → Active) versus `E-095` (Suspended → Active). Distinct lifecycle paths, correctly kept apart.

---

## 9. Repository Conflict Register

Repository defects are separated from inventory defects. A conflict correctly recorded and preserved by an inventory or watchlist is **not** counted against that section's score.

| # | Conflict | Sources | Positions | Affected candidates | Authority analysis | Phase 2 blocked? | Escalate to |
| --- | --- | --- | --- | --- | --- | --- | --- |
| RC-01 | **Delegation ownership** | `L2-A04` line 114, §10.5 vs `L2-E02` lines 359, 975, 2644, 2662 | A04: exclusively owns Delegation lifecycle under approved INT-301. E02: `Assignment Delegation` is Canonical, owns `DelegateAuthority` and `PositionAssignmentDelegated`. | `A-012`, `E-103` | **Unresolved by hierarchy.** Both are locked L2 documents at the same authority tier. A04's claim carries a dated approval reference, which is suggestive but does not by itself override E02's canonical declaration and event ownership. **CONFLICTED** — no side chosen. | **YES** for these two candidates | INT-301 approving authority |
| RC-02 | **Question paper governance has no owner** | `L2-I07` line 98 (excludes Question Paper Creation), line 835 ("Question paper governance remains outside this document"); `L2-I14` §9.2, `L2-I15` §9.2 both disclaim | No Section I document owns question paper creation, moderation, security or integrity. | `I-111`, `I-112`, `I-113` | Not resolvable by ruling between existing documents — needs a new owner or a new L2 document. Correctly preserved as `I-RC-07` `BLOCKED`. **Repository defect, not an inventory defect.** | **YES** for these three | Lead Architect / EAB |
| RC-03 | **Policy Management has no owner** | `DEPENDENCY_MAP.md` line 409 — status "**Open.** No owner assigned"; `L2-C04` §2.6; `L2-C08` §5 | `AlumniTransitionPolicyUpdated` and `AcademicPolicyUpdated` have no publisher anywhere in the repository. | Two candidates that **cannot** be discovered (structural analogues of `C-060`) | Correctly preserved as `CF-08` `BLOCKED`. The inventory's non-creation of these candidates is **correct**, not an omission. | **YES** for that family | Lead Architect (`DEPENDENCY_MAP` §7 requires sign-off) |
| RC-04 | **Admission state vocabularies incompatible** | `L2-C01` §6.4–§6.9 vs §10.3, §10.7 | §6: `Admission Initiated / Documents Submitted / … / Expired`. §10.3: `Draft / Submitted / UnderVerification / … / Readmitted`, drops `Expired`. | All C01 candidates (`C-001`…`C-006`) | Correctly preserved as `CF-02` `OPEN`; all C01 candidates quote §6 as the stated working vocabulary. **Inventory handled this correctly.** | No — wording constraint only | L2-C01 owner |
| RC-05 | **`Rejected` admission state not canonical** | `L2-C01` WF-C01-03 §2.7 EX-01, §3.2 vs §6.8, §6.9, §6.9.1, §6.13, §10.3 | Published as `AdmissionRejected` but present in no canonical state list. | The recorded mitigation (`C-072`) **does not exist** — see FIND-001 | Preserved as `CF-01` `OPEN`, but its stated remedy is absent from the repository. This converts a correctly-handled conflict into an **inventory integrity defect**. | **YES** until FIND-001 is settled | Lead Architect |
| RC-06 | **Alumni Transition ownership stated three ways** | `L2-C04` §1.5, `L2-C05` §5/§11.10, `L2-C08` §5.2 | C04 assigns it to C05; C05 lists it out of scope while containing a transition workflow; C08 claims exclusive ownership. | `C-069`, and by extension `H-041` (OV-01) | Correctly preserved as `CF-05` `OPEN` with the C08 reading applied and declared. **Inventory handled this correctly.** | No | L2-C04 owner (source correction) |
| RC-07 | **Guardian capability defined in two documents** | `L2-C02` WF-C02-13 vs `L2-C06` §2, §5 | C02 claims guardian registration, replacement, primary designation and emergency contacts; C06 declares the same capabilities "exclusively owned" and not to be duplicated. | `C-048`…`C-058` (11 candidates) | Correctly preserved as `CF-04` `OPEN` with the C06 exclusivity reading applied **and explicitly recorded as a reconciliation of a live conflict**. Exemplary handling. | No | EAB |
| RC-08 | **D03 infrastructure ownership** | Preserved by the inventory as `FIND-D-01`, `FIND-D-02` | D03 aggregate ownership lists differ; infrastructure ownership unresolved. | `D-016`, `D-017` | Correctly preserved; the inventory keeps resource-context wording ownership-safe and states "FIND-D-01 and FIND-D-02 remain unresolved and are **not silently reconciled here**". **Exemplary.** | No | L2-D03 owner |
| RC-09 | **B02/B03 decommission-versus-archive divergence** | `L2-B02` WF08 (one workflow) vs `L2-B03` §111, §112 (two) | Whether decommission and archive are one outcome or two. | `B-009`, `B-032` | Correctly preserved as an explicit "Open item" in the inventory, with the registration/activation convention deliberately **not** extended pending source reconciliation. **Exemplary.** | No | L2-B02 / L2-B03 owners |

**Inventory defects, as distinct from the above:** FIND-001 (Section C divergence), FIND-002's *non-recording* of RC-01, FIND-003, FIND-008, FIND-010, FIND-012.

---

## 10. Cross-Section Findings

A cross-inventory scan was run over all 1,215 rows, comparing normalised Title + Final Outcome token sets across every pair of candidates in different discovery areas.

**Result: zero cross-inventory near-duplicates at the 0.55 similarity threshold.** At a relaxed 0.45 threshold only seven pairs surfaced, and every one is a structurally similar but substantively distinct outcome — e.g. `B-073 Calendar Change Simulation` versus `D-057 Course Offering Change Simulation` (0.71): same governed-simulation pattern, different subject, different owner, both repository-supported. **No merge is warranted from any of the seven.**

Checks performed against the boundaries the prompt named in advance:

| Check | Result |
| --- | --- |
| Same canonical business result assigned to multiple owners | **None found** across 1,215 rows |
| Section H / read-model services becoming business owners | **Clean.** `H-DEC-01` explicitly assigns dashboard composition to H07 and source facts, reports and KPI definitions to F13/G10. `H-050`…`H-058` consistently disclaim source ownership. |
| Section F monetary ownership duplicated elsewhere | **Clean.** `G-054`, `J-070`, `C-054`, `E-062` all explicitly decline financial-transaction ownership. |
| Section I official result ownership duplicated elsewhere | **Clean.** `C-022`, `C-024`…`C-028`, `C-035` consume results with C as owner and I as participant. |
| Section J enterprise resource ownership duplicated elsewhere | **Clean.** `G-053` records maintenance consumption "without transferring inventory ownership to G01"; `G-038`/`G-039` supply operational evidence to J02 without changing the canonical supplier record. |
| B03 institutional Campus versus G10 physical infrastructure | **Clean, and unusually well handled.** `B-006`…`B-010` and `G-450`…`G-453` each state the other side's ownership explicitly. |
| Dependency reversal | **None found** |
| Scenario boundaries re-owning another section's lifecycle | **None found** |
| Missing major hand-off outcomes | **None proven.** Every hand-off examined (C→F scholarship, D→I assessment, E→F payroll, J→F asset, I→C progression, C→H alumni) has candidates on both sides. |

Genuine cross-section issues are FIND-002 (RC-01), FIND-008 (unperformed A/J ↔ B/D/E comparison), FIND-012 (F→H subsumption) and FIND-013 (C↔H alumni boundary). The Section I watchlist's `I-XI-02` (C-046/C-047 versus I-262…I-269 ordering) and `I-XI-03` (G-002 versus I-021/I-085/I-094/I-095/I-099) remain correctly `OPEN` and are not re-raised here.

---

## 11. Clean-Coverage Summary

Compact counts only. No candidate-by-candidate commentary.

| Section | L2 documents | Documents with coverage | Candidates checked | Material exceptions |
| --- | ---: | ---: | ---: | --- |
| A | 11 (A00–A10) | 10 workflow docs + A00 review baseline | 62 | FIND-007 (partial), FIND-008, FIND-016 |
| B | 6 (B00–B05) | 5 workflow docs + B00 review | 32 | **None material** — highest-scoring section |
| C | 9 (C00–C08) | 8 workflow docs + C00 review | 71 | FIND-001, FIND-006 |
| D | 8 (D00–D07) | 7 workflow docs + D00 review | 57 | FIND-010 |
| E | 8 (E00–E07) | 7 workflow docs + E00 review | 75 | FIND-002 (unrecorded side) |
| F | 15 (F00–F14) | 15 | 87 | FIND-007, FIND-012, FIND-014 |
| G | 11 (G00–G10) | 11 | 391 | FIND-005, FIND-011, FIND-014 |
| H | 11 (H00–H10) | 10 workflow docs + H00 governance | 85 | FIND-009, FIND-012 |
| I | 17 (I00–I16) | 16 workflow docs + I00 review | 245 | FIND-004, FIND-007, FIND-014, FIND-015 |
| J | 11 (J00–J10) | 11 | 110 | FIND-003, FIND-007, FIND-018 |

**Every L2 workflow document in Sections A–J has candidate coverage. No document-level coverage gap exists anywhere in the repository.** The `*00` documents are control/review/governance baselines that define no workflows and correctly carry no candidates of their own.

Conformance: `check_l3_inventory.py` — 5 files, 1,215 candidates, **0 errors**, 221 warnings. Distinct-outcome ratio is 100% for four inventories and 99% for Section I.

---

## 12. Overall Score

Arithmetic mean of the ten audited section scores, unweighted by candidate count (weighting by count would give an over-fragmented section artificial influence):

```
(8.7 + 9.1 + 7.4 + 8.5 + 8.4 + 8.5 + 7.3 + 8.5 + 7.6 + 7.3) / 10 = 81.3 / 10
```

**Overall score: 8.1 / 10.0**

Interpretation band: *good but requires meaningful corrections.*

Range across sections: 7.3 (G, J) to 9.1 (B). The spread is driven almost entirely by granularity and by Type/Priority discipline, not by coverage — coverage scores range only from 1.3 (C, depressed solely by FIND-001) to 2.4.

---

## 13. Phase 2 Readiness Verdict

## **BLOCKED — REPOSITORY DECISION REQUIRED**

The numerical score of 8.1 does not override this. Three unresolved ownership questions make the ownership or existence of material scenarios indeterminate, which meets the BLOCKER definition:

1. **RC-01 / FIND-002 — Delegation.** Ownership of `A-012` and `E-103` cannot be settled from repository evidence. Requires a ruling from the INT-301 approving authority.
2. **RC-02 — Question paper governance.** `I-111`, `I-112`, `I-113` describe real institutional outcomes resting on a capability no repository document assigns. Requires a new owner or a new L2 document.
3. **FIND-001 / RC-05 — Section C row set.** Governance records 74 candidates; the inventory holds 71. Until this is settled, Section C's authoritative candidate set — and the recorded mitigation for `CF-01` — do not exist.

RC-03 (Policy Management) is a fourth open repository decision, though its effect is narrower: it correctly prevents two candidates from being discovered at all.

**Scope of the block.** Sections B, D, E, F and H are individually ready for Phase 2 consolidation subject to their targeted corrections. Sections G, I and J need consolidation and re-classification work but are not blocked by a repository decision. The block is specific: it prevents Phase 2 from *closing* while these four questions are open, and it prevents any affected candidate from being authored.

---

## 14. Prioritised Correction Recommendations

**Tier 1 — must be settled before Phase 2 consolidation closes**

1. **Rule RC-01 (Delegation).** Amend either `L2-A04` line 114 to scope its exclusivity to authorization-layer delegation, or `L2-E02` to consume A04 delegation. Then set the owner for `A-012` and `E-103`. *(FIND-002)*
2. **Reconcile the Section C row set.** Either apply `C-072`, `C-073`, `C-074` and the `C-011` split to the inventory, or withdraw the watchlist §6 application record and reopen `CF-01`. *(FIND-001, FIND-006, MC-01…MC-03)*
3. **Assign an owner for question paper governance**, or create the L2 document that owns it. *(RC-02)*
4. **Assign an owner for Policy Management** under `DEPENDENCY_MAP` §7, unblocking the two undiscoverable policy-lifecycle candidates. *(RC-03)*

**Tier 2 — required before Phase 2 consolidation can be relied on**

5. **Normalise Section J `Domains` to section letters** and re-derive Type for the 11 falsely `Cross-domain` candidates. *(FIND-003)*
6. **Second consolidation pass on Section I** against `L3_Phase1_Discovery_Rules.md` §8 — merge clusters MG-01 through MG-06. *(FIND-004)*
7. **Extend the accepted `G-FR-08`/`G-FR-09` reasoning to G10 and G01** — merge clusters MG-07 through MG-09. *(FIND-005)*
8. **Re-run the A/J ↔ B/D/E peer comparison** and correct the "empty Pallav inventory" statement in both `L3_Phase1_Inventory_Dhruv.md` line 28 and `L3_Section_A_Phase2_Consolidation_Watchlist.md` line 153. *(FIND-008)*
9. **Re-derive Type and Priority for Sections F, I, G and J10** under `L3_Scenario_Type_and_Priority_Rules.md` §7 and §12. Route through Lead Architect review per §15 rather than back to the discoverers. *(FIND-007, FIND-014)*

**Tier 3 — correctness and hygiene**

10. Retire `F-086` and `F-087` into `H-010` and `H-050`. *(FIND-012)*
11. Replace the all-ten-domain listings on the 19 Section H rows with materially participating domains or a governed `VAR:` token. *(FIND-009)*
12. Apply the §11 participation test to Section G01/G02 `Domains` values. *(FIND-011)*
13. Correct the two stale Section D notes (the "38 candidates" figure and the `D-020` boundary note). *(FIND-010)*
14. Create Section B/D/E and Section J Phase 2 consolidation watchlists and move narrative content out of those inventories. *(FIND-017)*
15. Record the `C-069` / `H-041` alumni boundary as a cross-inventory item dependent on `CF-05`. *(FIND-013)*
16. Correct the Section I watchlist header from 258 to 245. *(FIND-015)*
17. Pick one Primary Owner convention within `L3_Phase1_Inventory_Dhruv.md`. *(FIND-016)*
18. Resolve the three identical Final Outcome strings in Section I (`I-074`/`I-096`, `I-079`/`I-102`, `I-092`/`I-121`) — distinguish the wording or merge the pairs. *(OV-03)*
19. Phase 2 judgment on the J10 disposal-method set — flagged, not recommended. *(FIND-018)*

---

## 15. Audit Limitations and Items Not Verified

Stated plainly so nothing in this report is read as stronger than its evidence.

**Method limitations**

- This was a staged audit, not a line-by-line read of all 104 L2 documents. All 1,215 candidate rows were read in full. L2 sources were opened only where a finding required proof; nine L2 documents were opened for targeted verification. **No material finding rests on an AI Context summary** — every one cites an original L2 source, a governance document, `DEPENDENCY_MAP.md`, or the inventory text itself.
- Cross-inventory duplicate detection used lexical similarity over Title + Final Outcome. It reliably catches restatements and near-copies. It would not catch two candidates describing the same outcome in wholly disjoint vocabulary. The manual boundary checks in §10 partially compensate, but semantic duplicate detection across 1,215 rows is not exhaustive.

**Not verified**

- **Section G's 391 candidates were sampled, not exhaustively verified against `L2-G01`…`L2-G10`.** G00, G01, G02 and G10 were read in full (about 150 rows); G03–G09 were assessed from the extracted title/owner/domain/type/priority data only. FIND-005 and FIND-011 are proven for the sections read; whether the same patterns persist in G03–G09 is **NOT VERIFIED**. The scope of MG-07 through MG-09 may therefore be understated.
- **Section I's 245 candidates were read in full at title/owner/domain/type/priority level**, and I01–I05 were read with complete Trigger and Final Outcome text. I06–I16 Trigger/Outcome text was not read in full. The merge clusters MG-01 through MG-06 rest on title-and-catalogue evidence plus the §8 rule; each should be confirmed against the candidate's full Trigger and Final Outcome before execution.
- **Devang's and Chetan's inventories were audited against their own watchlists**, which record prior audit findings I did not independently re-derive. Where a watchlist marks an item `CLOSED` or `APPLIED`, I verified the current inventory state matches the recorded outcome for Sections A, F, G, H and I — this is how FIND-001 was found for C. I did not re-litigate the underlying reasoning of any `CLOSED` item.
- **Type and Priority re-derivation was not performed.** FIND-007 and FIND-014 establish that the distributions are inconsistent with the governed rules and give counter-examples. They do **not** supply a corrected value for any individual candidate. `L3_Scenario_Type_and_Priority_Rules.md` §15 reserves that judgment for Lead Architect review, and this audit respects that.
- **RC-01 was not resolved.** A reading exists in which A04 owns authorization-layer delegation and E02 owns organisational-assignment delegation, and that reading may well be the architects' intent. No repository source states it, so the conflict is preserved rather than reconciled.
- **No estimate is offered for the "correct" number of L3 scenarios** in any section. The merge clusters identify specific over-fragmentation with evidence; they do not imply a target count. Section G's watchlist target and Section I's 200–230 estimate are both prior-audit inferences and are neither endorsed nor contradicted here.

**Not in scope**

- Final L3 scenario ownership, final numbering, final creators and final reviewers remain unlocked per `L3_Phase1_Discovery_Rules.md` §18. Nothing in this report assigns any of them.
- No candidate was criticised for lacking full-L3 fields (actors, event sequences, failure recovery, authorization, audit evidence, preconditions, completion criteria). Phase 1 governance does not require them.

**Write discipline**

This report is the only file written. No architecture, governance, index, matrix, dependency, context or inventory file was created, modified, or deleted. No candidate was added, removed, merged, split, reordered or renumbered in any repository file. No repository conflict was resolved.
