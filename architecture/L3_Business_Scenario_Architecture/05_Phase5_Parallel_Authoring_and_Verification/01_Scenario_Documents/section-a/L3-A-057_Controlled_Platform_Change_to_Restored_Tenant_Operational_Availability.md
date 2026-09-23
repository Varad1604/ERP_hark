# Controlled Platform Change to Restored Tenant Operational Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-057  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Platform & Tenant  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; `L2-A01_Tenant_Controlled_Change_Authority_Amendment_2026-09-23.md`; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Every business domain dependent on Tenant operational state

---

# 2. Scenario Purpose

Govern an Active Tenant through controlled Maintenance or Upgrade restrictions, platform change execution, validation and return to operational availability while preserving data, configuration and audit history.

---

# 3. Business Outcome

The Tenant enters the governed Maintenance or Upgrade state with the applicable operation restrictions in force, the change completes under Platform control, and the Tenant returns to Active with data, configuration and audit history intact

The Locked L2-A01 controlled-change amendment defines the abstract Tenant Change Authority, controlled Maintenance/Upgrade operations and governed return-to-Active validation. It intentionally introduces no dedicated Maintenance/Upgrade event names.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A controlled maintenance activity or platform upgrade must be applied to an Active Tenant.

## 4.2 End Boundary

The intended change has completed and the Tenant has returned to `Active` after validation, with preserved state and audit history. Exact canonical execution cannot be specified from current L2-A01 contracts.

## 4.3 In Scope

- Tenant and change-scope validation.
- `Active → Maintenance → Active` or `Active → Upgrade → Active`.
- Policy-driven read/write/administrative restrictions.
- Change validation and restored context availability.
- Preservation of data, configuration, security and audit history.

## 4.4 Out of Scope

- Business-domain maintenance workflows.
- Software deployment, infrastructure topology or technical rollback.
- Tenant suspension, provisioning or archival.
- Inventing actors, commands, events or publishers.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Uses | L3-A-056 — Platform and Tenant Configuration | Supplies resolved configuration but not Tenant state transitions. |
| Related | Tenant suspension and archival scenarios | Distinct lifecycle purposes outside this controlled change. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Platform & Tenant / Section A | Exclusively owns Tenant lifecycle, restrictions, context and Platform change control. |
| Authorization & Policy / Section A | Supplies authorization for privileged actions. |
| Event Infrastructure / Section A | Would distribute canonical Tenant lifecycle events after definition. |
| Audit Infrastructure / Section A | Preserves immutable change and lifecycle evidence. |

---

# 6. Business Actors, Roles and Responsibilities

`Tenant Change Authority` is the canonical abstract authority for Maintenance/Upgrade entry and governed restoration. An Authorized Platform Actor may request/execute an approved change. Platform & Tenant retains sole Tenant lifecycle ownership.

---

# 7. Preconditions and Entry Criteria

- The Tenant exists and is `Active`.
- The change is classified as Maintenance or Upgrade.
- Applicable restriction and validation policies are known.
- Data, configuration, security and audit preservation obligations can be satisfied.
- Affected consumers can observe Tenant state through Platform Context.
- Tenant Change Authority approval and applicable Authorization & Policy enforcement govern protected lifecycle progression.

---

# 8. Trigger

A controlled maintenance activity or platform upgrade must be applied to an Active Tenant

---

# 9. Main Business Flow

| Step | Intended Business Behaviour | Responsible Domain | Intended Result |
|---|---|---|---|
| 1 | Receive and classify the controlled Platform change. | Platform & Tenant | Maintenance or Upgrade path is selected. |
| 2 | Resolve the Active Tenant, configuration, dependencies and affected operations. | Platform & Tenant | Change scope is established. |
| 3 | Validate authorization, preservation controls, restrictions and completion criteria. | Platform & Tenant | Change is eligible to begin. |
| 4 | After Tenant Change Authority approval, transition `Active → Maintenance` or `Active → Upgrade`. | Tenant Change Authority; Platform & Tenant | Tenant enters the governed restricted lifecycle state. |
| 5 | Enforce applicable read, write and administrative restrictions through Tenant Context. | Platform & Tenant | Invalid business execution is prevented. |
| 6 | Perform the controlled Platform change. | Platform & Tenant | Intended maintenance or upgrade work completes. |
| 7 | Validate data, configuration, security, services, context and audit integrity. | Platform & Tenant | Return-to-service eligibility is established. |
| 8 | After successful integrity validation and Tenant Change Authority acceptance, transition `Maintenance → Active` or `Upgrade → Active`. | Tenant Change Authority; Platform & Tenant | Tenant operational availability is restored. |
| 9 | Publish an Active Tenant Context to downstream domains. | Platform & Tenant | Business execution may resume. |
| 10 | Preserve change authority, restrictions, validations, transitions and correlation evidence. | Platform & Tenant; Audit Infrastructure | Intended outcome is auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Intended Path | Outcome |
|---|---|---|---|
| Read-only maintenance | Policy permits reads. | Restrict writes while allowing authorized reads. | Tenant remains in Maintenance. |
| Full restriction | Policy prohibits business operations. | Permit only governed Platform administration. | Downstream execution stops. |
| Validation fails | Change completes but readiness checks fail. | Keep Tenant outside Active and initiate governed recovery. | Operational availability is not asserted. |
| Change cancelled before transition | Approval is withdrawn. | Retain `Active`. | No Tenant lifecycle change. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Tenant is not Active. | Transition precondition fails. | Reject the controlled change path. | Platform & Tenant |
| Restriction policy is unavailable. | Safe execution cannot be assured. | Do not enter Maintenance or Upgrade. | Platform & Tenant |
| Change fails. | Tenant cannot return to normal operation. | Retain the governed non-Active state and begin authorized recovery; exact workflow is undefined. | Platform & Tenant |
| Integrity validation fails. | Return to Active is prohibited. | Preserve evidence and continue recovery. | Platform & Tenant |
| State transition publication is unavailable. | Consumers cannot receive a canonical fact. | Do not invent an event; use valid Platform Context and resolve upstream gap. | Platform & Tenant |

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|
| Platform exclusively owns Tenant lifecycle. | Downstream domains cannot change Tenant state. | L2-A01 §5.2 |
| Only listed transitions are valid. | Maintenance and Upgrade return only to Active. | L2-A01 §5.8 |
| Maintenance restrictions are policy-governed. | Reads, writes and administration follow applicable policy. | L2-A01 §5.7 |
| Business execution requires a valid Active Tenant Context. | Non-Active state prevents invalid operations. | L2-A01 §§4.8, 5.2 |
| Platform preserves Tenant data and configuration. | Controlled change cannot erase historical state. | L2-A01 Tenant lifecycle principles |
| Lifecycle changes are auditable. | Authority, restrictions and outcome remain traceable. | L2-A01 §5.3 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Intended Impact | Result |
|---|---|---|---|
| Tenant — Maintenance path | Active | `Active → Maintenance → Active`. | Restored operational availability. |
| Tenant — Upgrade path | Active | `Active → Upgrade → Active`. | Restored operational availability. |
| Platform Context | Active context | Reflect restricted state, then Active state. | Downstream execution is consistently gated. |
| Data / Configuration / Audit | Existing | Preserve through change. | Intact history. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Event Determination

L2-A01 performs no dedicated event cataloguing and defines no Tenant-entered-maintenance, upgrade-started, change-completed or Tenant-reactivated event or publisher. None is invented.

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|
| Tenant Context | Platform & Tenant | All domains | Current lifecycle state and execution eligibility. |
| Resolved configuration | Platform & Tenant | Platform change control | Applicable restrictions and validation rules. |
| Authorization decision | Authorization & Policy | Platform & Tenant | Permission for privileged transition. |
| Audit input | Platform & Tenant | Audit Infrastructure | Change and lifecycle evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

All lifecycle and Platform-change actions require privileged Tenant-scoped authorization.

## 15.2 Approval and Separation of Duty

Tenant Change Authority is the canonical abstract authority for maintenance/upgrade and reactivation. No universal named job title or universal separation-of-duty rule is imposed.

## 15.3 Audit Obligations

Record Tenant, change classification, authority, prior/resulting state, restriction policy, validations, preservation evidence, actor, timestamps and correlation.

---

# 16. Completion Criteria and Postconditions

The intended outcome is now fully governed by the Locked L2-A01 controlled-change amendment. Dedicated Maintenance/Upgrade event names are intentionally not required; authoritative Tenant lifecycle state and Tenant context remain the source of truth.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue | Required Resolution |
|---|---|---|---|
| A057-BLOCKER-01 — RESOLVED | Closed | Controlled change initiation, authorization, validation and restoration operations are defined by the L2-A01 amendment. | Closed. |
| A057-BLOCKER-02 — RESOLVED | Closed | Tenant Change Authority and Authorized Platform Actor boundaries are defined without inventing a universal job title. | Closed. |
| A057-BLOCKER-03 — RESOLVED | Closed | The amendment explicitly decides that no dedicated Maintenance/Upgrade event family is required; Tenant lifecycle state/context is authoritative. | Closed. |

No missing authority, transition or event is supplied by inference; the scenario consumes the explicit Locked L2-A01 amendment.


## 2026-09-23 Upstream Authority Reconciliation

L3-A-057 is reconciled to `L2-A01_Tenant_Controlled_Change_Authority_Amendment_2026-09-23.md`. The prior actor/authority and event-governance blockers are closed without introducing a new Tenant state or event family.
