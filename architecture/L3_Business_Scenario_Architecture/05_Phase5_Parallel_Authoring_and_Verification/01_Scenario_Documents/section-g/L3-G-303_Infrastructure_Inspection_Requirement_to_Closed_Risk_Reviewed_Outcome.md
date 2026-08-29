# Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-G-303  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern the frozen end-to-end Infrastructure inspection outcome while preserving all unambiguous G10 inspection facts and explicitly recording the unresolved G10/G01 inspection-ownership contradiction.

# 3. Business Outcome

An Inspection Record is created against the applicable physical infrastructure, the authorised inspection is completed with findings recorded and the applicable infrastructure condition updated, and any significant condition is recorded with an authorised infrastructure risk or remediation path initiated before the inspection is closed.

The scenario is **BLOCKED** because locked G10 and G01 disagree materially on ownership of inspection scheduling/execution.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an infrastructure inspection becomes due or is authorised.

## 4.2 End Boundary

The intended end boundary is the frozen closed Inspection Record with findings, condition update and significant-condition follow-up established; however, the canonical ownership boundary for scheduling/execution must be resolved upstream before that end-to-end flow can be stated source-completely.

## 4.3 In Scope

- G10-owned Inspection Record and physical Infrastructure facts.
- Source-defined G10 inspection lifecycle and events.
- Findings and condition reconciliation supported by G10.
- Source-supported remediation initiation.
- Explicit recording of the inspection ownership conflict.

## 4.4 Out of Scope

- Locally assigning inspection scheduling/execution ownership between G10 and G01.
- Facilities Work Order, technician or vendor execution.
- Any invented Infrastructure Risk aggregate or hand-off contract.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Unambiguously owns physical Infrastructure and lists `Inspection Record` as a G10 aggregate; also defines an Infrastructure inspection workflow and events. |
| Facilities Management / G01 | Unambiguously owns operational maintenance execution and separately owns a Facilities `Inspection` aggregate, inspection planning/recording and operational inspection workflows. |
| Platform Foundation / A | Supplies authorization and event-governance capabilities only. |

The precise split for inspection scheduling/execution is unresolved by the locked sources.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the G10 Infrastructure/Inspection context where G10 ownership is unambiguous. | Cannot resolve the G10/G01 ownership conflict locally. |
| Compliance Officer | Participates where the inspection concerns Infrastructure compliance or safety. | No universal inspection approver is inferred. |

G10 uses the term `Inspector` in the workflow but does not define one universal canonical Inspector role/authority model. No actor is invented to fill that gap.

# 7. Preconditions and Entry Criteria

- The applicable physical Infrastructure exists.
- An inspection becomes due or is authorized.
- Applicable inspection criteria are available.
- The business operation is authorized.
- The unresolved scheduling/execution ownership conflict remains acknowledged.

# 8. Trigger

An infrastructure inspection becomes due or is authorised.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the canonical Infrastructure and inspection requirement. | Infrastructure Management | The target physical resource is resolved. |
| 2 | Establish the G10 Inspection Record and source-defined inspection context where ownership is unambiguous. | Infrastructure Management | G10 inspection governance is established. |
| 3 | Schedule, assign and conduct the inspection. | **BLOCKED — G10/G01 ownership conflict** | Locked sources do not permit one canonical owner to be selected locally. |
| 4 | Record findings and reconcile the applicable Infrastructure condition. | Infrastructure Management where G10 source facts apply | Findings/current condition are preserved. |
| 5 | Initiate a source-supported remediation or governed risk path for a significant condition before closure. | Infrastructure Management / applicable canonical owner | Significant findings are not silently closed. |
| 6 | Close the inspection under the source-defined lifecycle. | **BLOCKED pending ownership resolution** | End-to-end completion cannot be asserted source-completely until Step 3 ownership is resolved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| No significant finding | Findings do not require follow-up. | Preserve findings and proceed toward source-defined approval/closure after ownership conflict is resolved. | No remediation is invented. |
| Corrective maintenance required | Finding requires corrective action. | Establish the G10 corrective-maintenance requirement; G01 owns later maintenance execution. | Ownership boundary for maintenance remains intact. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| G10/G01 inspection ownership remains unresolved. | The scenario cannot state who canonically schedules/conducts the inspection. | Resolve through upstream Section G architecture change control. | Architecture governance |
| Significant finding lacks a canonical risk/remediation route. | Inspection cannot safely claim full frozen outcome. | Preserve the finding and use only an existing source-supported remediation route; escalate any missing risk contract. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 owns `Inspection Record` and defines an Infrastructure inspection workflow. | Infrastructure Management | G10 inspection semantics cannot be discarded. | L2-G10 §§8, 32 |
| G10 separately states Maintenance owns `Inspection Scheduling`. | Infrastructure Management / Maintenance boundary | This conflicts with G10's own end-to-end inspection workflow. | L2-G10 §9A.254 |
| G01 owns an `Inspection` aggregate, Inspection Planning/Recording and operational inspection workflows. | Facilities Management | G01 inspection ownership cannot be ignored. | L2-G01 §§3, 5, 8, 26 |
| G10 inspection lifecycle is `Scheduled → In Progress → Completed → Approved → Closed`. | Infrastructure Management | No extra inspection state is invented. | L2-G10 lifecycle summary |
| G10 events include `InfrastructureInspectionScheduled`, `FindingRecorded`, `InfrastructureInspectionCompleted`. | Infrastructure Management | Exact G10 event names are preserved only where G10 ownership is unambiguous. | L2-G10 Inspection Events |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| G10 Inspection Record | `Scheduled` after establishment | Intended to progress through G10 lifecycle. | `Closed` is the frozen target, but end-to-end ownership is unresolved. | Infrastructure Management subject to conflict | L2-G10 §32 |
| G01 Inspection | Source-defined G01 inspection lifecycle | Potentially overlaps scheduling/execution described by G10. | No local transition asserted. | Facilities Management | L2-G01 §26 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10 defines `InfrastructureInspectionScheduled`, `FindingRecorded` and `InfrastructureInspectionCompleted`. G01 separately defines Facilities inspection events for its own Inspection aggregate. Because the locked sources conflict on scheduling/execution ownership, this L3 Draft does not use event publication to silently choose the owner.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No exact canonical contract resolves the G10/G01 inspection scheduling/execution boundary. For later remediation, G10 may establish a corrective-maintenance requirement while G01 owns maintenance execution.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Authorization is required, but no universal named inspection approver or canonical actor resolves the ownership conflict.

## 15.2 Separation of Duties, Overrides and Exceptions

No universal separation-of-duty pattern is defined for all Infrastructure inspections. `Inspection Approved` is source-defined, but the canonical approving role is not universally specified.

## 15.3 Business Audit and Evidence Obligations

G10 requires inspection history to be preserved. Audit obligations do not resolve the ownership conflict.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen completion criteria cannot be source-completely satisfied until:
- the canonical owner of inspection scheduling/execution is resolved;
- any required significant-condition risk/remediation route is source-supported;
- the Inspection Record can validly reach `Closed`.

## 16.2 Postconditions

All unambiguous G10 Infrastructure/Inspection facts remain preserved. No G01 execution ownership is transferred to G10.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 owns an `Inspection Record` and defines a complete inspection workflow, but G10 §9A.254 says Maintenance owns Inspection Scheduling; G01 separately owns its own Inspection aggregate and inspection planning/recording workflows. | Material ownership and scenario-boundary conflict: L3 cannot canonically assign scheduling/execution. | Section G architecture governance must define the canonical distinction/boundary between G10 Infrastructure Inspection and G01 Facilities Inspection. |
| G10 has a `Critical Risk` finding classification but no G10 Infrastructure Risk aggregate or exact cross-context risk-referral contract. | Risk-only significant findings lack a fully canonicalized hand-off, although corrective-maintenance remediation is source-supported. | Define a canonical risk-referral contract if required. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
