# Approved Procurement Demand to Budget Commitment Outcome

## 1. Scenario Metadata

**Scenario ID:** L3-F-080  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Procurement Finance Consumption  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** L2-F00; L2-F14; L2-F06; L2-J03; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs the Finance-owned budget validation and commitment decision arising from Section J procurement facts while preserving Section J operational ownership.

## 3. Business Outcome

Finance records a governed commitment or non-commitment outcome for the approved demand

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Finance receives an authoritative approved procurement demand from Section J

### 4.2 End Boundary

Finance records a governed commitment or non-commitment outcome for the approved demand

### 4.3 In Scope

- validation of authoritative procurement and Finance references
- Budget validation and commitment decision
- decision, period, authority and evidence traceability

### 4.4 Out of Scope

- requisition approval, sourcing, supplier selection, purchase-order lifecycle, receipt, invoice matching and payment

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Procurement Finance Consumption | Owns the Finance decision or accounting outcome and its evidence |
| Section J — Enterprise Resource Management | Provides authoritative operational facts and retains its aggregate lifecycles |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Scenario Responsibility |
|---|---|
| Procurement Finance Analyst | Prepares the governed commitment, invoice match/acceptance or Finance-closure assessment. |
| Financial Controller | Independently authorizes material commitments, exceptions, invoice acceptance and Finance closure confirmation. |
| Accounts Payable Officer | Performs F08-owned payable preparation where liability registration participates. |
| Section J operational actor | Supplies authoritative procurement evidence and retains operational aggregate ownership. |

Roles, authority and separation of duty are established by approved `ACR-F-2026-08-21-001`.

## 7. Preconditions and Entry Criteria

- Authoritative Section J source facts and identifiers are available.
- Applicable Finance references and accounting/Budget period are valid.

## 8. Trigger

Finance receives an authoritative approved procurement demand from Section J.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate authoritative Section J evidence and Finance references | Procurement Finance Consumption / Section J | Input is eligible for Finance processing |
| 2 | Apply governed Finance policy for budget validation and commitment decision | Procurement Finance Consumption | Finance outcome is determined |
| 3 | Obtain required Finance decision authority | Procurement Finance Consumption | Outcome is authorized |
| 4 | Complete budget validation and commitment decision without modifying Section J aggregates | Procurement Finance Consumption | Finance-owned result exists |
| 5 | Reference the F06 `BudgetValidationCompleted` decision and publish `BudgetCommitmentCreated` where applicable | Procurement Finance Consumption | Outcome is communicated |

## 10. Alternate Business Flows

Where the governed Finance decision is negative or exceptional, Finance records that outcome and Section J retains authority over the downstream procurement consequence.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authoritative evidence is missing or inconsistent | Finance outcome cannot complete | Return the business exception to the owning domain for governed correction | F / J |
| Applicable Finance condition is not satisfied | Positive Finance outcome cannot be produced | Record the governed negative or deferred outcome | Procurement Finance Consumption |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Section J retains procurement and supplier ownership | Section J | Finance cannot amend procurement aggregates | L2-F14 §§1, 3 |
| Finance owns monetary and accounting consequences | Finance | Section J cannot create Finance monetary state | L2-F14 §4 |
| Cross-domain facts retain canonical source references | F / J | Outcome remains traceable without duplicate ownership | L2-F14 §§5, 8 |

## 13. State and Lifecycle Impact

The approved owner-domain monetary outcome and F14 collaboration lifecycle apply without altering Section J operational state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| PurchaseRequisitionApproved | Section J | Procurement Finance Consumption | Provides approved demand | L2-J03; L2-F14 §6 |
| BudgetValidationCompleted | Budget & Cost Center Management | Procurement Finance Consumption and Section J | Communicates the authoritative Budget decision | Approved ACR-F-2026-08-21-001 |
| BudgetCommitmentCreated | Procurement Finance Consumption | Section J and interested Finance domains | Communicates conditional Finance outcome | L2-F14 §7 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Procurement Finance Consumption | Authoritative procurement/resource fact and business references | Provide operational basis | Applicable L2-J; L2-F14 §§5–6 |
| Procurement Finance Consumption | Section J | Governed Finance decision or accounting confirmation | Inform downstream procurement governance without transferring ownership | L2-F14 §5 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Make and authorize Finance outcome | Defined by approved ACR-F-2026-08-21-001 | Procurement Finance Analyst prepares; Financial Controller independently authorizes | Procurement Finance Consumption |

### 15.2 Separation of Duties, Overrides and Exceptions

Procurement actors and Finance approval/payment actors must remain segregated; the Procurement Finance Analyst and Financial Controller roles are authoritative.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Section J source references | Procurement Finance Consumption |
| Preserve scenario decision traceability | Finance decision, policy and period references | Procurement Finance Consumption |
| Preserve scenario decision traceability | Authority and resulting Finance event references | Procurement Finance Consumption |

Section A retains ownership of Authorization and Audit infrastructure; Procurement Finance Consumption owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Finance records a governed commitment or non-commitment outcome for the approved demand.
- `BudgetValidationCompleted` is published by Budget & Cost Center Management and referenced by Procurement Finance Consumption.
- Section J ownership and state remain unchanged by Finance.

### 16.2 Postconditions

- The Finance outcome is available to the next governed Finance or Section J scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No unresolved material architecture issue remains for this scenario. The former upstream gaps were resolved by approved `ACR-F-2026-08-21-001` and its exact amendment specification.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** PASS  
**Validation Note:** Source-grounded Draft revalidated against approved ACR-F-2026-08-21-001; no unresolved material L3 issue remains.
