# Infrastructure Decommissioning Initiation to Historical Closure

# 1. Scenario Metadata

**Scenario ID:** L3-G-315  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** L3-G-316 — Infrastructure Archive Eligibility to Controlled Archival

# 2. Scenario Purpose

Govern permanent retirement of physical Infrastructure only after source-owned dependencies are resolved and mandatory decommissioning authority is established, while preserving G10 lifecycle ownership and historical identity.

# 3. Business Outcome

Decommissioning begins according to policy; dependent reservations, allocations, operational references and responsibilities are reviewed and resolved before decommissioning proceeds; and the infrastructure resource reaches its decommissioned state while its historical identity remains preserved

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary
The scenario starts when an authorised infrastructure resource is no longer intended for continued use.

## 4.2 End Boundary
The scenario requires all mandatory dependencies to be resolved, required approval to exist, the G10 resource to reach `Decommissioned`, and historical identity to remain preserved.

## 4.3 In Scope
G10 eligibility/dependency validation; D03 timetable-resource dependency impact; mandatory approval; `Operational → Decommissioned`; `InfrastructureDecommissioned`; historical preservation.

## 4.4 Out of Scope
Archival; physical deletion; G10 modification of D03-owned Timetable/Resource Reservation records; invented cancellation/resolution contracts or approvers.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| G10 | Owns physical Infrastructure, its lifecycle and decommissioning outcome. |
| Section A | Owns authorization and audit capability. |
| D03 Academic Delivery | Owns Timetable and its root-owned Resource Reservation records affected by the physical resource. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates decommissioning and G10 dependency validation. | Authorized G10 scope. |
| Infrastructure Manager | Reviews G10 physical/operational blockers. | G10 responsibility. |
| Infrastructure Coordinator | Reviews D03 scheduling/resource-reservation impact where applicable. | D03 responsibility; no G10 lifecycle ownership. |
| Auditor | Reviews evidence. | Read-only evidence role. |

The mandatory generic decommissioning approver is not identified upstream.

# 7. Preconditions and Entry Criteria

- The Infrastructure resource exists under G10 ownership and is no longer intended for use.
- Its current lifecycle permits decommissioning.
- Active dependencies are identifiable and must be resolved.
- Required decommissioning approval must exist before transition.
- The action is authorized/auditable.

# 8. Trigger

An authorised infrastructure resource is no longer intended for continued use

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify canonical G10 resource proposed for retirement. | G10 | Scope is fixed. |
| 2 | Validate lifecycle and parent/child constraints. | G10 | G10 blockers are identified. |
| 3 | Review active reservations, allocations, maintenance, compliance, inspection and other dependencies. | G10 | Retirement blockers are identified. |
| 4 | Identify affected D03 Timetable Resource Reservation/reference obligations. | D03 / G10 | Scheduling impact is identified without ownership transfer. |
| 5 | Resolve every mandatory dependency through its canonical owner. | Respective owner | Decommissioning preconditions would be satisfied. |
| 6 | Obtain mandatory decommissioning approval. | **Approving business authority unresolved upstream** | Mandatory decision cannot be assigned safely. |
| 7 | Transition G10 Infrastructure to `Decommissioned` and publish `InfrastructureDecommissioned`. | G10 | Retirement outcome would complete once blockers are resolved. |

Steps 5–6 cannot be proven architecture-safely for the full frozen outcome because the D03 hand-off and approval authority are unresolved.

# 10. Alternate Business Flows

No successful alternate path bypasses unresolved dependencies or mandatory approval.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Active dependency remains | Decommissioning cannot proceed. | Resolve through canonical owner first. | Canonical dependency owner |
| Parent has active child Infrastructure | Parent retirement cannot complete. | Resolve child lifecycle under G10 rules. | G10 |
| Approval is absent | Transition is blocked. | Obtain valid approval after authority is resolved upstream. | Unresolved authority |
| Authorization fails | Transition cannot proceed. | Preserve current state. | A / G10 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| WF-INF-013 requires dependency validation and Approval before decommissioning. | G10 | Dependencies/approval cannot be bypassed. | L2-G10 WF-INF-013 |
| G10 checks active reservations, maintenance, compliance, inspections and other dependencies. | G10 | Active blockers must be resolved. | L2-G10 §41 |
| `Operational → Decommissioned` is permitted; historical identity remains visible and unavailable for future use. | G10 | Successful outcome is controlled retirement, not deletion. | L2-G10 §11A / BR-OP-05 |
| D03 owns Timetable and root-owned Resource Reservation records. | D03 | G10 cannot directly resolve D03 reservations. | L2-D03 aggregate ownership |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | `Operational` | Permanent retirement after valid dependency resolution/approval. | `Decommissioned` | G10 | L2-G10 §11A; WF-INF-013 |
| D03 Timetable / Resource Reservation | Existing D03 state | Affected scheduling dependency must be resolved by D03. | Exact required D03 resolution state/contract is not defined for G10 decommissioning. | D03 | L2-D03; L2-G10 §41 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events
`InfrastructureDecommissioned` is canonically published by G10 after successful lifecycle transition. No D03 event is defined as the mandatory reservation-resolution confirmation for this scenario.

## 14.2 Business Contracts and Cross-Domain Hand-offs
G10 physical identity/context can identify affected D03 reservations. The exact D03→G10 confirmation proving those source-owned reservations no longer block decommissioning is not defined and is not invented.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate decommissioning | Infrastructure Administrator | Authorized G10 scope. | G10 |
| Resolve D03 reservation/reference dependency | D03 source-owned responsibility | D03 retains ownership. | D03 |
| Approve decommissioning | **Not defined upstream** | Approval is mandatory. | G10 decision; actor unresolved |

## 15.2 Separation of Duties, Overrides and Exceptions
No universal generic decommissioning approver/SoD rule is canonically assigned.

## 15.3 Business Audit and Evidence Obligations
Preserve lifecycle transition, dependency validation, approval outcome, reason/context and historical references; Platform Audit owns immutable audit evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria
Eligibility established; all dependencies resolved through owners; mandatory approval completed by canonical authority; G10 reaches `Decommissioned`; `InfrastructureDecommissioned` published; history preserved. Dependency-resolution contract and approval authority remain unresolved.

## 16.2 Postconditions
No physical deletion; Infrastructure remains historically visible under G10; future operational use is prohibited; archival is separate.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 reservation/dependency wording overlaps D03 ownership of Timetable Resource Reservation, and no decommissioning resolution-confirmation contract is defined. | Material ownership/mandatory hand-off gap. | Reconcile reservation concepts and define authoritative cross-domain resolution contract. |
| G10 requires Approval but does not assign the generic decommissioning approving business actor/role. | Material authority gap. | Define approver resolution/authority upstream. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions
No assumptions.

## 17.3 Deferred Questions
No additional deferred questions beyond blocking issues.
