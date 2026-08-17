# L3 Section G — 350-Candidate Independent Boundary Audit


> **Numbering note (2026-08-17).** Candidate IDs in this historical report reflect the numbering in force when it was written. Repository-wide candidate numbering was normalized on 2026-08-17; for current IDs see `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.
**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section G — Campus Operations, current candidates `CAN-L3-G-001`…`CAN-L3-G-541` (350 sparse rows)
**Audit Type:** Independent adversarial boundary re-audit of the CURRENT 350 survivors (not a re-run of the prior consolidation)
**Auditor role:** Senior Enterprise Architecture Auditor
**Date:** 2026-08-17
**Status:** Audit-only. No architecture, inventory, or watchlist file was modified.
**Primary source:** `architecture/L3_Business_Scenario_Architecture/01_Phase1_Discovery/L3_Phase1_Inventory_Varad.md` (Section G portion only)
**Prior consolidation record reviewed:** `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_G_Phase2_Consolidation_Watchlist.md`

---

## 1. Executive Verdict

> **SECTION G IS MOSTLY RIGHT-SIZED — MINOR CONSOLIDATION REQUIRED**

Two prior consolidation passes (541 → 391 → 377 → 350) already removed the large, obvious fragmentation
classes (notification rows, dashboard rows, audit-history rows, generic rejection/config-mechanism
fragments, single-step access/movement rows). That work holds up under independent re-check — no
`APPLIED`/`CLOSED` watchlist decision is reversed by this audit.

However, the watchlist's own root-cause note (2026-08-15 entry) is correct and still live: several of the
*already-approved* consolidation principles (single-aggregate-lifecycle-as-one-outcome; effective-dating
as a Section A mechanism, not a G outcome; severity/trigger-type as a configurable attribute rather than a
new scenario) were applied cluster-by-cluster and did not get swept across the full 350. This audit found
**20 further candidates that are lifecycle-stage restatements of an outcome already owned by a sibling row
in the same subdomain**, **7 further candidates that are trigger/severity variants of one outcome already
established by L2** as a single configurable workflow, and **2 further exact-mechanism duplicates** — concentrated
in G00, G01, G07, G08, G09 and G10, verified against the relevant L2 workflow diagrams. One candidate
(G-202, First Aid) is genuinely ambiguous under L2 and is left `REVIEW-REQUIRED` rather than decided either
way.

This is a **targeted, evidence-backed 29-candidate consolidation opportunity** (8.3% of 350), not a
structural failure of Section G's boundary discipline. G02–G05 (Laboratory, Hostel, Transport, Library)
show no further fragmentation beyond what the watchlist already applied.

---

## 2. Current Count

**Confirmed exact current count: 350** rows matching `CAN-L3-G-\d{3}` in
`L3_Phase1_Inventory_Varad.md`, counted directly against the file (not taken from the header claim).
IDs are sparse across the range `CAN-L3-G-001`…`CAN-L3-G-541`, consistent with the watchlist's traceability
model (retired IDs are not reused, per `L3_Naming_Numbering_Standard.md` and watchlist §9).

---

## 3. G00–G10 Breakdown

Canonical subdomain names are taken from the inventory's own section headers and the
`L2_Domain_Workflow_Architecture/section-g/` filenames (which differ from the task brief's shorthand list —
e.g. the repository's G02 is **Laboratory Management**, not Hostel; G04 is **Transport Management**, not
Security). Canonical names are used throughout, per the "do not invent sub-domain names" instruction.

| Subdomain | Canonical name | Current | KEEP | MERGE | VARIANT | FAILURE-RECOVERY | DUPLICATE | SPLIT-REQUIRED | BLOCKED | REVIEW-REQUIRED |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| G00 | Campus Operations Review & Governance | 20 | 15 | 0 | 5 | 0 | 0 | 0 | 0 | 0 |
| G01 | Facilities Management | 35 | 34 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| G02 | Laboratory Management | 28 | 28 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| G03 | Hostel Management | 15 | 15 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| G04 | Transport Management | 31 | 31 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| G05 | Library Management | 31 | 31 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| G06 | Institutional Health Services & Medical | 32 | 31 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| G07 | Security Operations | 35 | 30 | 3 | 2 | 0 | 0 | 0 | 0 | 0 |
| G08 | Housekeeping Operations & Service Governance | 46 | 36 | 9 | 0 | 0 | 1 | 0 | 0 | 0 |
| G09 | Teacher Guardian Scheme (TGS) | 29 | 26 | 3 | 0 | 0 | 0 | 0 | 0 | 0 |
| G10 | Infrastructure Management | 48 | 43 | 4 | 0 | 0 | 1 | 0 | 0 | 0 |
| **Total** | | **350** | **320** | **20** | **7** | **0** | **2** | **0** | **0** | **1** |

G02–G05 were read in full and cross-checked against the prior watchlist's applied clusters for those
subdomains (G-OV-03…07); no new fragmentation pattern was found there. G03 (15 rows) in particular is
already tightly scoped — no chain of ≥3 consecutive lifecycle-stage rows exists in it.

---

## 4. Consolidation Candidates

Every row below was checked against the governing L2 document before disposition; title similarity alone
was never used as evidence (merge-safety rule, §8 of the brief).

| Cluster ID | Candidate IDs | Current Titles (abridged) | Recommended Parent | Disposition | Reason | L2 Evidence | Confidence |
|---|---|---|---|---|---|---|---|
| SG-01 | `G-002, G-003, G-004, G-005, G-540` | Examination / Admission / Event / Emergency / Academic Session Readiness Assessment | `G-001` Daily Campus Readiness (rewritten to generic Campus Readiness Assessment) | VARIANT | L2 defines exactly one "Campus Readiness Workflow" (Assessment Scheduled → Domain Readiness Published → Aggregated → Operational Review → Executive Recommendation → Assessment Closed) and explicitly lists Daily/Academic Session/Examination/Admission/Event/Emergency/Accreditation as **configurable "Readiness Types"** within that single workflow, not as separate workflows. Final Outcome wording across all 5 rows is structurally identical to `G-001`. | `L2-G00_Campus_Operations_Review_and_Governance.md` §"Campus Readiness Workflow" / "Readiness Types" (lines 769–800) | High |
| SG-02 | `G-047` | Inspection Finding to Corrective Work Resolution | `G-025` Facility Work Order to Verified Closure | MERGE | L2's Inspection Workflow branches "Corrective Action Required? → Yes → Work Order Generated," feeding the same Work Order aggregate `G-025` already owns end-to-end (assigned, accepted, executed, evidenced, verified, closed) from "complaint, request, **inspection**, preventive plan, emergency, or administrative source." `G-047` restates that span from the inspection-triggering side — the exact pattern already retired for `G-027/G-051/G-052` under `G-FR-13`, not yet swept to this row. | `L2-G01_Facilities_Management_Workflows.md` §"Inspection Workflow" (lines 1995–2013); watchlist §4A `G-FR-13` | High |
| SG-03 | `G-270, G-271, G-272, G-273, G-274` | Incident Assessment→Investigation; Investigation→Resolution; Resolution Review→Closure; Routine Incident→Resolution; Critical Incident→Emergency Declaration | `G-269` Security Incident Report to Registration (rewritten to span full closure) | MERGE (270–272) / VARIANT (273–274) | L2 diagrams and tabulates **one** Security Incident lifecycle: "Reported → Assessment → Investigation → Resolved → Closed" (also stated as a single state-transition row: `Reported → Assessment → Investigation → Resolved → Closed`). `G-270/271/272` are consecutive spans of that one lifecycle. `G-273`/`G-274` restate the same lifecycle differentiated only by the Severity Levels L2 lists as configurable within the same workflow (Low/Medium/High/Critical), not as separate workflows — `G-274`'s "Emergency Declaration" outcome is the Critical-severity branch point of the same incident lifecycle, distinct from the materially different, separately-diagrammed Emergency Response coordination that follows in `G-275` (which stays separate — see §5). | `L2-G07_Security_Operations_Workflows.md` §"Incident Workflow" (lines 1281–1370) and the Security Incident state table (line 1703) | High |
| SG-04 | `G-311` | Approved Cleaning Schedule to Active Cleaning Programme | `G-310` Cleaning Schedule Planning to Approved Schedule (rewritten to span Draft→Active) | MERGE | L2's own state-transition table lists Cleaning Schedule as **one** aggregate lifecycle: `Draft → Approved → Active → Completed`. `G-310`/`G-311` are two consecutive stages of that single aggregate. | `L2-G08_Housekeeping_Operations_and_Service_Governance_Workflows.md`, Aggregate Lifecycle table (line 1586) | High |
| SG-05 | `G-313, G-315` | Cleaning Task to Operational Assignment; Routine Cleaning Task to Verified Completion | `G-312` Active Cleaning Schedule to Cleaning Task Generation (rewritten to span Generated→Completed) | MERGE | Same lifecycle table lists Cleaning Task as **one** aggregate lifecycle: `Generated → Assigned → Executing → Completed`. `G-312/313/315` are three consecutive stages of that single aggregate (task generation, assignment, execution/verification). | Same table (line 1588) | High |
| SG-06 | `G-323, G-324, G-326` | Priority Assessment; Assignment to Reactive Execution; to Verified Resolution | `G-321` Complaint Registration to Validated Cleaning Request (rewritten to span registration→closure) | MERGE | L2 diagrams **one** Complaint Workflow: `Complaint Registered → Validation → Priority Assignment → Operational Assignment → Cleaning Completed → Inspection → Closure`. `G-321/323/324/326` are four consecutive stages of that single documented workflow — directly analogous to the already-applied G01 work-order precedent (`G-FR-13`). | `L2-G08…` §"Complaint Workflow" (lines 1085–1101) | High |
| SG-07 | `G-331, G-332, G-333` | Hygiene Inspection Compliant Outcome; Failure to Corrective Cleaning; Failure to Escalated Risk | `G-330` Hygiene Inspection Scheduling to Inspection Execution (rewritten to span scheduling→outcome) | MERGE | L2's aggregate lifecycle table lists Hygiene Inspection as **one** lifecycle: `Scheduled → Inspection → Evaluated → Closed`. `G-331/332/333` are the "Evaluated" branch outcomes (compliant / corrective / escalated) of the same single aggregate, not independent institutional outcomes. | `L2-G08…` Aggregate Lifecycle table (line 1589) | High |
| SG-08 | `G-334` | Washroom Inspection to Hygiene Compliance Outcome | `G-330` (same parent as SG-07) | DUPLICATE | `G-330`'s own Trigger text already enumerates "a routine, surprise, complaint, **washroom**, event, or post-clean inspection" as its trigger variants — `G-334` restates the washroom-specific instance of the same aggregate `G-330` already claims to cover. | `L2-G08…` §"Complaint Workflow" trigger enumeration + inventory row `G-330` Trigger field | High |
| SG-09 | `G-400, G-401, G-402` | Referral Creation to Student Guidance; Follow-up to Updated Status; Completion to Mentorship Follow-up Outcome | `G-390` Mentoring Issue to Governed Institutional Referral (rewritten to span the full G09-side referral record) | MERGE | L2 diagrams **one** Referral Workflow: `Mentoring Discussion → Issue Identified → Department Selected → Referral Recorded → Student Advised → Referral Status Updated → Referral Closed`. `G-390` already covers steps 1–4; `G-400/401/402` restate steps 5–7 of the same single workflow as separate rows. | `L2-G09_Teacher_Guardian_Scheme_TGS_Workflows.md` §15 "Referral Workflow" (lines 905–968) | High |
| SG-10 | `G-514, G-515` | Decommissioning Validation to Dependency Resolution; Decommissioning Completion to Historical Closure | `G-513` Infrastructure Decommissioning Initiation (rewritten to span initiation→closure) | MERGE | L2's `WF-INF-013 — Decommission Infrastructure` is **one** workflow: `Request Decommission → Dependency Validation → Approval → Update Lifecycle → Publish InfrastructureDecommissioned`. `G-513/514/515` are three consecutive stages of that single workflow. | `L2-G10_Infrastructure_Management_Workflows.md` §"WF-INF-013" (lines 4381–4425) | High |
| SG-11 | `G-496, G-497` | Inspection Completion to Condition Update; Finding to Infrastructure Risk Review | `G-495` Infrastructure Inspection Requirement to Inspection Record (rewritten to span requirement→risk review) | MERGE | L2's §32 "Inspection Workflow" is **one** diagrammed workflow: `Scheduled → Inspector Assigned → Conducted → Findings Recorded → Approved → Closed`. `G-495/496/497` are three consecutive stages of that single workflow. | `L2-G10…` §32 "Inspection Workflow" (lines 7597–7611) | High |
| SG-12 | `G-520` | Infrastructure Configuration Effective Date to Policy Transition | `G-518` Infrastructure Configuration Change to Future Policy Availability | DUPLICATE | Same generic Section-A-owned configuration/effective-dating mechanism already retired everywhere else in Section G under `G-FR-02`/`G-FR-12` (every other G subdomain carries exactly one config-change row; G10 alone carries two covering the same mechanism twice — the "not swept section-wide" gap the watchlist itself flagged for G08/G09 also reached G10). | Watchlist §4A root-cause note; inventory rows `G-518`/`G-520` (near-identical Trigger/Outcome text) | Medium-High |

**Net effect if all of the above were applied:** 350 → **321** candidates (29 rows absorbed: 20 as
lifecycle-portion MERGE, 7 as trigger/severity VARIANT, 2 as exact-mechanism DUPLICATE). No candidate ID
would need to be reused; retired IDs remain retired per the existing traceability convention.

---

## 5. Candidates That Must Stay Separate

To prevent over-consolidation, these look similar to the clusters above but have distinct L2 evidence
supporting independence:

- **`G-006` Campus Operations Evidence to Accreditation Evidence Readiness** vs. the SG-01 readiness
  family. L2 lists "Accreditation Evidence Readiness" as one of the same configurable Readiness Types, but
  `G-006`'s Final Outcome is a materially different *kind* of result (assembling an evidence package for
  H09, explicitly **not** producing a readiness decision) — already correctly scoped in the watchlist as
  `CLOSED` under `G-XI-03`/`H-DEC-04`. Not part of the SG-01 merge.
- **`G-275` Security Emergency to Coordinated Emergency Response** vs. the SG-03 incident chain. L2's
  §20 "Emergency Response" is a separately-named capability (external coordination with Health Services,
  Fire, Police, Ambulance) triggered *by* a Critical incident, not a further stage of the same Incident
  Workflow diagram. Genuinely distinct institutional outcome — kept out of SG-03.
- **`G-199` Walk-in Consultation to Completed Medical Encounter** vs. **`G-201` Emergency Incident to
  Immediate Institutional Medical Response**. L2 diagrams these as two separate workflows ("Consultation
  Workflow": Patient Registered → Clinical Assessment → Diagnosis → Treatment → Prescription → Closed;
  "Emergency Workflow": Incident Reported → Emergency Assessment → Immediate Treatment → Referral Decision
  → Stabilised → Closed). "Walk-in" is a configurable Consultation Type, not evidence that Consultation and
  Emergency should merge — they remain two L2-documented workflows.
- **`G-492`/`G-493`/`G-494` Infrastructure Compliance trio** (Requirement→Record; Status Change→Update;
  Failure→Restriction). Unlike the SG-10/SG-11 chains, L2 diagrams **three distinct** workflows here —
  "Compliance Registration Workflow," "Compliance Renewal Workflow," and an "Expired Compliance" handling
  section with its own distinct enforcement outcome — not one lifecycle split into three rows. All three
  candidates are independently evidenced and must stay separate.
- **`G-516` Infrastructure Archive Eligibility to Controlled Archival** vs. the SG-10 decommission chain.
  L2 defines Archive as a separate subsequent workflow (`WF-INF-014 — Archive Infrastructure`, triggered
  by a `Decommissioned` state), distinct from `WF-INF-013` Decommission itself. Confirmed separate.
- **`G-330` (Hygiene Inspection, all sources)** vs. **`G-321` (Complaint Workflow)** in G08. Both begin
  with a report/schedule and end in a closed outcome, and both are owned by Housekeeping, but they are two
  different L2-tabulated aggregate lifecycles (Hygiene Inspection: `Scheduled→Inspection→Evaluated→Closed`
  vs. Complaint: `Registered→Validation→Priority→Assignment→Cleaning→Inspection→Closure`) with different
  triggers (scheduled/compliance-driven vs. reported-issue-driven) — title overlap ("Inspection") is not
  sufficient grounds to merge them into each other.

---

## 6. Domains Field Findings

This continues the row-by-row review the watchlist explicitly left `OPEN` at §4A ("Further G01/G02 rows
list D, I or H on the same impacted-not-participating basis... each needs a row-by-row judgment"). The
same seven rows the watchlist named were reviewed against their Final Outcome text; no additional
instances were found elsewhere in the 350 during this pass.

| Candidate | Current Domains | Finding | Evidence | Recommended Phase 2 Action |
|---|---|---|---|---|
| `G-022` Facility Complaint to Verified Closure | `G, D, I, H` | D (Academic Delivery) and I (Examination) do not act in classification/investigation/verification — they are affected if the complaint concerns their space/schedule, but perform no step | Final Outcome: "classified, investigated, resolved, verified, and closed" — no D/I action named | IMPACT-ONLY — remove D, I |
| `G-032` Corrective Maintenance to Restored Operation | `G, E, D, I` | E (Facilities workforce) genuinely executes repair; D/I only affected if the defect is in academic/exam space | Final Outcome: "assessed, repaired, verified, restored" — only G/E act | VALID PARTICIPATION (E); IMPACT-ONLY — remove D, I |
| `G-044` Utility Outage to Service Restoration | `G, H, D, I` | D/I impacted (an outage may affect classrooms/exam venues) but do not register, track, or restore the utility | Final Outcome: "registered... tracked... restored and closed" — no D/I verb | IMPACT-ONLY — remove D, I; H — UNCERTAIN, needs the same H-participation-vs-notification test the watchlist applied elsewhere |
| `G-045` Utility Anomaly to Operational Intervention | `G, H, D, I` | Same pattern as `G-044` | Final Outcome: "assessed... intervention or resolution completed" | IMPACT-ONLY — remove D, I; H — UNCERTAIN |
| `G-055` Facility Incident to Immediate Maintenance Intervention | `G, D, I, H` | Same pattern | Final Outcome: "assessed and the required operational maintenance intervention is completed and verified" — G-only action | IMPACT-ONLY — remove D, I; H — UNCERTAIN |
| `G-058` Facility Work Conflict to Replanned Execution | `G, E, D, I` | E schedules/executes; D/I are the source of the competing demand (a scheduling conflict *with* an academic/exam activity) — arguably D/I do "co-produce" the conflict itself even if not its resolution | Final Outcome: "resolved through reassignment or replanning" — resolution is G/E only | UNCERTAIN — REVIEW REQUIRED (the conflict's existence depends on D/I demand, unlike the other five rows here) |
| `G-076` Laboratory Incident to Operational Resolution | `G, H, D, I` | Same impacted-not-participating pattern as `G-022`, one subdomain over (G02) | Final Outcome: "classified, assessed, operationally resolved, and closed" — no D/I action | IMPACT-ONLY — remove D, I; H — UNCERTAIN |

No further Domains-inflation instances were found in G03–G10 during the full read-through; those
subdomains' Domains entries consistently correspond to a named actor or consumer verb in the Final Outcome
text (e.g., G10's `G-483` legitimately lists `D` because "a consuming domain **requires** an infrastructure
resource" — D actively initiates the allocation, which is genuine participation, not impact).

---

## 7. Remaining Architecture Blockers

- **Watchlist §4A "residual participation review" — still not fully closed.** This audit resolved the D/I
  half of the seven named rows (all IMPACT-ONLY, high confidence) but the H (Institutional Services)
  question remains genuinely open for `G-044, G-045, G-055, G-076` — H's Domains listing may reflect either
  real cross-domain participation or notification consumption, and this audit did not find L2 text
  sufficient to decide which, per §11 of the brief ("consumption of another domain's event does not
  transfer ownership"). Marked `UNCERTAIN — REVIEW REQUIRED`, not resolved either way.
- **No `BLOCKED — UPSTREAM ARCHITECTURE CONFLICT` items were found** in this pass. All cross-inventory
  ownership questions the watchlist previously flagged (`G-XI-01…03`) remain `CLOSED`/`APPLIED` with no new
  contradicting evidence surfaced during this audit.
- **`G-202` First Aid Incident to Stabilised Patient Outcome — `REVIEW-REQUIRED`.** L2 documents "Emergency
  & First Aid Management" as a single §19 heading with one diagrammed "Emergency Workflow," while First Aid
  itself is described only as a list of operational activities ("Wound Care, Minor Injury Treatment, Basic
  Medication, Vital Assessment, Observation Recommendation") without its own workflow diagram. This is
  insufficient evidence to confidently call it either an independent outcome or a portion of `G-201`'s
  Emergency Workflow — `INSUFFICIENT EVIDENCE — DO NOT CONSOLIDATE` without a Phase 2 judgment call informed
  by whichever team member owns L2-G06 clarification.

---

## 8. Estimated Canonical Scenario Range

- Certain standalone scenarios (KEEP, evidence-supported independence): **320**
- Candidates clearly absorbable (MERGE + VARIANT + DUPLICATE, L2-evidenced): **29**
- Candidates uncertain/blocked (REVIEW-REQUIRED): **1**

**Evidence-supported scenario count if all definite consolidations were applied: 321**

This is not a target — it is the arithmetic result of applying only the 12 evidence-backed clusters in §4.
No speculative reduction beyond that evidence is proposed.

---

## 9. Final Recommendation

> **B. Receive a small targeted consolidation.**

The twelve clusters in §4 (29 candidates) should go through the same Phase 2 judgment-and-execution process
already used for `G-FR-01`…`G-FR-13` and `MG-01`…`MG-11` — most carry `High` confidence and directly extend
principles the Lead Architect already approved (single-aggregate-lifecycle consolidation, effective-dating
mechanism ownership, severity-as-configurable-attribute), so they are properly `DETERMINED` rather than
requiring fresh debate. The one `REVIEW-REQUIRED` item (`G-202`) and the four `UNCERTAIN` Domains rows in §6
need a genuine Phase 2 judgment call, not a mechanical application of an existing principle. Section G does
not need a substantial re-audit or structural rework — its underlying boundary discipline (L2-traceable
titles, no invented events/states, correct ownership separation from H07/A07/J02) remains sound.

---

## Quality Score

Scored against the CURRENT (pre-this-audit) 350-candidate state:

| Dimension | Score | Note |
|---|---:|---|
| Outcome granularity | 7/10 | 29/350 rows are lifecycle-portion or trigger-variant restatements of a sibling row |
| Coverage | 9/10 | All 11 L2 documents represented; no missing institutional outcome surfaced in this pass |
| Ownership integrity | 9/10 | G07/G08/G10 cross-domain handoffs (J02, H07, A07, B03) are consistently scoped to evidence-supply, not ownership transfer |
| Cross-domain consistency | 8/10 | Domains field is accurate for ~98% of rows; residual D/I/H impact-only inflation confined to 7 named rows |
| Traceability | 9/10 | Sparse-ID crosswalk and watchlist documentation are exemplary; every retired ID is accounted for |
| Classification quality | 8/10 | Type/Priority distributions are risk-based, not uniform; no evidence of mechanical assignment |
| L3 boundary correctness | 7/10 | Same 29-row gap as granularity — recurrence of already-approved patterns not yet swept section-wide |

**CURRENT SECTION G SCORE: 8/10**

**EXPECTED SCORE AFTER EVIDENCE-BACKED FIXES: 9/10** — reflects the same architecture, with the granularity
and boundary-correctness gaps closed by executing §4; not increased merely because the count would drop.

---

## Final Safety Review (§17 self-check)

- Did I recommend merging because the count looked large? No — every §4 disposition cites a specific L2
  workflow diagram or state-transition table; §5 explicitly documents five look-alike clusters kept
  separate.
- Did I use similar wording as proof of duplication? No — `G-334`/`G-518`+`G-520` were flagged because the
  *inventory's own* trigger/mechanism text overlaps with an existing row's stated scope, not because titles
  merely sound alike; `G-330` vs `G-321` was explicitly kept separate despite shared "inspection" wording.
- Did I invent a broader parent scenario unsupported by L2? No — every recommended parent already exists
  in the current 350; none was newly authored.
- Did I convert an exception into a separate scenario without evidence, or the reverse? No new exception
  scenarios were created; `G-333` (escalated hygiene risk) was folded in because L2's own aggregate table
  places it inside the same Hygiene Inspection lifecycle, not because it is "just an exception."
- Did I silently change ownership? No — all recommended parents retain the same Primary Owner as the rows
  being absorbed.
- Did I treat participation as ownership? No — §6 explicitly separates "affected by" from "materially
  participates in producing."
- Did I reverse an approved watchlist decision without stronger evidence? No — every finding in §4 extends
  a principle the watchlist already marked `APPLIED`/`DETERMINED` to rows the watchlist's own root-cause
  note admits were not yet swept; no `CLOSED`/`APPLIED` decision is contradicted.
- Did I optimize for a target count? No — §8 states the arithmetic result only; no target was set going in
  or adjusted to reach afterward.

---

**Report:** `generated/reviews/L3_Section_G_350_Candidate_Independent_Boundary_Audit.md`
