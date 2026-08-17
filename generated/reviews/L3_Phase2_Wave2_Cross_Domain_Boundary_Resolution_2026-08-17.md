# L3 Phase 2 — Wave 2 Cross-Domain Boundary Resolution

# Executive Summary

Wave 2 reviewed the 10 current Phase 2 rows carrying `C-C01`, `I-XI-02`, or `I-XI-03`. Repository authority deterministically resolves `C-C01` and confirms that `I-XI-03` was a stale L3 cross-domain blocker. The Lead Architect approved the recommended origin-sensitive post-conferment ordering for `I-XI-02`, which is now recorded in `L2-C05` and `L2-I13` using existing repository events.

All 10 Wave 2 rows are unblocked: nine become `KEEP`, and `CAN-L3-G-002` becomes `VARIANT` into `CAN-L3-G-001` under the already-evidenced SG-01 readiness-type consolidation. No new event or owner was introduced.

# Verified Starting State

Phase 1 Candidates: **1,096**

Phase 2 Register Rows: **1,124**

BLOCKED Rows Before: **38**

The Phase 1 count was verified directly by `check_l3_inventory.py`: Chetan 75, Devang 197, Dhruv 172, Pallav 164, and Varad 488. The Phase 2 count and dispositions were recomputed directly from register data rows, not copied from the previous report. The working tree was clean before Wave 2 edits.

# Resolution Matrix

| Blocker | Rows | Classification | Resolution | Remaining Decision |
| --- | ---: | --- | --- | --- |
| `C-C01` | 2 | `DETERMINISTIC_FIX` | `CAN-L3-C-009` and `CAN-L3-C-012` -> `KEEP` | None |
| `I-XI-02` | 3 | `DETERMINISTIC_FIX` | Lead Architect approved origin-sensitive ordering; all three rows -> `KEEP` | None |
| `I-XI-03` | 5 | `STALE_L3_BLOCKER` | Four I rows -> `KEEP`; `CAN-L3-G-002` -> `VARIANT` into `CAN-L3-G-001` | None |

# C-C01 Resolution

**Classification:** `DETERMINISTIC_FIX`

**Affected current candidates:** `CAN-L3-C-009`, `CAN-L3-C-012`.

`L2-C02` defines materially different business outcomes even though both workflows may change Department, Batch, or Section references:

- WF-C02-04 Academic Assignment is triggered by Student Activation, Administrative Assignment, Academic Reassignment, or consumption of an already-approved `ProgrammeMigrationApproved` association. It completes or updates the Student Lifecycle-owned Academic Assignment, preserves history, and publishes `AcademicAssignmentCompleted`.
- WF-C02-08 Student Transfer is triggered by a Student Transfer Request or Administrative Approval. It performs a governed non-Programme organisational or placement relocation, enters the `Transferred` lifecycle state, returns to `Active`, preserves the prior assignment, and publishes `StudentTransferred`.

The canonical discriminator is the business decision and lifecycle outcome, not the overlap in changed fields. Assignment maintenance that does not invoke the Transfer workflow remains `CAN-L3-C-009`; an authorized same-Institute non-Programme relocation processed through the Transfer workflow remains `CAN-L3-C-012`. Both are `KEEP`. Programme-changing mobility remains governed separately by C04 APW-08 under the approved C-C02 ruling.

No Phase 1 wording was changed.

# I-XI-02 Resolution

**Classification:** `DETERMINISTIC_FIX`

**Affected current candidates:** `CAN-L3-C-050`, `CAN-L3-I-162`, `CAN-L3-I-163`.

Locked L1 and `L2-C05` establish that C owns Qualification Completion, approval, conferment, correction/revocation, and canonical qualification events. `L2-I13` owns examination completion evidence and publishes `ExaminationCompletionEvidencePrepared` for C05 review; it never owns the qualification decision.

```text
I-owned examination completion evidence
    -> C-owned qualification evaluation and decision
```

The Lead Architect approved the recommended origin-sensitive post-conferment sequence on 2026-08-17:

- For an Examination-originating change, I13 consumes the applicable authoritative revised fact, including `ExaminationResultPublished` or `RevaluationCompleted`, first withdraws/restores/re-establishes its own evidence as required, and publishes the superseding current position through existing `ExaminationCompletionEvidencePrepared`. C05 then alone decides qualification correction, revocation, or no change.
- For a non-Examination qualification-governance change, C05 decides first. A qualification-only correction does not alter I13 evidence unless an Examination outcome also changes. An approved revocation requiring evidence review uses existing `QualificationAwardRevoked`; I13 then dispositions only its own evidence.

The decision is recorded in `L2-C05` and `L2-I13`. `CAN-L3-C-050`, `CAN-L3-I-162`, and `CAN-L3-I-163` are distinct `KEEP` outcomes. No new event or owner was introduced.

# I-XI-03 Resolution

**Classification:** `STALE_L3_BLOCKER`

**Affected current candidates:** `CAN-L3-G-002`, `CAN-L3-I-009`, `CAN-L3-I-048`, `CAN-L3-I-055`, `CAN-L3-I-056`.

The locked L1 ownership model and L2 sources explicitly separate physical facts from Examination decisions:

- `L2-G10` section 9A.249 states that Examination consumes halls, capacity, accessibility, operational availability, and campus information. G owns examination facilities; Examination owns seating plans, invigilation, scheduling, and candidate allocation.
- `L2-G10` sections 44A.177-199 publish physical facts through domain events and public read contracts. Examination is a named consumer of examination-hall availability; consumers remain responsible for decisions, scheduling, and allocations.
- `L2-G00` defines Examination Readiness as a configurable type inside the single campus-operational Campus Readiness Workflow. G00 aggregates readiness indicators; it does not execute Examination operations.
- `L2-I03` owns venue planning and allocation while retaining venue ownership in Campus & Infrastructure.
- `L2-I06` owns examination-specific practical preparation and valid alternative arrangements after resource unavailability.
- `L2-I07` consumes `Examination Centre Available`, keeps physical infrastructure external, and owns centre operations, hall preparation, seating, and theory-examination conduct.

The four I candidates are `KEEP`. `CAN-L3-G-002` is not merged into an I outcome; it becomes `VARIANT` into the G00 parent `CAN-L3-G-001`, completing SG-01 because Examination Readiness is readiness-type data inside the existing G00 workflow. This consolidates G00 readiness assessment variants without transferring the physical lifecycle to I or an Examination lifecycle decision to G.

# Cross-Domain Ownership Matrix

| Business Fact / Outcome | Canonical Owner | Consumer | Boundary |
| --- | --- | --- | --- |
| Student Academic Assignment | Section C / C02 Student Lifecycle | Academic Delivery and other authorized consumers | WF-C02-04 publishes `AcademicAssignmentCompleted`; no Transfer lifecycle occurs |
| Non-Programme Student Transfer decision | Section C / C02 Student Lifecycle | Academic Delivery and other affected domains | WF-C02-08 enters `Transferred`, returns to `Active`, and publishes `StudentTransferred` |
| Examination completion evidence | Section I / I13 | Section C / C05 | `ExaminationCompletionEvidencePrepared` is evidence, never a qualification decision |
| Qualification correction or revocation | Section C / C05 | Qualification-event consumers; I13 when evidence review is required | C retains the qualification lifecycle; existing `QualificationAwardRevoked` triggers I13 review without transferring the decision |
| Physical venue, capacity, accessibility, lifecycle, operational availability | Section G / G10 | I03, I06, I07 | G publishes physical facts and public read contracts; consumers decide use |
| Campus-operational readiness assessment | Section G / G00 | Campus governance and applicable activity domains | Configurable assessment type; not an Examination readiness/conduct decision |
| Examination venue allocation and schedule feasibility | Section I / I03 | Examination workflows | I applies Examination requirements to G-owned physical facts |
| Practical/theory preparation, exception arrangement, and conduct | Section I / I06 and I07 | Downstream Examination workflows | I owns examination-specific operations without owning physical infrastructure |

# Phase 2 Disposition Changes

| Current candidate | Before | After | Basis |
| --- | --- | --- | --- |
| `CAN-L3-C-009` | `BLOCKED` (`C-C01`) | `KEEP` | WF-C02-04 assignment outcome |
| `CAN-L3-C-012` | `BLOCKED` (`C-C01`) | `KEEP` | WF-C02-08 transfer lifecycle and event |
| `CAN-L3-C-050` | `BLOCKED` (`I-XI-02`) | `KEEP` | Approved origin-sensitive qualification/evidence ordering; C05 owns the qualification decision |
| `CAN-L3-G-002` | `BLOCKED` (`I-XI-03`) | `VARIANT` -> `CAN-L3-G-001` | I-XI-03 boundary plus existing SG-01 readiness-type consolidation |
| `CAN-L3-I-009` | `BLOCKED` (`I-XI-03`) | `KEEP` | I03 venue allocation decision |
| `CAN-L3-I-048` | `BLOCKED` (`I-XI-03`) | `KEEP` | I06 examination-specific exception arrangement |
| `CAN-L3-I-055` | `BLOCKED` (`I-XI-03`) | `KEEP` | I07 theory-examination conduct |
| `CAN-L3-I-056` | `BLOCKED` (`I-XI-03`) | `KEEP` | I07 multi-centre conduct outcome |
| `CAN-L3-I-162` | `BLOCKED` (`I-XI-02`) | `KEEP` | I13 evidence withdrawal precedes C05 review for an Examination-originating change |
| `CAN-L3-I-163` | `BLOCKED` (`I-XI-02`) | `KEEP` | I13 restores/re-establishes its evidence after the Examination-owned condition resolves |

Recomputed Phase 2 dispositions after Wave 2:

| Disposition | Count |
| --- | ---: |
| KEEP | 1,065 |
| MERGE | 20 |
| VARIANT | 7 |
| FAILURE-RECOVERY | 0 |
| DUPLICATE | 2 |
| SPLIT-REQUIRED | 1 |
| REVIEW-REQUIRED | 1 |
| BLOCKED | 28 |
| **Total** | **1,124** |

# Remaining BLOCKED Rows

| Issue | Rows |
| --- | ---: |
| `FIND-D-02` | 2 |
| `FIND-D-06` | 5 |
| `FIND-D-07` | 8 |
| `FIND-D-08` | 1 |
| `I-TV-04-residual` | 5 |
| `J-TV-01` | 7 |
| **Total** | **28** |

All 28 remaining rows are explicitly outside this wave's scope.

# Lead Architect Decisions Required

None. The only Wave 2 decision, `I-XI-02`, was approved and applied on 2026-08-17.

# Files Modified

- `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md`
- `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md`
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_C_Phase2_Consolidation_Watchlist.md`
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_G_Phase2_Consolidation_Watchlist.md`
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_I_Phase2_Consolidation_Watchlist.md`
- `architecture/L3_Business_Scenario_Architecture/02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md`
- `generated/reviews/L3_Phase2_Repository_Wide_Consolidation_Review.md`
- `generated/reviews/L3_Phase2_Wave2_Cross_Domain_Boundary_Resolution_2026-08-17.md`

No L0, L1, or Phase 1 file was modified. The two locked L2 files were clarified only after the approved Lead Architect decision and without changing canonical owners.

# Validation

- Phase 1 candidate count: **1,096**.
- Phase 1 candidate IDs changed: **0**.
- Phase 1 rows deleted: **0**.
- P2W IDs changed: **0**; 1,124 remain unique.
- New business events invented: **0**.
- New canonical owners invented: **0**.
- Phase 2 register rows: **1,124**.
- BLOCKED before: **38**.
- BLOCKED after: **28**.
- Rows unblocked: **10**.
- `python check_l3_inventory.py`: **PASS — 0 errors, 0 warnings**.

# Repository Consistency Verdict

**PASS**

All three Wave 2 boundaries are resolved. The approved `I-XI-02` ordering preserves one owner for Qualification and one owner for completion evidence, reuses existing events, and introduces no artificial cross-domain ownership.
