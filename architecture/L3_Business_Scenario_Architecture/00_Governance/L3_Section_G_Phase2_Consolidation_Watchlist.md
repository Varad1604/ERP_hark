# L3 Section G — Phase 2 Consolidation Watchlist

**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section G — Campus Operations, 391 current sparse candidates in range `CAN-L3-G-001` … `CAN-L3-G-541`; 541 former candidates
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
- **Candidate IDs.** Unique across all 391 current rows; sparse former IDs preserve the consolidation crosswalk.
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

No identified Section G audit decision remains open. Type and Priority remain subject to the repository's ordinary Phase 2 approval gate, but no known classification defect or unresolved ownership choice remains in this watchlist.
