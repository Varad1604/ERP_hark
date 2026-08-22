# Preventive Maintenance Governance to Infrastructure Maintenance Requirement

# 1. Scenario Metadata

**Scenario ID:** L3-G-304  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows  
**Referenced By:** Facilities Management and authorized Section G consumers of preventive-maintenance governance facts

# 2. Scenario Purpose

Govern a preventive-maintenance need identified from Infrastructure condition or policy so that the maintenance governance requirement is recorded against authoritative physical Infrastructure and passed toward Facilities-owned operational execution without transferring Infrastructure ownership or creating a second maintenance-execution lifecycle inside G10.

# 3. Business Outcome

The maintenance governance requirement is recorded while execution remains owned by G01

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when infrastructure condition or policy identifies a preventive maintenance requirement.

## 4.2 End Boundary

The scenario ends when the preventive-maintenance governance requirement is recorded against the applicable G10 Infrastructure and the source-defined hand-off toward Facilities-owned execution is established, without G10 executing the maintenance work.

## 4.3 In Scope

- Identification of the authoritative physical Infrastructure requiring preventive maintenance.
- Evaluation of the applicable preventive-maintenance governance need from Infrastructure condition or policy.
- Recording of the G10 preventive-maintenance governance requirement against the Infrastructure.
- Initiation of the source-defined maintenance-request hand-off toward Facilities Management.
- Preservation of Infrastructure ownership in G10 and operational maintenance execution in G01.

## 4.4 Out of Scope

- Creation, assignment, execution, verification or closure of a Facilities Work Order.
- Ownership of the G01 Preventive Maintenance Plan or Maintenance Schedule aggregates by G10.
- Technician or vendor assignment.
- Completion verification or Infrastructure condition reconciliation after work is completed.
- Invention of a local preventive-maintenance lifecycle or duplicate event publisher.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-G-307 — Maintenance Completion to Infrastructure Condition Reconciliation | Completed Facilities-owned maintenance may later require G10 reconciliation of the Infrastructure condition. |
| Related | L3-G-305 — Corrective Maintenance Governance to Infrastructure Remediation Requirement | Corrective maintenance is a separate governance outcome triggered by an identified defect. |
| Related | L3-G-306 — Emergency Infrastructure Condition to Emergency Maintenance Governance | Emergency maintenance is a separate urgent governance outcome. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns physical Infrastructure and the Infrastructure-side preventive-maintenance governance requirement. G10 governs preventive-maintenance planning but does not execute maintenance. |
| Facilities Management / Section G | Owns operational maintenance, Preventive Maintenance Plan, Maintenance Schedule, Work Orders and execution once the requirement enters the Facilities execution context. Physical Infrastructure remains owned by G10. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Coordinator | Coordinates the Infrastructure-side preventive-maintenance requirement. | May progress G10 maintenance governance within authorized Infrastructure responsibility. |
| Infrastructure Administrator | Administers the applicable Infrastructure and preventive-maintenance governance context. | May record or govern the requirement within authorized Infrastructure scope. |
| Facilities Manager | Receives or governs the Facilities-owned operational maintenance execution after hand-off. | Does not own the source Infrastructure or the G10 governance requirement. |
| Maintenance Supervisor | Participates in Facilities-owned operational planning/execution after hand-off. | Does not execute work inside this G10 scenario boundary. |

# 7. Preconditions and Entry Criteria

- The applicable physical Infrastructure exists under G10 ownership.
- Infrastructure condition or an applicable policy identifies a preventive-maintenance need.
- The maintenance need can be associated with an authoritative Infrastructure reference.
- Any required authorization for the business operation has been satisfied under the platform authorization architecture.

# 8. Trigger

Infrastructure condition or policy identifies a preventive maintenance requirement

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative physical Infrastructure and the preventive-maintenance need derived from its condition or applicable policy. | Infrastructure Management | The maintenance-governance scope is established against canonical Infrastructure. |
| 2 | Validate that the preventive-maintenance requirement is applicable to the Infrastructure and current governance context. | Infrastructure Management | The requirement is accepted for G10 preventive-maintenance governance. |
| 3 | Record the Infrastructure-side preventive-maintenance governance requirement without creating a Facilities Work Order or claiming ownership of G01 execution aggregates. | Infrastructure Management | The required preventive-maintenance governance fact is preserved against the Infrastructure. |
| 4 | Establish the source-defined maintenance-request hand-off toward Facilities Management. | Infrastructure Management → Facilities Management | Operational execution can proceed under G01 ownership without transferring physical-Infrastructure ownership. |
| 5 | End the G10 scenario before Facilities scheduling, assignment or execution begins. | Infrastructure Management | The frozen outcome is reached: the governance requirement is recorded and execution remains G01-owned. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Configured maintenance programme varies | Institutional policy uses a supported frequency or institution-configured maintenance programme. | Record the requirement according to the applicable G10 governance policy while leaving G01 operational plan/schedule ownership unchanged. | The same Infrastructure governance boundary is preserved. | Infrastructure Management |
| Requirement is not preventive | Evaluation establishes that the condition is corrective or emergency rather than preventive. | Stop this scenario and use the separately governed corrective or emergency Infrastructure-maintenance outcome. | Preventive governance is not misused for a different maintenance category. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Infrastructure reference is invalid or unavailable | The preventive-maintenance requirement cannot be attached to authoritative physical Infrastructure. | Resolve the G10 Infrastructure reference before a later governance attempt. | Infrastructure Management |
| Requirement conflicts with applicable policy | The preventive-maintenance governance requirement cannot be accepted as proposed. | Resolve the governing condition or policy before recording a new requirement. | Infrastructure Management |
| Facilities hand-off cannot be canonically expressed because event ownership is ambiguous | The requirement may be recorded, but the event-driven hand-off cannot be published without risking duplicate event ownership. | Preserve the Draft and require upstream resolution of the canonical event publisher before asserting the event contract. | Section G architecture authority |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure is the canonical owner of physical institutional Infrastructure. | Infrastructure Management | The maintenance requirement references G10 Infrastructure and never transfers physical ownership to Facilities. | L2-G10 §§1–6; SutraERP L1 Business Domain Architecture |
| Infrastructure governs preventive-maintenance planning; execution of maintenance activities belongs to Facilities. | Infrastructure Management / Facilities Management | The G10 scenario ends before operational execution. | L2-G10 §33; L2-G01 §§1–6 |
| G10's preventive-maintenance workflow proceeds through Maintenance Schedule → Reminder Generated → Maintenance Request Created → Facilities Domain → Maintenance Completed → Infrastructure Updated. | Infrastructure Management / Facilities Management | This scenario covers the governance requirement and hand-off only; later completion/reconciliation is outside its end boundary. | L2-G10 §33 Preventive Maintenance Workflow |
| G01 owns Preventive Maintenance Plan, Maintenance Schedule, Work Orders and operational maintenance execution. | Facilities Management | G10 cannot claim G01's execution aggregates merely because G10 governs the Infrastructure-side requirement. | L2-G01 §§3, 5, 8, 20 |
| Infrastructure does not execute maintenance. | Infrastructure Management | Technician, vendor and Work Order activities remain outside G10. | L2-G10 §33 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | Existing source-defined lifecycle / operational condition | The scenario records a preventive-maintenance governance need; it does not itself change Infrastructure lifecycle or condition. | Unchanged by this scenario unless a separate governed availability/lifecycle outcome applies. | Infrastructure Management | L2-G10 lifecycle and maintenance-governance boundary |
| G01 Preventive Maintenance Plan / Maintenance Schedule | Not owned by this scenario | The G10 requirement may lead into Facilities-owned planning and scheduling after hand-off. | Any resulting G01 lifecycle is outside this scenario boundary. | Facilities Management | L2-G01 §8 and Preventive Maintenance lifecycle |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No canonical publisher is asserted for `PreventiveMaintenanceScheduled` in this Draft. Both locked L2-G10 and locked L2-G01 define the exact event name `PreventiveMaintenanceScheduled` in their maintenance event catalogues, but they attach it to their respective Infrastructure-governance and Facilities-operational contexts. Selecting either as the sole publisher would silently resolve an upstream canonical-publisher conflict.

## 14.2 Business Contracts and Cross-Domain Hand-offs

G10 explicitly defines a preventive-maintenance flow that creates a maintenance request and hands execution to the Facilities domain. G01 owns the resulting operational maintenance execution. The exact named synchronous contract for that hand-off is not defined in the reviewed locked sources, so this Draft describes only the source-supported business hand-off and does not invent a contract name.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Record an Infrastructure preventive-maintenance governance requirement | Maintenance Coordinator or Infrastructure Administrator within applicable authorized scope | The requirement must be grounded in Infrastructure condition or applicable policy. | Infrastructure Management |
| Execute resulting preventive maintenance | Facilities roles under the G01 authorization model | Execution occurs only after the requirement enters G01; it is not performed by G10. | Facilities Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal scenario-specific maker-checker, approval or separation-of-duty rule for creating the G10 preventive-maintenance governance requirement is defined by the authoritative sources. G01 separately defines configurable maintenance approval and execution authorities for its own operational lifecycle.

## 15.3 Business Audit and Evidence Obligations

The G10 requirement and subsequent G01 operational maintenance history must remain traceable under their respective canonical owners. This Draft does not duplicate the Facilities Work Order or maintenance history inside Infrastructure Management.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authoritative Infrastructure and preventive-maintenance need have been identified.
- The G10 preventive-maintenance governance requirement has been recorded.
- The business hand-off toward G01-owned maintenance execution has been established at the source-supported semantic level.
- No G01 Work Order, technician assignment or execution activity has been claimed by G10.

## 16.2 Postconditions

- Infrastructure Management retains ownership of the physical Infrastructure and Infrastructure-side governance fact.
- Facilities Management remains responsible for any resulting preventive-maintenance plan, schedule, Work Order and execution lifecycle.
- Event publication for the preventive-maintenance scheduling fact remains unresolved pending canonical publisher clarification.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L2-G10 and locked L2-G01 both define the exact event name `PreventiveMaintenanceScheduled`. G10 describes it as “Preventive maintenance planned”; G01 describes it as “Schedule generated.” | SutraERP requires one canonical publisher for every business event. This Draft cannot publish the event without choosing between two locked L2 definitions. | Architecture change control must identify the canonical publisher and, if both facts are required, define distinct canonical event names/semantics. |
| L2-G10 says Infrastructure governs preventive-maintenance schedules and records, while L2-G01 explicitly owns the `Preventive Maintenance Plan` and `Maintenance Schedule` aggregates. | The frozen outcome can be preserved as an Infrastructure-side maintenance governance requirement, but the exact canonical business concept used to store that G10 requirement is not explicitly defined as a distinct aggregate. | Section G architecture should clarify the G10 governance-record concept versus the G01 operational plan/schedule aggregates. |

**Architecture assessment:** `BLOCKED`

The business boundary and frozen outcome are clear, but the duplicate canonical event publisher and overlapping schedule-governance language materially prevent a source-complete event/ownership contract from being asserted.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
