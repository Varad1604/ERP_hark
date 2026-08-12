# L3 Section A — Phase 2 Consolidation Watchlist

**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section A — Platform Foundation, candidates `CAN-L3-A-001` … `CAN-L3-A-064`
**Status:** Active — Phase 2 input
**Created:** 2026-08-08
**Resolved:** 2026-08-08
**Supersedes:** section 3.3 of `../01_Phase1_Discovery/L3_Phase1_Inventory_Dhruv.md` and section 7 of `L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md`

---

## 1. Purpose

This document holds the candidate relationships identified during the independent Section A verification of 2026-08-08 and records, for each one, whether it is settled or still requires a Phase 2 decision.

It exists as a separate document because `L3_Phase1_Discovery_Rules.md` section 3 requires each Phase 1 inventory file to contain candidate scenario rows only, and section 16 requires the inventory to avoid long explanations. Relationship analysis is review output, not discovery output, and belongs here.

**Nothing in this document has been consolidated.** Every candidate named below remains in the inventory under its original ID. `L3_Phase1_Discovery_Rules.md` section 17 places duplicate identification, merging and splitting in the post-Phase-1 review, and that review cannot begin while four of the five Phase 1 inventory files contain zero rows.

This document is not a governance standard. It adds and amends no Phase 1 rule.

---

## 2. Status Values

| Status | Meaning | Phase 2 action |
| ------ | ------- | -------------- |
| `CLOSED` | Repository evidence establishes the candidates are distinct. No consolidation question remains. | None. Do not re-raise. |
| `DETERMINED` | Repository evidence is one-sided. The consolidation is decided; only its execution is deferred. | Execute the recorded merge. |
| `OPEN` | Repository evidence supports both readings. A judgement is genuinely required. | Decide. |
| `BLOCKED` | The cluster cannot be assessed until an integrity finding is ruled on. | Wait for the Enterprise Architecture Board. |

Resolution counts: 10 `CLOSED`, 3 `DETERMINED`, 3 `OPEN`, 2 `BLOCKED`. Total 18.

Only the 3 `OPEN` and 2 `BLOCKED` clusters carry unfinished Phase 2 work. The other 13 are recorded so the analysis is not repeated.

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

## 4. Determined — Consolidation Decided, Execution Deferred

Repository evidence in these three clusters is one-sided. The decision is recorded here; only its execution waits for Phase 2, because Phase 1 must not delete a supported candidate.

| Cluster | Candidates | Determination | Evidence |
| ------- | ---------- | ------------- | -------- |
| A-C01 Tenant reversible restriction | 002, 003 | **Merge 003 into 002.** | `L2-A01` defines no tenant restoration workflow. The only support for 003 is the section 5.8 transition-table row `Suspended → Active` at `L2-A01:1734`. A permitted transition is not a discovered business scenario. |
| A-C07 Audit investigation family | 024, 025, 026 | **Merge 025 and 026 into 024.** | See section 4.1. |
| A-C10 Module suspension and restoration | 033, 034 | **Merge 034 into 033.** | `L2-A08:3164` lists `Suspend Module` as a registry capability. Section 15 defines fifteen registry workflows and **no reactivation workflow**. The only support for 034 is the transition-table row `Suspended → Active` at `L2-A08:1305`. |

A-C01 and A-C10 are the same defect in two sections: a candidate resting on a state-machine edge with no workflow behind it. Phase 2 should settle the general rule once — **a permitted lifecycle transition with no source workflow is not an independent L3 scenario** — and apply it across all sections rather than re-litigating it per candidate. Applying that rule to Section A affects 003 and 034 only; 029 and 045 were previously grouped with them in error and are addressed in sections 3.1 and 5.

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

## 5. Open — Phase 2 Decision Required

| Cluster | Candidates | Decision needed | Position |
| ------- | ---------- | --------------- | -------- |
| A-C05 Workflow execution and human task | 016, 064 | Is human task resolution an outcome in its own right, or an internal phase of workflow instance execution? | 064 has its own canonical aggregate and an SLA and escalation dimension absent from 016. Leaning retain. 017 is settled — failure recovery is separable and stays. |
| A-C12 Dataset version creation | 039, 046 | Is `L2-A09` MD-17 a second entry point into MD-10, or an independent outcome? | Genuinely balanced. Shared successor-creation core, but MD-17 has a different actor (Authorized Steward **or Platform Administrator** vs Data Steward), a different authorization gate (restoration policy), and a different published event (`HistoricalVersionRestored` vs `DatasetVersionCreated`). |
| A-C14 Dataset lifecycle closure | 044, 045 | Is archival of a retired dataset an independent outcome? | **Downgraded from "merge" on 2026-08-08.** MD-18 has its own actor (Platform Administrator), its own authorization gate, its own event (`DatasetArchived`), and a success outcome that adds long-term archival storage and preserved searchability, which MD-11/MD-12 retirement does not deliver. Its precondition "Dataset is retired" establishes sequence, not equivalence. Leaning retain; decide in Phase 2. |

---

## 6. Blocked — Awaiting Enterprise Architecture Board

Both clusters sit inside the `L2-A10` Operational Readiness block and cannot be assessed while **INT-302** is open. See `generated/reviews/review-l3-section-a-conflicts-2026-08-08.md`.

| Cluster | Candidates | Question | Provisional position |
| ------- | ---------- | -------- | -------------------- |
| A-C15 Readiness assessment cycle | 047, 053, 054, 055 | Where does one governance cycle end and the next begin? `L2-A10` sections 23.1 and 28.1 share five trigger conditions, and 047's trigger list already contains 054's entry point. | Fold 054. Retain 053 — a "no reassessment required" decision is a real outcome. Review 055 against 047's closure phase. |
| A-C17 Certificate lifecycle | 051, 052 | **052 is a compound outcome**, combining suspension, restoration, supersession, revocation and archival — outcomes with different approval models and different section 35.2 downstream consequences. | Recorded as a Phase 2 **split** candidate. Proposed three-way separation: (1) temporary suspension and restoration; (2) supersession by a successor certification; (3) revocation and historical closure. Archival remains lifecycle handling unless it independently qualifies. **Do not split before the Board rules.** |

A third blocked item is not a consolidation question but is recorded for completeness: candidates 008–012, 056–058 and 061 carry `[owner contested — INT-301]`. Their Primary Owner, not their boundary, is disputed.

---

## 7. Phase 2 Items Outside This Watchlist

These were recorded in the audit application report as watchlist entries but are not candidate-relationship questions. They are restated here so section 7 of that report can be retired without loss.

* **`X` in the Domains field** — 26 rows use `X` to mean a genuinely varying participant. Phase 2 must confirm the extension of the `L3_Naming_Numbering_Standard.md` section 5 discovery-area code into the Domains field, or define an alternative.
* **Repository-wide Phase 1 completeness** — the Chetan, Devang, Pallav and Varad inventories contain zero candidate rows. Section A's `X` participants and cross-domain claims cannot be tested until Sections B, C and F are discovered.
* **This inventory's own outstanding scope** — Section J, cross-domain (`X`), governance and pilot-framing candidates remain unrecorded, and are marked as such in inventory sections 4, 5 and 6.

---

## 8. Effect on Section A Candidate Count

No change. The inventory holds 64 candidates and continues to hold 64.

If Phase 2 executes the three determinations in section 4 without changing anything else, Section A would resolve to 60 candidates: 003, 025, 026 and 034 fold into 002, 024 and 033. That figure is indicative and is not a Phase 1 commitment.
