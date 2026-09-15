# L5-A-07-002 — Finance Computation Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-002 |
| Specification Title | Finance Computation Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Executes approved/versioned financial calculations; cannot autonomously approve/post canonical financial transactions. |
| Technical Architecture Owner | Finance Computation Engine Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-A-07-001; L5-A-07-005; L5-B-02-001; L5-B-02-003; L5-D-03-001; L5-E-04-001; L4-F-001; L4-F-005; L4-F-009; L4-F-012 |
| Referenced By | Student Charge & Fee Management; Financial Accounting; Student Billing workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0006; ADR-L5-0013 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution architecture for executing governed financial calculations within SutraERP. It realizes pure financial calculations—including student fee schedule composition, concession evaluations, penalty and interest assessments, installment breakdowns, and currency scaling—while strictly prohibiting the engine from autonomously approving charges, posting general ledger journal entries, or altering financial transaction states.

### In Scope

- Execution of approved and versioned financial fee, charge, discount, penalty, and tax formulas.
- Precise numerical calculation preserving currency-specific decimal scale and rounding semantics.
- Multi-component charge composition and reconciliation differential calculation.
- Lineage evidence generation binding calculated monetary amounts to exact fee structure versions.
- Technical validation of calculated amounts against maximum threshold invariants.

### Out of Scope / Non-Goals

- Approving student fee structures, tuition rates, or financial concessions (owned by Finance business domain).
- Posting journal vouchers or modifying General Ledger balances (owned by Financial Accounting).
- Payment gateway integration or bank reconciliation (owned by L5-C-03 and Finance).
- Currency exchange rate market polling.
- Direct mutation of invoice or student charge database tables.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-F-001` | Approved / Locked | `L4-F-001-BH-004` | Calculation of published student charge from approved fee structure | Fee calculation pipeline realization |
| L4 | `L4-F-005` | Approved / Locked | `L4-F-005-BH-003` | Calculation of installment schedule amounts and due-date splits | Installment numerical composition |
| L4 | `L4-F-005` | Approved / Locked | `L4-F-005-BH-004` | Reconciliation validation that sum of installments equals total charge | Mathematical invariant verification |
| L4 | `L4-F-005` | Approved / Locked | `L4-F-005-BH-005` | Numerical reconciliation of fee adjustments against established charge | Adjustment differential calculation |
| L4 | `L4-F-009` | Approved / Locked | `L4-F-009-BH-002` | Calculation of late payment penalty and overdue interest | Penalty formula evaluation |
| L4 | `L4-F-012` | Approved / Locked | `L4-F-012-BH-003` | Calculation of fee concession and scholarship deduction amounts | Concession deduction evaluation |
| Decision | Corpus Register | Locked | `L5-A-07-002` | Executes financial calculations; cannot approve or post transactions | Canonical technical responsibility |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Monetary values shall use exact decimal arithmetic (never floating point). | SutraERP Financial Constitution | All internal calculations enforce fixed-point decimal scaling (minimum 4 decimal places internal, 2 decimal places display). |
| C-002 | The Finance Engine shall never autonomously post accounting entries. | L5 Candidate Register §3.1 | Results are returned to Student Billing / Accounting as uncommitted calculation proposals. |
| C-003 | Rounding methods (Half-Up, Bankers) must be explicitly bound to institutional policy. | Institutional Financial Standards | Rounding mode is injected via `FeeCalculationPolicy` context, never hardcoded. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Calling finance workflows provide authenticated student/fee context. | Engine does not perform database lookups. | Missing fee parameters causes immediate calculation abort. | Input context validated before calculation admission. |
| AS-002 | Declarative fee calculation formulas are authored and validated via L5-A-07-005. | Eliminates duplicate formula parsing logic. | Syntax errors caught during rule package build. | Formula rules are resolved through L5-A-07-005 package binder. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Pure financial calculation algorithms (fee composition, installment splits, tax calculation, concession deductions).
- High-precision monetary scale and rounding enforcement.
- Component-level fee reconciliation (sum of components == total fee; sum of installments == net charge).
- Financial calculation evidence assembly and lineage hash emission.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-A-07-001` | Secure Computational Engine Baseline | Execution framework | Inherits admission gate, sandbox execution, and lineage hashing. |
| `L5-A-07-005` | General Rules & Formula Engine | Formula evaluation | Executes declarative concession and fee tier formulas. |
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Persistence handoff | Stores calculation evidence records immutably (ADR-L5-0005). |
| `L5-D-03-001` | Canonical Trust Boundary & Segmentation | Security boundary | Enforces isolation between financial calculations and external services. |
| `L5-E-04-001` | Runtime Packaging & Deployment Baseline | Worker execution | Packages finance computation workers as isolated, stateless runtimes. |

### 4.3 Must Not Own

- Approving or waiving student fees, penalties, or fines.
- General Ledger journal creation, account reconciliation, or bank deposit ledger.
- Invoicing, receipt generation, or payment collection.
- Tax policy determination or legal tax schedule authority.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `FinanceAdmissionGate` | Control Point | Validates financial calculation request, currency consistency, and tenant context. | `FinancialCalculationRequest` | Stateless | Admitted calculation envelope | `L5-A-07-001` | Rejects mismatched currency or negative base amounts. | L4-F-001-BH-004 |
| `FeeStructureBinder` | Component | Resolves and validates approved fee schedule version and tax rate tables. | Fee structure ID & Version | Cached fee schedule AST | Bound fee calculation plan | `L5-A-07-005` | Fails on unapproved or expired fee schedule version. | C-001 |
| `MonetaryCalculationPipeline` | Component | Executes sequential calculation: Base Fee -> Concessions -> Surcharges -> Taxes -> Rounding. | Bound fee plan & student parameters | Ephemeral decimal registers | `ComputedFinancialBreakdown` | `L5-A-07-001` | Traps arithmetic overflow or invalid rounding. | C-001, L4-F-012-BH-003 |
| `ReconciliationVerifier` | Control Point | Verifies mathematical equality: Sum(Components) == Gross; Gross - Concession + Tax == Net. | `ComputedFinancialBreakdown` | Stateless | Validated financial result | Invariant engine | Flags rounding discrepancies or split mismatches. | L4-F-005-BH-004 |
| `InstallmentCalculator` | Component | Splits net payable amount across approved installment schedule dates and percentages. | Net amount, installment policy | Ephemeral registers | `InstallmentScheduleResult` | `L5-A-07-001` | Fails if sum of installments does not equal net amount. | L4-F-005-BH-003 |

### 5.2 Primary Technical Flow

```text
Student Billing Workflow (L4-F-001)
         ↓
FinanceAdmissionGate (validate Currency, Tenant, Non-Negative Inputs)
         ↓
FeeStructureBinder (bind approved Fee Schedule & Tax Tables via L5-A-07-005)
         ↓
MonetaryCalculationPipeline (Fixed-Point Decimal: Base - Concession + Tax)
         ↓
ReconciliationVerifier (Verify Component Sum == Total Net Payable)
         ↓
InstallmentCalculator (if applicable: compute installment splits and round cent residuals)
         ↓
LineageEvidenceEmitter (emit SHA-256 evidence record via L5-A-07-001)
         ↓
Return FinancialCalculationResult to Billing Workflow (Status: CALCULATED_UNPOSTED)
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Currency Code Mismatch | `FinanceAdmissionGate` | Calculation rejected (`ERR_CURRENCY_MISMATCH`). | Cross-currency calculation without explicit exchange rate. | Calling workflow must normalize currency. | C-001 |
| Unapproved Fee Schedule | `FeeStructureBinder` | Immediate abort (`ERR_FEE_SCHEDULE_UNAPPROVED`). | Calculation using draft or superseded fee structure. | Institutional Fee Administrator must approve structure. | L4-F-001-BH-004 |
| Component Sum Discrepancy | `ReconciliationVerifier` | Rejection (`ERR_RECONCILIATION_IMBALANCE`); emit audit log. | Emitting charge invoice where items do not sum to total. | Formula rule must adjust cent allocation logic. | L4-F-005-BH-004 |
| Installment Cent Allocation Residual | `InstallmentCalculator` | Assign residual rounding cent to final installment; ensure 100.00% equality. | Loss of cents or installment total exceeding total charge. | Deterministic residual adjustment rule. | L4-F-005-BH-003 |

### 5.4 Conceptual Contracts

#### `FinancialCalculationRequest`
```text
FinancialCalculationRequest {
  TenantId: UUID
  InstitutionId: UUID
  CalculationCorrelationId: UUID
  StudentId: UUID
  AcademicPeriodId: String
  FeeStructureId: String
  FeeStructureVersion: String
  CurrencyCode: String (ISO-4217, e.g., "INR")
  StudentAttributes: Map<String, Any> (e.g., category, domicile, merit score)
  InstallmentPlanRequested: Boolean
  InstallmentPlanId: Optional<String>
}
```

#### `FinancialCalculationResult`
```text
FinancialCalculationResult {
  CalculationCorrelationId: UUID
  EvidenceRecordId: UUID
  CurrencyCode: String
  GrossAmount: Decimal(18, 4)
  ConcessionAmount: Decimal(18, 4)
  TaxAmount: Decimal(18, 4)
  NetAmountPayable: Decimal(18, 4)
  LineItemBreakdown: List<FeeComponentBreakdown>
  InstallmentSchedule: Optional<List<InstallmentSplit>>
  CalculationLineageHash: String (SHA-256)
  CalculationStatus: Enum [SUCCESS, FAILED_RECONCILIATION, FAILED_ADMISSION]
}
```

### 5.5 State, Persistence & Consistency Boundaries

The Finance Computation Engine is strictly **stateless**. It does not persist financial ledgers or mutate student account balances. It persists only calculation evidence records into `L5-B-02` via `L5-A-07-001` lineage emission. Concurrency is handled by caller idempotency: calling workflows pass a `CalculationCorrelationId` to ensure identical requests do not produce redundant calculation records.

### 5.6 Asynchronous & Job Realization

Bulk billing runs (e.g., term tuition calculations for 10,000 students) are broken down into chunked work-units orchestrated by `L5-A-08-001` under `ADR-L5-0013`. Each work-unit processes an admitted slice of students independently and idempotently.

### 5.7 Security, Trust & Isolation

- High-precision monetary invariants are protected against memory alteration.
- Concession rules and scholarship formulas are isolated to prevent unauthorized student fee reductions.
- All monetary operations enforce strict non-negative constraints unless calculating explicit adjustment credit notes.

### 5.8 Runtime & Deployment Implications

Deploys as part of the secure computational worker pool (`L5-E-04-001`). Workers are auto-scaled during peak billing periods (semester enrollments) without requiring state replication.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-A07-020` | Fixed-Point Decimal Arithmetic | Mandatory 128-bit fixed-point decimal arithmetic (minimum 4 fractional decimal places internally). | Financial integrity baseline; C-001 | Eliminates IEEE 754 floating point precision loss and cent discrepancy errors. | None (Local L5 Architecture Decision) |
| `AD-A07-021` | Deterministic Cent Residual Allocation | Any fractional cent remainder resulting from installment division is automatically allocated to the final installment. | L4-F-005-BH-003 | Guarantees exact mathematical reconciliation: Sum(Installments) === TotalNetPayable. | None (Local L5 Architecture Decision) |
| `AD-A07-022` | Calculation vs Posting Separation | Finance Engine output carries status `CALCULATED_UNPOSTED` and cannot write General Ledger tables. | Corpus Register L5-A-07-002; C-002 | Protects accounting integrity; general ledger entries require authorized workflow approval. | Governed by ADR-L5-0005 |
| `AD-A07-023` | Multi-Tier Concession Application Sequence | Concessions are applied sequentially: Fixed Statutory -> Institutional Merit -> Discretionary Waivers. | L4-F-012-BH-003 | Prevents negative base calculation and enforces institutional scholarship precedence. | None (Local L5 Architecture Decision) |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Monetary Precision | Zero tolerance for rounding leakages or precision truncation across multi-currency calculations. | Financial auditing standard | `L5-A-07-002` |
| Tamper-Proof Lineage | Every fee calculation binds fee structure version and concession rules into a verifiable SHA-256 hash. | Tax & audit compliance | `L5-D-03-002`, `L5-D-04-003` |
| Tenant Isolation | Multi-tenant financial data strictly separated; fee structures cannot be shared across tenants. | Multi-tenant security baseline | `L5-A-04-001` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | The implementation shall enforce fixed-point decimal representation for all monetary calculations, prohibiting floating-point types. | §5.1, `AD-A07-020` | C-001 |
| `CR-002` | The engine shall verify that the fee structure referenced is approved and currently effective before admitting calculation. | §5.1 | `L4-F-001-BH-004` |
| `CR-003` | The implementation shall verify that the sum of all computed fee components equals the calculated gross charge. | §5.1, §5.3 | `L4-F-005-BH-004` |
| `CR-004` | When generating installment schedules, the sum of all installment amounts must exactly equal the total net payable amount. | §5.1, `AD-A07-021` | `L4-F-005-BH-003` |
| `CR-005` | Fractional cent residuals resulting from division shall be deterministically allocated to the final installment. | §5.1, `AD-A07-021` | `L4-F-005-BH-003` |
| `CR-006` | Concessions and scholarship deductions shall never reduce net payable amounts below zero unless processing an authorized credit note. | §5.1, `AD-A07-023` | `L4-F-012-BH-003` |
| `CR-007` | Late payment penalties and interest shall be calculated strictly against the outstanding overdue balance. | §5.1 | `L4-F-009-BH-002` |
| `CR-008` | The engine shall emit an immutable evidence record binding inputs, fee structure version, and output amounts via `L5-A-07-001`. | §5.1, §5.4 | `L4-F-001-BH-004` |
| `CR-009` | The engine shall not possess authority or capability to write to general ledger tables or approve student charges. | §4.3, `AD-A07-022` | C-002 |
| `CR-010` | In recalculation scenarios, historical fee calculations shall be retained and linked via prior calculation attempt hashes. | §5.5 | `L5-A-07-001` |
| `CR-011` | Inconsistent or unbalanced calculations shall be rejected with structured diagnostic codes and quarantined. | §5.3 | `L4-F-005-BH-004` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Exact Currency Rounding Mode Catalog | Governance | Repository-wide ISO currency rounding rules | Financial Architecture Lead | No | Define institutional rounding table in Stage 6 consolidation. |
| `OD-002` | Multi-Currency Real-Time Conversion | External Integration | Exchange rate provider integration | L5-C-03 Owner | No | Deferred to Stage 5 external integration architecture. |
| `OD-003` | Automated Fee Recalculation Trigger Policy | Workflow Integration | Event trigger policy upon retroactive fee change | Student Billing Domain Lead | No | Binding established during billing workflow realization. |

---

## 10. Traceability Summary

- `L4-F-001-BH-004` — Realized by `MonetaryCalculationPipeline` computing student charges from approved fee schedules (§5.1, `CR-001`, `CR-002`).
- `L4-F-005-BH-003` — Realized by `InstallmentCalculator` dividing net charge across installment schedule (§5.1, `CR-004`).
- `L4-F-005-BH-004` — Realized by `ReconciliationVerifier` ensuring installment sum equals total charge (§5.1, `CR-003`, `CR-004`).
- `L4-F-005-BH-005` — Realized by numerical reconciliation of fee adjustments against base charges (§5.1).
- `L4-F-009-BH-002` — Realized by penalty and overdue interest calculation logic (§5.1, `CR-007`).
- `L4-F-012-BH-003` — Realized by concession and scholarship evaluation pipeline (§5.1, `CR-006`).
