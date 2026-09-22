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
| Version | 1.1.1 |
| Status | Locked |
| Last Updated | 2026-09-22 |
| Depends On | L5-A-07-001; L5-A-07-005; L5-B-02-001; L5-D-03-001; L5-A-08-001; L4-F-001; L4-F-005; L4-F-009; L4-F-012; L5 governance/template/validation standards |
| Referenced By | Student billing and finance workflows requiring governed calculation |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014; ADR-L5-0032; ADR-L5-0045 |

## 2. Purpose & Boundary

Provides pure financial computation over authoritative fee structures, installment definitions, concession rules, penalty rules and other financial parameters supplied by owning Finance workflows. It returns calculation results and evidence; it does not approve fee policy, determine concession precedence, create tax rules, post ledger entries or mutate student-account state.

`L4-F-001-BH-004` establishes that, when charging is applicable, the system resolves the applicable Fee Structure and calculates the monetary value of Student Charge. This specification realizes only the technical calculation portion after the authoritative structure/policy inputs have been resolved.

## 3. Authoritative Constraints

- Monetary representation, precision, scale and rounding mode are governed inputs from authoritative financial policy or downstream governed technical design; this document does not invent decimal precision or rounding defaults.
- Installment allocation follows the authoritative installment definition. Any residual-allocation rule must come from that authoritative definition; it is not hardcoded here.
- Concession, scholarship, tax, surcharge and penalty precedence is supplied by authoritative rules. No local sequence is defined.
- Calculation output is a proposal/result for the owning workflow, never a posted transaction.

## 4. Technical Realization

| Element | Responsibility |
|---|---|
| `FinanceCalculationAdmission` | Validate tenant/context, calculation type, currency consistency and references to authoritative fee/rule inputs. |
| `FinancialRuleBinder` | Bind exact approved/versioned financial rule package and parameters. |
| `MonetaryEvaluationPipeline` | Execute the bound arithmetic using a numeric representation that satisfies the authoritative monetary contract. |
| `ReconciliationVerifier` | Verify caller-supplied mathematical invariants such as component/total and installment/total equality. |
| `FinancialEvidenceAssembler` | Record exact rule/version/input/output lineage through A07-001. |
| `FinanceResultHandoff` | Return calculation result to the owning Finance workflow without posting it. |

Failure is closed: missing policy/rule input, ambiguous currency, invariant mismatch or deterministic formula error rejects the calculation. Transient infrastructure failures may be retried only through A08.

## 5. Ownership

Owns calculation execution mechanics and technical reconciliation. Does not own fee structures, concession eligibility, concession precedence, statutory/tax policy, installment policy, approval, ledger posting, invoice lifecycle or payment collection.

## 6. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-020 | Finance computation consumes authoritative monetary/rule semantics rather than hardcoding them. | Repository-corrected architecture position |
| AD-A07-021 | Reconciliation is mandatory where the parent workflow provides an invariant to be checked. | L4-derived |
| AD-A07-022 | Calculation and canonical posting remain separate responsibilities. | Corpus/L4-derived |
| OD-A07-020 | **Numeric representation resolved:** `rust_decimal::Decimal`. Finance-owned precision/scale/rounding semantics and measured performance targets remain authoritative/profile-governed. | `ADR-L5-0032`; `ADR-L5-0045` |

## 7. Conformance Requirements

1. No fixed decimal precision, rounding mode, residual rule or concession sequence may be imposed without authoritative source.
2. Exact approved fee/rule versions must be bound to each completed calculation.
3. The engine shall not post ledger entries or approve charges/concessions.
4. Reconciliation failures shall be returned to the owning workflow rather than silently adjusted by an invented policy.
5. Bulk execution shall delegate scheduling/retry to A08.

## 8. Traceability Summary

`L4-F-001-BH-004` supports financial calculation from an applicable Fee Structure. Other referenced L4 Finance specifications remain authoritative for their business semantics; this engine consumes their resolved rules rather than redefining them.

## 2026-09-22 Controlled Technology Amendment

This Locked specification is amended and re-locked under `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`. Only implementation technology is changed; upstream business architecture is unchanged.


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0032`  
`rust_decimal::Decimal` is the default authoritative Rust monetary scalar. Precision, scale, minor-unit and rounding semantics remain authoritative rule inputs; no global default is invented.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

`rust_decimal::Decimal` is now the technical monetary representation. No finance-owned rounding/precision rule or universal performance number is invented by this synchronization.
