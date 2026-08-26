# Budget Proposal to Approved Active Budget

## 1. Scenario Metadata

**Scenario ID:** L3-F-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Budget & Cost Center Management  
**Participating Domains:** B, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F06; L2-B02; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario establishes the governed institutional path by which a prepared budget proposal becomes the approved active financial plan for its Budget Period.

## 3. Business Outcome

A complete Budget Proposal is approved under the governed Budget process, transitions to Active, preserves approval history, and publishes `BudgetApproved`.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A prepared Budget Proposal is submitted for governed review.

### 4.2 End Boundary

The approved Budget is Active and its approval outcome is published.

### 4.3 In Scope

- proposal consolidation and completeness review
- validation of Budget Period and organisational scope
- governed approval decision and Budget activation

### 4.4 Out of Scope

- controlled allocation (`L3-F-035`)
- revision or virement (`L3-F-034`)
- actual accounting and expenditure

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Budget & Cost Center Management | Owns the Budget proposal, approval lifecycle, activation outcome and evidence |
| Section B — Institution & Structure | Provides authoritative organisational references without transferring ownership |

## 6. Business Actors, Roles and Responsibilities

L2-F06 does not define canonical business actors for Budget preparation or approval. Budget & Cost Center Management owns the business responsibilities in this scenario, but the institutional actor/decision-authority mapping remains an upstream architecture gap; see Section 17.

## 7. Preconditions and Entry Criteria

- A valid Budget Period exists.
- The proposal identifies its institution and authoritative Section B organisational references.
- The proposal contains the planned allocations required for approval review.

## 8. Trigger

A prepared Budget Proposal is submitted for governed approval.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate proposal scope, Budget Period and organisational references | Budget & Cost Center Management | Proposal is eligible for review |
| 2 | Review the proposal against governed Budget policy and planning requirements | Budget & Cost Center Management | Approval decision is reached |
| 3 | Apply the governed approval decision and activate the approved Budget | Budget & Cost Center Management | Budget becomes Active when approved |
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

L2-F06 requires governed approval for the `PendingApproval → Active` transition but does not define the canonical institutional actor or role that prepares, reviews, approves or rejects the Budget Proposal. No L3 role is assigned until that upstream mapping is governed.

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is introduced at L3 because L2-F06 does not define the participating institutional roles.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Budget Proposal, Budget Period and organisational scope | Budget & Cost Center Management |
| Preserve scenario decision traceability | Governed approval or rejection decision and preserved Budget version | Budget & Cost Center Management |
| Preserve scenario decision traceability | Resulting Budget state and `BudgetApproved` business fact | Budget & Cost Center Management |

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

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F06 explicitly records that its Actors section is absent and therefore does not establish the institutional actor/decision-authority mapping for Budget proposal preparation, review, approval or rejection. | The Budget lifecycle and approval requirement are source-supported, but L3 cannot authoritatively name a Finance Committee or other approving role. The scenario remains Review. | Define or approve the Budget actor/decision-authority mapping in authoritative L2/governance before promotion beyond Review. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Business semantics remain grounded in L2-F06. Unsupported named approval actors were removed; the unresolved upstream actor/authority mapping is recorded rather than invented at L3.
