# Accepted Supplier Invoice to Vendor Liability Recognition

## 1. Scenario Metadata

**Scenario ID:** L3-F-082  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Procurement Finance Consumption  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F14; L2-F08; L2-J02; L2-J04; L2-J05; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs Finance-owned vendor liability recognition arising from Section J procurement facts while preserving Section J operational ownership and Accounts Payable ownership of the Payable aggregate.

## 3. Business Outcome

Finance recognizes the vendor liability with complete source and financial traceability.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An authoritative supplier invoice reaches the governed accepted state required for recognition.

### 4.2 End Boundary

The vendor liability is recognized and the applicable Finance/AP business facts are published.

### 4.3 In Scope

- validation of authoritative procurement and Finance references
- vendor liability recognition
- hand-off to Accounts Payable where payable registration participates
- decision, period and evidence traceability

### 4.4 Out of Scope

- supplier governance, Purchase Order/receipt changes, payment approval and treasury execution

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Procurement Finance Consumption | Owns the Finance liability-recognition outcome and its evidence |
| Accounts Payable | Owns the Payable aggregate when a recognized liability is registered as a payable |
| Section J — Enterprise Resource Management | Provides authoritative procurement facts and retains operational aggregate lifecycles |

## 6. Business Actors, Roles and Responsibilities

L2-F14 and L2-F08 record no canonical Actors section for the relevant Finance/AP responsibilities. The owning Finance domains retain their responsibilities, while the institutional actor/decision-authority mapping remains unresolved upstream.

## 7. Preconditions and Entry Criteria

- Authoritative Section J source facts and identifiers are available.
- The governed invoice acceptance/matching condition required by Finance is satisfied.
- Applicable Finance references and accounting period are valid.

## 8. Trigger

An authoritative supplier invoice reaches the governed accepted state required for recognition.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate authoritative Section J evidence and Finance references | Procurement Finance Consumption / Section J | Input is eligible for Finance processing |
| 2 | Apply governed Finance policy for vendor liability recognition | Procurement Finance Consumption | Finance liability outcome is determined |
| 3 | Obtain the governed Finance decision required for recognition | Procurement Finance Consumption | Outcome is authorized at business level |
| 4 | Recognize the vendor liability and preserve canonical procurement references | Procurement Finance Consumption | Finance-owned liability recognition exists |
| 5 | Publish `VendorLiabilityRecognized`; where Accounts Payable registers the liability, AP publishes `PayableRegistered` | Finance / Accounts Payable | Canonical Finance/AP facts are communicated |

## 10. Alternate Business Flows

Where the governed Finance condition for recognition is not satisfied, Finance records the non-recognition/exception outcome and Section J retains authority over any procurement consequence.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authoritative evidence is missing or inconsistent | Liability recognition cannot complete | Return the business exception to the owning domain for governed correction | F / J |
| Required Finance authority is not established | Recognition cannot complete | Preserve the last valid position and obtain governed authorization | Procurement Finance Consumption |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Section J retains procurement/supplier ownership | Section J | Finance cannot amend procurement aggregates | L2-F14 §§1, 3 |
| Finance owns monetary/accounting consequences | Finance | Section J cannot create Finance monetary state | L2-F14 §4 |
| Accounts Payable owns canonical Payable state | Accounts Payable | Liability registration does not transfer payable ownership to F14 or J | L2-F08 |

## 13. State and Lifecycle Impact

Procurement Finance Consumption records the Finance liability-recognition outcome without altering Section J operational state. Where a Payable is registered, its lifecycle is owned exclusively by Accounts Payable under L2-F08.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| VendorInvoiceMatched | Finance | Procurement Finance Consumption / Accounts Payable | Provides canonical Finance matching evidence | L2-F14 §7 |
| VendorInvoiceApproved | Finance | Accounts Payable and interested Finance domains | Provides governed invoice approval where applicable | L2-F14 §7 |
| VendorLiabilityRecognized | Finance | Accounts Payable, Accounting and interested domains | Communicates the Finance-owned liability-recognition fact | L2-F14 §7 |
| PayableRegistered | Accounts Payable | Accounting, Treasury and interested Finance domains | Communicates canonical Payable aggregate creation where AP registration occurs | L2-F08 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Procurement Finance Consumption | Authoritative procurement/resource facts and references | Provide operational basis | L2-F14 §§5–6; applicable L2-J |
| Procurement Finance Consumption | Accounts Payable | Recognized vendor-liability fact | Permit AP-owned payable registration without ownership duplication | L2-F14 §7; L2-F08 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

Finance owns the liability-recognition decision, but L2-F14/L2-F08 do not define the canonical institutional actor or role that prepares or authorizes it. No named L3 authority is assigned.

### 15.2 Separation of Duties, Overrides and Exceptions

L2-F14 preserves segregation between procurement actors and Finance approval/payment actors. No unsupported named roles or additional exception authority are introduced.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve source traceability | Section J procurement/invoice references | Procurement Finance Consumption |
| Preserve Finance recognition traceability | Match/acceptance, policy, period and liability-recognition decision | Procurement Finance Consumption |
| Preserve Payable ownership traceability | `PayableRegistered` where AP registration occurs | Accounts Payable |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Finance recognizes the vendor liability with complete traceability.
- `VendorLiabilityRecognized` is published by Finance.
- Where AP registration occurs, `PayableRegistered` is published by Accounts Payable.
- Section J ownership and state remain unchanged by Finance.

### 16.2 Postconditions

- The recognized liability is available for Accounts Payable lifecycle governance.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F14/L2-F08 do not define canonical institutional actors for liability recognition/payable preparation; prior named roles and authority depended on `ACR-F-2026-08-21-001`, whose canonical artifact was not located. | Finance/AP ownership and canonical events are source-supported, but named institutional authority cannot be asserted. The scenario remains Review. | Establish actor/decision-authority mapping in authoritative L2/governance or provide independent approval provenance. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** ACR-dependent actor claims were removed; canonical `VendorLiabilityRecognized` and AP-owned `PayableRegistered` were grounded in locked L2-F14/L2-F08.
