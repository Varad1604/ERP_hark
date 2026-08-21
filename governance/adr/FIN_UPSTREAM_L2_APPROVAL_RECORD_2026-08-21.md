# Finance Upstream L2 Architecture Approval Record — 2026-08-21

| Attribute | Value |
|---|---|
| Record Type | Architecture Review Decision / Repository Update Authorization |
| Date | 2026-08-21 |
| Branch | dhruv |
| Status | Approved and Incorporated |
| Approval Source | Explicit Lead Architect / repository-authority approval recorded in the architecture working session: “I approve changes in L2 go ahead and do it.” |

## 1. Purpose

This record closes the decision gate for the Finance upstream architecture remediation required by Level 3 Phase 5 authoring. It records approval of the recommended, repository-consistent resolutions and authorizes their incorporation into Locked Level 2 architecture.

This approval does not authorize unrelated architectural invention or implementation-level design.

## 2. Approved ADR Decisions

| ADR | Decision | Incorporated L2 effect |
|---|---|---|
| ADR-FIN-ADJ-002 | **Approved — Option A.** Preserve the specific L2-F00 Board approval requirement for Student-Charge write-off as the mandatory instance of the generic Enterprise Approval Authority. | L2-F04 v1.1.1 directly amended and re-locked. |
| ADR-FIN-GL-001 | **Approved — recommended FIN-GL remediation.** Resolve Journal rejection, Posting Batch failure/retry, reversal, reconciliation, Trial Balance, authority and close semantics; remove competing Financial Year/Period lifecycle ownership from F05. | Locked L2-F05 amendment v1.1.0. |
| ADR-FIN-AR-001 | **Approved — recommended AR clarification.** Add canonical business authorities; use only source-owned exact obligation events and do not invent absent source events. | Locked L2-F07 amendment v1.1.0. |
| ADR-FIN-AP-001 | **Approved with repository-source resolution.** Section J owns procurement facts; L2-F14 owns Finance-side `VendorInvoiceApproved` and `VendorLiabilityRecognized`; L2-F08 owns `PayableRegistered` and `PayableValidated`. | Locked L2-F08 v1.1.0 and L2-F14 v1.1.1 amendments. |
| ADR-FIN-PERIOD-001 | **Approved — canonical ownership clarified.** L2-F09 owns Financial Year and Financial Period lifecycle governance as distinct aggregates; Financial Period references one Financial Year and Financial Year may govern many periods. | Locked L2-F09 amendment v1.1.0. |
| ADR-FIN-TREAS-001 | **Approved — minimum Treasury Account lifecycle.** `Registered → Active → Closed`, with governed actors, separation of duties, canonical events and historical preservation. | Locked L2-F10 amendment v1.1.0. |

## 3. Canonical Amendment Files

The following files are authoritative parts of their target L2 documents. Where they explicitly conflict with an earlier target version, the amendment has precedence for the amended subject only; all unaffected target-document content remains authoritative.

- `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F04_Financial_Adjustment_Management.md` — v1.1.1
- `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` — v1.1.0 amendment
- `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` — v1.1.0 amendment
- `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md` — v1.1.0 amendment
- `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md` — v1.1.1 amendment
- `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` — v1.1.0 amendment
- `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md` — v1.1.0 amendment

## 4. Cross-Domain Ownership Decisions

1. Student-Charge write-off retains the specific Board approval rule; Financial Adjustment Management consumes the approval outcome and does not own approval execution.
2. L2-F09 is the canonical lifecycle-governance owner for Financial Year and Financial Period; L2-F05 consumes those lifecycle facts for posting eligibility.
3. Section J remains owner of procurement business facts and supplier/vendor procurement concepts.
4. L2-F14 owns the Finance procurement-consumption interpretation through vendor invoice approval and vendor liability recognition.
5. L2-F08 owns the Payable representation and AP lifecycle after `VendorLiabilityRecognized`.
6. L2-F07 owns Receivable representation only; source domains retain originating obligation ownership.
7. L2-F10 owns Treasury Account business truth and lifecycle; external banking infrastructure remains external.

## 5. Event Publisher Decisions

- `VendorInvoiceApproved` — L2-F14.
- `VendorLiabilityRecognized` — L2-F14.
- `PayableRegistered` — L2-F08.
- `PayableValidated` — L2-F08.
- `ChargeEstablished` — Student Charge Management; consumed by L2-F07.
- `FinancialYearOpened` / `FinancialYearClosed` — L2-F09 as Finance year-governance facts, preserving L2-F00 event terminology.
- `TreasuryAccountRegistered`, `TreasuryAccountActivated`, `TreasuryAccountClosed` — L2-F10.
- `LedgerReconciled` and `ReconciliationExceptionRaised` — L2-F05 Accounting & General Ledger.

## 6. Downstream Rule

Level 3 scenarios may now consume these approved L2 decisions but may not redefine their ownership, event publisher, authority or lifecycle semantics. Any remaining source-specific event absent from its canonical source remains an explicit upstream gap rather than an L3 invention.

## 7. Repository Update Boundary

This decision record covers only the Finance upstream L2 remediation described above. No Level 3 scenario content is approved or modified by this record. L3 synchronization is a separate controlled pass.
