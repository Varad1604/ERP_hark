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
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows  
**Referenced By:** Authorized Section G consumers of current Infrastructure condition, health and operational facts

# 2. Scenario Purpose

Reconcile authoritative Infrastructure condition after an authorized maintenance domain completes work affecting physical Infrastructure so that G10 stores the current supported condition assessment while the maintenance execution record and lifecycle remain owned by the executing maintenance domain.

# 3. Business Outcome

G10 reconciles the resulting infrastructure state without taking ownership of maintenance execution

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a consuming maintenance domain completes authorised work affecting infrastructure condition.

## 4.2 End Boundary

The scenario ends when G10 has reconciled the authoritative current Infrastructure condition or health assessment from the source-domain maintenance completion fact and any resulting source-defined G10 update fact has been published, without copying or re-owning the maintenance execution lifecycle.

## 4.3 In Scope

- Consumption of a source-defined completed maintenance fact from the maintenance execution owner.
- Resolution of the affected authoritative G10 Infrastructure reference.
- Use of the maintenance-domain condition assessment as an input to G10 reconciliation.
- Update of the applicable G10 current condition or health assessment where source rules support it.
- Publication of the applicable source-defined G10 lifecycle or health-update event when reconciliation changes an authoritative Infrastructure fact.
- Preservation of G01 ownership of Work Orders, maintenance execution and maintenance history.

## 4.4 Out of Scope

- Reopening, verifying or closing the Facilities Work Order on behalf of G01.
- Re-performing maintenance or changing maintenance completion evidence.
- Invention of a generic Infrastructure condition state or a new condition-reconciliation event not defined upstream.
- Automatic availability, compliance or lifecycle changes that require their own governed outcomes.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| May follow | L3-G-304 — Preventive Maintenance Governance to Infrastructure Maintenance Requirement | Preventive work can later generate a maintenance completion requiring reconciliation. |
| May follow | L3-G-305 — Corrective Maintenance Governance to Infrastructure Remediation Requirement | Corrective work can later generate a maintenance completion requiring reconciliation. |
| May follow | L3-G-306 — Emergency Infrastructure Condition to Emergency Maintenance Governance | Emergency work can later generate a maintenance completion requiring reconciliation. |
| Related | L3-G-290 — Infrastructure Availability Change to Suspension or Restoration Outcome | A reconciled condition may separately support an authorized availability restoration or restriction decision. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns physical Infrastructure and the authoritative current Infrastructure condition/health facts reconciled after maintenance. |
| Facilities Management / Section G | Owns Work Orders, preventive/corrective/emergency maintenance execution, completion evidence, verification, closure and maintenance history. It supplies completed maintenance facts without transferring execution ownership. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Coordinator | Coordinates the Infrastructure-side reconciliation of maintenance outcome against the affected physical resource. | Does not change G01 execution history. |
| Infrastructure Administrator | Administers the current G10 Infrastructure condition/health fact after supported maintenance completion. | May apply a source-defined G10 update within authorized Infrastructure scope. |
| Facilities Manager | Owns the operational maintenance completion context in G01. | Supplies or governs the Facilities completion fact; does not update G10 Infrastructure directly. |
| Maintenance Supervisor | Participates in G01 completion/verification according to Facilities governance. | Does not own the G10 Infrastructure condition. |

# 7. Preconditions and Entry Criteria

- A source-defined maintenance completion fact exists from the authoritative maintenance execution owner.
- The completed work references authoritative physical Infrastructure owned by G10.
- Maintenance completion includes sufficient source-domain condition or completion evidence to support reconciliation.
- The affected Infrastructure remains identifiable without creating a duplicate physical record.

# 8. Trigger

A consuming maintenance domain completes authorised work affecting infrastructure condition

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the source-defined completed maintenance fact from the authoritative maintenance execution domain. | Facilities Management → Infrastructure Management | G10 receives evidence that authorized maintenance work affecting Infrastructure has completed. |
| 2 | Resolve the affected authoritative G10 Infrastructure reference from the maintenance completion context. | Infrastructure Management | The maintenance outcome is associated with canonical physical Infrastructure. |
| 3 | Evaluate the maintenance-determined condition assessment and relevant completion evidence without modifying the G01 maintenance record. | Infrastructure Management | G10 has the source-domain input needed for condition reconciliation. |
| 4 | Reconcile the applicable G10 lifecycle and current condition/health facts using the maintenance completion evidence and source-defined rules. | Infrastructure Management | The authoritative Infrastructure state reflects the supported post-maintenance position without copying the maintenance execution lifecycle. |
| 5 | If Infrastructure transitions from `Under Maintenance` to `Operational`, publish the source-defined G10 lifecycle event `MaintenanceCompleted`; if an applicable Building or Space health fact changes, publish the corresponding source-defined health-update event. | Infrastructure Management | Authorized consumers receive only the canonical completed facts that actually changed. |
| 6 | End the scenario without changing Work Order state, completion evidence or maintenance-history ownership. | Infrastructure Management | The frozen outcome is reached with execution ownership preserved in the maintenance domain. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Maintenance completion does not support a G10 state change | Source-domain evidence confirms work completion but does not support a lifecycle or health change. | Complete reconciliation without inventing a new Infrastructure state or event. | G10 current Infrastructure facts remain unchanged. | Infrastructure Management |
| Building health changes | The affected aggregate is a Building and maintenance evidence supports a new Building health assessment. | Apply the source-defined Building health update and publish `BuildingHealthUpdated`. | Current Building health is reconciled under G10 ownership. | Infrastructure Management |
| Space health changes | The affected aggregate is a Space and maintenance evidence supports a new Space health assessment. | Apply the source-defined Space health update and publish `SpaceHealthUpdated`. | Current Space health is reconciled under G10 ownership. | Infrastructure Management |
| Availability must change | Reconciled condition implies that Infrastructure usability should be suspended or restored. | Complete condition reconciliation and initiate the separately governed availability change outcome rather than silently modifying availability here. | Condition and availability governance remain distinct. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Maintenance completion fact cannot be associated with canonical Infrastructure | G10 cannot safely reconcile physical condition. | Resolve the authoritative Infrastructure reference from the source-domain maintenance history before reconciliation. | Facilities Management provides source reference; Infrastructure Management resolves canonical Infrastructure. |
| Completion evidence is insufficient to support a condition assessment | No trustworthy G10 condition change can be asserted. | Preserve the existing G10 condition until source-supported evidence or assessment becomes available. | Infrastructure Management; Facilities Management retains execution evidence. |
| Source-domain execution history conflicts with requested G10 update | Reconciliation cannot proceed without risking ownership corruption. | Resolve the source-domain maintenance fact first; do not rewrite G01 history from G10. | Facilities Management for maintenance fact; Infrastructure Management for G10 condition fact. |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Facilities owns operational maintenance execution; Infrastructure owns physical Infrastructure. | Facilities Management / Infrastructure Management | Completion is consumed as a source-domain fact; G10 never re-owns the Work Order or repair execution. | L2-G01 §§1–6, 8–11; L2-G10 §§1–6 |
| G10 preventive-maintenance governance explicitly proceeds through Facilities Domain → Maintenance Completed → Infrastructure Updated. | Infrastructure Management / Facilities Management | G10 reconciliation occurs after the maintenance-domain completion point. | L2-G10 §33 Preventive Maintenance Workflow |
| Maintenance determines condition; Infrastructure stores the latest assessment. | Facilities Management / Infrastructure Management | G10 consumes the maintenance-determined assessment but owns the authoritative current Infrastructure assessment. | L2-G10 Infrastructure health/condition rules |
| Source-defined health classifications include Excellent, Good, Fair, Poor and Critical for applicable Infrastructure health profiles. | Infrastructure Management | The scenario does not invent a new generic condition vocabulary. | L2-G10 Building/Space health rules |
| G01 Work Orders have their own Created → Assigned → Accepted → In Progress → Paused → Completed → Verified → Closed lifecycle. | Facilities Management | G10 reconciliation does not advance or rewrite the Work Order lifecycle. | L2-G01 Work Order Lifecycle |
| G10 permits `Under Maintenance → Operational` and defines `MaintenanceCompleted` as the completed lifecycle event for that return. | Infrastructure Management | A post-maintenance return to Operational is asserted only when source-defined lifecycle validation supports it. | L2-G10 §11A.53, §11A.58 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| G01 Work Order / maintenance execution | Source-defined completed execution position such as `Completed`, with later verification/closure owned by G01 | Consumed as evidence only; no G10 mutation. | Unchanged by G10. | Facilities Management | L2-G01 Work Order Lifecycle |
| Physical Infrastructure lifecycle | `Under Maintenance` where the G10 lifecycle had been placed into maintenance | Reconciled from the maintenance completion evidence and G10 lifecycle rules. | `Operational` only when the permitted `Under Maintenance → Operational` transition is valid; otherwise no unsupported transition is asserted. | Infrastructure Management | L2-G10 §11A.53, §11A.58 |
| Building health | Existing source-defined health assessment | Updated only when maintenance evidence supports a new assessment. | One of the existing source-defined health values; no new state created. | Infrastructure Management | L2-G10 Building Health workflow |
| Space health | Existing source-defined health assessment | Updated only when maintenance evidence supports a new assessment. | One of the existing source-defined health values; no new state created. | Infrastructure Management | L2-G10 Space Health workflow |
| Infrastructure availability / lifecycle | Existing source-defined state | Not automatically changed by condition reconciliation. | Unchanged unless a separate governed availability/lifecycle scenario is invoked. | Infrastructure Management | L2-G10 lifecycle and availability governance |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `WorkOrderCompleted` | Facilities Management | Infrastructure Management where the completed work affects authoritative Infrastructure condition | Source-defined G01 fact that Work Order execution has completed. | L2-G01 Work Order Events |
| `PreventiveMaintenanceCompleted` | Facilities Management | Infrastructure Management where preventive work affects authoritative Infrastructure condition | Source-defined G01 fact that preventive maintenance activity completed. | L2-G01 Maintenance Events |
| `MaintenanceCompleted` | Infrastructure Management | Authorized consumers of Infrastructure lifecycle facts | Publishes the completed G10 lifecycle fact when Infrastructure validly transitions from `Under Maintenance` back to `Operational`. | L2-G10 §11A.53 and §11A.58 |
| `BuildingHealthUpdated` | Infrastructure Management | Authorized consumers of current Building health | Publishes a completed G10 Building health reconciliation when applicable. | L2-G10 Building Health workflow/events |
| `SpaceHealthUpdated` | Infrastructure Management | Authorized consumers of current Space health | Publishes a completed G10 Space health reconciliation when applicable. | L2-G10 Space Health workflow/events |

No separate generic infrastructure-condition-reconciled event is defined by the reviewed locked sources, so this Draft uses only the source-defined lifecycle or aggregate-specific health event that corresponds to the fact actually changed.

## 14.2 Business Contracts and Cross-Domain Hand-offs

G01 supplies source-defined maintenance-completion facts. G10 resolves the referenced Infrastructure and reconciles only the physical-condition/health facts it owns. The collaboration does not transfer ownership of Work Orders, completion evidence, verification or maintenance history to G10.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Complete maintenance execution | Authorized Facilities roles under G01 | Completion, verification and closure authority remain governed by G01. | Facilities Management |
| Reconcile G10 Infrastructure health/condition | Infrastructure Administrator or Maintenance Coordinator within applicable authorized responsibility | Reconciliation must be grounded in authoritative maintenance completion evidence and G10 condition rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

G01 requires verification before operational Work Order closure and defines configurable verification authority. This L3 scenario does not transfer those verification duties to G10 or invent an additional G10 maker-checker rule for condition reconciliation.

## 15.3 Business Audit and Evidence Obligations

- G01 preserves Work Order, execution, completion, verification and maintenance history under Facilities ownership.
- G10 preserves the current authoritative Infrastructure condition/health fact and its source-defined update history.
- The relationship between the maintenance completion fact and resulting G10 update must remain traceable without duplicating the maintenance aggregate.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A source-defined completed maintenance fact has been received from the authoritative maintenance owner.
- The affected canonical G10 Infrastructure has been resolved.
- Maintenance condition evidence has been evaluated without modifying G01 execution history.
- The applicable G10 lifecycle and condition/health facts have been reconciled or explicitly remain unchanged when evidence does not support a change.
- Any applicable source-defined `MaintenanceCompleted`, `BuildingHealthUpdated` or `SpaceHealthUpdated` event has been published only when its corresponding fact changed.

## 16.2 Postconditions

- Infrastructure Management owns the reconciled current physical condition/health fact.
- Facilities Management continues to own the complete maintenance execution lifecycle and history.
- Any required availability, compliance or lifecycle change remains a separate governed outcome.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking architecture issue was identified. L2-G01 owns maintenance execution and publishes source-defined completion facts; L2-G10 explicitly states that Maintenance determines condition while Infrastructure stores the current assessment, permits `Under Maintenance → Operational`, and defines `MaintenanceCompleted` plus aggregate-specific health-update events. The scenario can therefore reconcile only the source-defined G10 facts that actually change without inventing a generic condition event.

**Architecture assessment:** `PASS`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
