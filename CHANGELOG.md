# SutraERP Architecture Changelog

## [Unreleased] - 2026-09-21 — L6 Stage-2 full-corpus workflow realization PASS

- Completed semantic reconciliation of all **1,084 / 1,084** canonical L3 scenarios across Stage-2 Groups 1–14.
- Completed Group-15 master cross-group reconciliation and established **595** final L6 realization workflows.
- Populated `L6_Workflow_Realization_Register_DRAFT.md` with **1,084 / 1,084** master-reconciled rows and final `L6-RWF-####` reviewer-trace identities.
- Confirmed zero missing canonical scenario IDs, zero duplicate primary assignments, zero invalid cross-group workflow merges, and zero material unresolved upstream blockers.
- Added `validate-stage2-master-reconciliation.mjs`; the worker-corpus and master-register validation gates both pass with zero errors.
- Preserved A01 configuration governance, F13 analytics-definition governance, G00 exception recovery, corrected Section-I successors, and all canonical cross-domain ownership boundaries.
- Marked **STAGE 2 — PASS** and unblocked Stage 3 under its separate programme controls.
- No L0–L5 ownership, lifecycle, event, state, rule, authority, API, persistence, deployment or implementation architecture was redefined by Group 15.


## [Unreleased] - 2026-09-20 — Group-6 G00/A01 upstream remediation

- Added `ADR-G-EXCEPTION-001` and locked `L2-G00_Exception_Recovery_Governance_Amendment_2026-09-20.md`.
- Established G00 exception-recovery responsibilities: Governance Coordinator coordination, Campus Operations Manager ordinary recovery decisions, Executive Reviewer Administrative Override/escalated recovery authority.
- Added canonical G00 `GovernanceExceptionRecoveryRecorded` for non-escalation recovery while preserving existing `GovernanceEscalationRaised`.
- Added `ADR-PLAT-CONFIG-001` and locked `L2-A01_Configuration_Governance_Completion_Amendment_2026-09-20.md`.
- Completed the shared A01 configuration governance contract with owner-scoped Configuration Approval Authority, lifecycle operations and generic framework lifecycle events.
- Revalidated and updated `L3/L4-A-056`, `L3/L4-G-013` and `L3/L4-G-014`.
- Cleared both Stage-2 Group-6 blockers and marked Group 6 PASS without changing canonical domain ownership or introducing implementation architecture.

## [Unreleased] - 2026-09-20 — F13 analytics-definition governance remediation

- Added `ADR-FIN-ANALYTICS-001` to resolve the Stage-2 Group-5 blocker for `L3-F-076`.
- Added locked `L2-F13_KPI_and_Analytical_Model_Governance_Amendment_2026-09-20.md`.
- Bound KPI Definition and Analytical Model governance to the canonical Finance configuration lifecycle `Draft → Validation → Approval → Published → Active → Superseded → Archived`.
- Established `Financial Analytics Definition Approval Authority` and aggregate-specific KPI Definition / Analytical Model lifecycle events with one canonical publisher per event.
- Revalidated `L3-F-076`, re-derived `L4-F-076`, and cleared the Stage-2 Group-5 blocker without changing Finance ownership, source-fact ownership, H07 dashboard ownership, or implementation architecture.

## [Unreleased] - 2026-09-18 — L6 experience-slice planning pilot

**Draft branch / PR-pending change. No new Architecture Approval is asserted by this entry.**

- Retains the 99 business workspace families + 6 shared L6 surfaces as the architecture coverage baseline for the 1,084 canonical L3 scenarios.
- Adds `L6_Experience_Slice_Register_DRAFT.md` as a thin delivery locator for selected implementation increments; it does not replace the workspace inventory, journey map, traceability register, or canonical BH→L5 relation store.
- Clarifies the distinction between workspace-family coverage, UX journeys, delivery experience slices, supporting flows/states/fixtures, and frontend acceptance evidence.
- Synchronizes W-C02 planning/status language with the existing 2026-09-17 controlling proceed-gate decision; historical pre-remediation blocker wording is preserved only as superseded evidence.
- Strengthens `02_Implementation_Review/` with reproducible W-C02 cases, fixture-fidelity rules, state applicability, connected-journey acceptance, and separate architecture/implementation/integration readiness dimensions.
- No L0–L5 ownership, business rule, lifecycle, event, API, persistence, or authorization architecture is changed.

This document tracks changes to the SutraERP Architecture Repository layout, guidelines, baseline, and locks.

## [1.11.0] - 2026-08-17

**Controlled, one-time, repository-wide L3 Phase 1 candidate ID numbering migration.** Every active
section (A–J) is renumbered to a continuous range starting at `CAN-L3-<AREA>-001`. This is an
identifier/governance/traceability remediation only — no business scenario was added, removed, merged,
split, or reclassified, and no L0/L1/L2 business architecture changed. Sections G and I had each
already undergone an earlier, narrower "disjoint-start" renumbering earlier the same day (`G-542`…`G-863`,
`I-315`…`I-511`); this migration supersedes that approach for every section, including those two.

Complete old→new crosswalk:
`architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.
Validation record:
`generated/reviews/L3_Phase1_Continuous_Numbering_Migration_2026-08-17.md`.

Also corrected five pre-existing dangling cross-references where a citation had never been updated after
its target was renumbered by an earlier, separate consolidation (`CAN-L3-E-103`→`E-119`,
`CAN-L3-H-010`→`H-095`, `CAN-L3-H-050`→`H-133`, `CAN-L3-J-028`→`J-709`, `CAN-L3-J-029`→`J-710`, each
verified by exact Scenario Title match before correction), and resynchronized several governance documents
(`README.md`, `L3_Naming_Numbering_Standard.md`, `check_l3_inventory.py`) that had drifted from the live
repository state independent of this renumbering (Phase status, Policy Management ownership resolution,
`check_l3_inventory.py`'s digit-width cap).

## [1.10.0] - 2026-08-14

Pallav's assigned Sections B, D and E were corrected following the independent all-assigned Phase 1 audit and the user's architecture rulings. **Minor class — this release carries one locked L1 ownership statement down into L2 and consolidates candidate boundaries.** Section D moves from 38 field-incomplete parents plus 954 generated variant rows to **48 institutional-outcome candidates**; Sections B and E move from 101 to **107** combined. The total inventory moves from 101 conformant candidates to **155**.

### Changed — canonical ownership and contracts

- **Institutional policy:** `L2-B01` §2 now lists `Institution Policy` and `Institution Policy Lifecycle` as owned, with a new §2.1 clarification. This carries down the locked L1 business rule "Institute owns institutional policies" and L1's designation of `Institute Policy` as a canonical entity distinct from `Institute Profile`. No new ownership is created. `L2-A04` retains Authorization Policy and the Policy Engine; `L2-A00` continues to disclaim institutional policy. The two do not overlap.

### Fixed — L2 source defects

- **`L2-E02`:** workflow W-016 Administrative Correction named `W-018 Rollback Assignment` as its compensating workflow. The document's own §8.23 catalogue defines W-017 as Rollback Assignment and W-018 as Assignment Freeze. Corrected to `W-017`.
- **`L2-E06`:** §6.1 stated "four primary aggregates" while the table listed three. The `Payroll Configuration Aggregate`, already present in the section's aggregate diagram and carrying its own `Draft → Review → Approved → Active → Superseded → Archived` lifecycle, was restored to the table.

### Consolidated and corrected — L3 Phase 1

- `L3_Phase1_Inventory_Pallav.md` now contains 155 candidates: B 32, D 48, E 75. Every row has all eight required fields and approved Type/Priority values. File length falls from 1,490 to 247 lines.
- **Section D locked-format register established.** The 38 canonical parents previously carried only ID, Title, Owner and variant count; Trigger, Final Outcome, Domains, Type and Priority were absent for all 38 and were derived from `L2-D01` through `L2-D07`.
- **954 generated variant rows removed.** They used non-conforming `CAN-L3-D-001-V01` identifiers, added four columns beyond the locked eight, carried step-level fragment titles, and held only 458 distinct Final Outcome values across 954 rows. `Type` was fully determined by participating-domain count and `Priority` by scenario class, contrary to `L3_Scenario_Type_and_Priority_Rules.md` §6, §7 and §12. The FIND-D-01 through FIND-D-09 repository findings they carried are retained.
- **Thirteen missing outcomes added:** campus suspension (B-010); the D06 learning-opportunity lifecycle (D-039, D-040), previously absent so that no candidate covered making an elective or MOOC available; D05 governance review, completion certification and case reopening (D-041 – D-043); offering freeze/unfreeze (D-044); finalized internal assessment correction (D-045); external certificate verification (D-046); E02 assignment administrative correction, rollback and bulk operations (E-099, E-100, E-102); and payroll configuration (E-101).
- **Registration and activation ruled separate outcomes.** Adopted as the Section B convention; `CAN-L3-B-005` split into registration (B-005) and activation (B-023), matching the existing campus treatment and `L2-B02` WF01/WF02.
- **Splits:** `CAN-L3-D-006` into eligibility/restrictions, enrollment window and quota (D-006, D-047, D-048); `CAN-L3-E-018` into supplementary occupancy, delegated authority and additional charge, and committee membership (E-018, E-103, E-104).
- **Merges:** `CAN-L3-E-029` and `CAN-L3-E-030` retired into `E-028` and `E-031`; both described entities inside the FacultyWorkforce and Faculty Capability aggregates rather than independent outcomes.
- **Boundary and classification:** `CAN-L3-D-020` extended to Attendance Finalization per `L2-D04` §23; `D-020` and `D-021` reclassified `Critical`, matching the examples named in `L3_Scenario_Type_and_Priority_Rules.md` §6 and §9.
- FIND-D-06 and FIND-D-08 are superseded by D-041, D-042 and D-046.

### Validation

- Pallav inventory: 155 unique IDs; B 32, D 48, E 75; no duplicate IDs; no malformed rows; no invalid Type/Priority values; no empty Trigger or Final Outcome cells; retired IDs E-029 and E-030 absent.
- Audit verdict for `L3_Phase1_Inventory_Pallav.md` moves from `FAIL — MATERIAL COVERAGE GAPS` to a recommended `PASS WITH MINOR CORRECTIONS`, subject to independent re-verification.
- Original audit report remains unchanged as historical evidence; the correction application is recorded at `generated/reviews/L3_Phase1_Audit_Pallav_Correction_Application_2026-08-14.md`.

### Added — Phase 1 conformance gate

- **`check_l3_inventory.py`** at the repository root validates candidate inventories against the locked Phase 1 rules. `L3_Phase1_Discovery_Rules.md` §16.1 now requires a clean run before any inventory is accepted for review.
- Beyond field, ID, duplicate and Type/Priority validation, it detects the generation pattern that produced the withdrawn 954-row Section D catalogue: a low distinct-outcome ratio, and `Type` fully predicted by participating-domain count rather than institutional risk.
- Verified against known-bad inputs. The withdrawn 954-row catalogue fails with 954 errors; the 1,080-row pre-consolidation submission fails with 1,073, catching `P0<br><br>` HTML contamination across 990 rows and free-text Domains such as `A + B + E01` across 83.
- All five current inventories pass: **1,206 candidates, 0 errors, 221 warnings.** The warnings are two open conventions — Dhruv's Section J uses subdomain codes in `Domains` and mixes owner formats — plus five step-verb titles and three duplicate outcome texts in Devang's Section I.

### Open

- `L2-B05` publishes a `CalendarRestored` event but its §28 lifecycle defines no restoration transition. Referred to the B05 owner: either add the transition or withdraw the event. Not resolved here because it changes domain behaviour.
- `L2-E06` names an upstream "Compensation Domain" that has no L2 document and no Architecture Index entry. Compensation policy ownership remains unassigned.
- `Domains` convention: Sections A–I use section letters, Dhruv's Section J uses subdomain codes (`J01`–`J10`). `L3_Phase1_Discovery_Rules.md` §11 specifies section letters. Needs a ruling; currently a checker warning rather than an error.
- `Primary Owner` convention: most inventories use subdomain codes, Dhruv's Section A uses capability names. Needs one convention before Phase 2 grouping.

---

## [1.9.0] - 2026-08-13

Varad's assigned Sections F, G and H were corrected following the independent all-assigned Phase 1 audit and the user's architecture rulings. **Minor class — this release clarifies canonical ownership and consolidates candidate boundaries.** Section F moves from 1,858 step-level rows to **87 institutional-outcome candidates**; Section H moves from 893 malformed/fragmented rows to **85 conformant candidates**; Section G moves from 541 to **391** after all identified boundary, overlap, handoff and classification recommendations were applied.

### Changed — canonical ownership and contracts

- **Dashboard composition:** H07 owns dashboard metadata, layouts, workspaces, widgets and publication. F13 and Campus Operations retain source facts, reports, KPI definitions and governed projections. L2-F13, L2-G00 and the Dependency Map now state the handoff explicitly.
- **External credit recognition:** L2-C04 owns academic equivalency, approval/rejection, recognized credit and the Academic Credit Ledger. H10 owns provider verification, external evidence exchange, mapping, synchronization, recovery and reconciliation. C04 now publishes the authoritative recognition/rejection facts; H10 publishes only integration and decision-synchronization outcomes.
- **Accreditation readiness:** H09 owns the institutional accreditation-readiness decision. G00 owns Campus Operations evidence readiness; A10's operational go-live/certification eligibility remains separate and provisional under INT-302.
- **H09 source completion:** Added explicit integration contracts, dependency classes and locked decisions for evidence references, CAPA execution, SSR handoff, notifications, dashboards and audit evidence.

### Consolidated and corrected — L3 Phase 1

- `L3_Phase1_Inventory_Varad.md` now contains 563 candidates: G 391, F 87 and H 85. F/H IDs are contiguous and conform to `CAN-L3-<AREA>-NNN`; every row has all eight required fields and approved Type/Priority values.
- Removed 237 foreign Section A–J summary rows formerly placed under H09 and the unsupported former H10 provider-reinstatement candidate.
- Added the previously missing H05 research-proposal withdrawal and eleven material H09 outcomes.
- Corrected the G00 accreditation-evidence and representative G07/G08 dashboard boundaries; corrected `CAN-L3-C-029` to show the C04/H10 credit-recognition handoff.
- Applied all accepted Section G decisions: A07 audit-history ownership, one domain-owned configuration outcome per G subdomain, eight G07 security journeys, removal of vague exception catch-alls, consolidated mentoring context, J02 supplier-evidence handoffs, source-report/H07 dashboard separation, section-level Domains normalization, and final Type/Priority review. In total, 150 former G fragments were retired and 30 retained outcomes were rewritten.
- Added Section F and H traceability registers under L3 governance and updated the Section G register to close the former H-dependent ownership blockers.

### Validation

- Varad inventory: 563 unique IDs; F 87, G 391, H 85; no malformed rows; no invalid Type/Priority or Domains values; no obsolete Fxx/Hxx identifiers; no mojibake detected.
- Original audit reports remain unchanged as historical evidence; correction application and old-to-new crosswalks are recorded in the governance watchlists.

---

## [1.8.0] - 2026-08-12

Section I ownership conflicts resolved and consolidation completed. **Minor class — this release moves
canonical ownership in two further places.** Section I moves from 258 to **245 candidates**. The audit
verdict for `L3_Phase1_Inventory_Devang.md` moves from `CONDITIONAL PASS` to
`PASS — COMPLETE FOR PHASE 1`, readiness score 89 → 94.

### Changed — L2 canonical ownership
- **`I-RC-02` ruled: `L2-I15` owns suspension, resumption, rescheduling and cancellation of an
  interrupted in-progress examination.** `L2-I06` §10.7 and `L2-I07` §10.7 enumerate possible outcomes
  and then defer — "Institutional authorities shall govern continuation decisions" — without defining a
  workflow, preconditions or postconditions, while `L2-I15` §11.1 specifies EW-09 to EW-12 in full and
  §9.1 reserves exception cases exclusively to that domain. `L2-I06` and `L2-I07` each amended with an
  Ownership Resolution note, a §10.7 redirect and a §14 non-ownership row. The conduct domains detect
  and report; conducting a rescheduled or re-conducted examination remains theirs.
- **`I-RC-03` ruled: `L2-I04` owns the Examination Registration aggregate and its lifecycle for every
  registration category, including Supplementary, Backlog and Improvement.** `L2-I04` §7.3 enumerates
  Regular, Backlog, Supplementary, Improvement and Special as categories of one registration mechanism,
  and §20 defines a single Registration State Model governing "Every Examination Registration".
  `L2-I12` amended in six places — Ownership Resolution note, §2, §5.7 retitled to Examination
  Registration Initiation, §14 owned and non-owned lists, §18 capability catalogue, §22. `L2-I12`
  retains application, eligibility, fee verification, session allocation and attempt lifecycle, and
  initiates registration in `L2-I04`.

### Fixed — L2 document integrity
- `I-RC-04` — `L2-I13` §14 note recording that "CGPA" in the document title denotes CGPA consumption
  for completion verification, not calculation, which `L2-I09` owns. Title retained because
  `ARCHITECTURE_INDEX.md` and `L2-I09`, `L2-I11`, `L2-I12` reference it.
- `I-RC-06` — `L2-I16` §13 read "Closed archive lifecycles shall not be reopened unless permitted by
  institutional policy", contradicting §21.5 (reopening is an invalid transition) and §9.3 (immutable
  finalized sessions). Re-worded: reopening is prohibited outright, and policy-permitted post-closure
  access is served by authorised retrieval and restoration under §21.4.

### Retracted
- **`I-RC-05` withdrawn as an audit error.** The 1.7.0 entry recorded `L2-I00` as citing a non-existent
  "L2 Domain Workflow Matrix". `architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md`
  exists (11,763 bytes); the original check ran from a drifted working directory. No defect, no change
  required. Corrected in the audit report §3 and the Section I watchlist §16.2.

### Added
- `CAN-L3-I-314` Prepare and Authorize an Examination Contingency Plan — `L2-I15` EW-04, EW-05, split
  out of the over-broad `CAN-L3-I-290`.
- New repository conflict `I-RC-07`, unresolved: **question paper governance has no owner in Section I.**
  `L2-I07` §2 excludes Question Paper Creation, Moderation and Printing and §9.5 states governance
  "remains outside this document"; `L2-I14` §9.2 and `L2-I15` §9.2 both disclaim question papers.
  `CAN-L3-I-111`, `-112` and `-113` rest on an unassigned capability. Cannot be settled between existing
  documents; needs a new owner or a new L2 document.

### Consolidated
- 14 rows retired, 11 rewritten, 1 added. `I-DU-05` (six rows into `I-291`, `I-292`, `I-293` under the
  `I-RC-02` ruling); `I-DU-07` (three rows into `I-033`, `I-035`, `I-037` under the `I-RC-03` ruling,
  with `I-047` and `I-217` re-scoped); `I-FR-06`/`I-LS-01` (`I-036` into `I-035` — `L2-I04` §19 glosses
  Registration Withdrawal as "controlled registration cancellation" and §20 defines no `Cancelled`
  state); `I-FR-13`, newly found (`I-070`, `I-071` into `I-069`; `I-073` into `I-072`; `I-077` into
  `I-076` — `L2-I06` §4.3 *Unified Assessment Conduct Framework* requires all non-theory assessment to
  follow "a common business workflow regardless of assessment type"); `I-OB-03` and `I-OB-04` split and
  narrowed. `I-DU-10` and `I-DU-11` examined and closed as genuinely distinct, both retained.

### Known open
- `I-RC-07` — question paper ownership. The only substantive item this round could not close.
- `Priority` not re-derived: 41 `P0`, 203 `P1`, 1 `P2`. Reserved to Lead Architect review under
  `L3_Scenario_Type_and_Priority_Rules.md` §15.
- Cross-inventory boundaries with Sections C, G and D/E need the counterpart discovery owners.
- Candidate IDs remain sparse (`CAN-L3-I-001` … `CAN-L3-I-314` with gaps); renumbering deferred to
  Phase 2 close.
- The 1.7.0 estimate of 90–110 material outcomes for Section I is withdrawn as too aggressive. Against
  the sixteen L2 workflow catalogues a defensible target is 200–230; at 245 the section is close to it.

---

## [1.7.0] - 2026-08-12

Section I Phase 1 inventory audited, corrected and consolidated. **Minor class — this release moves
canonical ownership.** A Lead Architect ruling on repository conflict `I-RC-01` assigns Examination
Session closure and archival to `L2-I16`, and `L2-I02` was amended accordingly. This is the first
release in the L3 effort to change L2 content.

Section I moves from 304 candidates as submitted, to 311 after conformance, to **258 after
consolidation**. The audit verdict for `L3_Phase1_Inventory_Devang.md` moves from
`FAIL — MATERIAL COVERAGE GAPS` to `CONDITIONAL PASS`, with one condition outstanding: repository
conflict `I-RC-03` (`L2-I04` versus `L2-I12` ownership of supplementary, backlog and improvement
registration) was raised for ruling and has not been answered.

### Changed — L2 canonical ownership
- `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I02_Examination_Session_Management_Workflows.md`
  — 17 edits under the `I-RC-01` ruling. `L2-I02` §14 previously listed Session Closure as owned
  while the same section assigned "Session Closure & Archival Operations" to `L2-I16`, and `L2-I16`
  §9.1 declared exclusive ownership of the Examination Session Aggregate after operational
  conclusion. Session Closure removed from the owned lists in §2, §4, §14 and §22; §5.7, §8.10,
  §8.11, §9.7 and §10.9 redirected to `L2-I16` as consuming context; the two closure events in §11
  replaced with a single hand-over event; the §20 state model truncated at `Reopened` with the
  hand-off to `L2-I16` §21 recorded; an Ownership Resolution note added. **No other L2 document was
  changed and no other ownership was moved.**

### Added
- `generated/reviews/L3_Phase1_Audit_Devang_ALL_ASSIGNED.md` — the independent adversarial audit,
  its post-conformance re-rating (Part II) and its post-consolidation re-rating (Part III).
  Readiness score 63 → 79 → 89.
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_I_Phase2_Consolidation_Watchlist.md`
  — twelve fragmentation clusters, four over-broad candidates, eleven duplication findings, six
  repository conflicts, five cross-inventory relationships, and the full execution record.
- Nine candidates traced to repository evidence:
  - `CAN-L3-I-305` Malpractice Case Adjudication to Penalty Determination. `L2-I14` §21.2 defines
    `Decision Pending → Decision Recorded → Penalty Assigned`; MW-12 postconditions are "Official
    decision finalized. Decision becomes immutable." No candidate produced this outcome, while
    `I-170`, `I-188`, `I-256`, `I-268` and `I-286` all consumed it as a trigger.
  - `CAN-L3-I-306` Malpractice Case Closure and Archival — `L2-I14` MW-17, MW-18, §21.6.
  - `CAN-L3-I-307` Examination Exception Case Resolution to Closure — `L2-I15` EW-13, EW-14.
  - `CAN-L3-I-308` Retire an Examination Policy with Historical Preservation — `L2-I01` §8.6, EX-005.
  - `CAN-L3-I-309` Postpone and Reschedule an Examination Under Authorized Contingency — `L2-I15`
    EW-08, EW-09.
  - `CAN-L3-I-310` Authorized Disposal of Archived Examination Records After Retention Expiry —
    `L2-I16` SW-11, §9.2 Disposal Authorization, §21.3 `Retention Expired → Pending Disposal → Disposed`.
  - `CAN-L3-I-311` Apply and Release Legal Hold on Examination Records — `L2-I16` §9.2 Legal Hold
    Record, §21.4 `Retention Active ↔ Legal Hold`.
  - `CAN-L3-I-312` Examination Record Archival — `L2-I16` SW-07, SW-08, split from `I-301`.
  - `CAN-L3-I-313` Subsequent Examination Execution Coordination — `L2-I12` §5.9–5.12, the
    initiation and coordination outcomes that document actually owns.

### Fixed
- Seven rows were unedited template output, with Trigger reading `An authorized requirement triggers
  <title>` and Final Outcome reading `The workflow <title> is successfully completed`. Six were
  authored from their L2 source, including `CAN-L3-I-177` Publish Official Examination Results —
  the flagship P0 outcome of the section. The seventh, `CAN-L3-I-302`, was removed instead: see below.
- Twelve rows under the I12 heading asserted final outcomes that `L2-I12` §14 explicitly disclaims —
  Hall Ticket Generation, Practical and Theory Conduct, Marks Collection and Academic Evaluation —
  where `L2-I12` owns only the corresponding *initiation* and *coordination* capabilities. Removed;
  their outcomes are carried by `CAN-L3-I-065`, `-083`, `-107` and `-145` under canonical owners.
- `Primary Owner` read the flat value `Examination` on all 304 rows. Normalised to canonical
  `I01`…`I16` codes, matching the `F00`…`F14` and `G00`…`G10` conventions.
- `CAN-L3-I-255` claimed a CGPA outcome for the completion domain, which `L2-I13` §14.3 states it
  "shall never own". Re-worded to consume the CGPA recomputed by `L2-I09`.
- `Type` re-derived across all 258 rows against `L3_Scenario_Type_and_Priority_Rules.md` §7.
  Final distribution: `Critical` 233, `Cross-domain` 18, `Simple` 7. Type remains provisional
  under §15 and is not locked by this release.
- Document header and per-block `Source` metadata added; blocks I04–I16 previously carried neither.

### Removed
- `CAN-L3-I-302` Reopen Closed Examination Session for Authorized Correction — unsupported.
  `L2-I16` §21.4 defines no transition out of `Session Finalized`, §9.3 requires immutable finalized
  sessions, and §21.5 lists reopening as invalid; reopening a *suspended* session is already
  `CAN-L3-I-015` under `L2-I02` §5.6.

### Consolidated
- Eleven fragmentation clusters merged, retiring 45 rows into twelve rewritten candidates; two
  over-broad candidates split or narrowed. Executed under explicit Lead Architect authorization,
  ahead of the ordinary `L3_Phase1_Discovery_Rules.md` §17 sequence. Every cluster and its evidence
  is recorded in the Section I watchlist §15.

### Known open
- `I-RC-03` — `L2-I04` §5.5–5.7 and `L2-I12` §5.7/§14 both claim supplementary, backlog and
  improvement registration. Raised for ruling, unanswered. Leaves `CAN-L3-I-047` and `CAN-L3-I-217`
  a known duplicate.
- `I-RC-02` — suspend and resume of an in-progress examination claimed by `L2-I06`, `L2-I07` and
  `L2-I15`. Keeps `CAN-L3-I-291` and cluster `I-DU-05` blocked.
- `I-RC-06` — `L2-I16` §13 permits reopening a closed archive lifecycle "unless permitted by
  institutional policy" while §21.5 lists it as an invalid transition. Document-integrity defect,
  left for separate correction.
- Candidate IDs are sparse after consolidation (`CAN-L3-I-001` … `CAN-L3-I-313` with gaps).
  Renumbering deferred to Phase 2 close per `L3_Phase1_Discovery_Rules.md` §6.
- `Priority` not re-derived: 42 `P0`, 215 `P1`, 1 `P2`.
- Sections B, D and E contain no examination-facing candidates, while Section I names `E` in
  roughly fifteen rows and `D` in roughly thirty. Not a Section I defect; unowned.

---

## [1.6.4] - 2026-08-11

Section G Phase 1 inventory corrected following independent adversarial audit. Patch class —
Phase 1 discovery corrections and documentation only. **No L1 or L2 content was changed, no
canonical ownership moved, no repository conflict was resolved, and no candidate was renumbered,
merged, split or removed.** Section G moves from 539 to 541 candidates.

The audit verdict for `L3_Phase1_Inventory_Varad.md` as a whole remains
`FAIL — MATERIAL COVERAGE GAPS`, and is **not** cleared by this release. Sections F (15 locked
L2 documents) and H (11 locked L2 documents) are assigned to the same inventory under
`L3_Phase1_Workload_Distribution_Decision.md` §2 and remain entirely undiscovered. This release
corrects Section G only.

### Added
- `generated/reviews/l3-phase1-section-g-corrections-2026-08-11.md` — the correction manifest,
  recording every changed row with its before and after value, the rules applied, the L2
  evidence used, and the list of items deliberately not changed.
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_G_Phase2_Consolidation_Watchlist.md`
  — nine fragmentation clusters, six over-broad candidates, eight overlap clusters, three
  cross-inventory relationships with Section J, and four open items.
- Two Section G candidates, each traced to repository evidence:
  - `CAN-L3-G-540` Academic Session Readiness Assessment to Session Commencement Decision.
    `L2-G00` §16 enumerates seven readiness types; six had a candidate (`CAN-L3-G-001` through
    `CAN-L3-G-006`). Academic Session Readiness had none. Its academic-calendar trigger and its
    D and B participation distinguish it from the consolidated daily readiness candidate.
  - `CAN-L3-G-541` Medical Clearance Request to Institutional Clearance Decision. `L2-G06` §12
    Administrative Operations lists four operations; three were covered (Medical Certificates by
    `CAN-L3-G-214`, Sick Leave Recommendation subsumed by the same via §22, Occupational Health
    by `CAN-L3-G-220`). Medical Clearance had none. Structurally parallel to library clearance
    (`CAN-L3-G-182`) and hostel exit clearance (`CAN-L3-G-108`).

### Fixed
- 109 candidate rows carried the Priority value `P1` in the Type column, which
  `L3_Phase1_Discovery_Rules.md` §12 does not permit. Each was reassigned a valid Type.
  Type remains provisional under §12 and is not locked by this release.
- `CAN-L3-G-395` carried nine data cells against the locked eight-column schema. The stray Type
  value was removed; `Cross-domain` was retained to match sibling referral rows `CAN-L3-G-391`
  through `CAN-L3-G-397`, and the authored `P0` was preserved.
- 43 rows carried unresolved placeholder text in the Domains column (`relevant affected domain`,
  `responsible G domain(s)`, `participating domains` and six further variants), against
  `L3_Phase1_Discovery_Rules.md` §11. Each was resolved to concrete section letters using the
  Bounded Context Relationships section of the governing L2 document.
- 59 rows used six non-canonical Domains tokens (`Finance`, `Security`, `Institutional
  Reporting`, `Analytics`, `Accreditation`, `Audit`). Normalised to `F`, `G07`, `H`, `H`, `H`
  and `A` respectively, with de-duplication. Four of the six had been concealing the Section H
  dependency on 28 rows.
- Inventory heading structure. The G09 block had no section heading; the G10 block used a
  non-standard heading; a stray empty `#` heading followed the G09 notes.

### Deferred
- Approximately 122 fragmented candidates across nine clusters, and six over-broad candidates.
  Merging and splitting are assigned to Phase 2 by `L3_Phase1_Discovery_Rules.md` §17 and are
  recorded on the watchlist rather than applied.
- Priority distribution (275 of 541 candidates at `P0`) and 235 `Critical` classifications
  await an institutional P0 definition and risk threshold. Both remain provisional under §12
  and §13.
- Ownership of approximately 20 reporting and dashboard candidates against `L2-H07` §1.1, which
  names the Enterprise Dashboard domain the authoritative owner of dashboard composition and
  governance. Blocked until Section H is discovered.
- Consolidation of `CAN-L3-G-006` with `CAN-L3-A-047`, blocked on INT-302.

## [1.6.3] - 2026-08-09

Section C Phase 1 inventory corrected following independent verification. Patch class —
Phase 1 discovery corrections and documentation only. **No L1 or L2 content was changed, no
canonical ownership moved, no repository conflict was resolved, and no candidate was
removed.** Section C moves from 71 to 74 candidates.

### Added
- `generated/reviews/l3-phase1-section-c-verification-2026-08-09.md` — the independent
  adversarial verification of `L3_Phase1_Inventory_Chetan.md`. Verdict at audit time:
  `FAIL — BOTH COVERAGE AND ARCHITECTURE PROBLEMS`; `PASS` after application, subject to the
  recorded conflicts. Supersedes the 2026-08-08 Section C review, which audited the
  pre-remediation 45-row version.
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_C_Phase2_Consolidation_Watchlist.md`
  — four candidate relationship clusters (2 `CLOSED`, 2 `OPEN`) and nine repository conflicts
  (2 `DETERMINED`, 5 `OPEN`, 1 `BLOCKED`, 1 conflict-dependent wording constraint).
  Registered in the L3 `README.md` folder tree and governance list.
- Three Section C candidates, each traced to repository evidence:
  - `CAN-L3-C-072` Admission Document Verification and Approval Decision. The
    `Admission Initiated → Admission Approved` span of `L2-C01` §6.4/§6.9.1 had no candidate:
    `CAN-L3-C-001`'s trigger begins at "Admission is approved", placing WF-C01-02 Document
    Verification and WF-C01-03 Admission Approval outside the inventory.
  - `CAN-L3-C-073` Registration Hold Placement and Release. `L2-C03` §5.3 grants Academic
    Registration **Full** ownership of `Registration Hold`, §6.6 defines it as an owned child
    entity, and §12.8 defines the governed `OverrideRegistrationHold()` service. No candidate
    placed or released a hold.
  - `CAN-L3-C-074` Inter-Institute Student Transfer with Student Context Re-establishment.
    `L2-C02` §11.10.3 defines Institute-crossing transfer as originating context terminal plus
    a new context at the receiving Institute under SC-INV-004 — a different terminal outcome
    from the intra-Institute form.

### Changed
- `CAN-L3-C-011` split. Retitled **Intra-Institute Student Academic Reassignment** and
  restricted to same-Institute campus, department, batch and section change per `L2-C02`
  WF-C02-08 §1. Its stated outcome ("Student Context reflects the completed transfer")
  contradicted `L2-C02` §11.10.3 for the Institute-crossing case; that case is now
  `CAN-L3-C-074`.
- `CAN-L3-C-005` outcome corrected. It asserted that readmission restores the admission
  lifecycle, contradicting `L2-C01` §6.13 ("No terminal Admission Record shall ever be
  reopened") and §7.6 rule 6. Now states that a new Admission Record is created and linked to
  the same canonical Person with the prior terminal record preserved and closed. `Domains`
  `C` → `A, C`, which also resolves a `Cross-domain` type carrying a single domain.
- `CAN-L3-C-004` extended to name the policy-controlled `Student Created → Revoked` path
  (`L2-C01` §6.9.1) and the hand-off of the consequential Student Context disposition. The
  unwinding behaviour itself remains unowned — recorded as conflict CF-06.
- `CAN-L3-C-034` `Domains` `A, C` → `C, H`. `ARCHITECTURE_INDEX:221` and `L2-C04` §18.10 place
  APAAR & ABC integration in `L2-H01`. `L1` §4 "APAAR belongs to Identity" governs
  APAAR-as-identity, not ABC credit synchronisation. Credit Ledger ownership unchanged.
- `CAN-L3-C-029` `Domains` `C` → `B, C, H`; `L2-C04` APW-07 lists External University, ABC,
  NEP Transfer and Exchange Programme as credit sources.
- `CAN-L3-C-022` extended to Improvement Registration (`L2-C03` §4.3, §15).
- `CAN-L3-C-046` extended to qualification completion cancellation before conferment
  (`L2-C05` §11.12).
- `CAN-L3-C-059` retitled **Scholarship Definition Lifecycle Governance** and extended across
  the full `L2-C07` §6.6 definition lifecycle including suspension, expiry and archival.
- `CAN-L3-C-066` `Type` `Cross-domain` → `Simple`; it listed a single domain, and `L2-C07`
  §2.2 places external portal business logic outside the domain.
- `L3_Phase1_Inventory_Chetan.md` header gains a `Last Revised` line pointing at the
  verification review and the Section C watchlist.

### Not changed — recorded deliberately
- **Policy Management remains unowned.** `L2-C08` (10 references) and `L2-C04` (3 references)
  depend on it; no publisher exists (`DEPENDENCY_MAP` section 7, status Open; `L2-C04` §2.6
  records it in-document as "unresolved, no owner"). The two policy-lifecycle candidates this
  would support were **not** created. Their absence is correct until the Lead Architect rules.
- **No standalone Admission Rejection candidate.** `L2-C01` WF-C01-03 §2.7 EX-01 places an
  admission in a `Rejected` state and §3.2 publishes `AdmissionRejected`, but no canonical
  state list (§6.8, §6.9, §6.9.1, §6.13, §10.3) contains `Rejected`. The negative decision is
  carried inside `CAN-L3-C-072` rather than creating a candidate against an undefined state.
- **Nine repository conflicts were surfaced, not resolved**, per
  `L3_Phase1_Discovery_Rules.md` section 14. They include two incompatible admission state
  vocabularies (`L2-C01` §6 vs §10.3), duplicate guardian capability definition (`L2-C02`
  WF-C02-13.3 vs `L2-C06` §5), and Alumni Transition ownership stated three ways (`L2-C04`
  §1.5, `L2-C05` §5/§11.10, `L2-C08` §5.2). All are held on the Section C watchlist.

## [1.6.2] - 2026-08-08

Section A Phase 2 consolidation watchlist resolved and relocated. Patch class — corrections
and documentation structure only. **No candidate was consolidated, no candidate ID changed,
and the Section A count remains 64.**

### Added
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_A_Phase2_Consolidation_Watchlist.md`
  — the authoritative record of the eighteen Section A relationship clusters, each resolved
  to `CLOSED` (10), `DETERMINED` (3), `OPEN` (3) or `BLOCKED` (2). Registered in the L3
  `README.md` folder tree and governance list.

### Changed
- Two recommendations from the 2026-08-08 verification were **withdrawn on re-verification**:
  - **A-C08** — "merge A-029 into A-028" was wrong. `L2-A07:1391-1393` defines A07-W11
    Retention Governance, A07-W12 Evidence Archival and A07-W13 Evidence Restoration as
    three separate workflows, and `L2-A07:1408-1409` places them in two different workflow
    categories. A-028, A-029 and A-030 are retained as three candidates.
  - **A-C14** — "merge A-045 into A-044" downgraded to an open Phase 2 question. `L2-A09`
    MD-18 has its own actor, authorization gate and published event, and delivers archival
    storage and preserved searchability that MD-11/MD-12 retirement does not. A precondition
    establishes sequence, not equivalence.
- Two clusters left unresolved by that verification were **closed**:
  - **A-C07** — determined. `L2-A07:1407` places A07-W07, A07-W08 and A07-W09 in a single
    `Investigation` category; `:1899-1901` shows W07 and W08 feeding W09; `:867`, `:1286`
    and `:831` establish both inputs as derived artifacts owning no lifecycle. A-025 and
    A-026 merge into A-024 in Phase 2.
  - **A-C13** — closed. A-041 is retained. `L2-A09` MD-14 has its own service (§18.10),
    export policy gate, `DatasetExported` event and failure outcome, and is a consumer pull
    where MD-15 is a publication push.
- Section 3.3 of `L3_Phase1_Inventory_Dhruv.md` reduced to a pointer, and section 3.1
  condensed, restoring compliance with `L3_Phase1_Discovery_Rules.md` section 3 ("candidate
  scenario rows only") and section 16 ("should not include long explanations").
- Section 7 of `L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md` marked superseded
  and reduced to a change record of the two withdrawals, eliminating a third divergent copy
  of the same analysis.

### Unchanged
- **INT-301 and INT-302 remain open.** Clusters A-C15 and A-C17 stay `BLOCKED` and no
  candidate in the A10 block is authorable. No locked L1 or L2 content was amended.

---

## [1.6.1] - 2026-08-08

Section A L3 Phase 1 audit applied. Scope is Phase 1 discovery plus two Patch-class
defect corrections. **No canonical ownership, lifecycle, event, dependency direction or
business-rule content changed, and no locked decision was overridden.** Two blocking
repository conflicts were recorded, not resolved.

### Added
- `generated/reviews/l3-phase1-section-a-verification-2026-08-08.md` — independent
  verification of CAN-L3-A-001 through CAN-L3-A-055 against the L2 Section A corpus,
  locked L1, the Dependency Map and the L3 Phase 1 governance standards.
- `generated/reviews/review-l3-section-a-conflicts-2026-08-08.md` — conflict register in
  the existing `Issue ID : INT-NNN` format, new `INT-3xx` block.
  - **INT-301 (Critical, OPEN)** — `L2-A02`, `L2-A03` and `L2-A04` claim duplicate
    *exclusive* ownership of Credential, Authentication Session, MFA / Authentication
    Factors, Role, Role Assignment and Delegation, against `L2-A00:427` "Ownership SHALL
    never be shared". Requires an Enterprise Architecture Board decision under
    `L2-A00:3031`. **No L2 ownership statement amended.**
  - **INT-302 (Critical, OPEN)** — locked L1 sections 15 and 29 define Operational
    Readiness as calculated, not stored and dynamically evaluated, with canonical entities
    Readiness Rule and Readiness Result; `L2-A10` defines five persisted aggregates
    including an immutable Operational Readiness Certificate, and `L2-A10:2920` has B01
    update an institution readiness flag that `L1:1539` says does not exist.
    `L1:3128` states no Level 2 workflow may violate a locked decision. **Neither locked
    L1 nor `L2-A10` amended.**
  - INT-303 and INT-304 — Medium, both fixed; see Changed.
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md`
  records the inputs reviewed, conflicts recorded, corrections applied, every candidate
  change with evidence, the candidates deliberately **not** added and why, the Phase 2
  watchlist, the remaining blockers, the validation results and the final Section A
  coverage audit. Registered in the L3 `README.md` folder tree and governance list.
- Nine Phase 1 candidates, `CAN-L3-A-056` through `CAN-L3-A-064`, from a second
  source-first discovery pass over A01–A05. They close zero-coverage gaps on three
  canonical `L2-A04` aggregates (Workspace, Separation of Duties Policy, Authorization
  Policy), the `L2-A02` identity-closure and population-onboarding families, the `L2-A03`
  Authentication Provider aggregate, the `L2-A01` §6 Platform Configuration Framework, the
  Tenant Maintenance and Upgrade lifecycle states, and `L2-A05` Human Task Management.
  Numbering continues sequentially; **no existing candidate ID was renumbered.**

### Changed
- `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A09_Master_Data_Workflows.md`
  (**INT-304**) — sections 3.4 and 59 assigned Configuration Data to "the configuration
  management domain that will be defined separately in A10". `L2-A10` is Operational
  Readiness Workflows and defines no configuration responsibility; the reference was a
  dangling pointer. Both occurrences repointed to `L2-A01` section 6 with its section 6.5
  layer distribution named. A09's substantive boundary statement — that the Master Data
  Platform does not absorb configuration ownership — is unchanged. **No configuration
  domain was created and no ownership was invented or moved.**
- `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A08_Module_Registry_Configuration_Workflows.md`
  (**INT-303**) — the title says "Module Registry **& Configuration**" while section 2.2
  excludes runtime configuration and section 5 lists seventeen owned concepts, none of
  them configuration. A scope-clarification note was added after the H1 stating the title
  establishes no configuration ownership, and section 5.1's ambiguous "Platform Foundation"
  pointer was made precise. **The document was deliberately not renamed:** it is `Locked`,
  its title string is referenced in eight files including the historical
  `SECURITY_REVIEW_CHANGELOG.md`, and changing a locked document's identity is Major class
  under `Volume_5_Change_Management.md` section 7, not a defect correction. A rename
  remains available to the Architecture Review Board.
- `architecture/L3_Business_Scenario_Architecture/01_Phase1_Discovery/L3_Phase1_Inventory_Dhruv.md`
  — 52 of the 55 existing Section A candidates edited: 43 Domains fields converted from
  role placeholders to section letters or the `X` code, 9 Type reclassifications under the
  `L3_Scenario_Type_and_Priority_Rules.md` section 7 selection order (correcting the
  zero-`Simple` imbalance that section 14 identifies as a classification mistake), 4
  Priority reassessments, 24 wording corrections, and 14 conflict markers. Two invented
  clauses were removed — "revalidated as required" (A-034) and "validated" (A-003) — and
  one architecture-repository phrase was removed from an institutional outcome
  ("repository traceability", A-055). A Phase 1 Governance Note and an 18-cluster Phase 2
  Consolidation Watchlist were added. **No candidate was renumbered, merged, split or
  deleted; discovery breadth is preserved and merge/split decisions are deferred to
  Phase 2.**
- `architecture/L3_Business_Scenario_Architecture/README.md` — the new application report
  registered in the folder tree and the governance document list, with the two open
  blockers named.

### Unchanged
- **L1** (`Status: LOCKED`) and every **L2** document except the two Patch-class
  corrections above. `L2-A02`, `L2-A03`, `L2-A04` and `L2-A10` were read in full at their
  ownership, lifecycle, aggregate, workflow and event sections and **deliberately not
  edited**: their conflicts are recorded as INT-301 and INT-302 for board decision.
- `ARCHITECTURE_INDEX.md`, `L2_DOMAIN_WORKFLOW_MATRIX.md`, `DEPENDENCY_MAP.md`,
  `TEAM_ASSIGNMENTS.md`, the constitution volumes and the L3 Phase 1 governance standards.
  No Phase 1 rule was added or amended.
- The four other Phase 1 inventory files, which remain empty. Repository-wide Phase 1
  discovery is incomplete and Section A cannot be consolidated against sections that have
  not been discovered.

---

## [1.6.0] - 2026-08-07

L0, L1 and L2 locked. This is the final lock decision for the business architecture
levels. No architectural content changed: only document status metadata and the
status columns of the index, matrix and AI-context trackers were touched.

### Changed
- **All 107 L2 documents are now `Locked`.** 104 were promoted from
  `Trust Certification Ready`; 252 status declarations across 105 files under
  `architecture/L2_Domain_Workflow_Architecture/` were updated (header status lines,
  Document Metadata tables, and status-like footers together, preserving the INT-020
  one-canonical-status-per-document invariant).
- `L2-E04_HR_Workflows.md`, `L2-E05_Leave_Management_Workflows.md` and
  `L2-E06_Payroll_Workflows.md` carried `Draft` in their Document Metadata tables and no
  header status line, and were the three documents the index recorded as unresolved.
  Their metadata status is now `Locked` (E05's malformed multiline `Draft`/`Phase` cell
  was collapsed to a single canonical value; E06 was corrected in both its header
  metadata table and its §25.1 Document Status table).
- `architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md` — all 110
  per-document status entries updated to `Status: [Locked]`.
- `architecture/ARCHITECTURE_INDEX.md` — 104 L2 rows moved from
  `Trust Certification Ready` to `Locked` and 3 from `Draft`; the status note under the
  Document Status Legend rewritten to record the lock and to retire the
  "E04/E05/E06 excluded" exception; a Recent Changes entry added.
- `AI_CONTEXT/REPO_UNDERSTANDING.md` — 106 L2 rows synced to `Locked`. This also cleared
  10 rows that had drifted out of sync with the index and still carried the pre-INT-020
  values `Architecture Locked`, `Approved Baseline`, `Architecture Baseline`,
  `Architecture Approved` and `Certified & Locked` (`L2-A00`, `L2-B02`, `L2-D00`
  through `L2-D07`).
- `AI_CONTEXT/SECTION_J/README.md` — section status synced to `Locked`.

### Unchanged
- **L0** (`SutraERP_Architecture_Workflow_Context_Baseline.md`, `Document Status: Locked`)
  and **L1** (`SutraERP_L1_Business_Domain_Architecture_Locked.md`, `Status: LOCKED`)
  were already locked. Verified, not edited.
- **L3** and **L5** are outside the requested scope. L3 governance documents keep their
  Phase 1 statuses and `L5-00`/`L5-01` remain `Trust Certification Ready`.
- `TEAM_ASSIGNMENTS.md` remains `Active`: it is an ownership control document, not an
  architecture specification, and the index records it as such.
- The status vocabulary in `governance/constitution/Volume_2_Repository_Standards.md` §7,
  the legend tables that mirror it, `README.md`, `CONTRIBUTING.md` and the historical
  records under `generated/reviews/` were not modified.

---

## [1.5.3] - 2026-08-07

L3 Phase 1 discovery workload distribution locked. Scope is Phase 1 discovery only:
no canonical ownership, dependency, lifecycle, event or business-rule content changed.

### Added
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Workload_Distribution_Decision.md`
  records the locked Phase 1 discovery distribution, its rationale, its non-impact on
  canonical ownership, and its change-control rules. Registered in the L3 `README.md`
  folder tree and governance document list.

### Changed
- **Locked Phase 1 discovery distribution** — Dhruv: Sections A and J, cross-domain
  candidates, governance, pilot framing. Varad: Sections G, F and H. Pallav: Sections B,
  D and E. Chetan: Section C. Devang: Section I. Applied to
  `L3_Phase1_Team_Instructions.md` section 5 and to
  `SutraERP_Level_3_Development_Path_Team_Discussion.md` section 12, replacing the
  previous split (Dhruv: A + H; Varad: Student Lifecycle/Workforce; Pallav: Institution/
  Delivery; Devang: Examination + Campus Operations; Chetan: Finance/Procurement/
  Inventory/Asset). The mandatory non-impact wording accompanies every occurrence.
- `SutraERP_Level_3_Development_Path_Team_Discussion.md` — removed the illustrative
  Creator / Mandatory Reviewers example table. It embedded the superseded per-person
  mapping and would have survived as a stale distribution reference. Replaced by a
  statement that creator and reviewer assignment is decided during Phase 2 consolidation
  and is not implied by the Phase 1 distribution. The file keeps its
  `Discussion Draft — Not Yet Locked` status.
- `AI_CONTEXT/L3_Phase1_Governance_Context.md` regenerated from the seven canonical L3
  governance sources, now including the new decision file. The previous build was
  mojibake-corrupted from an earlier cp1252 round-trip: every em dash, arrow and
  box-drawing character had been mangled into a three-character sequence. The rebuild
  reads and writes UTF-8, so em dashes, arrows, bullets and table pipes are correct. This file is a convenience AI context file only; the separate
  governance files remain canonical.
- `AI_CONTEXT/REPO_UNDERSTANDING.md` — the L3 section claimed "placeholder folder only,
  NO SOURCE DOCUMENTS PRESENT" and quoted a README text that no longer exists. Corrected
  to record that Phase 1 discovery is active with six governance documents present and no
  L3 scenario documents, and to carry the Phase 1 distribution with an explicit warning
  that it is not section ownership. The canonical Domain Map and Team Ownership tables in
  the same file are unchanged.

### Unchanged (deliberately)
- `ARCHITECTURE_INDEX.md`, `DEPENDENCY_MAP.md`,
  `SutraERP_Architecture_Workflow_Context_Baseline.md`,
  `SutraERP_L1_Business_Domain_Architecture_Locked.md`, `L2_DOMAIN_WORKFLOW_MATRIX.md`,
  `architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md`, all
  `*_Canonical_AI_Context.md` files, `README.md` and `CONTRIBUTING.md`. Canonical section
  ownership (A and H: Dhruv; B and D: Pallav; C and E: Varad; F and J: Chetan; G and I:
  Devang) and the Finance Review Team are untouched and continue to differ from the
  Phase 1 discovery distribution by design.

---

## [1.5.2] - 2026-08-06

Remediation against `generated/reviews/review-repository-integrity-2026-08-06.md`.
Reconstructed from commit history and dated in-file notes; the 1.5.0 and 1.5.1 blocks
below were missing entirely, contrary to `CONTRIBUTING.md` section 7 item 2, which
requires a changelog entry on every merge (integrity finding INT-209).

### Changed
- **INT-202 (Critical):** Resolved the Section G dependency conflict and the **D ↔ G
  cycle** by reclassification. Ten Section G documents declared Sections C, D and F under
  `**Upstream Dependencies**` against a foundational set of A, B and E. Each declaration
  was checked against the document's Appendix B Canonical Ownership Matrix and body text;
  all resolved to ownership routing rather than foundational dependency. They were moved
  to a new `**Consumed From**` block in G02–G09 and recorded as 15 new rows in the
  `DEPENDENCY_MAP.md` section 7A register. Section G's foundational set is unchanged.
  Three Section D declarations were deleted rather than reclassified — `L2-G04`, `L2-G05`
  and `L2-G06` each listed Section D upstream with no Academic Delivery consumption
  anywhere in the document. Verified: no Section G document now declares C, D or F as a
  foundational upstream.
- **INT-201:** Indexed the two L5 Solution Architecture documents that every control
  artifact previously denied. `ARCHITECTURE_INDEX.md` gains an `L5 — Solution
  Architecture` section and its statistics move to 2 L5 documents and 115 total.
  `architecture/L5_Solution_Architecture/README.md`, `README.md` and
  `AI_CONTEXT/REPO_UNDERSTANDING.md` corrected to stop describing L5 as an empty
  placeholder. Both documents remain `Draft` and neither traces to an approved L4.
- **INT-203:** `AI_CONTEXT/SECTION_G/Dependency_Contract.md` asserted that no Section G
  document declares dependency tables. Ten of eleven do. Replaced with the actual
  per-file upstream table read from source, including the INT-202 conflict it had
  concealed.
- **INT-204:** `DEPENDENCY_MAP.md` Section I row corrected from "Sections C, D, F" to
  "Sections A, B, C, D, F". It was the only row in the matrix omitting Section A.
- **INT-205:** Removed the legacy `Owner` field from 44 L2 documents — 41 deletions and
  13 conversions to `Review Authority`, 54 occurrences in total. The INT-003 capability
  ownership migration added `Document Owner` without removing its predecessor, leaving
  44 documents with two owners. All 107 documents retain their canonical capability
  owner. `L2-C08`'s contradictory second `Document Owner` reconciled to its header value.
- **INT-206:** Repaired `L2-C08`'s Consumed Domain Events table, in which every row named
  an event or a publishing domain that does not exist. `QualificationCompleted` →
  `QualificationAwarded` from `L2-C05`. `QualificationCompletionVerified` →
  `ProgrammeCompletionVerified` from `L2-I13`, which defines it in §11.3 with a "Published
  When" clause. `ClearanceCompleted` from the non-existent "Clearance Management" →
  `InstitutionalClearanceCompleted` from `L2-C05`, whose Qualification Completion
  Aggregate owns institutional clearance coordination (`L2-C05` line 4085). The
  `AlumniTransitionApprovalCompleted` row was deleted: it was not a consumed event at all,
  but a duplicate of C08's own published `AlumniTransitionApproved` (§10.4), attributed to
  an invented "Approval Management" domain. Three of four consumed events now resolve to a
  document that genuinely publishes them.
- **INT-207:** `Volume_2_Repository_Standards.md` line 147 requires the Architecture Index
  status legend to mirror its canonical table **exactly**; the Index listed 9 of 13 values
  and stated at line 351 that the subsetting was deliberate. Resolved by completing the
  Index legend to all 13 values, reproduced verbatim in Volume 2's order. Because Volume 2
  already describes Proposed, Approved, Deprecated and Archived as "Not yet in active
  use", the legal-versus-in-use distinction is preserved, and **no amendment to the LOCKED
  Volume 2 was required** — it remains v1.1. Also corrected the Index's claim that the
  v1.0 → v1.1 amendment adopted a 9-value vocabulary; it adopted 13. Verified: the two
  tables are now an exact row-for-row match.
- **INT-208:** Removed the `tools/` entry from `README.md`; the directory no longer
  exists. Corrected the `generated/` description from HTML presentation artifacts to
  review artifacts, and dropped "helper scripts" from the repository philosophy.
- **INT-210:** Repaired the corrupted YAML dependency entry at `L2-C04` line 20 and its
  stale `L2-B05` title reference.
- **INT-211:** `AI_CONTEXT/INDEX.md` no longer lists the deleted
  `AI_CONTEXT/SECTION_H/README.md`.
- **INT-212:** `L2-C08` section 26.8 retitled "Proposed Repository Structure — Not Yet
  Created"; it presented a `diagrams/`/`adr/`/`assets/` tree that does not exist.
- **INT-213:** Removed the duplicate `document-owner` frontmatter key from `L2-B05`.
- Corrected one factual error in the 2026-08-05 review addendum: it dismissed INT-018 on
  the grounds that `architecture_audit_report.md` does not exist. It does; the finding is
  resolved by that file's own Superseded label.

- **Minor changes closed 2026-08-06.** Both items the review left open were resolved:
  - **Section G runtime routings.** Added the two Section J rows (`L2-G07` Security Vendor
    Contracts, `L2-G08` Vendor Contracts) to the `DEPENDENCY_MAP.md` section 7A register.
    Registered the nine Section H notification routings as an **exemption** rather than
    entries, because section 5 of the map already states that cross-cutting Section H
    services do not create a foundational upstream dependency for consuming sections.
  - **Missing bounded contexts.** Investigating the Alumni Transition policy gap showed it
    was not confined to one row. Added `DEPENDENCY_MAP.md` **section 7B — Known Missing
    Bounded Contexts**, recording **Policy Management** as depended upon by `L2-C08` (10
    references) and `L2-C04` (3 references, including an unpublished `AcademicPolicyUpdated`
    event and a `Get Academic Policy` query). No owner was assigned: `L2-C08` line 5037
    states policy ownership is external to it, `L2-A04` is access control and explicitly
    independent of business domains, and `L2-A08` section 18 governs "architectural
    metadata only". Assigning an owner is an architecture decision requiring Lead Architect
    sign-off, not a documentation correction. Consuming documents are marked at the point
    of use with a do-not-implement instruction.
  - **Two further invented bounded contexts** were found in `L2-C08`'s primary upstream
    table while doing this. **Clearance Management** was replaced with `L2-C05`, which
    aggregates clearances from Sections F, G03 and G05 and publishes
    `InstitutionalClearanceCompleted`. **Approval Management** was removed as an upstream
    entirely — C08 publishes `AlumniTransitionApproved` itself. A governing note at C08
    line 188 maps all three names for the fifteen scattered occurrences left in place.
  - **`L2-D03` false attribution.** Section 7.6 stated "Enterprise governance
    responsibilities are provided by Section I." Section I is Examination.
    `DEPENDENCY_MAP.md` lines 277-286 scope it to assessment and results. Replaced with a
    table routing Compliance Validation to `L2-H09`, Audit Governance to `L2-A07`,
    Operational Readiness to `L2-A10`, Architectural Governance and Enterprise Standards to
    the Repository Constitution, and marking Policy Management, Risk Management and
    Decision Governance as unowned.

### Open
- **Policy Management has no owner** — see section 7B of `DEPENDENCY_MAP.md`. Needs a Lead
  Architect decision, not research; the three candidate homes are excluded on the evidence.
- **Risk Management and Decision Governance have no owner.** Named as governance
  capabilities by `L2-D03` section 7.6; no document in the repository claims either.
- **INT-207 (remainder):** `Volume_2_Repository_Standards.md` line 147 requires the
  Architecture Index legend to mirror its 13-value table exactly, which the Index
  knowingly does not. Volume 2 is LOCKED; amending it needs Lead Architect approval.

---

## [1.5.1] - 2026-08-06

### Changed
- INT-003 follow-up: aligned the `ARCHITECTURE_INDEX.md` owner column with the capability
  owners established in `TEAM_ASSIGNMENTS.md` (`0613700`).

---

## [1.5.0] - 2026-08-05

Remediation round 2 against `generated/reviews/integrity-2026-08-03-repository-wide-review.md`
and `generated/reviews/review-repository-integrity-2026-08-05.md`.

### Added
- `TEAM_ASSIGNMENTS.md` Canonical Capability Owners table: every L2 document now has a
  capability-area owner with a named Accountable Architect, so ownership survives
  reassignment of people (INT-003, `780eb65`).
- `DEPENDENCY_MAP.md` section 7A Dependency Classes and the Runtime Contract and Event
  Consumption Register, separating foundational dependencies from runtime ones
  (INT-035, `f578dca`).
- `architecture/L5_Solution_Architecture/L5-01_Infrastructure_Persistence_Baseline.md`,
  extracted from `L2-B03` Part 14 (INT-008, `356bf5b`).
- Integrity reviews brought under version control so findings stop being re-raised
  (`631690b`).

### Changed
- INT-008: canonicalized the result publication event as `ExaminationResultPublished`
  (`9bdde91`), then removed `FinalResultPublished` as a sixth variant rather than a
  distinct event (`6914ec8`).
- INT-008: migrated the physical infrastructure architecture from `L2-B03` to `L2-G10`
  across six steps (`5d5754c`, `1c0adda`, `7ea1e58`, `356bf5b`).
- INT-004/005/017: separated the infrastructure, scholarship eligibility and payroll
  boundaries (`d10593f`).
- INT-009/010/011: removed three reversed dependency declarations in A10, D07 and F14
  (`d84f0cf`).
- INT-001/004/035: ownership reconciliation in C08, I13 and J06 (`f578dca`).
- Prompts 0/1/2 re-run across all ten sections, replacing existing AI context content
  (`f6ce1ff`).

---

## [1.4.0] - 2026-08-04

Remediation round 1 against `generated/reviews/integrity-2026-08-03-repository-wide-review.md`.

### Changed
- INT-005: L1 closing metadata version corrected to 2.1 to match its header.
- INT-019: `ARCHITECTURE_INDEX.md` core-document count corrected to 6 and total to 113.
- INT-021: Normalised L2 H1 separators and corrected section-name drift in D01, E06,
  E07, F00, and F01 to match `ARCHITECTURE_INDEX.md` verbatim.
- INT-022: Corrected D01 self-references to its real filename.
- INT-024: Prompts 0/1/2 now use the prefixed AI-context basename; Section J file
  layout inverted to match Sections A-I.
- INT-025: Section J README trust-source labels replaced with exact A03-A07 names.
- INT-003: **L1 section 25 replaced.** The obsolete two-team ownership model was removed
  from the locked L1 document and from L0, and replaced with Section-based ownership
  sourced from `TEAM_ASSIGNMENTS.md`. L1 is a LOCKED document; this amendment is
  recorded here and in the L1 header.

- INT-007: **L1 StudentContext corrected.** Section 3.5 gated a new Context on a
  "Completed" state that exists in neither L1's own state machine nor C02's canonical
  list. The gate is now the five terminal states, and Suspended (non-terminal,
  reversible) was added to L1's machine to match C02 section 11.10.1. A terminal-state
  table was added so the enumeration is unambiguous.
- INT-006: **L1 Tenant lifecycle expanded.** L1 defined only Requested/Provisioning/
  Active/Failed. L2-A01 adds Maintenance, Suspended, Upgrade and Archived. L1 now
  carries the full set with a terminal-state table. A01's own contradiction was fixed
  first: its diagram showed Upgrade while its state table (5.5) and transition table
  (5.8) omitted it, and the Provisioning Failed retry transition was undeclared.

- INT-002: **L0 content boundary restored.** Volume 1 requires L0 to be
  business-facing and technology-independent. L0 section 3 prescribed a concrete
  stack (Rust, Axum, Tokio, PostgreSQL, SQLx, PgBouncer, Redis, React, Vite,
  TypeScript, Zustand, Axios, MinIO, JWT/Argon2id). That content moved verbatim to
  the new `architecture/L5_Solution_Architecture/L5-00_Technology_Stack_Baseline.md`
  (Draft, explicitly not yet re-reviewed as L5). L0 section 3 now states the modular
  platform decision and its standing risk without naming technology. The event-bus
  passage in section 13 was restated technology-independently, and section 25's
  stray H1 asserting L1 content inside L0 was demoted and marked historical.

- INT-001: **Architecture hierarchy made single-valued.** L0 Section 23 gave the levels
  different meanings from Section 0, calling L4 "Individual Module Workflows" and L5
  "Role / Position-Based Workflows". Section 0 confirmed canonical by the repository
  owner; Section 23 now reproduces it verbatim. L2-C02 carried a separate non-canonical
  five-level scheme ending at "L4 Technical Design & Implementation" and was corrected
  to the canonical seven levels.
- INT-004: **L1 dependency order replaced.** L1 described a single serial chain ending
  with Communication and then the Workflow Engine. The Workflow Engine is L2-A05, inside
  foundational Section A, so it cannot be terminal; the chain also placed Examination
  before Finance, reversing the canonical matrix and creating a false cycle. Per the
  owner's decision, DEPENDENCY_MAP.md governs. L1 now carries its directed matrix and a
  locked rule that Section A is foundational.
- INT-035: **Dependency classes added to DEPENDENCY_MAP.md** (new Section 7A). The register
  recorded only foundational dependencies, so runtime integrations were invisible.
  Foundational, runtime-contract, event-consumption and read-model classes are now defined,
  with a register recording F14 to Section J, Section C to Section I, and the J07 stock
  movement events. Only foundational dependencies constrain build order.
- INT-009, INT-010, INT-011, INT-014: ownership boundaries reconciled in I13, C08 and J06.
  Fixing J06's stock ownership removed the InventoryStockIssued/InventoryStockReturned
  names entirely rather than renaming them.

### Removed
- INT-003: Deleted `generated/html/` (untracked, gitignored). To be regenerated with
  `tools/convert_all.js` after remediation completes.

---

## [1.3.0] - 2026-07-29

### Added
- Added placeholder infrastructure for L3 through L6 architecture folders.
- Added Section J AI context placeholders for Enterprise Resource Management.
- Added `governance/policies/Architecture_Level_Validation_Standards.md`.

### Changed
- Standardized future-level architecture folder names to the approved L0-L6 hierarchy.
- Updated templates, prompts, repository metadata, and validation references for Phase 2 governance migration.
- Updated Section J repository references to Enterprise Resource Management.

---

## [1.2.0] - 2026-07-08

### Added
- Created `DEPENDENCY_MAP.md` mapping downstream/upstream constraints, level structures, L2 section flows, and LLM context segmentation strategy.

### Changed
- Updated `README.md` to include a Repository Governance registry summary.
- Updated `CONTRIBUTING.md` to add Repository Maintenance post-merge requirements.
- Updated `ARCHITECTURE_INDEX.md` to index the new `DEPENDENCY_MAP.md` file.

---

## [1.1.0] - 2026-07-08

### Added
- Created `templates/` folder with templates for Levels 2, 3, 4, 5, and 6.
- Added `CONTRIBUTING.md` defining Git, PR, and review guidelines.
- Added `ARCHITECTURE_INDEX.md` mapping domain ownership and lifecycle status.
- Added main `README.md` navigation hub.
- Added `.gitignore`.

### Changed
- Reorganized the repository structure to align with the target architecture blueprint.
- Relocated baseline documents, Level 1 architecture, Level 2 matrix, and presentation files into their corresponding hierarchical folders:
  - `docs/baseline/`
  - `docs/collaboration/`
  - `docs/level-1/`
  - `docs/level-2/`
  - `presentation/`

---

## [1.0.0] - 2026-07-08

### Added
- Initial repository setup containing baseline workflows, collaboration rules, Level 1 locks, Level 2 domain planning matrix, and presentation pages.
