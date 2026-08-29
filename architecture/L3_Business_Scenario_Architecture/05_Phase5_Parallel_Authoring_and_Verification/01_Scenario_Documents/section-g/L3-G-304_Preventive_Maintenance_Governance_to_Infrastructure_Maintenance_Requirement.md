# Preventive Maintenance Governance to Infrastructure Maintenance Requirement

# 1. Scenario Metadata

**Scenario ID:** L3-G-304  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Record a preventive-maintenance governance requirement against canonical Infrastructure while keeping Facilities Management as owner of operational maintenance execution.

# 3. Business Outcome

The maintenance governance requirement is recorded while execution remains owned by G01.

The scenario is **BLOCKED** because locked G10 and G01 overlap on preventive-maintenance schedule ownership and both define `PreventiveMaintenanceScheduled` with different meanings.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when infrastructure condition or policy identifies a preventive maintenance requirement.

## 4.2 End Boundary

The intended end boundary is the recorded G10 governance requirement and hand-off to G01 execution, but the canonical schedule/event boundary must be resolved upstream.

## 4.3 In Scope

- Identification of the affected canonical Infrastructure.
- G10 preventive-maintenance governance requirement.
- Preservation of G01 execution ownership.
- Explicit publisher/ownership conflict recording.

## 4.4 Out of Scope

- G01 Work Order, technician/vendor assignment, execution, verification or closure.
- Locally choosing ownership of Preventive Maintenance Plan or Maintenance Schedule.
- Locally choosing a publisher for duplicated `PreventiveMaintenanceScheduled`.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns physical Infrastructure and states that Infrastructure governs preventive-maintenance planning. |
| Facilities Management / G01 | Owns operational maintenance execution and explicitly owns Preventive Maintenance Plan and Maintenance Schedule aggregates. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Coordinator | Coordinates the G10 Infrastructure-side maintenance governance requirement. | Cannot take ownership of G01 execution aggregates. |
| Infrastructure Administrator | Administers the affected Infrastructure. | Cannot resolve the schedule/event ownership conflict locally. |
| Facilities Manager | Owns the later G01 operational maintenance execution. | Does not own physical Infrastructure. |

# 7. Preconditions and Entry Criteria

- Canonical Infrastructure exists.
- Infrastructure condition or policy identifies a preventive-maintenance need.
- The need can be associated with the canonical Infrastructure.
- Existing ownership boundaries are preserved.

# 8. Trigger

Infrastructure condition or policy identifies a preventive maintenance requirement.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the canonical Infrastructure and preventive-maintenance need. | Infrastructure Management | The G10 governance scope is established. |
| 2 | Validate that the requirement is preventive rather than corrective or emergency. | Infrastructure Management | The correct maintenance category is identified. |
| 3 | Record the Infrastructure-side governance requirement without creating a G01 Work Order. | Infrastructure Management | The frozen G10 governance fact is preserved. |
| 4 | Establish the hand-off toward G01 operational maintenance. | Infrastructure Management → Facilities Management | Execution ownership remains with G01. |
| 5 | Publish the canonical preventive-maintenance scheduling/governance event. | **BLOCKED — publisher conflict** | Both G10 and G01 define `PreventiveMaintenanceScheduled` with different meanings. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Corrective condition | A defect already exists. | Use the corrective-maintenance governance scenario. | Preventive path is not misused. |
| Emergency condition | Urgent intervention is required. | Use the emergency-maintenance governance scenario. | Emergency path remains distinct. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Canonical event publisher remains ambiguous. | Required event cannot be published without violating one-publisher governance. | Resolve upstream publisher/event semantics. | Architecture governance |
| Schedule ownership remains ambiguous. | L3 cannot state whether the G10 governance fact is itself a Maintenance Schedule/Plan. | Clarify G10 governance record versus G01 operational schedule/plan. | Architecture governance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure governs preventive-maintenance planning; Infrastructure does not execute maintenance. | Infrastructure Management | G10 scenario ends before execution. | L2-G10 §33 |
| G01 owns Preventive Maintenance Plan, Maintenance Schedule, Work Orders and maintenance execution. | Facilities Management | G10 cannot re-own these aggregates. | L2-G01 §§3, 5, 8, 20 |
| Both G10 and G01 define `PreventiveMaintenanceScheduled` but with different meanings. | Conflicting locked sources | L3 cannot assign one canonical publisher. | L2-G10 Maintenance Events; L2-G01 Maintenance Events |
| Physical Infrastructure remains owned by G10. | Infrastructure Management | Facilities execution never transfers physical ownership. | L2-G10 §§1–6; L2-G01 §§1–4 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | Existing source-defined condition | Preventive governance need is recorded; no maintenance execution occurs. | Infrastructure state unchanged unless a separate scenario applies. | Infrastructure Management | L2-G10 §33 |
| G01 Preventive Maintenance Plan / Schedule | Outside G10 ownership | May be established after hand-off. | G01-owned lifecycle only. | Facilities Management | L2-G01 §§8, 20 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

`PreventiveMaintenanceScheduled` is defined in both locked G10 (“Preventive maintenance planned”) and locked G01 (“Schedule generated”). No canonical publisher is selected in this Draft.

## 14.2 Business Contracts and Cross-Domain Hand-offs

G10's preventive flow hands maintenance toward the Facilities domain. The exact synchronous hand-off contract name is not defined in the reviewed locked sources.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Source-supported G10 roles may record the requirement; G01 roles own later execution. No actor may use this scenario to transfer aggregate ownership.

## 15.2 Separation of Duties, Overrides and Exceptions

No universal G10 maker-checker or approval rule is defined for this governance requirement.

## 15.3 Business Audit and Evidence Obligations

G10 governance history and G01 maintenance execution history must remain separately traceable under their canonical owners.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

Full canonical completion requires:
- a source-supported G10 preventive-maintenance governance record;
- a resolved G10/G01 schedule-ownership boundary;
- one canonical publisher/event semantic for preventive-maintenance scheduling;
- preservation of G01 execution ownership.

## 16.2 Postconditions

Physical Infrastructure remains owned by G10; any operational preventive-maintenance lifecycle remains owned by G01.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Both G10 and G01 define `PreventiveMaintenanceScheduled` with different meanings. | Material canonical-publisher conflict. | Architecture governance must assign canonical event semantics/publisher(s), using distinct names if both facts are required. |
| G10 says Infrastructure governs preventive-maintenance planning while G01 owns Preventive Maintenance Plan and Maintenance Schedule aggregates. | Material ownership ambiguity for the stored governance/schedule concept. | Clarify G10 governance requirement versus G01 operational plan/schedule ownership. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
