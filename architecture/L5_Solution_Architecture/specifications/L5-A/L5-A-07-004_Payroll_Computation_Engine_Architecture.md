# L5-A-07-004 — Payroll Computation Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-004 |
| Specification Title | Payroll Computation Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Executes approved payroll calculations while Payroll retains business ownership and Finance owns downstream monetary consequences. |
| Technical Architecture Owner | Payroll Computation Engine Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.1 |
| Status | Locked |
| Last Updated | 2026-09-15 |
| Depends On | L5-A-07-001; L5-A-07-005; L5-A-08-001; L5-B-02-001; L5-D-03-001; L4-E-053; L4-E-055; L4-E-056; L5 governance/template/validation standards |
| Referenced By | Payroll and downstream Finance workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose & Boundary

Executes payroll calculations from authoritative payroll inputs and approved/versioned payroll rules. It does not create statutory thresholds, deduction caps, wage-floor rules, benefit policy, final-settlement policy, approval rules or Finance posting consequences.

Any statutory or institutional constraint is an authoritative business-rule input. A07-004 evaluates that input and reports the result; it does not infer a legal requirement from generic industry practice.

## 3. Technical Realization

| Element | Responsibility |
|---|---|
| `PayrollCalculationAdmission` | Validate employee/pay-period context, authoritative earnings/deduction inputs and exact payroll-rule package/version. |
| `PayrollRuleBinder` | Bind the approved payroll rule set supplied by the Payroll owner. |
| `PayrollEvaluationPipeline` | Execute earnings, deduction, contribution and net calculations exactly as defined by bound rules. |
| `PayrollInvariantEvaluator` | Evaluate authoritative caller/rule-defined invariants without inventing new statutory policy. |
| `PayrollEvidenceAssembler` | Preserve rule/input/output lineage and technical calculation evidence. |
| `PayrollResultHandoff` | Return computation result to Payroll; downstream disbursement/accounting remain outside this engine. |

## 4. Ownership

Owns computation mechanics and technical validation. Does not own employee/payroll business policy, statutory interpretation, voluntary deduction policy, final settlement entitlement, payroll approval, payment execution, ledger posting or bank integration.

## 5. Failure & Recovery

Missing rule/version or authoritative input causes admission failure. A violated authoritative payroll invariant returns a structured technical failure to Payroll; the engine must not silently clamp, waive or substitute values. Deterministic formula failures are not automatically retried. Transient infrastructure failures may be retried through A08.

## 6. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-040 | Payroll policies and statutory constraints are authoritative inputs, not local engine rules. | Repository-corrected boundary |
| AD-A07-041 | Payroll computation is separated from approval, payment and accounting posting. | Canonical ownership boundary |
| OD-A07-040 | Concrete monetary representation, payroll formula technology and numeric performance/resource limits remain unresolved until governed. | Open decision |

## 7. Conformance Requirements

1. A07-004 shall not hardcode wage floors, deduction caps, statutory rates or final-settlement rules unless cited to authoritative repository policy.
2. Every payroll calculation shall bind exact authoritative inputs and rule version.
3. The engine shall not approve payroll, initiate payment or post Finance state.
4. Violated business constraints shall be reported to Payroll rather than silently repaired.
5. Background work/retry remains A08-owned.

## 8. Traceability Summary

Referenced L4-E payroll behaviors remain the authority for exact payroll semantics. This L5 specification provides execution infrastructure only and deliberately removes previously unsupported statutory/business-policy assertions.