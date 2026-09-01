# Active Tenant Governance Concern to Controlled Restriction and Restored Availability

## 1. Specification Metadata

**Specification ID:** L4-A-002  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Platform & Tenant  
**Realizes Scenario:** L3-A-002  
**Primary Business Owner:** Platform & Tenant  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-A-002_Active_Tenant_Governance_Concern_to_Controlled_Restriction_and_Restored_Availability.md`; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall restrict normal business execution for an `Active` Tenant only through the canonical Tenant suspension lifecycle, preserve the Tenant and its governed history while restricted, and restore permitted availability only after the authoritative suspension condition has been resolved.

The boundary begins with an `Active` Tenant for which a governed suspension decision exists and ends when the Tenant is either validly retained in `Suspended` or restored through `Suspended → Active`. Tenant archival and downstream business-domain recovery are outside this specification.

## 3. Required Business Context

- Canonical Tenant lifecycle states `Active` and `Suspended`.
- A source-authoritative decision that suspension is required or that the suspension condition has been resolved; L4 does not assign the decision to an unsupported actor.
- Tenant isolation, identity, configuration, security, audit and historical context that must remain preserved during restriction.
- Platform & Tenant ownership of all Tenant lifecycle transitions.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-002-BH-001 | The system shall prevent suspension unless the Tenant is `Active`, the suspension decision is authoritative, and `Active → Suspended` is permitted by the canonical Tenant lifecycle. | L3-A-002 §§4, 9, 12–13 |
| L4-A-002-BH-002 | When valid suspension conditions are satisfied, the system shall transition the Tenant from `Active` to `Suspended`. | L3-A-002 §§3, 9, 13 |
| L4-A-002-BH-003 | While the Tenant is `Suspended`, the system shall prevent normal business execution for that Tenant. | L3-A-002 §§2–4, 9, 12 |
| L4-A-002-BH-004 | While suspended, the system shall preserve the Tenant identity, isolation boundary, configuration, security context, audit history and governed institutional data without treating suspension as archival. | L3-A-002 §§2–4, 9, 12, 16 |
| L4-A-002-BH-005 | The system shall prevent restoration while the authoritative suspension condition remains unresolved. | L3-A-002 §§9, 11–13 |
| L4-A-002-BH-006 | When the authoritative suspension condition is resolved and lifecycle validation succeeds, the system shall transition the Tenant from `Suspended` to `Active` and restore permitted business availability. | L3-A-002 §§3–4, 9, 13, 16 |
| L4-A-002-BH-007 | The system shall reject any suspension or restoration progression not permitted by the canonical Tenant lifecycle and preserve the last valid Tenant state. | L3-A-002 §§11–13 |

## 5. Cross-Domain & Event Behavior

No canonical Tenant suspension or restoration business event is defined by the approved parent. The system shall not invent or publish a `TenantSuspended`, `TenantRestored`, `TenantReactivated`, or equivalent event for this specification.

## 6. Failure / Rejection / Continuation

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-002-BH-008 | If the suspension decision, lifecycle entry state or required validation is invalid, the system shall prevent `Active → Suspended` and retain `Active`. | L3-A-002 §11 |
| L4-A-002-BH-009 | If restoration conditions are not satisfied, the system shall retain `Suspended` and continue to prevent normal business execution. | L3-A-002 §11 |

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-002-BH-010 | The system shall preserve attributable evidence of the governed suspension and restoration decisions and the resulting `Active → Suspended → Active` lifecycle history where those transitions occur. | L3-A-002 §§12, 15–16 |

## 8. Observable Completion Conditions

A suspension outcome is complete when the Tenant is `Suspended`, normal business execution is unavailable, and required Tenant history/context remains preserved. A restoration outcome is complete when the governing suspension condition is resolved, the Tenant returns to `Active`, and permitted business availability is restored. No archival outcome or downstream-domain state change is absorbed.

## 9. Traceability & Provenance

**Parent:** L3-A-002 — Active Tenant Governance Concern to Controlled Restriction and Restored Availability  
**Primary verification source:** L2-A01 — Platform & Tenant Foundation  
**Higher-level verification:** SutraERP L1 Business Domain Architecture; Dependency Map

Every BH is derived from the Locked L3 parent. Higher-level sources verify lifecycle ownership and Tenant-boundary semantics only.

## 10. Open Issues / Deferred Design

The upstream architecture does not name the specific business actor authorized to decide Tenant suspension or restoration. This draft therefore requires an authoritative source decision without assigning that authority to a role. No additional L4 correctness issue is introduced.

Implementation mechanisms for restricting execution, persistence, technical access enforcement, infrastructure and UI/UX remain deferred to L5/L6.