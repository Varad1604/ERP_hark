# Financial Asset Revaluation to Updated Carrying Value

## 1. Scenario Metadata

**Scenario ID:** L3-F-069  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Asset Financial Accounting  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F12; L2-J08; L2-J10 (where disposal/write-off applies); SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario governs the Finance-owned asset revaluation accounting outcome for a Section J-owned operational Asset without transferring operational Asset ownership.

## 3. Business Outcome

The governed revaluation is posted and produces a traceable updated Finance carrying value.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A governed revaluation requirement and supporting evidence are presented for a valid financial Asset.

### 4.2 End Boundary

The governed revaluation is posted and the Finance carrying value reflects the outcome.

### 4.3 In Scope

- validation of authoritative Asset and prior value
- revaluation evidence and Finance decision
- updated carrying value and history traceability

### 4.4 Out of Scope

- operational Asset changes, depreciation, impairment, disposal and write-off

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Asset Financial Accounting | Owns the Finance revaluation outcome and evidence |
| Section J — Enterprise Resource Management | Provides authoritative operational Asset facts and retains its lifecycle |

## 6. Business Actors, Roles and Responsibilities

L2-F12 records no canonical Actors section. Asset Financial Accounting owns the Finance responsibilities and Section J supplies authoritative operational facts; the institutional actor/decision-authority mapping remains unresolved upstream.

## 7. Preconditions and Entry Criteria

- A valid Finance financial Asset accounting record exists.
- The canonical Section J operational Asset reference is available.
- The applicable accounting period permits the Finance outcome.

## 8. Trigger

A governed revaluation requirement and supporting evidence are presented for a valid financial Asset.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate authoritative operational Asset and Finance references | Asset Financial Accounting / Section J | Evidence is eligible for Finance treatment |
| 2 | Apply governed Finance policy for revaluation accounting | Asset Financial Accounting | Revaluation outcome is determined |
| 3 | Obtain the governed Finance decision required for posting | Asset Financial Accounting | Outcome is authorized at business level |
| 4 | Complete revaluation accounting and preserve prior financial history | Asset Financial Accounting | Carrying value reflects the outcome |
| 5 | Publish `AssetRevaluationPosted` | Asset Financial Accounting | Finance outcome is communicated |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required Asset, evidence, policy or period condition is invalid | Revaluation cannot complete | Correct the authoritative business dependency before reconsideration | F / J |
| Required Finance authorization is not established | Posting cannot complete | Preserve the last valid Finance position and obtain governed authorization | Asset Financial Accounting |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Finance owns monetary and accounting treatment only | Asset Financial Accounting | Operational Asset state is not modified | L2-F12 §§3–5 |
| Section J owns operational Asset lifecycle | Section J | Finance retains canonical Asset reference | L2-F12 §§3, 5; L2-J08/J10 |
| Detailed financial treatment comes from governed Finance policy | Finance | No local calculation rule is created | L2-F12 §§2, 4 |

## 13. State and Lifecycle Impact

The Finance accounting record reflects the governed revaluation outcome; Section J operational Asset state remains unchanged by Finance.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| AssetRevaluationPosted | Asset Financial Accounting | Accounting, reporting and interested Section J domains | Communicates Finance-owned revaluation outcome | L2-F12 §5 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J | Asset Financial Accounting | Authoritative operational Asset fact and identifier | Provide operational basis without transferring ownership | L2-J08/J10; L2-F12 §5 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

L2-F12 does not define the canonical institutional actor or role that prepares or authorizes revaluation. No named L3 authority is assigned.

### 15.2 Separation of Duties, Overrides and Exceptions

Finance authority does not confer operational Asset authority. No additional named separation-of-duty or override role is introduced at L3.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve revaluation traceability | Prior value, revaluation evidence, decision and updated carrying value | Asset Financial Accounting |
| Preserve source ownership traceability | Canonical Section J Asset reference | Asset Financial Accounting |
| Preserve governed Finance context | Applicable Finance policy and accounting period | Asset Financial Accounting |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The governed revaluation is posted and produces a traceable updated carrying value.
- `AssetRevaluationPosted` is published by Asset Financial Accounting.
- Section J operational Asset state remains under Section J ownership.

### 16.2 Postconditions

- Finance carrying value and accounting history reflect the governed outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F12 has no canonical Actors section; prior named roles and authority depended on `ACR-F-2026-08-21-001`, whose canonical repository artifact was not located in the established Pallav remediation. | Source-supported business semantics remain usable, but named institutional authority cannot be asserted. The scenario remains Review. | Establish actor/decision-authority mapping in authoritative L2/governance or provide independent approval provenance. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Unsupported ACR-based actors and authority were removed while preserving source-supported revaluation semantics, event and ownership.
