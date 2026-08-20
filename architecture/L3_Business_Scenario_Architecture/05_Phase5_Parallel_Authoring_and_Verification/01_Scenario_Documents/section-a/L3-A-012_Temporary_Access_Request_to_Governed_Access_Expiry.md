# Temporary Access Request to Governed Access Expiry

# 1. Scenario Metadata

**Scenario ID:** L3-A-012  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authorization & Policy  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorization evaluations that may include an active Temporary Access Grant

---

# 2. Scenario Purpose

Govern exceptional, least-privilege and time-bound authorization beyond normal assigned access so approved Permissions become available only for a defined scope and duration and then cease automatically without modifying permanent Role Assignments.

---

# 3. Business Outcome

Approved least-privilege access becomes available for the permitted scope and duration and expires automatically without permanently altering role assignments.

Locked L2-A04 defines the Temporary Access lifecycle and core rules but does not define the canonical approval authority, commands, event names or publishers required to complete this outcome exactly. This scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an exceptional operational requirement needs temporary authorization beyond normal assigned access.

## 4.2 End Boundary

The intended end boundary is reached when the Temporary Access Grant has automatically reached `Expired`, no longer contributes granted Permissions to authorization evaluation, permanent Role Assignments remain unchanged, and the grant history is auditable.

## 4.3 In Scope

- Resolution of the Tenant, requestor identity and proposed access subject.
- Business justification, granted Permissions, scope, effective date and expiry date.
- Least-privilege, Authorization Policy and SoD evaluation.
- Required approval and approval reference.
- Creation, approval and grant of Temporary Access.
- Participation of a `Granted` Temporary Access Grant in authorization evaluation.
- Automatic expiry.
- Governed revocation before expiry where required.
- Audit evidence for approval, grant, use, expiry and revocation.

## 4.4 Out of Scope

- Permanent Role Assignment or changes to Role definitions.
- Authorization Delegation of an existing responsibility.
- Break-Glass emergency access.
- Permanent Permission assignment to a Person or User Account.
- Execution of the operational activity for which temporary access was requested.
- Authentication Session or Identity lifecycle management.
- Technical RBAC, IAM, APIs, database design, UI or infrastructure.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-A-010 — Institutional Responsibility Assignment to Effective Authorized Access | A012 supplements normal assigned access without changing the Role Assignment. |
| Related | L3-A-011 — Responsibility Delegation to Automatic Authority Reversion | A011 transfers an existing responsibility rather than granting exceptional Permissions. |
| Related | L3-A-013 — Emergency Access Request to Audited Emergency Access Closure | A013 governs emergency Break-Glass elevation with mandatory audit review. |
| Related control | L3-A-051 — Conflicting Responsibility Risk to Enforced Separation of Duties | Temporary Access participates in SoD evaluation. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authorization & Policy / Section A | Primary owner. Owns Temporary Access Grant, its lifecycle, Permissions, policy evaluation, SoD evaluation and later Authorization Decisions. |
| Platform & Tenant / Section A | Supplies Tenant and institutional scope context. |
| Person & Identity / Section A | Supplies canonical Person, Identity and User Account references without owning temporary authorization. |
| Authentication / Section A | Supplies authenticated context for protected requests; authentication does not grant access. |
| Event Infrastructure / Section A | Governs registered event publication and distribution without owning Temporary Access facts. |
| Audit Infrastructure / Section A | Owns immutable audit evidence generated from canonical business events. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Requestor | Supplies the exceptional operational requirement and business justification for temporary access. | Initiates the request; does not gain access without policy evaluation and approval. |
| Approver | Determines whether the requested temporary access is approved. | The label is explicitly required by L2-A04, but the canonical role, eligibility and authority boundaries are not defined. |

L2-A04 requires each Temporary Access Grant to identify a Requestor and Approver but does not define the access subject separately, the approver's canonical institutional role, or the activation and revocation authorities.

---

# 7. Preconditions and Entry Criteria

- The request belongs to exactly one resolved Tenant.
- The Requestor and proposed access subject can be resolved through canonical A02 identity references.
- The exceptional operational requirement and business justification are available.
- The requested Permissions, access scope, effective date and expiry date are defined.
- Applicable least-privilege, Temporary Access, Authorization and SoD policies can be evaluated.
- The request does not require permanent modification of any Role Assignment, Role or Permission definition.

---

# 8. Trigger

An exceptional operational requirement needs temporary authorization beyond normal assigned access.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the exceptional temporary-access requirement and business justification. | Requestor; Authorization & Policy | A governed Temporary Access request begins. |
| 2 | Resolve the Tenant and canonical identity references for the Requestor and proposed access subject. | Platform & Tenant; Person & Identity; Authorization & Policy | The request is associated with valid Tenant and identity context. |
| 3 | Define the requested Permissions, scope, effective date and expiry date. | Requestor; Authorization & Policy — Temporary Access Grant | The proposed least-privilege access boundary is established. |
| 4 | Evaluate the request against Temporary Access and Authorization Policies, including duration, scope and automatic-expiry requirements. | Authorization & Policy | Policy eligibility is determined. |
| 5 | Evaluate least privilege and applicable SoD Policies using assigned, delegated and temporary responsibilities. | Authorization & Policy | A prohibited conflict is detected or the request remains eligible. |
| 6 | Create the Temporary Access Grant in `Requested`. | Authorization & Policy — Temporary Access Grant | The canonical lifecycle begins. |
| 7 | Obtain the required approval and complete `Requested → Approved`, preserving the Approver and Approval Reference. | Approver; Authorization & Policy | The request is approved; the approver's canonical authority remains undefined upstream. |
| 8 | Complete `Approved → Granted` on the permitted effective date and preserve the auditable `Temporary Access Granted` activity. | Authorization & Policy — Temporary Access Grant | Approved Permissions become available as additional authorization context. |
| 9 | Include the `Granted` Temporary Access Grant in subsequent authorization evaluation without bypassing policy, SoD or Explicit Deny. | Authorization & Policy | Temporary Permissions may contribute to a later Authorization Decision. |
| 10 | At the governed expiry condition, complete `Granted → Expired`, preserve the auditable `Temporary Access Expired` activity and exclude the Grant from later evaluation. | Authorization & Policy — Temporary Access Grant | Temporary access ceases automatically. |
| 11 | Preserve immutable approval, grant, use and expiry history. | Authorization & Policy; Audit Infrastructure | The temporary-access outcome remains traceable. |

Steps 7, 8, 10 and 11 cannot be finalized as exact canonical collaboration because the approval authority, commands, event contracts and event publishers are undefined.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Future-effective Grant | Approval occurs before the effective date. | Retain the Grant in `Approved` until its effective date and then complete `Approved → Granted`. | Permissions do not participate before the governed start. | Authorization & Policy |
| Governed early revocation | A `Granted` Temporary Access Grant must cease before expiry. | Complete `Granted → Revoked` and exclude it from subsequent authorization evaluation. | Temporary access ends without changing Role Assignments. | Authorization & Policy |

A manual extension is not an alternate path. L2-A04 requires a new approval process.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant or identity context cannot be resolved. | The request cannot preserve Tenant or subject integrity. | Reject the request without producing granted access. | Platform & Tenant; Person & Identity; Authorization & Policy |
| Business justification is absent. | A required Grant fact is missing. | Stop progression until a valid justification is supplied. | Requestor; Authorization & Policy |
| Requested Permissions, scope or duration are invalid. | Least-privilege temporary access cannot be established. | Correct the request before approval; no Grant becomes effective. | Authorization & Policy |
| Policy denies the request. | The exceptional requirement is not eligible for Temporary Access. | Deny progression without modifying normal assigned access. | Authorization & Policy |
| SoD conflict is prohibited. | The Grant would create conflicting responsibilities. | Deny the request unless an approved policy explicitly permits continuation. | Authorization & Policy |
| Approval is not granted. | `Requested → Approved` cannot complete. | Keep the request outside `Granted`; no temporary Permissions become available. | Approver; Authorization & Policy |
| Grant activation does not complete. | Approved Permissions remain unavailable. | Do not treat the Grant as `Granted`. The exact recovery workflow is undefined upstream. | Authorization & Policy |
| Expiry cannot be represented by a canonical event. | Evidence and other eligible consumers lack a governed terminal fact. | Do not invent an event or publisher; obtain an upstream L2-A04 event decision. | Authorization & Policy |

No technical retry, rollback or implementation recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Temporary Access Grant is owned exclusively by L2-A04. | Authorization & Policy | No other domain may create or govern its authorization lifecycle. | L2-A04 §§3, 9 and 10.6 |
| Every Grant includes Requestor, Approver, business justification, effective date, expiry date, granted Permissions, Approval Reference and Audit Reference. | Authorization & Policy | These facts are required for the governed Grant outcome. | L2-A04 §63 — Temporary Access Rules |
| Temporary Access follows least privilege, is time-bound, policy-controlled and fully auditable. | Authorization & Policy | Access must be limited to the permitted need and period. | L2-A04 §§10.6, 63 and 66 |
| Temporary Access always expires automatically. | Authorization & Policy | `Granted` access cannot remain indefinite. | L2-A04 §§63 and 66 |
| Manual extension requires a new approval process. | Authorization & Policy | The existing Grant cannot be silently prolonged. | L2-A04 §63 — Temporary Access Rules |
| Temporary Access never permanently alters Role Assignments. | Authorization & Policy | The access expires or is revoked without changing normal assigned access. | L2-A04 §63 — Temporary Access Constraints |
| A granted Permission does not automatically authorize execution. | Authorization & Policy | Later policy, resource, scope, SoD and Explicit Deny evaluation still applies. | L2-A04 §§40, 41 and 53 |
| Explicit Deny overrides Allow. | Authorization & Policy | Temporary Access cannot override an Explicit Deny. | L2-A04 AP-06 |
| SoD evaluation includes Temporary Access. | Authorization & Policy | Conflicting temporary Permissions may prevent approval, grant or use. | L2-A04 §64 |
| Every advanced authorization action is auditable. | Authorization & Policy; Audit Infrastructure | Approval, grant, use and expiry history must remain traceable. | L2-A04 AP-11 and §66; L2-A07 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Temporary Access Grant | No Grant exists | Create and progress through approval and grant. | `Requested → Approved → Granted` | Authorization & Policy | L2-A04 §§63 and 98.7 |
| Temporary Access Grant | `Granted` | Automatic expiry at the governed expiry condition. | `Expired` | Authorization & Policy | L2-A04 §§63 and 98.7 |
| Temporary Access Grant | `Granted` | Governed early revocation. | `Revoked` | Authorization & Policy | L2-A04 §98.7 |
| Role Assignment, Role and Permission definitions | Existing authorization model | Referenced without permanent modification. | Unchanged. | Authorization & Policy | L2-A04 §§10.6 and 63 |
| Person and Identity | Existing canonical references | Referenced for the Requestor and access subject; no lifecycle transition. | Unchanged. | Person & Identity | L2-A02 INT-301; L2-A04 §89 |
| Authorization Context | No A012 Grant contribution | Includes granted Permissions only while the Grant is `Granted`; excludes them after expiry or revocation. | Returns to the normal assigned authorization context. | Authorization & Policy | L2-A04 §§17, 40, 53 and 63 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Locked L2-A04 lists auditable activity labels but does not define canonical Temporary Access event contracts or sole publishers.

| Required Business Fact | Upstream Evidence | Scenario Treatment |
|---|---|---|
| Temporary access requested | `Requested` is a lifecycle state; no event label is defined. | No event is invented. |
| Temporary access approved | `Approved` is a lifecycle state; no event label is defined. | No event is invented. |
| Temporary access granted | `Temporary Access Granted` appears in L2-A04 §78. | Retained as an auditable activity label; no publisher is invented. |
| Temporary access expired | `Temporary Access Expired` appears in L2-A04 §78. | Retained as an auditable activity label; no publisher is invented. |
| Temporary access revoked | `Revoked` exists in the state diagram, but no §78 activity label is defined. | No event is invented. |

L2-A06 requires a registered canonical event contract and exactly one owning publisher for every business fact. Those definitions are missing for A012.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Platform & Tenant | Authorization & Policy | Tenant and institutional context | Establish the single-Tenant scope. | L2-A04 §88; L2-A01 P4 |
| Person & Identity | Authorization & Policy | Person, Identity and User Account references | Resolve the Requestor and access subject without transferring identity ownership. | L2-A02 INT-301; L2-A04 §§89 and 95 |
| Authentication | Authorization & Policy | Authenticated user and Session context | Supply trusted context for later protected requests. | L2-A03; L2-A04 §§90 and 95 |
| Temporary Access Grant | Authorization evaluation | Granted Permissions, scope and effective period | Contribute additional authorization context while valid. | L2-A04 §§40, 53 and 63 |
| Authorization & Policy | Business domains | Authorization Decision | Govern a later protected business request. | L2-A04 §§19 and 95 |
| Authorization & Policy | Audit Infrastructure | Canonical Temporary Access business event, once defined | Enable immutable evidence without transferring Grant ownership. | L2-A06; L2-A07 A07-W01 |

The command and approval contracts for requesting, approving, granting, expiring and revoking Temporary Access are not defined upstream.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Request Temporary Access | Requestor | Must provide the exceptional requirement and business justification. | Authorization & Policy |
| Approve Temporary Access | Approver | Must approve the defined Permissions, scope and period; canonical role and eligibility are undefined. | Authorization & Policy |

L2-A04 does not define whether the Requestor and access subject may differ or who has grant, expiry and revocation authority.

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| SoD evaluation | Requestor; Approver; access subject not separately defined | Evaluate assigned Roles, delegated Roles, Temporary Access and the applicable workflow. Deny a prohibited conflict unless an approved policy explicitly permits continuation. |
| Explicit Deny | Authorization & Policy | Temporary Access cannot override Explicit Deny. |
| Extension | Requestor; Approver | A manual extension requires a new approval process. |

No override actor is named upstream.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Request and approval | Requestor, Approver, business justification, scope, period, granted Permissions, Approval Reference and Audit Reference. | Authorization & Policy |
| Grant effectiveness | `Temporary Access Granted` auditable activity and the governed active period. | Authorization & Policy |
| Temporary access use | Authorization evaluation and decision history identifying that Temporary Access was applied. | Authorization & Policy |
| Access cessation | `Temporary Access Expired` or governed revocation outcome. | Authorization & Policy |
| Immutable evidence | Audit evidence generated from canonical business events after their event contracts exist. | Audit Infrastructure |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The request remained within exactly one Tenant.
- The Requestor, access subject, business justification, Permissions, scope and period were resolved.
- Applicable least-privilege, Temporary Access, Authorization and SoD policies were satisfied.
- Required approval was granted and referenced.
- The Grant completed `Requested → Approved → Granted`.
- Granted Permissions contributed only within the approved scope and period.
- The Grant completed `Granted → Expired` automatically.
- Expired access no longer contributed to subsequent authorization evaluation.
- Permanent Role Assignments and Role and Permission definitions remained unchanged.
- Approval, grant, use and expiry remained auditable.

These criteria cannot yet be validated as a complete canonical outcome because the approval authority, command contracts and event publishers are undefined.

## 16.2 Postconditions

- The Temporary Access Grant is `Expired`.
- Normal assigned authorization context applies without the expired Grant.
- Person, Identity, Role Assignment, Role and Permission-definition lifecycles are unchanged.
- Historical approval, use and expiry evidence remains available under audit governance.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A012-BLOCKER-01:** L2-A04 requires an Approver but does not define the canonical approving role, eligibility, authority limits or whether the Requestor and access subject may differ. | The material approval decision and subject relationship cannot be assigned exactly in Sections 6, 9 and 15. | Define the canonical actors and approval-authority rules. |
| **A012-BLOCKER-02:** L2-A04 defines no commands or business contracts for request, approval, grant, expiry or revocation. | The exact governed hand-offs required for later L4 behaviour are missing. | Define the Temporary Access Grant commands/contracts and transition responsibilities. |
| **A012-BLOCKER-03:** `Temporary Access Granted` and `Temporary Access Expired` are auditable labels, not canonical event contracts with publishers; request, approval and revocation events are absent. | The Grant lifecycle cannot publish governed business facts without invention. | Define exact event names, publication conditions and one canonical publisher for each required event. |
| **A012-ISSUE-01:** The state diagram includes `Granted → Revoked`, but §78 omits a revocation audit label and §63 does not define revocation rules. | Early revocation can be recognized as a lifecycle path, but its authority and evidence are incomplete. | Define revocation authority, rules and audit/event treatment. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
