# Purchase Order, Receipt, and Invoice Evidence to Match Decision

## 1. Scenario Metadata

**Scenario ID:** L3-F-081  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Procurement Finance Consumption  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F14; L2-J04; L2-J05; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs the Finance-owned invoice evidence matching and variance decision arising from Section J procurement facts while preserving Section J operational ownership.

## 3. Business Outcome

Finance records a governed matched, unmatched or exception match decision.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Finance has authoritative Purchase Order, receipt/acceptance and invoice evidence available.

### 4.2 End Boundary

Finance records and communicates the governed match decision.

### 4.3 In Scope

- validation of authoritative procurement and Finance references
- invoice evidence matching and variance decision
- decision, period and evidence traceability

### 4.4 Out of Scope

- supplier governance, Purchase Order amendment, goods acceptance, liability recognition and payment

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Procurement Finance Consumption | Owns the Finance matching/variance decision and its evidence |
| Section J — Enterprise Resource Management | Provides authoritative procurement facts and retains operational aggregate lifecycles |

## 6. Business Actors, Roles and Responsibilities

L2-F14 records no canonical Actors section for this Finance decision. Finance owns the matching responsibility and Section J supplies authoritative procurement evidence; the institutional actor/decision-authority mapping remains unresolved upstream.

## 7. Preconditions and Entry Criteria

- Authoritative Purchase Order, receipt/acceptance and invoice evidence are available.
- Applicable Finance references and period are valid.

## 8. Trigger

Finance has authoritative Purchase Order, receipt/acceptance and invoice evidence available.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate authoritative Section J evidence and Finance references | Procurement Finance Consumption / Section J | Input is eligible for Finance processing |
| 2 | Apply governed Finance policy for invoice matching and variance | Procurement Finance Consumption | Finance match outcome is determined |
| 3 | Obtain the governed Finance decision required for the outcome | Procurement Finance Consumption | Outcome is authorized at business level |
| 4 | Record matched, unmatched or exception outcome without modifying Section J aggregates | Procurement Finance Consumption | Finance-owned result exists |
| 5 | Publish `VendorInvoiceMatched` where the canonical matching condition is satisfied | Finance | Outcome is communicated |

## 10. Alternate Business Flows

Matched, unmatched and governed exception outcomes may be valid Finance results; Section J retains authority over any procurement consequence.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authoritative evidence is missing or inconsistent | Match decision cannot complete | Return the business exception to the owning domain for governed correction | F / J |
| Required Finance authority is not established | Decision cannot complete | Preserve the last valid position and obtain governed authorization | Procurement Finance Consumption |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Section J retains procurement ownership | Section J | Finance cannot amend procurement aggregates | L2-F14 §§1, 3 |
| Finance owns monetary/accounting consequences | Finance | Section J cannot create Finance monetary state | L2-F14 §4 |
| Cross-domain facts retain canonical source references | F / J | Outcome remains traceable without duplicate ownership | L2-F14 §§5, 8 |

## 13. State and Lifecycle Impact

L2-F14 defines no standalone lifecycle transition for the Section J procurement aggregates in this Finance scenario. The Finance match outcome is recorded without altering Section J state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| PurchaseOrderIssued | Section J | Procurement Finance Consumption | Provides authoritative order evidence | L2-J04; L2-F14 §6 |
| PurchaseOrderAmended | Section J | Procurement Finance Consumption | Provides authoritative order evidence | L2-J04; L2-F14 §6 |
| GoodsAccepted | Section J | Procurement Finance Consumption | Provides receipt evidence | L2-J05; L2-F14 §6 |
| ReceiptVarianceRecorded | Section J | Procurement Finance Consumption | Provides variance evidence | L2-J05; L2-F14 §6 |
| VendorInvoiceMatched | Finance | Section J and interested Finance domains | Communicates canonical Finance match outcome | L2-F14 §7 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Procurement Finance Consumption | Authoritative order, receipt and invoice evidence | Provide operational basis | L2-J04/J05; L2-F14 §§5–6 |
| Procurement Finance Consumption | Section J | Governed Finance match/variance decision | Inform downstream governance without transferring ownership | L2-F14 §5 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

Finance owns the match/variance decision, but L2-F14 does not define the canonical institutional actor or role that prepares or authorizes it. No named L3 authority is assigned.

### 15.2 Separation of Duties, Overrides and Exceptions

L2-F14 preserves segregation between procurement actors and Finance approval/payment actors. No unsupported named roles or additional exception authority are introduced.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve source traceability | Purchase Order, receipt/acceptance and invoice references | Procurement Finance Consumption |
| Preserve Finance decision traceability | Match/variance decision, policy and period references | Procurement Finance Consumption |
| Preserve resulting business fact | Applicable `VendorInvoiceMatched` reference | Procurement Finance Consumption |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Finance records the governed match outcome.
- Applicable canonical Finance event is published.
- Section J ownership and state remain unchanged by Finance.

### 16.2 Postconditions

- The Finance match outcome is available to downstream liability governance.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F14 does not define canonical institutional actors for invoice matching; prior named roles/authority depended on `ACR-F-2026-08-21-001`, whose canonical artifact was not located. | Finance ownership and canonical events remain source-supported, but named decision authority cannot be asserted. The scenario remains Review. | Establish actor/decision-authority mapping in authoritative L2/governance or provide independent approval provenance. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Unsupported ACR-based roles were removed while source-supported Section J inputs and `VendorInvoiceMatched` were preserved.
