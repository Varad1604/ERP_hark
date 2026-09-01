# Responsibility Delegation to Automatic Authority Reversion

## 1. Specification Metadata

**Specification ID:** L4-A-011  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Authorization & Policy  
**Realizes Scenario:** L3-A-011  
**Primary Business Owner:** Authorization & Policy  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-A-011_Responsibility_Delegation_to_Automatic_Authority_Reversion.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A07_Audit_Infrastructure_Workflows.md`; L2-E02 — Position & Assignment Workflows (boundary verification only)  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an A04 Authorization Delegation so approved responsibility contributes additional authorization context only within its permitted scope and period, then ceases automatically on expiry or earlier governed revocation without changing Role Assignments, Roles, Permissions, or E02 Assignment Delegation.

## 3. Required Business Context

- One Tenant and canonical delegator/delegate identity references.
- An existing authorization-layer responsibility eligible for delegation.
- Delegated scope, effective date and expiry date.
- Applicable Delegation, least-privilege, Authorization and SoD policies.
- Required approval outcome.
- Canonical Delegation lifecycle: `Requested → Approved → Active`, then `Expired` or `Revoked`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-011-BH-001 | The system shall prevent Delegation creation when Tenant, delegator, delegate, delegated responsibility, scope, effective date or expiry date cannot be resolved. | L3-A-011 §§7, 9, 11–12 |
| L4-A-011-BH-002 | Before approval, the system shall evaluate source-defined Delegation policy, least privilege and applicable SoD constraints. | L3-A-011 §§9, 11–12 |
| L4-A-011-BH-003 | A valid request shall begin in `Requested`, and the system shall prevent `Requested → Approved` until the required approval outcome exists. | L3-A-011 §§9, 13, 15 |
| L4-A-011-BH-004 | The system shall prevent an `Approved` Delegation from contributing authority before its permitted effective date. | L3-A-011 §§9–10, 13 |
| L4-A-011-BH-005 | When activation conditions are satisfied, the system shall transition `Approved → Active` and make only the approved delegated scope available to subsequent authorization evaluation. | L3-A-011 §§9, 12–13 |
| L4-A-011-BH-006 | While the Delegation is `Active`, the system shall continue to apply Authorization Policy, SoD and Explicit Deny rather than treating delegation as an automatic allow decision. | L3-A-011 §§9, 12 |
| L4-A-011-BH-007 | At the governed expiry condition, the system shall transition `Active → Expired` and exclude the Delegation from subsequent authorization evaluation. | L3-A-011 §§3, 9–13 |
| L4-A-011-BH-008 | When governed early revocation occurs, the system shall transition `Active → Revoked` and exclude the Delegation from subsequent authorization evaluation. | L3-A-011 §§9–13 |
| L4-A-011-BH-009 | The system shall not modify permanent Role Assignments, Roles, Permissions or E02 organizational assignment authority as a result of A011. | L3-A-011 §§2, 4–5, 12–13 |

## 5. Cross-Domain & Event Behavior

L2-A04 provides auditable activity labels for Delegation creation, activation and revocation but does not establish canonical Delegation event contracts or publishers, and defines no canonical expiry event. The system shall therefore not invent event names. Any later event publication must preserve A04 ownership and A06 event-governance rules.

## 6. Failure / Rejection / Continuation

Invalid Tenant/identity context, scope, duration, policy, SoD or missing approval shall keep the Delegation outside `Active`. If terminal event semantics or exact approval/revocation authority are unresolved, the system shall still preserve the source-defined lifecycle outcome but shall not invent authority or event contracts.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-011-BH-010 | The system shall preserve attributable evidence of delegator, delegate, delegated scope, period, policy/SoD evaluation, approval, activation, use, expiry or revocation, and the resulting return to non-delegated authorization context. | L3-A-011 §§15–16 |

## 8. Observable Completion Conditions

Completion requires the Delegation to have reached `Expired` or `Revoked`, to be excluded from later authorization evaluation, and to leave permanent authorization definitions and organizational assignments unchanged, with the full temporary-authority history auditable.

## 9. Traceability & Provenance

**Parent:** L3-A-011 — Responsibility Delegation to Automatic Authority Reversion  
**Primary verification source:** L2-A04 — Authorization & Policy Workflows  
**Additional verification:** L2-A07 — Audit Infrastructure; L2-E02 boundary evidence identified by the parent

## 10. Open Issues / Deferred Design

The Locked parent leaves the canonical Delegation approver, activation/revocation authority, lifecycle command contracts, and canonical Delegation event names/publishers undefined. It also preserves the distinction between A04 Authorization Delegation and E02 Assignment Delegation. No local resolution is introduced.
