# Budget Proposal to Approved Active Budget

## 1. Scenario Metadata

**Scenario ID:** L3-F-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Budget & Cost Center Management  
**Participating Domains:** B, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** L2-F00; L2-F06; L2-B02; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario establishes the governed institutional path by which a prepared budget proposal becomes the approved active financial plan for its Budget Period.

## 3. Business Outcome

A complete Budget Proposal is approved by the Finance Committee, transitions to Active, preserves approval history, and publishes `BudgetApproved`.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A prepared Budget Proposal is submitted for governed review.

### 4.2 End Boundary

The approved Budget is Active and its approval outcome is published.

### 4.3 In Scope

- proposal consolidation and completeness review
- validation of Budget Period and organisational scope
- Finance Committee decision and Budget activation

### 4.4 Out of Scope

- controlled allocation (`L3-F-035`)
- revision or virement (`L3-F-034`)
- actual accounting and expenditure

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Budget & Cost Center Management | Owns the Finance decision or accounting outcome and its evidence |
| Section B — Institution & Structure | Provides authoritative organisational references without transferring ownership |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Finance Department | Prepares and consolidates the Budget Proposal. | Initiates and presents |
| Finance Committee | Reviews and decides the submitted proposal. | Approves or rejects |

## 7. Preconditions and Entry Criteria

- A valid Budget Period exists.
- The proposal identifies its institution and authoritative Section B organisational references.
- The proposal contains the planned allocations required for approval review.

## 8. Trigger

A prepared Budget Proposal is submitted for governed approval.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate proposal scope, Budget Period and organisational references | Finance Department | Proposal is eligible for review |
| 2 | Review the proposal against governed Budget policy and planning requirements | Finance Committee | Approval decision is reached |
| 3 | Approve the proposal and activate the Budget | Finance Committee / Budget & Cost Center Management | Budget becomes Active |
| 4 | Preserve approval history and publish `BudgetApproved` | Budget & Cost Center Management | Approved active Budget is available |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined beyond governed proposal composition permitted by L2-F06.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Proposal is rejected | The Budget cannot become Active | Return `PendingApproval` to `Draft`; preserve rejection and permit governed revision/resubmission | Budget & Cost Center Management |
| Required period or organisational reference is invalid | Review cannot validly proceed | Correct the authoritative dependency before resubmission | B / F |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Budget belongs to exactly one Budget Period | Budget & Cost Center Management | Proposal cannot be approved without a valid period | L2-F06 §9.6 |
| Planned allocations do not replace accounting records | Finance | Scenario creates no actual transaction | L2-F06 §§1–3 |
| Organisational data is referenced, not duplicated | Section B | Proposal uses authoritative organisational scope | L2-F06 §8.6; L2-B02 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Budget | Draft; PendingApproval | `Draft → PendingApproval → Active`; rejection returns to Draft | Active | Budget & Cost Center Management | L2-F06 §13 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| BudgetApproved | Budget & Cost Center Management | Accounting & General Ledger and interested Finance domains | Communicates the approved active Budget | L2-F06 §§14–15 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section B | Budget & Cost Center Management | Authoritative organisational structure reference | Establish governing scope | L2-B02; L2-F06 §8.3 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Prepare and submit proposal | Finance Department | Must act within governed institutional Budget process | Budget & Cost Center Management |
| Approve or reject proposal | Finance Committee | Approval is required for `PendingApproval → Active` | Budget & Cost Center Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is defined for initial approval beyond the Finance Committee approval transition.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Budget Proposal, Budget Period and organisational scope | Budget & Cost Center Management |
| Preserve scenario decision traceability | Finance Committee approval or rejection and preserved Budget version | Budget & Cost Center Management |
| Preserve scenario decision traceability | Resulting Budget state and `BudgetApproved` reference | Budget & Cost Center Management |

Section A retains ownership of Authorization and Audit infrastructure; Budget & Cost Center Management owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Budget is in `Active`.
- Approval evidence and planning history are preserved.
- `BudgetApproved` is published by Budget & Cost Center Management.

### 16.2 Postconditions

- The active Budget is available for controlled allocation.
- No actual accounting transaction has been created.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** PASS  
**Validation Note:** Source-grounded Draft; independent review and architecture approval remain required.
