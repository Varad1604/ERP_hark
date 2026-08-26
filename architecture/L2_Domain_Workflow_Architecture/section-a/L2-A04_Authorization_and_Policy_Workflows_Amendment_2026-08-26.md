# L2-A04 — Authorization & Policy Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-A04 — Authorization & Policy Workflows v1.1 |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-PLAT-AUTH-001 |
| Authority | Lead Architect approval recorded 2026-08-26 |

This amendment is part of the canonical L2-A04 architecture. Where it conflicts with L2-A04 v1.1, this amendment is authoritative for Role Assignment governance. All unaffected L2-A04 content remains unchanged.

## 1. Canonical Role Assignment Subject

The canonical subject of a Role Assignment is an A02 **Person** reference.

- An A02 Identity reference may be associated as supporting authorization context.
- A User Account is authentication/runtime account context and is not the canonical Role Assignment target.
- The subject need not have an active authenticated session merely to receive a Role Assignment.
- Any earlier L2-A04 wording stating that Roles are assigned to authenticated User Accounts is superseded for Role Assignment subject semantics.

This decision preserves A02 ownership of Person, Identity and User Account truth.

## 2. Canonical Business Responsibilities

| Business Responsibility | Responsibility | Boundary |
|---|---|---|
| Assignment Subject | The A02 Person receiving the authorization-layer Role Assignment. | Has no request, approval or activation authority merely by being the subject. |
| Assignment Requestor | Initiates a governed Role Assignment request and supplies the authoritative business basis or evidence. | Does not approve merely because it requested. |
| Assignment Authority | Governs assignment scope, authorization-policy compliance, least privilege and determination of the required approval path. | Does not originate source-domain eligibility facts. |
| Assignment Approver | Grants or denies the Role Assignment where policy requires approval. | Must remain independently governable from the requestor where Separation of Duties requires it. |
| Assignment Administrator | Records and administers an approved assignment and makes it effective according to the approved scope and effective period. | Cannot bypass required approval, SoD or policy constraints. |
| Source Eligibility Authority | The authority in the originating business domain/process that establishes the business fact making the Person eligible for the responsibility. | A04 consumes that evidence; it does not redefine the source-domain eligibility decision. |

Institutions may map these canonical business responsibilities to local organizational roles. The responsibility boundaries remain canonical.

## 3. Canonical Role Assignment Lifecycle

The Role Assignment aggregate exclusively owns these states:

`Requested`, `Pending Approval`, `Approved`, `Active`, `Suspended`, `Rejected`, `Cancelled`, `Expired`, `Revoked`.

| From | Business Transition | To | Governing Condition |
|---|---|---|---|
| Not Existing | Request Role Assignment | Requested | Canonical A02 Person, Role, scope, business basis and effective-period information are sufficient to begin evaluation. |
| Requested | Route for required approval | Pending Approval | Policy requires a distinct approval decision. |
| Requested | Approve without separate approval stage | Approved | Policy explicitly permits approval without a separate approver stage and the Assignment Authority records the decision. |
| Pending Approval | Approve Role Assignment | Approved | Required approval is granted and applicable SoD/policy constraints are satisfied. |
| Requested / Pending Approval | Reject Role Assignment | Rejected | Required validation, policy, SoD or approval decision fails. |
| Requested / Pending Approval / Approved | Cancel Role Assignment | Cancelled | Governed cancellation occurs before effectiveness. |
| Approved | Activate Role Assignment | Active | Approved effective date is reached and activation prerequisites remain valid. |
| Active | Suspend Role Assignment | Suspended | Temporary governed suspension is required. |
| Suspended | Resume Role Assignment | Active | Suspension condition is resolved and resumption remains authorized. |
| Approved / Active / Suspended | Expire Role Assignment | Expired | Approved expiry condition or date is reached. |
| Active / Suspended | Revoke Role Assignment | Revoked | Authority is withdrawn before normal expiry. |

`Rejected`, `Cancelled`, `Expired` and `Revoked` are terminal for that Role Assignment instance. Re-establishment requires a new governed request. Historical states and decisions remain auditable.

## 4. Canonical Role Assignment Business Operations

L2-A04 owns the following implementation-independent business operations:

- Request Role Assignment.
- Determine Role Assignment Approval Requirement.
- Approve Role Assignment.
- Reject Role Assignment.
- Activate Role Assignment.
- Suspend Role Assignment.
- Resume Role Assignment.
- Cancel Role Assignment.
- Expire Role Assignment.
- Revoke Role Assignment.

A governed cross-domain Role Assignment request shall provide or reference:

- canonical A02 Person;
- requested A04 Role;
- requested authorization scope;
- originating business basis or source evidence;
- effective period where applicable; and
- originating business context sufficient for audit and policy evaluation.

This is a business contract only. No API, message schema, storage model or technical interface is defined.

## 5. Canonical Role Assignment Events

The A04 Role Assignment aggregate is the sole semantic publisher of the following business facts:

| Event | Publication Meaning | Canonical Publisher |
|---|---|---|
| `RoleAssignmentRequested` | A governed Role Assignment request has entered the Role Assignment lifecycle. | A04 Role Assignment |
| `RoleAssignmentApproved` | Required approval has been granted and the assignment is approved for effectiveness subject to its effective date and remaining prerequisites. | A04 Role Assignment |
| `RoleAssignmentRejected` | The requested assignment has been denied. | A04 Role Assignment |
| `RoleAssignmentActivated` | The approved assignment has become effective within its governed scope. | A04 Role Assignment |
| `RoleAssignmentSuspended` | An active assignment has been temporarily suspended. | A04 Role Assignment |
| `RoleAssignmentResumed` | A suspended assignment has been restored to active effectiveness. | A04 Role Assignment |
| `RoleAssignmentCancelled` | A non-effective assignment has been cancelled. | A04 Role Assignment |
| `RoleAssignmentExpired` | The assignment has reached its governed expiry. | A04 Role Assignment |
| `RoleAssignmentRevoked` | The assignment has been withdrawn before normal expiry. | A04 Role Assignment |

L2-A06 Event Infrastructure may distribute these events but does not own or republish them as canonical business truth.

## 6. Approval, Least-Privilege and Separation-of-Duties Rules

- Every Role Assignment must remain Tenant-isolated and scope-bounded.
- The Assignment Authority determines the applicable approval requirement under governing authorization policy.
- A separate Assignment Approver is required where policy or SoD requires independent approval.
- Where policy explicitly permits approval without a separate approver stage, the Assignment Authority may record the approval decision; this does not waive SoD or least-privilege evaluation.
- A prohibited SoD conflict results in rejection unless an already-established policy explicitly permits a governed continuation path.
- An assignment may not become `Active` before approval and its effective date.
- A Role Assignment grants no protected business operation by itself; later access still requires the normal A04 authorization decision process.

## 7. Organizational Responsibility Boundary

Role Assignment is an authorization-layer aggregate. It does not create organizational authority.

Where a business domain, including L2-E02, establishes an organizational responsibility that requires authorization-layer access, that source responsibility may be used as business evidence for **Request Role Assignment**. A04 independently owns and governs the resulting Role Assignment.

This hand-off:

- does not transfer source-domain ownership to A04;
- does not allow an A04 Role to become the source of organizational authority;
- does not make A04 depend on L2-E02; and
- preserves foundational authorization as a service consumed by business domains.

## 8. Role Assignment Audit Obligations

The following are canonical auditable Role Assignment activities:

- request;
- approval-requirement determination;
- approval or rejection;
- activation;
- suspension and resumption;
- cancellation;
- expiry;
- revocation;
- any separately governed amendment to scope or effective period;
- SoD and least-privilege evaluation outcomes relevant to the assignment.

Audit evidence shall preserve the acting business responsibility, assignment subject, Role, scope, effective period, source business basis, decision outcome and temporal/correlation context sufficient for institutional traceability.

L2-A07 Audit Infrastructure retains ownership of immutable audit evidence, evidence integrity and evidence retention. A04 retains ownership of Role Assignment business facts and lifecycle meaning.

## 9. Locked Decisions

The following are Locked under ADR-PLAT-AUTH-001:

1. Role Assignment canonical subject is the A02 Person reference.
2. User Account is not the canonical Role Assignment target.
3. A04 exclusively owns the Role Assignment lifecycle and transition semantics defined here.
4. A04 exclusively owns the Role Assignment business operations defined here.
5. A04 Role Assignment is the sole semantic publisher of the canonical Role Assignment events defined here.
6. Organizational responsibility/authority and authorization Role Assignment remain separate concepts.
7. Source-domain eligibility evidence may initiate an A04 Role Assignment request without transferring ownership.
8. Role Assignment audit obligations are explicit and A07 remains owner of immutable audit evidence.