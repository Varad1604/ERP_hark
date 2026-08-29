# Compliance Status Change to Infrastructure Compliance Update

# 1. Scenario Metadata

**Scenario ID:** L3-G-301  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Record a changed Infrastructure compliance condition using the G10 Compliance lifecycle and version history so the current compliance position is authoritative while prior compliance remains immutable and traceable.

# 3. Business Outcome

The current compliance state is recorded while historical compliance information remains traceable.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a compliance condition changes.

## 4.2 End Boundary

The scenario ends when the current Compliance Record state/version reflects the source-supported change, prior history is preserved and the exact event appropriate to the completed change is published.

## 4.3 In Scope

- Identification of the current Compliance Record.
- Authorization and policy validation.
- Source-defined Compliance lifecycle progression where applicable.
- Effective-dated/versioned preservation of historical compliance.
- Publication of `ComplianceUpdated`, `ComplianceExpired` or `ComplianceRenewed` when its exact fact occurs.

## 4.4 Out of Scope

- Initial Compliance Record creation.
- Any unlisted Compliance state.
- Automatic Infrastructure restriction as part of ordinary status update.
- Infrastructure inspection execution.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns Compliance Record state, versions, history and completed compliance-change facts. |
| Platform Foundation / A | Owns authorization, event governance and audit evidence without owning compliance. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers an authorized compliance update. | Subject to G10 lifecycle/policy constraints. |
| Compliance Officer | Participates in Infrastructure compliance governance. | No universal approver role is inferred. |

# 7. Preconditions and Entry Criteria

- The physical Infrastructure exists.
- A current Compliance Record exists.
- A compliance condition has changed.
- The update business operation is authorized.

# 8. Trigger

A compliance condition changes.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Infrastructure and current Compliance Record. | Infrastructure Management | The affected compliance fact is resolved. |
| 2 | Confirm authorization for the update. | Platform Foundation / Infrastructure Management | The update may proceed. |
| 3 | Validate the changed condition against applicable G10 compliance policy and lifecycle rules. | Infrastructure Management | Only a source-supported change continues. |
| 4 | Record the current compliance state/version while preserving effective dates, expiry dates, superseded versions and historical versions as applicable. | Infrastructure Management | Current compliance is authoritative and history remains immutable. |
| 5 | Publish the exact applicable event: `ComplianceUpdated`, `ComplianceExpired` or `ComplianceRenewed`. | Infrastructure Management | The completed business fact is published canonically. |
| 6 | Generate immutable audit evidence from the canonical event. | Platform Foundation — Audit | Evidence of the change is preserved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Expiry | Compliance reaches `Expired`. | Record `Expired`, preserve history and publish `ComplianceExpired`. | Expiry is canonical; restriction is separately governed. |
| Renewal | An expired obligation is validly renewed. | Record `Renewed`, preserve prior versions and publish `ComplianceRenewed`. | Current compliance reflects renewal. |
| Other permitted modification | Compliance information changes without expiry/renewal. | Update the current version/state as supported and publish `ComplianceUpdated`. | Prior compliance remains traceable. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization fails. | Current Compliance Record remains unchanged. | A later authorized update may proceed. | Platform Foundation / Infrastructure Management |
| Proposed change is not supported by the canonical lifecycle/policy. | The requested compliance position cannot be recorded. | Resolve the business condition and use only a supported state/transition. | Infrastructure Management |
| Required canonical facts are inconsistent. | A trustworthy update cannot be asserted. | Resolve the source facts first. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Compliance lifecycle is `Draft → Active → Expiring → Expired → Renewed`. | Infrastructure Management | No other compliance state is used. | L2-G10 lifecycle summary |
| Compliance history supports versions, effective dates, expiry dates and superseded/historical versions; history is immutable. | Infrastructure Management | Prior compliance is not overwritten. | L2-G10 §31 Compliance Version Management |
| Compliance events are `ComplianceUpdated`, `ComplianceExpired`, `ComplianceRenewed` for their respective completed facts. | Infrastructure Management | Exact event semantics are retained. | L2-G10 Compliance Events |
| Every business operation requires authorization. | Platform Foundation — Authorization | Unauthorized changes do not proceed. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Record | Existing source-defined lifecycle position | Current state/version changes only as source-supported. | `Active`, `Expiring`, `Expired` or `Renewed`, as applicable. | Infrastructure Management | L2-G10 lifecycle; §31 |
| Infrastructure | Existing source-defined lifecycle/availability | Compliance update alone does not silently change Infrastructure state. | Unchanged unless a separate restriction/review scenario applies. | Infrastructure Management | L2-G10 §§25, 31 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Condition |
|---|---|---|
| `ComplianceUpdated` | Infrastructure Management | General completed compliance modification. |
| `ComplianceExpired` | Infrastructure Management | Compliance reaches `Expired`. |
| `ComplianceRenewed` | Infrastructure Management | Compliance is renewed. |

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A supplies platform authorization, event governance and audit evidence only. No compliance ownership transfers.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Update Compliance Record | Infrastructure Administrator or source-supported Compliance Officer within authorized scope | Must satisfy applicable G10 lifecycle and compliance policy. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal approval or separation-of-duty pattern is defined for every compliance status change; applicable configured compliance policy remains authoritative.

## 15.3 Business Audit and Evidence Obligations

Current and historical Compliance Record versions remain reconstructable under G10. A07 owns immutable audit evidence generated from canonical events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The affected Compliance Record is identified.
- Authorization and G10 validation succeed.
- Current compliance is recorded using only source-defined lifecycle/version rules.
- Historical compliance remains immutable.
- The exact applicable event is published.

## 16.2 Postconditions

- G10 remains owner of current and historical compliance.
- Any compliance-driven Infrastructure restriction remains a separate scenario.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking source conflict was identified.

**Source Assessment:** **PASS**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.
