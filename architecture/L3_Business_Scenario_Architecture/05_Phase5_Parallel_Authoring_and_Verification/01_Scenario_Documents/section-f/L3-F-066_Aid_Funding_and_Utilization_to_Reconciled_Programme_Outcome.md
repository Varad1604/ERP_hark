# Aid Funding and Utilization to Reconciled Programme Outcome

**Scenario ID:** L3-F-066  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Scholarship & Financial Aid  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F11 — Scholarship & Financial Aid; L2-F00 — Finance Domain Model & Foundation; applicable Financial Reporting and Audit architecture  
**Referenced By:** Financial Reporting; Audit; Executive Dashboards; Student Charge & Fee Management where assistance utilization affects downstream financial processing

## 2. Scenario Purpose

Govern the institutional reconciliation of financial-aid funding and utilization so that the approved funding source, allocated assistance, and recorded utilization remain traceable to a coherent programme-level financial assistance outcome.

The scenario reconciles business facts within Scholarship & Financial Aid. It does not create accounting entries or replace the responsibilities of downstream Finance domains.

## 3. Business Outcome

Funding assigned to financial assistance and the corresponding utilization history are reconciled into a complete, traceable programme outcome, with historical funding and utilization facts preserved and any variance represented through new business facts rather than mutation of prior history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a financial assistance programme has assigned funding and recorded utilization facts requiring programme-level reconciliation.

### 4.2 End Boundary

The scenario ends when the programme-level funding and utilization position is reconciled and its authoritative business facts are available for reporting, audit and downstream financial interpretation.

### 4.3 In Scope

- Funding Source assignment and history.
- Benefit utilization history.
- Comparison of approved funding context with recorded utilization.
- Programme-level reconciliation outcome.
- Historical traceability.
- Reporting and audit consumption.

### 4.4 Out of Scope

- General Ledger posting.
- Budget ownership.
- Treasury operations.
- Bank reconciliation.
- Student Charge ownership.
- Accounts Receivable ownership.
- Technical reconciliation jobs, databases, APIs or implementation mechanics.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| Scholarship & Financial Aid | Owns Funding Source assignment, Benefit Utilization and assistance history. | Canonical programme assistance ownership. |
| Financial Reporting / Audit | Consumes authoritative financial-assistance facts for reporting and evidence. | Reporting/audit records remain separately owned. |
| Budget Management | Remains owner of budget allocation where applicable. | Budget ownership is external. |
| General Ledger | Remains owner of accounting entries. | Accounting ownership is external. |

L2-F11 explicitly keeps Budget Allocation and General Ledger outside Scholarship & Financial Aid ownership. fileciteturn237file0

## 6. Business Actors, Roles and Responsibilities

L2-F11 does not define an Actors section. No finance analyst, scholarship accountant, programme manager, auditor, or reconciliation officer is invented.

The capability owns the reconciliation business outcome; external reporting and audit consumers use the resulting authoritative facts.

## 7. Preconditions and Entry Criteria

- A valid Funding Source is associated with allocated assistance.
- Benefit utilization has been recorded against active assistance.
- Funding history is available.
- Utilization history is available.
- The programme context can be identified.
- Historical records remain intact for reconciliation.

L2-F11 requires every allocated benefit to reference one valid Funding Source and every utilization record to reference active assistance. fileciteturn239file0

## 8. Trigger

A financial-assistance programme reaches the business point at which assigned funding and recorded utilization require reconciliation.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | Identify the programme's assigned Funding Source history. | Scholarship & Financial Aid | Funding position is established. |
| 2 | Identify recorded Benefit Utilization against the programme's assistance. | Scholarship & Financial Aid | Utilization position is established. |
| 3 | Validate that utilization references active assistance and that funding assignments remain valid. | Scholarship & Financial Aid | Reconciliation inputs satisfy canonical invariants. |
| 4 | Reconcile the programme's funding and utilization business facts. | Scholarship & Financial Aid | Programme-level financial-assistance position is established. |
| 5 | Preserve the funding and utilization histories used in the reconciliation. | Scholarship & Financial Aid | Historical reconstruction remains possible. |
| 6 | Record any reconciliation outcome as a new business fact without overwriting prior utilization or funding history. | Scholarship & Financial Aid | Variances or corrections remain traceable. |
| 7 | Publish applicable canonical funding/utilization events. | Scholarship & Financial Aid | Reporting and audit consumers receive authoritative facts. |
| 8 | Financial Reporting and Audit consume the authoritative facts for their own downstream responsibilities. | Reporting / Audit | Reporting and evidence remain separately owned. |

L2-F11 defines `BenefitUtilizationRecorded`, `BenefitUtilizationUpdated`, `FundingSourceAssigned` and `FundingSourceChanged` as immutable business facts with reconstructable history. fileciteturn238file0

## 10. Alternate Business Flows

### A1 — Funding and Utilization Reconcile

The programme reaches the expected reconciled outcome and the supporting funding/utilization facts remain available for reporting and audit.

### A2 — Utilization Exceeds the Governed Funding Context

The variance is not hidden by altering historical funding or utilization. A new business fact records the required corrective/reconciliation outcome under the applicable policy.

### A3 — Funding Source History Changes

A legitimate funding-source reassignment is recorded as a new business fact. Previous funding history remains preserved.

### A4 — Utilization Requires Correction

A valid utilization adjustment produces a new utilization business fact. Historical utilization remains reconstructable.

### A5 — Accounting Consequence Exists

Any accounting consequence is passed to the canonical General Ledger context. Scholarship & Financial Aid does not create journal entries.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Funding Source is invalid | Do not treat the programme funding position as reconciled. |
| Utilization does not reference active assistance | Reject the invalid utilization/reconciliation outcome. |
| Historical funding or utilization is incomplete | Preserve the known history and resolve the missing business evidence before claiming a complete reconciliation. |
| A variance is identified | Record a new corrective/reconciliation business fact; do not overwrite history. |
| Reporting consumer has a separate issue | Preserve the authoritative F11 facts; reporting/audit resolve their own downstream concern. |

L2-F11 requires business-rule violations to reject the command, preserves historical records, and requires corrections through new business facts. fileciteturn239file0

## 12. Business Rules and Decision Constraints

- Every allocated benefit references one valid Funding Source.
- Funding Source history is immutable.
- Utilization must reference active assistance.
- Utilization history is immutable.
- Utilization never modifies approval history.
- Historical information is never deleted, overwritten, reassigned or repurposed.
- Corrections occur through new business facts.
- Funding Source ownership remains governed within the Financial Aid context for assignment/history, while external funding owners remain external.
- Scholarship & Financial Aid does not own Budget Allocation or General Ledger accounting.
- Domain Events represent completed business facts and remain immutable. fileciteturn239file0turn238file0

## 13. State and Lifecycle Impact

The reconciliation scenario does not introduce a new Financial Aid lifecycle state.

Relevant canonical business facts include:

- Funding Source assigned or changed.
- Benefit Utilization recorded or legitimately updated.
- Assistance lifecycle state remains governed by the Financial Aid Aggregate.

The scenario's “reconciled programme outcome” is a business outcome derived from authoritative funding and utilization facts, not an invented Aggregate state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Events

Relevant F11 events include:

- `FundingSourceAssigned`
- `FundingSourceChanged`
- `BenefitUtilizationRecorded`
- `BenefitUtilizationUpdated`

The Financial Aid Aggregate is the canonical publisher. These events are immutable, reconstructable business facts. fileciteturn238file0

### 14.2 Cross-Domain Hand-offs

| From | To | Business Fact | Purpose |
|---|---|---|---|
| Scholarship & Financial Aid | Financial Reporting | Funding and utilization facts | Enables programme financial reporting. |
| Scholarship & Financial Aid | Audit | Funding and utilization history | Supports institutional evidence and reconstruction. |
| Scholarship & Financial Aid | Executive Dashboards | Approved financial-assistance facts | Supports programme-level management visibility. |
| Scholarship & Financial Aid | General Ledger | Applicable financial-assistance facts | Enables accounting under General Ledger ownership; F11 does not create journal entries. |

Events remain distinct from business contracts and synchronous hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

No reconciliation-specific human authority is defined in L2-F11. None is invented.

### 15.2 Approval / Separation of Duties

No scenario-specific separation-of-duty rule is defined in the source. None is created locally.

### 15.3 Audit Evidence

The reconciled programme outcome must remain traceable to:

- programme/assistance identity;
- Funding Source history;
- Benefit Allocation;
- utilization history;
- effective periods;
- relevant correction facts;
- published funding and utilization events;
- downstream reporting/audit references.

L2-F11 requires permanent traceability and immutable historical business events. fileciteturn238file0

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Funding Source history is established.
- Relevant utilization history is established.
- Funding and utilization satisfy applicable invariants.
- The programme-level reconciliation outcome is determined.
- Any variance/correction is represented through new business facts.
- Historical funding and utilization remain reconstructable.
- Applicable canonical events are available to downstream reporting and audit consumers.

### 16.2 Postconditions

- Funding history remains immutable.
- Utilization history remains immutable.
- The programme outcome is traceable to its underlying business facts.
- Reporting and audit can reconstruct the financial-assistance position.
- No budget, treasury or accounting ownership is transferred to Scholarship & Financial Aid.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

L2-F11 does not define a dedicated `Programme Reconciled` lifecycle state or a `ProgrammeReconciled` event. Therefore this scenario treats reconciliation as a business outcome derived from canonical funding and utilization facts rather than inventing a new state/event.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

If the enterprise requires a formal programme-level reconciliation aggregate, state machine, approval authority, or dedicated reconciliation event, those must be established in upstream architecture before being introduced at L3.
