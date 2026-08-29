# Emergency Access Request to Audited Emergency Access Closure

# 1. Scenario Metadata

**Scenario ID:** L3-A-013  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authorization & Policy  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Emergency authorization governance, audit review and protected operations using Break-Glass context

---

# 2. Scenario Purpose

Govern exceptional Break-Glass access so an immediate institutional need may receive policy-approved, least-privilege and time-bound authorization, followed by automatic expiry or governed revocation and mandatory audit review without creating permanent Roles or Permissions.

---

# 3. Business Outcome

Policy-approved emergency access is temporarily provided, automatically expires or is revoked, and is retained for mandatory audit review.

Locked L2-A04 defines the Break-Glass workflow and mandatory controls but does not define canonical lifecycle states, actors, approval authority, commands, event publishers or the completion criteria and responsible actor for mandatory audit review. This scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an exceptional situation requires immediate controlled access beyond normal authorization.

## 4.2 End Boundary

The intended end boundary is reached when the temporary Break-Glass elevation has automatically expired or been revoked, no longer contributes to authorization evaluation, all mandatory audit evidence exists, and the required audit review has reached its governed closure.

The exact audit-review closure condition is not defined upstream.

## 4.3 In Scope

- Resolution of the single-Tenant, identity and emergency context.
- Business justification and emergency scope.
- Break-Glass Policy, least-privilege, SoD and Explicit Deny evaluation.
- Required emergency approval.
- Temporary elevation under Break-Glass authority.
- Use of active Break-Glass context for the emergency business operation.
- Administrator notification.
- Automatic expiry or manual revocation.
- Immutable audit evidence.
- Mandatory audit review and closure.

## 4.4 Out of Scope

- Normal Role Assignment or Permission assignment.
- Authorization Delegation.
- Non-emergency Temporary Access Grants.
- Permanent privilege elevation.
- Detailed execution of the emergency business operation owned by another scenario or domain.
- Authentication Session or Identity lifecycle management.
- Technical RBAC, IAM, APIs, database design, UI or infrastructure.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Upstream | L3-A-007 — Eligible Identity to Authenticated Session Availability | Authentication establishes trusted identity but does not grant emergency authority. |
| Related | L3-A-011 — Responsibility Delegation to Automatic Authority Reversion | A011 governs temporary transfer of an existing responsibility, not emergency elevation. |
| Related | L3-A-012 — Temporary Access Request to Governed Access Expiry | A012 governs approved non-emergency temporary access without the Break-Glass controls and mandatory review outcome. |
| Related control | L3-A-051 — Conflicting Responsibility Risk to Enforced Separation of Duties | Break-Glass remains subject to policy and SoD evaluation. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authorization & Policy / Section A | Primary owner. Owns Break-Glass Session, emergency authorization policy, temporary elevation, expiry, revocation and its contribution to Authorization Decisions. |
| Platform & Tenant / Section A | Supplies Tenant and institutional scope context. |
| Person & Identity / Section A | Supplies canonical Person, Identity and User Account references without owning emergency access. |
| Authentication / Section A | Supplies authenticated context; authentication does not grant Break-Glass authority. |
| Event Infrastructure / Section A | Governs registered event publication and distribution without owning emergency-access facts. |
| Audit Infrastructure / Section A | Owns immutable audit evidence and governed investigation access; the scenario-specific mandatory review contract is undefined. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

Locked L2-A04 does not name the business actor who:

- makes the Emergency Request;
- receives the Temporary Elevation;
- approves the request;
- performs manual revocation;
- receives the Administrator Notification; or
- performs and closes the Mandatory Audit Review.

No emergency requestor, approver, administrator or audit reviewer role is invented.

---

# 7. Preconditions and Entry Criteria

- The exceptional situation and immediate institutional need are established.
- The request belongs to exactly one resolved Tenant.
- The proposed emergency-access subject can be resolved through canonical A02 identity references.
- A valid authenticated context is available for the proposed subject.
- The emergency business justification, requested scope and protected operation are identifiable.
- Applicable Break-Glass, least-privilege, Authorization, SoD and Explicit Deny policies can be evaluated.
- The request does not require permanent changes to Role Assignments, Roles or Permissions.

---

# 8. Trigger

An exceptional situation requires immediate controlled access beyond normal authorization.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the Emergency Request with its business justification and required emergency scope. | Canonical requestor undefined; Authorization & Policy | A governed Break-Glass request begins. |
| 2 | Resolve the Tenant, identity, authenticated context and protected operation. | Platform & Tenant; Person & Identity; Authentication; Authorization & Policy | The emergency request is associated with valid execution context. |
| 3 | Evaluate the request under the applicable Break-Glass Policy, including emergency eligibility, duration, scope and approval requirements. | Authorization & Policy | Break-Glass policy eligibility is determined. |
| 4 | Evaluate least privilege, SoD and Explicit Deny constraints. | Authorization & Policy | A prohibited condition is detected or the request remains eligible. |
| 5 | Obtain the required policy-approved emergency approval. | Canonical approver undefined; Authorization & Policy | The temporary elevation is approved; the approval authority and contract are undefined upstream. |
| 6 | Establish the controlled Temporary Elevation and preserve the auditable `Break-Glass Activated` activity. | Authorization & Policy — Break-Glass Session | Emergency authorization becomes available for the approved scope and duration. |
| 7 | Produce the required Administrator Notification. | Authorization & Policy | The mandated notification occurs; its recipient role and canonical business contract are undefined. |
| 8 | Evaluate the emergency Business Operation using the Break-Glass context together with normal Authorization Policy, SoD and Explicit Deny. | Authorization & Policy | A governed Authorization Decision is produced for the emergency operation. |
| 9 | Automatically expire the Temporary Elevation at the governed condition or complete manual revocation earlier where required. | Authorization & Policy — Break-Glass Session | Emergency authority ceases and cannot become permanent. |
| 10 | Preserve the auditable `Break-Glass Expired` activity or governed revocation outcome and exclude the closed elevation from later authorization evaluation. | Authorization & Policy; Audit Infrastructure | Emergency-access cessation is traceable. |
| 11 | Assemble and perform the Mandatory Audit Review using immutable emergency-access and Authorization Decision evidence. | Canonical audit-review actor undefined; Authorization & Policy; Audit Infrastructure | The required review occurs, but its governed decision and closure criteria are undefined upstream. |
| 12 | Close the emergency-access outcome after mandatory review completion. | Canonical closure authority undefined; Authorization & Policy | The intended frozen outcome is reached only after a review closure contract exists. |

Steps 5–12 cannot be finalized as an exact canonical outcome because actors, lifecycle states, commands, event publishers, notification contract and audit-review closure are undefined.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Stricter institution policy | Institution configuration imposes stricter controls than the platform default. | Apply the stricter duration, approval or eligibility requirements without weakening mandatory platform invariants. | Emergency access remains policy-approved, time-bound and auditable. | Authorization & Policy |
| Governed manual revocation | Emergency authority must cease before automatic expiry. | Revoke the Temporary Elevation, remove it from authorization context and preserve the revocation outcome for review. | Emergency access closes early but still requires Mandatory Audit Review. | Authorization & Policy |

Normal Temporary Access is not an alternate A013 path.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant, identity or authenticated context cannot be resolved. | Emergency authority cannot be associated with a valid subject and Tenant. | Reject the request without creating Temporary Elevation. | Platform & Tenant; Person & Identity; Authentication; Authorization & Policy |
| Business justification is absent or the situation is not eligible as an emergency. | Break-Glass cannot be used as a substitute for normal authorization. | Deny Break-Glass progression; use the applicable normal access scenario where eligible. | Authorization & Policy |
| Requested scope exceeds least privilege. | The proposed elevation violates a mandatory control. | Restrict or reject the request before approval. | Authorization & Policy |
| Policy, SoD or Explicit Deny prohibits access. | Emergency elevation cannot validly proceed. | Deny the request unless an approved policy explicitly permits the applicable continuation; no override actor is invented. | Authorization & Policy |
| Required approval is not granted. | Temporary Elevation cannot begin. | Do not activate Break-Glass access. | Authorization & Policy |
| Temporary Elevation cannot be established. | No emergency authority is available. | Do not treat the subject as elevated; the exact business recovery workflow is undefined upstream. | Authorization & Policy |
| Administrator Notification cannot be associated with a canonical recipient or contract. | A mandatory default control cannot be verified. | Do not invent a recipient; obtain an upstream architecture decision. | Authorization & Policy |
| Automatic expiry or revocation lacks a canonical event. | Auditable closure and other eligible consumers lack a governed terminal fact. | Do not invent an event or publisher; obtain an upstream L2-A04 event decision. | Authorization & Policy |
| Mandatory Audit Review cannot be completed or closed. | The frozen audited-closure outcome remains incomplete. | Preserve immutable evidence and keep the scenario unresolved until review authority and closure criteria are defined. | Authorization & Policy; Audit Infrastructure |

No technical retry, rollback or implementation recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Break-Glass Session and emergency access are owned exclusively by L2-A04. | Authorization & Policy | No other domain may create or govern the emergency authorization lifecycle. | L2-A04 §§3, 9 and 10.8 |
| Break-Glass is reserved for exceptional situations requiring immediate action to protect institutional operations. | Authorization & Policy | It cannot substitute for normal authorization procedures. | L2-A04 §65 |
| Break-Glass is policy-controlled, approval-required, time-bound and automatically expires. | Authorization & Policy | Emergency access cannot become indefinite or self-approved without an upstream rule. | L2-A04 §§10.8 and 65 |
| Platform default duration is 45 minutes. | Authorization & Policy | The default applies unless an institution configures a stricter policy. | L2-A04 §§10.8 and 65 |
| Institutions may configure stricter policies but cannot weaken mandatory platform security invariants. | Authorization & Policy | Institution configuration can narrow but not relax the mandatory controls. | L2-A04 §65 — Platform Default Behaviour |
| Break-Glass follows least privilege and requires business justification. | Authorization & Policy | Scope must be limited to the emergency need. | L2-A04 §65 — Break-Glass Constraints |
| Break-Glass generates immutable audit records and administrative notifications. | Authorization & Policy; Audit Infrastructure | Evidence and notification are mandatory components of the outcome. | L2-A04 §§10.8 and 65 |
| Manual revocation is supported. | Authorization & Policy | Emergency access may cease before automatic expiry. | L2-A04 §65 — Platform Default Behaviour |
| Break-Glass never permanently assigns Permissions or Roles. | Authorization & Policy | Permanent authorization definitions remain unchanged. | L2-A04 §§10.8, 65 and 66 |
| Advanced authorization never replaces standard authorization and remains subject to policy. | Authorization & Policy | Break-Glass context participates in, but does not bypass, centralized evaluation. | L2-A04 §66 |
| Explicit Deny overrides Allow. | Authorization & Policy | Emergency context cannot silently bypass Explicit Deny. | L2-A04 AP-06 |
| Audit evidence is generated from canonical business events and remains immutable. | Audit Infrastructure | Emergency-access evidence requires canonical A04 events and cannot be rewritten during review. | L2-A07 §§7, 19.1 and 21 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Break-Glass Session | No emergency elevation exists | Emergency Request, Policy Evaluation, Approval and Temporary Elevation occur. | Temporary elevation becomes available; no canonical state name is defined. | Authorization & Policy | L2-A04 §§65 and 98.8 |
| Break-Glass Session | Temporary Elevation available | Automatic expiry or governed manual revocation removes emergency authority. | Elevation no longer contributes to authorization; no terminal state name is defined. | Authorization & Policy | L2-A04 §§10.8 and 65 |
| Role Assignment, Role and Permission definitions | Existing authorization model | Referenced without permanent modification. | Unchanged. | Authorization & Policy | L2-A04 §§10.8, 65 and 66 |
| Person and Identity | Existing canonical references | Referenced for the emergency-access subject; no lifecycle transition. | Unchanged. | Person & Identity | L2-A02 INT-301; L2-A04 §89 |
| Authorization Context | No Break-Glass contribution | Includes Temporary Elevation only during the approved period; excludes it after expiry or revocation. | Returns to normal authorization context. | Authorization & Policy | L2-A04 §§17, 40, 53 and 65 |
| Audit Evidence | Canonical emergency event received | Generate immutable evidence and make it available for governed review. | Created once and immutable. | Audit Infrastructure | L2-A07 A07-W01 and §§21–22 |

L2-A04 provides a workflow sequence for Break-Glass but no lifecycle state model or transition matrix. This Draft therefore does not invent `Requested`, `Approved`, `Active`, `Expired`, `Revoked`, `Reviewed` or `Closed` as Break-Glass Session states.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Locked L2-A04 lists auditable activity labels but does not define canonical Break-Glass event contracts or sole publishers.

| Required Business Fact | Upstream Evidence | Scenario Treatment |
|---|---|---|
| Emergency request received | Workflow stage exists; no event label is defined. | No event is invented. |
| Emergency access approved | Workflow stage exists; no event label is defined. | No event is invented. |
| Break-Glass activated | `Break-Glass Activated` appears in L2-A04 §78. | Retained as an auditable activity label; no publisher is invented. |
| Break-Glass expired | `Break-Glass Expired` appears in L2-A04 §78. | Retained as an auditable activity label; no publisher is invented. |
| Break-Glass revoked | Manual revocation is supported, but no event label is defined. | No event is invented. |
| Mandatory audit review completed | Required workflow stage exists; no event label is defined. | No event is invented. |
| Emergency access closed | Required by the frozen outcome; no event or state is defined. | No event is invented. |

L2-A06 requires a registered canonical event contract and exactly one owning publisher for every business fact. Those definitions are missing for A013.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Platform & Tenant | Authorization & Policy | Tenant and institutional context | Establish the single-Tenant emergency scope. | L2-A04 §88; L2-A01 P4 |
| Person & Identity | Authorization & Policy | Person, Identity and User Account references | Resolve the emergency-access subject without transferring identity ownership. | L2-A02 INT-301; L2-A04 §§89 and 95 |
| Authentication | Authorization & Policy | Authenticated User and Session context | Supply trusted context for emergency authorization. | L2-A03; L2-A04 §§90 and 95 |
| Break-Glass Session | Authorization evaluation | Approved Temporary Elevation and scope | Contribute emergency authorization context during the permitted period. | L2-A04 §§53 and 65 |
| Authorization & Policy | Owning business domain | Authorization Decision | Govern the emergency Business Operation. | L2-A04 §§19 and 95 |
| Authorization & Policy | Notification recipient undefined | Administrator Notification | Satisfy a mandatory Break-Glass control. | L2-A04 §§10.8 and 65 |
| Authorization & Policy | Audit Infrastructure | Canonical Break-Glass business event, once defined | Generate immutable emergency-access evidence. | L2-A06; L2-A07 A07-W01 |
| Audit Infrastructure | Mandatory review authority undefined | Immutable emergency-access evidence and authorized historical view | Support the mandatory audit review without modifying evidence. | L2-A07 §§19.8–19.9 |

The command, approval, notification and mandatory-review closure contracts are not defined upstream.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

L2-A04 requires an Emergency Request, policy evaluation and approval but does not identify the requestor, emergency-access subject, approver, revocation authority, notification recipient, audit reviewer or closure authority. No authority table can be completed without inventing actors.

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Least privilege and SoD | Canonical requestor and approver undefined | Restrict elevation to the emergency need and evaluate applicable conflicting responsibilities. |
| Explicit Deny | Authorization & Policy | Break-Glass does not silently displace Explicit Deny; any permitted continuation must be explicitly supported by policy. |
| Stricter institution policy | Institution policy authority not named | Institution rules may strengthen but cannot weaken mandatory controls. |

No override actor or separate exception authority is named upstream.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Emergency request and justification | Emergency situation, requested scope and business justification. | Authorization & Policy |
| Policy evaluation and approval | Applicable Break-Glass Policy and approval outcome. | Authorization & Policy |
| Temporary elevation | `Break-Glass Activated` auditable activity and effective period. | Authorization & Policy |
| Emergency use | Authorization evaluation and decision history for the protected operation. | Authorization & Policy |
| Notification | Administrator Notification required by the platform default. | Authorization & Policy |
| Access cessation | `Break-Glass Expired` or governed revocation outcome. | Authorization & Policy |
| Immutable evidence | Audit Evidence with actor, authorization, Tenant, correlation and causation context. | Audit Infrastructure |
| Mandatory review and closure | Review scope, review outcome and closure evidence; exact definitions are missing. | Authorization & Policy; Audit Infrastructure |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The emergency request remained within exactly one Tenant.
- The exceptional situation, identity, authenticated context, business justification and emergency scope were resolved.
- Applicable Break-Glass, least-privilege, Authorization, SoD and Explicit Deny policies were satisfied.
- Required approval was granted.
- Controlled Temporary Elevation became available for the approved scope and duration.
- The emergency Business Operation received a governed Authorization Decision.
- Administrator Notification occurred.
- Temporary Elevation automatically expired or was revoked.
- Emergency authority ceased to contribute to later authorization evaluation.
- Permanent Role Assignments, Roles and Permission definitions remained unchanged.
- Immutable emergency-access evidence was retained.
- Mandatory Audit Review was completed and the emergency-access outcome reached governed closure.

These criteria cannot be validated as a complete canonical outcome because actors, lifecycle states, command contracts, event publishers, notification recipient and mandatory-review closure semantics are undefined.

## 16.2 Postconditions

- No emergency Temporary Elevation remains available.
- Normal authorization context applies again.
- Person, Identity, Role Assignment, Role and Permission-definition lifecycles are unchanged.
- Immutable emergency-access and Authorization Decision evidence remains available for authorized governance and investigation.
- The intended Mandatory Audit Review outcome remains unverifiable until its canonical authority and closure contract are defined.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A013-BLOCKER-01:** L2-A04 defines a Break-Glass workflow but no canonical Break-Glass Session states or transition matrix. | Section 13 cannot identify activation, expiry, revocation, review or closure states required to prove the final outcome. | Define the canonical lifecycle states, transitions and terminal positions. |
| **A013-BLOCKER-02:** L2-A04 names no emergency requestor, access subject, approver, revocation authority, notification recipient, audit reviewer or closure authority. | Material initiation, approval, notification, review and closure responsibilities cannot be assigned. | Define the canonical actors and authority boundaries. |
| **A013-BLOCKER-03:** L2-A04 defines no commands or business contracts for request, approval, activation, expiry, revocation, notification, mandatory review or closure. | The governed hand-offs required for later L4 behaviour are missing. | Define Break-Glass and review commands/contracts with ownership. |
| **A013-BLOCKER-04:** `Break-Glass Activated` and `Break-Glass Expired` are auditable labels, not canonical event contracts with publishers; request, approval, revocation, review and closure events are absent. | Emergency access cannot produce governed lifecycle facts or canonical audit inputs without invention. | Define exact event names, publication conditions and one canonical publisher for each required event. |
| **A013-BLOCKER-05:** Mandatory Audit Review is a required final workflow stage, but L2-A04 does not define its review scope, decision outcome, completion criteria, responsible actor or closure fact; L2-A07 defines evidence investigation capabilities but not this review decision. | The frozen outcome `Audited Emergency Access Closure` cannot be proven or closed. | Define the mandatory review workflow, authority, decision, evidence requirements and closure condition, preserving A04/A07 ownership boundaries. |
| **A013-ISSUE-01:** Administrator Notification is mandatory, but the canonical recipient and business contract are not defined. | Notification compliance cannot be verified. | Define the recipient role, notification fact and ownership without transferring Break-Glass ownership. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
