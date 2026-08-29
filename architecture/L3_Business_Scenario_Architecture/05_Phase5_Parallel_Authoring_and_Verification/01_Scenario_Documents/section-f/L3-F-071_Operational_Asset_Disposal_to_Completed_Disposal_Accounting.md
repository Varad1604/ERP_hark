# Operational Asset Disposal to Completed Disposal Accounting

## 1. Scenario Metadata

**Scenario ID:** L3-F-071  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Asset Financial Accounting  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F12; L2-J08; L2-J10 (where disposal/write-off applies); SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs the Finance-owned disposal-accounting outcome for a Section J-owned operational Asset without transferring operational Asset ownership.

## 3. Business Outcome

Finance completes disposal accounting and preserves carrying-value and gain/loss consequences.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Finance receives authoritative evidence that operational disposal reached the stage required for accounting.

### 4.2 End Boundary

Finance completes disposal accounting and preserves its financial consequences.

### 4.3 In Scope

- validation of authoritative Asset and disposal evidence
- prior value, proceeds and Finance outcome
- Finance history, period and decision traceability

### 4.4 Out of Scope

- disposal approval and physical execution, custody changes and replacement procurement

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Asset Financial Accounting | Owns the Finance disposal-accounting outcome and evidence |
| Section J — Enterprise Resource Management | Owns operational Asset and disposal facts and retains the operational lifecycle |

## 6. Business Actors, Roles and Responsibilities

L2-F12 records no canonical Actors section. Asset Financial Accounting owns the Finance responsibilities and Section J supplies the authoritative operational disposal facts; the institutional Finance actor/decision-authority mapping remains unresolved upstream.

## 7. Preconditions and Entry Criteria

- A valid Finance financial Asset accounting record exists.
- The canonical Section J Asset and applicable disposal evidence are available.
- The applicable accounting period permits the Finance outcome.

## 8. Trigger

Finance receives authoritative evidence that operational disposal reached the stage required for accounting.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate authoritative operational Asset/disposal facts and Finance references | Asset Financial Accounting / Section J | Evidence is eligible for Finance treatment |
| 2 | Apply governed Finance policy for disposal accounting | Asset Financial Accounting | Financial outcome is determined |
| 3 | Obtain the governed Finance decision required for posting | Asset Financial Accounting | Outcome is authorized at business level |
| 4 | Complete disposal accounting and preserve prior financial history | Asset Financial Accounting | Finance record reflects the outcome |
| 5 | Publish `AssetDisposalAccountingCompleted` | Asset Financial Accounting | Finance outcome is communicated |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required Asset, disposal evidence, policy or period condition is invalid | Finance outcome cannot complete | Correct the authoritative business dependency before reconsideration | F / J |
| Required Finance authorization is not established | Posting cannot complete | Preserve the last valid Finance position and obtain governed authorization | Asset Financial Accounting |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Finance owns monetary/accounting treatment only | Asset Financial Accounting | Operational Asset/disposal state is not modified | L2-F12 §§3–5 |
| Section J owns operational Asset and disposal lifecycle | Section J | Finance consumes authoritative facts without taking ownership | L2-F12 §§3, 5; L2-J08/J10 |
| Detailed financial treatment comes from governed Finance policy | Finance | No local calculation rule is created | L2-F12 §§2, 4 |

## 13. State and Lifecycle Impact

The Finance accounting record reflects the governed disposal-accounting outcome. Section J remains the sole owner of operational Asset and disposal lifecycle transitions.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| AssetRetired | Section J | Asset Financial Accounting | Provides operational disposal evidence | L2-J08/J10; L2-F12 §5 |
| DisposalApproved | Section J | Asset Financial Accounting | Provides operational disposal evidence | L2-J08/J10; L2-F12 §5 |
| DisposalExecuted | Section J | Asset Financial Accounting | Provides operational disposal evidence | L2-J08/J10; L2-F12 §5 |
| AssetDisposalAccountingCompleted | Asset Financial Accounting | Accounting, reporting and interested Section J domains | Communicates Finance-owned outcome | L2-F12 §5 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Asset Financial Accounting | Authoritative Asset/disposal facts and identifiers | Provide operational basis without transferring ownership | L2-J08/J10; L2-F12 §5 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

L2-F12 does not define the canonical Finance actor or role that prepares or authorizes disposal accounting. No named L3 Finance authority is assigned.

### 15.2 Separation of Duties, Overrides and Exceptions

Finance authority does not confer operational disposal authority. No additional named separation-of-duty or override role is introduced at L3.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve disposal-accounting traceability | Disposal evidence, prior value, proceeds and financial outcome | Asset Financial Accounting |
| Preserve source ownership traceability | Canonical Section J Asset and source-event references | Asset Financial Accounting |
| Preserve governed Finance context | Applicable Finance policy and accounting period | Asset Financial Accounting |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Finance completes disposal accounting and preserves carrying-value and gain/loss consequences.
- `AssetDisposalAccountingCompleted` is published by Asset Financial Accounting.
- Section J operational Asset/disposal state remains under Section J ownership.

### 16.2 Postconditions

- Finance accounting history reflects the governed disposal outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F12 has no canonical Actors section; prior named Finance roles and authority depended on `ACR-F-2026-08-21-001`, whose canonical repository artifact was not located in the established Pallav remediation. | Source-supported disposal-accounting semantics and events remain usable, but named Finance authority cannot be asserted. The scenario remains Review. | Establish actor/decision-authority mapping in authoritative L2/governance or provide independent approval provenance. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Unsupported ACR-based Finance actors and authority were removed while preserving Section J operational ownership and source-supported disposal-accounting events.
