# L3 Section A — Phase 2 Consolidation Watchlist

**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section A — Platform Foundation, active candidate IDs through `CAN-L3-A-131`; absorbed IDs remain traceable below
**Status:** Resolved — all 18 relationship clusters have a recorded and applied or closed outcome
**Created:** 2026-08-08
**Resolved:** 2026-08-08
**Applied:** 2026-08-13 — A-C01, A-C07 and A-C10; see `generated/reviews/L3_Phase1_Audit_Dhruv_Correction_Application_2026-08-13.md`
**Approved and Applied:** 2026-08-13 — A-C05, A-C12, A-C14, A-C15, A-C17 and INT-301/INT-302; see `generated/reviews/L3_Phase1_Audit_Dhruv_Approved_Decisions_Application_2026-08-13.md`
**Supersedes:** section 3.3 of `../01_Phase1_Discovery/L3_Phase1_Inventory_Dhruv.md` and section 7 of `L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md`

---

## 1. Purpose

This document holds the candidate relationships identified during the independent Section A verification of 2026-08-08 and records, for each one, whether it is settled or still requires a Phase 2 decision.

It exists as a separate document because `L3_Phase1_Discovery_Rules.md` section 3 requires each Phase 1 inventory file to contain candidate scenario rows only, and section 16 requires the inventory to avoid long explanations. Relationship analysis is review output, not discovery output, and belongs here.

The three evidence-determined clusters in section 4 were consolidated on 2026-08-13 under explicit user authorization. The same authority then approved the six decisions recorded in the Dhruv correction report and subsequently approved DEC-DHR-07 for newly unblocked A-C15. Temporary retained IDs were preserved, absorbed IDs were not reused, and split successors received new temporary IDs.

This document is not a governance standard. It adds and amends no Phase 1 rule.

---

## 2. Status Values

| Status | Meaning | Phase 2 action |
| ------ | ------- | -------------- |
| `CLOSED` | Repository evidence establishes the candidates are distinct. No consolidation question remains. | None. Do not re-raise. |
| `DETERMINED` | Repository evidence is one-sided. The consolidation is decided; only its execution is deferred. | Execute the recorded merge. |
| `APPLIED` | An evidence-determined consolidation has been executed in the Phase 1 inventory with retained/absorbed IDs recorded. | Preserve the mapping; do not re-add the absorbed fragment. |
| `OPEN` | Repository evidence supports both readings. A judgement is genuinely required. | Decide. |
| `BLOCKED` | The cluster cannot be assessed until an integrity finding is ruled on. | Wait for the Enterprise Architecture Board. |

Resolution counts: 12 `CLOSED`, 6 `APPLIED`. Total 18. No `OPEN` or `BLOCKED` cluster remains.

All relationship decisions are complete. The register remains as retained/absorbed/split traceability evidence for Phase 2.

---

## 3. Resolved — No Consolidation Question Remains

These ten clusters were raised during verification and are closed against repository evidence. They are retained rather than deleted so the same questions are not re-derived, consistent with the repository practice established by commit `631690b`.

| Cluster | Candidates | Resolution | Evidence |
| ------- | ---------- | ---------- | -------- |
| A-C02 Person consolidation and separation | 006, 007 | Retain separately. | `L2-A02` specifies Merge and Split as independent workflows with distinct validation and approval requirements. |
| A-C03 Bounded authorization elevation | 011, 012, 013, 014 | Retain separately. | `L2-A04` section 9 declares Delegation, Temporary Access Grant and Break-Glass Session as three distinct canonical aggregates with different lifecycles and approval models. 011 is the permanent assignment the other three modify. |
| A-C04 Workflow definition governance | 015, 018 | Retain separately. | `L2-A05` sections 36–37 give versioning and migration their own rules and version-binding constraint. First activation and subsequent version governance are distinct outcomes. |
| A-C06 Event delivery and recovery | 020, 021, 022 | Retain separately. | Each has its own aggregate or service in `L2-A06` section 20. Replay Session carries an authorization gate the delivery and retry paths do not. |
| A-C08 Audit evidence storage governance | 028, 029, 030 | Retain all three. | See section 3.1 — **this reverses the recommendation recorded on 2026-08-08.** |
| A-C09 Module lifecycle progression | 031, 032, 035 | Retain separately. | Distinct governance gates. 035 carries dependency-impact assessment absent from the others. |
| A-C11 Dataset registration and publication | 037, 038, 042 | Retain all three. | `L2-A09` MD-15 requires a prior publication as a precondition. Push distribution with consumer acknowledgement is a materially different outcome from publication. |
| A-C13 Dataset delivery | 041, 042 | Retain 041. | See section 3.2 — **this reverses the "needs further source review" status recorded on 2026-08-08.** |
| A-C16 Readiness finding governance | 049, 050 | Retain separately. | The accountable actor changes between planning and verification, and that handoff is the governance-execution separation `L2-A10` section 5.3 exists to enforce. |
| A-C18 Readiness assessment scope | 047, 051 | Retain separately. | `L2-A10` section 24.4 (Certification Eligibility Evaluation) and section 27.1 (Certificate Issuance) are separate workflows. 047 was re-worded on 2026-08-08 to terminate at the eligibility decision, so the overlap no longer exists. Boundary corrected at source; no split required. |

### 3.1 A-C08 — correction

The 2026-08-08 verification recommended "Merge 029 into 028" on the grounds that 029's trigger is 028's outcome. **That recommendation was wrong and is withdrawn.**

`L2-A07:1391-1393` defines three separate workflows, not one:

```text
| A07-W11 | Retention Governance | Applies policy-driven evidence lifecycle management |
| A07-W12 | Evidence Archival    | Moves retained evidence into governed archival storage |
| A07-W13 | Evidence Restoration | Restores archived evidence access under authorization |
```

`L2-A07:1408-1409` then places them in **two different workflow categories** — W10 and W11 under `Governance`, W12 and W13 under `Lifecycle Management`. The document separates the retention decision from the storage transition deliberately.

Each of the three produces a durable governed state change rather than a projection, and W13 carries an authorization gate the other two do not. A sequential precondition is not by itself grounds to merge; every scenario has preconditions.

**Resolution:** 028, 029 and 030 map to W11, W12 and W13 respectively and are retained as three candidates.

### 3.2 A-C13 — correction

The 2026-08-08 verification left this cluster as "needs further source review", describing 041 as "supported by a single short workflow with no aggregate and no lifecycle". The source review has now been done and 041 stands.

`L2-A09` MD-14 Export Dataset (section 20.14) has its own service (section 18.10 Import and Export Service), its own authorization gate ("Export policy permits access"), its own canonical event `DatasetExported`, its own failure outcome ("the export is denied and no dataset leaves the governed boundary"), and its own recorded export history.

It is also triggered differently from 042: MD-14 is a consumer-initiated pull under export policy; MD-15 Synchronize Dataset is a publication-initiated push to subscribers. Different trigger, different actor, different governed outcome.

**Resolution:** 041 is retained. It is not a consolidation candidate.

---

## 4. Applied — Evidence-Determined Consolidations

Repository evidence in these three clusters is one-sided. The consolidations were applied on 2026-08-13 without renumbering retained candidates.

| Cluster | Candidates | Determination | Evidence |
| ------- | ---------- | ------------- | -------- |
| A-C01 Tenant reversible restriction | 002, 003 | **APPLIED — 003 absorbed into 002.** | `L2-A01` defines no tenant restoration workflow. The only support for 003 is the section 5.8 transition-table row `Suspended → Active` at `L2-A01:1734`. A permitted transition is not a discovered business scenario. |
| A-C07 Audit investigation family | 024, 025, 026 | **APPLIED — 025 and 026 absorbed into 024.** | See section 4.1. |
| A-C10 Module suspension and restoration | 033, 034 | **APPLIED — 034 absorbed into 033.** | `L2-A08:3164` lists `Suspend Module` as a registry capability. Section 15 defines fifteen registry workflows and **no reactivation workflow**. The only support for 034 is the transition-table row `Suspended → Active` at `L2-A08:1305`. |

A-C01 and A-C10 are the same defect in two sections: a candidate resting on a state-machine edge with no workflow behind it. The applied general rule is: **a permitted lifecycle transition with no source workflow is not an independent L3 scenario**. In Section A it affected 003 and 034 only; 029 and 045 were previously grouped with them in error and are addressed in sections 3.1 and 5.

### 4.1 A-C07 — determination

`L2-A07:1407` places all three workflows in a **single** category:

```text
| Investigation | A07-W07, A07-W08, A07-W09 |
```

`L2-A07:1899-1901` shows both W07 and W08 feeding W09, which is the terminal consumer:

```text
| A07-W07 Evidence Packaging | A07-W04        | A07-W09              |
| A07-W08 Timeline Generation| A07-W05, W06   | A07-W09              |
| A07-W09 Audit Investigation| A07-W07, W08   | Governance Activities |
```

Neither input produces a durable governed artifact. `L2-A07:867` states the Timeline Projection Read Model "exists solely as a derived read model"; `:1286` states timeline read models "own no lifecycle"; `:831` describes the Evidence Package as "a derived, investigation-scoped artifact assembled from references to immutable evidence". `L2-A07:1735` has W09 step 5 retrieve the timeline projection directly.

This is the discriminator that separates A-C07 from A-C08: the investigation family is unified into one category and produces derived projections; the retention family is deliberately split across two categories and produces durable state changes.

**Determination:** 025 and 026 are delivery mechanisms of the investigation outcome recorded in 024, not independent outcomes. Merge in Phase 2.

---

## 5. Approved Phase 2 Judgements — Applied

| Cluster | Candidates | Approved resolution | Application |
| ------- | ---------- | --------------- | -------- |
| A-C05 Workflow execution and human task | 016, 064 | **CLOSED — retain separately.** Human-task resolution has its own aggregate, SLA and escalation outcome. | Titles and boundaries retained. |
| A-C12 Dataset version creation | 039, 046 | **APPLIED — 046 absorbed into 039.** Historical restoration is a governed entry path to the same successor-version outcome. | A039 trigger/outcome expanded; A046 retired without ID reuse. |
| A-C14 Dataset lifecycle closure | 044, 045 | **CLOSED — retain separately.** Retirement ends operational use; archival creates separately governed long-term storage and searchability. | Titles and boundaries retained. |

---

## 6. Formerly Blocked — Architecture Decisions Applied

`INT-301` and `INT-302` were approved and applied on 2026-08-13. The L1/L2 sources now define single ownership and the two-capability readiness boundary, so neither cluster remains blocked.

| Cluster | Candidates | Approved resolution | Application |
| ------- | ---------- | -------- | -------------------- |
| A-C15 Readiness assessment cycle | 047, 053, 054, 055 | **APPLIED under DEC-DHR-07.** Retain 053's reassessment-required/no-reassessment-required decision; absorb 054's successor-assessment creation into 047's governed assessment cycle; retain 055's post-assessment/remediation/certification governance closure. | A047 now includes new/successor assessment initialization and predecessor linkage; A054 retired without ID reuse. |
| A-C17 Certificate lifecycle | 051, 052, 067, 068 | **APPLIED — split A052 three ways.** Retain A052 for suspension/restoration; add A067 supersession and A068 revocation/terminal historical closure. | Certificate lifecycle split applied after INT-302 resolution. |

The former INT-301 owner markers were removed. Authentication candidates use A03 ownership; authorization candidates use A04 ownership. Candidates 056–058 were already correctly owned by A04 and required no wording change.

---

## 7. Phase 2 Items Outside This Watchlist

These were recorded in the audit application report as watchlist entries but are not candidate-relationship questions. They are restated here so section 7 of that report can be retired without loss.

* **Variable Domains notation — resolved.** `L3_Phase1_Discovery_Rules.md` section 11 now governs `VAR:<role>`. Dhruv's inventory uses no `X` wildcard and no free-text “requesting domain” placeholder.
* **Targeted peer comparison — completed 2026-08-13.** The available C, F/G/H and I registers now contain 879 candidate rows and were checked against every active A/J candidate. No exact normalized-title collision, material semantic duplicate, or competing canonical-owner claim was found. **The deferred B/D/E comparison was completed on 2026-08-14** against Pallav's 164 populated rows, covering all 1,215 repository candidates. No duplicate outcome and no competing canonical-owner claim was found. One ownership conflict was identified and is recorded as `RC-01` in section 9 below: `CAN-L3-A-080` versus `CAN-L3-E-103`. Evidence: `generated/reviews/l3-phase1-audit-all-2026-08-14.md` §10.
* **Dhruv's additional assigned work — completed 2026-08-13.** Section J, governed variable-participant notation, targeted cross-domain comparison, governance traceability and the procurement-to-inventory pilot are recorded and verified. See `generated/reviews/L3_Phase1_Dhruv_Revised_Submission_Verification_2026-08-13.md`.

---

## 8. Effect on Section A Candidate Count

The first three applied clusters absorbed A003, A025, A026 and A034 and added A065/A066, moving Section A from 64 to 62 rows. A-C12 then absorbed A046, A-C17 added A067/A068, and DEC-DHR-07 absorbed A054 into A047. Section A therefore now has **62 active candidate rows**.

Retained IDs were not renumbered. The gaps preserve the audit trail, and absorbed IDs must not be reused.

---

## 9. Repository Conflicts Affecting Section A Discovery

Added 2026-08-14 by the all-scope Phase 1 audit. Recorded so the conflict is visible at the point of use. **Not resolved here** — `L3_Phase1_Discovery_Rules.md` section 14 forbids silently resolving repository conflicts during Phase 1.

### RC-01 — Delegation ownership

| | |
| --- | --- |
| **Status** | `RESOLVED 2026-08-15` — amendment approved and applied. See resolution note below. |
| **Affected candidates** | `CAN-L3-A-080` (Responsibility Delegation to Automatic Authority Reversion, owner Authorization & Policy); `CAN-L3-E-103` (Delegated Authority and Additional Charge Assignment, owner E02); `CAN-L3-G-793` (Housekeeping Delegation to Temporary Operational Authority, owner Housekeeping Operations) — **G-793 was omitted from this list until 2026-08-15** |
| **Position A — `L2-A04`** | Line 114: "Canonical boundary approved 2026-08-13 under INT-301. L2-A04 **exclusively owns** Role, Role Assignment, **Delegation** and authorization-policy lifecycles." §10.5: "Delegation represents temporary transfer of business responsibility from one authorized user to another." |
| **Position B — `L2-E02`** | Line 359: `Assignment Delegation \| Canonical`. Line 975: `Delegation \| L2-E02 \| Operational delegation`. Line 2644: command `DelegateAuthority`. Line 2662: event `PositionAssignmentDelegated`. |
| **Authority analysis** | Unresolved by hierarchy. Both are locked L2 documents at the same authority tier. A04's claim carries a dated approval reference, which is suggestive but does not by itself override E02's canonical declaration together with its own command and event ownership. A reading exists in which A04 owns authorization-layer delegation and E02 owns organisational-assignment delegation — **but no repository source states that split**, and A04's "exclusively owns" wording forecloses it. |
| **Phase 2 blocked?** | **Yes**, for these two candidates only. |
| **Escalate to** | The authority that approved INT-301. |
| **Required decision** | Either amend `L2-A04` line 114 to scope its exclusivity to authorization-layer delegation, or amend `L2-E02` to consume A04 delegation rather than own it. Then set the owner for both candidates. |
| **Evidence** | `generated/reviews/l3-phase1-audit-all-2026-08-14.md` FIND-002, RC-01. Mirrored in `L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md` §4. |

Neither inventory recorded this conflict before 2026-08-14. Each proceeded on its own side's reading.

**Resolution, approved 2026-08-15.** The conflict was one overloaded noun, not two competing
claims. `L2-E02` already qualified its concept consistently — "Assignment Delegation" (line 359),
"temporary operational authority delegation" (line 242), "operational delegation" (line 975).
`L2-A04` was the only document using the bare word "Delegation", and it was the document
asserting exclusivity. The two documents describe different things.

Applied:

1. `L2-A04` line 114 — `Delegation` scoped to `Authorization Delegation`, plus an explicit
   delegation scope clarification recording that organisational assignment delegation belongs
   to `L2-E02` and consumes A04 authorization primitives without owning them.
2. `L2-A03` line 220 — the same sentence is mirrored there; qualified identically.
3. `L2-E02` line 975 — `Delegation` renamed to `Assignment Delegation`, matching E02's own
   canonical entity table, plus a reciprocal scope clarification in section 1.

No entity moved between documents and no substantive ownership changed. Resulting assignment:
`A-080` remains with Authorization & Policy (authorization-layer). `E-103` and `G-793` sit with
organisational delegation — `E-103` with Position & Assignment, `G-793` with Housekeeping
Operations consuming that concept.

Full record: `generated/reviews/L3_Phase1_Tranche3_Application_Report_2026-08-15.md`.
