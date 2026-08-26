# ADR-PLAT-AUTH-001 — Role Assignment Governance and Person Scope Alignment

| Field | Value |
|---|---|
| ADR ID | ADR-PLAT-AUTH-001 |
| Change Type | Cross-level architecture clarification and L2 capability completion |
| Target Locked Documents | SutraERP L1 Business Domain Architecture; L2-A04 Authorization & Policy Workflows; L2-E02 Position & Assignment Workflows |
| Supporting Authority | L2-A02 Person & Identity Workflows; DEPENDENCY_MAP.md |
| Blocked Downstream Scenario | L3-A-010 — Institutional Responsibility Assignment to Effective Authorized Access |
| Status | Approved — Repository Update Authorized |
| Decision Effect | Authorizes the locked amendments named in the approval record dated 2026-08-26 |
| Domain | Section A — Platform Foundation / Authorization, with Section E boundary clarification |
| Date | 2026-08-26 |

---

## 1. Need

L3-A-010 exposed eight upstream inconsistencies or omissions that cannot be resolved safely at L3:

1. L2-A04 owns Role Assignment approval, activation, suspension, expiry and revocation but does not define a canonical Role Assignment state model or transition rules.
2. L2-A04 does not define canonical business operations/contracts for requesting, approving, creating or making a Role Assignment effective.
3. L2-A04 does not define canonical Role Assignment lifecycle events or their sole publisher.
4. L2-A04 does not define the business responsibilities for requesting, governing, approving and administering Role Assignments.
5. Legacy L2-A04 wording states that Roles are assigned to authenticated User Accounts, while the later INT-301 aggregate amendment defines Role Assignment as binding a Role to an A02 Person/Identity reference.
6. L2-E02 owns organizational Responsibility Assignment and Organizational Authority and prohibits deriving organizational authority directly from a Security Role, but the hand-off into A04 authorization-layer Role Assignment is not defined.
7. L2-A04's auditable-event catalogue does not explicitly cover Role Assignment lifecycle changes.
8. Locked L1 §3.4 says that a Person record exists inside an Institute, while locked L2-A02 establishes Tenant-isolated canonical Person/Identity ownership.

These gaps prevent L3-A-010 from assigning exact actors, lifecycle effects, events and cross-domain responsibility without invention.

---

## 2. Repository Constraints

The decision shall preserve all of the following:

- A02 remains canonical owner of Person, Identity and User Account identity truth.
- A04 remains canonical owner of Role, Role Assignment, Permission, Authorization Policy and Authorization Decision.
- E02 remains canonical owner of Position Assignment, Responsibility Assignment and organizational authority facts.
- An organizational Responsibility Assignment does not itself grant authorization-layer Permissions.
- An A04 Role Assignment does not create or replace organizational authority.
- Authentication remains separate from authorization; a User Account is not the canonical Role Assignment subject.
- Cross-domain participation does not transfer ownership.
- Section A foundational dependencies are not reversed by making A04 depend on E02.
- Event Infrastructure may distribute events but does not become the semantic publisher of Role Assignment facts.
- Audit Infrastructure owns immutable audit evidence, not the Role Assignment business lifecycle.
- No API, database, UI, IAM-product or infrastructure design is introduced.

---

## 3. Approved Decisions

### 3.1 Canonical Person Scope

A canonical Person is Tenant-scoped. Every real human has exactly one canonical Person record within a Tenant. Institute-, campus-, student-, staff-, guardian- and other institutional contexts reference that Person and shall not create a duplicate Person merely because the same human participates in multiple Institutes within the same Tenant.

This decision aligns locked L1 with the later locked A02 Tenant-isolation architecture. It does not create cross-Tenant identity sharing.

### 3.2 Canonical Role Assignment Subject

The canonical subject of an A04 Role Assignment is an A02 Person reference.

An A02 Identity reference may be associated as supporting authorization context. A User Account is authentication/runtime account context and is not the canonical assignment target. Role Assignment therefore does not require the subject to have an active authenticated session or an already-active User Account unless a separate business rule explicitly requires one for the initiating actor.

Legacy L2-A04 wording that Roles are assigned to authenticated User Accounts is superseded for Role Assignment subject semantics.

### 3.3 Canonical Business Responsibilities

| Business responsibility | Canonical responsibility | Boundary |
|---|---|---|
| Assignment Subject | The A02 Person receiving the authorization-layer Role Assignment. | Has no request, approval or activation authority merely by being the subject. |
| Assignment Requestor | Initiates a governed request for a Role Assignment and supplies the authoritative business basis/evidence. | Does not approve merely because it requested. May belong to a consuming business process or an authorized administrative process. |
| Assignment Authority | A04 business authority accountable for assignment scope, authorization-policy compliance, least privilege and determination of the required approval path. | Does not originate source-domain eligibility facts. |
| Assignment Approver | Grants or denies the Role Assignment when policy requires approval. | Must remain independently governable from the requestor where Separation of Duties requires it. |
| Assignment Administrator | Records and administers an approved assignment and makes it effective according to the approved scope and effective period. | Cannot bypass required approval, SoD or policy constraints. |
| Source Eligibility Authority | The authority in the originating business domain/process that establishes the business fact making the Person eligible for a responsibility. | A04 consumes that evidence; A04 does not redefine the source-domain eligibility decision. |

Institutions may map these canonical responsibilities to local organizational roles. The responsibilities and separation boundaries remain canonical even when one person may hold multiple responsibilities where policy permits.

### 3.4 Canonical Role Assignment Lifecycle

The Role Assignment aggregate owns the following states:

`Requested`, `Pending Approval`, `Approved`, `Active`, `Suspended`, `Rejected`, `Cancelled`, `Expired`, `Revoked`.

Canonical transitions:

| From | Business transition | To | Governing condition |
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
| Approved / Active / Suspended | Expire Role Assignment | Expired | Approved expiry condition/date is reached. |
| Active / Suspended | Revoke Role Assignment | Revoked | Authority is withdrawn before normal expiry. |

`Rejected`, `Cancelled`, `Expired` and `Revoked` are terminal for that Role Assignment instance. Re-establishment requires a new governed request. Historical states and decisions remain auditable.

### 3.5 Canonical Business Operations / Contracts

A04 owns these implementation-independent Role Assignment business operations:

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

A cross-domain request into A04 shall provide or reference the canonical A02 Person, requested A04 Role, requested scope, business basis/source evidence, effective period where applicable, and originating business context. This is a business contract only; no API or message schema is defined here.

### 3.6 Canonical Role Assignment Events

The A04 Role Assignment aggregate is the sole semantic publisher of these business facts:

- `RoleAssignmentRequested`
- `RoleAssignmentApproved`
- `RoleAssignmentRejected`
- `RoleAssignmentActivated`
- `RoleAssignmentSuspended`
- `RoleAssignmentResumed`
- `RoleAssignmentCancelled`
- `RoleAssignmentExpired`
- `RoleAssignmentRevoked`

Event Infrastructure governs distribution infrastructure after publication but does not own or republish these facts as canonical business truth.

### 3.7 E02 Organizational Responsibility to A04 Authorization Boundary

E02 Responsibility Assignment and A04 Role Assignment are separate aggregates serving different business meanings:

- E02 establishes organizational responsibility and organizational authority.
- A04 establishes authorization-layer Role Assignment used during later Role Resolution, Permission Resolution and policy evaluation.
- No A04 Role or Role Assignment may be treated as the source of E02 organizational authority.
- No E02 Responsibility Assignment directly grants A04 Permissions.
- When an E02-owned organizational responsibility requires authorization-layer access, the originating E02/business process may invoke the governed **Request Role Assignment** business contract into A04 using the E02 responsibility as source evidence. A04 independently evaluates scope, least privilege, SoD and approval requirements and owns the resulting Role Assignment lifecycle.
- This is a hand-off from a consuming business domain into foundational A04 capability; it does not make A04 dependent on E02 and does not transfer ownership in either direction.

For L3-A-010 specifically, the scenario boundary begins with the authorization assignment requirement already presented to A04. The originating organizational-responsibility establishment remains outside A010, so its frozen Participating Domains value `A` does not change.

### 3.8 Role Assignment Audit Obligations

The following Role Assignment activities are canonical auditable business activities:

- request;
- approval requirement determination;
- approval or rejection;
- activation;
- suspension and resumption;
- cancellation;
- expiry;
- revocation;
- scope/effective-period changes permitted by a separately governed amendment process;
- SoD and least-privilege evaluation outcomes relevant to the assignment.

Audit evidence shall preserve the acting business responsibility, assignment subject, Role, scope, effective period, source business basis, decision outcome and temporal/correlation context sufficient for institutional traceability. A07 remains owner of immutable audit evidence and its retention/integrity lifecycle.

---

## 4. Impact Analysis

### L1

The Person boundary changes from Institute-scoped wording to Tenant-scoped canonical identity. Institute-scoped business contexts remain Institute-scoped. No business domain ownership transfer occurs.

### L2-A02

No ownership change is required. This ADR aligns L1 and A04 to A02's existing Tenant-isolated canonical Person/Identity architecture and INT-301 cross-domain identity contract.

### L2-A04

Role Assignment becomes complete enough for downstream scenarios to reference exact actors/responsibilities, subject semantics, lifecycle transitions, business operations and events without invention.

### L2-E02

Organizational authority remains E02-owned. A governed outward hand-off to A04 is clarified without converting E02 responsibilities into authorization Roles or reversing dependency direction.

### L3-A-010

The scenario can replace blocker language with canonical references to these approved upstream decisions. Its scenario identity, owner, title, trigger, outcome and frozen Participating Domains remain unchanged.

---

## 5. Decision

**Approved on 2026-08-26.**

Approval source: explicit repository-authority approval in the architecture working session: **“I approve them make it.”**

Approved implementation boundary:

1. publish locked amendments for L1, L2-A04 and L2-E02 containing the decisions above;
2. record the approval/incorporation decision;
3. synchronize L3-A-010 to remove resolved upstream blockers;
4. update ADR navigation and repository changelog;
5. do not modify unrelated L1/L2/L3 architecture.

---

## 6. Acceptance Criteria

1. L1 canonical Person scope is unambiguously Tenant-scoped without creating cross-Tenant sharing.
2. A04 Role Assignment subject is the A02 Person reference; User Account wording cannot override this rule.
3. Role Assignment has one canonical state model owned only by A04.
4. Role Assignment has exact business responsibility boundaries and approval/SoD semantics.
5. Role Assignment business operations are defined without implementation design.
6. Every Role Assignment lifecycle event has exactly one canonical publisher: A04 Role Assignment.
7. E02 organizational responsibility and A04 authorization Role Assignment remain distinct and non-overlapping.
8. E02-to-A04 authorization hand-off does not reverse the dependency map or transfer ownership.
9. Role Assignment audit activities are explicitly governed.
10. L3-A-010 can be authored without unresolved actor, lifecycle, event, publisher or subject assumptions.