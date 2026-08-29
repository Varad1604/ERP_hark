# Budget Period Completion to Governed Closure

## 1. Scenario Metadata

**Scenario ID:** L3-F-038  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Budget & Cost Center Management  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** L2-F00; L2-F06; SutraERP L1 Business Domain Architecture (Locked); DEPENDENCY_MAP.md; L3 Business Scenario Template v2.0; Stage 3 Canonical Scenario Register — Section F  
**Referenced By:** None yet

## 2. Scenario Purpose

This scenario closes an eligible Budget at completion of its planning period while preserving immutable planning history.

## 3. Business Outcome

The eligible Active Budget transitions to Closed, closure evidence is preserved, and `BudgetClosed` is published.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The Budget Period reaches governed completion and the Active Budget becomes closure-eligible.

### 4.2 End Boundary

The Budget is Closed and its history is immutable.

### 4.3 In Scope

- period-completion and closure-eligibility validation
- Active-to-Closed transition
- closure evidence and event publication

### 4.4 Out of Scope

- financial accounting-period close
- next-cycle Budget creation
- modification of closed history

### 4.5 Related Scenarios — Optional

No additional relationship is required to define this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Budget & Cost Center Management | Owns the Budget closure decision, lifecycle transition and evidence |

## 6. Business Actors, Roles and Responsibilities

L2-F06 does not define canonical institutional actors for Budget closure. Budget & Cost Center Management owns the closure responsibility, while the actor/decision-authority mapping remains an upstream architecture gap; no named closure role is introduced at L3.

## 7. Preconditions and Entry Criteria

- The Budget is Active.
- The associated Budget Period has reached governed completion.
- Required allocation and revision history is available.

## 8. Trigger

The applicable Budget Period reaches governed completion.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm Budget identity, period completion and closure eligibility | Budget & Cost Center Management | Budget is eligible for closure |
| 2 | Resolve any condition that prevents governed closure | Budget & Cost Center Management | Closure may proceed |
| 3 | Obtain the governed closure decision and apply `Active → Closed` | Budget & Cost Center Management | Budget is Closed |
| 4 | Preserve immutable history and publish `BudgetClosed` | Budget & Cost Center Management | Closure is communicated |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined upstream.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Closure eligibility is not satisfied | Budget remains Active | Resolve the governed business condition before re-evaluation | Budget & Cost Center Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Historical Budgets are immutable after closure | Budget & Cost Center Management | No ordinary modification after closure | L2-F06 §9.6 |
| Budget closure is distinct from accounting-period closure | Finance | Scenario changes planning lifecycle only | L2-F06 §§2–3 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Budget | Active | `Active → Closed` when the Budget Period completes and closure eligibility is satisfied | Closed | Budget & Cost Center Management | L2-F06 §13 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| BudgetClosed | Budget & Cost Center Management | Reporting and interested Finance domains | Communicates governed Budget closure | L2-F06 §14.1 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

No additional synchronous business contract is required beyond the authoritative references described above.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

L2-F06 requires governed Budget closure but does not name the institutional actor or role that authorizes the closure. L3 therefore preserves the Finance-owned closure decision without inventing a named authority.

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is defined upstream for Budget closure.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve scenario decision traceability | Budget and Budget Period references | Budget & Cost Center Management |
| Preserve scenario decision traceability | Closure eligibility and governed closure decision | Budget & Cost Center Management |
| Preserve scenario decision traceability | Closed state and `BudgetClosed` business fact | Budget & Cost Center Management |

Section A retains ownership of Authorization and Audit infrastructure; Budget & Cost Center Management owns the underlying business decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Budget is `Closed`.
- Historical Budget evidence is immutable.
- `BudgetClosed` is published.

### 16.2 Postconditions

- Closed Budget is available for historical performance review.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F06 explicitly records that its Actors section is absent and does not define the institutional actor/decision-authority mapping for Budget closure. Scenario references to `ACR-F-2026-08-21-001` do not independently establish that mapping. | The `Active → Closed` lifecycle and `BudgetClosed` event are source-supported, but L3 cannot authoritatively name the closure authority. The scenario remains Review. | Establish the Budget-closure actor/decision-authority mapping in authoritative L2/governance and provide traceable approval provenance before promotion beyond Review. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Source-supported Budget closure lifecycle and canonical event are preserved. Unsupported ACR-based actor/authority claims were removed and the upstream role gap remains explicit.
