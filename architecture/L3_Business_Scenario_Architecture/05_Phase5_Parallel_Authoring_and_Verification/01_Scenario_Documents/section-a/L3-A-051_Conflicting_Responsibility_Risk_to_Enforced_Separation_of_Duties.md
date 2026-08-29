# Conflicting Responsibility Risk to Enforced Separation of Duties

# 1. Scenario Metadata

**Scenario ID:** L3-A-051  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authorization & Policy  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** All protected business operations requiring Separation of Duties evaluation

---

# 2. Scenario Purpose

Establish a governed Separation of Duties Policy for conflicting responsibilities, apply platform defaults and permitted institution overrides, and enforce the policy during centralized authorization without transferring business-operation ownership.

---

# 3. Business Outcome

A Separation of Duties Policy is configured with platform defaults and permitted institution overrides, evaluated during authorization, and conflicting attempts are denied unless an approved policy explicitly permits continuation, with the outcome auditable

Locked L2-A04 defines SoD evaluation principles but omits the policy-configuration workflow, lifecycle states, commands, canonical events, publishers and approval authority for institution overrides or permitted continuation. This scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an institution requires that conflicting business responsibilities cannot be exercised by the same user within the same business process.

## 4.2 End Boundary

The intended end boundary is reached when the applicable SoD policy is governed and active, centralized authorization evaluates it, prohibited conflicts are denied, explicitly permitted continuation follows approved policy, and the outcome is auditable. Exact policy-governance completion is blocked by missing L2-A04 contracts.

## 4.3 In Scope

- Definition of conflicting responsibilities and applicable process scope.
- Platform defaults and permitted institution overrides.
- Validation and approval of SoD policy configuration.
- Evaluation of Roles, Delegations, Temporary Access and workflow context.
- Denial of prohibited conflicting attempts.
- Policy-approved continuation where explicitly permitted.
- Immutable evidence of policy and evaluation outcomes.

## 4.4 Out of Scope

- Creating organizational positions or responsibilities.
- Assigning or revoking Roles, Delegations or Temporary Access.
- Authentication.
- Performing the protected business operation.
- Inventing policy states, commands, events, publishers, approvers or override authorities.
- Technical IAM, rule-engine, API, database or UI design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Related | L3-A-010 — Institutional Responsibility Assignment to Effective Authorized Access | Role Assignment is a possible SoD input, not owned by this scenario. |
| Related | L3-A-011 — Responsibility Delegation to Automatic Authority Reversion | Delegations participate in SoD evaluation but retain their own lifecycle. |
| Related | L3-A-012 — Temporary Access Request to Governed Access Expiry | Temporary Access participates in SoD evaluation but retains its own lifecycle. |
| Related | L3-A-052 — Authorization Policy Change to Governed Active Policy Version | General Authorization Policy version governance remains distinct from SoD policy configuration. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authorization & Policy / Section A | Primary owner. Owns SoD Policy, evaluation, explicit-deny precedence and the Authorization Decision. |
| Person & Identity / Section A | Supplies canonical subject references. |
| Authentication / Section A | Supplies authenticated context for a later protected request. |
| Event Infrastructure / Section A | Distributes canonical events after policy events are defined. |
| Audit Infrastructure / Section A | Preserves immutable policy and decision evidence. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authenticated User | Initiates a protected request later evaluated under SoD policy. | Has no authority to bypass a conflict. |
| Authorization Decision Point | Evaluates applicable responsibilities, policies, context and explicit deny. | Produces the centralized decision defined by L2-A04. |

L2-A04 does not name the actors authorized to configure platform defaults, propose institution overrides, approve the policy or authorize an explicit continuation. No administrator, approver or override authority is invented.

---

# 7. Preconditions and Entry Criteria

- The Tenant, institution and business-process scope can be resolved.
- Conflicting responsibilities and prohibited combinations are known.
- Platform-default and institution-override boundaries can be determined.
- Relevant Role Assignments, Delegations and Temporary Access facts can be resolved for evaluation.
- A protected request supplies authenticated subject, action, resource and context.
- Policy configuration and approval are expected to exist, although their canonical workflow and actors are undefined.

---

# 8. Trigger

An institution requires that conflicting business responsibilities cannot be exercised by the same user within the same business process

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the requirement to prevent a defined conflicting-responsibility combination. | Authorization & Policy | A SoD governance need is identified. |
| 2 | Resolve the Tenant, institution, business process, actions and responsibility scope. | Authorization & Policy | The policy boundary is established. |
| 3 | Identify the conflicting Roles or responsibilities and applicable combinations. | Authorization & Policy — SoD Policy | Conflict semantics are defined. |
| 4 | Apply platform-default constraints and determine which institution overrides are permitted. | Authorization & Policy — SoD Policy | The configuration boundary is established. |
| 5 | Define any conditions under which an approved policy may explicitly permit continuation. | Authorization & Policy — SoD Policy | Intended exception semantics are bounded by policy. |
| 6 | Validate determinism, Tenant isolation, explicit-deny precedence and least privilege. | Authorization & Policy | The intended policy is internally consistent. |
| 7 | Complete required policy and override approval. | Authorization & Policy | Intended approval is obtained; actors and transition are undefined upstream. |
| 8 | Make the governed SoD Policy active for centralized evaluation. | Authorization & Policy — SoD Policy | Intended active availability is reached; state and event are undefined upstream. |
| 9 | On a protected request, resolve current Roles, Delegations, Temporary Access and workflow context. | Authorization & Policy | All applicable responsibility facts are available. |
| 10 | Evaluate the SoD Policy within the centralized authorization sequence. | Authorization Decision Point | A conflict is absent, prohibited or explicitly permitted by approved policy. |
| 11 | Deny a prohibited conflict; permit continuation only where an approved policy explicitly allows it and all other authorization rules succeed. | Authorization Decision Point | The SoD control is enforced without independently granting access. |
| 12 | Preserve policy version, inputs, conflict result, decision, actor, timestamp and correlation evidence. | Authorization & Policy; Audit Infrastructure | The outcome is auditable. |

Steps 7–8 cannot be expressed as canonical transitions because L2-A04 provides no SoD policy lifecycle, command or event model.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| No conflict | Resolved responsibilities do not match a prohibited combination. | Continue the remaining authorization evaluation. | SoD does not deny; it does not independently allow the operation. | Authorization & Policy |
| Platform default applies | No permitted institution override exists. | Evaluate the platform-default constraint. | The default governs the decision. | Authorization & Policy |
| Permitted institution override | Platform policy allows a scoped override. | Validate and apply only the approved override for the institution. | The scoped policy governs; the approval authority remains undefined upstream. | Authorization & Policy |
| Explicitly permitted continuation | A conflict exists but approved policy explicitly permits continuation under defined conditions. | Evaluate those conditions and all remaining authorization controls. | Continuation is possible only if the complete authorization decision allows it. | Authorization & Policy |
| Delegated or temporary responsibility | Conflict arises through Delegation or Temporary Access. | Include it in SoD evaluation like an assigned responsibility. | Prohibited conflict is denied. | Authorization & Policy |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Scope or responsibility definition is ambiguous. | Deterministic evaluation is impossible. | Do not activate the intended policy. | Authorization & Policy |
| Institution override exceeds permitted boundary. | Platform constraints would be weakened without authority. | Reject the override. | Authorization & Policy |
| Required approval is absent. | Policy or exception cannot be governed as active. | Do not apply the unapproved configuration. | Authorization & Policy |
| Relevant responsibility facts cannot be resolved. | SoD risk cannot be evaluated reliably. | Deny or fail closed according to established authorization policy; no allow is inferred. | Authorization & Policy |
| Canonical policy event is unavailable. | Consumers cannot receive an authoritative configuration fact. | Do not invent an event; obtain an L2-A04 definition. | Authorization & Policy |

No technical retry, rollback or implementation recovery is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| SoD evaluation is centralized in Authorization & Policy. | Authorization & Policy | Business domains do not independently decide SoD. | L2-A04 — Authorization Evaluation Pipeline |
| SoD considers Roles, Delegations, Temporary Access, workflow context and configuration. | Authorization & Policy | All current responsibility sources participate. | L2-A04 — SoD Evaluation |
| Explicit Deny overrides Allow. | Authorization & Policy | A prohibited conflict cannot be overcome by an Allow. | L2-A04 — AP-06 |
| A conflicting attempt is denied unless approved policy explicitly permits continuation. | Authorization & Policy | Exceptions must be policy-governed. | Frozen outcome; L2-A04 — SoD Policy |
| Platform defaults and permitted institution overrides govern configuration. | Authorization & Policy | Institution rules cannot exceed the permitted boundary. | L2-A04 — Policy Configuration |
| Authentication success does not grant authorization. | Authentication; Authorization & Policy | Authenticated requests still undergo SoD evaluation. | L2-A03; L2-A04 |
| Every authorization decision must be auditable. | Authorization & Policy; Audit Infrastructure | SoD inputs and outcomes remain traceable. | L2-A04 — AP-11 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Required Scenario Impact | Intended Result | Canonical Owner |
|---|---|---|---|---|
| Separation of Duties Policy | Configuration absent or requires change | Define, validate, approve and activate. | Governed active policy; canonical states are undefined. | Authorization & Policy |
| Role Assignment / Delegation / Temporary Access | Existing | Referenced during evaluation only. | Unchanged. | Authorization & Policy |
| Authorization Decision | Protected request received | Record deterministic decision including SoD result. | Allow or Deny under complete evaluation. | Authorization & Policy |
| Protected business object | Existing | No operation occurs if denied. | Unchanged on denial. | Owning business domain |

L2-A04 defines no SoD Policy lifecycle-state model; no requested, approved or active state is invented.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Event Determination

| Required Business Fact | Source Resolution Result | Scenario Treatment |
|---|---|---|
| SoD Policy configured or approved | No canonical event or publisher defined. | No event is invented. |
| SoD Policy activated | No canonical event or publisher defined. | No event is invented. |
| SoD conflict evaluated | `Separation of Duties Violation` appears as a human-readable auditable activity, not a canonical event contract. | Preserve audit evidence; do not claim a canonical event. |

The final protected-request Authorization Decision remains governed by L2-A04, but L2-A04 does not define an exact canonical SoD policy-governance event catalogue.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Identity and authenticated context | Person & Identity; Authentication | Authorization & Policy | Canonical subject and authenticated request. |
| Responsibility facts | Role Assignment, Delegation, Temporary Access | SoD Evaluation | Current effective responsibilities. |
| Workflow and resource context | Requesting business domain | SoD Evaluation | Process, action, resource and scope. |
| SoD evaluation result | SoD Policy | Authorization Decision | Conflict outcome under the applicable policy version. |
| Authorization Decision | Authorization & Policy | Requesting business domain | Final Allow or Deny. |
| Audit input | Authorization & Policy | Audit Infrastructure | Policy, inputs, conflict and decision evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- SoD evaluation is one mandatory part of the centralized authorization sequence.
- Absence of a conflict does not independently grant access.
- Explicit Deny retains precedence.

## 15.2 Approval and Separation of Duty

- Platform defaults and institution overrides require governance.
- L2-A04 does not name the override requestor, policy approver or authority that can approve explicit continuation.
- No override or approval authority is inferred.

## 15.3 Audit Obligations

Policy version, defaults, overrides, approval evidence, subject, effective responsibilities, workflow context, detected conflict, continuation basis, final decision, actor, timestamp and correlation must be auditable.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are:

- Conflicting responsibilities and scope were defined.
- Platform defaults and permitted institution overrides were applied.
- The policy was validated, approved and made active.
- Current responsibility and workflow context were evaluated during authorization.
- Prohibited conflicts were denied.
- Continuation occurred only when approved policy explicitly permitted it and all other controls allowed it.
- The outcome was auditable.

These criteria cannot be validated canonically because the SoD policy workflow, states, commands, events, publishers and approval authorities are missing from L2-A04. The scenario remains `BLOCKED`.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Precise Scenario Impact | Required Resolution |
|---|---|---|---|---|
| A051-BLOCKER-01 | Blocking | L2-A04 defines SoD evaluation semantics but no SoD Policy configuration workflow, commands or lifecycle states. | Exact progression from requirement to active policy cannot be specified. | Define the canonical workflow, commands and state transitions. |
| A051-BLOCKER-02 | Blocking | No canonical SoD Policy configuration, approval or activation events and publishers are defined. | The governed policy outcome cannot produce an authoritative event fact. | Define exact events, publication conditions and sole publishers. |
| A051-BLOCKER-03 | Blocking | The authorities for platform defaults, institution overrides, policy approval and explicitly permitted continuation are unnamed. | Approval and exception governance cannot be allocated without invention. | Define actors, authority boundaries and approval rules. |

No unsupported actor, state, command, event, publisher, exception authority or implementation mechanism is assumed.
