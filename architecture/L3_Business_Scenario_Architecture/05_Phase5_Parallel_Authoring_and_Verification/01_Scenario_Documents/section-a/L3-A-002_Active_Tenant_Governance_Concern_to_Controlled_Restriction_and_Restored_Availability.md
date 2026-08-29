# Active Tenant Governance Concern to Controlled Restriction and Restored Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-002  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Platform & Tenant  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md` L1-01; `L2-A01_Platform_Tenant_Foundation_Architecture.md` Sections 5.2 and 5.4–5.14; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Temporarily restrict business execution within an Active Tenant when a Platform lifecycle governance concern requires suspension, while preserving the Tenant and its governed information.

The scenario also governs restoration of Tenant availability after the suspension condition is resolved, without transferring Tenant lifecycle ownership to downstream business domains.

---

# 3. Business Outcome

Business execution is suspended while Tenant data, configuration, security information, audit history and Platform metadata remain preserved, and the Tenant returns to `Active` when the governed suspension condition is resolved.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a Platform lifecycle decision requires business execution within an `Active` Tenant to be temporarily disabled.

## 4.2 End Boundary

The scenario ends when the governed suspension condition has been resolved, the Tenant has validly returned to `Active`, and permitted business execution is restored.

## 4.3 In Scope

- Validation of the `Active` to `Suspended` lifecycle transition.
- Controlled transition of the Tenant to `Suspended`.
- Temporary disablement of business execution.
- Preservation of Tenant data, configuration, audit history, security information and Platform metadata.
- Continued Platform responsibility for the suspended Tenant.
- Determination that the governed suspension condition has been resolved.
- Validation of the `Suspended` to `Active` transition.
- Restoration of the Tenant to `Active`.

## 4.4 Out of Scope

- Tenant provisioning and initial activation.
- Controlled maintenance.
- Platform upgrades.
- Tenant archival or permanent operational retirement.
- Resolution of business-domain lifecycles affected by temporary Tenant unavailability.
- Technical access-control, infrastructure or recovery mechanisms.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-A-001 — Tenant Provisioning to Active Tenant Availability | Establishes the Active Tenant required by this scenario. |
| Related | L3-A-003 — Active Tenant to Governed Archival | Governs permanent operational retirement rather than temporary restriction. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform & Tenant | Owns the Tenant lifecycle, lifecycle decisions, transition validation, controlled suspension, preservation obligations and restoration to Active. |

All participating responsibilities remain within Section A.

Downstream business domains depend on the Tenant lifecycle state but do not participate as owners and cannot modify the Tenant lifecycle.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Platform Administrator | Participates in Platform-owned Tenant lifecycle administration. | Primary Platform actor identified by locked L1; specific authority to decide suspension or restoration is not defined upstream. |

The owning domain remains responsible for the scenario behaviour. No approval or decision authority beyond the authoritative sources is inferred.

---

# 7. Preconditions and Entry Criteria

- The Tenant exists and is in the canonical `Active` state.
- The Tenant identity remains valid and uniquely identifiable.
- Business execution is currently permitted within the Tenant.
- A Platform lifecycle governance concern has been identified.
- A Platform lifecycle decision requires temporary disablement of business execution.
- The Tenant has not entered the terminal `Archived` state.

---

# 8. Trigger

A Platform lifecycle decision requires business execution within an `Active` Tenant to be temporarily disabled.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---:|---|---|---|
| 1 | Establish that the Platform lifecycle decision requires temporary suspension of the Active Tenant. | Platform & Tenant | The Tenant is identified for controlled restriction. |
| 2 | Validate the Tenant identity, the requested `Active` to `Suspended` transition, Platform integrity, required Platform Services, lifecycle policies and governance requirements. | Platform & Tenant | The Tenant is eligible for the permitted suspension transition. |
| 3 | Perform the canonical `Active` to `Suspended` lifecycle transition. | Platform & Tenant | The Tenant enters `Suspended` and normal business execution is disabled. |
| 4 | Preserve Tenant data, configuration, audit history, security information and Platform metadata throughout the suspension. | Platform & Tenant | The Tenant remains intact and governed while unavailable for normal business execution. |
| 5 | Maintain the Tenant in `Suspended` until the governed suspension condition is resolved. | Platform & Tenant | Business execution remains disabled while the condition remains unresolved. |
| 6 | After resolution of the suspension condition, validate the permitted `Suspended` to `Active` transition and applicable lifecycle requirements. | Platform & Tenant | The Tenant becomes eligible for restoration. |
| 7 | Perform the canonical `Suspended` to `Active` transition. | Platform & Tenant | The Tenant becomes fully operational and permitted business execution is restored. |

The Main Business Flow ends when the Tenant returns to `Active` and availability is restored.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

The `Maintenance` and `Upgrade` lifecycle paths are not alternate flows for this scenario because its frozen trigger and outcome specifically govern temporary suspension of business execution.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The requested `Active` to `Suspended` transition is invalid. | The Tenant cannot enter `Suspended` through the requested progression. | Reject the invalid transition and preserve the current Tenant lifecycle state. | Platform & Tenant |
| The governed suspension condition remains unresolved. | The Tenant remains `Suspended`, and business execution remains disabled. | Continue preserving the Tenant until the authoritative suspension condition is resolved. | Platform & Tenant |
| Restoration validation conditions are not satisfied. | The Tenant cannot validly return to `Active`. | Preserve the `Suspended` state until the applicable lifecycle and governance requirements are satisfied. | Platform & Tenant |
| The requested `Suspended` to `Active` transition is invalid. | Tenant availability cannot be restored through the requested progression. | Reject the invalid transition and retain the Tenant in `Suspended`. | Platform & Tenant |

No technical retry, rollback, server-failure or infrastructure-recovery behaviour is defined at L3.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| The Platform exclusively owns the lifecycle of every Tenant. | Platform & Tenant | No downstream domain may suspend, restore or otherwise modify the Tenant lifecycle. | L2-A01 §5.2 |
| Only the Platform may perform Tenant lifecycle transitions. | Platform & Tenant | Suspension and restoration remain Platform-owned actions. | L2-A01 §5.13 |
| `Active` to `Suspended` is a permitted Tenant lifecycle transition. | Platform & Tenant | An Active Tenant may enter controlled suspension. | L2-A01 §5.8 |
| `Suspended` to `Active` is a permitted Tenant lifecycle transition. | Platform & Tenant | A suspended Tenant may return to operational availability. | L2-A01 §5.8 |
| Any transition not listed in the canonical Tenant lifecycle is invalid. | Platform & Tenant | No additional restriction or restoration state may be introduced by this scenario. | L2-A01 §5.8 |
| Business execution is disabled while the Tenant is `Suspended`. | Platform & Tenant | Downstream business operations cannot proceed during suspension. | L2-A01 §§5.5 and 5.7 |
| Suspended Tenants shall preserve all business data. | Platform & Tenant | Suspension cannot delete or discard Tenant business data. | L2-A01 §5.13 |
| Tenant configuration, audit history, security information and Platform metadata remain preserved during suspension. | Platform & Tenant | Controlled restriction does not destroy the governed Tenant context. | L2-A01 §5.7 |
| Tenant lifecycle transitions shall be auditable. | Platform & Tenant | Suspension and restoration must remain traceable. | L2-A01 §5.13 |
| Business domains shall treat the Tenant lifecycle as read-only. | Platform & Tenant | Downstream consumption cannot transfer or duplicate lifecycle ownership. | L2-A01 §5.13 |
| Tenant isolation shall be preserved across all Platform capabilities. | Platform & Tenant | Suspension and restoration remain within the same Tenant boundary. | L2-A01 §§2.6 and 2.14 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Tenant | `Active` | Governed temporary restriction of business execution | `Suspended` | Platform & Tenant | L2-A01 §§5.4, 5.5 and 5.8 |
| Tenant | `Suspended` | Preservation of the Tenant while business execution remains disabled | Remains `Suspended` until the governed condition is resolved | Platform & Tenant | L2-A01 §5.7 |
| Tenant | `Suspended` | Governed restoration after resolution of the suspension condition | `Active` | Platform & Tenant | L2-A01 §§5.4, 5.5 and 5.8 |

No other Tenant lifecycle state or transition is introduced by this scenario.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No exact canonical business event for Tenant suspension or restored availability is defined by the authoritative L1/L2 sources reviewed.

This scenario does not invent events such as `TenantSuspended`, `TenantRestored` or `TenantReactivated`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No scenario-specific synchronous business contract or governed hand-off is defined by the authoritative L2 architecture.

Downstream business domains consume the Tenant lifecycle state as a read-only foundational dependency:

- `Suspended` means normal business execution is disabled.
- `Active` means permitted business execution may resume.

This dependency does not make downstream domains participating owners and does not transfer Tenant lifecycle ownership.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Decide that an Active Tenant requires temporary suspension | Canonical business role not defined upstream | A Platform lifecycle decision must exist; no actor-specific authority is inferred. | Platform & Tenant |
| Perform the `Active` to `Suspended` transition | Canonical business role not defined in L2-A01 | Only the Platform may perform a permitted Tenant lifecycle transition. | Platform & Tenant |
| Determine that the governed suspension condition has been resolved | Canonical business role not defined upstream | Restoration cannot begin until the suspension condition is resolved. | Platform & Tenant |
| Perform the `Suspended` to `Active` transition | Canonical business role not defined in L2-A01 | Only the Platform may perform a permitted Tenant lifecycle transition. | Platform & Tenant |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception requirement is defined by the authoritative L1/L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Tenant suspension and restoration transitions must remain auditable. | Auditable Tenant lifecycle history covering the `Active` to `Suspended` and `Suspended` to `Active` transitions. | Platform & Tenant |
| Tenant audit history must remain preserved during suspension. | Preserved Tenant audit history throughout the restricted period. | Platform & Tenant |

No evidence fields, technical audit mechanism or additional audit artefact is defined at L3.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The governed suspension condition has been resolved.
- Tenant data, configuration, security information, audit history and Platform metadata remain preserved.
- The permitted `Suspended` to `Active` transition has been validated.
- The Tenant has returned to the canonical `Active` state.
- Permitted business execution within the Tenant has been restored.
- The suspension and restoration lifecycle transitions remain auditable.

## 16.2 Postconditions

- The Tenant is fully operational in the canonical `Active` state.
- The Tenant retains its identity, isolation boundary, configuration and preserved history.
- Downstream domains may resume permitted execution within the resolved Tenant context.
- Tenant lifecycle ownership remains exclusively with Platform & Tenant.
- No downstream lifecycle or business-domain ownership has been changed.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The authoritative sources do not define the canonical business actor or role authorized to decide Tenant suspension, determine resolution of the suspension condition, or authorize restoration to `Active`. | Platform ownership, lifecycle states, transitions and the final outcome are clear, so the Draft is usable. Actor-level business authority cannot be finalized for approval or later access-control specification. | Platform & Tenant architecture ownership must define or confirm the canonical business role and authority for suspension and restoration decisions. |

This issue is non-blocking for this Draft but must be resolved before the scenario can become `Locked`.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.