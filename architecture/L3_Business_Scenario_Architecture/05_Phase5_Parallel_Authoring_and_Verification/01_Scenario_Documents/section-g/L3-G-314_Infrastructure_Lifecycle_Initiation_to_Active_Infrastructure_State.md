# Infrastructure Lifecycle Initiation to Active Infrastructure State

# 1. Scenario Metadata

**Scenario ID:** L3-G-314  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Infrastructure operational-consumption and retirement scenarios

# 2. Scenario Purpose

Govern progression of a newly registered and validated physical Infrastructure aggregate into its authorised active lifecycle position without resolving contradictory locked G10 lifecycle models locally.

# 3. Business Outcome

The infrastructure aggregate enters its authorised active lifecycle state

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary
The scenario starts when a valid physical infrastructure aggregate completes registration and validation.

## 4.2 End Boundary
The scenario requires the aggregate to enter the authoritative active lifecycle state permitted by G10.

## 4.3 In Scope
Registered G10 Infrastructure; activation/lifecycle validation; parent/lifecycle prerequisites; authorization/audit; canonical transition event once upstream semantics are resolved.

## 4.4 Out of Scope
Re-registration; invented `Draft`, `Commissioned` or other states; skipped transitions; silently treating operational-status activation and lifecycle transition as identical.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| G10 | Owns physical Infrastructure, lifecycle, activation constraints and final physical lifecycle outcome. |
| Section A | Owns authorization and audit capability. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates source-permitted lifecycle progression after registration/validation. | Authorized G10 scope only. |
| Auditor | Reviews lifecycle evidence. | Read-only evidence role. |

No additional activation approver is invented.

# 7. Preconditions and Entry Criteria

- A valid physical Infrastructure aggregate has completed registration and validation.
- Required parent Infrastructure exists and satisfies applicable source rules.
- Mandatory operational validations must pass before Operational may be entered.
- The lifecycle action is authorized.

# 8. Trigger

A valid physical infrastructure aggregate completes registration and validation

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve registered G10 aggregate and current lifecycle state. | G10 | Progression starts from authoritative state if one can be determined. |
| 2 | Validate parent hierarchy, required metadata and operational prerequisites. | G10 | Activation prerequisites are assessed. |
| 3 | Validate requested lifecycle action and authorization. | G10 / A | Invalid/skipped transitions are prevented. |
| 4 | Apply only the next transition in the authoritative canonical lifecycle. | G10 | Lifecycle integrity would be preserved. |
| 5 | Reach the source-defined active lifecycle state and publish the corresponding canonical fact. | G10 | Frozen outcome requires upstream lifecycle/event semantics to be unambiguous. |

Locked G10 sources disagree on both the registration post-state and progression semantics, so this flow cannot be completed safely at L3.

# 10. Alternate Business Flows

No complete alternate flow is authored while the controlling lifecycle model remains contradictory.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Mandatory validation fails | Infrastructure cannot enter active/Operational position. | Preserve current valid state until business condition is corrected. | G10 |
| Requested transition skips a canonical state | Lifecycle integrity would be violated. | Reject the transition. | G10 |
| Source sections disagree on current/next state | L3 cannot choose a path. | Resolve contradiction upstream. | Architecture governance |
| Authorization fails | Progression cannot proceed. | Preserve existing state. | A / G10 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| §11A canonical lifecycle includes `Proposed → Planned → Approved → Under Construction → Operational`. | G10 | L3 cannot skip those transitions when §11A controls. | L2-G10 §11A |
| Registration text states newly registered Infrastructure is `Draft`. | G10 | Frozen trigger may begin from a state absent from §11A canonical set. | L2-G10 registration sections |
| Other G10 activation/entity matrices use `Draft`, `Approved`, `Commissioned`, `Closed`, `Reserved` and `Available` in variant lifecycle models. | G10 | Locked internal lifecycle definitions conflict. | L2-G10 activation/§46 sections |
| Only Operational Infrastructure may participate in institutional operations. | G10 | Final active meaning must not be inferred prematurely. | L2-G10 operational rules |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | **Conflicted:** `Draft` after registration versus `Proposed`/`Planned` and entity-specific variants | Must progress only through one authoritative lifecycle. | `Operational` is present in G10, but valid entry/path after registration is unresolved. | G10 | L2-G10 §§11A, registration, activation, 46 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events
G10 maps `Activate Infrastructure` to `InfrastructureActivated`, generic lifecycle transition to `InfrastructureLifecycleChanged`, and also defines lifecycle facts such as `InfrastructureOperational`. Because the frozen lifecycle-initiation semantics are internally inconsistent, no one event is silently selected as completion event.

## 14.2 Business Contracts and Cross-Domain Hand-offs
Section A supplies authorization/audit capability. No other material frozen-domain hand-off is required.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate lifecycle progression | Infrastructure Administrator | Must follow resolved G10 lifecycle and authorization. | G10 |

## 15.2 Separation of Duties, Overrides and Exceptions
No override is used to reconcile the lifecycle conflict. No additional SoD rule is invented.

## 15.3 Business Audit and Evidence Obligations
Each valid lifecycle change must remain auditable with prior/new state and authority/reason context.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria
Registered Infrastructure resolved; mandatory validations pass; every transition is authoritative; source-defined active state is reached; correct canonical event is published. These cannot all be proven while entry-state/path/event semantics conflict.

## 16.2 Postconditions
No skipped/invented transition is authored; G10 ownership is preserved; full completion remains blocked.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 §11A canonical lifecycle conflicts with registration `Draft`, WF-INF-001 Proposed/Planned, activation `Draft`, and §46 entity-specific variants. | Material lifecycle conflict affects start state, valid sequence and final-outcome correctness. | Reconcile G10 to one controlling lifecycle model. |
| Lifecycle/activation semantics use `InfrastructureOperational`, `InfrastructureActivated` and `InfrastructureLifecycleChanged` for related but distinct facts. | Completion-event semantics are ambiguous. | Define which canonical fact corresponds to this frozen scenario. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions
No assumptions.

## 17.3 Deferred Questions
No additional deferred questions beyond blocking issues.
