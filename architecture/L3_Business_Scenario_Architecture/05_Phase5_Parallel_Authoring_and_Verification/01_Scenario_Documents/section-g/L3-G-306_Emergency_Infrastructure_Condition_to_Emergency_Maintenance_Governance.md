# Emergency Infrastructure Condition to Emergency Maintenance Governance

# 1. Scenario Metadata

**Scenario ID:** L3-G-306  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows  
**Referenced By:** Facilities Management and authorized Section G consumers of Infrastructure emergency-governance facts

# 2. Scenario Purpose

Govern a critical Infrastructure condition requiring urgent intervention so that G10 records the authoritative physical condition and emergency-maintenance governance requirement while operational response remains with the appropriate Section G operational owner and physical-Infrastructure ownership remains with Infrastructure Management.

# 3. Business Outcome

G10 records the infrastructure emergency condition and governance requirement while execution remains with the appropriate operational domain

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a critical infrastructure condition requires urgent intervention.

## 4.2 End Boundary

The scenario ends when the critical condition has been associated with authoritative G10 Infrastructure, the emergency-maintenance governance requirement has been recorded, and the operational intervention has been handed to the appropriate Section G execution domain without G10 executing the maintenance response.

## 4.3 In Scope

- Identification of authoritative physical Infrastructure affected by a critical condition.
- Recognition of a source-supported emergency-maintenance condition requiring immediate intervention.
- Recording of the G10 Infrastructure emergency condition and governance requirement.
- Business hand-off toward the appropriate Section G operational execution domain, ordinarily Facilities Management for maintenance execution.
- Preservation of canonical physical-Infrastructure ownership in G10.

## 4.4 Out of Scope

- Facilities Work Order execution, technician/vendor response, repair verification or closure.
- Security, medical or other operational incident execution owned by another operational domain.
- Automatic Infrastructure closure, availability restriction, disaster-recovery state or lifecycle change unless a separate governed outcome applies.
- Invention of a local emergency-maintenance event publisher to resolve conflicting L2 definitions.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-G-307 — Maintenance Completion to Infrastructure Condition Reconciliation | Completed emergency maintenance may require G10 reconciliation of the physical Infrastructure condition. |
| Related | L3-G-305 — Corrective Maintenance Governance to Infrastructure Remediation Requirement | Non-emergency defects use the corrective-maintenance governance path. |
| Related | L3-G-290 — Infrastructure Availability Change to Suspension or Restoration Outcome | A critical condition may separately require an authoritative availability suspension or later restoration. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical Infrastructure, current Infrastructure condition facts and the Infrastructure-side emergency-governance requirement. |
| Facilities Management / Section G | Owns emergency-maintenance operational execution when the condition requires Facilities response; it never owns the physical Infrastructure. |
| Other applicable Section G operational owner | Retains its own operational execution where the urgent condition belongs to another established Section G operational responsibility. | 

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the affected Infrastructure and emergency-governance context. | May record or progress the G10 emergency requirement within authorized scope. |
| Maintenance Coordinator | Coordinates the Infrastructure-side urgent maintenance requirement. | Does not execute the Facilities Work Order or technician response inside G10. |
| Facilities Manager | Governs Facilities-owned emergency-maintenance execution after hand-off where Facilities is the applicable executor. | Does not take ownership of the physical Infrastructure. |
| Maintenance Supervisor | Coordinates later G01 emergency work execution. | Participation begins after the G10 scenario hands off operational execution. |

# 7. Preconditions and Entry Criteria

- The affected physical resource can be resolved to authoritative G10 Infrastructure.
- The condition is critical enough to require urgent intervention under the source-defined emergency-maintenance meaning.
- The applicable operational execution owner can be identified from locked Section G ownership.
- Required business authorization is governed under the platform authorization architecture; emergency authority is not invented by this scenario.

# 8. Trigger

A critical infrastructure condition requires urgent intervention

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative physical Infrastructure affected by the critical condition. | Infrastructure Management | The emergency governance is anchored to canonical Infrastructure. |
| 2 | Evaluate the condition against the source-defined emergency-maintenance meaning and preserve the current Infrastructure condition fact. | Infrastructure Management | The condition is confirmed as requiring urgent intervention without inventing a new condition category. |
| 3 | Record the G10 emergency-maintenance governance requirement against the affected Infrastructure. | Infrastructure Management | The urgent Infrastructure requirement is governed under G10. |
| 4 | Determine the established Section G operational owner responsible for intervention without transferring physical-Infrastructure ownership. | Infrastructure Management | The execution destination is selected from existing ownership, ordinarily G01 for maintenance work. |
| 5 | Hand the emergency requirement to the appropriate operational domain and end the G10 scenario before operational execution begins. | Infrastructure Management → applicable Section G operational owner | The frozen outcome is reached while execution remains outside G10. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Condition is not emergency-critical | Review establishes that the issue can be handled through ordinary corrective maintenance. | Stop the emergency path and use the separately governed corrective-maintenance requirement. | Emergency governance is not used for a non-emergency defect. | Infrastructure Management |
| Availability must be restricted | The critical condition also means Infrastructure may no longer be consumed safely or operationally. | Initiate the separately governed Infrastructure availability suspension/restriction outcome according to source-defined policy. | Emergency maintenance governance does not silently redefine the Availability Profile or state. | Infrastructure Management |
| Operational owner is not Facilities | The urgent condition belongs to another established Section G operational responsibility. | Hand execution to that canonical operational owner while retaining G10 Infrastructure ownership. | Execution follows existing ownership rather than a locally invented owner. | Applicable Section G operational domain |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Infrastructure reference cannot be resolved | The critical condition cannot be reliably associated with canonical physical Infrastructure. | Establish the authoritative G10 reference as part of emergency governance without creating a duplicate physical record. | Infrastructure Management |
| Operational owner cannot be resolved from locked architecture | G10 cannot safely transfer execution authority. | Preserve the emergency condition and governance requirement and escalate the ownership gap for architecture resolution; do not invent an executor. | Section G architecture authority |
| Canonical emergency event publisher is ambiguous | The business requirement can be recorded but its canonical event cannot be published without violating one-publisher governance. | Hold event publication pending upstream resolution of the duplicate event definition. | Section G architecture authority |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Emergency Maintenance addresses situations requiring immediate intervention, including electrical failure, water leakage, fire damage, structural damage, safety hazard and natural disaster. | Infrastructure Management | The trigger must represent a source-supported urgent Infrastructure condition. | L2-G10 §35 Emergency Maintenance Governance |
| Infrastructure owns physical Infrastructure; Facilities owns operational maintenance execution. | Infrastructure Management / Facilities Management | G10 records the condition and requirement but does not execute emergency maintenance. | L2-G10 §§1–6, 35; L2-G01 §§1–6, 22 |
| G01 is the single operational maintenance engine for institutional maintenance activities. | Facilities Management | Where emergency maintenance is required, G01 owns the operational response lifecycle. | L2-G01 §§1–6 |
| Any separate Infrastructure availability or lifecycle restriction must follow its own source-defined governance. | Infrastructure Management | Emergency maintenance does not automatically create an unlisted Infrastructure state. | L2-G10 lifecycle and availability governance |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | Existing source-defined lifecycle, operational status and availability | The emergency condition and governance requirement are recorded. The scenario does not itself invent or force a lifecycle transition. | Existing source-defined state remains until a separate authorized availability/lifecycle outcome changes it. | Infrastructure Management | L2-G10 lifecycle, availability and §35 governance |
| G01 Work Order / emergency-maintenance execution | Not progressed by G10 | The requirement may enter G01's operational lifecycle after hand-off. | Any execution state is owned and governed by G01. | Facilities Management | L2-G01 Work Order and Emergency Maintenance workflows |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No canonical publisher is asserted for `EmergencyMaintenanceDeclared` in this Draft. Both locked L2-G10 and locked L2-G01 define the exact event name `EmergencyMaintenanceDeclared`: G10 describes it as “Emergency maintenance required,” while G01 describes it as “Emergency response initiated.” Choosing one publisher or treating the two meanings as one event would silently alter locked architecture.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The source-supported business hand-off is from G10's Infrastructure emergency-governance requirement to the established Section G operational executor. For emergency maintenance, G01 owns the execution lifecycle. The reviewed sources do not define a unique exact named synchronous contract for this hand-off, and this Draft does not invent one.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Record an Infrastructure emergency-governance requirement | Infrastructure Administrator or Maintenance Coordinator within applicable authorized responsibility | The condition must satisfy the source-defined emergency-maintenance meaning. | Infrastructure Management |
| Execute emergency maintenance | Facilities roles or another established operational-domain role according to canonical ownership | Execution authority remains with the operational owner and is not created by G10. | Applicable Section G operational owner |

## 15.2 Separation of Duties, Overrides and Exceptions

The reviewed sources do not define one universal G10 emergency approval, override or separation-of-duty rule for all emergency Infrastructure conditions. This Draft therefore does not invent emergency authority or a break-glass business rule.

## 15.3 Business Audit and Evidence Obligations

The authoritative Infrastructure condition and G10 governance requirement must remain traceable separately from the operational execution record owned by G01 or another Section G executor. Event publication remains unresolved until the canonical `EmergencyMaintenanceDeclared` publisher is clarified.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The critical condition is associated with authoritative G10 Infrastructure.
- The emergency-maintenance governance requirement is recorded by G10.
- The applicable established operational executor is identified.
- Operational execution is handed off without G10 taking ownership of that execution.
- No unapproved Infrastructure lifecycle, availability or event-publisher decision has been invented.

## 16.2 Postconditions

- Infrastructure Management retains ownership of the physical Infrastructure and emergency condition/governance fact.
- The appropriate Section G operational owner retains execution responsibility.
- Any later maintenance completion and Infrastructure-condition reconciliation remains a separate governed outcome.
- Canonical event publication remains blocked pending upstream publisher resolution.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L2-G10 and locked L2-G01 both define the exact event name `EmergencyMaintenanceDeclared`, but with different business meanings (`Emergency maintenance required` vs `Emergency response initiated`). | SutraERP requires one canonical publisher for every business event. The Draft cannot identify a canonical publisher or safely collapse requirement declaration and response initiation into one fact. | Architecture change control must assign canonical publisher(s) and define distinct event semantics/names if both business facts are required. |
| The frozen outcome permits execution by the “appropriate operational domain,” while G10 states maintenance execution belongs to Facilities and does not enumerate a contract for non-Facilities emergency execution. | The primary emergency-maintenance path is clear for G01, but exceptional execution ownership cannot be generalized without source evidence. | Section G architecture should define any non-Facilities emergency Infrastructure execution hand-offs that are intended to be canonical. |

**Architecture assessment:** `BLOCKED`

The ownership boundary is clear enough to preserve the frozen outcome, but the duplicate canonical event definition materially prevents a source-complete emergency publication contract.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
