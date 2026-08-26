# Operational Asset Recognition to Capitalized Financial Asset

## 1. Scenario Metadata

**Scenario ID:** L3-F-067  
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

This scenario governs the Finance-owned capitalization accounting outcome for a Section J-owned operational Asset without transferring operational Asset ownership.

## 3. Business Outcome

A capitalized Finance-owned financial Asset record exists with opening carrying value and traceable Section J reference.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Finance receives authoritative operational Asset recognition or capitalization evidence.

### 4.2 End Boundary

A capitalized Finance-owned financial Asset record exists with opening carrying value and traceable Section J reference.

### 4.3 In Scope

- validation of authoritative Asset and operational evidence
- capitalization decision and opening carrying value
- Finance history, period and decision traceability

### 4.4 Out of Scope

- procurement, receipt, tagging, custody, assignment, depreciation, impairment and disposal

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Asset Financial Accounting | Owns the Finance capitalization outcome and its evidence |
| Section J — Enterprise Resource Management | Provides authoritative operational Asset facts and retains the operational Asset lifecycle |

## 6. Business Actors, Roles and Responsibilities

L2-F12 does not define canonical institutional actors for Asset financial-accounting decisions. Asset Financial Accounting owns the Finance responsibilities in this scenario; Section J provides the authoritative operational Asset fact. The institutional preparation/decision-authority mapping remains an upstream architecture gap and is not invented at L3.

## 7. Preconditions and Entry Criteria

- A valid Finance financial Asset accounting context exists where required.
- The canonical Section J operational Asset reference is available.
- The applicable accounting period permits the Finance outcome.

## 8. Trigger

Finance receives authoritative operational Asset recognition or capitalization evidence.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate authoritative operational Asset and Finance references | Asset Financial Accounting / Section J | Evidence is eligible for Finance treatment |
| 2 | Apply governed Finance policy for capitalization accounting | Asset Financial Accounting | Financial outcome is determined |
| 3 | Obtain the governed Finance decision required for capitalization | Asset Financial Accounting | Capitalization is authorized at business level |
| 4 | Complete capitalization accounting and preserve prior financial history | Asset Financial Accounting | Finance record reflects the outcome |
| 5 | Publish `AssetCapitalizationCompleted` | Asset Financial Accounting | Finance outcome is communicated |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required Asset, evidence, policy or period condition is invalid | Finance outcome cannot complete | Correct the authoritative business dependency before reconsideration | F / J |
| Required Finance authorization is not established | Capitalization cannot complete | Preserve the last valid Finance position and obtain governed authorization | Asset Financial Accounting |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Finance owns only monetary and accounting treatment | Asset Financial Accounting | Operational Asset state is not modified | L2-F12 §§3–5 |
| Section J owns operational Asset lifecycle | Section J | Finance retains the canonical Asset reference | L2-F12 §§3, 5; L2-J08/J10 |
| Detailed financial treatment comes from governed Finance policy | Finance | L3 creates no local calculation rule | L2-F12 §§2, 4 |

## 13. State and Lifecycle Impact

L2-F12 does not establish a canonical state machine for a separate financial-asset lifecycle in this scenario. The governed capitalization outcome is recorded by Finance; the Section J operational Asset lifecycle remains unchanged by Finance.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| AssetRegistered | Section J | Asset Financial Accounting | Provides authoritative operational basis | L2-J08; L2-F12 §5 |
| AssetCapitalizationRequested | Section J | Asset Financial Accounting | Provides authoritative operational basis | L2-J08; L2-F12 §5 |
| AssetCapitalizationCompleted | Asset Financial Accounting | Accounting, reporting and interested Section J domains | Communicates Finance-owned outcome | L2-F12 §5 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Asset Financial Accounting | Authoritative operational Asset fact and identifier | Provide operational basis without transferring ownership | L2-J08/J10; L2-F12 §5 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

L2-F12 establishes Finance ownership of the accounting outcome but does not define the canonical institutional actor or role that prepares or authorizes capitalization. No named L3 authority is assigned until the upstream mapping is governed.

### 15.2 Separation of Duties, Overrides and Exceptions

Finance authority does not confer operational custody, assignment, transfer, disposal or write-off authority. No additional named separation-of-duty or override role is introduced at L3.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve decision traceability | Operational Asset evidence, capitalization decision and opening carrying value | Asset Financial Accounting |
| Preserve source ownership traceability | Canonical Section J Asset and source-event references | Asset Financial Accounting |
| Preserve governed Finance context | Applicable Finance policy and accounting period | Asset Financial Accounting |

Section A retains ownership of Authorization and Audit infrastructure; Asset Financial Accounting owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- A capitalized Finance-owned financial Asset record exists with opening carrying value and traceable Section J reference.
- `AssetCapitalizationCompleted` is published by Asset Financial Accounting.
- Section J operational Asset state remains under Section J ownership.

### 16.2 Postconditions

- Finance carrying value and accounting history reflect the governed outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F12 records no canonical Actors section. The prior named roles and authority were attributed to `ACR-F-2026-08-21-001`, for which no canonical repository artifact was located in the established Pallav remediation. | Finance ownership and canonical events remain source-supported, but L3 cannot authoritatively assign `Asset Accounting Officer`, `Financial Controller`, `Finance Committee`, or equivalent decision authority. The scenario remains Review. | Establish the actor/decision-authority mapping in authoritative L2/governance or provide independently traceable approval provenance before promotion beyond Review. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Source-supported Asset Accounting ownership, events and Section J boundary are preserved. Unsupported ACR-based roles and authority were removed rather than recreated at L3.
