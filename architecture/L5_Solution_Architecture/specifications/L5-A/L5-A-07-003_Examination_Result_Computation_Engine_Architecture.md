# L5-A-07-003 — Examination Result Computation Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-003 |
| Specification Title | Examination Result Computation Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Executes approved result formulas/regulations; cannot approve or publish official results autonomously. |
| Technical Architecture Owner | Examination Result Computation Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.1 |
| Status | Locked |
| Last Updated | 2026-09-15 |
| Depends On | L5-A-07-001; L5-A-07-005; L5-B-02-001; L5-D-03-001; L5-A-08-001; L4-I-083; L4-I-084; L4-I-085; L4-I-086; L4-I-087; L5 governance/template/validation standards |
| Referenced By | Examination result workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose & Boundary

Executes examination-result calculations using authoritative marks, rule packages and mandatory result conditions supplied by the owning examination workflow. It does not define attendance thresholds, malpractice policy, backlog limits, fee/disciplinary holds, pass/fail policy, moderation policy or publication authority.

`L4-I-085-BH-001` requires evaluation of **parent-defined mandatory result conditions** at the required computation gate using authoritative facts. `L4-I-085-BH-002` prevents advancement when such a condition is unsatisfied. Therefore A07-003 evaluates the supplied conditions but must not define which conditions exist.

## 3. Technical Realization

| Element | Responsibility |
|---|---|
| `ResultCalculationAdmission` | Validate candidate identity, attempt/context references, authoritative marks input and exact result-rule package/version. |
| `MandatoryConditionEvaluator` | Evaluate the set of parent-defined mandatory conditions supplied by the owning workflow against authoritative facts. |
| `ResultFormulaExecutor` | Execute approved grading/aggregation formula definitions through the A07 baseline. |
| `ResultInvariantValidator` | Verify output structure and parent-provided invariants; it does not invent new academic rules. |
| `ResultEvidenceAssembler` | Preserve input/rule/output lineage and recomputation predecessor reference where applicable. |
| `ResultHandoff` | Return computed outcome/condition-evaluation evidence to the owning examination workflow for approval/publication decisions. |

## 4. Canonical Ownership

Owns technical execution of result formulas and condition evaluation. Does not own academic regulations, eligibility conditions, attendance rules, malpractice consequences, backlog policy, financial/disciplinary holds, result approval, publication, transcript issuance or lifecycle states.

A condition identifier is treated as opaque business meaning supplied by the parent workflow/rule package. The engine returns satisfied/unsatisfied/evaluation-error plus evidence; it does not translate that into a new local business status vocabulary.

## 5. Failure & Recovery

- Missing authoritative marks/rule version/mandatory-condition set: reject admission.
- Unsatisfied parent-defined condition: return condition failure to the owning workflow; do not advance the computation outcome as publishable.
- Formula or deterministic evaluation error: fail computation and preserve technical evidence.
- Transient runtime failure: retry only through A08-governed execution.
- Recalculation: create new technical lineage; do not erase prior attempts.

## 6. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-030 | Mandatory result conditions are parent-defined inputs, never local A07-003 policy. | `L4-I-085-BH-001/002` |
| AD-A07-031 | Computation and official result approval/publication remain separate. | Repository-derived boundary |
| AD-A07-032 | Recalculation preserves predecessor lineage where required by parent behavior. | L4-derived |
| OD-A07-030 | Concrete grading formula language, numeric precision and execution thresholds remain governed by authoritative academic rules/downstream decisions. | Open decision |

## 7. Conformance Requirements

1. A07-003 shall not hardcode attendance, malpractice, backlog, fee, disciplinary or other mandatory result conditions.
2. Every evaluated condition shall be traceable to a parent-supplied condition/rule reference.
3. The engine shall not approve or publish official results.
4. Recalculation shall preserve technical lineage to prior attempts where applicable.
5. Background retry/scheduling shall remain A08-owned.

## 8. Traceability Summary

- `L4-I-085-BH-001` — realized by evaluating parent-defined mandatory conditions from authoritative facts.
- `L4-I-085-BH-002` — supported by returning unsatisfied-condition outcomes that prevent downstream advancement by the owning workflow.
- Other referenced L4-I specifications remain authoritative for their exact result/recomputation business semantics.