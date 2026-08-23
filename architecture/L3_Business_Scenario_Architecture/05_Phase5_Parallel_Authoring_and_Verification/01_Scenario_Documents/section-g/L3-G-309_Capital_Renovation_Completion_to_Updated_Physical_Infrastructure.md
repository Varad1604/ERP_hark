# Capital Renovation Completion to Updated Physical Infrastructure

# 1. Scenario Metadata

**Scenario ID:** L3-G-309  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, J, F, B  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-J03 — Purchase Requisition & Approval Workflows; L2-J01 — Procurement Planning & Sourcing Workflows; L2-F06 — Budget & Cost Center Management; L2-B03 — Campus Workflows  
**Referenced By:** Authorized consumers of post-renovation Infrastructure facts

# 2. Scenario Purpose

Reconcile an authorised completed capital renovation into canonical physical Infrastructure so affected physical facts are updated with historical traceability while J/F/B facts remain with their owners.

# 3. Business Outcome

The affected physical infrastructure hierarchy, attributes, capacity, or capability is updated with historical traceability

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary
The scenario starts when an authorised capital renovation is completed.

## 4.2 End Boundary
The scenario ends when actual completed physical changes are reconciled into G10 and required post-renovation validation supports the resulting Infrastructure position.

## 4.3 In Scope
Completed-work reconciliation; affected hierarchy/attribute/capacity/capability updates; identity/history preservation; G10 inspection/compliance validation; applicable lifecycle facts.

## 4.4 Out of Scope
Renovation execution; procurement/financial execution; InstitutionalCampus lifecycle change; invented G↔J/F/B completion contracts.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| G10 | Owns physical Infrastructure, physical changes, lifecycle and historical truth. |
| J | Owns separately completed procurement/resource facts. |
| F | Owns separately completed budget/financial facts. |
| B | Owns InstitutionalCampus identity/governance context. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Reconciles completed authorized renovation results into G10-owned physical facts. | Acts within authorized scope. |
| Infrastructure Manager | Reviews physical reconciliation and lifecycle position. | G10 responsibility. |
| Compliance Officer | Participates where G10 post-renovation compliance validation applies. | Source-supported compliance responsibility. |

No J/F/B renovation-completion actor is invented.

# 7. Preconditions and Entry Criteria

- The renovation is authorised and completed.
- Affected physical Infrastructure is canonically identifiable.
- Pre-renovation history is available.
- Required inspection/compliance information is available.
- Reconciliation is authorized.

# 8. Trigger

An authorised capital renovation is completed

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify affected canonical Infrastructure and completed renovation. | G10 | Reconciliation scope is fixed. |
| 2 | Compare completed physical reality with authoritative pre-renovation facts. | G10 | Actual changes are distinguished. |
| 3 | Preserve existing identity where physical identity remains unchanged. | G10 | Historical continuity is maintained. |
| 4 | Update only affected source-supported physical facts. | G10 | Current authoritative Infrastructure reflects completed reality. |
| 5 | Complete required G10 inspection/compliance validation. | G10 | Readiness is established. |
| 6 | Complete the applicable renovation/lifecycle position only when G10 conditions are satisfied. | G10 | Physical lifecycle is reconciled. |
| 7 | Publish only canonical facts that actually occurred. | G10 | Consumers receive valid completed facts. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Attribute-only change | Hierarchy did not change. | Update only affected attributes/capabilities. | No artificial restructuring. |
| Hierarchy changed | Authorized physical structure changed. | Apply applicable G10 hierarchy rules with history preserved. | Canonical hierarchy reflects reality. |
| Validation not satisfied | Operational restoration is not yet permitted. | Preserve applicable non-operational position until resolved. | No premature operational outcome. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Completed result conflicts with G10 rules | Reconciliation cannot complete. | Resolve inconsistency through governed G10 path. | G10 |
| Required validation unresolved | Final lifecycle position cannot be confirmed. | Keep applicable controlled state until resolved. | G10 |
| J/F/B source fact unavailable | That context cannot be asserted. | Do not infer it. | Respective owner |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Capital renovation is governed by G10 and historical physical identity is preserved where identity remains unchanged. | G10 | Reconciliation updates facts without default identity replacement. | L2-G10 §36 |
| G10 requires inspection and compliance validation before return to Operational in its renovation flow. | G10 | Operational restoration cannot be asserted early. | L2-G10 §36 |
| J owns procurement/resource lifecycles; F owns financial truth; B owns InstitutionalCampus identity. | J/F/B | Physical reconciliation does not absorb those aggregates. | L2-J03/J01; L2-F06; L2-B03 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | `Under Renovation` where applicable | Completed renovation is reconciled and validated. | `Operational` where source-defined conditions are satisfied; otherwise current valid G10 state remains. | G10 | L2-G10 §§11A, 36 |
| Physical hierarchy/attributes/capacity/capability | Pre-renovation facts | Changed facts become current with history retained. | Updated authoritative physical facts. | G10 | L2-G10 §§8, 36 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events
`RenovationCompleted` is published by G10 when that lifecycle fact occurs. `InfrastructureOperational` applies only when the corresponding G10 lifecycle fact is actually reached. Attribute-specific events apply only to their respective changed facts.

## 14.2 Business Contracts and Cross-Domain Hand-offs
J/F/B retain their canonical contracts. G10 §36 does not define one universal mandatory renovation-completion hand-off to those domains.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Reconcile completed renovation | Infrastructure Administrator / Infrastructure Manager within authorized scope | Must satisfy G10 validation/lifecycle rules. | G10 |

## 15.2 Separation of Duties, Overrides and Exceptions
No universal scenario-specific SoD rule is defined; none is invented.

## 15.3 Business Audit and Evidence Obligations
Preserve pre/post physical facts, effective change history, inspection/compliance evidence and actor/authority context under source-defined audit obligations.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria
- Completed renovation is linked to canonical Infrastructure.
- Actual physical changes are reconciled.
- Historical identity is preserved where required.
- Required validation supports any final lifecycle position.
- Only applicable canonical events are published.

## 16.2 Postconditions
G10 exposes updated physical truth; historical reconstruction remains possible; J/F/B retain their own facts.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Frozen J/F/B participation has no exact universal G10 renovation-completion contract. | Collaboration granularity cannot be asserted universally. | Define upstream if mandatory. |
| Different physical changes can require different G10 events. | No single universal attribute-change event exists for every renovation. | Use the canonical event for the fact actually changed. |

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions
No assumptions.

## 17.3 Deferred Questions
No additional deferred questions.
