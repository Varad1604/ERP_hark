# L3 Sections B, D and E — Phase 2 Consolidation Watchlist

**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section B — Institution & Structure (32 candidates, `CAN-L3-B-001`…`B-032`); Section D — Academic Delivery (57 candidates, `CAN-L3-D-001`…`D-057`); Section E — People & Workforce (75 candidates, `CAN-L3-E-001`…`E-075`) — all three renumbered 2026-08-17, see `L3_Phase1_Candidate_ID_Migration_2026-08-17.md`
**Status:** Active — Phase 2 input
**Created:** 2026-08-14
**Source:** all-scope adversarial Phase 1 audit, 2026-08-14 — `generated/reviews/l3-phase1-audit-all-2026-08-14.md`
**Inventory:** `../01_Phase1_Discovery/L3_Phase1_Inventory_Pallav.md`

---

## 1. Purpose

This document holds the boundary questions, preserved repository findings and deferred decisions affecting Sections B, D and E.

It exists as a separate document because `L3_Phase1_Discovery_Rules.md` section 3 requires each Phase 1 inventory file to contain candidate scenario rows only, and section 16 requires the inventory to avoid long explanations. Until 2026-08-14 Sections B, D and E had no watchlist, so this material accumulated inside the inventory itself — where two notes went stale against the rows beside them and had to be corrected (audit finding FIND-010).

This document is not a governance standard. It adds and amends no Phase 1 rule and resolves no architectural conflict.

It follows the structure established by `L3_Section_A_Phase2_Consolidation_Watchlist.md` and `L3_Section_C_Phase2_Consolidation_Watchlist.md`.

---

## 2. Scope Statement

Sections B, D and E are the three discovery areas assigned to `L3_Phase1_Inventory_Pallav.md` under `L3_Phase1_Workload_Distribution_Decision.md` section 2. The assignment is complete and correctly scoped.

All L2 workflow documents have candidate coverage: `L2-B01`…`L2-B05`, `L2-D01`…`L2-D07`, `L2-E01`…`L2-E07`. `L2-B00`, `L2-D00` and `L2-E00` are review documents that define no workflows and correctly carry no candidates.

The pre-consolidation 1,080-row submission is preserved at `generated/reviews/L3_Phase1_Inventory_Pallav_PreConsolidation_2026-08-13.md`. The 954-row expanded variant catalogue was deleted on 2026-08-14; its disposition is recorded in the inventory's change note.

---

## 3. Status Values

| Status | Meaning | Phase 2 action |
| --- | --- | --- |
| `CLOSED` | Repository evidence establishes the candidates are distinct or the question is settled. | None. Do not re-raise. |
| `DETERMINED` | Repository evidence is one-sided. The decision is recorded; only its execution is deferred. | Execute the recorded decision. |
| `OPEN` | A genuine Phase 2 judgment is required. | Decide, then execute. |
| `BLOCKED` | Cannot be decided from current repository evidence. | Resolve the blocker first. |

---

## 4. Repository Conflicts Affecting B/D/E Discovery

### RC-01 — Delegation ownership (mirrored from `L3_Section_A_Phase2_Consolidation_Watchlist.md` §9)

| | |
| --- | --- |
| **Status** | `RESOLVED 2026-08-15` |
| **Affected candidates** | `CAN-L3-E-015` (Delegated Authority and Additional Charge Assignment, owner E02); `CAN-L3-A-011` (Responsibility Delegation to Automatic Authority Reversion, owner Authorization & Policy); `CAN-L3-G-252` (Housekeeping Delegation to Temporary Operational Authority) |
| **Position A — `L2-A04`** | Line 114: "Canonical boundary approved 2026-08-13 under INT-301. L2-A04 **exclusively owns** Role, Role Assignment, **Delegation** and authorization-policy lifecycles." §10.5 defines Delegation as "temporary transfer of business responsibility from one authorized user to another." |
| **Position B — `L2-E02`** | Line 359: `Assignment Delegation \| Canonical`. Line 975: `Delegation \| L2-E02 \| Operational delegation`. Line 2644: command `DelegateAuthority`. Line 2662: event `PositionAssignmentDelegated`. |
| **Authority analysis** | Unresolved by hierarchy — both are locked L2 documents at the same tier. A reading exists in which A04 owns authorization-layer delegation and E02 owns organisational-assignment delegation, but **no repository source states that split** and A04's exclusivity wording forecloses it. |
| **Phase 2 blocked?** | No — cleared 2026-08-15. |
| **Escalate to** | The authority that approved INT-301. |

Neither inventory recorded this conflict before 2026-08-14.

**Resolution, approved 2026-08-15.** `L2-A04` and `L2-A03` now scope their exclusivity to
`Authorization Delegation`; `L2-E02` line 975 now reads `Assignment Delegation`, matching its own
canonical entity table at line 359. Both documents carry reciprocal scope clarifications.
Assignment Delegation consumes A04 authorization primitives without owning them. `E-119` stays
with Position & Assignment. No entity moved between documents.

Full record: `generated/reviews/L3_Phase1_Tranche3_Application_Report_2026-08-15.md`; canonical
entry in `L3_Section_A_Phase2_Consolidation_Watchlist.md` section 9.

### Preserved Section D findings

Carried forward from the inventory, where they were recorded inline. Current status is recorded per finding; unresolved same-tier conflicts remain unreconciled.

| # | Finding | Status |
| --- | --- | --- |
| `FIND-D-01` | **Resolved 2026-08-17.** Locked L1 §17/§24 assigns Timetable to Academics and physical infrastructure to Infrastructure; `DEPENDENCY_MAP.md` §6 says Section D consumes campus and infrastructure context; `L2-G10` §9A.248 says G10 provides physical resource facts while Timetable determines allocation and scheduling. Stale B03 physical-infrastructure references in D00/D03 were corrected to G10. | `CLOSED` |
| `FIND-D-02` | **Resolved by explicit Lead Architect approval of Option B on 2026-08-17.** `L2-D03` §17 controls: `Timetable` is the sole Aggregate Root and `Timetable Version` and related scheduling records are governed beneath it. The former `L2-D00` three-root registry and inconsistent D03 ownership summaries were reconciled; see §9. | `CLOSED` |
| `FIND-D-03` | D00 versus detailed D03/D04 terminology remains a mapping issue. | `OPEN` |
| `FIND-D-04` | D00 command matrix coverage stops at D06; D07 commands were not invented into D00. | `OPEN` |
| `FIND-D-05` | D07 Actors and Locked Decisions gaps remain; actor-specific roles were not invented. | `OPEN` |
| `FIND-D-06` | D05 Governance Review and Completion Certification remain supporting concepts unless stronger source evidence establishes independent lifecycles. | `OPEN` |
| `FIND-D-07` | Flexible Learning Enrollment versus Elective Enrollment relationship unresolved; preserved as a boundary variant. | `OPEN` |
| `FIND-D-08` | Certificate Verification remains inside D06 recognition-related expansion unless later source review proves an independent lifecycle. | `OPEN` |
| `FIND-D-09` | D01 offering-side enrollment governance remains distinct from Section C Student Enrollment ownership. | `CLOSED` |

---

## 5. Boundary Questions

| # | Question | Candidates | Position | Status |
| --- | --- | --- | --- | --- |
| `BDE-B-01` | Do decommission and archive form one outcome or two? | `CAN-L3-B-012`, `CAN-L3-B-021` | **Resolved per aggregate 2026-08-17.** B02 WF08 is one workflow from Decommission Request through Archived, so `CAN-L3-B-012` is `KEEP`. B03 §111 ends with `CampusDecommissioned`; §112 begins later from an `Archive Request` and ends with `CampusArchived`, so `CAN-L3-B-021` is `SPLIT-REQUIRED`. No symmetry was imposed and both frozen Phase 1 rows remain unchanged. | `CLOSED` |
| `BDE-D-01` | Is `CAN-L3-D-024`'s boundary settled at Attendance Finalization? | `CAN-L3-D-024` | **Settled.** Extended on 2026-08-14 to match `L2-D04` §23 and the canonical example in `L3_Scenario_Type_and_Priority_Rules.md` §9. A stale inventory note claiming this was still open was corrected on 2026-08-14. | `CLOSED` |
| `BDE-E-01` | Sections D and E carry no examination-facing counterpart candidates. Section I lists `E` as a participant in ~15 candidates (invigilator, examiner, evaluator assignment) and `D` in ~30, but the B/D/E inventory returns **zero** matches for "examination", "invigilat" or "examiner". | none | **Not a B/D/E coverage defect.** `L2-E03` defines no invigilator or examiner duty capability, so there is no positive E-side source evidence for a missing outcome. `L2-I03` line 458 names Faculty Administration as coordinating invigilator resource planning — an actor dependency. Section I already owns `I-022`, `I-023`, `I-024`. Mirrors `I-XI-05` in the Section I watchlist. | `OPEN` |

---

## 6. Ownership Checks That Passed

Recorded so they are not re-derived.

* **Position Definition → B02, Position Assignment → E02.** Former `CAN-L3-E-012`…`E-014` were re-owned as `CAN-L3-B-013`…`B-1011`. Verified correct against `L2-B02` §"Approved Position Definition Ownership and Lifecycle Clarification", which states E02 "consumes a Position Definition as an external structural reference and shall never create" one, and `L2-E02` line 76 listing Position Definition as owned by the Organisation Domain. **Repository-correct, not a silent resolution.**
* **InstitutionalCampus → B03, Physical Campus → G10.** `CAN-L3-B-017`…`B-020` state that physical attributes remain owned by G10 and that any consumed G10 readiness fact stays G10-owned; `CAN-L3-G-280`, `G-281` (twice-renumbered 2026-08-17 — via disjoint IDs `G-821`, `G-822` — from pre-2026-08-17 `G-450`, `G-452`; sibling candidates `G-451`, `G-453` were retired in an earlier consolidation wave) state the reciprocal. No duplicate canonical ownership.
* **Financial transactions → Section F.** `CAN-L3-E-051` records leave-encashment eligibility "without performing payroll calculation or financial settlement"; `CAN-L3-E-056` produces a settlement result *for* Finance.
* **Examination results → Section I.** `CAN-L3-D-055` and `CAN-L3-D-057` publish finalized internal assessment *for* Section I consumption without claiming result ownership.
* **Employee-profile evidence → E07, faculty academic use → E03.** `CAN-L3-E-028` establishes the association while E07 remains owner of profile evidence.

No candidate in Sections B, D or E introduced duplicate canonical ownership, and the 2026-08-14 cross-inventory scan found no duplicate outcome shared with any other section.

---

## 7. Effect on Candidate Counts

| Section | Candidates | Note |
| --- | ---: | --- |
| B | 32 | Includes `B-1009`…`B-1021`; numbering intentionally sparse until Phase 2 close. |
| D | 57 | Was described in the inventory as 38 — a stale figure referring to the pre-correction parent set, corrected 2026-08-14. |
| E | 75 | Retired IDs `E-029`, `E-030` must not be reused. |

Retained IDs were not renumbered.

---

## 8. Wave 3 Outcome Consolidation Resolution — 2026-08-17

This section is the current resolution of `FIND-D-06`, `FIND-D-07`, and `FIND-D-08`. It preserves the earlier point-in-time findings above and supersedes only their `OPEN` disposition.

### 8.1 FIND-D-06 — RESOLVED

`L2-D05` defines one Monitoring Case lifecycle, but it also establishes independently consumable business facts at outcome publication, formal governance decision, certified closure, and controlled post-terminal reopening.

| Candidate | Disposition | Target | Outcome test and authoritative evidence |
| --- | --- | --- | --- |
| `CAN-L3-D-027` | `MERGE` | `CAN-L3-D-028` | Sections 8.3, 10.3-10.8 and 11.2 place activation before evaluation and publication in the same case; `Active` is an intermediate state. |
| `CAN-L3-D-028` | `KEEP` | — | Sections 11.2 and 11.9 end the execution span in a published monitoring fact, notify consumers, and publish `AcademicMonitoringOutcomePublished`. |
| `CAN-L3-D-032` | `KEEP` | — | Sections 8.11 and 13 give the review independent authority, a Governance Decision Record, conflict resolution, emergency override control, and published decision facts. |
| `CAN-L3-D-033` | `KEEP` | — | Sections 14.2-14.7 require certification and closure authority and establish the immutable `AcademicMonitoringClosed` terminal state. |
| `CAN-L3-D-034` | `KEEP` | — | Sections 14.8-14.11 begin after closure, preserve the original lifecycle, and create a new lifecycle context and identifier. |

### 8.2 FIND-D-07 — RESOLVED

`L2-D06` section 7, its aggregate catalogue, lifecycle models and ADRs distinguish Flexible Learning Enrollment from C03 Academic Registration and decompose the long-running D06 participation journey into separate consistency boundaries. D01 remains owner of Course Offering; C remains owner of Student Academic Registration and canonical academic records.

| Candidate | Disposition | Outcome test and authoritative evidence |
| --- | --- | --- |
| `CAN-L3-D-035` | `KEEP` | Flexible Learning Enrollment has its own D06 lifecycle and approval/withdrawal outcomes; ADR-003 explicitly decouples it from institutional registration. |
| `CAN-L3-D-036` | `KEEP` | Aggregate 2 owns the authoritative allocation/reservation/release fact. |
| `CAN-L3-D-037` | `KEEP` | Aggregate 3 and ADR-011 establish an independent waitlist lifecycle and terminal dispositions. |
| `CAN-L3-D-038` | `KEEP` | Aggregate 4 owns provider registration and synchronization status independently of the local enrollment decision. |
| `CAN-L3-D-039` | `KEEP` | Aggregate 5 owns verified learning completion before any later recognition decision. |
| `CAN-L3-D-040` | `KEEP` | Aggregate 7 and ADR-007 establish a separate institutional recognition decision; D06 only requests downstream posting. |
| `CAN-L3-D-041` | `KEEP` | D06-W01-W05 reach active, discoverable availability while referencing the D01-owned Course Offering. |
| `CAN-L3-D-042` | `SPLIT-REQUIRED` | D06-W06 suspension has an administrative trigger and temporary availability consequence; D06-W07 archival has post-session eligibility and an immutable historical consequence. Proposed later canonical boundaries: Active Opportunity to Governed Suspension; Archival-Eligible Opportunity to Archived Historical Opportunity. |

### 8.3 FIND-D-08 — RESOLVED

`CAN-L3-D-043` is `KEEP`. `L2-D06` Aggregate 6 and D06-W45-W51 define an independently governed Certificate Verification lifecycle with immutable evidence and accepted, rejected, and fraud-suspected terminal outcomes. Rejection or fraud can end verification and block recognition, so verification does not depend on a completed credit-recognition decision. No certificate master-data ownership is introduced.

At Wave 3 close, `FIND-D-02` remained `BLOCKED` and unchanged for `CAN-L3-D-020` and `CAN-L3-D-021`.

---

## 9. FIND-D-02 Lead Architect Resolution — 2026-08-17

The Lead Architect explicitly approved Option B. `L2-D03` section 17 is controlling: `Timetable` is the sole D03 Aggregate Root. `Timetable Version`, Schedule Entry, Schedule Slot, Scheduling Pattern, Resource Reservation, Validation Result, Conflict Record, Publication Record, Revision Record, and Audit Record are governed beneath that root. The inconsistent ownership summaries in `L2-D03` and the former three-root registry and mappings in `L2-D00` were reconciled to this hierarchy. No new capability, lifecycle state, event, owner, or external contract was introduced.

| Candidate | Disposition | Outcome test and authoritative evidence |
| --- | --- | --- |
| `CAN-L3-D-020` | `KEEP` | The initial timetable journey completes planning, generation, validation, conflict resolution, approval, publication, and activation under the sole `Timetable` root, producing the independently consumed `TimetablePublished` and `TimetableActivated` outcomes. |
| `CAN-L3-D-021` | `KEEP` | An authorized change after activation creates and governs a new immutable Timetable Version, completes approval and republication, and preserves the prior version. `TimetableRevisionCreated` and `TimetableRepublished` distinguish this outcome from initial publication. |

`FIND-D-02` is **CLOSED**. The B/D/E watchlist has no remaining `BLOCKED` Phase 2 row.
