# L5-A-07-002 — Finance Computation Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-002 |
| Specification Title | Finance Computation Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Executes approved/versioned financial calculations; cannot autonomously approve or post canonical financial transactions. |
| Technical Architecture Owner | Finance Computation Engine Technical Architecture |
| Assigned Author | Varad |
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5-A-07-001; L5-A-07-005; L5-B-02-001; L5-D-03-001; L5-A-08-001; L4-F-001; L4-F-005; L4-F-009; L4-F-012; L5 governance/template/validation standards |
| Referenced By | Student billing and finance workflows requiring governed calculation |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose & Boundary

Provides pure financial computation over authoritative fee structures, installment definitions, concession rules, penalty rules and other financial parameters supplied by owning Finance workflows. It returns calculation results and evidence; it does not approve fee policy, determine concession precedence, create tax rules, post ledger entries or mutate student-account state.

`L4-F-001-BH-004` establishes that, when charging is applicable, the system resolves the applicable Fee Structure and calculates the monetary value of Student Charge. This specification realizes only the technical calculation portion after the authoritative structure/policy inputs have been resolved.

## 3. Architectural Invariants

1. Financial policy, rates, precedence and eligibility remain authoritative inputs from the owning Finance workflow.
2. Monetary representation, precision, scale and rounding behavior must come from authoritative finance policy or governed downstream technical decision; no default is invented here.
3. Every completed calculation binds the exact fee/rule/version inputs used.
4. Calculation and posting are separate: a technical result is never equivalent to an approved charge, invoice, journal entry or payment obligation.
5. Reconciliation checks may evaluate authoritative invariants, but may not silently repair or rebalance results using locally invented policy.
6. Duplicate technical submissions must be correlated/idempotent where the owning workflow supplies an idempotency identity.
7. Historical calculation lineage is preserved where recalculation/repricing evidence is required.

## 4. Technical Realization

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `FinanceCalculationAdmission` | Validate tenant/context, calculation type, currency consistency, correlation identity and references to authoritative fee/rule inputs. | Authorized finance calculation request | Admitted calculation request | Rejects missing/ambiguous policy context or currency. |
| `FinancialRuleBinder` | Bind exact approved/versioned financial rule package and parameters. | Fee/rule references | Bound calculation policy | No fallback to unrelated or superseded policy. |
| `MonetaryEvaluationPipeline` | Execute the bound arithmetic using a representation that satisfies the authoritative monetary contract. | Bound policy + authoritative monetary inputs | Candidate financial result | Fails on non-representable or inconsistent authoritative input. |
| `ReconciliationVerifier` | Verify caller-supplied mathematical invariants such as component/total or installment/total equality where those invariants are authoritative. | Candidate result + invariant references | Validated financial result | Returns mismatch; does not silently alter values. |
| `FinancialEvidenceAssembler` | Record exact rule/version/input/output lineage through A07-001. | Completed calculation attempt | Finance calculation evidence | Evidence failure prevents successful completion. |
| `FinanceResultHandoff` | Return calculation result to the owning Finance workflow without posting it. | Validated result | Domain-consumable calculation result | Cannot approve or post financial state. |

## 5. Conceptual Contract

A finance calculation request conceptually contains:

- tenant/institution and owning-workflow reference;
- calculation/correlation identity;
- currency/context reference;
- authoritative fee structure/rule/version references;
- authoritative monetary input snapshot/reference;
- applicable caller-provided invariants.

A result conceptually returns:

- calculation attempt reference;
- computed components/totals or references to them;
- exact rule/input references used;
- reconciliation outcome;
- technical evidence/lineage reference;
- diagnostic classification when unsuccessful.

Exact field names and transport schema remain downstream design.

## 6. Monetary & Reconciliation Semantics

- Currency identity must be explicit enough to prevent accidental cross-currency aggregation.
- Precision/rounding rules are bound to authoritative policy; absence of required policy is an admission failure, not permission to use an implementation default.
- Installment residual allocation follows the authoritative installment definition when such a rule exists.
- Concession, scholarship, surcharge, penalty and tax precedence is consumed from authoritative rules only.
- Reconciliation verifies authoritative mathematical relationships; it does not create accounting policy.
- Recalculation creates a new technical attempt linked to the prior attempt where history is required; prior evidence is not overwritten.

## 7. Ownership

### Owns

Calculation execution mechanics, numeric evaluation according to supplied policy, technical reconciliation, lineage and result handoff.

### Does Not Own

Fee structures, concession eligibility/precedence, tax/statutory policy, installment policy, invoice lifecycle, charge approval, ledger posting, payment collection, accounting recognition or refund policy.

## 8. Failure & Recovery

| Failure | Treatment |
|---|---|
| Missing authoritative fee/rule input | Reject before calculation. |
| Ambiguous/mismatched currency context | Fail closed; do not coerce or convert implicitly. |
| Unsupported/missing monetary policy | Reject; do not use an implementation default. |
| Formula/rule evaluation failure | Return deterministic calculation failure; no blind retry. |
| Reconciliation mismatch | Return failed reconciliation and preserve evidence; do not auto-adjust. |
| Evidence/persistence failure | Do not report successful completion. |
| Transient execution dependency failure | Delegate retry/work recovery to A08 when permitted. |

## 9. Security, Trust & Evidence

- Finance inputs/results may contain sensitive financial information and must remain tenant/institution scoped.
- Authorization context is supplied by the owning workflow; A07-002 does not broaden access.
- Integrity/evidence operations use approved D03/D04 capabilities through A07-001.
- Diagnostics must avoid unnecessary exposure of sensitive monetary details while retaining enough correlation for investigation.
- No direct ledger/database mutation is permitted from the computation boundary.

## 10. Observability & Operability

A completed or failed calculation attempt should expose technical evidence for: correlation identity, rule/version reference, authoritative input snapshot/reference, currency context, reconciliation outcome, failure class, predecessor/recalculation link where applicable, and evidence handoff status. Numeric latency/throughput targets remain governed downstream.

## 11. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-020 | Finance computation consumes authoritative monetary/rule semantics rather than hardcoding them. | Repository-corrected architecture position |
| AD-A07-021 | Reconciliation is mandatory where the parent workflow provides an invariant to be checked. | L4-derived |
| AD-A07-022 | Calculation and canonical posting remain separate responsibilities. | Corpus/L4-derived |
| AD-A07-023 | Recalculation preserves prior technical lineage rather than replacing prior evidence. | Repository-consistent technical derivation |
| OD-A07-020 | Concrete numeric type, precision, scale, rounding implementation and performance targets remain unresolved until governed by authoritative financial/technical decisions. | Open decision |

## 12. Conformance Requirements

1. No fixed decimal precision, rounding mode, residual rule or concession sequence may be imposed without authoritative source.
2. Exact approved fee/rule versions must be bound to each completed calculation.
3. The engine shall not post ledger entries or approve charges/concessions.
4. Reconciliation failures shall be returned to the owning workflow rather than silently adjusted by invented policy.
5. Missing required monetary policy shall fail closed.
6. Currency context shall not be inferred when ambiguous.
7. Recalculation shall preserve lineage where authoritative evidence requirements apply.
8. Bulk/asynchronous execution shall delegate scheduling/retry to A08.

## 13. Architecture Acceptance Evidence

A conforming design must demonstrate that a calculation can be reproduced from its authoritative rule/input references, cannot proceed with ambiguous currency/policy context, cannot post canonical Finance state directly, and cannot silently repair reconciliation differences.

## 14. Traceability Summary

`L4-F-001-BH-004` supports financial calculation from an applicable Fee Structure. Other referenced L4 Finance specifications remain authoritative for exact fee, concession, installment and other business semantics; this engine consumes their resolved rules rather than redefining them.