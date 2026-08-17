# L3 Phase 2 — Wave 1 Upstream Blocker Resolution

# Executive Summary

Five Wave 1 blocker groups were reviewed under the repository authority hierarchy. Deterministic repository evidence closed `I-RC-07`, `FIND-D-01`, and `BDE-B-01`; the Lead Architect subsequently resolved `CF-06` and approved C-C02 Option B. No blocker was closed merely to reduce the count. `FIND-D-02` still requires a Lead Architect decision because incompatible locked L2 aggregate models remain. Phase 2 BLOCKED rows decreased from 45 to 38, while the frozen Phase 1 baseline remains 1,096 candidates.

# Before State

BLOCKED rows before = 45

The five target groups initially affected nine directly blocked Phase 2 rows: two B rows, two C rows, two D rows, and three I rows. `CAN-L3-C-012` is also architecturally impacted by `C-C02`, but its register row is blocked under the separate `C-C01` issue and was not reclassified in this task.

# Resolution Matrix

| Blocker | Classification | Rows Affected | Result | Decision Needed |
|---|---|---:|---|---|
| I-RC-07 | DETERMINISTIC_FIX | 3 | Approved I07 boundary made internally consistent; three rows changed `BLOCKED` → `KEEP` | No |
| CF-06 | DETERMINISTIC_FIX (approved decision applied) | 1 | C01 retains revocation ownership; C02 archives the existing Student Context under institutional policy through `WF-C02-10` / `ArchiveStudent()`; row changed `BLOCKED` → `KEEP` | No |
| C-C02 | DETERMINISTIC_FIX (approved decision applied) | 1 directly blocked; 2 architecturally impacted | Option B applied: C04 owns Programme-changing mobility; C02 transfer excludes Programme; one row changed `BLOCKED` → `KEEP` | No |
| FIND-D-01 / FIND-D-02 | LEAD_ARCHITECT_DECISION_REQUIRED | 2 | FIND-D-01 corrected deterministically; FIND-D-02 remains and both rows stay `BLOCKED` | Yes — FIND-D-02 only |
| BDE-B-01 | DETERMINISTIC_FIX | 2 | B02 row changed to `KEEP`; B03 row changed to `SPLIT-REQUIRED` | No |

# Deterministic Fixes Applied

`I-RC-07`: The approved 2026-08-15 question-paper boundary already established that L2-I07 owns system intake, system custody, timed release, and distribution, including direct Examination Controller authoring or document upload. Stale disclaimers in L2-I07 §9.5, §23, and Appendix A were corrected. A separate Question Paper Creation/paper-setting workflow, moderation boards, printing, and physical custody chains remain outside SutraERP; no broader question-paper capability was assigned to I07.

`FIND-D-01`: Locked L1 assigns Timetable to Academics and physical infrastructure to Infrastructure. L2-G10 confirms that it provides classrooms, laboratories, capacity, accessibility, and operational status while Timetable determines allocation, scheduling, conflict detection, and optimization. Stale physical-infrastructure ownership references in L2-D00 and L2-D03 were corrected from B03 to G10; D03 continues to own timetable and scheduling decisions and consumes G10-owned facts.

`BDE-B-01`: The result was determined per aggregate. L2-B02 WF08 has one Decommission Request, one Business Approval, one workflow, and no independent archive trigger or archive-completion workflow; Decommissioned is an intermediate state before Archived, so `CAN-L3-B-012` is `KEEP`. L2-B03 §111 completes at the stable `Decommissioned` lifecycle state and publishes `CampusDecommissioned`; §112 can occur later from its own Archive Request and completes at `Archived` with `CampusArchived`. B03 does not document a separate archive approval, but its separate trigger, workflow boundary, terminal outcome, and event establish an independently meaningful scenario, so `CAN-L3-B-021` is `SPLIT-REQUIRED`. Neither frozen Phase 1 row was changed.

`CF-06`: The Lead Architect decision supplied on 2026-08-17 selects archival under institutional policy. `DEPENDENCY_MAP.md`, L2-C01, and L2-C02 now state the same safe boundary: C01 owns and publishes the Admission revocation decision; after Student Context creation, C02 consumes `AdmissionRevoked` and routes its own Student Context through existing `WF-C02-10` / `ArchiveStudent()`. The existing `Archived` state and `StudentArchived` event are reused, all history is preserved, and physical deletion is prohibited. No new owner, aggregate, lifecycle state, or event was introduced; `CAN-L3-C-004` is `KEEP`.

`C-C02`: The Lead Architect approved recommended Option B on 2026-08-17. L2-C04 APW-08 now exclusively owns all Programme-changing mobility, including eligibility, curriculum mapping, credit mapping, approval, and `ProgrammeMigrationApproved`. L2-C02 WF-C02-08 excludes Programme and remains limited to non-Programme organisational or placement transfers. C02 retains Student Context ownership and consumes the approved C04 result only to update its Programme association; it does not make or publish a competing migration decision. `CAN-L3-C-033` is `KEEP`; `CAN-L3-C-012` remains `BLOCKED` only under C-C01.

# Lead Architect Decision Briefs

## FIND-D-02

Classification:
LEAD_ARCHITECT_DECISION_REQUIRED

Affected candidates:
`CAN-L3-D-020`, `CAN-L3-D-021`

Question:
Which locked Section D aggregate model is canonical for Timetable ownership?

Established repository facts:
- Locked L1 §17 and §24 assign Timetable to Academics; FIND-D-01 separately confirms G10 ownership of physical infrastructure facts.
- `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D00_Academic_Delivery_Review.md` Part 5 and Appendix A freeze `TimetableStructure`, `TimetableSlotAssignment`, and `SchedulingConflictRegister` as D03 aggregate roots.
- `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D03_Timetable_Workflows.md` §6.4 and §17 instead name `Timetable`, `Timetable Version`, and a different collection of owned objects; §17.5 classifies several of those as child or reference entities rather than aggregate roots.
- No higher-authority source selects either same-tier model.

Conflict:
The canonical Timetable owner is settled, but the locked D00 and D03 aggregate identities and consistency boundaries are incompatible.

Option A:
Retain the D00 three-aggregate-root model and amend D03 terminology, ownership lists, events, and references to map consistently to it.
Advantages:
- Preserves D00's explicit frozen aggregate registry and certification.
- Keeps its command and event mappings intact.
Risks:
- Requires broad reconciliation of the more detailed D03 data model.
- May collapse distinctions currently expressed throughout D03.

Option B:
Adopt the dedicated D03 aggregate model and amend D00's frozen registry, command mappings, event mappings, and certification to match it.
Advantages:
- Uses the most specific workflow document as the aggregate source.
- Aligns the section review with D03's detailed lifecycle and data-model sections.
Risks:
- Changes a locked review that explicitly declares its aggregates frozen.
- D03's own repeated ownership lists must first be made internally consistent.

Recommendation:
No recommendation — evidence balanced

Reason:
Both positions are locked L2 sources and each declares canonical or frozen status. Higher authority settles domain ownership but not aggregate identity. Selecting either model would change aggregate boundaries and downstream mappings. That choice requires Lead Architect approval.

Required repository changes if approved:
- `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D00_Academic_Delivery_Review.md`
- `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D03_Timetable_Workflows.md`
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md`
- `architecture/L3_Business_Scenario_Architecture/02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md`
- `generated/reviews/L3_Phase2_Repository_Wide_Consolidation_Review.md`

## FIND-D-02 Post-Decision Application — 2026-08-17

The Lead Architect subsequently approved Option B. `L2-D03` section 17 is controlling: `Timetable` is the sole Aggregate Root, and `Timetable Version` and all schedule, validation, conflict, publication, revision, reservation, and audit records are governed beneath it. `L2-D03` ownership summaries and the former `L2-D00` three-root registry, command mappings, event mappings, and certification were reconciled to that hierarchy without introducing a new capability, state, event, owner, or external contract.

`CAN-L3-D-020` and `CAN-L3-D-021` are both `KEEP`. The former governs initial planning through activation; the latter governs an authorized post-activation revision that creates a new immutable version, republishes it, and preserves the prior version. `FIND-D-02` is closed.

# Files Modified

- `DEPENDENCY_MAP.md`
- `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C01_Admissions_Workflows.md`
- `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C02_Student_Lifecycle_and_Activation.md`
- `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C04_Student_Academic_Progression_Workflows.md`
- `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D00_Academic_Delivery_Review.md`
- `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D03_Timetable_Workflows.md`
- `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I07_Theory_Examination_Conduct_Workflows.md`
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md`
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_C_Phase2_Consolidation_Watchlist.md`
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_I_Phase2_Consolidation_Watchlist.md`
- `architecture/L3_Business_Scenario_Architecture/02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md`
- `generated/reviews/L3_Phase2_Repository_Wide_Consolidation_Review.md`
- `generated/reviews/L3_Phase2_Wave1_Upstream_Blocker_Resolution_2026-08-17.md`

# Phase 2 Disposition Changes

| P2W ID | Phase 1 candidate | Before | After | Basis |
|---|---|---|---|---|
| `P2W-B-0700` | `CAN-L3-B-012` | BLOCKED | KEEP | B02 WF08 is one complete Decommission-and-Archive workflow |
| `P2W-B-0709` | `CAN-L3-B-021` | BLOCKED | SPLIT-REQUIRED | B03 §111 and §112 are separately triggered completed workflows |
| `P2W-C-1053` | `CAN-L3-C-004` | BLOCKED | KEEP | Approved CF-06 ruling routes post-creation `AdmissionRevoked` to existing C02 archival under institutional policy |
| `P2W-C-1082` | `CAN-L3-C-033` | BLOCKED | KEEP | Approved C-C02 Option B assigns Programme-changing mobility to C04 APW-08 and limits C02 WF-C02-08 to non-Programme transfer |
| `P2W-I-0915` | `CAN-L3-I-063` | BLOCKED | KEEP | Approved I07 question-paper boundary made internally consistent |
| `P2W-I-0916` | `CAN-L3-I-064` | BLOCKED | KEEP | Approved I07 question-paper boundary made internally consistent |
| `P2W-I-0917` | `CAN-L3-I-065` | BLOCKED | KEEP | Approved I07 question-paper boundary made internally consistent |
| `P2W-D-0740` | `CAN-L3-D-020` | BLOCKED (`FIND-D-01/FIND-D-02`) | BLOCKED (`FIND-D-02`) | Physical ownership settled; aggregate conflict remains |
| `P2W-D-0741` | `CAN-L3-D-021` | BLOCKED (`FIND-D-01/FIND-D-02`) | BLOCKED (`FIND-D-02`) | Physical ownership settled; aggregate conflict remains |

# Remaining Blocked Rows at Wave 1 Close

Current Phase 2 BLOCKED count = 38.

| Issue | Rows |
|---|---:|
| C-C01 | 2 |
| FIND-D-02 | 2 |
| FIND-D-06 | 5 |
| FIND-D-07 | 8 |
| FIND-D-08 | 1 |
| I-TV-04-residual | 5 |
| I-XI-02 | 3 |
| I-XI-03 | 5 |
| J-TV-01 | 7 |
| **Total** | **38** |

# Validation

- Phase 1 candidate total: 1,096.
- `python check_l3_inventory.py`: PASS — 0 errors, 0 warnings.
- Phase 1 files modified: 0; no Phase 1 row deleted and no Phase 1 ID changed.
- Register rows: 1,124; active frozen Phase 1 candidates represented: 1,096.
- P2W IDs: 1,124 unique; changed: 0.
- Recomputed dispositions: KEEP 1,056; MERGE 20; VARIANT 6; DUPLICATE 2; SPLIT-REQUIRED 1; REVIEW-REQUIRED 1; BLOCKED 38.
- No new canonical owner, aggregate, lifecycle state, event, API, or database concept was introduced. The approved CF-06 rule reuses C02's existing archive workflow, state, and event.
- Every L2 correction traces to the approved 2026-08-15 I07 scope decision, locked L1/G10 physical-infrastructure ownership, or the approved 2026-08-17 CF-06 and C-C02 Lead Architect decisions recorded in `DEPENDENCY_MAP.md`.
- The unresolved locked D00/D03 aggregate conflict remains unresolved.

# Repository Consistency Verdict at Wave 1 Close

NEEDS DECISION

The deterministic and approved-decision scope was internally propagated and validation passed. At Wave 1 close, repository consistency still depended on a Lead Architect decision for `FIND-D-02`; its affected rows remained blocked.

# Current Status After FIND-D-02 Decision

The subsequently approved Option B ruling described above closes `FIND-D-02`. `CAN-L3-D-020` and `CAN-L3-D-021` are `KEEP`; the current Phase 2 register has zero `BLOCKED` rows, and the current repository consistency verdict is **PASS**.
