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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; L2-F00 — Finance Domain Model & Foundation; L2-F07 — Accounts Receivable Management  
**Referenced By:** Related Research governance, audit, reporting, and downstream institutional use

# 2. Scenario Purpose

H05 supports consultancy registration, execution/progress, deliverable validation, and `Completed`; Finance owns revenue accounting, receivables, settlement, and reconciliation consequences.

# 3. Business Outcome

The engagement is approved, delivered, accepted, financially reconciled, and closed or reaches a governed cancellation outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized external consultancy engagement is proposed.

## 4.2 End Boundary

The frozen end outcome is reached only when all required branches in Section 3 are canonically supported. Locked H05 supports only the partial path stated in Section 2; material gaps are recorded in Section 17.

## 4.3 In Scope

- The H05-supported lifecycle path materially needed for this scenario.
- Source-domain references from E and, where applicable, Finance without ownership transfer.
- Exact H05 events, rules, authority, and evidence obligations that are defined upstream.

## 4.4 Out of Scope

- Any missing lifecycle, decision, actor authority, event, or Finance contract identified in Section 17.
- Changes to Faculty Workforce or Finance-owned records by Research.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| People & Workforce / Section E | Retains canonical Faculty Workforce identity, capability, availability, capacity, and eligibility used by Research. |
| Finance / Section F | Where frozen as participating, retains all monetary/accounting/receivable consequences; Research does not own them. |
| Research / Section H | Owns the Research aggregate/lifecycle and exact H05 business events for the supported path. |

For scenarios without F in frozen participating domains, the Finance row is non-participating context only and does not add Finance to scenario ownership or flow.

# 6. Business Actors, Roles and Responsibilities

Source-supported actors: Research Office; Principal Investigator; Consultancy Team.

# 7. Preconditions and Entry Criteria

- The applicable Research aggregate and authoritative source references exist as required by H05.
- Required institutional policy/configuration and evidence are available.
- Source-domain ownership remains unchanged.

# 8. Trigger

An authorized external consultancy engagement is proposed

# 9. Main Business Flow

Validate client/agreement context and register the consultancy; begin execution only after H05’s approval precondition is satisfied; record progress/deliverables; complete Research-side delivery when criteria are met; preserve Finance ownership and stop before unsupported reconciliation/closure/cancellation.

# 10. Alternate Business Flows

Phase-wise execution is supported; client acceptance is only a completion validation “where applicable”; H05 defines no cancellation branch.

# 11. Failure, Interruption and Business Recovery

Missing approval evidence prevents safe execution; incomplete deliverables prevent completion; unresolved Finance reconciliation prevents the frozen final outcome.

# 12. Business Rules and Decision Constraints

Client must be verified; deliverables align to agreement; Finance owns consultancy revenue accounting and receivables; completed consultancy history is immutable.

# 13. State and Lifecycle Impact

`Proposed` → `Active` → `Completed`; progress remains `Active`.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H05 defines: `ConsultancyProjectRegistered`, `ConsultancyProjectProgressUpdated`, `ConsultancyProjectCompleted`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Research consumes authoritative workforce facts from L2-E03. Where F participates, Finance retains canonical ownership of monetary consequences under L2-F00 and applicable Finance L2; no missing cross-domain contract is invented.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Only the source-supported actors named in Section 6 may perform the H05-defined decisions/actions. Missing frozen authority is an upstream gap.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 override may create a missing state, decision, event, actor authority, or Finance outcome.

## 15.3 Business Audit and Evidence Obligations

Preserve the applicable Research identity, prior/current lifecycle position, decision/action authority, reasons/conditions where required, evidence references, source-domain references, and exact H05 events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported H05 path may complete only to its defined lifecycle position. The full frozen institutional outcome cannot be claimed while any material Section 17 gap remains unresolved.

## 16.2 Postconditions

Research history remains traceable; E/F ownership remains unchanged; unsupported frozen semantics are not approximated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen **approval** is only an H05 execution precondition; no approval state/command/authority/event is defined.
- Frozen **client acceptance** has no authoritative acceptance state/event and is only “where applicable”.
- Frozen **financial reconciliation** lacks a consultancy-specific Research→Finance contract/event and Finance reconciliation return fact.
- Frozen **closed/cancellation** outcomes are missing because H05 consultancy terminates at `Completed`.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions beyond the open issues above.
