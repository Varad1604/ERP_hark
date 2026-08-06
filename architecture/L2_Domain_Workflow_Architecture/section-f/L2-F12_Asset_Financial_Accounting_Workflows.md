# L2-F12 — Asset Financial Accounting Workflows

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §2 Scope |
| Actors | — not present — |
| Entities | Entities / Aggregates |
| Business Rules | Rules / Invariants |
| Workflow | Workflows |
| State Machine | States |
| Contracts | — not present — |
| Events | Events |
| Dependencies | — not present — |
| Locked Decisions | — not present — |

> **Gap:** 4 required section(s) have no counterpart in this document: Actors, Contracts, Dependencies, Locked Decisions. Recorded, not authored.

---

# Document Metadata

| Attribute | Value |
|-----------|-------|
| Document ID | L2-F12 |
| Document Name | Asset Financial Accounting Workflows |
| Layer | Level 2 — Domain Workflow Architecture |
| Domain | Finance — Asset Accounting |
| Review Authority | Finance Architecture Team |
| Document Owner | Finance — Asset Accounting |
| Depends On | L2-F00 Finance Domain Model & Foundation, L2-F05 General Ledger, L2-F09 Financial Period & Closing, Section J |
| Referenced By | Financial Reporting, Accounts Payable, Treasury, Section J, Audit, Analytics |
| Status | Trust Certification Ready |
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

---

## Lifecycle

This document does not define a single Finance-owned lifecycle state machine. Accounting-stage lifecycle content is distributed across Section 2 (Scope: capitalization accounting decision, depreciation accounting lifecycle, revaluation/impairment accounting, disposal and write-off accounting) and Section 5 (Integration with Section J), where consumed operational events — AssetRegistered, AssetCapitalizationRequested, AssetAssigned, AssetTransferred, AssetVerified, AssetRetired, DisposalApproved, DisposalExecuted, WriteOffApproved — mark the operational stages that drive Finance's accounting treatment. Section 3 confirms the operational asset lifecycle itself is owned by Section J / J08, while Section 4 confirms Finance owns only the accounting lifecycle stages (capitalization, depreciation, revaluation, impairment, disposal, write-off).

---

## Workflows

No section is titled "Workflows," but workflow-equivalent content appears in Section 2 (Scope — capitalization accounting decision, depreciation accounting lifecycle, revaluation and impairment accounting, financial disposal and write-off accounting, reconciliation against Section J asset facts) and Section 5 (Integration with Section J — Finance consumes operational asset facts and, in response, publishes financial outcomes such as AssetCapitalizationCompleted, DepreciationPosted, AssetRevaluationPosted, AssetImpairmentPosted, AssetDisposalAccountingCompleted, and AssetWriteOffAccountingCompleted).

---

## States

Not separately defined as a formal state model in this document. Section 4 (Finance Ownership) references state-like concepts such as "Capitalization Accounting Status" and "Book Value," but no state enumeration or transition table is provided anywhere in the file.

---

## Rules / Invariants

Rule-like content exists as ownership and governance boundaries rather than a dedicated "Rules" section. Section 3 (Explicit Non-Ownership) states Finance shall not maintain an independent operational asset lifecycle or duplicate asset custody state. Section 6 (Authorization, Audit, and Governance) states Finance users shall not gain operational asset custody, assignment, transfer, or disposal authority through financial accounting permissions, and lists context/reference fields Finance shall preserve. Section 7 (Completion Criteria) states invariants such as: no claim to own operational asset lifecycle state, Finance ownership restricted to monetary/accounting treatment, and every operational asset reference pointing to Section J/J08 or J10.

---

## Events

Section 5 (Integration with Section J) lists the events this domain consumes and publishes. Consumed operational events: AssetRegistered, AssetCapitalizationRequested, AssetAssigned, AssetTransferred, AssetVerified, AssetRetired, DisposalApproved, DisposalExecuted, WriteOffApproved. Finance-published outcome events: AssetCapitalizationCompleted, DepreciationPosted, AssetRevaluationPosted, AssetImpairmentPosted, AssetDisposalAccountingCompleted, AssetWriteOffAccountingCompleted. Per Section 5, Finance outcomes reference the Section J asset identifier but do not modify Section J asset state.

---

## Entities / Aggregates

Section 4 (Finance Ownership) enumerates the canonical Finance-owned entities: Financial Asset Accounting Record, Capitalization Accounting Status, Book Value, Depreciation Method and depreciation accounting schedule, Depreciation postings, Revaluation accounting, Impairment accounting, Disposal accounting, Write-off accounting, Asset accounting ledger references, and Asset financial reporting facts.
