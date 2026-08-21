# Corrective Maintenance Governance to Infrastructure Remediation Requirement

# 1. Scenario Metadata

**Scenario ID:** L3-G-305  
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
**Referenced By:** Facilities Management and authorized Section G consumers of Infrastructure remediation requirements

# 2. Scenario Purpose

Govern an identified Infrastructure defect into a corrective-remediation requirement under G10 ownership so that the physical Infrastructure remains canonically owned by Infrastructure Management while operational maintenance execution is handed to Facilities Management.

# 3. Business Outcome

The infrastructure remediation requirement is governed by G10 while operational maintenance execution remains with G01

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an infrastructure defect requires corrective action.

## 4.2 End Boundary

The scenario ends when G10 has validated and recorded the corrective-remediation requirement against authoritative Infrastructure and has published the source-defined `CorrectiveMaintenanceRequested` fact for Facilities-owned execution.

## 4.3 In Scope

- Identification of the authoritative Infrastructure affected by the defect.
- Evaluation of the defect as a corrective-maintenance governance need.
- Recording of the G10 remediation requirement against the Infrastructure.
- Publication of `CorrectiveMaintenanceRequested` by Infrastructure Management.
- Hand-off of operational execution to G01 without transfer of physical-Infrastructure ownership.

## 4.4 Out of Scope

- Facilities Work Order creation, technician assignment, vendor assignment, repair execution, verification or closure.
- Preventive-maintenance or emergency-maintenance governance.
- Automatic Infrastructure lifecycle, availability or compliance-state change unless governed by a separate source-defined outcome.
- Creation of a new corrective-maintenance event or remediation lifecycle inside G10.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| May follow | L3-G-303 — Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome | Inspection findings are a source-defined trigger for corrective maintenance. |
| Followed by | L3-G-307 — Maintenance Completion to Infrastructure Condition Reconciliation | Completed corrective work may require G10 condition reconciliation. |
| Related | L3-G-306 — Emergency Infrastructure Condition to Emergency Maintenance Governance | Urgent safety or disruption conditions use the separate emergency governance outcome. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns physical Infrastructure, the Infrastructure-side corrective-maintenance planning/remediation requirement and the `CorrectiveMaintenanceRequested` business fact. |
| Facilities Management / Section G | Owns operational maintenance execution, Work Orders, technician/vendor execution, verification and closure after the requirement enters G01. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Coordinator | Coordinates the G10 corrective-remediation requirement against the affected Infrastructure. | May progress Infrastructure maintenance governance within authorized responsibility. |
| Infrastructure Administrator | Administers the affected Infrastructure and validates the G10 governance context. | May record or govern the remediation requirement within authorized scope. |
| Facilities Manager | Governs the resulting Facilities execution after hand-off. | Does not own physical Infrastructure or the G10 remediation requirement. |
| Maintenance Supervisor | Participates in later G01 work planning/execution. | Does not execute maintenance inside this G10 scenario boundary. |

# 7. Preconditions and Entry Criteria

- The affected physical Infrastructure exists under G10 ownership.
- A defect has been identified from a source-supported origin such as Inspection Findings, Maintenance Requests, Infrastructure Failure or Administrative Review.
- The corrective-maintenance requirement can be associated with the authoritative Infrastructure.
- Required business authorization has been satisfied under the platform authorization architecture.

# 8. Trigger

An infrastructure defect requires corrective action

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative Infrastructure and the source-supported defect requiring corrective action. | Infrastructure Management | The exact physical Infrastructure and defect context are established. |
| 2 | Validate that the condition is a corrective-maintenance governance need rather than preventive or emergency maintenance. | Infrastructure Management | The defect is placed on the correct G10 maintenance-governance path. |
| 3 | Record the Infrastructure-side remediation requirement without creating or owning a Facilities Work Order. | Infrastructure Management | The corrective-remediation requirement is governed against the canonical Infrastructure. |
| 4 | Publish `CorrectiveMaintenanceRequested` as the source-defined completed G10 request fact. | Infrastructure Management | Facilities and other authorized consumers can react to the governed remediation requirement. |
| 5 | Hand operational execution to Facilities Management and end the G10 scenario before repair execution begins. | Infrastructure Management → Facilities Management | G01 retains maintenance-execution ownership while G10 retains physical-Infrastructure ownership. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Defect is actually emergency-critical | Evaluation establishes an immediate intervention requirement. | Stop corrective governance and use the separate emergency-maintenance governance scenario. | The defect is not forced into a non-emergency path. | Infrastructure Management |
| Defect requires no maintenance execution | Authoritative review determines that the condition does not require corrective work. | Do not publish a corrective-maintenance request; preserve the reviewed condition and use any other applicable governed Infrastructure path. | No unsupported remediation requirement is asserted. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Infrastructure reference is invalid or unavailable | The remediation requirement cannot be reliably associated with physical Infrastructure. | Resolve the G10 Infrastructure reference before a later corrective-governance attempt. | Infrastructure Management |
| Defect information is insufficient to support corrective governance | A trustworthy remediation requirement cannot be established. | Resolve the business condition/finding before publishing a corrective request. | Infrastructure Management |
| Facilities execution has not yet begun | The G10 requirement remains valid but the operational resolution is incomplete. | Facilities governs the subsequent execution lifecycle; G10 does not take over execution. | Facilities Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Corrective Maintenance addresses defects identified after Infrastructure becomes operational. | Infrastructure Management | The scenario is used only for a source-supported corrective defect context. | L2-G10 §34 Corrective Maintenance Governance |
| Typical sources include Inspection Findings, Maintenance Requests, Infrastructure Failure and Administrative Review. | Infrastructure Management | The corrective requirement must originate from a supported business condition rather than an invented trigger category. | L2-G10 §34 |
| Corrective Maintenance planning belongs to Infrastructure; execution belongs to Facilities. | Infrastructure Management / Facilities Management | G10 governs the requirement and G01 owns operational work. | L2-G10 §34; L2-G01 §§1–6 |
| G01 owns Work Orders and operational maintenance execution. | Facilities Management | G10 cannot create or progress a G01 Work Order inside this scenario. | L2-G01 §§3, 5, 8, 18–22 |
| Physical Infrastructure ownership always remains within G10. | Infrastructure Management | Facilities execution cannot transfer canonical physical ownership. | L2-G10 §§1–6; L2-G01 §§1–4, 9 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | Existing source-defined lifecycle / operational condition | A corrective-remediation requirement is recorded; the scenario does not itself execute repair or assert a post-maintenance condition. | Infrastructure remains in its current source-defined state unless a separate lifecycle/availability outcome applies. | Infrastructure Management | L2-G10 lifecycle and §34 maintenance boundary |
| Facilities Work Order | Not created or progressed by G10 | The corrective request may lead to a G01 Work Order after hand-off. | Any Work Order lifecycle is outside this scenario. | Facilities Management | L2-G01 Work Order lifecycle |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `CorrectiveMaintenanceRequested` | Infrastructure Management | Facilities Management and other authorized Section G consumers | Publishes the source-defined G10 fact that corrective maintenance has been initiated as an Infrastructure remediation requirement. | L2-G10 Maintenance Events; §34 |

G01 separately defines `CorrectiveMaintenanceInitiated` for its operational corrective-work context. This Draft keeps the two source-defined events distinct and does not rename or merge them.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The completed G10 remediation requirement is handed to G01 for operational execution. G01 owns its resulting Work Order and maintenance lifecycle. The reviewed locked sources do not define a separate exact synchronous contract name for this hand-off; the source-defined `CorrectiveMaintenanceRequested` event provides the canonical business-fact signal used by this Draft.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Govern an Infrastructure corrective-remediation requirement | Maintenance Coordinator or Infrastructure Administrator within applicable authorized scope | The action must be grounded in a supported Infrastructure defect and remain within G10 maintenance-planning ownership. | Infrastructure Management |
| Execute corrective maintenance | Facilities roles under the G01 authorization model | G01 owns Work Order creation, assignment, execution, verification and closure. | Facilities Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal scenario-specific maker-checker or approval rule for the G10 corrective-remediation requirement is defined by the reviewed sources. G01 separately governs configurable operational maintenance approval and verification for its execution lifecycle.

## 15.3 Business Audit and Evidence Obligations

- The G10 remediation requirement and `CorrectiveMaintenanceRequested` fact remain traceable under Infrastructure Management ownership.
- G01 preserves its separate Work Order and maintenance-execution history.
- Historical Infrastructure ownership and maintenance-execution ownership must remain distinguishable.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authoritative affected Infrastructure and defect are identified.
- The condition is confirmed as a corrective-maintenance governance need.
- The G10 remediation requirement is recorded.
- `CorrectiveMaintenanceRequested` is published by Infrastructure Management.
- Operational execution is handed to G01 without G10 taking ownership of execution.

## 16.2 Postconditions

- Infrastructure Management retains ownership of the physical Infrastructure and remediation-governance fact.
- Facilities Management owns any subsequent corrective Work Order and execution lifecycle.
- Infrastructure condition reconciliation after completed maintenance remains a separate scenario.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking architecture issue was identified. L2-G10 explicitly assigns corrective-maintenance planning to Infrastructure, execution to Facilities, and defines the canonical `CorrectiveMaintenanceRequested` G10 event. L2-G01 separately owns Work Orders and operational corrective execution.

**Architecture assessment:** `PASS`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
