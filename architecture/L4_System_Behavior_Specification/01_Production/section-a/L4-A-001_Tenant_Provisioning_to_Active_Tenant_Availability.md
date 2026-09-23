# Tenant Provisioning to Active Tenant Availability

## 1. Specification Metadata

**Specification ID:** L4-A-001  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Platform & Tenant  
**Realizes Scenario:** L3-A-001  
**Primary Business Owner:** Platform & Tenant  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** `L3-A-001_Tenant_Provisioning_to_Active_Tenant_Availability.md`; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `L2-A01_Tenant_Provisioning_Authority_and_Event_Amendment_2026-08-26.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall realize the approved Tenant provisioning scenario by admitting only an authorized and eligible Tenant request into the canonical Tenant lifecycle, preserving the Platform-owned Tenant boundary through provisioning, and making the Tenant available for permitted downstream setup only after the Tenant reaches `Active`.

The L4 boundary begins with an eligible Tenant Registration Request in `Requested` after the Platform Administrator's authoritative provisioning-entry decision and ends when the Tenant reaches `Active` and the Active Tenant context is available to permitted downstream consumers. Institution creation, later Tenant restriction/restoration, archival, and technical provisioning design remain outside this specification.

## 3. Required Business Context

- Tenant Registration Request, Subscription Plan, and Initial Configuration required by the parent scenario.
- Canonical Tenant lifecycle positions `Requested`, `Provisioning`, `Provisioning Failed`, and `Active`.
- Platform Administrator authorization to commence the `Requested → Provisioning` path.
- Platform & Tenant ownership of Tenant identity, lifecycle validation, lifecycle transitions, activation, and Tenant business facts/events.
- Canonical `TenantCreated` and `TenantActivated` event semantics and publisher ownership.
- Active Tenant context required by Institution & Structure for subsequent institution establishment.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-001-BH-001 | The system shall prevent entry into `Provisioning` unless the Tenant is in `Requested`, the applicable provisioning prerequisites are satisfied, and the Platform Administrator has recorded the authoritative authorization to proceed. | L3-A-001 §§4.1, 6–9, 12, 15 |
| L4-A-001-BH-002 | When authorized provisioning begins, the system shall perform only the canonical `Requested → Provisioning` progression and shall keep the Tenant unavailable for downstream business operations while it remains in `Provisioning`. | L3-A-001 §§4.3, 9, 12–13 |
| L4-A-001-BH-003 | During provisioning, the system shall establish the canonical Tenant environment and the Platform context required by the parent scenario while preserving the single Tenant isolation boundary. | L3-A-001 §§2–5, 9, 12 |
| L4-A-001-BH-004 | The system shall prevent `Provisioning → Active` unless the canonical Tenant activation conditions are satisfied; the provisioning-entry authorization shall not itself be treated as activation. | L3-A-001 §§6, 9, 12, 15 |
| L4-A-001-BH-005 | When activation conditions are satisfied, the system shall perform the canonical `Provisioning → Active` progression and make the Active Tenant context available for permitted downstream setup. | L3-A-001 §§3–4, 9, 13, 16 |
| L4-A-001-BH-006 | The system shall reject any Tenant lifecycle progression requested by this scenario that is not permitted by the canonical Tenant lifecycle, preserving the last valid lifecycle position. | L3-A-001 §§11–13 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-001-BH-007 | When the canonical Tenant creation fact is established, the system shall publish `TenantCreated` on behalf of the Platform & Tenant canonical publisher without assigning Tenant ownership to event infrastructure or any consumer. | L3-A-001 §§9, 14, 16; L2-A01 amendment §2 |
| L4-A-001-BH-008 | When the Tenant reaches `Active`, the system shall publish `TenantActivated` on behalf of the Platform & Tenant canonical publisher. | L3-A-001 §§9, 14, 16; L2-A01 amendment §2 |
| L4-A-001-BH-009 | After activation, the system shall make the authoritative Active Tenant context available to Institution & Structure for its separate downstream institution-establishment outcome without transferring Tenant lifecycle ownership. | L3-A-001 §§4.4, 14, 16; L2-A01 amendment §3 |

## 6. Failure / Rejection / Continuation

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-001-BH-010 | If the Tenant Registration Request is ineligible or lacks the required provisioning-entry authorization, the system shall prevent the request from entering the Tenant provisioning lifecycle. | L3-A-001 §11 |
| L4-A-001-BH-011 | If provisioning terminates unsuccessfully, the system shall place the Tenant in `Provisioning Failed` and shall not expose it as operationally available. | L3-A-001 §§11, 13 |
| L4-A-001-BH-012 | When the source-defined cause preventing continuation has been remediated, the system shall permit only the canonical `Provisioning Failed → Provisioning` continuation. | L3-A-001 §§11, 13 |
| L4-A-001-BH-013 | If any required lifecycle validation condition is unsatisfied, the system shall prevent further progression and preserve the current valid Tenant lifecycle position until source-defined conditions permit continuation. | L3-A-001 §11 |

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-001-BH-014 | The system shall preserve attributable business evidence of the Platform Administrator authorization used to admit the Tenant request into provisioning. | L3-A-001 §15.3 |
| L4-A-001-BH-015 | The system shall preserve auditable evidence of Tenant lifecycle progression performed by this scenario, including unsuccessful provisioning and governed re-entry where applicable. | L3-A-001 §§12, 15.3 |
| L4-A-001-BH-016 | The system shall preserve traceable evidence of applicable `TenantCreated` and `TenantActivated` publication as Platform & Tenant business facts. | L3-A-001 §15.3 |

## 8. Observable Completion Conditions

This specification is complete for a successful outcome when:

- the provisioning-entry authorization and prerequisites were valid;
- the Tenant progressed only through the canonical `Requested → Provisioning → Active` path;
- required Tenant and Platform context initialization completed within the Tenant boundary;
- `TenantCreated` and `TenantActivated` were published at their source-defined lifecycle points;
- the Active Tenant context is available for permitted downstream setup; and
- no Institution creation or other downstream business outcome has been absorbed into this specification.

A source-supported unsuccessful provisioning outcome remains observable as `Provisioning Failed` and is not treated as successful completion.

## 9. Traceability & Provenance

**Parent:** L3-A-001 — Tenant Provisioning to Active Tenant Availability  
**Primary verification source:** L2-A01 — Platform & Tenant Foundation  
**Authority/event verification source:** L2-A01 — Tenant Provisioning Authority and Event Amendment — 2026-08-26  
**Higher-level verification:** SutraERP L1 Business Domain Architecture — L1-01 Tenant Provisioning; Dependency Map Section A dependency direction

Every BH above traces to the Locked L3 parent. L2/L1 sources are used only to verify exact lifecycle, ownership, authority, event, and dependency semantics already represented by the parent.

## 10. Open Issues / Deferred Design

No L4 correctness issue remains. This specification is Architecture Approved under the R1A Tenant/Institution authority decision dated 2026-09-23.

Implementation decisions including provisioning mechanisms, APIs/interfaces, persistence, service/component design, event transport, technical access control, transaction/recovery mechanics, infrastructure/deployment, and UI/UX are deferred to L5/L6 as applicable.


## Canonical Validation Note

This specification passed the scoped R1A hard-fail validation against the Locked L4 Governance and Architecture Standard and L4 Validation Standard and is `Architecture Approved` under `L4_R1A_Tenant_Institution_Authority_Architecture_Approval_Decision_2026-09-23.md`. This approval does not Lock the specification and does not invent upstream actor authority, lifecycle semantics, events, or implementation design.
