# Infrastructure Lifecycle Initiation to Active Infrastructure State

# 1. Scenario Metadata

**Scenario ID:** L3-G-314  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Infrastructure decommissioning and operational-consumption scenarios

# 2. Scenario Purpose

Govern the transition of a newly registered and validated physical Infrastructure aggregate into the authorized active/operational lifecycle position without redefining G10 lifecycle states or bypassing upstream lifecycle governance.

# 3. Business Outcome

The infrastructure aggregate enters its authorised active lifecycle state

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a valid physical infrastructure aggregate completes registration and validation.

## 4.2 End Boundary

The scenario ends when the Infrastructure aggregate has entered the authoritative active lifecycle state permitted by G10.

## 4.3 In Scope

- Registered physical Infrastructure owned by G10.
- Lifecycle-entry validation.
- Parent-hierarchy and operational prerequisite validation where applicable.
- Authorization and audit obligations for lifecycle transition.
- Publication of only the exact lifecycle event established by the resolved upstream transition.

## 4.4 Out of Scope

- Re-registration or identity creation after the trigger.
- Inventing `Draft`, `Commissioned` or any other lifecycle state as canonical merely because one G10 section uses it.
- Skipping canonical lifecycle states to reach `Operational`.
- Treating operational-status activation and lifecycle transition as the same concept where G10 distinguishes them.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns physical Infrastructure, lifecycle rules, parent-child consistency, validation and the final active/operational Infrastructure outcome. |
| Platform Foundation / Section A | Owns authorization and audit infrastructure used by the governed lifecycle transition. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates an authorized Infrastructure lifecycle progression after registration/validation. | May perform only source-permitted lifecycle actions within authorized scope. |
| Auditor | Reviews lifecycle evidence. | Read-only evidence role. |

No additional activation approver is invented where G10 does not assign one canonically.

# 7. Preconditions and Entry Criteria

- A valid physical Infrastructure aggregate has completed registration and validation.
- Required parent Infrastructure exists and satisfies applicable parent lifecycle requirements.
- Mandatory operational validations defined by G10 are satisfied before `Operational` may be entered.
- The requested lifecycle action is authorized.

# 8. Trigger

A valid physical infrastructure aggregate completes registration and validation

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the registered G10-owned Infrastructure aggregate and its current lifecycle state. | Infrastructure Management | Lifecycle progression starts from an authoritative state. |
| 2 | Validate parent hierarchy, required metadata, capacity, area, operational status and other mandatory activation conditions applicable to the aggregate. | Infrastructure Management | Activation prerequisites are established. |
| 3 | Validate that the next requested lifecycle transition is permitted and authorized. | Infrastructure Management using Section A authorization capability | Invalid/skipped transitions are prevented. |
| 4 | Apply only the next lifecycle transition established by the authoritative G10 lifecycle model. | Infrastructure Management | Canonical lifecycle integrity is preserved. |
| 5 | Continue authorized lifecycle progression until the authoritative active/operational state is reached. | Infrastructure Management | The frozen final outcome would be reached only if the upstream starting state and permitted sequence are unambiguous. |

The reviewed locked G10 source does not provide one unambiguous starting state and progression sequence after registration; therefore L3 cannot safely complete this flow.

# 10. Alternate Business Flows

No complete scenario-specific alternate flow is authored because the conflicting upstream lifecycle definitions must be resolved first.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Mandatory activation validation fails | Infrastructure cannot enter Operational. | Preserve the current lifecycle state until the failed business condition is corrected. | Infrastructure Management |
| Requested transition skips an authoritative lifecycle state | Lifecycle integrity would be violated. | Reject the transition and follow only the resolved canonical sequence. | Infrastructure Management |
| Authorization fails | Lifecycle progression cannot proceed. | Preserve the existing state. | Platform Foundation / Infrastructure Management |
| Source sections disagree on the current/next lifecycle state | L3 cannot choose a transition without redesigning G10. | Escalate the lifecycle contradiction for upstream resolution. | Architecture governance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Canonical lifecycle states are `Proposed`, `Planned`, `Approved`, `Under Construction`, `Operational`, `Under Maintenance`, `Under Renovation`, `Temporarily Closed`, `Decommissioned`, `Archived`. | Infrastructure Management | L3 may use only these states unless upstream formally resolves otherwise. | L2-G10 §11A.52 |
| `Proposed → Planned → Approved → Under Construction → Operational` is the canonical forward path to Operational. | Infrastructure Management | Skipping intermediate states is prohibited. | L2-G10 §11A.53; BR-LC-02 |
| Only Operational Infrastructure may participate in institutional operations. | Infrastructure Management | The final active outcome corresponds to source-defined operational availability only after valid lifecycle entry. | L2-G10 BR-OP-01 |
| Mandatory activation validations must pass before an Infrastructure entity becomes Operational. | Infrastructure Management | Failed validation blocks activation. | L2-G10 §13A.78 |
| Every business operation must be authorized before execution. | Platform Foundation — Authorization | Unauthorized lifecycle transition is prohibited. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | **Conflicted upstream:** §11A starts `Proposed`; §16 registration establishes `Draft`; WF-INF-001 says `Proposed or Planned`; §40/§46 use additional `Draft`/`Commissioned` style models. | Progression to active/operational Infrastructure must follow one authoritative lifecycle. | `Operational` is the canonical operational lifecycle state in §11A, but the valid starting point/path after registration is unresolved. | Infrastructure Management | L2-G10 §§11A, 16, 22A/WF-INF-001, 40, 46 |

No local lifecycle reconciliation is made.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10 associates the canonical lifecycle transition to `Operational` with `InfrastructureOperational`, while separate operational-status/activation workflows also use `InfrastructureActivated`. Because the scenario's upstream lifecycle/activation semantics are internally inconsistent, this Draft does not silently choose one event as the scenario-completion event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Platform Foundation | Infrastructure Management | Authorization decision and audit capability | Govern lifecycle transition and preserve evidence. | L2-A04; L2-A07 |

No other material cross-domain hand-off is required by the frozen participating-domain set.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate Infrastructure lifecycle progression | Infrastructure Administrator | Must operate within authorized Infrastructure scope and G10 lifecycle rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

G10 states that unlisted lifecycle transitions require explicit administrative override and audit approval, but this scenario does not rely on an override. No additional separation-of-duty rule is introduced.

## 15.3 Business Audit and Evidence Obligations

Each permitted lifecycle transition must be auditable. G10 requires lifecycle-transition evidence including previous/new state and the business authority/reason context; Platform Audit owns the immutable audit evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The registered Infrastructure aggregate is resolved canonically.
- All mandatory validations pass.
- Every lifecycle transition used is authoritative and authorized.
- The aggregate reaches the source-defined active/operational lifecycle position.
- Required lifecycle audit evidence and canonical event publication are preserved.

The authoritative starting state and transition sequence after registration are currently contradictory, so these criteria cannot all be proven safely.

## 16.2 Postconditions

- No skipped or invented lifecycle transition has been authored.
- Infrastructure remains under G10 ownership.
- Full successful completion remains blocked pending lifecycle reconciliation.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 §11A defines the canonical lifecycle beginning `Proposed → Planned → Approved → Under Construction → Operational`, while §16 registration sets newly registered Infrastructure to `Draft`, §17 describes `Draft → Operational`, WF-INF-001 says initial lifecycle `Proposed or Planned`, and §§40/46 use additional `Draft`/`Commissioned` lifecycle variants. | Material lifecycle conflict: the scenario trigger occurs immediately after registration/validation, but L3 cannot determine the valid entry state or sequence to the final active state without choosing among contradictory locked definitions. | Upstream G10 must reconcile registration post-state and activation/lifecycle progression to one canonical model. |
| G10 distinguishes lifecycle from operational status/availability, yet lifecycle completion uses `InfrastructureOperational` while activation/status workflows use `InfrastructureActivated`. | Material event/semantic ambiguity at scenario completion. | Upstream G10 must define which event corresponds to this frozen lifecycle-initiation outcome. |

**Architecture assessment:** `BLOCKED`

The conflict directly affects lifecycle states, permitted transitions, event semantics and the final outcome.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions beyond the upstream blocking issues.
