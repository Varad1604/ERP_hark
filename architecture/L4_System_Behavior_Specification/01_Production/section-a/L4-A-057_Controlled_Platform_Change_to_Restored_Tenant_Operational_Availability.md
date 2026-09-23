# Controlled Platform Change to Restored Tenant Operational Availability

## 1. Specification Metadata

**Specification ID:** L4-A-057  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Platform & Tenant  
**Realizes Scenario:** L3-A-057  
**Primary Business Owner:** Platform & Tenant  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** `L3-A-057_Controlled_Platform_Change_to_Restored_Tenant_Operational_Availability.md`; L2-A01 — Platform & Tenant Foundation; `L2-A01_Tenant_Controlled_Change_Authority_Amendment_2026-09-23.md`; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an Active Tenant through a source-permitted Maintenance or Upgrade restriction and return it to Active only after the controlled platform change has completed and the required integrity validation has succeeded, while preserving data, configuration and audit history.

The behavior begins when a controlled maintenance activity or platform upgrade must be applied to an Active Tenant. It ends when the Tenant is again `Active` with the governed restriction removed and preserved history demonstrating the change outcome. Technical deployment, rollback and infrastructure mechanics are outside this L4 boundary.

## 3. Required Business Context

- Exactly one Tenant is resolved and is eligible for the intended source-defined transition.
- The Tenant begins in `Active`.
- A governed maintenance or upgrade condition exists.
- Applicable operation restrictions can be determined from the Tenant lifecycle semantics.
- Required post-change integrity validation can be evaluated.
- Tenant Change Authority and Authorized Platform Actor responsibilities are available from the Locked L2-A01 controlled-change amendment.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-057-BH-001 | The system shall reject the controlled-change path when the Tenant cannot be resolved, is not `Active`, or the requested lifecycle transition is not source-permitted. | L3-A-057 §§4, 7, 9, 11–13 |
| L4-A-057-BH-002 | For a governed maintenance condition, the system shall place the eligible Tenant in `Maintenance` and apply the source-defined restrictions associated with that lifecycle position. | L3-A-057 §§2–4, 9, 12–13 |
| L4-A-057-BH-003 | For a governed upgrade condition, the system shall place the eligible Tenant in `Upgrade` and apply the source-defined restrictions associated with that lifecycle position. | L3-A-057 §§2–4, 9, 12–13 |
| L4-A-057-BH-004 | While the Tenant is in `Maintenance` or `Upgrade`, the system shall enforce the corresponding operational restriction through governed Tenant context and shall not represent the Tenant as normally available. | L3-A-057 §§2–4, 9, 12–13 |
| L4-A-057-BH-005 | The system shall preserve Tenant data, configuration context and completed governance history throughout the controlled change and shall not treat the lifecycle restriction as authority to rewrite business-domain records. | L3-A-057 §§2–5, 9, 12, 16 |
| L4-A-057-BH-006 | The system shall prevent `Maintenance → Active` or `Upgrade → Active` completion until the required post-change integrity validation has succeeded. | L3-A-057 §§3–4, 9, 11–13, 16 |
| L4-A-057-BH-007 | When the controlled change and required validation have succeeded, the system shall return the Tenant to `Active` and remove the temporary restriction for future eligible operations. | L3-A-057 §§3–4, 9, 13, 16 |
| L4-A-057-BH-008 | A failed or unresolved change shall not be represented as restored operational availability merely because the change window has ended. | L3-A-057 §§9, 11, 16 |

## 5. Cross-Domain & Event Behavior

Locked L2-A01 defines the relevant Tenant lifecycle states and restrictions but no canonical maintenance/upgrade commands, lifecycle events or sole publishers for this scenario. The system shall not invent maintenance-started, upgrade-completed or Tenant-restored events.

Business domains consume current Tenant operational context but retain ownership of their own records and workflows.

## 6. Failure / Rejection / Continuation

If transition eligibility, authorization, change completion or post-change integrity cannot be established, the system shall keep the Tenant in its current governed lifecycle position and shall not assert restored `Active` availability. No technical rollback algorithm, deployment recovery, infrastructure procedure or retry behavior is specified here.

## 7. Audit & Evidence Behavior

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-057-BH-009 | The system shall preserve evidence of the Tenant, governed change basis, prior and resulting lifecycle positions, applied restriction, validation outcome, available authority context and final operational-availability result. | L3-A-057 §§12, 15–16 |

## 8. Observable Completion Conditions

Completion requires a source-permitted `Active → Maintenance → Active` or `Active → Upgrade → Active` progression, enforcement of the applicable temporary restriction, successful integrity validation, Tenant Change Authority acceptance before restoration, and preserved Tenant data/configuration/audit history. Dedicated Maintenance/Upgrade event publication is intentionally not required.

## 9. Traceability & Provenance

**Parent:** L3-A-057 — Controlled Platform Change to Restored Tenant Operational Availability  
**Primary verification sources:** L2-A01 — Platform & Tenant Foundation; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows

This specification describes only observable business-system behavior. It does not define deployment architecture, maintenance tooling, infrastructure sequencing, technical rollback or implementation-specific availability mechanisms.

## 10. Open Issues / Deferred Design

The authority/operation gap is resolved by the Locked L2-A01 controlled-change amendment. No dedicated Maintenance/Upgrade event names are canonical; technical execution/deployment mechanisms remain downstream.


## 2026-09-23 Architecture Approval

Promoted to Architecture Approved after reconciliation with L3-A-057 v1.1 and the Locked L2-A01 Controlled Tenant Change Authority amendment. Tenant lifecycle ownership remains exclusively with Platform & Tenant.
