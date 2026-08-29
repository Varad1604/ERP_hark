# Approved Procurement Demand to Budget Commitment Outcome

## 1. Scenario Metadata

**Scenario ID:** L3-F-080  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Procurement Finance Consumption  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F14; L2-F06; L2-J03; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs the Finance-owned budget validation and commitment decision arising from Section J procurement facts while preserving Section J operational ownership.

## 3. Business Outcome

Finance records a governed commitment or non-commitment outcome for the approved demand.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Finance receives an authoritative approved procurement demand from Section J.

### 4.2 End Boundary

Finance records and communicates the governed commitment or non-commitment outcome.

### 4.3 In Scope

- validation of authoritative procurement and Finance references
- Budget validation and commitment decision
- decision, period and evidence traceability

### 4.4 Out of Scope

- requisition approval, sourcing, supplier selection, purchase-order lifecycle, receipt, invoice matching and payment

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Procurement Finance Consumption | Owns the Finance commitment/non-commitment outcome and its evidence |
| Section J — Enterprise Resource Management | Provides authoritative procurement facts and retains operational aggregate lifecycles |

## 6. Business Actors, Roles and Responsibilities

L2-F14 and L2-F06 record no canonical Actors section for this Finance decision. Finance owns the monetary decision responsibility and Section J supplies authoritative procurement evidence; the institutional actor/decision-authority mapping remains unresolved upstream.

## 7. Preconditions and Entry Criteria

- Authoritative Section J source facts and identifiers are available.
- Applicable Finance Budget and period references are valid.

## 8. Trigger

Finance receives an authoritative approved procurement demand from Section J.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate authoritative Section J evidence and Finance references | Procurement Finance Consumption / Section J | Input is eligible for Finance processing |
| 2 | Apply governed Finance policy for Budget validation and commitment | Procurement Finance Consumption | Finance outcome is determined |
| 3 | Obtain the governed Finance decision required for the outcome | Procurement Finance Consumption | Outcome is authorized at business level |
| 4 | Record the commitment or non-commitment without modifying Section J aggregates | Procurement Finance Consumption | Finance-owned result exists |
| 5 | Use `BudgetValidationCompleted` and publish `BudgetCommitmentCreated` where applicable | Finance | Outcome is communicated |

## 10. Alternate Business Flows

A governed non-commitment outcome remains a valid Finance result; Section J retains authority over the downstream procurement consequence.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authoritative evidence is missing or inconsistent | Finance outcome cannot complete | Return the business exception to the owning domain for governed correction | F / J |
| Applicable Finance condition is not satisfied | Positive commitment cannot be produced | Record the governed non-commitment or deferred outcome | Procurement Finance Consumption |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Section J retains procurement ownership | Section J | Finance cannot amend procurement aggregates | L2-F14 §§1, 3 |
| Finance owns monetary consequences | Finance | Section J cannot create Finance monetary state | L2-F14 §4 |
| Cross-domain facts retain canonical source references | F / J | Outcome remains traceable without duplicate ownership | L2-F14 §§5, 8 |

## 13. State and Lifecycle Impact

L2-F14 defines no standalone cross-domain lifecycle for this scenario. The Finance monetary outcome is recorded without altering Section J operational state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| PurchaseRequisitionApproved | Section J | Procurement Finance Consumption | Provides approved demand | L2-J03; L2-F14 §6 |
| BudgetValidationCompleted | Finance | Procurement Finance Consumption and Section J | Communicates authoritative Budget validation outcome | L2-F14 §7 |
| BudgetCommitmentCreated | Finance | Section J and interested Finance domains | Communicates commitment outcome | L2-F14 §7 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Procurement Finance Consumption | Authoritative procurement demand and references | Provide operational basis | L2-J03; L2-F14 §§5–6 |
| Procurement Finance Consumption | Section J | Governed Finance decision | Inform downstream procurement governance without transferring ownership | L2-F14 §5 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

Finance owns the commitment decision, but L2-F14/L2-F06 do not define the canonical institutional actor or role that prepares or authorizes it. No named L3 authority is assigned.

### 15.2 Separation of Duties, Overrides and Exceptions

L2-F14 requires procurement actors and Finance approval/payment actors to remain segregated. This cross-domain separation is preserved without inventing `Procurement Finance Analyst`, `Financial Controller`, or other named roles.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve source traceability | Section J source references | Procurement Finance Consumption |
| Preserve Finance decision traceability | Budget decision, period and policy references | Procurement Finance Consumption |
| Preserve resulting business facts | `BudgetValidationCompleted` and applicable `BudgetCommitmentCreated` references | Procurement Finance Consumption |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Finance records a governed commitment or non-commitment outcome.
- Applicable canonical Finance events are published.
- Section J ownership and state remain unchanged by Finance.

### 16.2 Postconditions

- The Finance outcome is available to downstream Section J governance.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F14/L2-F06 do not define canonical institutional actors for the commitment decision; prior named roles and authority depended on `ACR-F-2026-08-21-001`, whose canonical artifact was not located. | Finance ownership and canonical events remain source-supported, but named decision authority cannot be asserted. The scenario remains Review. | Establish actor/decision-authority mapping in authoritative L2/governance or provide independent approval provenance. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Unsupported ACR-based roles were removed and `BudgetValidationCompleted` provenance was aligned to locked L2-F14.
