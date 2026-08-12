# L3 Section G — Phase 2 Consolidation Watchlist

**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section G — Campus Operations, candidates `CAN-L3-G-001` … `CAN-L3-G-541`
**Status:** Active — Phase 2 input
**Created:** 2026-08-11
**Source:** independent adversarial audit of `L3_Phase1_Inventory_Varad.md`, 2026-08-11
**Corrections applied:** `generated/reviews/l3-phase1-section-g-corrections-2026-08-11.md`

---

## 1. Purpose

This document holds the candidate relationships, boundary questions and blocked decisions identified during the independent Section G audit of 2026-08-11, and records for each one whether it is settled or still requires a Phase 2 decision.

It exists as a separate document because `L3_Phase1_Discovery_Rules.md` section 3 requires each Phase 1 inventory file to contain candidate scenario rows only, and section 16 requires the inventory to avoid long explanations.

**Nothing in this document has been consolidated.** Every candidate named below remains in the inventory under its own ID. `L3_Phase1_Discovery_Rules.md` section 17 places duplicate identification, merging and splitting in the post-Phase-1 review.

This document is not a governance standard. It adds and amends no Phase 1 rule and resolves no architectural conflict.

It follows the structure established by `L3_Section_A_Phase2_Consolidation_Watchlist.md` and `L3_Section_C_Phase2_Consolidation_Watchlist.md`.

---

## 2. Scope Warning

Section G is one of three sections assigned to `L3_Phase1_Inventory_Varad.md` under `L3_Phase1_Workload_Distribution_Decision.md` section 2.

**Sections F and H remain entirely undiscovered.** This watchlist covers Section G only. Two of its clusters cannot be actioned until Section H exists, and this is recorded against them.

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
| G-FR-01 | Cross-cutting audit history, one per sub-domain | 12 — `G-020, G-059, G-090, G-118, G-155, G-192, G-226, G-301, G-365, G-413, G-431, G-528` | Governed institutional audit evidence | Preserved | `OPEN` — merge, or defer to Section A `CAN-L3-A-023`…`A-030` |
| G-FR-02 | Cross-cutting configuration change | 17 — `G-018, G-060, G-089, G-117, G-154, G-191, G-227, G-295, G-296, G-297, G-298, G-358, G-359, G-360, G-361, G-449, G-518` | Governed effective configuration | Preserved | `OPEN` — merge, or defer to `CAN-L3-A-062` |
| G-FR-03 | Authorization failure | 7 — `G-300, G-363, G-419, G-420, G-491, G-526, G-527` | Controlled denial of an unauthorised operation | Preserved | `OPEN` — merge, or defer to Section A |
| G-FR-04 | G07 single-step movement | ~24 — `G-234…G-237, G-244…G-250, G-252…G-263, G-267` | Controlled campus access lifecycle; governed material movement; completed patrol | Preserved | `OPEN` — merge to ~6–8 lifecycle scenarios, denial and override retained as alternate outcomes |
| G-FR-05 | G10 "invalid X to controlled rejection" | 14 — `G-451, G-455, G-457, G-459, G-462, G-464, G-468, G-472, G-479, G-480, G-484, G-489, G-512, G-519` | Validation-failure branch of the preceding candidate | Preserved | `DETERMINED` — merge each into its parent as an alternate outcome. Precedent: `L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md` section on administrative session revocation |
| G-FR-06 | G09 audited correction | 4 — `G-415, G-416, G-417, G-418` | Governed correction of a mentorship record | Preserved | `DETERMINED` — merge to one, per `L2-G09` section 26 |
| G-FR-07 | G08 re-clean | 4 — `G-316, G-326, G-332, G-337` | Verified re-clean after quality or hygiene failure | Preserved | `DETERMINED` — `G-316` and `G-337` are semantic duplicates; merge. Retain `G-332` only if the inspection origin proves materially distinct |
| G-FR-08 | Notification and awareness | 39 — incl. `G-115, G-151, G-152, G-184…G-187, G-205, G-221…G-223, G-280, G-289…G-294, G-353…G-356, G-425…G-429` | Publisher-side event emission; delivery belongs to `L2-H02` | **Potentially distorted** | `BLOCKED` — do not merge until Section H is discovered. These are currently the only L3 representation of notification behaviour in the repository |
| G-FR-09 | Reporting and dashboards | 43 — incl. `G-193, G-228, G-303…G-306, G-351, G-352, G-366…G-369, G-432…G-437, G-448, G-530…G-537` | Read-model surfaces, not institutional outcomes | **Potentially distorted** | `BLOCKED` — do not merge until Section H is discovered. See also section 7 |

---

## 5. Over-Broad Candidates

| ID | Candidate | Hidden outcomes | Status |
| --- | --- | --- | --- |
| G-SP-01 | `G-001` Campus Readiness Assessment to Readiness Decision | Reads as both the daily readiness type and the generic assessment engine, while `G-002`…`G-006` and `G-540` each carve out one named type | `OPEN` — restrict to Daily Campus Readiness per `L2-G00` section 16 |
| G-SP-02 | `G-017` Governance Exception to Resolution | Six unrelated trigger conditions with different owners and consequences | `OPEN` — split authorization failure out; fold overdue-action escalation into `G-016` |
| G-SP-03 | `G-088, G-116, G-153, G-188, G-224, G-364, G-447, G-539` — the parallel "Operational Exception to Recovery" family | Each enumerates 6–9 heterogeneous exception classes | `OPEN` — split authorization-failure outcomes out; fold validity rejections into parent lifecycle candidates |
| G-SP-04 | `G-511` Infrastructure Lifecycle Transition to Valid State Progression | Absorbs `Temporarily Closed` and `Under Maintenance`, which have distinct triggers and consumer consequences per `L2-G10` section 11 | `OPEN` — split both states out; retain `G-511` for routine progression |
| G-SP-05 | `G-445` Mentorship Case State Transition to Valid Lifecycle Progression | Generic state-machine restatement; specific transitions already covered by `G-370, G-409, G-411, G-413` | `OPEN` — consider removal rather than split |
| G-SP-06 | `G-379` Student Context Retrieval to Mentoring Readiness | Spans six source domains; partly redundant with `G-440`…`G-444` | `OPEN` — likely retain; review redundancy |

---

## 6. Overlap Clusters

| ID | Candidates | Assessment | Status |
| --- | --- | --- | --- |
| G-OV-01 | `G-316`, `G-337` | Same trigger, journey, outcome, owner and domains | `DETERMINED` — merge |
| G-OV-02 | `G-319`, `G-325` | Both end in emergency cleaning; differ only in origin | `OPEN` — merge or differentiate |
| G-OV-03 | `G-105`, `G-106`, `G-107` | `G-106` is the general complaint scenario; the other two are its routed branches | `OPEN` — likely one scenario with alternate outcomes |
| G-OV-04 | `G-048`, `G-049` | SLA satisfied and SLA breached are two outcomes of one monitoring lifecycle | `OPEN` |
| G-OV-05 | `G-092`, `G-094`, `G-095` | One continuous journey from allocation to active residency; `G-094` and `G-095` have near-identical outcomes | `OPEN` |
| G-OV-06 | `G-125`, `G-126` | Consecutive states of one journey | `OPEN` |
| G-OV-07 | `G-390`, `G-391`…`G-397`, `G-399`, `G-406` | Ten candidates for one referral mechanism; destination is a dimension, not a scenario | `OPEN` |
| G-OV-08 | `G-475`, `G-476`, `G-477` vs `G-511` | Same state changes represented at two altitudes | `OPEN` — see G-SP-04 |
| G-OV-09 | `G-077, G-105, G-137, G-148, G-339, G-357, G-499` | Seven instances of the maintenance-recommendation hand-off to G01. Each is legitimately owned by its originating domain and correctly disclaims G01's execution ownership | `CLOSED` — not an architectural error. Phase 2 may decide whether the pattern collapses |
| G-OV-10 | `G-063`, `G-083` vs `G-476` | `G-476` is the canonical G10 publication; the others are G02's consumption responses | `CLOSED` — correctly separated by ownership |

---

## 7. Cross-Inventory Relationships

| ID | Section G | Other inventory | Assessment | Status |
| --- | --- | --- | --- | --- |
| G-XI-01 | `G-039` Vendor Performance Review to Operational Evaluation | `CAN-L3-J-028` Supplier Performance or Incident Evidence to Governed Supplier Assessment Record | Both legitimate. `L2-G01` section 12 grants G01 operational vendor performance; `L2-J02` owns the canonical supplier assessment record. `G-039` correctly stops short of J's record. **No ownership contradiction.** | `OPEN` — model as G01 evidence feeding J02 assessment |
| G-XI-02 | `G-038`, `G-350` | `CAN-L3-J-029` Supplier Risk Finding to Governed Restriction | Operational corrective action vs canonical supplier restriction. No contradiction | `OPEN` |
| G-XI-03 | `G-006` Accreditation Readiness Assessment | `CAN-L3-A-047` `[provisional — INT-302]` | `G-006` is supported independently by `L2-G00` section 16 and is unaffected by INT-302. It carries no conflict marker | `BLOCKED` — do not consolidate until INT-302 is resolved by the Enterprise Architecture Board |

---

## 8. Open Items

| ID | Item | Status |
| --- | --- | --- |
| G-OI-01 | Ownership of approximately 20 reporting and dashboard candidates (`G-303`…`G-306`, `G-366`…`G-368`, `G-530`…`G-537`). `L2-G07` section 27, `L2-G08` section 26 and the `L2-G10` reporting sections each explicitly grant the domain operational reports and executive dashboards. `L2-H07` section 1.1 names the Enterprise Dashboard domain the authoritative owner of dashboard composition, orchestration and governance. | `BLOCKED` — Section H is undiscovered, so the counterparty cannot be checked |
| G-OI-02 | Priority distribution: 275 of 541 candidates at `P0`, only 9 at `P2`. `L3_Phase1_Discovery_Rules.md` section 13 states priority must reflect implementation readiness, institutional risk and architectural coverage. | `OPEN` — awaiting an institutional P0 definition. Priority remains provisional |
| G-OI-03 | 235 candidates typed `Critical`. Section 12 reserves `Critical` for high institutional risk; several single-domain administrative steps carry it. | `OPEN` — awaiting a risk threshold. Type remains provisional |
| G-OI-04 | Wing and Block registration. `L2-G10` section 12 lists it as a registration workflow, but section 8B establishes a Wing as an optional child entity of the Building Aggregate and explicitly not an independent Aggregate Root. | `CLOSED` — folded into `CAN-L3-G-458` Building Registration. Confirmed 2026-08-11. Do not re-raise |
| G-OI-05 | `CAN-L3-G-540` and `CAN-L3-G-541` carry IDs deliberately non-contiguous with their subsection blocks. Renumbering 519 rows to insert two was rejected as higher risk. Phase 1 IDs are temporary under section 6. | `CLOSED` — informational |

---

## 9. Items Confirmed Sound

Recorded so Phase 2 does not re-audit them.

- **Canonical ownership.** No violation was found across 541 candidates. Ownership disclaimers were verified directly against the `Out of Scope` sections of `L2-G01`, `L2-G03`, `L2-G05`, `L2-G09` and `L2-G10`.
- **Lifecycle states.** Every state term used in the inventory was verified present in the governing L2 document. No invented state.
- **Business events.** Candidates refer to events generically. No event name was fabricated.
- **Implementation leakage.** None. No API, schema, class, screen, broker, queue or deployment reference appears in any candidate.
- **Candidate IDs.** Unique across all 541 rows.
- **L2 document coverage.** All eleven Section G documents are represented.
