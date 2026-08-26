# Maintenance Completion to Infrastructure Condition Reconciliation

# 1. Scenario Metadata

**Scenario ID:** L3-G-307  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Reconcile the authoritative current Infrastructure condition after the maintenance execution owner completes authorized work, without transferring Work Order or maintenance-history ownership to G10.

# 3. Business Outcome

G10 reconciles the resulting infrastructure state without taking ownership of maintenance execution.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a consuming maintenance domain completes authorised work affecting infrastructure condition.

## 4.2 End Boundary

The scenario ends when G10 has resolved the affected canonical Infrastructure, reconciled only the G10 condition/health/lifecycle facts supported by the maintenance completion evidence, and published any exact source-defined G10 event corresponding to a fact that actually changed.

## 4.3 In Scope

- Consumption of source-defined G01 maintenance-completion facts.
- Resolution of canonical G10 Infrastructure.
- Reconciliation of G10 current health/condition.
- Valid `Under Maintenance → Operational` reconciliation where source-supported.
- Aggregate-specific health events where applicable.

## 4.4 Out of Scope

- G01 Work Order completion, verification, closure or maintenance-history mutation.
- Re-performing maintenance.
- A locally invented generic “condition reconciled” event.
- Automatic availability/compliance changes that require separate governance.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns physical Infrastructure and current authoritative Infrastructure condition/health/lifecycle facts. |
| Facilities Management / G01 | Owns Work Orders, maintenance execution, completion evidence, verification, closure and maintenance history. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Coordinator | Coordinates G10 reconciliation against the affected Infrastructure. | Does not alter G01 execution history. |
| Infrastructure Administrator | Applies source-supported G10 condition/health/lifecycle reconciliation. | Must remain within G10 rules. |
| Facilities Manager | Owns the source maintenance completion context. | Supplies completed maintenance facts without modifying G10 directly. |

# 7. Preconditions and Entry Criteria

- A source-defined maintenance completion fact exists from G01 or the authoritative maintenance execution owner.
- Completed work references canonical physical Infrastructure.
- Completion evidence is sufficient to evaluate the relevant G10 condition/health fact.

# 8. Trigger

A consuming maintenance domain completes authorised work affecting infrastructure condition.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the source-defined maintenance completion fact from the execution owner. | Facilities Management → Infrastructure Management | G10 has authoritative evidence that maintenance work completed. |
| 2 | Resolve the affected canonical Infrastructure. | Infrastructure Management | Completion evidence is linked to the correct G10 resource. |
| 3 | Evaluate the maintenance-determined condition assessment without changing the G01 maintenance record. | Infrastructure Management | G10 has the source input for reconciliation. |
| 4 | Reconcile only the G10 health/condition/lifecycle facts supported by the evidence. | Infrastructure Management | Current Infrastructure facts become authoritative for the post-maintenance condition. |
| 5 | If valid lifecycle reconciliation returns Infrastructure from `Under Maintenance` to `Operational`, publish `MaintenanceCompleted`; if Building/Space health changes, publish the corresponding health-update event. | Infrastructure Management | Only actual completed G10 facts are published. |
| 6 | End without changing G01 Work Order state, verification, closure or maintenance history. | Infrastructure Management | Execution ownership remains with G01. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| No G10 fact changes | Completion does not support a lifecycle/health change. | Reconciliation ends with existing G10 facts unchanged and no invented event. | Ownership remains consistent. |
| Building health changes | Evidence supports a new Building health assessment. | Apply source-supported Building health update and publish `BuildingHealthUpdated`. | Building health is current. |
| Space health changes | Evidence supports a new Space health assessment. | Apply source-supported Space health update and publish `SpaceHealthUpdated`. | Space health is current. |
| Availability also needs change | Post-maintenance condition affects usability. | Complete condition reconciliation and invoke the separate availability scenario. | Condition and availability remain distinct. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Maintenance completion cannot be associated with canonical Infrastructure. | G10 condition cannot be safely reconciled. | Resolve the authoritative Infrastructure reference from G01 evidence. | G01 / G10 |
| Evidence is insufficient for a new condition assessment. | No trustworthy G10 change can be asserted. | Retain existing G10 condition until source-supported evidence becomes available. | Infrastructure Management |
| G01 execution history conflicts with a requested G10 update. | Reconciliation would corrupt ownership. | Resolve the G01 source fact first; G10 must not rewrite it. | Facilities Management / Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Facilities owns maintenance execution; G10 owns physical Infrastructure. | G01 / G10 | Completion is consumed as evidence only. | L2-G01 §§1–6; L2-G10 §§1–6 |
| Maintenance determines condition; Infrastructure stores the latest assessment. | G01 / G10 | G10 reconciles its current assessment from the maintenance fact. | L2-G10 health/condition rules |
| `Under Maintenance → Operational` is an allowed G10 lifecycle transition. | Infrastructure Management | Return to Operational is permitted only where valid. | L2-G10 lifecycle transition matrix |
| `MaintenanceCompleted` is a G10 lifecycle event. | Infrastructure Management | Exact event is used only when that lifecycle fact occurs. | L2-G10 lifecycle events |
| G01 Work Orders remain under their own lifecycle through completion, verification and closure. | Facilities Management | G10 never advances G01 Work Order state. | L2-G01 Work Order lifecycle |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| G01 Work Order / maintenance execution | Source-defined completed execution position | Consumed as evidence only. | Unchanged by G10. | Facilities Management | L2-G01 Work Order lifecycle |
| Physical Infrastructure lifecycle | `Under Maintenance` where applicable | May validly return to `Operational` after source-supported reconciliation. | `Operational` only when transition rules support it; otherwise unchanged. | Infrastructure Management | L2-G10 lifecycle matrix |
| Building / Space health | Existing source-defined health assessment | May be updated from maintenance-determined condition evidence. | Existing source-supported health classification. | Infrastructure Management | L2-G10 Building/Space Health |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Scenario Purpose |
|---|---|---|
| `WorkOrderCompleted` | Facilities Management | Source completion fact where a Work Order completes. |
| `PreventiveMaintenanceCompleted` | Facilities Management | Source completion fact for preventive maintenance activity. |
| `MaintenanceCompleted` | Infrastructure Management | G10 lifecycle return from `Under Maintenance` to `Operational`. |
| `BuildingHealthUpdated` | Infrastructure Management | Completed Building health change. |
| `SpaceHealthUpdated` | Infrastructure Management | Completed Space health change. |

No generic condition-reconciliation event is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

G01 supplies source-defined completion facts. G10 resolves the referenced Infrastructure and changes only G10-owned current condition/health/lifecycle facts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Complete/verify maintenance execution | Source-supported G01 roles | Remains governed entirely by G01. | Facilities Management |
| Reconcile G10 condition/health | Infrastructure Administrator or Maintenance Coordinator within source-supported responsibility | Must be grounded in authoritative maintenance completion evidence and G10 rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

G01 verification/closure authority remains with G01. No additional G10 maker-checker rule is invented for reconciliation.

## 15.3 Business Audit and Evidence Obligations

G01 preserves execution/completion/verification history. G10 preserves current Infrastructure condition/health and its own source-defined update history. Their relationship remains traceable without duplicating aggregates.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Authoritative maintenance completion evidence is available.
- Canonical affected Infrastructure is resolved.
- G10 condition/health/lifecycle facts are reconciled only where supported.
- Applicable exact source-defined G10 event is published only when its fact changed.
- G01 execution history remains untouched.

## 16.2 Postconditions

Infrastructure Management owns the reconciled current physical facts; Facilities Management continues to own complete maintenance execution history.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking source conflict was identified for the frozen reconciliation outcome.

**Source Assessment:** **PASS**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.
