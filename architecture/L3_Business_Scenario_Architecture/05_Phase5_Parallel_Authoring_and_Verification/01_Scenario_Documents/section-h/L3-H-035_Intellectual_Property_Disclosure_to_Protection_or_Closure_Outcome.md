# Intellectual Property Disclosure to Protection or Closure Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; L2-F00 — Finance Domain Model & Foundation  
**Referenced By:** Related Research governance, audit, reporting, and downstream institutional use

# 2. Scenario Purpose

H05 supports Patent/IP registration, filing, verified statutory-status updates, and lifecycle closure; Finance alone owns monetary consequences.

# 3. Business Outcome

Ownership review, protection decision, filing, commercialization, abandonment, or closure is completed under policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A potentially protectable research result is disclosed.

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

Source-supported actors: Principal Investigator / Inventor; Research Office.

# 7. Preconditions and Entry Criteria

- The applicable Research aggregate and authoritative source references exist as required by H05.
- Required institutional policy/configuration and evidence are available.
- Source-domain ownership remains unchanged.

# 8. Trigger

A potentially protectable research result is disclosed

# 9. Main Business Flow

Validate inventor/project/ownership-policy context; register Patent/IP metadata; submit an approved filing; record verified statutory status; close an eligible lifecycle; stop before unsupported disclosure/decision/commercialization/abandonment outcomes.

# 10. Alternate Business Flows

Rejected or expired statutory status may proceed to closure when H05 closure criteria are met; administrative closure is permitted by H05.

# 11. Failure, Interruption and Business Recovery

Invalid inventor/ownership context blocks registration; incomplete filing blocks filing; commercialization/financial consequences cannot be locally invented.

# 12. Business Rules and Decision Constraints

Configured IP ownership policy applies; at least one inventor is required; patent-office status is authoritative; filing/status history is immutable; Finance owns accounting and monetary consequences.

# 13. State and Lifecycle Impact

`Draft` → `Filed` → `Filed` / `Under Examination` / `Granted` / `Rejected`; `Granted` / `Rejected` / `Expired` → `Closed`.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H05 defines: `PatentRegistered`, `PatentFiled`, `PatentStatusUpdated`, `PatentLifecycleClosed`.

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

- Frozen trigger starts at **disclosure**, but H05 defines no IP Disclosure aggregate/event.
- Frozen **ownership review** and **protection decision** are not defined as H05 decisions/states/events.
- Frozen **commercialization** has no H05 workflow or Research→Finance business hand-off.
- H05 names “abandoned” as a closure trigger, but its Patent state machine has no `Abandoned` state/transition/event.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions beyond the open issues above.
