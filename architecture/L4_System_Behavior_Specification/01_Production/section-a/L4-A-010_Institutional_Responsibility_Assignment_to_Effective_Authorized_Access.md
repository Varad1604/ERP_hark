# Institutional Responsibility Assignment to Effective Authorized Access

## 1. Specification Metadata

**Specification ID:** L4-A-010  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Authorization & Policy  
**Realizes Scenario:** L3-A-010  
**Primary Business Owner:** Authorization & Policy  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-A-010_Institutional_Responsibility_Assignment_to_Effective_Authorized_Access.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows_Amendment_2026-08-26.md`; `SutraERP_L1_Business_Domain_Architecture_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a Role Assignment for the canonical Tenant-scoped Person, enforce source-defined scope, effective-period, least-privilege, SoD and approval conditions, and make the Role Assignment effective only when it reaches `Active`. The assignment shall not modify Role/Permission definitions, create organizational authority, or itself authorize a protected business operation.

## 3. Required Business Context

- Canonical A02 Person as Role Assignment subject.
- Existing governed A04 Role and its grouped Permissions.
- Source business basis/eligibility evidence.
- Assignment scope and effective period.
- Applicable Authorization Policy, least-privilege and SoD rules.
- Canonical Role Assignment lifecycle and events from the Locked A04 amendment.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-010-BH-001 | The system shall prevent a Role Assignment request unless the Tenant, canonical Person subject, existing Role, scope, effective period and source business basis are resolvable. | L3-A-010 §§7, 9, 12 |
| L4-A-010-BH-002 | When a valid request is recorded, the system shall place the Role Assignment in `Requested` and publish `RoleAssignmentRequested` on behalf of A04 Role Assignment. | L3-A-010 §§9, 13–14 |
| L4-A-010-BH-003 | Before approval, the system shall evaluate the source-defined Tenant isolation, scope, effective-period, least-privilege, Authorization Policy and SoD constraints. | L3-A-010 §§9, 12, 15 |
| L4-A-010-BH-004 | When policy requires a distinct approval decision, the system shall route the assignment through `Pending Approval` and shall prevent progression to `Approved` until the required Assignment Approver decision exists. | L3-A-010 §§9, 12–15 |
| L4-A-010-BH-005 | Where policy explicitly permits approval without a separate approver stage, the system shall recognize the governed Assignment Authority approval only after required validation and SoD evaluation succeed. | L3-A-010 §§9–10, 12, 15 |
| L4-A-010-BH-006 | When the governed approval succeeds, the system shall place the Role Assignment in `Approved` and publish `RoleAssignmentApproved`. | L3-A-010 §§9, 13–14 |
| L4-A-010-BH-007 | The system shall prevent an `Approved` Role Assignment from becoming effective before its approved effective date or while activation prerequisites are invalid. | L3-A-010 §§9–11, 13 |
| L4-A-010-BH-008 | When the effective date is reached and prerequisites remain valid, the system shall transition the Role Assignment `Approved → Active` and publish `RoleAssignmentActivated`. | L3-A-010 §§9, 13–16 |
| L4-A-010-BH-009 | The system shall make an `Active` Role Assignment available to later Role Resolution and Permission Resolution only within its approved scope and effective period. | L3-A-010 §§3–4, 9, 12, 16 |
| L4-A-010-BH-010 | The system shall not assign Permissions directly to the Person, modify Role definitions, create E02 organizational authority, or treat the active assignment as a final Authorization Decision. | L3-A-010 §§2–5, 12–16 |

## 5. Cross-Domain & Event Behavior

The system shall preserve the exact Role Assignment events and sole A04 Role Assignment publisher. `RoleAssignmentRejected` and `RoleAssignmentCancelled` shall be published only when their source-defined transitions occur. Source-domain eligibility evidence remains source-owned; E02 organizational responsibility does not become A04 organizational authority.

## 6. Failure / Rejection / Continuation

If identity/scope/Role/period validation, SoD, policy or required approval fails, the system shall prevent activation and apply only the source-defined `Rejected` or eligible `Cancelled` outcome. A prohibited SoD conflict shall not become `Active` unless an already-established policy explicitly permits a governed continuation path.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-010-BH-011 | The system shall preserve attributable evidence of subject, Role, source basis, scope, period, least-privilege/SoD evaluation, approval requirement and decision, and Role Assignment lifecycle events. | L3-A-010 §15.3 |

## 8. Observable Completion Conditions

Completion requires a canonical A02 Person to have an A04 Role Assignment in `Active`, with required policy/SoD/approval obligations satisfied and `RoleAssignmentActivated` published. Role/Permission definitions, organizational authority, Person identity and protected-operation Authorization Decisions remain outside the outcome.

## 9. Traceability & Provenance

**Parent:** L3-A-010 — Institutional Responsibility Assignment to Effective Authorized Access  
**Primary verification source:** L2-A04 locked Role Assignment amendment 2026-08-26

## 10. Open Issues / Deferred Design

No unresolved L4 correctness issue remains from the A010 remediation package. Technical authorization enforcement, claims, IAM/RBAC implementation and UI remain deferred.