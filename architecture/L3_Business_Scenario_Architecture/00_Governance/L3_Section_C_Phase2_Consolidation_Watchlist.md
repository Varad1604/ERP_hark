# L3 Section C — Phase 2 Consolidation Watchlist

**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section C — Student Lifecycle, candidates `CAN-L3-C-001` … `CAN-L3-C-013`
**Status:** Active — Phase 2 input
**Created:** 2026-08-09
**Source:** `generated/reviews/l3-phase1-section-c-verification-2026-08-09.md`

---

## 1. Purpose

This document holds the candidate relationships and repository conflicts identified during the independent Section C verification of 2026-08-09, and records for each one whether it is settled or still requires a Phase 2 decision.

It exists as a separate document because `L3_Phase1_Discovery_Rules.md` section 3 requires each Phase 1 inventory file to contain candidate scenario rows only, and section 16 requires the inventory to avoid long explanations. Relationship and conflict analysis is review output, not discovery output, and belongs here.

**Nothing in this document has been consolidated.** Every candidate named below remains in the inventory under its own ID. `L3_Phase1_Discovery_Rules.md` section 17 places duplicate identification, merging and splitting in the post-Phase-1 review.

This document is not a governance standard. It adds and amends no Phase 1 rule. It resolves no architectural conflict — section 4 records conflicts so they are visible, in keeping with `L3_Phase1_Discovery_Rules.md` section 14, which forbids silently resolving repository conflicts during Phase 1.

It follows the structure established by `L3_Section_A_Phase2_Consolidation_Watchlist.md`.

---

## 2. Status Values

| Status | Meaning | Phase 2 action |
| ------ | ------- | -------------- |
| `CLOSED` | Repository evidence establishes the candidates are distinct. No consolidation question remains. | None. Do not re-raise. |
| `DETERMINED` | Repository evidence is one-sided. The decision is recorded; only its execution is deferred. | Execute the recorded decision. |
| `OPEN` | Repository evidence supports both readings. A judgement is genuinely required. | Decide. |
| `BLOCKED` | The item cannot be assessed until an ownership or integrity question is ruled on. | Wait for the Enterprise Architecture Board. |

Resolution counts — candidate relationships: 2 `CLOSED`, 2 `OPEN`. Repository conflicts: 2 `DETERMINED`, 5 `OPEN`, 1 `BLOCKED`, 1 conflict-dependent wording constraint. Total 13.

---

## 3. Candidate Relationships

### 3.1 Closed — no consolidation question remains

| Cluster | Candidates | Resolution | Evidence |
| ------- | ---------- | ---------- | -------- |
| C-C03 Student exit and persona archival | 019, 040 | Retain separately. | Distinct terminal outcomes. `CAN-L3-C-040` closes the academic relationship (`Student Exit Completed`, `L2-C05` §9.2); `CAN-L3-C-019` archives the persona once every Student Context is terminal (`L2-C02` §11.10.3 exit-state reconciliation). |
| C-C04 Guardian relationship and emergency contact | 048, 049, 053 | Retain separately. | `L2-C06` §7.2 defines GW-08 Emergency Contact Management as its own catalogue workflow, and §6.2 defines `Emergency Contact` as an aggregate-managed entity distinct from Guardian Assignment. |

### 3.2 Open — Phase 2 decision required

| Cluster | Candidates | Decision needed | Position |
| ------- | ---------- | --------------- | -------- |
| C-C01 Student placement change | 008, 011 | Is an academic assignment change distinguishable from an intra-Institute reassignment, or are they one outcome reached by two routes? | `L2-C02` defines WF-C02-04 Academic Assignment and WF-C02-08 Student Transfer as separate catalogue workflows and both mutate programme, batch and section. Provisional boundary: 008 is initial and corrective assignment within an existing placement; 011 is authorized relocation to a different organisational placement. Leaning retain both. Do not delete either — the sources define them separately. |
| C-C02 Programme change ownership | 011, 030 | Does an approved change of programme belong to Student Lifecycle transfer or to Academic Progression migration? | **This is an L2-level overlap, not a discovery defect.** `L2-C02` WF-C02-08 §1 lists **Programme** among transferable dimensions; `L2-C04` APW-08 owns Programme Migration with credit-preservation and curriculum-mapping semantics absent from WF-C02-08. Both candidates are repository-supported. Escalate the boundary question to the owning documents; do not resolve it inside the inventory. |

---

## 4. Repository Conflicts Affecting Section C Discovery

These are architecture questions surfaced by discovery. None was resolved during Phase 1.

### 4.1 Determined — applied to candidate wording, conflict still requires an L2 correction

| # | Conflict | Sources | Applied position |
| - | -------- | ------- | ---------------- |
| CF-03 | `L2-C01` §6.13 states no terminal Admission Record shall ever be reopened, and §7.6 rule 6 states readmission creates a **new** Admission Record. `L2-C01` §10.3 defines a `Readmitted` state meaning "Admission restored under institutional policy". | `L2-C01` §6.13, §7.6, §6.9.1 vs §10.3 | §6.13 applied. `CAN-L3-C-005` now states that readmission creates a new Admission Record linked to the same canonical Person, with the prior terminal record preserved and closed. The §10.3 `Readmitted` state should be corrected at source. |
| CF-07 | ABC / APAAR integration partner is ambiguous. `L1` §4 states "APAAR belongs to Identity" (Section A); `ARCHITECTURE_INDEX:221` and `L2-C04` §18.10 place "APAAR & DigiLocker" / "APAAR & ABC Integration" in `L2-H01` (Section H). | `L1` §4 vs `ARCHITECTURE_INDEX:221`, `L2-C04` §18.10 | APAAR-as-identity (A) and ABC-credit-synchronisation (H) are different capabilities. `H` applied to `CAN-L3-C-037` and `CAN-L3-C-032`. Ledger ownership remains with Academic Progression per `L1` §4. |

### 4.2 Open — Phase 2 or Enterprise Architecture Board decision required

| # | Conflict | Sources | Impact on discovery |
| - | -------- | ------- | ------------------- |
| CF-01 | `Rejected` is used as an admission state and published as `AdmissionRejected`, but appears in **no** canonical state list. | `L2-C01` WF-C01-03 §2.7 EX-01 and §3.2 vs §6.8, §6.9, §6.9.1, §6.13, §10.3 | **Conflict-dependent wording.** No standalone rejection candidate was created against an undefined state. The negative decision is carried inside `CAN-L3-C-007`. If `Rejected` is ratified as canonical, reassess whether it warrants its own candidate. |
| CF-02 | Two incompatible admission state vocabularies. §6 uses *Admission Initiated / Documents Submitted / Verification Completed / Admission Approved / Financial Clearance / Admission Confirmed / Student Created / Admission Completed / Expired*; §10.3 uses *Draft / Submitted / UnderVerification / Verified / ApprovalPending / Approved / FinancialPending / FinancialCleared / Confirmed / AdmissionStudentCreated*, adds `Readmitted` and drops `Expired`. | `L2-C01` §6.4–§6.9 vs §10.3, §10.7 | Trigger and outcome wording for C01 candidates cannot be stated against a single authoritative vocabulary. All C01 candidates quote §6 (Domain Lifecycle) as the working vocabulary. |
| CF-04 | Guardian capability is defined in two documents. `L2-C02` WF-C02-13.3 claims guardian registration, updates, **replacement**, activation/deactivation, **Primary Guardian designation** and emergency contacts. `L2-C06` §5 declares the same capabilities "exclusively owned" and states they "shall not be duplicated by other domains". | `L2-C02` WF-C02-13 vs `L2-C06` §2, §5 | All guardian candidates (048–058) sit under C06, which is the correct read of the exclusivity language — but that is a reconciliation of a live conflict and is recorded here so it is not silent. If the Board assigns guardian management to C02, ownership of eleven candidates moves. |
| CF-05 | Alumni Transition ownership is stated three ways. `L2-C04` §1.5 assigns "Alumni Transition → L2-C05"; `L2-C05` §5 lists "Alumni Relationship Management" out of scope while §11.10 contains an Alumni Transition workflow; `L2-C08` §5.2 claims exclusive ownership of the transition lifecycle. | `L2-C04` §1.5, `L2-C05` §5/§11.10, `L2-C08` §5.2 | Affects the Primary Owner of `CAN-L3-C-072`. The dedicated `L2-C08` bounded context is the defensible reading and was applied. The `L2-C04` §1.5 misattribution should be corrected at source. |
| CF-06 | `Student Created → Revoked (Policy Controlled)` is an allowed transition, but WF-C01-08 §1.4 explicitly excludes "Academic record management" and defines no consequence for the already-created Student Context. | `L2-C01` §6.9.1 vs WF-C01-08 §1.4 | **Conflict-dependent wording.** `CAN-L3-C-004` names the hand-off to Student Lifecycle without inventing the unwinding process. The unwinding behaviour needs an owner before this candidate can be drafted as a full L3 document. |
| CF-09 | The two C01 transition matrices disagree. §6.9 permits `Admission Approved → Financial Clearance, Revoked` and `Financial Clearance → Admission Confirmed` only; §6.9.1 permits `Admission Approved → Cancelled` and `Financial Clearance → Cancelled`, and moves `Revoked` to `Admission Confirmed`. | `L2-C01` §6.9 vs §6.9.1 | Low impact. `CAN-L3-C-002` and `CAN-L3-C-004` remain valid under either matrix. Record for L2 correction. |

### 4.3 Blocked — awaiting owner assignment

| # | Conflict | Sources | Effect |
| - | -------- | ------- | ------ |
| CF-08 | **Policy Management has no owner.** `L2-C08` (10 references) and `L2-C04` (3 references) depend on institutional policy definitions and their change events — `AlumniTransitionPolicyUpdated`, `AcademicPolicyUpdated`, and a `Get Academic Policy` query contract. Neither event has a publisher anywhere in the repository. `L2-C04` §2.6 records the gap in-document as "Policy Management — unresolved, no owner". | `DEPENDENCY_MAP` §7 (status **Open**); `L2-C04` §2.6; `L2-C08` §5 policy references | **Two candidates cannot be discovered:** an academic-progression policy lifecycle scenario and an alumni-transition policy lifecycle scenario — the structural analogues of `CAN-L3-C-063`, which exists only because `DEPENDENCY_MAP` line 429 confirms Eligibility Policy Management is a workflow within `L2-C07`. Their absence from the inventory is **correct**, not an omission. `DEPENDENCY_MAP` §7 states that assigning an owner "is an architecture decision… and requires Lead Architect sign-off". Do not create these candidates before that ruling. |

**Update, 2026-08-17 (numbering migration pass).** `DEPENDENCY_MAP.md` line 409/434 now records **Resolved 2026-08-15 (RC-03): institutional policy is owned by `L2-B01`**, with `AcademicPolicyUpdated` published by `L2-C04` and `AlumniTransitionPolicyUpdated` published by `L2-C08`. `CF-08` is therefore no longer blocking as an ownership question. The row above is left exactly as originally written — it accurately describes the state on 2026-08-09 when it was raised. This numbering migration does not add the two candidates the row describes; that is a separate Phase 1 discovery action for the Section C owner to take up now that ownership is resolved.

---

## 5. Boundary Checks That Passed

Recorded so they are not re-derived.

* **Person / Identity → Section A.** `CAN-L3-C-016`, `CAN-L3-C-017` and `CAN-L3-C-051` all state identity ownership remains with A. Correct against `L2-C06` §5 Rule 1 and `L2-A02`.
* **Institution / Structure → Section B.** `CAN-L3-C-012`, `018`, `030`, `059`, `074` list B as participant only; none claims institutional hierarchy.
* **Course Offering / Delivery → Section D.** `CAN-L3-C-020`, `019`, `020`, `022` list D as participant; no candidate claims offering ownership. Correct against `L2-C03` §4.5.
* **Financial transactions → Section F.** `CAN-L3-C-071` hands off "without transferring scholarship-eligibility ownership"; `CAN-L3-C-057` declines financial-transaction ownership. Correct against `L2-C07` §2.2 and `L2-C06` §5 Rule 4.
* **Official Examination results → Section I.** `CAN-L3-C-024`, `024`, `025`, `026`, `027`, `028`, `035` consume results with C as owner and I as participant. Correct against `L2-C04` §1.5 and `DEPENDENCY_MAP` lines 129–130, 339.
* **Institutional services → Section H.** `CAN-L3-C-018` retains only the request context and hands fulfilment to Document Services (`L2-C02` WF-C02-11.3). `CAN-L3-C-072` hands the alumni engagement context to the Alumni lifecycle without transferring the historical Student Context (`L2-C08` §2.2, §5.3).
* **Campus Operations → Section G.** `CAN-L3-C-058` makes pickup authorization available to the enforcing capability rather than claiming enforcement.

No candidate introduced duplicate canonical ownership.

---

## 6. Effect on Section C Candidate Count

| | Count |
| - | ----: |
| Candidates at verification | 71 |
| Added (`072`, `073`, `074`) | +3 |
| Merged | 0 |
| Removed | 0 |
| **Candidates after application** | **74** |

Neither open cluster in section 3.2 proposes a merge. If Phase 2 decides C-C01 and C-C02 in favour of consolidation, Section C would resolve to 72 candidates. That figure is indicative and is not a Phase 1 commitment.

### 6.1 Application execution record — 2026-08-14

**This table described an application that had not in fact been made.** The all-scope Phase 1 audit of 2026-08-14 found `L3_Phase1_Inventory_Chetan.md` holding 71 rows with maximum ID `CAN-L3-C-074`, no `C-072`/`C-073`/`C-074`, and `C-011` still carrying its pre-split title. `CF-01` below therefore referenced a mitigation that did not exist in the repository, and section 5 cited a boundary check against an absent candidate.

The recorded application was executed on 2026-08-14 under user authorization:

| Change | Applied to | Basis |
| --- | --- | --- |
| Added `CAN-L3-C-007` Admission Document Verification and Approval Decision | `C01` table | `L2-C01` §6.4 Primary Lifecycle; §6.6 states *Documents Submitted*, *Verification Completed*, *Admission Approved*; §7.3 catalogue `WF-C01-02`, `WF-C01-03` |
| Added `CAN-L3-C-026` Registration Hold Placement and Release | `C03` table | `L2-C03` §5.3 `Registration Hold — Full`; child entity `RegistrationHold`; `OverrideRegistrationHold()` |
| Added `CAN-L3-C-013` Inter-Institute Student Transfer with Student Context Re-establishment | `C02` table | `L2-C02` §11.10.3: "the originating context becomes terminal; a new context opens at the receiving Institute" |
| `CAN-L3-C-012` retitled *Intra-Institute Student Academic Reassignment* and restricted to same-Institute change | `C02` table | `L2-C02` WF-C02-08 §1; §11.5 `Transferred → Active` |

Trigger and Final Outcome wording for the three added candidates was authored from the cited L2 sources at application time; the 2026-08-09 verification recorded only ID, Title, Owner, Domains, Type and Priority. `C-072` carries the negative admission decision as a governed disposition **without** asserting a `Rejected` state, preserving conflict `CF-01`.

Section C now holds **74** rows, matching this table. Audit reference: `generated/reviews/l3-phase1-audit-all-2026-08-14.md` FIND-001, FIND-006, MC-01…MC-03.
