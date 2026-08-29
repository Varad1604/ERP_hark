# Institutional Responsibility Assignment to Effective Authorized Access

# 1. Scenario Metadata

**Scenario ID:** L3-A-010  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authorization & Policy  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.2  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture and its locked 2026-08-26 Person-scope amendment; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows and its locked 2026-08-26 Role Assignment amendment; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L2-E02 — Position & Assignment Workflows and its locked 2026-08-26 authorization hand-off amendment  
**Architecture Decision Trace:** `ADR-PLAT-AUTH-001`; `PLATFORM_AUTHORIZATION_UPSTREAM_APPROVAL_RECORD_2026-08-26.md`  
**Referenced By:** Business domains requiring centralized authorization decisions

---

# 2. Scenario Purpose

Establish an approved, scoped and effective-dated A04 Role Assignment for an eligible Person so the Role and its grouped Permissions participate in centralized policy-based authorization decisions without modifying the Role definition, assigning Permissions directly to the Person, or creating organizational authority.

---

# 3. Business Outcome

An approved A04 Role Assignment for the canonical A02 Person becomes `Active` within its governed scope and effective period. The assigned Role and its grouped Permissions then participate in later policy-based authorization decisions without permanently altering the Role definition or transferring ownership of identity or organizational-authority facts.

The previously unresolved Role Assignment subject, lifecycle, business responsibilities, contracts, events, publisher and E02/A04 boundary are resolved by the locked upstream amendments approved through `ADR-PLAT-AUTH-001`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a governed authorization-layer Role Assignment requirement for an eligible A02 Person reaches the A04 Authorization & Policy boundary with sufficient source business basis or evidence to begin evaluation.

The upstream business process that establishes the organizational or business responsibility is outside this scenario boundary.

## 4.2 End Boundary

The scenario ends when:

- the canonical A02 Person has a governed A04 Role Assignment;
- all required validation, least-privilege, SoD and approval obligations have been satisfied;
- the Role Assignment is `Active` within its approved Tenant and institutional scope and effective period;
- the Role's grouped Permissions are available to later Role Resolution and Permission Resolution;
- the Role and Permission definitions remain unchanged;
- no organizational authority has been created by the A04 Role Assignment; and
- the request, decision and activation history are auditable.

## 4.3 In Scope

- Tenant and institutional-scope validation.
- Resolution of the canonical A02 Person and supporting Identity/User Account context where relevant.
- Receipt and validation of the source business basis or eligibility evidence.
- Selection of an existing A04 Role representing the required authorization-layer responsibility.
- Validation that the Role has an approved grouping of Permissions.
- Definition of assignment scope and effective period.
- Least-privilege evaluation.
- Role Assignment constraint evaluation.
- Separation of Duties evaluation.
- Determination and completion of applicable Role Assignment approval.
- Governed Role Assignment lifecycle progression from request through approval to activation.
- Availability of the active Role Assignment for later authorization decisions.
- Publication of canonical A04 Role Assignment events.
- Immutable audit evidence for request, decision and activation.

## 4.4 Out of Scope

- Creating, modifying, activating, suspending or archiving the Role definition.
- Adding or removing Permissions from the Role.
- Assigning Permissions directly to a Person or User Account.
- Creating or changing A02 Person, Identity or User Account identity truth.
- Establishing E02 Position Assignment, Responsibility Assignment or organizational authority.
- Deriving organizational authority from an A04 Role or Role Assignment.
- Authorization Delegation.
- Assignment Delegation owned by L2-E02.
- Temporary Access Grants.
- Break-Glass or emergency access.
- Direct execution of a protected business operation.
- Authentication or Authentication Session creation.
- Workspace definition, navigation or presentation behaviour.
- Technical RBAC, IAM groups, claims, ACLs, APIs, database structures, UI design or infrastructure.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Upstream | L3-A-007 — Eligible Identity to Authenticated Session Availability | Authentication establishes authenticated context but does not create the Role Assignment or grant authorization. |
| Related | L3-A-011 — Responsibility Delegation to Automatic Authority Reversion | Owns temporary Authorization Delegation and does not permanently change the Role Assignment. |
| Related | L3-A-012 — Temporary Access Request to Governed Access Expiry | Owns exceptional time-bound access beyond normal Role Assignments. |
| Related | L3-A-013 — Emergency Access Request to Audited Emergency Access Closure | Owns controlled Break-Glass access rather than normal responsibility assignment. |
| Related control | L3-A-051 — Conflicting Responsibility Risk to Enforced Separation of Duties | Governs the broader outcome when conflicting responsibilities require enforced SoD treatment. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authorization & Policy / Section A | Primary owner. Owns Role, Role Assignment, Permission, Authorization Policy, SoD Policy, Role Assignment lifecycle, approval governance and canonical Role Assignment events. |
| Platform & Tenant / Section A | Supplies Tenant, institution, organization and campus context used to validate assignment scope. |
| Person & Identity / Section A | Supplies the canonical Tenant-scoped Person and supporting Identity/User Account references without owning Role Assignment or Permissions. |
| Authentication / Section A | Supplies authenticated context when an initiating actor or the assigned Person later performs protected activity; authentication does not create or activate the Role Assignment. |
| Event Infrastructure / Section A | Owns governed event distribution infrastructure without owning Role Assignment event semantics or facts. |
| Audit Infrastructure / Section A | Owns immutable audit evidence, integrity and retention without owning the Role Assignment lifecycle. |

The frozen Participating Domains value remains exactly `A`.

L2-E02 may establish source organizational responsibility and may invoke the A04 `Request Role Assignment` business contract, but that upstream establishment is outside the A010 start boundary. This hand-off therefore does not add Section E as a participating domain to this scenario.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Assignment Subject | The canonical A02 Person receiving the authorization-layer Role Assignment. | No request, approval or activation authority is implied merely by being the subject. |
| Assignment Requestor | Initiates the governed Role Assignment request and supplies the authoritative business basis or source evidence. | Cannot approve merely because it initiated the request. |
| Assignment Authority | Governs assignment scope, authorization-policy compliance, least privilege and determination of the required approval path. | Does not originate source-domain eligibility facts. |
| Assignment Approver | Grants or denies the Role Assignment where policy requires approval. | Must remain independently governable from the requestor where SoD requires separation. |
| Assignment Administrator | Records/administers the approved assignment and makes it effective according to approved scope and effective period. | Cannot bypass approval, SoD or policy constraints. |
| Source Eligibility Authority | Establishes the source business fact or evidence making the Person eligible for the responsibility. | Its eligibility decision remains owned by the originating business process; A04 consumes the evidence without redefining it. |

Institutions may map these canonical responsibilities to local organizational roles in accordance with policy. Such mapping does not change the canonical responsibility boundaries.

---

# 7. Preconditions and Entry Criteria

- The Role Assignment request belongs to exactly one resolved Tenant.
- The applicable institution, campus, department or other authorization scope can be resolved.
- The canonical A02 Person exists within the Tenant and is the intended assignment subject.
- Supporting Identity or User Account context, where relevant, can be resolved without becoming the canonical assignment target.
- Sufficient source business basis or eligibility evidence exists for the request.
- The selected A04 Role already exists and is available for assignment.
- The Role's Permission grouping has already been configured and validated.
- The assignment does not require modification of Role or Permission definitions.
- The proposed effective period is valid.
- Applicable least-privilege, authorization-policy and SoD rules can be evaluated.
- The Assignment Authority can determine whether separate approval is required.

---

# 8. Trigger

A governed authorization-layer Role Assignment request is presented to A04 for an eligible canonical A02 Person within a defined institutional scope.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the governed Role Assignment request with the canonical Person, requested Role, scope, effective period and source business basis/evidence. | Assignment Requestor; Authorization & Policy | The request is eligible to enter A04 evaluation. |
| 2 | Resolve and validate the Tenant and applicable institutional scope. | Platform & Tenant; Authorization & Policy | The assignment boundary is established without crossing Tenant boundaries. |
| 3 | Resolve the canonical A02 Person and supporting Identity/User Account context where relevant. | Person & Identity; Authorization & Policy | The assignment is associated with the correct canonical Person. |
| 4 | Validate the source business basis or eligibility evidence without re-owning the source decision. | Assignment Authority | The authorization request has a valid business basis. |
| 5 | Select the existing A04 Role representing the required authorization-layer responsibility. | Assignment Authority; Authorization & Policy — Role | The request refers to a governed Role without changing its definition. |
| 6 | Confirm the Role's grouped Permissions and define the proposed assignment scope and effective period. | Assignment Authority; Authorization & Policy — Role Assignment | Assignment boundaries are established while Permission definitions remain unchanged. |
| 7 | Record the governed Role Assignment request. | Assignment Administrator; Authorization & Policy — Role Assignment | The Role Assignment enters `Requested` and `RoleAssignmentRequested` is published by A04 Role Assignment. |
| 8 | Evaluate Tenant isolation, scope, effective-period, least-privilege, Authorization Policy and Separation of Duties constraints. | Assignment Authority; Authorization & Policy | The request is either eligible to proceed or follows Section 11 rejection handling. |
| 9 | Determine whether a distinct approval decision is required. | Assignment Authority | The approval path is canonically determined. |
| 10 | Where separate approval is required, route the assignment to `Pending Approval` and obtain the Assignment Approver decision; where policy explicitly permits no separate approval stage, record the governed approval through the Assignment Authority. | Assignment Authority; Assignment Approver where required | The assignment reaches `Approved` and `RoleAssignmentApproved` is published, or follows Section 11 rejection handling. |
| 11 | Preserve the approved assignment until its effective date while confirming activation prerequisites remain valid. | Assignment Administrator; Authorization & Policy — Role Assignment | The assignment remains `Approved` and is not yet effective if its effective date is in the future. |
| 12 | On the approved effective date, activate the Role Assignment. | Assignment Administrator; Authorization & Policy — Role Assignment | The assignment reaches `Active` and `RoleAssignmentActivated` is published. |
| 13 | Make the active Role Assignment available to later Role Resolution and Permission Resolution. | Authorization & Policy | The Role and its grouped Permissions may participate in later policy-based authorization decisions within the approved scope. |
| 14 | Preserve immutable evidence of request, validation, SoD evaluation, approval and activation. | Authorization & Policy; Audit Infrastructure | The outcome is traceable without changing historical Role, identity or organizational-authority facts. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Future-effective assignment | The approved effective date is later than the approval date. | Keep the Role Assignment in `Approved` until its governed effective date, then continue activation if prerequisites remain valid. | The Role does not participate in authorization decisions before `Active`. | Authorization & Policy |
| Multiple concurrent Roles | The Person already has another Role Assignment. | Evaluate all applicable assignments, scope rules, least privilege and SoD constraints. | Multiple `Active` Roles may coexist where policy permits. | Authorization & Policy |
| Institution-scoped responsibility | The Role applies across the institution. | Validate the institution boundary and applicable policies. | The assignment becomes effective only within the approved institution scope. | Authorization & Policy |
| Department- or campus-scoped responsibility | The Role applies to a narrower operational scope. | Validate the department or campus boundary and scope-specific policies. | Permissions participate only within the approved scope. | Authorization & Policy |
| Policy-permitted approval without separate approver stage | Governing policy explicitly permits approval without a distinct Assignment Approver. | Assignment Authority records the governed approval after all validation, SoD and least-privilege conditions are satisfied. | The same approved outcome is reached without inventing a separate approval actor. | Authorization & Policy |

Conditions that stop normal progression—such as prohibited SoD conflict, failed validation or denied approval—are handled only in Section 11 and are not alternate business flows.

Authorization Delegation, Temporary Access and Break-Glass remain separate scenarios.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant or institutional scope cannot be resolved. | The assignment cannot preserve Tenant isolation or scope validity. | Reject the request; if it already entered the lifecycle, move it to `Rejected` and publish `RoleAssignmentRejected`. | Platform & Tenant; Authorization & Policy |
| Canonical Person cannot be resolved or is ineligible under the supplied source evidence. | The Role Assignment cannot be associated with a valid subject/business basis. | Reject the request; source identity or eligibility correction remains with its canonical owner. | Person & Identity; Authorization & Policy |
| Selected Role is unavailable or invalid for assignment. | The requested responsibility cannot be represented by that governed Role. | Reject or correct the request without modifying the Role definition. | Authorization & Policy |
| Required scope information is invalid. | Effective access cannot be constrained to an approved boundary. | Correct before approval where permitted; otherwise reject the request. | Authorization & Policy |
| Effective period is invalid. | The assignment cannot be effective-dated consistently. | Correct before approval where permitted; otherwise reject the request. | Authorization & Policy |
| SoD conflict is prohibited. | Assignment would create a prohibited responsibility combination. | Move the request to `Rejected` and publish `RoleAssignmentRejected` unless an already-established policy permits a governed continuation path. | Authorization & Policy |
| Required approval is denied. | The assignment cannot become effective. | Move the assignment to `Rejected`, publish `RoleAssignmentRejected`, and do not expose it during Role Resolution. | Assignment Approver; Authorization & Policy |
| Request is withdrawn before effectiveness. | The assignment should not proceed to active access. | Move the eligible pre-effective assignment to `Cancelled` and publish `RoleAssignmentCancelled`. | Authorization & Policy |
| Activation prerequisites are not valid on the effective date. | Approved access must not become effective. | Keep the assignment non-active while the governed condition is resolved, or cancel/reject according to the applicable business decision; never treat it as `Active` prematurely. | Authorization & Policy |

No technical retry, rollback or implementation recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Canonical Person identity is Tenant-scoped. | Person & Identity / Platform Foundation | The Role Assignment references one canonical Person within the Tenant rather than an Institute-local duplicate identity. | Locked L1 Person-scope amendment; L2-A02 |
| Role Assignment canonical subject is the A02 Person reference. | Authorization & Policy | User Account is not the canonical assignment target. | L2-A04 locked 2026-08-26 amendment |
| L2-A04 exclusively owns Role and Role Assignment lifecycles. | Authorization & Policy | No other domain may create or transition the authorization Role Assignment. | L2-A04; ADR-PLAT-AUTH-001 |
| Role Assignment uses the canonical states and transitions defined by the locked A04 amendment. | Authorization & Policy | A010 may use `Requested`, `Pending Approval`, `Approved`, `Active`, `Rejected` and `Cancelled` only according to that lifecycle. | L2-A04 locked 2026-08-26 amendment |
| A Role represents an authorization-layer responsibility and groups business Permissions. | Authorization & Policy | Role selection does not create organizational authority or alter Identity. | L2-A04 |
| Permissions are assigned to Roles rather than permanently to individual users. | Authorization & Policy | A010 cannot create direct Person/User Account Permission grants. | L2-A04 |
| Role Assignment must preserve Tenant isolation and approved scope. | Authorization & Policy | Cross-Tenant or out-of-scope effectiveness is prohibited. | L2-A04 |
| Least privilege and applicable SoD constraints must be evaluated before effectiveness. | Authorization & Policy | Prohibited requests do not become `Active`. | L2-A04 locked 2026-08-26 amendment |
| Required approval is determined by Assignment Authority; distinct approval is used where policy or SoD requires it. | Authorization & Policy | Approval governance is explicit without hard-coding institutional job titles. | L2-A04 locked 2026-08-26 amendment |
| An E02 organizational Responsibility Assignment does not directly grant A04 Permissions. | Position & Assignment; Authorization & Policy | Upstream organizational responsibility may be source evidence only. | L2-E02 locked 2026-08-26 amendment |
| An A04 Role Assignment does not create E02 organizational authority. | Authorization & Policy; Position & Assignment | Authorization responsibility and organizational authority remain separate facts. | L2-E02 locked 2026-08-26 amendment |
| An active Role Assignment does not independently authorize a protected operation. | Authorization & Policy | Later protected requests still require A04 authorization decision evaluation. | L2-A04 |
| Role Assignment lifecycle changes and decisions are auditable. | Authorization & Policy; Audit Infrastructure | Request, decision and activation evidence must be preserved. | L2-A04 locked 2026-08-26 amendment; L2-A07 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Required Scenario Impact | Intended Result | Canonical Owner |
|---|---|---|---|---|
| Person | Existing canonical Tenant-scoped Person | Referenced as assignment subject; no Person transition. | Unchanged. | Person & Identity |
| Identity | Existing where applicable | May provide supporting authorization context; no Identity transition. | Unchanged. | Person & Identity |
| User Account | Existing or absent according to separate authentication context | Not the canonical assignment subject; no User Account transition. | Unchanged. | Person & Identity |
| Role | Existing and available for assignment | Referenced without modification. | Definition and lifecycle unchanged. | Authorization & Policy |
| Permission | Existing and grouped through the Role | Becomes indirectly available through the active Role Assignment. | Definition and Role grouping unchanged. | Authorization & Policy |
| Role Assignment | Does not yet exist or has just been requested | Progresses through `Requested`, optionally `Pending Approval`, `Approved`, and `Active`; failure paths may end `Rejected` or `Cancelled`. | One approved, scope-bounded `Active` Role Assignment. | Authorization & Policy |
| Authorization Policy | Applicable policy exists | Evaluated without policy-state change. | Unchanged. | Authorization & Policy |
| Authorization Decision | None for a specific protected request | No access decision is created merely by assignment. | Produced only when a later protected operation is evaluated. | Authorization & Policy |
| E02 Responsibility Assignment | May exist as upstream source evidence | Referenced only as business basis where applicable; no E02 transition occurs inside A010. | Unchanged. | Position & Assignment |

Later `Suspended`, `Expired` and `Revoked` Role Assignment lifecycle states remain governed by A04 but are outside A010's successful completion boundary.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Role Assignment Events

| Business Fact | Canonical Event | Sole Semantic Publisher | Scenario Use |
|---|---|---|---|
| Role Assignment entered lifecycle | `RoleAssignmentRequested` | A04 Role Assignment | Published when the governed request is recorded. |
| Role Assignment approved | `RoleAssignmentApproved` | A04 Role Assignment | Published when the assignment reaches `Approved`. |
| Role Assignment rejected | `RoleAssignmentRejected` | A04 Role Assignment | Published on governed rejection. |
| Role Assignment activated | `RoleAssignmentActivated` | A04 Role Assignment | Published when the assignment reaches `Active`; this is the successful lifecycle fact for A010. |
| Role Assignment cancelled | `RoleAssignmentCancelled` | A04 Role Assignment | Published when an eligible pre-effective assignment is cancelled. |

`RoleAssignmentSuspended`, `RoleAssignmentResumed`, `RoleAssignmentExpired` and `RoleAssignmentRevoked` are canonical A04 events defined upstream but occur outside A010's successful completion boundary.

L2-A06 Event Infrastructure owns governed distribution infrastructure; it is not the semantic publisher of these Role Assignment facts.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Tenant and institutional context | Platform & Tenant | Authorization & Policy | Tenant and approved institutional scope. |
| Canonical Person and supporting identity context | Person & Identity | Authorization & Policy | Canonical assignment subject and supporting authorization context. |
| Request Role Assignment | Authorized originating business process / Assignment Requestor | Authorization & Policy | Person, requested Role, scope, business basis/source evidence, effective period and originating context. |
| Role definition and grouped Permissions | Role and Permission aggregates | Role Assignment | Existing authorization responsibility and indirect Permission grouping. |
| Active Role Assignment | Role Assignment | Role Resolution and Permission Resolution | Assignment facts required for later policy-based authorization decisions. |
| Assignment audit input | Authorization & Policy | Audit Infrastructure | Request, source basis, scope, period, SoD, approval, actor/responsibility and lifecycle evidence. |

Where the source basis is an E02 organizational Responsibility Assignment, E02 retains ownership of that organizational fact; A04 owns the resulting Role Assignment.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Authentication success alone does not grant the assigned responsibility.
- An `Active` Role Assignment does not independently authorize a protected business operation.
- Every protected operation remains subject to centralized Role Resolution, Permission Resolution, Policy Evaluation, SoD Evaluation, Explicit Deny Evaluation and final Authorization Decision.
- Workspace selection cannot grant or remove Permissions.
- No technical RBAC, claim, IAM group or enforcement implementation is defined.

## 15.2 Approval and Separation of Duty

- Assignment Authority determines the applicable approval requirement under governing authorization policy.
- Where policy or SoD requires distinct approval, Assignment Approver grants or denies the assignment.
- The requestor does not obtain approval authority merely by initiating the request.
- Where policy explicitly permits approval without a separate approver stage, Assignment Authority may record the governed approval after all required validation and SoD evaluation.
- A prohibited SoD conflict cannot become `Active` unless an already-established policy explicitly permits a governed continuation path.

## 15.3 Audit Obligations

The following must be auditable:

- canonical assignment subject;
- Tenant and institutional scope;
- selected Role and referenced Permission grouping;
- source business basis or eligibility evidence;
- effective period;
- least-privilege and SoD evaluation outcomes;
- approval requirement and decision;
- acting canonical business responsibility;
- Role Assignment request, approval/rejection and activation/cancellation events;
- temporal and correlation context sufficient for institutional traceability.

Authorization & Policy owns Role Assignment facts and business meaning. Event Infrastructure owns governed distribution. Audit Infrastructure owns immutable evidence, integrity and retention.

---

# 16. Completion Criteria and Postconditions

The scenario is complete when:

- the request was governed within exactly one Tenant;
- the canonical A02 Person was resolved as the Role Assignment subject;
- a valid source business basis or eligibility evidence was accepted without ownership transfer;
- an existing Role was selected without modifying its definition;
- the Role's grouped Permissions remained unchanged;
- the assignment scope and effective period were validated;
- least-privilege and applicable SoD policies were satisfied;
- every required approval was granted;
- the Role Assignment progressed through its canonical A04 lifecycle to `Active`;
- `RoleAssignmentActivated` was published by A04 Role Assignment;
- the Role and grouped Permissions became available to later Role Resolution and Permission Resolution within scope;
- assignment request, decision and activation evidence became auditable;
- no Permission was assigned directly to the Person;
- no organizational authority was created by the A04 Role Assignment; and
- no authorization decision was created without a later protected business request.

The previously blocking upstream architecture required to validate these completion conditions is now established by the locked amendments approved through `ADR-PLAT-AUTH-001`.

---

# 17. Open Issues, Assumptions and Deferred Questions

No unresolved architecture blocker remains from the A010 upstream remediation package approved on 2026-08-26.

For traceability, the previously recorded items are closed as follows:

| ID | Status | Resolution | Authoritative Source |
|---|---|---|---|
| A010-BLOCKER-01 | Resolved | Canonical Role Assignment lifecycle and transition matrix established. | L2-A04 locked 2026-08-26 amendment |
| A010-BLOCKER-02 | Resolved | Canonical implementation-independent Role Assignment business operations/contracts established. | L2-A04 locked 2026-08-26 amendment |
| A010-BLOCKER-03 | Resolved | Exact canonical Role Assignment events and sole A04 publisher established. | L2-A04 locked 2026-08-26 amendment |
| A010-BLOCKER-04 | Resolved | Canonical Assignment Subject, Requestor, Authority, Approver, Administrator and Source Eligibility Authority responsibilities established. | L2-A04 locked 2026-08-26 amendment |
| A010-BLOCKER-05 | Resolved | Canonical Role Assignment subject is the A02 Person; legacy User Account target wording is superseded. | L2-A04 locked 2026-08-26 amendment |
| A010-BLOCKER-06 | Resolved | E02 organizational responsibility and A04 authorization Role Assignment are separated; governed Request Role Assignment hand-off defined without changing A010's frozen domain participation. | L2-E02 locked 2026-08-26 amendment |
| A010-ISSUE-01 | Resolved | Role Assignment lifecycle activities and evidence obligations are explicitly auditable. | L2-A04 locked 2026-08-26 amendment; L2-A07 |
| A010-ISSUE-02 | Resolved | Canonical Person scope aligned to Tenant level while Institute-scoped business contexts remain domain-owned. | Locked L1 2026-08-26 Person-scope amendment; L2-A02 |

No unsupported actor, approval authority, lifecycle state, transition, event, publisher, organizational hand-off or implementation mechanism is assumed by this Draft.