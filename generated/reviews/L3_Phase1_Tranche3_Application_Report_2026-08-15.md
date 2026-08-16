# L3 Phase 1 — Audit Correction Application Report

**Date:** 2026-08-15
**Applied by:** Claude (Opus 5), under Lead Architect approval
**Source audit:** `generated/reviews/l3-phase1-audit-all-2026-08-14-2.md` (score 8.8/10.0, verdict BLOCKED)
**Scope:** Tranches 1, 2 and the approved parts of Tranche 3
**Conformance:** `check_l3_inventory.py` — **PASS, 1,123 candidates, 0 errors, 0 warnings**

---

# 1. Summary

| | Before | After |
|---|---:|---:|
| Candidates | 1,175 | **1,123** |
| Checker errors | 0 | 0 |
| Checker warnings | 8 | **0** |
| Open repository conflicts | 3 | **0** |

52 candidates retired, 20 retained candidates rewritten to absorb them, 146 titles converted to
outcome form, 929 Primary Owner values normalised, 3 repository conflicts resolved in the L2
documents.

**Section movement**

| Section | Before | After | Δ |
|---|---:|---:|---:|
| G Campus Operations | 377 | 350 | −27 |
| I Examination | 218 | 197 | −21 |
| H Enterprise Capabilities | 85 | 81 | −4 |
| J, F, E, C, A, D, B | unchanged | unchanged | 0 |

Retired candidate IDs must not be reused. Sparse numbering preserves the consolidation crosswalk.

---

# 2. Repository conflicts

## RC-01 Delegation ownership — RESOLVED

The conflict was one overloaded noun. `L2-E02` already qualified its concept in three places
("Assignment Delegation" line 359, "temporary operational authority delegation" line 242,
"operational delegation" line 975). `L2-A04` was the only document using the bare word
"Delegation" — and the only one asserting exclusivity over it.

Applied:

| Document | Change |
|---|---|
| `L2-A04` line 114 | `Delegation` → `Authorization Delegation`; added a delegation scope clarification |
| `L2-A03` line 220 | Same sentence mirrored there; qualified identically |
| `L2-E02` line 975 | `Delegation` → `Assignment Delegation`; added a reciprocal scope clarification in section 1 |

No entity moved between documents; no substantive ownership changed. `A-012` stays with
Authorization & Policy. `E-103` and `G-362` are organisational delegation.

**`CAN-L3-G-362` was missing from the RC-01 affected-candidate list** in both the Section A and
Section B/D/E registers. Added.

## RC-02 Question paper governance — RESOLVED

`L2-I07` section 2 disclaimed Question Paper Creation, Moderation and Printing, deferring them to
"their respective workflow documents." Section I contains `L2-I01` through `L2-I16` and **no such
document exists** — the capability had been formally disowned into a vacuum.

The institutional decision recorded 2026-08-15 is that SutraERP does not model external paper
setting, moderation boards, printing or physical custody chains. The Examination Controller
authors the paper directly — typed or uploaded — and the system holds it and releases it at the
scheduled start.

Applied to `L2-I07`: added *Question Paper Intake* and *Question Paper Custody and Timed Release*
to the governed capability list, removed the three orphaned out-of-scope lines, and recorded the
decision and its rationale in a scope note. This is the upstream of the Question Paper
Distribution I07 already governed at section 9.5.

Consequence: `I-111`, `I-112` and `I-113` are legitimately I07-owned and were never blocked. No
new L2 document was required.

## RC-03 Policy Management — RESOLVED

`DEPENDENCY_MAP` line 409 recorded Policy Management as "Open. No owner assigned," having checked
and excluded three candidate homes: `L2-C08`, `L2-A04` and `L2-A08`.

**`L2-B01` was never among the candidates checked.** `L2-B01` section 2.1, *Approved Institution
Policy Ownership Clarification*, cites `SutraERP_L1_Business_Domain_Architecture_Locked.md`:
"Institute owns institutional policies." Institution Policy is a canonical entity of that domain,
explicitly separated there from A04 authorization policy, with `L2-A00` disclaiming institutional
policy ownership.

Discovery Rules section 15 ranks Locked L1 (tier 4) above the Dependency Map (tier 5). The
hierarchy resolved this; the register was stale relative to L1. Recorded as a documentation
correction, not a new architecture decision.

The two orphaned events were assigned per the convention already recorded at lines 432–433
(Examination Policy → `L2-I01`, Leave Policy → `L2-E05`, Eligibility Policy → `L2-C07`):
`AcademicPolicyUpdated` → `L2-C04`, `AlumniTransitionPolicyUpdated` → `L2-C08`. The bar on
subscribers and query clients is lifted; consuming documents carrying that instruction should be
updated.

`CAN-L3-B-022` *Institutional Policy Revision* owned by Institution is therefore
repository-correct. It was not a candidate silently resolving an open conflict.

---

# 3. Inventory corrections

## 3.1 Primary Owner normalisation — 929 rows

Section-code owners (`F05`, `I07`, `G03`) replaced with business-domain names, per Discovery
Rules section 10, whose worked examples all use domain names. Values derived from the L2 document
filenames. Pallav 164, Varad 547, Devang 218.

Two deliberate adjustments: `B03` → *Institutional Campus* rather than bare *Campus*, preserving
the B03/G10 boundary; `G00` → *Campus Operations Governance*.

## 3.2 Section G consolidation — 27 retired

See `L3_Section_G_Phase2_Consolidation_Watchlist.md` section 10 for the full cluster table.

**Root cause.** Consolidation principles recorded as `APPLIED` (`G-FR-05` validation-failure
absorption, `G-FR-08` publisher-side information delivery, `G-FR-12` consumer-side read models)
were applied cluster-by-cluster rather than swept across the section. Identical constructs
survived in G08 Housekeeping and G09 Teacher Guardian, which no enumerated cluster had reached.
All 7 surviving Discovery-Rules-section-8-disqualified rejection rows were in those two
sub-domains.

**Recommendation for future watchlists:** recording a principle as `APPLIED` should require a
section-wide sweep for the construct, not only for the enumerated cluster.

## 3.3 Section I — 21 retired, 146 retitled

Cluster rulings are recorded in `L3_Section_I_Phase2_Consolidation_Watchlist.md`. Two of the four
clusters were ruled MERGE, one KEEP SEPARATE, one SPLIT.

`I-074`/`I-096` and `I-092`/`I-121` were **differentiated, not merged**. They are the practical
and theory forms of the same situation, owned by `L2-I06` and `L2-I07`; Section I mirrors the two
conduct modes deliberately throughout. The duplication was in the Final Outcome wording.

146 titles converted from imperative action-fragment form to the canonical `<condition> to
<outcome>` form. Every replacement was derived from the row's own recorded Final Outcome. No
outcome, state, trigger, owner or business rule was changed by the retitling.

## 3.4 Section H — 4 retired

H03/H07 boundary ruling recorded in `L3_Section_H_Phase2_Consolidation_Watchlist.md`. `H-021` and
`H-023` were prefixed with "Enterprise"; they were never action fragments — the checker was
matching the noun "Search" against `FRAGMENT_VERBS`.

## 3.5 VAR notation — 2 rows

`H-018` → `A, H, VAR:Source-Owning Domain`; `H-085` → `A, H, VAR:Exchanging Domain`, per Discovery
Rules section 11. Both previously enumerated participating domains as a literal list.

---

# 4. Findings withdrawn

## FIND-A13 (`CAN-L3-C-044`) — WITHDRAWN

The audit flagged *Administrative Exit Closure* as an under-specified catch-all. Checked against
source: `L2-C05` section 9.3 defines Administrative Exit as one of three **named canonical
alternative exits** alongside Termination and Deceased, with its own state in the lifecycle
machine at lines 1247 and 1256, and states that "each alternative exit follows institution-specific
governance."

The generic wording is source-faithful. A replacement was drafted, found to invent both a state
name ("Administratively Closed" — the actual state is `AdministrativeExit`) and triggering
conditions the source does not state, and was fully reverted. **The finding was wrong; the row is
correct as written.** Section C loses that deduction.

## H-021 / H-023 — reclassified as checker false positives

Recorded in the audit as Discovery Rules section 8 violations. Both are noun-phrase outcome titles.
`check_l3_inventory.py` lines 65–69 match on first word only, and "search" is in `FRAGMENT_VERBS`.
The inventory was correct; the tool's heuristic is imprecise. Resolved by prefixing rather than by
weakening the checker.

---

# 5. Deliberately not changed

## Type and Priority — left as recorded, by Lead Architect decision

Section F is 85 rows with **zero Simple and zero P2**; Section I was 90% Critical. Type and
Priority Rules section 14, Mistake 2, names both patterns by name as incorrect.

The Lead Architect elected on 2026-08-15 to leave both as recorded rather than re-derive, and to
carry this as a known Phase 2 item. Nothing was changed. Sections A, C, D, E and J retain their
correctly varied ratings.

The checker's near-uniform-Type warning for Section I cleared incidentally when the `I-TV` merges
retired Critical rows. **That is not a fix** — the underlying distribution issue stands.

Recommended approach when it is addressed: agree a short per-section rule sheet (~6 rules) and
apply it mechanically with an exception list, rather than re-deriving ~300 values individually.

## Remaining open items

| Item | Where |
|---|---|
| `I-139`, `I-148`, `I-149`, `I-150`, `I-156` — in the `I-TV-04` range, outside approved scope | Section I watchlist |
| `H-020` retained while the equivalent `H-058` was retired — asymmetry flagged | Section H watchlist |
| Residual participation review | Section G watchlist section 4A |
| `BDE-B-01`, `FIND-D-01`…`D-08`, `CF-01`, `CF-05` | Respective watchlists |
| Type and Priority for F and I | This report, section 5 |

---

# 6. Method and integrity

All bulk edits were applied through a table-cell edit engine operating on lines split with
`splitlines(keepends=True)`, so a retirement removes the whole list element including its line
terminator. This structurally cannot concatenate adjacent rows — the corruption class that
affected 12 rows during the 2026-08-14 correction, which used exact-match string replacement.

Files were read and written with `newline=""` throughout, preserving the repository's mixed line
endings per file (Varad and Dhruv LF; Devang, Chetan and Pallav CRLF).

Every retirement was logged with its candidate ID, title and a stated reason before saving. The
conformance checker was run after each script.

**No business scenario, canonical owner, aggregate, event, state, business rule, dependency,
domain participation or institutional requirement was invented.** Every rewritten Final Outcome
is composed from text already present in the rows it absorbs. Every new title is derived from its
row's own recorded Final Outcome. The one instance where this rule was breached — the C-044 draft
— was caught against source and reverted, and is reported in section 4 above.

---

# 7. Score

The source audit scored 8.8/10.0. This report does **not** restate a score.

The corrections here were applied by the same agent that produced the audit, and re-scoring one's
own corrections is not an independent result. A re-audit, if the score is used as a gate, should
be run by a reviewer who did not apply these changes.

What can be stated factually: all 3 repository conflicts recorded as blockers are resolved and
recorded; the checker reports 0 errors and 0 warnings against 1,123 candidates; and the Type and
Priority distribution issue in Sections F and I remains open by decision.

---

**End of Report**
