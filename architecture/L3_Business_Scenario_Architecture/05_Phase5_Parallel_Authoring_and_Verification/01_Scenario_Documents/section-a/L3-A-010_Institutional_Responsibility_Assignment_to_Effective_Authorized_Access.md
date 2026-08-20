# Institutional Responsibility Assignment to Effective Authorized Access

# 1. Scenario Metadata

**Scenario ID:** L3-A-010  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authorization & Policy  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Business domains requiring centralized authorization decisions

---

# 2. Scenario Purpose

Establish an approved, scoped and effective-dated A04 Role Assignment for an eligible Person so the Role and its grouped Permissions participate in centralized policy-based authorization decisions without modifying the Role definition or assigning Permissions directly to the Person.

---

# 3. Business Outcome

The role and its grouped permissions become effective within the approved scope and participate in policy-based authorization decisions, without permanently altering the role definition.

Locked L2-A04 does not define the Role Assignment workflow, lifecycle states, commands, canonical events or publishers required to make this outcome exact. The scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an eligible Person requires an approved organizational responsibility within a defined institutional scope.

## 4.2 End Boundary

The intended end boundary is reached when:

- an existing Role has been assigned through an authorized administrative process;
- the Role Assignment has received every required approval;
- the assignment is effective within its approved Tenant, institution, campus, department or other authorized scope;
- the Role’s grouped Permissions participate in policy-based authorization evaluation;
- the Role and Permission definitions remain unchanged; and
- the assignment and its approval and activation history are auditable.

L2-A04 does not define the exact assignment states, transitions, event names or publishers needed to validate this end boundary.

## 4.3 In Scope

- Tenant and institutional-scope validation.
- Resolution of the eligible A02 Person, Identity and applicable User Account references.
- Selection of an existing Role representing the required responsibility.
- Validation that the Role has an approved grouping of Permissions.
- Definition of assignment scope.
- Definition of effective and expiry dates where applicable.
- Least-privilege validation.
- Role Assignment constraint evaluation.
- Separation of Duties evaluation.
- Determination and completion of applicable administrative approval.
- Creation and activation of the Role Assignment.
- Availability of the assigned Role and its grouped Permissions for policy-based authorization decisions.
- Immutable audit evidence for assignment, approval and activation.

## 4.4 Out of Scope

- Creating, modifying, activating, suspending or archiving the Role definition.
- Adding or removing Permissions from the Role.
- Assigning Permissions directly to a Person or User Account.
- Position Assignment, workforce occupancy or organizational-authority creation owned by L2-E02.
- Authorization Delegation.
- Assignment Delegation owned by L2-E02.
- Temporary Access Grants.
- Break-Glass or emergency access.
- Direct execution of a protected business operation.
- Authentication or Authentication Session creation.
- Workspace definition, navigation or presentation behaviour.
- Technical RBAC, IAM groups, claims, ACLs, APIs, database structures, UI design or infrastructure.
- Inventing Role Assignment states, events, publishers, commands or approval authorities.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Upstream | L3-A-007 — Eligible Identity to Authenticated Session Availability | Authentication establishes identity but does not grant authorization. |
| Related | L3-A-011 — Responsibility Delegation to Automatic Authority Reversion | Owns temporary Authorization Delegation and does not permanently change the Role Assignment. |
| Related | L3-A-012 — Temporary Access Request to Governed Access Expiry | Owns exceptional time-bound access beyond normal Role Assignments. |
| Related | L3-A-013 — Emergency Access Request to Audited Emergency Access Closure | Owns controlled Break-Glass access rather than normal responsibility assignment. |
| Related control | L3-A-051 — Conflicting Responsibility Risk to Enforced Separation of Duties | Governs the broader outcome when conflicting responsibilities require enforced SoD treatment. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authorization & Policy / Section A | Primary owner. Owns Role, Role Assignment, Permission, Authorization Policy, SoD Policy, assignment approval, activation, suspension, expiry, revocation and later Authorization Decisions. |
| Platform & Tenant / Section A | Supplies Tenant, institution, organization and campus context used to validate the assignment scope. |
| Person & Identity / Section A | Supplies canonical Person, Identity and User Account references without owning the Role Assignment or Permissions. |
| Authentication / Section A | Supplies authenticated context when the assigned Person later requests access; authentication does not create or activate the Role Assignment. |
| Event Infrastructure / Section A | Owns governed event publication and delivery infrastructure without owning Role Assignment facts. |
| Audit Infrastructure / Section A | Owns immutable audit evidence, integrity, retention and evidence lifecycle. |

The frozen Participating Domains value remains exactly `A`.

L2-E02 was inspected only to validate the ownership boundary. Adding Section E as a participant would change the frozen Participating Domains value and is not performed by this Draft.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Eligible Person | Receives the approved authorization-layer responsibility within the defined scope. | Business subject identified by the frozen trigger; no assignment or approval authority is implied. |

L2-A04 states that Role Assignments occur through “authorized administrative processes” and that the Role Assignment owns “assignment authority,” but it does not name:

- the actor who requests the assignment;
- the actor who determines Person eligibility;
- the assignment authority;
- the approval role; or
- the actor who activates the assignment.

No administrator, manager, security officer, HR actor or approval authority is invented in this scenario.

---

# 7. Preconditions and Entry Criteria

- The assignment request belongs to exactly one resolved Tenant.
- The applicable institution, campus, department or other assignment scope can be resolved.
- A canonical Person and associated Identity exist and are eligible for the proposed assignment.
- Where authenticated access is required, an applicable User Account reference exists.
- The selected Role already exists and is available for assignment.
- The Role represents the required authorization-layer responsibility.
- The Role’s Permission grouping has already been configured and validated.
- The assignment does not require modification of the Role or Permission definitions.
- The proposed effective period and expiry constraints, where applicable, are available.
- Applicable Role Assignment, least-privilege, Authorization Policy and SoD rules can be evaluated.
- The authorized administrative process and required approval are expected to exist, although their canonical actors and contracts are not defined by L2-A04.

---

# 8. Trigger

An eligible Person requires an approved organizational responsibility within a defined institutional scope.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the requirement to assign an approved responsibility to an eligible Person within a defined scope. | Authorization & Policy | A Role Assignment requirement is identified. |
| 2 | Resolve the Tenant, institution, organization, campus, department and other applicable scope references. | Platform & Tenant; Authorization & Policy | The proposed assignment boundary is established without crossing Tenant boundaries. |
| 3 | Resolve the canonical Person, Identity and applicable User Account references through L2-A02 contracts. | Person & Identity; Authorization & Policy | The proposed assignment is associated with the correct A02 reference. |
| 4 | Select the existing Role that represents the required authorization-layer responsibility. | Authorization & Policy — Role | The assignment refers to a governed Role without changing its definition. |
| 5 | Confirm that the Role’s grouped Permissions are configured and available for later policy evaluation. | Authorization & Policy — Role and Permission | The Permission grouping remains attached to the Role rather than directly to the Person. |
| 6 | Define the proposed institutional scope, effective date and expiry date where applicable. | Authorization & Policy — Role Assignment | The intended assignment boundaries and effective period are established. |
| 7 | Evaluate Tenant isolation, institution, department, campus, effective-date, expiry-date and least-privilege constraints. | Authorization & Policy | The proposed assignment satisfies the applicable assignment constraints. |
| 8 | Evaluate applicable Separation of Duties Policies, including other assigned, delegated or temporary responsibilities. | Authorization & Policy — Separation of Duties Policy | A conflicting responsibility is detected or the assignment remains eligible to proceed. |
| 9 | Determine the applicable administrative approval requirement. | Authorization & Policy | The required approval path is identified, but L2-A04 does not name its actor or contract. |
| 10 | Obtain the required approval through the authorized administrative process. | Authorization & Policy | The responsibility is approved; the exact approval transition and state are not defined upstream. |
| 11 | Create the Role Assignment binding the Role to the appropriate A02 Person/Identity reference, approved scope, effective period, status and assignment authority. | Authorization & Policy — Role Assignment | The Role Assignment aggregate exists; the exact initial state and creation event are not defined upstream. |
| 12 | Activate or otherwise make the Role Assignment effective on its approved effective date. | Authorization & Policy — Role Assignment | The assigned Role and grouped Permissions become effective within scope; the canonical activation state and event are not defined upstream. |
| 13 | Include the effective Role Assignment during Role Resolution and Permission Resolution for later protected requests. | Authorization & Policy | The Role and its grouped Permissions can participate in policy-based authorization decisions. |
| 14 | Preserve immutable evidence of validation, SoD evaluation, approval, assignment and activation. | Authorization & Policy; Audit Infrastructure | The assignment outcome is traceable without changing historical Role definitions. |

Steps 9–12 cannot be finalized as exact L3 behaviour because the authoritative actors, workflow, states, transitions, commands and events are absent from locked L2-A04.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Future-effective assignment | The approved effective date is later than the approval date. | Preserve the approved assignment until its effective date and make it effective only when the applicable date is reached. | The Role must not participate in authorization decisions before becoming effective. Exact pre-effective states are undefined. | Authorization & Policy |
| Multiple concurrent Roles | The Person already has another Role Assignment. | Evaluate all applicable assignments, scope rules, least privilege and SoD constraints. | Multiple active Roles may coexist where permitted. | Authorization & Policy |
| Institution-scoped responsibility | The Role applies across the institution. | Validate the institution boundary and applicable policies. | The assignment becomes effective only within the approved institution scope. | Authorization & Policy |
| Department- or campus-scoped responsibility | The Role applies to a narrower operational scope. | Validate the department or campus boundary and scope-specific policies. | Permissions participate only within the approved scope. | Authorization & Policy |
| Configurable administrative approval | Tenant policy requires additional administrative approval. | Complete the policy-required approval before assignment effectiveness. | The outcome remains blocked until approval succeeds. The approving role is not defined upstream. | Authorization & Policy |
| SoD conflict | The proposed Role conflicts with another responsibility. | Deny the assignment unless an approved policy explicitly permits continuation. | No effective Role Assignment is produced through the normal path while the conflict remains prohibited. | Authorization & Policy |

Authorization Delegation, Temporary Access and Break-Glass are not alternate A010 paths; they remain separate scenarios.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant or institutional scope cannot be resolved. | The assignment cannot preserve Tenant isolation or scope validity. | Reject the assignment without modifying the Role or Permission definitions. | Platform & Tenant; Authorization & Policy |
| Person or Identity cannot be resolved or is ineligible. | The Role Assignment cannot be associated with a valid canonical subject. | Reject the assignment; Person and Identity correction remains governed by L2-A02. | Person & Identity; Authorization & Policy |
| Selected Role is unavailable or not valid for assignment. | The responsibility cannot be assigned through that Role. | Stop assignment without modifying the Role. Role lifecycle governance remains separate. | Authorization & Policy |
| Required scope information is invalid. | Effective access cannot be constrained to an approved boundary. | Reject or correct the assignment request before approval. | Authorization & Policy |
| Effective or expiry dates are invalid. | The assignment cannot be effective-dated consistently. | Stop progression until a valid period is supplied. | Authorization & Policy |
| SoD conflict is prohibited. | Assignment would create conflicting responsibilities. | Deny the assignment unless an approved policy explicitly permits continuation. | Authorization & Policy |
| Required approval is not granted. | The assignment cannot become effective. | Do not activate or expose the assignment during Role Resolution. | Authorization & Policy |
| Assignment creation is interrupted. | No reliable assignment outcome exists. | Do not treat the responsibility as effective. L2-A04 does not define a Role Assignment recovery workflow. | Authorization & Policy |
| Assignment activation cannot be completed. | The Role and Permissions must not participate in authorization evaluation. | Preserve the incomplete auditable outcome; exact recovery states and transitions are undefined. | Authorization & Policy |
| Canonical assignment event cannot be identified. | Downstream consumers cannot receive a governed assignment fact. | Do not invent an event or publisher; obtain an upstream L2-A04 decision. | Authorization & Policy |

No technical retry, rollback or implementation recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| L2-A04 exclusively owns Role and Role Assignment lifecycles. | Authorization & Policy | No other domain may create or activate the authorization Role Assignment. | L2-A04 — Architectural Responsibility and INT-301 |
| A Role represents an organizational responsibility and groups business Permissions. | Authorization & Policy | The selected Role provides the responsibility and Permission grouping without owning Identity. | L2-A04 — Role aggregate; AP-08 |
| Role Assignment binds a Role to an A02 Person/Identity reference within a scope, effective period, status and assignment authority. | Authorization & Policy | The assignment remains separate from both Role and Identity lifecycles. | L2-A04 — Aggregate Ownership and Role responsibility |
| Role Assignment owns approval, activation, suspension, expiry and revocation. | Authorization & Policy | Assignment effectiveness cannot be delegated to another aggregate. | L2-A04 — Role responsibility |
| Roles are assigned through authorized administrative processes. | Authorization & Policy | Normal self-assignment is not established by the locked architecture. | L2-A04 — Role Assignment |
| A user may hold multiple active Roles simultaneously. | Authorization & Policy | Existing Roles do not automatically prevent a new assignment. | L2-A04 — Role Assignment |
| Role Assignment must preserve Tenant isolation. | Authorization & Policy | Cross-Tenant assignment is prohibited. | L2-A04 — Role Assignment Constraints; AP-12 |
| Assignment validation may include institution, department, campus, effective dates, expiry dates, SoD and approval requirements. | Authorization & Policy | Scope and governance constraints must be evaluated before effectiveness. | L2-A04 — Role Assignment Constraints |
| Permissions are assigned to Roles rather than permanently to individual users. | Authorization & Policy | A010 cannot create direct user-Permission grants. | L2-A04 — Permission Assignment and Permission Invariants |
| Users receive only Permissions required for assigned responsibilities. | Authorization & Policy | Assignment must follow least privilege. | L2-A04 — AP-10 |
| Roles do not grant authorization independently. | Authorization & Policy | The assigned Role and Permissions only participate in later policy evaluation. | L2-A04 — Role Invariants |
| A granted Permission does not automatically authorize execution. | Authorization & Policy | Explicit Deny, scope, resource state, policy and SoD may still deny later access. | L2-A04 — Permission Constraints |
| Explicit Deny overrides Allow. | Authorization & Policy | Later authorization decisions remain deny-precedent even after assignment. | L2-A04 — AP-06 |
| Workspace selection does not change Role Assignment or authorization outcomes. | Authorization & Policy | Workspace configuration remains outside assignment effectiveness. | L2-A04 — AP-07, AP-09 and Role Assignment |
| Every Role and Permission change and authorization decision must be auditable. | Authorization & Policy; Audit Infrastructure | Assignment approval and activation require immutable traceability, although assignment-specific audit events are missing. | L2-A04 — AP-11 and Authorization Audit |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Required Scenario Impact | Intended Result | Canonical Owner |
|---|---|---|---|---|
| Person | Eligible canonical Person | Referenced by the assignment; no Person transition. | Unchanged. | Person & Identity |
| Identity | Existing canonical Identity | Referenced by the assignment; no Identity transition. | Unchanged. | Person & Identity |
| User Account | Existing where authenticated access is required | Supplies the account reference used during later authentication and authorization. | Unchanged. | Person & Identity |
| Role | Existing and available for assignment | Referenced without modification. | Definition and lifecycle unchanged. | Authorization & Policy |
| Permission | Existing and grouped through the Role | Becomes indirectly available through the effective Role Assignment. | Definition and Role grouping unchanged. | Authorization & Policy |
| Role Assignment | Does not yet exist | Must be validated, approved, created and made effective within scope. | Intended result is an effective approved Role Assignment; canonical states are undefined. | Authorization & Policy |
| Authorization Policy | Applicable policy exists | Evaluated without policy-state change. | Unchanged. | Authorization & Policy |
| Authorization Decision | None for a specific protected request | No access decision is created merely by assignment. | Produced only when a later protected operation is evaluated. | Authorization & Policy |
| Workspace | Existing or absent | Role Assignment may influence workspace resolution through authorized mapping, but does not change Workspace state. | Unchanged. | Authorization & Policy |

L2-A04 names approval, activation, suspension, expiry and revocation as Role Assignment lifecycle responsibilities but provides no canonical state names or transition matrix. This Draft therefore does not introduce `Requested`, `Approved`, `Active`, `Suspended`, `Expired` or `Revoked` as Role Assignment states.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Role Assignment Event Determination

Locked L2-A04 does not define a canonical Role Assignment event catalogue, Aggregate-to-Event Matrix or sole publisher.

| Required Business Fact | Source Resolution Result | Scenario Treatment |
|---|---|---|
| Role Assignment requested or created | No canonical event name or publisher defined. | No event is invented. |
| Role Assignment approved | No canonical event name or publisher defined. | No event is invented. |
| Role Assignment activated or made effective | No canonical event name or publisher defined. | No event is invented. |
| Role Assignment suspended | Lifecycle ownership stated, but no event defined. | Outside A010 completion; no event is invented. |
| Role Assignment expired | Lifecycle ownership stated, but no event defined. | Outside A010 completion; no event is invented. |
| Role Assignment revoked | Lifecycle ownership stated, but no event defined. | Outside A010 completion; no event is invented. |

L2-A04 lists human-readable auditable activities such as `Role Created`, `Permission Assigned`, `Authorization Granted` and `Authorization Denied`. These are not used as A010 Role Assignment events because:

- Role creation is outside A010;
- Permission assignment to the Role is outside A010;
- authorization decisions occur only during later protected requests; and
- the document does not define canonical event names or publishers for those labels.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Tenant and institutional context | Platform & Tenant | Authorization & Policy | Tenant, institution, organization and campus scope. |
| Person, Identity and User Account references | Person & Identity | Authorization & Policy | Canonical assignment subject and authentication-reference context. |
| Authentication Session | Authentication | Authorization & Policy | Authenticated context for later authorization requests; not a precondition for assigning the target Person unless policy requires it. |
| Role definition and grouped Permissions | Role and Permission aggregates | Role Assignment | Existing responsibility and indirect Permission grouping. |
| Effective Role Assignment | Role Assignment | Role Resolution and Permission Resolution | Assignment facts required for later policy-based authorization decisions. |
| Authorization Decision | Authorization & Policy | Business domains | Later immutable outcome for a specific protected business request. |
| Assignment audit input | Authorization & Policy | Audit Infrastructure | Scope, dates, approval, SoD, assignment authority and effectiveness evidence. |

The contract through which L2-E02 organizational authority should inform A04 Role Assignment is not defined. Adding an E-domain hand-off here would change the frozen participating-domain architecture and is therefore deferred to Section 17.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Authentication success alone does not grant the assigned responsibility.
- An effective Role Assignment does not independently authorize a business operation.
- Every protected operation must still undergo centralized Role Resolution, Permission Resolution, Policy Evaluation, SoD Evaluation, Explicit Deny Evaluation and final Authorization Decision.
- Workspace selection cannot grant or remove Permissions.
- No technical RBAC, claim, IAM group or enforcement implementation is defined.

## 15.2 Approval and Separation of Duty

- The frozen trigger requires an approved responsibility.
- L2-A04 states that Role Assignment owns assignment approval and supports configurable administrative approval requirements.
- L2-A04 does not name the requestor, assignment authority or approving role.
- Applicable SoD Policies must be evaluated before the assignment becomes effective.
- A prohibited SoD conflict must be denied unless an approved policy explicitly permits continuation.
- No override actor or new approval authority is invented.

## 15.3 Audit Obligations

The following must be auditable:

- Tenant and institutional scope.
- Person, Identity and applicable User Account references.
- Selected Role.
- Grouped Permissions referenced by the Role.
- Effective and expiry dates.
- Least-privilege validation.
- SoD evaluation.
- Required approval and approval outcome.
- Assignment authority.
- Assignment creation.
- Assignment effectiveness.
- Historical Role and Permission versions relevant to later decisions.
- Actor, timestamp, correlation and outcome information.

Authorization & Policy owns the assignment facts. Event Infrastructure owns governed event distribution after canonical assignment events exist. Audit Infrastructure owns immutable evidence and its lifecycle.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are:

- The request was processed within exactly one Tenant.
- The eligible Person and canonical Identity were resolved.
- An existing Role was selected without modifying its definition.
- The Role’s grouped Permissions remained unchanged.
- The assignment scope and effective period were defined.
- Tenant, institution, department, campus, date and least-privilege constraints were satisfied.
- Applicable SoD Policies were evaluated.
- Every required approval was granted.
- A Role Assignment was created under its canonical assignment authority.
- The Role Assignment became effective within the approved scope.
- The Role and its grouped Permissions became available to Role Resolution and Permission Resolution.
- Assignment approval and effectiveness were auditable.
- No Permission was assigned directly to the Person.
- No authorization decision was created without a later protected business request.

These criteria cannot be validated as a complete canonical outcome because locked L2-A04 does not define:

- the Role Assignment workflow;
- canonical Role Assignment states;
- transition rules;
- commands or assignment contracts;
- canonical Role Assignment events and publishers;
- the assignment and approval actors; or
- the relationship between L2-E02 organizational responsibility and A04 authorization Role Assignment.

The scenario therefore remains `BLOCKED`.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Precise Scenario Impact | Required Resolution |
|---|---|---|---|---|
| A010-BLOCKER-01 | Blocking | L2-A04 declares Role Assignment as an aggregate owning approval, activation, suspension, expiry and revocation but provides no Role Assignment workflow or lifecycle-state model. | The Draft cannot define the exact creation, approval and effectiveness transitions required for the frozen outcome. | Add the canonical Role Assignment workflow, state definitions and transition matrix to L2-A04. |
| A010-BLOCKER-02 | Blocking | L2-A04 defines no Role Assignment commands or assignment-specific synchronous contracts. | L4 cannot identify the canonical business operations that request, approve, create or activate the assignment. | Define the authoritative commands/contracts and owning aggregate responsibilities. |
| A010-BLOCKER-03 | Blocking | L2-A04 defines no canonical Role Assignment events or sole publishers. | The Draft cannot publish assignment creation, approval or effectiveness as canonical business facts without invention. | Define exact Role Assignment event names, publication conditions and one canonical publisher per event. |
| A010-BLOCKER-04 | Blocking | L2-A04 does not name the requestor, Person-eligibility decision-maker, assignment authority, approver or activation authority. | Business responsibilities and approval authority cannot be assigned in Sections 6, 9 and 15. | Define the canonical business actors and their authority boundaries. |
| A010-BLOCKER-05 | Blocking | L2-A04 §27 says Roles are assigned to authenticated User Accounts, while the later INT-301 amendment defines Role Assignment as binding a Role to an A02 Person/Identity reference. | The canonical assignment subject and cross-domain reference contract are inconsistent. The Draft follows the later amendment but cannot resolve the User Account wording. | Align L2-A04 on whether Role Assignment targets Person, Identity, User Account or an explicitly defined reference combination. |
| A010-BLOCKER-06 | Blocking | L2-E02 owns organizational Responsibility Assignment and Organizational Authority and states that authority must not be derived directly from a Security Role. Frozen A010 begins with an approved organizational responsibility but assigns an A04 Role, with Participating Domains frozen to `A`. | The source of organizational responsibility, the E02-to-A04 hand-off and the distinction between organizational authority and authorization-layer responsibility are undefined. | Reconcile L2-E02 and L2-A04 without changing A010’s frozen owner/domains unless Phase 4 governance formally reopens them. |
| A010-ISSUE-01 | Non-blocking | L2-A04 lists Role and Permission audit activities but omits Role Assignment creation, approval and activation from its auditable-event list. | Assignment-specific audit meaning is incomplete even though AP-11 requires every permission grant and authorization activity to be audited. | Add Role Assignment audit activities and their correlation requirements. |
| A010-ISSUE-02 | Non-blocking | Locked L1 §3.4 uses an Institute-scoped Person boundary while A02 and A04 use Tenant-scoped identity and authorization references. | The Draft follows L2 Tenant isolation, but the enterprise scope of the target Person remains inconsistent. | Lead Architect alignment is required before Architecture Approval or locking. |

No unsupported actor, approval authority, Role Assignment state, transition, event, publisher, organizational hand-off or implementation mechanism is assumed by this Draft.
