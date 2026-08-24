# Consultancy Request to Delivered and Financially Reconciled Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-037  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; L2-F00 — Finance Domain Model & Foundation; L2-F07 — Accounts Receivable Management

# 2. Scenario Purpose

Govern the Research-owned Consultancy lifecycle through registration, execution, deliverable completion, and the supported Research completion outcome while preserving Finance ownership of receivables, accounting, settlement, and reconciliation.

# 3. Business Outcome

The engagement is approved, delivered, accepted, financially reconciled, and closed or reaches a governed cancellation outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized external consultancy engagement is proposed

## 4.2 End Boundary

H05 supports `Proposed → Active → Completed`. The frozen outcome additionally requires explicit approval, client acceptance, Finance reconciliation, closed state, and cancellation outcome, which are not all canonically defined.

## 4.3 In Scope

Consultancy registration; approval as a required execution precondition; execution/progress; deliverable validation; Research-side completion; Finance ownership preservation; audit evidence.

## 4.4 Out of Scope

Invented consultancy approval/cancellation/closed lifecycles; invented client-acceptance event; Finance accounting/payment/reconciliation mechanics; invented Research→Finance reconciliation fact.

# 5. Ownership and Participating Domains

E03 owns workforce capability/eligibility/capacity/availability. Finance owns accounting, receivables, payments, settlement, and financial reconciliation consequences. Research owns Consultancy Project metadata, execution progress, deliverables, and H05 Consultancy state.

# 6. Business Actors, Roles and Responsibilities

Research Office and Principal Investigator register the consultancy. Principal Investigator and Consultancy Team execute. Research Office completes the Research-side engagement after H05 completion validation.

# 7. Preconditions and Entry Criteria

Authorized external engagement exists; client organization and agreement are verified; consultancy approval required by H05 execution precondition is satisfied; team and workforce references are valid.

# 8. Trigger

An authorized external consultancy engagement is proposed

# 9. Main Business Flow

1. Validate client, agreement, team, and authorization context.
2. Register the Consultancy Project; enter `Proposed` and publish `ConsultancyProjectRegistered`.
3. Confirm the H05 approval precondition before execution; H05 provides no separate approval state/event.
4. Execute consultancy work and preserve progress through `ConsultancyProjectProgressUpdated`.
5. Validate required deliverables and client acceptance where H05 says applicable.
6. Complete the Research-side Consultancy and publish `ConsultancyProjectCompleted`.
7. Preserve Finance ownership of all monetary consequences.
8. Stop before claiming Finance reconciliation, closed, or cancellation outcomes because no canonical end-to-end contract/lifecycle exists.

# 10. Alternate Business Flows

Phase-wise execution and multi-client work are source-supported where configured. H05 provides no cancellation branch. Client acceptance is a validation input “where applicable,” not a standalone lifecycle fact.

# 11. Failure, Interruption and Business Recovery

Missing approval evidence blocks execution; incomplete deliverables block completion; missing Finance reconciliation prevents the frozen terminal outcome; unsupported cancellation is not mapped to `Completed`.

# 12. Business Rules and Decision Constraints

Client organization must be verified; deliverables align with agreement; Finance owns consultancy revenue accounting and receivables; completed Research history is immutable; Research cannot create financial settlement facts.

# 13. State and Lifecycle Impact

Consultancy Project: `Proposed → Active → Completed`; progress stays `Active`. H05 defines no `Approved`, `Accepted`, `Closed`, or `Cancelled` state.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Research publishes `ConsultancyProjectRegistered`, `ConsultancyProjectProgressUpdated`, and `ConsultancyProjectCompleted`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Finance is the financial authority. F07 owns receivable representations but explicitly does not execute payment reconciliation; H05/F architecture does not define a consultancy-specific canonical reconciliation completion hand-off back to Research.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Use only H05-defined Research Office, PI, and Consultancy Team actions. The approval precondition has no explicit H05 approval actor/state/event to assign.

## 15.2 Separation of Duties, Overrides and Exceptions

Research cannot self-create Finance reconciliation or cancellation authority.

## 15.3 Business Audit and Evidence Obligations

Preserve client/agreement references, registration authority, approval evidence where externally/governedly available, execution/progress, deliverable evidence, applicable client-acceptance reference, Research completion, and Finance references.

# 16. Completion Criteria and Postconditions

The H05-supported Research path can reach `Completed`. The full frozen scenario cannot complete until approval, client acceptance, Finance reconciliation, closed state, and cancellation semantics are canonical.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H05 requires approval before execution but defines no Consultancy approval state/command/event/decision authority.
- Client acceptance is only an optional completion validation input, not a canonical acceptance state/event.
- No consultancy-specific Research→Finance reconciliation completion contract/event exists.
- H05 ends at `Completed`; no `Closed` or `Cancelled` state/transition/event exists.

These gaps materially affect the frozen end boundary.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
