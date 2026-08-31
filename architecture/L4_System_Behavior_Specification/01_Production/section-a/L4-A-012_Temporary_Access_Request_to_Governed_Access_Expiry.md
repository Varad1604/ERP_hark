# Temporary Access Request to Governed Access Expiry

## 1. Specification Metadata

**Specification ID:** L4-A-012  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Authorization & Policy  
**Realizes Scenario:** L3-A-012  
**Primary Business Owner:** Authorization & Policy  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-A-012_Temporary_Access_Request_to_Governed_Access_Expiry.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A07_Audit_Infrastructure_Workflows.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern exceptional, least-privilege Temporary Access so approved Permissions contribute to authorization evaluation only for the approved scope and period and then cease automatically, without permanently changing Role Assignments, Roles or Permission definitions.

## 3. Required Business Context

- One resolved Tenant and canonical Requestor/access-subject identity context.
- Exceptional operational requirement and business justification.
- Requested Permissions, scope, effective date and expiry date.
- Applicable Temporary Access, least-privilege, Authorization and SoD policies.
- Required approval and approval reference.
- Canonical Temporary Access lifecycle: `Requested → Approved → Granted`, then `Expired` or `Revoked`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-012-BH-001 | The system shall prevent Temporary Access progression unless Tenant, identity context, business justification, requested Permissions, scope and effective period are resolvable. | L3-A-012 §§7, 9, 11–12 |
| L4-A-012-BH-002 | Before approval, the system shall evaluate source-defined Temporary Access policy, least privilege, duration, scope and applicable SoD constraints. | L3-A-012 §§9, 11–12 |
| L4-A-012-BH-003 | A valid Temporary Access request shall enter `Requested`, and the system shall prevent `Requested → Approved` until the required approval outcome exists. | L3-A-012 §§9, 13, 15 |
| L4-A-012-BH-004 | The system shall prevent `Approved → Granted` before the permitted effective date or when grant prerequisites no longer hold. | L3-A-012 §§9–10, 13 |
| L4-A-012-BH-005 | When the grant becomes effective, the system shall transition `Approved → Granted` and make only the approved Permissions and scope available as additional authorization context. | L3-A-012 §§9, 12–13 |
| L4-A-012-BH-006 | A `Granted` Temporary Access Grant shall remain subject to Authorization Policy, SoD and Explicit Deny and shall not itself guarantee a protected-operation allow decision. | L3-A-012 §§9, 12 |
| L4-A-012-BH-007 | At the governed expiry condition, the system shall transition `Granted → Expired` and exclude the Grant from subsequent authorization evaluation. | L3-A-012 §§3, 9, 12–13 |
| L4-A-012-BH-008 | When governed early revocation occurs, the system shall transition `Granted → Revoked` and exclude the Grant from subsequent authorization evaluation. | L3-A-012 §§9–13 |
| L4-A-012-BH-009 | The system shall require a new approval process rather than silently extending an existing Temporary Access Grant. | L3-A-012 §§10, 12 |
| L4-A-012-BH-010 | The system shall not permanently modify Role Assignments, Roles or Permission definitions through A012. | L3-A-012 §§2, 4, 12–13 |

## 5. Cross-Domain & Event Behavior

`Temporary Access Granted` and `Temporary Access Expired` are upstream auditable activity labels, not canonical event contracts. No request, approval, grant, expiry or revocation event name or publisher is invented. Later event definitions must preserve A04 ownership and A06 governance.

## 6. Failure / Rejection / Continuation

Missing identity context, justification, invalid scope/duration, policy denial, prohibited SoD conflict or missing approval shall prevent the Grant from becoming `Granted`. Incomplete activation shall remain outside `Granted`; no unsupported recovery workflow is authored.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-012-BH-011 | The system shall preserve attributable evidence of Requestor, access subject where distinct, justification, Permissions, scope, period, policy/SoD evaluation, approval reference, grant, use, expiry or revocation, and final access cessation. | L3-A-012 §§15–16 |

## 8. Observable Completion Conditions

Completion requires the Temporary Access Grant to reach `Expired` or governed `Revoked`, cease contributing Permissions to later authorization evaluation, leave permanent Role Assignments unchanged, and retain complete approval and access history.

## 9. Traceability & Provenance

**Parent:** L3-A-012 — Temporary Access Request to Governed Access Expiry  
**Primary verification source:** L2-A04 — Authorization & Policy Workflows  
**Additional verification:** L2-A07 — Audit Infrastructure

## 10. Open Issues / Deferred Design

The parent leaves the canonical approver role/eligibility, separate access-subject definition, grant/revocation authorities, lifecycle command contracts, and canonical Temporary Access events/publishers undefined. This Draft does not infer them.
