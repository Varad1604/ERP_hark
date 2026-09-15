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
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-A-07-001; L5-A-07-005; L5-A-08-001; L5-B-02-001; L5-D-03-001; L5-E-04-001; L4-E-053; L4-E-055; L4-E-056 |
| Referenced By | People & Workforce Payroll Management; Finance Disbursement workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution architecture for the Payroll Computation Engine within SutraERP. The engine executes governed salary calculations, statutory deductions (tax, provident fund, insurance), attendance/leave prorations, retroactive salary adjustments, and final separation settlements, while strictly ensuring that the People & Workforce domain retains full business ownership of Payroll approval and the Finance domain retains sole authority for monetary disbursement and general ledger posting.

### In Scope

- Execution of approved payroll rule definitions, salary structures, allowance matrices, and statutory deduction schedules.
- Gross-to-net salary calculation applying attendance penalties, unpaid leave prorations, and tax withholding brackets.
- Multi-period retroactive salary differential calculation (retro-pay) preserving historical period baselines.
- Final settlement compensation computation for separated employees (gratuity, encashment, notice period recovery).
- Technical validation of calculated payroll results against wage protection invariants and minimum net pay constraints.
- Generation of immutable calculation lineage evidence records.

### Out of Scope / Non-Goals

- Authorizing or approving payroll runs (owned by Payroll Administrator / HR Authority in L4-E-053).
- Disbursing salary payments, bank transfer orchestration, or payroll ledger journal creation (owned by Finance).
- Determining workforce policy, salary band revisions, or attendance approvals.
- Modifying employee bank account details or tax declaration forms.
- Direct mutation of payroll run database entities.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-E-053` | Approved / Locked | `L4-E-053-BH-003` | Execution of governed payroll calculation from Calculating to Calculated | Core gross-to-net calculation pipeline |
| L4 | `L4-E-053` | Approved / Locked | `L4-E-053-BH-004` | Technical validation of calculated payroll results before approval | Statutory invariant and threshold validation |
| L4 | `L4-E-053` | Approved / Locked | `L4-E-053-BH-008` | Preservation of calculation references and rule versions for audit | Provenance and rule version binding |
| L4 | `L4-E-055` | Approved / Locked | `L4-E-055-BH-001` | Technical calculation context across affected historical payroll periods | Historical period baseline resolution |
| L4 | `L4-E-055` | Approved / Locked | `L4-E-055-BH-002` | Calculation of difference between processed payroll and retrospective change | Retroactive differential computation |
| L4 | `L4-E-055` | Approved / Locked | `L4-E-055-BH-003` | Production of retro-pay results without editing historical finalized payroll | Non-destructive retroactive adjustment |
| L4 | `L4-E-055` | Approved / Locked | `L4-E-055-BH-004` | Technical validation of retroactive calculation prior to payroll approval | Retroactive differential validation |
| L4 | `L4-E-056` | Approved / Locked | `L4-E-056-BH-003` | Execution of employee final settlement payroll calculations | Separation settlement calculation pipeline |
| Decision | Corpus Register | Locked | `L5-A-07-004` | Executes payroll calculations; Payroll retains business ownership | Canonical technical responsibility |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Payroll calculations shall never overwrite finalized historical payroll runs. | L4-E-055-BH-003 | Retroactive adjustments generate separate positive/negative adjustment lines in the active run. |
| C-002 | Total statutory and voluntary deductions shall never reduce net pay below statutory wage minimums. | Statutory Wage Protection | Negative net pay is strictly prohibited; deduction caps enforce minimum take-home pay. |
| C-003 | The engine carries status `CALCULATED_UNAPPROVED` and cannot disburse funds. | L5 Candidate Register §3.1 | Financial disbursement requires explicit downstream Finance authorization. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Workforce workflows provide certified attendance totals and approved compensation structures. | Engine does not parse biometric swipe logs directly. | Missing attendance data halts calculation for affected employee. | Context validated during admission gate. |
| AS-002 | Statutory tax and pension rules are authored as declarative formula packages in L5-A-07-005. | Ensures separation of tax logic from engine code. | Tax formula errors trapped during rule package binding. | Formulas verified against statutory tax tables. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Pure gross-to-net payroll calculation algorithms (earnings, deductions, statutory withholdings, employer contributions).
- Proration algorithms for mid-month joiners, leavers, and unpaid leaves.
- Multi-period retrospective wage recalculation and differential aggregation.
- Final settlement calculations (leave encashment, statutory gratuity, notice period shortfall recovery).
- Technical wage protection limit checking and negative net pay prevention.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-A-07-001` | Secure Computational Engine Baseline | Execution framework | Inherits admission gate, sandbox execution, and lineage hashing. |
| `L5-A-07-005` | General Rules & Formula Engine | Formula evaluation | Evaluates declarative tax slabs, allowance formulas, and deduction tiers. |
| `L5-A-08-001` | Background Processing & Job Execution | Asynchronous work-unit | Orchestrates large-scale monthly payroll batch runs (ADR-L5-0013). |
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Persistence handoff | Stores calculation evidence records immutably (ADR-L5-0005). |
| `L5-D-03-001` | Canonical Trust Boundary & Segmentation | Security boundary | Enforces strict encryption and isolation for sensitive salary information. |
| `L5-E-04-001` | Runtime Packaging & Deployment Baseline | Worker execution | Deploys payroll workers in isolated, resource-controlled runtimes. |

### 4.3 Must Not Own

- Approving or locking payroll runs (owned by HR/Payroll management).
- Authorizing bank disbursement files or initiating NEFT/RTGS payments (owned by Finance).
- Creating General Ledger payroll accrual or bank credit journal vouchers.
- Managing employee attendance disputes or leave approval workflows.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `PayrollAdmissionGate` | Control Point | Validates employee calculation eligibility, period lock state, and attendance completeness. | `PayrollCalculationRequest` | Stateless | Admitted payroll envelope | `L5-A-07-001` | Rejects uncertified attendance or unlocked prior periods. | L4-E-053-BH-003 |
| `SalaryStructureBinder` | Component | Resolves approved salary structure, allowance rules, and statutory tax tables. | Structure ID & Version | Cached structure AST | Bound payroll calculation plan | `L5-A-07-005` | Fails on unapproved salary structure version. | L4-E-053-BH-008 |
| `GrossToNetPipeline` | Component | Computes Earnings -> Prorations -> Statutory Deductions -> Voluntary Deductions -> Net Pay. | Bound plan & employee inputs | Ephemeral decimal registers | `EmployeePayrollItem` | `L5-A-07-001` | Enforces minimum wage protection and non-negative net pay. | C-002, L4-E-053-BH-004 |
| `RetroPayCalculator` | Component | Evaluates historical period differences under retrospective revisions and outputs adjustment items. | Historical period records & new rules | Ephemeral registers | `List<RetroAdjustmentLine>` | `L5-A-07-001` | Prevents modification of historical finalized run tables. | C-001, L4-E-055-BH-002 |
| `FinalSettlementCalculator` | Component | Computes terminal benefits: leave encashment, gratuity, notice adjustments, asset deductions. | Separation context & service history | Ephemeral registers | `SettlementPayrollBreakdown` | `L5-A-07-001` | Rejects calculation if mandatory exit clearances are missing. | L4-E-056-BH-003 |

### 5.2 Primary Technical Flow

```text
Monthly Payroll Workflow (L4-E-053)
         ↓
PayrollAdmissionGate (verify Period Open, Certified Attendance, Active Structure)
         ↓
SalaryStructureBinder (bind approved Salary Package & Tax Slabs via L5-A-07-005)
         ↓
GrossToNetPipeline (Base Earnings + Allowances - Attendance Prorations = Gross)
         ↓
DeductionsEngine (Gross - Tax - Provident Fund - Insurance = Net Pay)
         ↓
RetroPayCalculator (if retrospective revisions exist: calculate net differential lines)
         ↓
WageProtectionVerifier (Verify Net Pay >= Statutory Minimum Take-Home)
         ↓
LineageEvidenceEmitter (emit SHA-256 evidence record via L5-A-07-001)
         ↓
Return CalculatedPayrollResult to HR Payroll Manager (Status: CALCULATED_UNAPPROVED)
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Uncertified Attendance Context | `PayrollAdmissionGate` | Rejection of employee calculation (`ERR_ATTENDANCE_UNCERTIFIED`). | Processing payroll with incomplete or estimated attendance. | Attendance supervisor must certify timesheet. | L4-E-053-BH-003 |
| Negative Net Pay Condition | `GrossToNetPipeline` | Trapped condition (`ERR_NEGATIVE_NET_PAY`); cap voluntary deductions. | Emitting a payroll slip requiring employee payment without legal attachment. | HR must review voluntary loan deductions. | C-002 |
| Missing Historical Baseline | `RetroPayCalculator` | Halt retroactive calculation (`ERR_HISTORICAL_BASELINE_MISSING`). | Calculating retro-pay based on estimated previous earnings. | System administrator must reconcile prior period evidence. | L4-E-055-BH-001 |
| Final Settlement Unresolved Clearance | `FinalSettlementCalculator` | Rejection (`ERR_CLEARANCE_PENDING`). | Emitting final settlement while asset/recovery amounts are undefined. | Separation workflow must finalize inventory deductions. | L4-E-056-BH-003 |

### 5.4 Conceptual Contracts

#### `PayrollCalculationRequest`
```text
PayrollCalculationRequest {
  TenantId: UUID
  InstitutionId: UUID
  CalculationCorrelationId: UUID
  EmployeeId: UUID
  PayrollPeriodId: String (e.g., "2026-09")
  SalaryStructureId: String
  SalaryStructureVersion: String
  AttendanceContext: {
    PayableDays: Decimal(5, 2)
    UnpaidLeaveDays: Decimal(5, 2)
    OvertimeHours: Decimal(5, 2)
  }
  TaxDeclarationContext: Map<String, Any>
  RetrospectiveChanges: Optional<List<RetrospectiveAdjustmentRequest>>
  IsFinalSettlement: Boolean
}
```

#### `PayrollCalculationResult`
```text
PayrollCalculationResult {
  CalculationCorrelationId: UUID
  EvidenceRecordId: UUID
  EmployeeId: UUID
  PayrollPeriodId: String
  GrossEarnings: Decimal(18, 4)
  TotalDeductions: Decimal(18, 4)
  NetSalaryPayable: Decimal(18, 4)
  EarningsBreakdown: List<SalaryComponentAmount>
  DeductionsBreakdown: List<SalaryComponentAmount>
  RetroactiveAdjustments: List<RetroAdjustmentLine>
  CalculationLineageHash: String (SHA-256)
  CalculationStatus: Enum [SUCCESS, FAILED_WAGE_PROTECTION, FAILED_ADMISSION]
}
```

### 5.5 State, Persistence & Consistency Boundaries

The Payroll Computation Engine is strictly **stateless**. It does not persist salary registers or update employee bank transfer tables. Calculation results are emitted as unapproved candidate records. Evidence records are durably persisted in `L5-B-02` via `L5-A-07-001`. Recalculations append new evidence records explicitly referencing prior attempt hashes.

### 5.6 Asynchronous & Job Realization

Institution-wide payroll runs (thousands of employees) are executed as distributed background batches partitioned by department, orchestrated via `L5-A-08-001` under `ADR-L5-0013`. Each employee calculation constitutes an idempotent sub-work-unit.

### 5.7 Security, Trust & Isolation

- Employee salary data is classified as Confidential / High-Sensitivity under `L5-B-05-002`.
- Ephemeral memory containing salary calculations is wiped immediately upon sandbox termination.
- Calculation logs omit sensitive bank account numbers and tax identifier numbers.

### 5.8 Runtime & Deployment Implications

Deploys as part of the secure computational worker pool (`L5-E-04-001`). Workers are auto-scaled during month-end payroll cycles and isolated from public internet ingress.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-A07-040` | Non-Destructive Retroactive Adjustment | Retroactive salary changes never alter historical finalized payroll; differences are computed and emitted as current adjustment items. | L4-E-055-BH-003; C-001 | Preserves immutable historical tax filings, accounting closes, and audit trails. | None (Local L5 Architecture Decision) |
| `AD-A07-041` | Strict Wage Protection Floor | Voluntary deductions are capped dynamically to ensure net take-home salary never falls below statutory limits. | Statutory labor standards; C-002 | Eliminates illegal negative net pay scenarios in automated execution. | None (Local L5 Architecture Decision) |
| `AD-A07-042` | Separation of Payroll Calculation from Disbursement | Engine produces `CALCULATED_UNAPPROVED` output; disbursement requires explicit HR approval and Finance execution. | Corpus Register L5-A-07-004; C-003 | Protects organizational financial controls and segregation of duties. | Governed by ADR-L5-0005 |
| `AD-A07-043` | Declarative Statutory Tax Binding | Income tax and statutory pension deductions are evaluated through immutable declarative formula ASTs. | L5-A-07-005 Baseline | Allows statutory tax rule updates without modifying core payroll engine application code. | None (Local L5 Architecture Decision) |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Salary Confidentiality | Ephemeral memory sanitization; strict masking of personal financial identifiers. | Privacy legislation | `L5-B-05-002`, `L5-D-03-002` |
| Calculation Accuracy | Fixed-point decimal arithmetic across all earnings, deductions, and tax calculations. | Labor law compliance | `L5-A-07-004` |
| Audit Reproducibility | Lineage hash binds exact salary structure and tax table versions to every pay slip calculation. | Tax authority audit standard | `L5-D-04-003` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | The implementation shall admit payroll calculations only after validating certified attendance context and active salary structure. | §5.1 | `L4-E-053-BH-003` |
| `CR-002` | All calculations shall use fixed-point decimal arithmetic with a minimum of 4 fractional decimal places internally. | §5.1 | `L5-A-07-002` Baseline |
| `CR-003` | The engine shall enforce that net pay is non-negative and complies with statutory minimum take-home pay floors. | §5.1, `AD-A07-041` | C-002 |
| `CR-004` | Retrospective salary adjustments shall be calculated as differentials without modifying historical finalized payroll runs. | §5.1, `AD-A07-040` | `L4-E-055-BH-003`, C-001 |
| `CR-005` | Final settlement calculations shall evaluate leave encashment, gratuity, and notice period adjustments strictly according to policy rules. | §5.1 | `L4-E-056-BH-003` |
| `CR-006` | The engine shall emit an immutable evidence record binding salary structure version and calculated amounts via `L5-A-07-001`. | §5.1, §5.4 | `L4-E-053-BH-008` |
| `CR-007` | The engine shall not possess authority or capability to update employee bank balances or approve payroll runs. | §4.3, `AD-A07-042` | C-003 |
| `CR-008` | In recalculation cycles, historical payroll calculation evidence records shall remain intact and linked via prior attempt hashes. | §5.5 | `L4-E-055-BH-007` |
| `CR-009` | The implementation shall trap attendance discrepancies or missing tax slabs and return structured diagnostic errors. | §5.3 | `L4-E-053-BH-004` |
| `CR-010` | Large-scale monthly batch payroll calculations shall be partitioned and orchestrated via `L5-A-08-001` under `ADR-L5-0013`. | §5.6 | ADR-L5-0013 |
| `CR-011` | Ephemeral calculation memory shall be sanitized immediately upon completion of each employee calculation. | §5.7 | `L5-B-05-002` |
| `CR-012` | Statutory deduction formulas shall be executed via the declarative AST engine provided by `L5-A-07-005`. | §5.1, `AD-A07-043` | `L5-A-07-005` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Statutory Tax Slab Update Packaging Model | Architecture / Governance | Procedure for deploying annual statutory tax revisions | Payroll Lead / Architecture Lead | No | Tax updates packaged as signed L5-A-07-005 declarative formula bundles. |
| `OD-002` | Multi-Country Labor Law Parameterization | Extensibility | Country-specific deduction ordering algorithms | Internationalization Architecture | No | Enforced through country-specific salary structure plugins in L5-A-06. |
| `OD-003` | Direct Bank Disbursement File Format Packaging | Integration | Bank-specific payment file schemas (NACH/ISO20022) | Finance Domain / L5-C-03 | No | Owned by downstream Finance and integration connectors. |

---

## 10. Traceability Summary

- `L4-E-053-BH-003` — Realized by `PayrollAdmissionGate` and `GrossToNetPipeline` executing payroll calculations (§5.1, `CR-001`).
- `L4-E-053-BH-004` — Realized by technical validation of payroll results against wage protection invariants (§5.1, `CR-003`).
- `L4-E-053-BH-008` — Realized by `LineageEvidenceEmitter` binding rule versions and calculation inputs (§5.1, `CR-006`).
- `L4-E-055-BH-001` — Realized by historical period context resolution during retrospective adjustments (§5.1).
- `L4-E-055-BH-002` — Realized by `RetroPayCalculator` evaluating differential amounts between historical and revised terms (§5.1, `CR-004`).
- `L4-E-055-BH-003` — Realized by emitting non-destructive retro-adjustment lines without mutating past runs (§5.1, `CR-004`).
- `L4-E-055-BH-004` — Realized by validation of retroactive calculations prior to payroll approval progression (§5.1).
- `L4-E-055-BH-007` — Realized by immutable preservation of retroactive calculation attempt lineage (§5.5, `CR-008`).
- `L4-E-056-BH-003` — Realized by `FinalSettlementCalculator` evaluating employee separation compensation (§5.1, `CR-005`).
