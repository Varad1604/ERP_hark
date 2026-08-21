# Active Financial Asset to Periodic Depreciation Posting

## 1. Scenario Metadata

**Scenario ID:** L3-F-068  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Asset Financial Accounting  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** L2-F00; L2-F12; L2-J08; L2-J10 (where disposal/write-off applies); SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs the Finance-owned periodic depreciation posting outcome for a Section J-owned operational Asset without transferring operational Asset ownership.

## 3. Business Outcome

Applicable periodic depreciation is posted and the Finance-owned carrying value is updated

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An active capitalized financial Asset reaches a governed periodic depreciation requirement

### 4.2 End Boundary

Applicable periodic depreciation is posted and the Finance-owned carrying value is updated

### 4.3 In Scope

- validation of authoritative Asset and depreciation period, governed method, amount, prior and resulting carrying value references
- Periodic depreciation posting
- financial history, period, authority and posting traceability

### 4.4 Out of Scope

- operational Asset changes, revaluation, impairment, disposal and write-off

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Asset Financial Accounting | Owns the Finance decision or accounting outcome and its evidence |
| Section J — Enterprise Resource Management | Provides authoritative operational facts and retains its aggregate lifecycles |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Scenario Responsibility |
|---|---|
| Asset Accounting Officer | Prepares the governed Asset financial-accounting outcome and supporting evidence. |
| Financial Controller | Independently authorizes the applicable capitalization, revaluation, impairment, disposal-accounting or write-off-accounting outcome; authorizes depreciation at the governed batch/outcome level. |
| Finance Committee | Owns approval of institutional Asset Accounting and Materiality Policies. |
| Section J operational actor | Supplies the authoritative operational Asset, disposal or write-off fact without transferring operational ownership. |

Roles and authority are established by approved `ACR-F-2026-08-21-001`.

## 7. Preconditions and Entry Criteria

- A valid Finance financial Asset accounting record exists where required.
- The canonical Section J operational Asset reference is available.
- The applicable accounting period permits the Finance outcome.

## 8. Trigger

An active capitalized financial Asset reaches a governed periodic depreciation requirement.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate authoritative operational Asset and Finance references | Asset Financial Accounting / Section J | Evidence is eligible for Finance treatment |
| 2 | Apply governed Finance policy for periodic depreciation posting | Asset Financial Accounting | Financial outcome is determined |
| 3 | Obtain required Finance decision or posting authority | Asset Financial Accounting | Outcome is authorized |
| 4 | Complete periodic depreciation posting and preserve prior financial history | Asset Financial Accounting | Finance record reflects the outcome |
| 5 | Publish `DepreciationPosted` | Asset Financial Accounting | Finance outcome is communicated |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required Asset, evidence, policy or period condition is invalid | Finance outcome cannot complete | Correct the authoritative business dependency before reconsideration | F / J |
| Finance treatment is not authorized | No financial posting occurs | Preserve the governed decision and await valid authorization | Asset Financial Accounting |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Finance owns only monetary and accounting treatment | Asset Financial Accounting | Operational Asset state is not modified | L2-F12 §§3–5 |
| Section J owns operational Asset lifecycle | Section J | Finance retains canonical Asset reference | L2-F12 §§3, 5; L2-J08/J10 |
| Detailed financial treatment must come from governed Finance policy | Finance | No local calculation rule is created | L2-F12 §§2, 4 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Financial Asset Accounting Record | Existing Finance lifecycle position | Records the governed accounting outcome; the approved Finance lifecycle applies | Periodic depreciation posting | Asset Financial Accounting | L2-F12 Lifecycle / States note |
| Operational Asset | Authoritative Section J state | Consumed as evidence only; no Finance-owned transition | Unchanged by Finance | Section J | L2-F12 §§3, 5 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| DepreciationPosted | Asset Financial Accounting | Accounting, reporting and interested Section J domains | Communicates Finance-owned outcome | L2-F12 §5 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Asset Financial Accounting | Authoritative operational Asset fact and identifier | Provide operational basis without transferring ownership | L2-J08/J10; L2-F12 §5 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Determine and post Finance outcome | Asset Accounting Officer prepares; Financial Controller authorizes under approved policy | Approved authority and separation of duty apply | Asset Financial Accounting |

### 15.2 Separation of Duties, Overrides and Exceptions

Finance authority does not confer operational custody, assignment, transfer, disposal or write-off authority.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Depreciation period, governed method, amount, prior and resulting carrying value | Asset Financial Accounting |
| Preserve scenario decision traceability | Canonical Section J Asset and source-event references | Asset Financial Accounting |
| Preserve scenario decision traceability | Finance policy, accounting period, decision authority and posting reference | Asset Financial Accounting |

Section A retains ownership of Authorization and Audit infrastructure; Asset Financial Accounting owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Applicable periodic depreciation is posted and the Finance-owned carrying value is updated.
- `DepreciationPosted` is published by Asset Financial Accounting.
- Section J operational Asset state remains under Section J ownership.

### 16.2 Postconditions

- Finance carrying value and accounting history reflect the governed outcome.

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
