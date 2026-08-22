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
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern an identified Infrastructure defect into a G10 corrective-remediation requirement while preserving G01 ownership of operational maintenance execution.

# 3. Business Outcome

The infrastructure remediation requirement is governed by G10 while operational maintenance execution remains with G01.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an infrastructure defect requires corrective action.

## 4.2 End Boundary

The scenario ends when the corrective-remediation requirement is established against canonical Infrastructure, `CorrectiveMaintenanceRequested` is published by G10, and execution is handed to G01.

## 4.3 In Scope

- Identification of canonical Infrastructure and defect.
- G10 corrective-maintenance planning/remediation requirement.
- Exact G10 event publication.
- Hand-off to G01 execution.

## 4.4 Out of Scope

- G01 Work Order creation/execution/verification/closure.
- Preventive or emergency maintenance governance.
- Automatic Infrastructure availability/lifecycle change.
- Any duplicate maintenance execution lifecycle in G10.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns physical Infrastructure, corrective-maintenance planning/remediation requirement and `CorrectiveMaintenanceRequested`. |
| Facilities Management / G01 | Owns Work Orders, technician/vendor execution, verification, closure and maintenance history. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Coordinator | Coordinates the G10 corrective-remediation requirement. | Does not execute the G01 Work Order. |
| Infrastructure Administrator | Administers the affected Infrastructure and governance context. | Remains within G10 ownership boundary. |
| Facilities Manager | Owns later operational corrective-maintenance execution. | Does not own physical Infrastructure. |

# 7. Preconditions and Entry Criteria

- Canonical Infrastructure exists.
- A source-supported defect exists.
- The condition is corrective rather than preventive/emergency.
- The remediation requirement can be linked to canonical Infrastructure.

# 8. Trigger

An infrastructure defect requires corrective action.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the canonical Infrastructure and source-supported defect. | Infrastructure Management | Corrective scope is established. |
| 2 | Validate that the defect belongs to corrective-maintenance governance. | Infrastructure Management | Correct maintenance category is confirmed. |
| 3 | Establish the G10 remediation requirement without creating a G01 Work Order. | Infrastructure Management | G10 governance outcome is recorded. |
| 4 | Publish `CorrectiveMaintenanceRequested`. | Infrastructure Management | Canonical G10 remediation fact is published. |
| 5 | Hand operational execution to Facilities Management and end the G10 scenario. | Infrastructure Management → Facilities Management | G01 execution ownership is preserved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Emergency-critical defect | Immediate intervention is required. | Use emergency-maintenance governance. | Corrective path is not misapplied. |
| No corrective work required | Authoritative review determines no repair execution is necessary. | Do not publish `CorrectiveMaintenanceRequested`. | No unsupported remediation request exists. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Canonical Infrastructure reference is unavailable. | Remediation cannot be reliably governed. | Resolve the G10 reference before a later request. | Infrastructure Management |
| Defect evidence is insufficient. | Corrective requirement cannot be established. | Resolve the source condition before publishing a request. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Corrective Maintenance addresses defects after Infrastructure becomes operational. | Infrastructure Management | Corrective path is limited to source-supported defects. | L2-G10 §34 |
| Corrective planning belongs to Infrastructure; execution belongs to Facilities. | G10 / G01 | G10 ends before operational work begins. | L2-G10 §34 |
| G01 owns Work Orders and maintenance execution. | Facilities Management | G10 cannot create/progress G01 Work Orders. | L2-G01 §§3, 5, 8, 18–22 |
| `CorrectiveMaintenanceRequested` is the G10 corrective-maintenance event. | Infrastructure Management | Exact event and publisher are source-supported. | L2-G10 Maintenance Events |
| G01's `CorrectiveMaintenanceInitiated` is a distinct operational execution fact. | Facilities Management | Events are not renamed or merged. | L2-G01 Maintenance Events |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | Existing source-defined state | Gains a corrective-remediation governance requirement; no repair is executed here. | Existing state remains unless another scenario changes it. | Infrastructure Management | L2-G10 §34 |
| G01 Work Order | Not created/progressed by G10 | May be created after hand-off under G01. | Outside this scenario. | Facilities Management | L2-G01 Work Order lifecycle |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Meaning |
|---|---|---|
| `CorrectiveMaintenanceRequested` | Infrastructure Management | G10 remediation requirement exists. |
| `CorrectiveMaintenanceInitiated` | Facilities Management | Later G01 operational corrective work has started. |

## 14.2 Business Contracts and Cross-Domain Hand-offs

The G10 remediation requirement is handed to G01 for execution. No ownership of physical Infrastructure transfers.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Source-supported G10 roles may govern the remediation requirement; source-supported G01 roles own later maintenance execution.

## 15.2 Separation of Duties, Overrides and Exceptions

No universal maker-checker rule is defined for the G10 remediation requirement.

## 15.3 Business Audit and Evidence Obligations

G10 remediation history and G01 execution history remain separately traceable.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Canonical Infrastructure and defect are identified.
- Corrective governance is valid.
- G10 remediation requirement is established.
- `CorrectiveMaintenanceRequested` is published by G10.
- G01 retains operational execution.

## 16.2 Postconditions

Infrastructure remains G10-owned; any Work Order/execution lifecycle remains G01-owned.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking source conflict was identified.

**Source Assessment:** **PASS**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.
