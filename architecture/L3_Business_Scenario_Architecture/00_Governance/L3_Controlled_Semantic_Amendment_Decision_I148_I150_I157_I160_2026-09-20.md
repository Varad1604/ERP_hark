# L3 Controlled Semantic Amendment Decision — I148–I150 and I157–I160 — 2026-09-20

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Architectural Level:** L3 — Business Scenario Architecture Governance  
**Domain:** Examination — Academic Completion & Completion Evidence  
**Decision Date:** 2026-09-20  
**Affected Scenarios:** `L3-I-148`, `L3-I-149`, `L3-I-150`, `L3-I-157`, `L3-I-158`, `L3-I-159`, `L3-I-160`  
**Change Authorization:** Explicit Lead Architect instruction in the SutraERP Stage-2 reconciliation session on 2026-09-20 to perform the required upstream corrections.  
**Governing Baseline:** `L3_Final_Corpus_Lock_Decision_1084_2026-08-29.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `Section_I_141_197_Canonical_Register.md`; `Architecture_Level_Validation_Standards.md`; `L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md`; `L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md`.

## 1. Finding

Stage-2 L6 semantic reconciliation identified Phase-5 copied-body drift in seven Locked Section-I scenario files.

The final IDs and titles are correct, but the substantive bodies describe different neighbouring canonical scenarios.

The authoritative frozen mappings are:

| Final ID | Frozen canonical semantic source | Canonical title |
|---|---|---|
| L3-I-148 | CAN-L3-I-153 | Completion Identity Mismatch to Correct Student Context Association |
| L3-I-149 | CAN-L3-I-154 | Duplicate Completion Evaluation to Single Authoritative Evaluation |
| L3-I-150 | CAN-L3-I-155 | Revised Result to Re-Evaluated Completion Eligibility |
| L3-I-157 | CAN-L3-I-162 | Authorized Result Change to Withdrawn Invalid Completion Evidence |
| L3-I-158 | CAN-L3-I-163 | Completion Evidence Resolution to Restored Valid Evidence |
| L3-I-159 | CAN-L3-I-164 | Duplicate Completion Evidence to Single Authoritative Evidence Position |
| L3-I-160 | CAN-L3-I-165 | Completion Evidence Identity Mismatch to Correct Candidate Association |

The canonical IDs/titles are not reopened or renumbered.

## 2. Impact Assessment

This is an authoring-correction amendment, not a new business-architecture decision.

The amendment preserves:

- the final IDs and canonical titles;
- Section I placement;
- Primary Owner `Academic Completion & Completion Evidence`;
- Scenario Type `Critical`;
- Priority `P1`;
- L2-I13 ownership of examination-derived completion evidence;
- L2-C05 ownership of the institutional Qualification Completion decision;
- Examination ownership of result/revaluation/malpractice/subsequent-examination outcomes;
- Identity and Student Lifecycle ownership of canonical identity/context facts;
- the total Locked L3 population of 1,084.

Participating-domain metadata is restored from the frozen canonical register:

- `L3-I-148` → `A, C, I`
- `L3-I-149` → `C, I`
- `L3-I-150` → `C, I`
- `L3-I-157` → `C, H, I`
- `L3-I-158` → `C, H, I`
- `L3-I-159` → `C, H, I`
- `L3-I-160` → `A, C, H, I`

## 3. Controlled Amendment Decision

The seven affected scenario bodies shall be replaced with content faithful to their frozen canonical trigger/outcome semantics and locked upstream ownership.

The corrections shall:

1. preserve each existing final ID and title;
2. restore the canonical participating domains above;
3. use authoritative Person/Identity and Student Lifecycle facts without transferring their ownership;
4. use authoritative Examination outcomes without recomputing or rewriting them;
5. preserve L2-C05 as the sole owner of Qualification Completion consequences;
6. reuse `ExaminationCompletionEvidencePrepared` only where I13 supplies the current examination-owned completion-evidence position to Student Lifecycle;
7. consume existing Examination facts such as `ExaminationResultPublished` and `RevaluationCompleted` where applicable;
8. preserve evidence/history when an evidence position is withdrawn, restored, superseded, de-duplicated or reassociated;
9. introduce no new lifecycle state, event, publisher, approval authority, aggregate owner or technical design.

## 4. Successor-State Decision

Each corrected scenario advances from version `1.0` to `1.1`, retains `Status: Locked`, and carries `Last Updated: 2026-09-20`.

The corpus is not reopened globally. These seven files become the corrected Locked successor content for their existing canonical identities.

## 5. Downstream Consequences

The directly dependent L4 specifications `L4-I-148`, `149`, `150`, `157`, `158`, `159`, and `160` shall be re-derived from the corrected parents.

Repository inspection found no L5 specifications named for these seven parent identities, so there is no direct L5 artifact to amend. Any indirect L5 dependency discovered during later trace validation remains subject to normal impact review.

The Stage-2 L6 realization rows for the seven scenarios shall then be revalidated against the corrected parent semantics.

## 6. Validation Requirements

The corrected successor set must pass:

- exact final ID/title mapping;
- canonical owner and participating-domain alignment;
- frozen trigger/outcome fidelity;
- L2-I13 evidence ownership;
- L2-C05 Qualification Completion ownership;
- Person/Identity and Student Lifecycle ownership;
- event/publisher integrity;
- lifecycle integrity;
- implementation independence;
- downstream L4 re-derivation;
- targeted Stage-2 realization revalidation.

## 7. Decision Statement

**CONTROLLED L3 AMENDMENT:** APPROVED  
**AFFECTED L3 SCENARIOS:** 7  
**FINAL IDS / TITLES:** PRESERVED  
**CORRECTED SUCCESSOR STATUS:** LOCKED  
**CORRECTED SUCCESSOR VERSION:** 1.1  
**L3 CORPUS SIZE:** UNCHANGED — 1,084  
**DIRECT L4 SPECIFICATIONS REQUIRING RE-DERIVATION:** 7  
**DIRECT L5 SPECIFICATIONS FOUND:** 0  
**STAGE-2 REVALIDATION:** REQUIRED AFTER L3/L4 CORRECTION
