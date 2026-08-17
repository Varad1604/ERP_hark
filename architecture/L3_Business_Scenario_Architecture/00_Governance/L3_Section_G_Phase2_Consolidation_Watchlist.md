# L3 Section G — Phase 2 Consolidation Watchlist

**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section G — Campus Operations, 350 current sparse candidates in range `CAN-L3-G-001` … `CAN-L3-G-541`; 541 former candidates
**Status:** Determined consolidation applied — open Phase 2 judgments retained
**Created:** 2026-08-11
**Source:** independent adversarial audit of `L3_Phase1_Inventory_Varad.md`, 2026-08-11
**Corrections applied:** `generated/reviews/l3-phase1-section-g-corrections-2026-08-11.md`

---

## 1. Purpose

This document holds the candidate relationships, boundary questions and blocked decisions identified during the independent Section G audit of 2026-08-11, and records for each one whether it is settled or still requires a Phase 2 decision.

It exists as a separate document because `L3_Phase1_Discovery_Rules.md` section 3 requires each Phase 1 inventory file to contain candidate scenario rows only, and section 16 requires the inventory to avoid long explanations.

The one-sided and user-approved clusters in this document were consolidated on 2026-08-12. Seventy-two fragment rows were retired and eleven retained rows were rewritten as complete outcomes. Former IDs remain listed here for traceability; genuinely open clusters remain untouched.

This document is not a governance standard. It adds and amends no Phase 1 rule and resolves no architectural conflict.

It follows the structure established by `L3_Section_A_Phase2_Consolidation_Watchlist.md` and `L3_Section_C_Phase2_Consolidation_Watchlist.md`.

---

## 2. Scope Warning

Section G is one of three sections assigned to `L3_Phase1_Inventory_Varad.md` under `L3_Phase1_Workload_Distribution_Decision.md` section 2.

Sections F and H are now present. This watchlist covers Section G only; its former notification, dashboard and accreditation blockers were resolved by the accepted Section H ownership decisions on 2026-08-12.

---

## 3. Status Values

| Status | Meaning | Phase 2 action |
| ------ | ------- | -------------- |
| `CLOSED` | Repository evidence establishes the candidates are distinct or the question is settled. | None. Do not re-raise. |
| `DETERMINED` | Repository evidence is one-sided. The decision is recorded; only its execution is deferred. | Execute the recorded decision. |
| `OPEN` | A genuine Phase 2 judgment is required. | Decide, then execute. |
| `BLOCKED` | Cannot be decided from current repository evidence. | Resolve the blocker first. |

---

## 4. Fragmentation Clusters

| ID | Cluster | Candidates | Combined outcome | Coverage | Status |
| --- | --- | ---: | --- | --- | --- |
| G-FR-01 | Cross-cutting audit history, one per sub-domain | 12 — `G-020, G-059, G-090, G-118, G-155, G-192, G-226, G-301, G-365, G-413, G-431, G-528` | Governed institutional audit evidence | Preserved in each parent G journey and governed centrally by A07 | `APPLIED` — user selected Option 1 on 2026-08-13; standalone G audit-history candidates retired in favour of Section A `CAN-L3-A-023`…`A-030` |
| G-FR-02 | Cross-cutting configuration change | 17 former rows — broad parents `G-018, G-060, G-089, G-117, G-154, G-191, G-227, G-295, G-358, G-449, G-518`; fragments `G-296…G-298, G-359…G-361` | One effective business-policy configuration outcome per G subdomain; Section A supplies the governed configuration mechanism | Preserved in 11 domain-owned parents | `APPLIED` — user selected Option 1 on 2026-08-13; six G07/G08 fragments absorbed into rewritten `G-295` and `G-358` |
| G-FR-03 | Authorization failure | 7 — `G-300, G-363, G-419, G-420, G-491, G-526, G-527` | Authorization is a rule/alternate denial inside the governed parent journey | Preserved in L2/L4 | `APPLIED` — standalone fragments retired. |
| G-FR-04 | G07 single-step movement | 34 former rows — `G-234…G-267` | Eight complete journeys: person access; visitor authorization/pass; visitor presence/closure; vehicle movement; material movement; courier visit; contractor access; patrol | Preserved in `G-234, G-238, G-244, G-247, G-252, G-257, G-259, G-261` | `APPLIED` — user selected Option 1 on 2026-08-13; 26 step rows absorbed, with denial, override, expiry, overstay and patrol deviations retained as alternate outcomes |
| G-FR-05 | G10 "invalid X to controlled rejection" | 14 — `G-451, G-455, G-457, G-459, G-462, G-464, G-468, G-472, G-479, G-480, G-484, G-489, G-512, G-519` | Validation-failure branch of the preceding candidate | Preserved in parent | `APPLIED` — standalone rejection fragments retired. |
| G-FR-06 | G09 audited correction | 4 — `G-415, G-416, G-417, G-418` | Governed correction of a mentorship record | `G-415` retained | `APPLIED` — four variants consolidated to one. |
| G-FR-07 | G08 re-clean | 4 — `G-316, G-326, G-332, G-337` | Verified re-clean after quality or hygiene failure | `G-316` rewritten; distinct request/inspection origins retained | `APPLIED` — exact duplicate `G-337` retired. |
| G-FR-08 | Notification and awareness | 39 — incl. `G-115, G-151, G-152, G-184…G-187, G-205, G-221…G-223, G-280, G-289…G-294, G-353…G-356, G-425…G-429` | Publisher-side business facts feed H02; H02 owns delivery | Preserved by source parents and `CAN-L3-H-010`/`H-014` | `APPLIED` — notification-only fragments retired; genuine source information outcomes retained. |
| G-FR-09 | Reporting and dashboards | 43 former rows — incl. `G-193, G-228, G-303…G-306, G-351, G-352, G-366…G-369, G-432…G-437, G-448, G-530…G-537` | G owns source reports, KPIs and projections; H07 owns dashboard composition; A07 owns institutional audit reconstruction | Preserved by domain reports plus `CAN-L3-H-050`…`H-058` and A07 | `APPLIED` — consolidated security, housekeeping, mentoring and infrastructure report families; retired duplicate H07 dashboard and A07 reconstruction rows |

---

## 4A. Second-Pass Consolidation — 2026-08-14

The all-scope Phase 1 audit of 2026-08-14 found that the reasoning accepted for `G-FR-08` (notification) and `G-FR-09` (reporting and dashboards) had not been extended to three further families. Applied under user authorization on 2026-08-14.

| ID | Cluster | Retired | Absorbed into | Basis |
| --- | --- | --- | --- | --- |
| `G-FR-10` | G10 effective-dated attribute maintenance, one row per attribute class | `G-460`, `G-465`, `G-470`, `G-473`, `G-505`, `G-507` | `G-454`, rewritten as **Physical Infrastructure Attribute Change to Effective-Dated Governed Update** and retyped `Critical` to carry the inherited governance sensitivity of capability and security-classification changes | Six rows repeated one effective-dated attribute-maintenance outcome across Building, Space, capability, capacity, sustainability and security classification. Definition and establishment outcomes were **retained**; only the change outcomes were consolidated. |
| `G-FR-11` | G10 physical campus profile population | `G-453` | `G-450`, Final Outcome extended to include the authorised physical attribute profile | Profile completion is part of establishing the Physical Campus, not a separate institutional outcome. |
| `G-FR-12` | G10 event delivery, consumer refresh and read contract | `G-523`, `G-524`, `G-525`, `G-538` | Publishing parents (`G-454`, `G-475`) | Consumer-side event delivery, read-model reconciliation and a query contract are mechanism, not institutional outcomes. Same class as `G-FR-08`/`G-FR-09`. `L3_Phase1_Discovery_Rules.md` §8. |
| `G-FR-13` | G01 work-order internal steps | `G-027`, `G-051`, `G-052` | `G-025` Facility Work Order to Verified Closure | `G-025`'s existing Final Outcome already reads "Work is **assigned, accepted**, executed, evidenced, **verified, and closed**" — the three retired rows were restatements of spans inside it. `G-027`'s outcome ("a technician… is ready to execute the work") is not an institutional result. **No rewrite of `G-025` was needed.** |

### Domain participation correction

`G-023`, `G-026`, `G-033`, `G-034`, `G-047` and `G-051` listed `D` and `I` as participating domains while their Final Outcomes are wholly internal to facilities operations. `Domains` narrowed to `G` per `L3_Phase1_Discovery_Rules.md` §11 ("Use only domains that genuinely participate"). Academic Delivery and Examination are *affected by* a facility issue; they perform no act in consolidating duplicate complaints or verifying maintenance.

**`OPEN` — residual participation review.** Further G01/G02 rows list `D`, `I` or `H` on the same impacted-not-participating basis (for example `G-022`, `G-032`, `G-044`, `G-045`, `G-055`, `G-058`, `G-076`). These were **not** changed: each needs a row-by-row judgment, and `H` in particular requires distinguishing genuine Section H business participation from notification delivery, which §11 excludes. Phase 2 should complete this pass.

Section G total: **391 → 377 candidates** at the 2026-08-13 correction, then **377 → 350** at the 2026-08-15 audit application (see section 10). Retired IDs must not be reused. Audit reference: `generated/reviews/l3-phase1-audit-all-2026-08-14.md` FIND-005, FIND-011, MG-07…MG-09.

---

## 5. Over-Broad Candidates

| ID | Candidate | Hidden outcomes | Status |
| --- | --- | --- | --- |
| G-SP-01 | Former `G-001` Campus Readiness Assessment to Readiness Decision | Read as both the daily type and generic engine | `APPLIED` — narrowed to Daily Campus Readiness. |
| G-SP-02 | Former `G-017` Governance Exception to Resolution | Mixed six unrelated trigger conditions | `APPLIED` — narrowed to evidence/dependency exceptions; authorization and overdue-action fragments absorbed elsewhere. |
| G-SP-03 | `G-088, G-116, G-153, G-188, G-224, G-302, G-364, G-447, G-539` — the parallel "Operational Exception to Recovery" family | Each enumerated unrelated failures already covered by specific recovery journeys or parent alternate outcomes | `APPLIED` — user selected Option 1 on 2026-08-13; all nine vague catch-all candidates retired; material recoveries remain explicit |
| G-SP-04 | Former `G-511` Infrastructure Lifecycle Transition to Valid State Progression | Duplicated availability and decommissioning outcomes | `APPLIED` — generic restatement retired; `G-475` owns suspension/restoration and `G-513`…`G-515` retain decommissioning. |
| G-SP-05 | Former `G-445` Mentorship Case State Transition to Valid Lifecycle Progression | Generic state-machine restatement | `APPLIED` — retired; specific relationship and case outcomes remain. |
| G-SP-06 | `G-379` Student Context Retrieval to Mentoring Readiness | Spanned six source domains and duplicated `G-440`…`G-444` context views | `APPLIED` — retained and rewritten `G-379` as the single privacy-controlled context outcome; five source-specific read-model fragments retired |

---

## 6. Overlap Clusters

| ID | Candidates | Assessment | Status |
| --- | --- | --- | --- |
| G-OV-01 | `G-316`, `G-337` | Same trigger, journey, outcome, owner and domains | `APPLIED` — retained rewritten `G-316`. |
| G-OV-02 | `G-319`, `G-325` | Emergency cleaning differed only by origin | `APPLIED` — retained rewritten `G-319` with critical complaint as an alternate trigger. |
| G-OV-03 | `G-105`, `G-106`, `G-107` | Maintenance/admin destinations were branches of one complaint | `APPLIED` — retained rewritten `G-106`. |
| G-OV-04 | `G-048`, `G-049` | Satisfied and breached were outcomes of one SLA lifecycle | `APPLIED` — retained rewritten `G-048`. |
| G-OV-05 | `G-092`, `G-094`, `G-095` | One journey from allocation to active residency | `APPLIED` — retained rewritten `G-092`. |
| G-OV-06 | `G-125`, `G-126` | Consecutive states of one transport journey | `APPLIED` — retained rewritten `G-125`. |
| G-OV-07 | `G-390`, `G-391`…`G-397`, `G-399`, `G-406` | Destination is data within one referral lifecycle | `APPLIED` — retained rewritten `G-390`. |
| G-OV-08 | `G-475`, `G-476`, `G-477` vs `G-511` | Same state changes represented at two altitudes | `APPLIED` — retained rewritten `G-475`; generic `G-511` retired. |
| G-OV-09 | `G-077, G-105, G-137, G-148, G-339, G-357, G-499` | Originating G domains publish maintenance evidence to G01 without taking maintenance ownership. `G-105` was absorbed into `G-106`; duplicate housekeeping rows `G-339`/`G-357` were consolidated into rewritten `G-339`; the other originating outcomes remain distinct. | `APPLIED` — ownership preserved and duplicate housekeeping handoff removed |
| G-OV-10 | `G-063`, `G-083` vs former `G-476` | G02 consumption outcomes remain valid; the former G10 publication fragment was absorbed into `G-475` | `CLOSED` — ownership remains separated. |

---

## 7. Cross-Inventory Relationships

| ID | Section G | Other inventory | Assessment | Status |
| --- | --- | --- | --- | --- |
| G-XI-01 | `G-039` Vendor Performance Review to Operational Evaluation | `CAN-L3-J-028` Supplier Performance or Incident Evidence to Governed Supplier Assessment Record | G01 owns operational evaluation evidence; J02 owns the canonical supplier assessment record. | `APPLIED` — `G-039` now ends at published operational evidence for J02; no owner transfer |
| G-XI-02 | `G-038`, `G-350` | `CAN-L3-J-029` Supplier Risk Finding to Governed Restriction | G01/G08 own service correction and breach evidence; J02 alone owns supplier restriction. | `APPLIED` — both G outcomes explicitly stop at operational correction and evidence handoff |
| G-XI-03 | `G-006` Campus Operations Evidence to Accreditation Evidence Readiness | `CAN-L3-H-073`; `CAN-L3-A-047` | G00 supplies campus evidence; H09 owns accreditation readiness; A10 owns persisted operational governance/certification under the INT-302 ruling, while L1-06 owns dynamic capability readiness. | `CLOSED` — boundaries are distinct and recorded in `H-DEC-04`; INT-302 resolved 2026-08-13. |

---

## 8. Open Items

| ID | Item | Status |
| --- | --- | --- |
| G-OI-01 | Ownership of approximately 20 reporting and dashboard candidates (`G-303`…`G-306`, `G-366`…`G-368`, `G-530`…`G-537`). Source G domains own operational reports, KPI definitions and projections; H07 owns dashboard composition, orchestration and governance. | `APPLIED` — source report families consolidated; duplicate dashboard candidates retired; retained outcomes explicitly stop at dashboard-ready projections |
| G-OI-02 | Priority distribution requires re-count and institutional review after consolidation. | `APPLIED` — 162 `P0`, 222 `P1`, 7 `P2`; first-release priority is retained for core safety, health, custody, access, lifecycle and readiness outcomes |
| G-OI-03 | Type distribution requires re-count and risk review after consolidation. | `APPLIED` — 156 `Critical`, 141 `Cross-domain`, 94 `Simple`; internal G codes normalized to section `G`, and platform/notification participation alone no longer forces Cross-domain |
| G-OI-04 | Wing and Block registration. `L2-G10` section 12 lists it as a registration workflow, but section 8B establishes a Wing as an optional child entity of the Building Aggregate and explicitly not an independent Aggregate Root. | `CLOSED` — folded into `CAN-L3-G-458` Building Registration. Confirmed 2026-08-11. Do not re-raise |
| G-OI-05 | `CAN-L3-G-540` and `CAN-L3-G-541` carry IDs deliberately non-contiguous with their subsection blocks. Renumbering 519 rows to insert two was rejected as higher risk. Phase 1 IDs are temporary under section 6. | `CLOSED` — informational |

---

## 9. Items Confirmed Sound

Recorded so Phase 2 does not re-audit them.

- **Canonical operational ownership.** Source business ownership remains sound. Dashboard composition, accreditation readiness, audit history, configuration mechanisms, and supplier-governance handoffs are now explicitly separated from the source G outcomes.
- **Lifecycle states.** Every state term used in the inventory was verified present in the governing L2 document. No invented state.
- **Business events.** Candidates refer to events generically. No event name was fabricated.
- **Implementation leakage.** None. No API, schema, class, screen, broker, queue or deployment reference appears in any candidate.
- **Candidate IDs.** Unique across all 350 current rows; sparse former IDs preserve the consolidation crosswalk.
- **L2 document coverage.** All eleven Section G documents are represented.

---

## 10. Consolidation Applied 2026-08-12

The user authorized the all-assigned audit recommendations. Through the 2026-08-13 boundary, ownership and classification decisions, the correction has retired **150** standalone fragments and rewritten **30** retained candidates as complete outcomes. Section G moved from 541 to **391** candidates without renumbering; sparse IDs preserve exact traceability.

Applied groups:

- authorization-denial fragments `G-300`, `G-363`, `G-419`, `G-420`, `G-491`, `G-526`, `G-527` absorbed as rules or alternate outcomes;
- fourteen G10 invalid-input/invalid-transition fragments absorbed into their parent lifecycles;
- notification-only rows absorbed into source-domain parent journeys and H02 communication scenarios;
- paired and grouped outcomes consolidated onto rewritten `G-048`, `G-092`, `G-106`, `G-125`, `G-316`, `G-319`, `G-390`, `G-415`, and `G-475`;
- over-broad `G-001` and `G-017` narrowed; generic state-machine restatements `G-445` and `G-511` retired; and
- representative dashboard candidates `G-305` and `G-368` re-owned to H07, while G07/G08 retain report, KPI and projection ownership.
- twelve standalone audit-history candidates retired; their parent G journeys retain audit capture while A07 owns the governed institutional audit trail.
- six G07/G08 policy fragments absorbed into domain-owned `G-295` and `G-358`; Section A provides the configuration mechanism without taking ownership of G business rules.
- twenty-six G07 access, movement and patrol steps absorbed into eight complete business journeys, with material denial, override and recovery outcomes preserved inside their parents.
- nine heterogeneous "Operational Exception to Recovery" catch-alls retired; specific recovery journeys remain, and routine failures stay as alternate outcomes in their parent scenarios.
- five mentoring source-view fragments absorbed into privacy-controlled `G-379`;
- supplier operational evaluation/correction candidates bound explicitly to J02 evidence consumption without transferring canonical supplier ownership; and
- security, housekeeping, mentoring and infrastructure reporting consolidated, with duplicate H07 dashboard and A07 reconstruction candidates retired.
- duplicate housekeeping maintenance handoff `G-357` absorbed into rewritten `G-339`, and the remaining A07-owned infrastructure reconstruction row `G-517` retired; and
- all Domains normalized to section-level identifiers, followed by a risk-first Type and first-release Priority review.

**Superseded 2026-08-15.** The claim that no Section G decision remained open was incorrect when written: section 4A already recorded an `OPEN` residual participation review, and the 2026-08-14 re-audit identified further surviving instances of consolidation principles recorded as `APPLIED`. See section 11 for what was subsequently applied and what remains.


---

## 11. 2026-08-15 Audit Application

Applied under Lead Architect approval following `generated/reviews/l3-phase1-audit-all-2026-08-14-2.md`.
Full record: `generated/reviews/L3_Phase1_Tranche3_Application_Report_2026-08-15.md`.

**Section G: 377 -> 350 candidates.** 27 retirements, 10 retained candidates rewritten to
absorb the retired outcomes. Retired IDs must not be reused.

| Cluster | Retired | Retained and rewritten |
|---|---|---|
| MG-01 rejection-only rows (extends `G-FR-03`, `G-FR-05`, `G-FR-13`) | `G-322`, `G-371`, `G-372`, `G-388`, `G-398`, `G-446` | `G-321`, `G-370`, `G-387` |
| MG-05 consumer-side read model (extends `G-FR-12`) | `G-438`, `G-439` | — |
| DUP-01 hostel exit span | `G-108`, `G-110` | `G-096` |
| MG-09 publisher-side information delivery (extends `G-FR-08`) | `G-113`, `G-114`, `G-150`, `G-170` | `G-169` |
| MG-02 mentoring session recording steps | `G-382`, `G-383`, `G-384`, `G-385` | `G-381` |
| MG-03 mentoring follow-up transitions | `G-403`, `G-405`, `G-407`, `G-408` | `G-404` |
| MG-04 mentoring disclosure requester variants | `G-422`, `G-423`, `G-424` | `G-421` |
| MG-10 hostel visit span | `G-101` | `G-100` |
| MG-11 resident leave span | `G-098` | `G-097` |

**Root cause.** The principles behind `G-FR-05`, `G-FR-08` and `G-FR-12` were recorded as
`APPLIED` but were applied cluster-by-cluster rather than swept across the section. Identical
constructs survived in G08 Housekeeping and G09 Teacher Guardian, which no cluster had reached.
Recording a principle as `APPLIED` should in future require a section-wide sweep for the
construct, not only the enumerated cluster.

**Still open.** The residual participation review at section 4A. Type and Priority remain
subject to the ordinary Phase 2 approval gate.

---

## 12. 2026-08-17 Targeted Phase 2 Remediation

**Source audit:** `generated/reviews/L3_Section_G_350_Candidate_Independent_Boundary_Audit.md` (independent
adversarial boundary re-audit of the current 350 candidates, 2026-08-17).
**Applied to:** `architecture/L3_Business_Scenario_Architecture/02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md`.
**Application report:** `generated/reviews/L3_Section_G_Phase2_Targeted_Consolidation_Application.md`.
**Scope:** exactly the twelve clusters below (SG-01…SG-12). No further merges were discovered or applied
during this remediation pass. The Phase 1 inventory (`L3_Phase1_Inventory_Varad.md`) was **not** modified;
all Phase 1 Candidate IDs remain preserved as traceability keys, and no absorbed ID was reused.

This section does not erase or supersede the `G-FR-*`/`G-SP-*`/`G-OV-*`/`G-XI-*`/`MG-*` history above; it
records a separate, later pass that found further evidence-backed fragmentation the earlier sweeps missed.

| ID | Candidates | Surviving parent | Disposition | Authoritative evidence | Application status | Date |
| --- | --- | --- | --- | --- | --- | --- |
| SG-01 | `G-003, G-004, G-005, G-540` absorbed; `G-543` **excluded** | `CAN-L3-G-542` (rewritten) | VARIANT | L2-G00 "Campus Readiness Workflow" defines readiness type as configurable data within one workflow. | `APPLIED` for G-003/004/005/540. `G-543` — **REVALIDATION REQUIRED, NOT APPLIED**: carries a pre-existing `BLOCKED` status under `I-XI-03` (cross-inventory boundary conflict with Section I, higher authority rank than this Section G audit) that this remediation did not have standing to resolve. See §13 open item below. | 2026-08-17 |
| SG-02 | `G-047` absorbed | `CAN-L3-G-562` | MERGE | L2-G01 Inspection Workflow "Corrective Action Required? -> Yes -> Work Order Generated" feeds the same Work Order aggregate `G-562` already owns; extends `G-FR-13`. | `APPLIED` | 2026-08-17 |
| SG-03 | `G-270, G-271, G-272` MERGE; `G-273, G-274` VARIANT absorbed; `G-744` **excluded** | `CAN-L3-G-743` (rewritten) | MERGE / VARIANT | L2-G07 single "Incident Workflow" diagram and Security Incident state-transition table (Reported -> Classification -> Severity Assessment -> Investigation -> Resolution -> Closure). `G-744` (Emergency Response coordination) is a separately-diagrammed, materially different outcome. | `APPLIED` | 2026-08-17 |
| SG-04 | `G-311` absorbed | `CAN-L3-G-762` (rewritten) | MERGE | L2-G08 aggregate lifecycle table: Cleaning Schedule = `Draft -> Approved -> Active -> Completed`, one aggregate. | `APPLIED` | 2026-08-17 |
| SG-05 | `G-313, G-315` absorbed | `CAN-L3-G-763` (rewritten) | MERGE | Same table: Cleaning Task = `Generated -> Assigned -> Executing -> Completed`, one aggregate. | `APPLIED` | 2026-08-17 |
| SG-06 | `G-323, G-324, G-326` absorbed | `CAN-L3-G-770` (rewritten) | MERGE | L2-G08 "Complaint Workflow" diagram: `Registered -> Validation -> Priority Assignment -> Operational Assignment -> Cleaning Completed -> Inspection -> Closure`, one workflow. | `APPLIED` | 2026-08-17 |
| SG-07 | `G-331, G-332, G-333` absorbed | `CAN-L3-G-774` (rewritten) | MERGE | Aggregate lifecycle table: Hygiene Inspection = `Scheduled -> Inspection -> Evaluated -> Closed`, one aggregate with branch outcomes. | `APPLIED` | 2026-08-17 |
| SG-08 | `G-334` absorbed | `CAN-L3-G-774` | DUPLICATE | `G-774`'s own Trigger already enumerates "washroom" as a covered variant. | `APPLIED` | 2026-08-17 |
| SG-09 | `G-400, G-401, G-402` absorbed | `CAN-L3-G-808` (rewritten) | MERGE | L2-G09 §15 "Referral Workflow" is one diagrammed workflow (`Mentoring Discussion -> ... -> Referral Closed`); `G-808` already covered steps 1-4. | `APPLIED` | 2026-08-17 |
| SG-10 | `G-514, G-515` absorbed; `G-859` **excluded** | `CAN-L3-G-858` (rewritten) | MERGE | L2-G10 `WF-INF-013 — Decommission Infrastructure`, one workflow. `G-859` Archive is the separate `WF-INF-014`. | `APPLIED` | 2026-08-17 |
| SG-11 | `G-496, G-497` absorbed | `CAN-L3-G-846` (rewritten) | MERGE | L2-G10 §32 "Inspection Workflow", one diagrammed workflow (`Scheduled -> ... -> Closed`). | `APPLIED` | 2026-08-17 |
| SG-12 | `G-520` absorbed | `CAN-L3-G-860` | DUPLICATE | Same generic Section-A-owned effective-dating mechanism already retired elsewhere under `G-FR-02`/`G-FR-12`. | `APPLIED` | 2026-08-17 |

**Explicitly preserved as independent (not merged despite surface similarity to the clusters above):**
`CAN-L3-G-544`, `CAN-L3-G-702`, `CAN-L3-G-704`, `CAN-L3-G-744`, `CAN-L3-G-843`, `CAN-L3-G-844`,
`CAN-L3-G-845`, `CAN-L3-G-859`. Each has distinct L2 evidence recorded in the source audit §5.

**`CAN-L3-G-705` (First Aid) — left `REVIEW-REQUIRED`, not merged.** L2-G06 documents "Emergency & First
Aid Management" as a single §19 heading with one diagrammed Emergency Workflow, while First Aid is
described only as a list of operational activities without its own workflow diagram — insufficient
evidence to decide independence either way. `INSUFFICIENT EVIDENCE — DO NOT CONSOLIDATE`.

**Domains-field corrections applied:** `G-559` (removed D, I), `G-568` (retained E; removed D, I), `G-580`
(removed D, I; H left `REVIEW-REQUIRED`), `G-581` (removed D, I; H left `REVIEW-REQUIRED`), `G-587`
(removed D, I; H left `REVIEW-REQUIRED`), `G-607` (removed D, I; H left `REVIEW-REQUIRED`). `G-590` was
**not** changed automatically — its Domains participation is itself `REVIEW-REQUIRED` because D/I may
materially create the scheduling conflict even though G/E own its resolution. This continues (but does not
close) the residual participation review opened at §4A; H's status for `G-580/045/055/076` remains
unresolved.

**Open item carried forward: SG-01 partial application.** `CAN-L3-G-543` could not be absorbed into
`CAN-L3-G-542` under SG-01 because of the pre-existing `I-XI-03` block, which the source Section G audit
did not have visibility into (it audits Section G only). This is recorded as `REVALIDATION REQUIRED`, not
`APPLIED` — resolving it requires the same Section H/I boundary confirmation `I-XI-03` already calls for,
not a new Section G decision. As a direct consequence, the evidence-supported Section G scenario count
after this remediation is **322**, not the source audit's estimate of 321.

**Section G total at this point: 350 candidates on the Phase 1 inventory (unchanged by this §12); 322 independently proposed
Phase 2 canonical scenarios** after this remediation (320 KEEP + 1 BLOCKED `G-543` + 1 REVIEW-REQUIRED
`G-705`, each still counted as a standing candidate pending its own separate resolution). 28 candidates
(20 MERGE + 6 VARIANT + 2 DUPLICATE) are absorbed and no longer independently proposed.

**Superseded by §13 below.** This §12 pass modified the Phase 2 register only and left the Phase 1
inventory's 350 rows untouched, pending a separate authorization to remove absorbed rows from the active
Phase 1 set. That authorization was given the same day; §13 records the Phase 1 row removal itself.

---

## 13. Independent Boundary Re-Audit Application — 2026-08-17

**Source audit:** `generated/reviews/L3_Section_G_350_Candidate_Independent_Boundary_Audit.md`.
**Applied to:** `architecture/L3_Business_Scenario_Architecture/01_Phase1_Discovery/L3_Phase1_Inventory_Varad.md`
(Section G portion only) **and** the Phase 2 register (already updated under §12).
**Application report:** `generated/reviews/L3_Section_G_Phase1_and_Phase2_Targeted_Remediation_Application.md`.
**Authorization:** explicit remediation instruction authorizing modification of both the Phase 1 Section G
inventory and the Phase 2 register, 2026-08-17.

This section records the physical removal of the 28 absorbed rows from the **active Phase 1 Section G
inventory**. Retired IDs are not reused; the sparse ID range is preserved. This does not erase or
contradict §12 — it is the Phase 1-side execution of the same 12 clusters, re-validated against the current
repository state immediately before application (candidates confirmed to still exist, wording confirmed
unchanged, L2 evidence re-checked, no newer decision found superseding it, owners confirmed unchanged).

**Revalidation result:** all 11 clusters not touching `CAN-L3-G-543` revalidated clean and were applied.
SG-01 revalidated **partially**: `I-XI-03` (Section I Phase 2 Consolidation Watchlist §10/§11, `OPEN`) is
still unresolved, so `CAN-L3-G-543` was **not** removed from the active Phase 1 inventory and remains a
standing candidate. This is why the active Section G count is **322**, not 321.

## Absorbed Candidate Crosswalk

| Cluster | Former Candidate ID | Disposition | Surviving Candidate ID | Reason | L2 Evidence | Application Status |
| --- | --- | --- | --- | --- | --- | --- |
| SG-01 | `CAN-L3-G-003` | VARIANT | `CAN-L3-G-542` | Configurable readiness type, not a separate workflow | `L2-G00` "Campus Readiness Workflow" | `APPLIED` |
| SG-01 | `CAN-L3-G-004` | VARIANT | `CAN-L3-G-542` | same | same | `APPLIED` |
| SG-01 | `CAN-L3-G-005` | VARIANT | `CAN-L3-G-542` | same | same | `APPLIED` |
| SG-01 | `CAN-L3-G-540` | VARIANT | `CAN-L3-G-542` | same | same | `APPLIED` |
| SG-01 | `CAN-L3-G-543` | VARIANT (targeted) | `CAN-L3-G-542` | Same readiness-type reasoning applies, **but** the candidate carries a pre-existing higher-authority block | `I-XI-03` (Section I Watchlist), unresolved | `REVALIDATION REQUIRED` — **NOT APPLIED**, remains active in Phase 1, unchanged |
| SG-02 | `CAN-L3-G-047` | MERGE | `CAN-L3-G-562` | Inspection-sourced work order restates `G-562`'s existing scope; extends `G-FR-13` | `L2-G01` Inspection Workflow -> Work Order Generated branch | `APPLIED` |
| SG-03 | `CAN-L3-G-270` | MERGE | `CAN-L3-G-743` | Consecutive span of one Incident Workflow | `L2-G07` "Incident Workflow" | `APPLIED` |
| SG-03 | `CAN-L3-G-271` | MERGE | `CAN-L3-G-743` | same | same | `APPLIED` |
| SG-03 | `CAN-L3-G-272` | MERGE | `CAN-L3-G-743` | same | same | `APPLIED` |
| SG-03 | `CAN-L3-G-273` | VARIANT | `CAN-L3-G-743` | Severity is configurable within the same workflow | same | `APPLIED` |
| SG-03 | `CAN-L3-G-274` | VARIANT | `CAN-L3-G-743` | Critical-severity branch of the same workflow; `G-744` stays separate | same | `APPLIED` |
| SG-04 | `CAN-L3-G-311` | MERGE | `CAN-L3-G-762` | Consecutive stage of one Cleaning Schedule aggregate | `L2-G08` aggregate lifecycle table | `APPLIED` |
| SG-05 | `CAN-L3-G-313` | MERGE | `CAN-L3-G-763` | Consecutive stage of one Cleaning Task aggregate | same | `APPLIED` |
| SG-05 | `CAN-L3-G-315` | MERGE | `CAN-L3-G-763` | same | same | `APPLIED` |
| SG-06 | `CAN-L3-G-323` | MERGE | `CAN-L3-G-770` | Consecutive stage of one Complaint Workflow | `L2-G08` "Complaint Workflow" | `APPLIED` |
| SG-06 | `CAN-L3-G-324` | MERGE | `CAN-L3-G-770` | same | same | `APPLIED` |
| SG-06 | `CAN-L3-G-326` | MERGE | `CAN-L3-G-770` | same | same | `APPLIED` |
| SG-07 | `CAN-L3-G-331` | MERGE | `CAN-L3-G-774` | Branch outcome of one Hygiene Inspection aggregate | `L2-G08` aggregate lifecycle table | `APPLIED` |
| SG-07 | `CAN-L3-G-332` | MERGE | `CAN-L3-G-774` | same | same | `APPLIED` |
| SG-07 | `CAN-L3-G-333` | MERGE | `CAN-L3-G-774` | same | same | `APPLIED` |
| SG-08 | `CAN-L3-G-334` | DUPLICATE | `CAN-L3-G-774` | Washroom already an explicit trigger variant of `G-774` | `G-774` Trigger field | `APPLIED` |
| SG-09 | `CAN-L3-G-400` | MERGE | `CAN-L3-G-808` | Consecutive step of one Referral Workflow | `L2-G09` §15 "Referral Workflow" | `APPLIED` |
| SG-09 | `CAN-L3-G-401` | MERGE | `CAN-L3-G-808` | same | same | `APPLIED` |
| SG-09 | `CAN-L3-G-402` | MERGE | `CAN-L3-G-808` | same | same | `APPLIED` |
| SG-10 | `CAN-L3-G-514` | MERGE | `CAN-L3-G-858` | Consecutive stage of one `WF-INF-013` workflow | `L2-G10` `WF-INF-013` | `APPLIED` |
| SG-10 | `CAN-L3-G-515` | MERGE | `CAN-L3-G-858` | same | same | `APPLIED` |
| SG-11 | `CAN-L3-G-496` | MERGE | `CAN-L3-G-846` | Consecutive stage of one Inspection Workflow | `L2-G10` §32 "Inspection Workflow" | `APPLIED` |
| SG-11 | `CAN-L3-G-497` | MERGE | `CAN-L3-G-846` | same | same | `APPLIED` |
| SG-12 | `CAN-L3-G-520` | DUPLICATE | `CAN-L3-G-860` | Same Section-A effective-dating mechanism as `G-860` | Watchlist §4A root cause; `G-FR-02`/`G-FR-12` precedent | `APPLIED` |

**Totals: 28 `APPLIED` (20 MERGE + 6 VARIANT + 2 DUPLICATE), 1 `REVALIDATION REQUIRED` (`CAN-L3-G-543`).**

## G-705 Status

`CAN-L3-G-705` (First Aid Incident to Stabilised Patient Outcome) remains **active** in the Phase 1
inventory, unmodified. Disposition: `REVIEW-REQUIRED`. `INSUFFICIENT EVIDENCE — DO NOT CONSOLIDATE`. L2-G06
documents "Emergency & First Aid Management" as a single §19 heading with one diagrammed Emergency
Workflow; First Aid itself has no independent workflow diagram, so neither independence nor absorption can
be confirmed from current L2 evidence.

## Remaining Domains-Field Questions

Continuing the §4A residual participation review, now applied directly to the active Phase 1 rows:

- `CAN-L3-G-559`, `CAN-L3-G-568` — closed. D/I removed as impact-only.
- `CAN-L3-G-580`, `CAN-L3-G-581`, `CAN-L3-G-587`, `CAN-L3-G-607` — D/I removed as impact-only; **H
  participation remains `REVIEW-REQUIRED`** on all four (not removed without further evidence).
- `CAN-L3-G-590` — **unchanged**, Domains left as `G, E, D, I`. Marked `DOMAINS REVIEW REQUIRED`: current
  evidence does not conclusively establish whether D/I materially participate in creating the scheduling
  conflict or are only affected by it.
- The broader §4A review (further G01/G02 rows beyond the seven already named) remains open and was not
  re-swept by this targeted pass.

## Type Revalidation

Per `L3_Scenario_Type_and_Priority_Rules.md`, Type was **not** mechanically changed for the Domains-only
corrections (`G-559, G-568, G-580, G-581, G-587, G-607`) — removing an impact-only participant does not by
itself change whether genuine multi-domain collaboration exists, and none of these rows' remaining
collaboration pattern changed. For the four rewritten MERGE parents that absorbed a Cross-domain row's
scope (`CAN-L3-G-763`, `CAN-L3-G-770`), Type was raised from `Simple` to `Cross-domain` because the merged
outcome now itself includes the absorbed row's genuine E-domain (Housekeeping workforce) participation step
— this is a direct consequence of the merge, not a mechanical recount. All other surviving parents retained
their pre-existing Type.

## Section G Count After §13

**Phase 1 active Section G candidates: 322** (350 − 28 absorbed; `CAN-L3-G-543` retained pending `I-XI-03`).
Active ID range is now `CAN-L3-G-542` … `CAN-L3-G-863`, continuous and gap-free — see §14 for the
complete renumbering crosswalk and rationale.

---

## 14. ID Renumbering — 2026-08-17

**Authorization:** explicit user request to apply consistent, continuous numbering to the active Section G
candidate set, with explicit acknowledgment and override of the "Phase 1 Candidate IDs must never be
renumbered" rule enforced in §12/§13 above and in `L3_Naming_Numbering_Standard.md`.

**What changed.** The 322 candidates active after §13 (range `CAN-L3-G-001`…`CAN-L3-G-541`, sparse) were
renumbered to a single continuous block **`CAN-L3-G-542`…`CAN-L3-G-863`**, in the same subdomain order
(G00→G10) and relative sequence they already held. Applied to the Phase 1 inventory, the Phase 2 register,
this watchlist (§13 and this section only — see below), the repository-wide review, and the small number of
live cross-references in the Section A, B/D/E and H watchlists that named a still-active Section G
candidate.

**Why the range starts at 542, not 001.** A plain 1–322 renumbering would have reused 94 numbers already
permanently assigned to now-retired candidates (e.g. former `CAN-L3-G-047`, absorbed under `SG-02`, would
collide with whatever candidate a 1–322 scheme placed at position 47). Retired IDs are never reused — that
rule is the entire reason this watchlist's crosswalk tables above remain resolvable. Starting at 542 (one
past the historical maximum `CAN-L3-G-541`) makes the new range provably disjoint from every ID ever
allocated in Section G, so no historical reference — anywhere, in any document — can ever become ambiguous.

**Why §1–§11's historical text still uses the old numbers.** Sections §1–§11 document decisions made between
2026-08-11 and 2026-08-15, under the ID scheme that existed at the time. Retroactively rewriting them to use
today's new numbers was attempted mechanically and produced corrupted output — several clusters describe a
group of IDs as a range (e.g. `G-234…G-267`) where only some members are still active; renumbering only the
active endpoint breaks the range. That attempt was reverted. §1–§11 are therefore intentionally left exactly
as originally written, using the **pre-2026-08-17 numbering**; this section is the single authoritative
crosswalk between old and new numbers for anyone reading them. §12 and §13 above use the **new** numbering
throughout, since they describe the current, post-renumbering state.

**Not touched, for the same historical-integrity reason:** `CHANGELOG.md`, and the dated pre-session audit
reports (`generated/reviews/l3-phase1-section-g-corrections-2026-08-11.md`,
`L3_Phase1_Audit_*_ALL_ASSIGNED.md`, `l3-phase1-audit-all-2026-08-14-2.md`,
`L3_Phase1_Tranche3_Application_Report_2026-08-15.md`,
`L3_Phase1_Repository_Remediation_and_Phase2_Readiness.md`), and this session's own already-published
point-in-time reports (`L3_Section_G_350_Candidate_Independent_Boundary_Audit.md`,
`L3_Section_G_Phase2_Targeted_Consolidation_Application.md`,
`L3_Section_G_Phase1_and_Phase2_Targeted_Remediation_Application.md`). Any old ID encountered in those files
resolves through the crosswalk below.

**`CAN-L3-G-543`** (formerly `CAN-L3-G-002`) was renumbered along with every other active candidate — its
`BLOCKED`/`I-XI-03` status is unaffected by the renumbering and remains open (see §13).

### Full Crosswalk (old → new), by subdomain

Computed directly from the renumbered inventory (each new block is itself fully contiguous, confirming the
renumbering preserved subdomain grouping exactly):

| Subdomain | New range (continuous) | Count |
| --- | --- | --- |
| G00 — Campus Operations Governance | `CAN-L3-G-542`…`CAN-L3-G-557` | 16 |
| G01 — Facilities Management | `CAN-L3-G-558`…`CAN-L3-G-591` | 34 |
| G02 — Laboratory Management | `CAN-L3-G-592`…`CAN-L3-G-619` | 28 |
| G03 — Hostel Management | `CAN-L3-G-620`…`CAN-L3-G-634` | 15 |
| G04 — Transport Management | `CAN-L3-G-635`…`CAN-L3-G-665` | 31 |
| G05 — Library Management | `CAN-L3-G-666`…`CAN-L3-G-696` | 31 |
| G06 — Institutional Health Services & Medical | `CAN-L3-G-697`…`CAN-L3-G-728` | 32 |
| G07 — Security Operations | `CAN-L3-G-729`…`CAN-L3-G-758` | 30 |
| G08 — Housekeeping Operations & Service Governance | `CAN-L3-G-759`…`CAN-L3-G-794` | 36 |
| G09 — Teacher Guardian Scheme | `CAN-L3-G-795`…`CAN-L3-G-820` | 26 |
| G10 — Infrastructure Management | `CAN-L3-G-821`…`CAN-L3-G-863` | 43 |
| **Total** | `CAN-L3-G-542`…`CAN-L3-G-863` | **322** |

Old (pre-2026-08-17) IDs were sparse throughout `CAN-L3-G-001`…`CAN-L3-G-541` and are not repeated here —
§1–§11 above remain the authoritative record of which old ID belonged to which subdomain.

Exact per-candidate old→new pairs are recorded, table by table, in the Phase 2 register's Section G
"Phase 1 ID" column (unchanged Phase 2 Working IDs, `P2W-G-0173`…`P2W-G-0522`, now paired with the new
`CAN-L3-G-542`…`863` Phase 1 IDs) — that register is the authoritative full crosswalk; this section
summarizes it by subdomain rather than repeating all 322 rows.

**Phase 2 Working IDs (`P2W-G-####`) were deliberately NOT renumbered.** They follow a mechanical
extraction order spanning all ten sections (A–J) of the repository-wide register, not a per-section
sequence; renumbering them would require rebuilding that entire cross-repository register, which is outside
Section G's scope. If that is wanted, it needs a separate, repository-wide authorization.
