# Compliance Failure to Controlled Infrastructure Restriction

# 1. Scenario Metadata

**Scenario ID:** L3-G-302  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Apply the configured G10 review or restriction consequence when Infrastructure compliance becomes invalid or expires, without inventing a new Compliance state or automatically disabling Infrastructure where policy does not require it.

# 3. Business Outcome

The applicable infrastructure is placed under the configured restriction or review path.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Infrastructure compliance becomes invalid or expires.

## 4.2 End Boundary

The scenario ends when the affected Infrastructure has entered the source-supported configured review path or, where stricter policy requires it, an authorized source-defined restriction path.

## 4.3 In Scope

- Identification of the affected Infrastructure and Compliance Record.
- Canonical expiry handling.
- Configured review versus restriction decision.
- Authorization before any Infrastructure restriction/change.
- Preservation of compliance history.
- `ComplianceExpired` publication when actual expiry occurs.

## 4.4 Out of Scope

- A locally invented `Invalid` Compliance lifecycle state.
- Automatic shutdown solely because of expiry when stricter policy is absent.
- A locally invented generic compliance-restriction event.
- Compliance renewal or remediation execution.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns Compliance Record, compliance consequence, Infrastructure availability/restriction fact and configured enforcement policy application. |
| Platform Foundation / A | Owns authorization, event governance and audit evidence without deciding Infrastructure compliance policy. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Applies an authorized Infrastructure review/restriction consequence under G10 policy. | Cannot invent or bypass configured enforcement policy. |
| Compliance Officer | Participates in compliance review where source-supported. | No universal approval authority is inferred. |

# 7. Preconditions and Entry Criteria

- The affected Infrastructure exists.
- A Compliance Record exists.
- The compliance condition is expired or otherwise invalid under applicable business policy.
- The configured enforcement policy is available.
- Any Infrastructure restriction/change is authorized before execution.

# 8. Trigger

Infrastructure compliance becomes invalid or expires.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the affected Infrastructure and Compliance Record. | Infrastructure Management | The compliance-failure scope is established. |
| 2 | Determine whether the source-supported condition is expiry or another configured invalid condition without introducing an `Invalid` lifecycle state. | Infrastructure Management | The applicable policy branch is identified. |
| 3 | Where expiry occurred, preserve the canonical `Expired` compliance position and publish `ComplianceExpired`. | Infrastructure Management | The exact expiry fact is recorded. |
| 4 | Evaluate configured enforcement policy to determine whether review alone or a stricter restriction is required. | Infrastructure Management | The frozen review/restriction outcome is selected from authoritative policy. |
| 5 | If a restriction is required, confirm authorization and apply only a source-defined/configured Infrastructure restriction. | Platform Foundation / Infrastructure Management | Infrastructure use is restricted without inventing a state. |
| 6 | If review is required instead, retain the source-supported compliance-attention/review path. | Infrastructure Management | The Infrastructure remains under governed review. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Expiry with default policy | No stricter enforcement applies. | Follow G10 compliance-attention/review treatment; do not automatically disable Infrastructure. | Review path satisfies the frozen outcome. |
| Expiry with stricter policy | Configured policy requires operational restriction. | After authorization, apply an existing source-defined/configured availability restriction. | Restriction is policy-driven. |
| Non-expiry invalid condition | Business policy considers compliance invalid without canonical expiry. | Use only a configured review/restriction path that can be grounded in source facts. | No `Invalid` lifecycle state or invented event is created. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Enforcement policy is unavailable or ambiguous. | A stricter restriction decision cannot be asserted. | Retain only the source-supported review/attention condition until policy is resolved. | Infrastructure Management |
| Authorization denies a proposed restriction. | Infrastructure availability is not changed. | Continue only with source-supported review obligations. | Platform Foundation / Infrastructure Management |
| Canonical Compliance/Infrastructure reference is inconsistent. | The consequence cannot be applied safely. | Resolve the G10 source facts before a later action. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Compliance lifecycle does not define `Invalid`; it defines `Draft → Active → Expiring → Expired → Renewed`. | Infrastructure Management | “Invalid” is retained as a frozen business condition, not converted into a new state. | L2-G10 lifecycle summary |
| Expired compliance shall not automatically disable Infrastructure; stricter enforcement may be configured. | Infrastructure Management | Default handling is review/attention, not unconditional shutdown. | L2-G10 §31 Expired Compliance |
| Availability remains distinct from lifecycle and uses source-defined/configured availability states. | Infrastructure Management | Restriction uses only authoritative availability governance. | L2-G10 §25 |
| Every business operation requires authorization. | Platform Foundation — Authorization | Any restriction/change must be authorized. | L2-A04 AP-01 |
| `ComplianceExpired` is canonical only for actual expiry. | Infrastructure Management | No generic invalid-condition event is invented. | L2-G10 Compliance Events |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Record | `Expiring` for canonical expiry; another existing state for non-expiry condition | Expiry progresses to `Expired`; no `Invalid` state is introduced. | `Expired` for expiry; otherwise unchanged unless a source-defined compliance transition applies. | Infrastructure Management | L2-G10 lifecycle |
| Infrastructure Availability | Existing source-defined availability | May enter configured restriction if policy requires it. | Existing source-defined/configured restriction, or review/attention with no availability change. | Infrastructure Management | L2-G10 §§25, 31 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

`ComplianceExpired` is published by Infrastructure Management only for actual expiry. No exact canonical event is defined for the frozen non-expiry “invalid” branch or for a generic compliance-driven restriction, so none is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A supplies authorization, event governance and audit evidence. G10 retains compliance and Infrastructure consequence ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Apply configured Infrastructure restriction | Infrastructure Administrator | Requires applicable configured G10 enforcement policy and authorization. | Infrastructure Management |
| Participate in compliance review | Compliance Officer | Participation depends on configured compliance governance. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal separation-of-duty or override pattern is defined for every compliance failure.

## 15.3 Business Audit and Evidence Obligations

The Compliance Record and resulting G10 review/restriction facts remain historically traceable. A07 may preserve immutable evidence of canonical completed events such as `ComplianceExpired`.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The affected canonical Infrastructure and Compliance Record are resolved.
- The condition is evaluated without inventing `Invalid`.
- Configured policy determines review versus restriction.
- Any restriction is authorized.
- `ComplianceExpired` is published only when expiry occurred.

## 16.2 Postconditions

- Infrastructure is under the configured review or restriction path.
- G10 remains owner of compliance and Infrastructure availability/restriction facts.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen trigger includes non-expiry “invalid” compliance, but G10 has no `Invalid` Compliance state. | L3 cannot state a canonical lifecycle position for that branch. | G10 should clarify representation of non-expiry invalid compliance. |
| No exact canonical event exists for the non-expiry invalid branch or generic compliance-driven restriction. | Event treatment for that branch cannot be asserted without invention. | Upstream event governance should define an event if canonical publication is required. |

**Source Assessment:** **PASS WITH ISSUES**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
