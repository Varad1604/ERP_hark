# Active Tenant to Governed Archival

## 1. Specification Metadata

**Specification ID:** L4-A-003  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Platform & Tenant  
**Realizes Scenario:** L3-A-003  
**Primary Business Owner:** Platform & Tenant  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** `L3-A-003_Active_Tenant_to_Governed_Archival.md`; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall retire a Tenant only through the canonical `Active → Archived` transition, preserve required historical Tenant information and audit history, and prevent normal business execution after archival. The boundary begins with an approved permanent operational-retirement decision for an `Active` Tenant and ends when the Tenant is terminally `Archived`, immutable and read-only.

## 3. Required Business Context

- An `Active` Tenant.
- An approved permanent operational-retirement decision; the canonical approving role is not defined upstream.
- Canonical `Active → Archived` transition and terminal `Archived` state.
- Applicable retention governance for required historical information.
- Platform & Tenant ownership of Tenant lifecycle transitions.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-003-BH-001 | The system shall prevent archival unless the Tenant is `Active`, an approved permanent-retirement decision exists, and the canonical archival conditions are satisfied. | L3-A-003 §§7, 9, 12–16 |
| L4-A-003-BH-002 | When archival conditions are satisfied, the system shall perform only the canonical `Active → Archived` transition. | L3-A-003 §§9, 12–13 |
| L4-A-003-BH-003 | After the Tenant reaches `Archived`, the system shall prevent normal business execution and normal modification of the Tenant. | L3-A-003 §§12–13, 16 |
| L4-A-003-BH-004 | The system shall not permit any unsupported transition from `Archived` back to an operational Tenant state. | L3-A-003 §§11–13, 16 |
| L4-A-003-BH-005 | The system shall preserve required Tenant historical information and audit history after archival according to applicable retention governance. | L3-A-003 §§12, 15–16 |
| L4-A-003-BH-006 | The system shall preserve the canonical Tenant identity rather than treating archival as Tenant deletion or recreation. | L3-A-003 §§12, 16 |

## 5. Cross-Domain & Event Behavior

No exact canonical Tenant archival event is defined by the approved parent or its authoritative L1/L2 sources. The system shall not invent or publish `TenantArchived`, `TenantRetired`, `TenantDeactivated`, or an equivalent event under this specification. Downstream domains may consume the canonical `Archived` lifecycle fact as read-only context without gaining Tenant lifecycle ownership.

## 6. Failure / Rejection / Continuation

If the retirement decision, lifecycle state, or archival conditions are invalid, the system shall prevent archival and preserve the last valid Tenant lifecycle position. Once `Archived`, no normal restoration path is permitted by this scenario.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-003-BH-007 | The system shall preserve attributable evidence of the approved permanent-retirement decision and the resulting `Active → Archived` lifecycle transition. | L3-A-003 §15 |

## 8. Observable Completion Conditions

Completion requires the Tenant to be `Archived`, read-only, unavailable for normal business execution, historically preserved according to retention governance, and not deleted or restored through an unsupported transition.

## 9. Traceability & Provenance

**Parent:** L3-A-003 — Active Tenant to Governed Archival  
**Primary verification source:** L2-A01 — Platform & Tenant Foundation  
**Higher-level verification:** SutraERP L1 Business Domain Architecture; Dependency Map

## 10. Open Issues / Deferred Design

The authoritative architecture requires approval for permanent Tenant retirement but does not identify the canonical approving business role. This specification therefore requires the approved decision without assigning unsupported authority. The bounded absence of a named approving role does not prevent approval of the observable system behavior because the specification does not invent that role. Technical archival, persistence and access-control mechanisms remain deferred to L5/L6.

## Canonical Validation Note

This specification passed the scoped R1A hard-fail validation against the Locked L4 Governance and Architecture Standard and L4 Validation Standard and is `Architecture Approved` under `L4_R1A_Tenant_Institution_Authority_Architecture_Approval_Decision_2026-09-23.md`. This approval does not Lock the specification and does not invent upstream actor authority, lifecycle semantics, events, or implementation design.
