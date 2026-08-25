# Active Financial Asset to Periodic Depreciation Posting

## 1. Scenario Metadata

**Scenario ID:** L3-F-068  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Asset Financial Accounting  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F12; L2-J08; L2-J10 (where disposal/write-off applies); SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs Finance-owned periodic depreciation for a Section J-owned operational Asset without transferring operational Asset ownership.

## 3. Business Outcome

Applicable periodic depreciation is posted and the Finance-owned carrying value is updated.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An active capitalized financial Asset reaches a governed periodic depreciation requirement.

### 4.2 End Boundary

Applicable periodic depreciation is posted and the Finance-owned carrying value is updated.

### 4.3 In Scope

- validation of authoritative Asset, depreciation period and governed Finance policy
- periodic depreciation posting
- financial history and evidence traceability

### 4.4 Out of Scope

- operational Asset changes, revaluation, impairment, disposal and write-off

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Asset Financial Accounting | Owns the Finance depreciation outcome and its evidence |
| Section J — Enterprise Resource Management | Provides authoritative operational Asset facts and retains the operational lifecycle |

## 6. Business Actors, Roles and Responsibilities

L2-F12 defines no canonical Actors section. Asset Financial Accounting owns the Finance responsibilities and Section J supplies authoritative operational facts; the institutional actor/decision-authority mapping is unresolved upstream.

## 7. Preconditions and Entry Criteria

- A valid Finance financial Asset accounting record exists.
- The canonical Section J operational Asset reference is available.
- The applicable accounting period permits the Finance outcome.

## 8. Trigger

An active capitalized financial Asset reaches a governed periodic depreciation requirement.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate authoritative operational Asset and Finance references | Asset Financial Accounting / Section J | Evidence is eligible for Finance treatment |
| 2 | Apply governed Finance policy for periodic depreciation | Asset Financial Accounting | Depreciation outcome is determined |
| 3 | Obtain the governed Finance decision required for posting | Asset Financial Accounting | Outcome is authorized at business level |
| 4 | Complete periodic depreciation and preserve prior financial history | Asset Financial Accounting | Carrying value reflects the outcome |
| 5 | Publish `DepreciationPosted` | Asset Financial Accounting | Finance outcome is communicated |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required Asset, policy or period condition is invalid | Depreciation cannot complete | Correct the authoritative business dependency before reconsideration | F / J |
| Required Finance authorization is not established | Posting cannot complete | Preserve the last valid Finance position and obtain governed authorization | Asset Financial Accounting |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Finance owns only monetary and accounting treatment | Asset Financial Accounting | Operational Asset state is not modified | L2-F12 §§3–5 |
| Section J owns operational Asset lifecycle | Section J | Finance retains the canonical Asset reference | L2-F12 §§3, 5; L2-J08/J10 |
| Detailed treatment comes from governed Finance policy | Finance | L3 creates no local calculation rule | L2-F12 §§2, 4 |

## 13. State and Lifecycle Impact

The Finance accounting record reflects the governed periodic depreciation outcome; no Section J operational Asset lifecycle transition is performed by Finance.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| DepreciationPosted | Asset Financial Accounting | Accounting, reporting and interested Section J domains | Communicates Finance-owned depreciation outcome | L2-F12 §5 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Asset Financial Accounting | Authoritative operational Asset fact and identifier | Provide operational basis without transferring ownership | L2-J08/J10; L2-F12 §5 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

L2-F12 does not define the institutional actor or role that prepares or authorizes depreciation. No named L3 authority is assigned.

### 15.2 Separation of Duties, Overrides and Exceptions

Finance authority does not confer operational Asset authority. No additional named separation-of-duty or override role is introduced at L3.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve depreciation traceability | Depreciation period, governed method, amount, prior and resulting carrying value | Asset Financial Accounting |
| Preserve source ownership traceability | Canonical Section J Asset reference | Asset Financial Accounting |
| Preserve governed Finance context | Applicable Finance policy and accounting period | Asset Financial Accounting |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Applicable periodic depreciation is posted and the Finance-owned carrying value is updated.
- `DepreciationPosted` is published by Asset Financial Accounting.
- Section J operational Asset state remains under Section J ownership.

### 16.2 Postconditions

- Finance carrying value and history reflect the governed outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F12 has no canonical Actors section; prior named roles and authority depended on `ACR-F-2026-08-21-001`, whose canonical artifact was not located in the established Pallav remediation. | The business outcome and event remain source-supported, but named institutional authority cannot be asserted. The scenario remains Review. | Establish actor/decision-authority mapping in authoritative L2/governance or provide independent approval provenance. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Unsupported ACR-based actors and authority were removed; source-supported depreciation semantics, event and ownership remain unchanged.
