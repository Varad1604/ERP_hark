# Platform Authorization Upstream Architecture Approval Record — 2026-08-26

| Attribute | Value |
|---|---|
| Record Type | Architecture Review Decision / Repository Update Authorization |
| Date | 2026-08-26 |
| Branch | dhruv |
| Status | Approved and Incorporated |
| Approval Source | Explicit Lead Architect / repository-authority approval recorded in the architecture working session: “I approve them make it.” |

## 1. Purpose

This record closes the decision gate for the upstream architecture remediation required by L3-A-010 — Institutional Responsibility Assignment to Effective Authorized Access.

It records approval of the repository-consistent resolution package and authorizes its incorporation into Locked L1/L2 architecture. This approval does not authorize unrelated architecture changes or implementation-level design.

## 2. Approved ADR Decision

| ADR | Decision | Incorporated Effect |
|---|---|---|
| ADR-PLAT-AUTH-001 | **Approved — Role Assignment governance and Person-scope alignment.** Establish Tenant-scoped canonical Person identity, canonical A04 Role Assignment subject/responsibilities/lifecycle/contracts/events/audit obligations, and the E02-to-A04 organizational-responsibility hand-off boundary. | Locked L1 amendment v1.1.0; locked L2-A04 amendment v1.2.0; locked L2-E02 amendment v1.1.0; downstream synchronization of L3-A-010. |

## 3. Canonical Amendment Files

The following files are authoritative parts of their target architecture. Where they explicitly conflict with an earlier target version, the amendment has precedence for the amended subject only; all unaffected target-document content remains authoritative.

- `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Amendment_2026-08-26.md` — v1.1.0 amendment.
- `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A04_Authorization_and_Policy_Workflows_Amendment_2026-08-26.md` — v1.2.0 amendment.
- `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E02_Position_and_Assignment_Workflows_Amendment_2026-08-26.md` — v1.1.0 amendment.

## 4. Canonical Ownership Decisions

1. A02 remains canonical owner of Person, Identity and User Account identity truth.
2. The canonical Person is Tenant-scoped; Institute-scoped business contexts reference that Person rather than duplicating canonical identity.
3. A04 remains canonical owner of Role, Role Assignment, Permission, Authorization Policy and Authorization Decision.
4. E02 remains canonical owner of Position Assignment, Responsibility Assignment and organizational authority.
5. E02 organizational responsibility does not directly grant A04 authorization Permissions.
6. A04 Role Assignment does not create or replace E02 organizational authority.
7. An E02 or other source-domain business responsibility may provide source evidence for a governed A04 Role Assignment request without transferring ownership.

## 5. Role Assignment Lifecycle Decision

A04 Role Assignment exclusively owns:

`Requested`, `Pending Approval`, `Approved`, `Active`, `Suspended`, `Rejected`, `Cancelled`, `Expired`, `Revoked`.

The locked A04 amendment defines the complete governed transition set, approval rules and terminal-state semantics.

## 6. Role Assignment Publisher Decisions

The A04 Role Assignment aggregate is the sole semantic publisher of:

- `RoleAssignmentRequested`
- `RoleAssignmentApproved`
- `RoleAssignmentRejected`
- `RoleAssignmentActivated`
- `RoleAssignmentSuspended`
- `RoleAssignmentResumed`
- `RoleAssignmentCancelled`
- `RoleAssignmentExpired`
- `RoleAssignmentRevoked`

Event Infrastructure may distribute these events but does not own their business meaning.

## 7. Subject and Actor Decisions

- Canonical Role Assignment subject: A02 Person reference.
- Identity may support authorization context.
- User Account is authentication/runtime context, not the canonical assignment target.
- Canonical business responsibilities: Assignment Subject, Assignment Requestor, Assignment Authority, Assignment Approver, Assignment Administrator, Source Eligibility Authority.
- Institutional policies may map those responsibilities to local roles without changing the canonical responsibility boundaries.

## 8. Cross-Domain Hand-off Decision

Where an E02-owned organizational responsibility requires authorization-layer access, the originating E02/business process may invoke the A04 **Request Role Assignment** business contract using the E02 responsibility as source evidence.

A04 then independently governs scope, least privilege, Separation of Duties, approval and the Role Assignment lifecycle.

This does not reverse the foundational dependency direction and does not require adding Section E as a participating domain to L3-A-010, because A010 begins at the A04 authorization-assignment boundary.

## 9. Downstream Rule

L3-A-010 may now consume these decisions and remove the corresponding upstream blockers. It may not redefine the approved subject, actor responsibilities, lifecycle, event publisher, ownership or E02/A04 boundary.

The scenario itself remains subject to the normal L3 review/approval/lock process; this record approves the upstream architecture remediation, not automatic L3 lock status.

## 10. Repository Update Boundary

This approval record covers only:

- ADR-PLAT-AUTH-001;
- the three locked amendments listed above;
- synchronization of L3-A-010;
- governance index/changelog updates required for traceability.

No unrelated L1, L2, L3 or implementation architecture is approved by this record.