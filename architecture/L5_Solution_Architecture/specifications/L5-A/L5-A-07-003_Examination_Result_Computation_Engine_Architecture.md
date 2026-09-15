# L5-A-07-003 — Examination Result Computation Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-003 |
| Specification Title | Examination Result Computation Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Executes approved result formulas/regulations and evaluates parent-defined mandatory result conditions; cannot approve or publish official results autonomously. |
| Technical Architecture Owner | Examination Result Computation Technical Architecture |
| Assigned Author | Varad |
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5-A-07-001; L5-A-07-005; L5-B-02-001; L5-D-03-001; L5-A-08-001; L4-I-083; L4-I-084; L4-I-085; L4-I-086; L4-I-087; L5 governance/template/validation standards |
| Referenced By | Examination result workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose & Boundary

Executes examination-result calculations using authoritative marks, rule packages and mandatory result conditions supplied by the owning examination workflow. It does not define attendance thresholds, malpractice policy, backlog limits, fee/disciplinary holds, pass/fail policy, moderation policy or publication authority.

`L4-I-085-BH-001` requires evaluation of **parent-defined mandatory result conditions** at the required computation gate using authoritative facts. `L4-I-085-BH-002` prevents advancement when such a condition is unsatisfied. Therefore A07-003 evaluates the supplied conditions but must not define which conditions exist.

## 3. Architectural Invariants

1. Mandatory result conditions are opaque authoritative inputs from the owning examination workflow/rule package.
2. The engine may evaluate a condition but may not invent condition meaning, precedence, waiver or override rules.
3. Marks/assessment inputs and result rules must be bound to exact authoritative references sufficient for reproducibility where required.
4. Computed technical outcome is not equivalent to official result approval/publication.
5. Unsatisfied mandatory conditions must remain distinguishable from formula/runtime failure.
6. Recalculation creates a new technical attempt with predecessor lineage where evidence/history is required.
7. The engine does not create a parallel `ResultStatus` or other business lifecycle vocabulary.

## 4. Technical Realization

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `ResultCalculationAdmission` | Validate candidate/attempt context, authoritative marks input, correlation identity and exact result-rule package/version. | Authorized result-computation request | Admitted request | Rejects missing/ambiguous context. |
| `MandatoryConditionEvaluator` | Evaluate parent-defined mandatory conditions against authoritative facts supplied/referenced by the owning workflow. | Condition set + authoritative facts | Condition-evaluation outcomes | Cannot add, waive or reinterpret conditions. |
| `ResultFormulaExecutor` | Execute approved grading/aggregation formula definitions through A07-001/A07-005. | Bound rule + authoritative marks | Candidate result | Deterministic formula failure remains technical failure. |
| `ResultInvariantValidator` | Verify output structure and parent-provided invariants. | Candidate result + invariant references | Validated result | Cannot invent new academic invariants. |
| `ResultEvidenceAssembler` | Preserve marks/rule/condition/output lineage and recomputation predecessor reference where applicable. | Completed attempt | Technical result evidence | Evidence failure prevents successful completion. |
| `ResultHandoff` | Return computed outcome and condition-evaluation evidence to the owning examination workflow. | Validated technical result | Domain-consumable result package | Cannot approve/publish official result. |

## 5. Conceptual Contract

A request conceptually carries:

- tenant/institution and owning-workflow reference;
- candidate/examination/attempt context references as supplied by the owner;
- authoritative marks/input snapshot reference;
- exact result-rule package/version reference;
- parent-defined mandatory-condition set/reference;
- correlation/recalculation predecessor reference where applicable.

A result conceptually returns:

- technical attempt reference;
- computed result values or references;
- per-condition evaluation outcome/reference;
- exact rule/input references used;
- technical failure classification where unsuccessful;
- evidence/lineage and predecessor references.

Exact payload fields remain downstream design.

## 6. Mandatory Condition Semantics

- A condition identifier is opaque business meaning; the engine only evaluates the supplied predicate/constraint definition against authoritative facts.
- An unsatisfied condition is not automatically translated into a business status such as withheld, failed, detained or blocked unless the owning workflow defines that consequence.
- If required facts for a condition cannot be resolved authoritatively, evaluation returns unresolved/error rather than assuming satisfaction.
- Condition precedence, waiver, override and exception policy remain with the owning examination business authority.

## 7. Recalculation & Result Lineage

A recalculation/reprocessing request is treated as a distinct technical attempt when history is required. The engine preserves the predecessor reference, exact rule/version and authoritative input references used. It does not erase or rewrite prior attempts merely because a later calculation becomes the business-selected result.

## 8. Canonical Ownership

### Owns

Technical execution of result formulas, mandatory-condition evaluation mechanics, structural validation, result lineage and handoff.

### Does Not Own

Academic regulations, eligibility conditions, attendance rules, malpractice consequences, backlog policy, financial/disciplinary holds, moderation authority, result approval, publication, transcript issuance or result lifecycle state.

## 9. Failure & Recovery

| Failure | Treatment |
|---|---|
| Missing marks/rule/condition reference | Reject admission. |
| Mandatory condition unsatisfied | Return explicit unsatisfied evaluation; owning workflow decides consequence. |
| Mandatory condition facts unavailable | Return unresolved/evaluation error; do not assume pass. |
| Formula/deterministic evaluation error | Fail technical computation and preserve evidence. |
| Output invariant violation | Reject technical result; no local academic repair. |
| Evidence handoff failure | Do not report successful completion. |
| Transient runtime failure | Retry only through A08-governed execution where permitted. |

## 10. Security, Privacy & Evidence

- Candidate/assessment information is tenant/institution scoped and handled under applicable privacy/classification rules.
- Result computation diagnostics should identify failure categories without unnecessarily exposing marks or sensitive conditions.
- Integrity/evidence operations follow A07-001/D03/D04 boundaries.
- The engine shall not directly alter academic records or publication state.

## 11. Observability & Operability

Technical evidence should make it possible to correlate a computation with the candidate/attempt reference, exact marks/input snapshot, rule version, mandatory-condition set, per-condition technical outcome, predecessor recalculation and final technical handoff state. Numeric performance targets remain downstream governed choices.

## 12. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-030 | Mandatory result conditions are parent-defined inputs, never local A07-003 policy. | `L4-I-085-BH-001/002` |
| AD-A07-031 | Computation and official result approval/publication remain separate. | Repository-derived boundary |
| AD-A07-032 | Recalculation preserves predecessor lineage where required by parent behavior. | L4-derived |
| AD-A07-033 | Unresolved mandatory-condition facts fail as unresolved/error rather than being presumed satisfied. | Repository-consistent technical derivation |
| OD-A07-030 | Concrete grading formula language, numeric precision and execution thresholds remain governed by authoritative academic rules/downstream decisions. | Open decision |

## 13. Conformance Requirements

1. A07-003 shall not hardcode attendance, malpractice, backlog, fee, disciplinary or other mandatory result conditions.
2. Every evaluated condition shall be traceable to a parent-supplied condition/rule reference.
3. Missing authoritative facts shall not be treated as condition satisfaction.
4. The engine shall not approve or publish official results.
5. Recalculation shall preserve technical lineage to prior attempts where applicable.
6. The engine shall not create a parallel business result lifecycle/status vocabulary.
7. Background retry/scheduling shall remain A08-owned.

## 14. Architecture Acceptance Evidence

A conforming design must demonstrate that parent-defined conditions are consumed without local policy invention, unsatisfied conditions are distinguishable from runtime errors, official publication cannot occur inside A07-003, and recalculation preserves traceable lineage.

## 15. Traceability Summary

- `L4-I-085-BH-001` — realized by evaluating parent-defined mandatory conditions from authoritative facts.
- `L4-I-085-BH-002` — supported by returning unsatisfied-condition outcomes that prevent downstream advancement by the owning workflow.
- Other referenced L4-I specifications remain authoritative for exact result, approval, recomputation and publication business semantics.