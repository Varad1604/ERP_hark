# L2-F12 — Asset Financial Accounting Workflows

---

# Document Metadata

| Attribute | Value |
|-----------|-------|
| Document ID | L2-F12 |
| Document Name | Asset Financial Accounting Workflows |
| Layer | Level 2 — Domain Workflow Architecture |
| Domain | Finance — Asset Accounting |
| Owner | Finance Architecture Team |
| Depends On | L2-F00 Finance Domain Model & Foundation, L2-F05 General Ledger, L2-F09 Financial Period & Closing, Section J |
| Referenced By | Financial Reporting, Accounts Payable, Treasury, Section J, Audit, Analytics |
| Status | Draft |
| Version | 1.1.0 |

---

# 1. Purpose

This document defines the Finance-side accounting treatment of institutional assets whose operational lifecycle is owned by **Section J — Enterprise Resource Management**.

Finance does not own the operational Asset Register.

Finance owns financial asset records, capitalization accounting, depreciation accounting, valuation accounting, impairment accounting, disposal accounting, write-off accounting, and financial reporting consequences.

---

# 2. Scope

This architecture defines:

- Financial asset accounting profile.
- Capitalization accounting decision.
- Depreciation accounting lifecycle.
- Revaluation and impairment accounting.
- Asset accounting ledger integration.
- Financial disposal and write-off accounting.
- Financial reporting views for assets.
- Finance-side reconciliation against Section J asset facts.

---

# 3. Explicit Non-Ownership

The following capabilities remain outside Finance Asset Financial Accounting.

| Capability | Canonical Owner |
|------------|-----------------|
| Operational Asset Register | J08 — Asset Lifecycle Management |
| Asset identity and tag governance | J08 — Asset Lifecycle Management |
| Asset custody and assignment | J08 — Asset Lifecycle Management |
| Asset operational transfer | J08 — Asset Lifecycle Management |
| Asset condition and warranty tracking | J08 — Asset Lifecycle Management |
| Asset retirement business decision | J08/J10 according to Section J policy |
| Physical disposal execution evidence | J10 — Disposal, Scrap & Write-Off |
| Inventory stock and item master | J06 — Inventory & Warehouse Management |
| Goods receipt and inspection | J05 — Goods Receipt, Inspection & Acceptance |
| Purchase orders | J04 — Purchase Order Management |
| Supplier master | J02 — Vendor & Supplier Management |

Finance shall not maintain an independent operational asset lifecycle or duplicate asset custody state.

---

# 4. Finance Ownership

Finance is the canonical owner of:

- Financial Asset Accounting Record.
- Capitalization Accounting Status.
- Book Value.
- Depreciation Method and depreciation accounting schedule.
- Depreciation postings.
- Revaluation accounting.
- Impairment accounting.
- Disposal accounting.
- Write-off accounting.
- Asset accounting ledger references.
- Asset financial reporting facts.

---

# 5. Integration with Section J

Finance consumes operational asset facts from Section J.

Representative consumed facts include:

- AssetRegistered
- AssetCapitalizationRequested
- AssetAssigned
- AssetTransferred
- AssetVerified
- AssetRetired
- DisposalApproved
- DisposalExecuted
- WriteOffApproved

Finance may publish financial outcomes such as:

- AssetCapitalizationCompleted
- DepreciationPosted
- AssetRevaluationPosted
- AssetImpairmentPosted
- AssetDisposalAccountingCompleted
- AssetWriteOffAccountingCompleted

Finance outcomes shall reference the Section J asset identifier but shall not modify Section J asset state.

---

# 6. Authorization, Audit, and Governance

Financial asset accounting operations are governed by L2-A04 Authorization & Policy and recorded through L2-A07 Audit Infrastructure.

Finance shall preserve:

- Tenant and institution context.
- Section J operational asset reference.
- Accounting period.
- Actor and approval authority.
- Source operational event reference.
- General Ledger posting reference.
- Reversal or correction reference where applicable.

Finance users shall not gain operational asset custody, assignment, transfer, or disposal authority through financial accounting permissions.

---

# 7. Completion Criteria

This document is complete when:

- It contains no claim to own operational asset lifecycle state.
- Finance ownership is restricted to monetary and accounting treatment.
- Every operational asset reference points to Section J/J08 or J10 as applicable.
- Financial postings are traceable to Section J operational facts.
- Audit, authorization, period governance, and GL integration remain explicit.

---

# 8. Final Architecture Statement

L2-F12 owns asset financial accounting.

Section J owns the operational asset lifecycle.

Finance and Section J collaborate through stable contracts and immutable events without shared ownership.
