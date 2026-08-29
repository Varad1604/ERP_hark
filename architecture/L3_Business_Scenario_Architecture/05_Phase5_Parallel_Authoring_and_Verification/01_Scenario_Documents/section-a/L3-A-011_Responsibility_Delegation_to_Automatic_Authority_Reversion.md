# Responsibility Delegation to Automatic Authority Reversion

# 1. Scenario Metadata

**Scenario ID:** L3-A-011  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authorization & Policy  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L2-E02 — Position & Assignment Workflows for delegation-boundary validation  
**Referenced By:** Authorization evaluations that may include active delegated authority

---

# 2. Scenario Purpose

Govern a time-bound Authorization Delegation so a defined authorization-layer responsibility may be exercised by another eligible Person during a permitted period and then cease automatically on expiry or through governed revocation, without changing Role Assignments, Roles or Permissions.

---

# 3. Business Outcome

Approved scoped responsibility is delegated for the permitted period and automatically expires or is revoked without permanently changing role assignments or definitions.

Locked L2-A04 does not define the canonical approval authority, commands, business contracts, event names or publishers needed to complete this outcome exactly. The boundary between A04 Authorization Delegation and E02 Assignment Delegation is also unresolved for the frozen trigger. This scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized responsibility must temporarily be exercised by another eligible Person during absence, leave, training or temporary assignment.

## 4.2 End Boundary

The intended end boundary is reached when the approved Authorization Delegation has reached `Expired` or `Revoked`, the delegated authority no longer contributes to authorization evaluation, the original authorization ownership remains unchanged, and the delegation history is auditable.

## 4.3 In Scope

- Resolution of the single-Tenant boundary and the delegator and delegate identity references.
- Definition of effective date, expiry date and delegated scope.
- Delegation-policy, least-privilege and Separation of Duties evaluation.
- Required approval.
- Creation, approval and activation of the Delegation.
- Inclusion of an `Active` Delegation in authorization context.
- Automatic expiry or governed revocation.
- Removal of the delegated authority from subsequent authorization evaluation.
- Audit evidence for creation, approval, activation, use, expiry or revocation.

## 4.4 Out of Scope

- E02 Assignment Delegation of Position or Responsibility Assignment authority.
- Creation, modification, suspension, expiry or revocation of a Role Assignment.
- Changes to Role or Permission definitions.
- Temporary Access Grants.
- Break-Glass emergency access.
- Execution of a protected business operation beyond determining that delegated authority may participate in its authorization evaluation.
- Authentication Session creation or Identity lifecycle changes.
- Technical RBAC, IAM, APIs, database design, UI or infrastructure.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-A-010 — Institutional Responsibility Assignment to Effective Authorized Access | A011 must not permanently alter the underlying Role Assignment. |
| Related | L3-A-012 — Temporary Access Request to Governed Access Expiry | A012 grants exceptional temporary access rather than transferring an existing responsibility. |
| Related | L3-A-013 — Emergency Access Request to Audited Emergency Access Closure | A013 governs emergency Break-Glass access rather than normal delegation. |
| Related control | L3-A-051 — Conflicting Responsibility Risk to Enforced Separation of Duties | Delegated authority participates in SoD evaluation. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authorization & Policy / Section A | Primary owner. Owns Authorization Delegation, its lifecycle, policy evaluation, SoD evaluation and its contribution to Authorization Decisions. |
| Platform & Tenant / Section A | Supplies the canonical Tenant and institutional scope context. |
| Person & Identity / Section A | Supplies canonical Person, Identity and User Account references without owning Delegation. |
| Authentication / Section A | Supplies authenticated context for protected requests; authentication does not create delegated authority. |
| Event Infrastructure / Section A | Governs registered event publication and distribution without owning delegation facts. |
| Audit Infrastructure / Section A | Owns immutable audit evidence generated from canonical business events. |

The frozen Participating Domains value remains exactly `A`.

L2-E02 is referenced only to validate the ownership boundary. It owns Assignment Delegation and organizational authority; it is not added as a participant because that would change the frozen scenario architecture.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Delegator | Identifies the authorization-layer responsibility and scope proposed for temporary delegation. | Initiates the Delegation as an authorized user; may not transfer more authority than is validly held or permitted by policy. |
| Delegate | Receives the approved delegated responsibility for the permitted scope and period. | Exercises only authority made available through the `Active` Delegation and subsequent policy evaluation. |

L2-A04 explicitly names `Delegator` and `Delegate` but does not name the actor who approves, activates or revokes a Delegation. No approval authority is invented.

---

# 7. Preconditions and Entry Criteria

- The proposed Delegation belongs to exactly one resolved Tenant.
- The delegator and delegate can be resolved to canonical A02 Person/Identity references.
- Both participants satisfy the upstream requirement to be authorized users.
- The delegator has an existing authorization-layer responsibility eligible for policy-controlled delegation.
- The delegated scope, effective date and expiry date are available.
- Applicable Delegation, least-privilege, Authorization and SoD policies can be evaluated.
- The proposed Delegation does not require modification of Role, Permission or Role Assignment definitions.

---

# 8. Trigger

An authorized responsibility must temporarily be exercised by another eligible Person during absence, leave, training or temporary assignment.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the request to delegate a defined authorization-layer responsibility for a temporary period. | Delegator; Authorization & Policy | A governed Delegation request begins. |
| 2 | Resolve the Tenant and canonical delegator and delegate identity references. | Platform & Tenant; Person & Identity; Authorization & Policy | The Delegation is associated with one Tenant and two identifiable participants. |
| 3 | Define the effective date, expiry date and delegated scope without changing the underlying Role Assignment, Role or Permissions. | Delegator; Authorization & Policy — Delegation | The proposed temporary authority boundary is established. |
| 4 | Evaluate Delegation validity, duration, scope, approval and expiry requirements under the applicable Authorization Policy. | Authorization & Policy | Policy eligibility is determined. |
| 5 | Evaluate least privilege and applicable SoD Policies using assigned, delegated and temporary responsibilities. | Authorization & Policy | A prohibited conflict is detected or the Delegation remains eligible. |
| 6 | Create the Delegation in `Requested` and preserve the auditable `Delegation Created` activity. | Authorization & Policy — Delegation | The canonical Delegation lifecycle begins. |
| 7 | Obtain the required approval and complete `Requested → Approved`. | Authorization & Policy | The Delegation is approved; the approving actor and approval contract are undefined upstream. |
| 8 | Complete `Approved → Active` on the permitted effective date and preserve the auditable `Delegation Activated` activity. | Authorization & Policy — Delegation | The delegated responsibility becomes available as additional authorization context. |
| 9 | Include the `Active` Delegation in subsequent authorization evaluation without bypassing Authorization Policy, SoD or Explicit Deny. | Authorization & Policy | Delegated authority may contribute to a later Authorization Decision. |
| 10 | On the expiry condition, complete `Active → Expired`; where governed revocation occurs first, complete `Active → Revoked`. | Authorization & Policy — Delegation | The Delegation reaches a terminal lifecycle position. |
| 11 | Exclude the expired or revoked Delegation from subsequent authorization evaluation while leaving Role Assignments and definitions unchanged. | Authorization & Policy | Authority automatically reverts to the non-delegated authorization context. |
| 12 | Preserve immutable delegation, approval, activation, expiry or revocation and use history. | Authorization & Policy; Audit Infrastructure | The complete temporary-authority outcome remains traceable. |

Steps 7, 10 and 12 cannot be finalized as exact canonical collaboration because the approval authority, command contracts, expiry event and event publishers are not defined by locked L2-A04.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Future-effective Delegation | Approval occurs before the effective date. | Retain the Delegation in `Approved` until the effective date and then complete `Approved → Active`. | Delegated authority does not participate before activation. | Authorization & Policy |
| Governed early revocation | An `Active` Delegation must cease before its expiry date. | Complete `Active → Revoked`, preserve the auditable `Delegation Revoked` activity and remove it from authorization context. | Authority reverts without waiting for automatic expiry. | Authorization & Policy |

Temporary Access and Break-Glass are separate scenarios, not alternate A011 paths.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant or participant identity cannot be resolved. | The Delegation cannot preserve Tenant or identity integrity. | Reject the request without creating active delegated authority. | Platform & Tenant; Person & Identity; Authorization & Policy |
| Proposed Delegation crosses Tenant boundaries. | The Delegation violates a platform invariant. | Reject the request. | Authorization & Policy |
| Scope or duration is invalid. | Policy-controlled temporary authority cannot be established. | Correct the request before approval; no active Delegation is produced. | Authorization & Policy |
| Delegation is not permitted by policy. | The responsibility cannot be delegated through A011. | Deny progression without changing the existing authorization context. | Authorization & Policy |
| SoD conflict is prohibited. | Delegation would create conflicting responsibilities. | Deny the Delegation unless an approved policy explicitly permits continuation. | Authorization & Policy |
| Required approval is not granted. | `Requested → Approved` cannot complete. | Leave the Delegation outside `Active`; no delegated authority may be used. | Authorization & Policy |
| Activation does not complete. | The Delegation cannot contribute to authorization evaluation. | Do not treat the Delegation as active. The exact recovery workflow is undefined upstream. | Authorization & Policy |
| Expiry or revocation cannot be represented by a canonical event. | Downstream evidence and consumers lack a governed terminal business fact. | Do not invent an event or publisher; obtain an upstream L2-A04 event decision. | Authorization & Policy |

No technical retry, rollback or implementation recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Authorization Delegation is owned exclusively by L2-A04. | Authorization & Policy | A011 cannot create or change E02 Assignment Delegation. | L2-A04 §3 — RC-01 boundary clarification |
| Delegation is policy-controlled, time-bound and fully auditable. | Authorization & Policy | The Delegation cannot become indefinite or bypass policy. | L2-A04 §§10.5, 62 and 66 |
| Delegation supports Delegator, Delegate, effective date, expiry date, delegated scope, approval status, revocation and audit trail. | Authorization & Policy | These facts govern the Delegation outcome. | L2-A04 §62 — Delegation Rules |
| Delegation cannot cross Tenant boundaries. | Authorization & Policy | Cross-Tenant requests are rejected. | L2-A04 §62 — Delegation Constraints |
| Delegation cannot permanently assign Roles or modify Role or Permission definitions. | Authorization & Policy | The existing authorization model remains unchanged. | L2-A04 §62 — Delegation Constraints |
| Delegation cannot bypass Authorization Policies. | Authorization & Policy | Delegated authority remains subject to normal policy evaluation. | L2-A04 §62 — Delegation Constraints |
| Delegation contributes additional authorization context only while valid and active. | Authorization & Policy | A Delegation does not itself guarantee authorization. | L2-A04 §§62 and 66 |
| Explicit Deny overrides Allow. | Authorization & Policy | A valid Delegation cannot override an Explicit Deny. | L2-A04 AP-06 |
| SoD evaluation includes delegated Roles. | Authorization & Policy | Conflicting delegated authority may prevent activation or later use. | L2-A04 §64 — SoD Evaluation |
| Every advanced authorization action is auditable. | Authorization & Policy; Audit Infrastructure | Delegation history must remain traceable. | L2-A04 AP-11 and §66; L2-A07 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Delegation | No Delegation exists | Create and progress through approval and activation. | `Requested → Approved → Active` | Authorization & Policy | L2-A04 §98.6 |
| Delegation | `Active` | Automatic expiry at the governed expiry condition. | `Expired` | Authorization & Policy | L2-A04 §§62 and 98.6 |
| Delegation | `Active` | Governed early revocation. | `Revoked` | Authorization & Policy | L2-A04 §§62 and 98.6 |
| Role Assignment, Role and Permission | Existing definitions and assignments | Referenced without modification. | Unchanged. | Authorization & Policy | L2-A04 §§10, 62 and 66 |
| Person and Identity | Existing eligible references | Referenced as delegator and delegate; no lifecycle transition. | Unchanged. | Person & Identity | L2-A02 INT-301; L2-A04 §89 |
| Authorization Context | No active A011 Delegation contribution | Includes the Delegation only while `Active`; excludes it after expiry or revocation. | Returns to the non-delegated context after terminal transition. | Authorization & Policy | L2-A04 §§17, 40, 53 and 62 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Locked L2-A04 lists auditable activity labels but does not define canonical Delegation event contracts or sole publishers.

| Required Business Fact | Upstream Evidence | Scenario Treatment |
|---|---|---|
| Delegation created | `Delegation Created` appears in L2-A04 §78. | Retained as an auditable activity label; not asserted as a canonical event. |
| Delegation activated | `Delegation Activated` appears in L2-A04 §78. | Retained as an auditable activity label; no publisher is invented. |
| Delegation revoked | `Delegation Revoked` appears in L2-A04 §78. | Retained as an auditable activity label; no publisher is invented. |
| Delegation expired | Required by the lifecycle and frozen outcome, but absent from the §78 advanced-event list. | No event name or publisher is invented. |

L2-A06 requires a registered canonical event contract and exactly one owning publisher for every business fact. Those definitions are missing for A011.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Platform & Tenant | Authorization & Policy | Tenant and institutional context | Establish the single-Tenant scope. | L2-A04 §88; L2-A01 P4 |
| Person & Identity | Authorization & Policy | Person, Identity and User Account references | Resolve delegator and delegate without transferring identity ownership. | L2-A02 INT-301; L2-A04 §§89 and 95 |
| Authentication | Authorization & Policy | Authenticated user and Session context | Supply trusted context for later protected requests. | L2-A03; L2-A04 §§90 and 95 |
| Delegation | Authorization evaluation | Active delegated scope and effective period | Contribute additional authorization context while valid. | L2-A04 §§17, 40, 53 and 62 |
| Authorization & Policy | Business domains | Authorization Decision | Govern a later protected business request. | L2-A04 §§19 and 95 |
| Authorization & Policy | Audit Infrastructure | Canonical delegation business event, once defined | Enable immutable evidence without transferring delegation ownership. | L2-A06; L2-A07 A07-W01 |

The command and approval contracts for creating, approving, activating, expiring and revoking a Delegation are not defined upstream.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate Delegation | Delegator | Must be an authorized user and remain within the policy-permitted delegated scope. | Authorization & Policy |
| Receive delegated responsibility | Delegate | May exercise only the approved scope and only while the Delegation is `Active`. | Authorization & Policy |

L2-A04 requires approval but does not name the approving, activation, expiry or revocation authority. Those authorities remain blocking gaps.

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| SoD evaluation | Delegator; Delegate; canonical approval actor undefined | Evaluate assigned Roles, delegated Roles, Temporary Access and the applicable workflow. Deny a prohibited conflict unless an approved policy explicitly permits continuation. |
| Explicit Deny | Authorization & Policy | Delegation cannot override Explicit Deny. |

No override actor is named upstream.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Delegation creation | `Delegation Created` auditable activity with the applicable delegator, delegate, Tenant, scope and period context. | Authorization & Policy |
| Approval and activation | Approval outcome and `Delegation Activated` auditable activity. | Authorization & Policy |
| Delegated authority use | Authorization evaluation and decision history identifying that Delegation was applied. | Authorization & Policy |
| Terminal reversion | Expiry or `Delegation Revoked` outcome showing delegated authority ceased. | Authorization & Policy |
| Immutable evidence | Audit evidence generated from canonical business events after their event contracts exist. | Audit Infrastructure |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Delegation remained within exactly one Tenant.
- The delegator, delegate, scope, effective date and expiry date were resolved.
- Applicable Delegation, least-privilege, Authorization and SoD policies were satisfied.
- Required approval was granted.
- The Delegation completed `Requested → Approved → Active`.
- Delegated responsibility contributed only during the permitted active period.
- The Delegation completed `Active → Expired` automatically or `Active → Revoked` through governed revocation.
- The delegated authority ceased to contribute to subsequent authorization evaluation.
- Role Assignments, Roles and Permission definitions remained unchanged.
- The delegation lifecycle and use remained auditable.

These criteria cannot yet be validated as a complete canonical outcome because the approval authority, command contracts, terminal event contract and publisher are undefined, and the A04/E02 delegation boundary conflicts with the frozen trigger.

## 16.2 Postconditions

- The Delegation is terminal in `Expired` or `Revoked`.
- The non-delegated authorization context applies again.
- The delegator and delegate Person/Identity lifecycles are unchanged.
- No Role Assignment, Role or Permission definition was permanently altered.
- Historical Delegation and Authorization Decision evidence remains available under audit governance.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A011-BLOCKER-01:** The frozen trigger describes a responsibility exercised during absence, leave, training or temporary assignment, and legacy A04 prose calls it an organizational responsibility. L2-E02 owns Assignment Delegation of Position responsibilities, while RC-01 restricts A04 to Authorization Delegation. | The source of the responsibility and the E02-to-A04 hand-off cannot be defined without changing or clarifying the frozen A-only boundary. | Reconcile the frozen semantics with RC-01 and define whether A011 begins only after an E02 organizational delegation fact or is strictly authorization-layer delegation. |
| **A011-BLOCKER-02:** L2-A04 requires Delegation approval but names no approving actor or approval authority. | Sections 6, 9 and 15 cannot allocate the material approval decision. | Define the canonical approval role and authority constraints. |
| **A011-BLOCKER-03:** L2-A04 defines no commands or business contracts for Delegation creation, approval, activation, expiry or revocation. | The exact governed hand-offs required for later L4 behaviour are missing. | Define the owning aggregate's canonical commands/contracts and transition responsibilities. |
| **A011-BLOCKER-04:** L2-A04 provides auditable labels for creation, activation and revocation, but no canonical event contracts or publishers; it also omits a Delegation expiry label. | A canonical terminal business fact cannot be published or consumed without invention. | Define exact Delegation event names, publication conditions and one publisher for each event, including expiry. |
| **A011-ISSUE-01:** L2-A04's state diagram defines `Requested → Approved → Active → Expired/Revoked`, but its lifecycle prose does not state transition authorities or unsuccessful approval outcomes. | The valid success path is known, while non-success lifecycle handling remains incomplete. | Add transition-authority and rejection/cancellation rules to L2-A04. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
