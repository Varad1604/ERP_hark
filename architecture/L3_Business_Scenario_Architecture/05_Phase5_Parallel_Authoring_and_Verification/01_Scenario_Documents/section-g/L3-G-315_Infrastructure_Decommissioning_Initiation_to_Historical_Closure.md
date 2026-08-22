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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** L3-G-316 — Infrastructure Archive Eligibility to Controlled Archival

# 2. Scenario Purpose

Govern permanent retirement of an authorized physical Infrastructure resource while ensuring active dependencies are resolved, the canonical G10 lifecycle reaches `Decommissioned`, and historical identity remains preserved without transferring ownership to consuming domains.

# 3. Business Outcome

Decommissioning begins according to policy; dependent reservations, allocations, operational references and responsibilities are reviewed and resolved before decommissioning proceeds; and the infrastructure resource reaches its decommissioned state while its historical identity remains preserved

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised infrastructure resource is no longer intended for continued use.

## 4.2 End Boundary

The scenario ends when all mandatory decommissioning dependencies are resolved, the G10-owned Infrastructure resource reaches `Decommissioned`, operational use is prohibited, and historical identity/references remain preserved.

## 4.3 In Scope

- G10 decommissioning eligibility and dependency validation.
- Review/resolution of active Infrastructure reservations, allocations, child dependencies, operational references and responsibilities required before retirement.
- Collaboration with Academic Delivery where timetable-owned resource reservations/reference obligations are affected.
- Required authorization, approval and audit obligations defined upstream.
- `Operational → Decommissioned` lifecycle transition and `InfrastructureDecommissioned` event where applicable to the generic Infrastructure lifecycle.

## 4.4 Out of Scope

- Archival after decommissioning.
- Physical deletion of Infrastructure.
- Re-owning Academic Delivery timetable/reservation records in G10.
- Inventing a cross-domain cancellation/revision contract not defined by the locked sources.
- Assigning a decommissioning approver where upstream does not identify one.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical Infrastructure resource, its lifecycle, Infrastructure allocations/responsibilities and decommissioning decision/outcome. |
| Platform Foundation / Section A | Owns authorization and audit infrastructure used by the governed decommissioning action. |
| Academic Delivery / Section D | Owns the canonical Timetable and its D03-owned `Resource Reservation` records/references that may depend on the Infrastructure resource. |

No participation transfers ownership between G10 physical Infrastructure and D03 timetable records.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates decommissioning and performs G10 dependency validation within authorized scope. | Initiates/coordinates G10 retirement steps. |
| Infrastructure Manager | Reviews Infrastructure operational dependencies, allocations and responsibilities. | Reviews G10-owned physical/operational facts. |
| Infrastructure Coordinator | Reviews Academic Delivery infrastructure allocation/resource-reservation impact where timetable records are affected. | Acts within D03 scheduling responsibility; does not own G10 lifecycle. |
| Auditor | Reviews preserved decommissioning evidence. | Read-only evidence role. |

The canonical business actor authorized to give the mandatory decommissioning approval is not identified by G10 and is not invented.

# 7. Preconditions and Entry Criteria

- The Infrastructure resource exists under G10 ownership and is not intended for continued use.
- The resource is in a lifecycle position from which decommissioning is permitted by the canonical G10 lifecycle.
- Active operational dependencies must be identifiable and resolvable before decommissioning.
- Required decommissioning approval must be completed before the transition.
- The action must be authorized and auditable.

# 8. Trigger

An authorised infrastructure resource is no longer intended for continued use

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the canonical G10-owned Infrastructure resource proposed for permanent retirement. | Infrastructure Management | Decommissioning scope is established. |
| 2 | Validate that the current G10 lifecycle permits decommissioning and evaluate child Infrastructure where applicable. | Infrastructure Management | Lifecycle/parent-child constraints are known. |
| 3 | Review G10-owned active reservations, allocations, operational responsibilities, compliance/inspection/maintenance dependencies and other retirement blockers. | Infrastructure Management | G10-owned blockers are identified. |
| 4 | Identify affected D03 timetable resource reservations/references that depend on the physical resource. | Academic Delivery / Infrastructure Management | Cross-domain scheduling impact is identified without ownership transfer. |
| 5 | Resolve all mandatory dependencies through their canonical owners before G10 proceeds. | Respective canonical owners | Decommissioning precondition “no active operational dependencies” is satisfied only after authoritative resolution. |
| 6 | Obtain the mandatory decommissioning approval required by G10. | **Approving business authority unresolved upstream** | Approval must exist before lifecycle transition. |
| 7 | Transition the G10 Infrastructure lifecycle from `Operational` to `Decommissioned`. | Infrastructure Management | Resource is permanently removed from operational availability. |
| 8 | Publish `InfrastructureDecommissioned` and preserve historical identity/references and audit evidence. | Infrastructure Management / Platform Audit | Historical closure is preserved. |

Steps 5–6 cannot be completed architecture-safely for all frozen dependencies because upstream ownership/contract and approval-authority gaps remain unresolved.

# 10. Alternate Business Flows

No complete alternate path is authored. If active dependencies remain, decommissioning is a blocked/interrupted path under Section 11 rather than a valid alternate path to the same outcome.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Active reservation or operational dependency remains | Decommissioning cannot proceed. | Resolve/cancel/end the dependency through its canonical owner before retrying the decommissioning decision. | Canonical dependency owner |
| Parent Infrastructure has active child Infrastructure | Parent decommissioning cannot proceed safely. | Evaluate and transition dependent children according to G10 rules. | Infrastructure Management |
| Mandatory approval is not complete | Lifecycle transition is blocked. | Obtain valid approval once the canonical approver is defined/resolved upstream. | **Unresolved authority** |
| Authorization fails | Decommissioning cannot proceed. | Preserve current Infrastructure state. | Platform Foundation / Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Generic decommissioning requires no active operational dependencies and completed approval. | Infrastructure Management | Active dependencies and missing approval block retirement. | L2-G10 WF-INF-013 |
| Decommissioning validates no active reservations, pending maintenance, active compliance actions, active inspections or other preventing dependencies. | Infrastructure Management | These dependencies must be resolved before transition. | L2-G10 §41 |
| Parent decommissioning requires evaluation of dependent children. | Infrastructure Management | Parent-child lifecycle consistency must be preserved. | L2-G10 LR-06 |
| `Operational → Decommissioned` is a permitted canonical lifecycle transition. | Infrastructure Management | Successful retirement reaches only the canonical `Decommissioned` state. | L2-G10 §11A.53 |
| Decommissioned Infrastructure remains historically visible but unavailable for future operational use. | Infrastructure Management | Historical identity/references are preserved. | L2-G10 BR-OP-05 |
| D03 owns the Timetable and its `Resource Reservation` root-owned record. | Academic Delivery | G10 cannot directly modify D03 scheduling reservations. | L2-D03 §§6.4, 17.11 |
| Every business operation must be authorized before execution. | Platform Foundation — Authorization | Unauthorized decommissioning is prohibited. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | `Operational` | Permanent retirement after dependency validation and required approval. | `Decommissioned` | Infrastructure Management | L2-G10 §11A.53; §41 |
| D03 Timetable / Resource Reservation | Existing D03 timetable lifecycle/reservation state | Any affected scheduling reservation/reference must be resolved by D03, not by G10. | **Required resulting D03 state/action is not defined by the G10↔D03 decommissioning hand-off.** | Academic Delivery | L2-D03 §§6.4, 17.11; L2-G10 §41 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `InfrastructureDecommissioned` | Infrastructure Management | Consumers are not canonically enumerated for this scenario | Announces completed G10 decommissioning after the lifecycle transition. | L2-G10 WF-INF-013 / §44 |

No D03 event or G10 event is defined as the mandatory pre-decommissioning reservation-resolution hand-off, so none is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Infrastructure Management | Academic Delivery | Physical Infrastructure identity, lifecycle/availability context | Identify timetable reservations/references affected by planned retirement. | L2-G10 public read contracts; L2-D03 consumes G10 physical resources |
| Academic Delivery | Infrastructure Management | **Mandatory reservation-resolution confirmation is required by the frozen outcome but no exact authoritative hand-off is defined.** | Prove D03-owned resource reservations no longer block decommissioning. | Gap between L2-G10 §41 and L2-D03 reservation ownership |
| Platform Foundation | Infrastructure Management | Authorization decision and audit capability | Govern decommissioning action/evidence. | L2-A04; L2-A07 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate decommissioning | Infrastructure Administrator | Must act within authorized Infrastructure scope. | Infrastructure Management |
| Confirm D03 scheduling dependency resolution | Infrastructure Coordinator / Academic Delivery responsibility | D03 retains ownership of timetable reservations/references. | Academic Delivery |
| Approve decommissioning | **Not defined upstream** | Approval is mandatory in G10 but the canonical approving business role is not identified. | Infrastructure Management decision; actor unresolved |

## 15.2 Separation of Duties, Overrides and Exceptions

G10 identifies decommissioning of critical Buildings as an example of an operation that may require elevated governance approval, but no universal approver or separation-of-duty rule is canonically assigned. None is invented.

## 15.3 Business Audit and Evidence Obligations

Decommissioning must preserve lifecycle-transition evidence, dependency-validation outcome, required approval outcome, reason/context and historical references. Platform Audit owns immutable audit evidence; G10 owns the decommissioning business fact.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Infrastructure resource is validly eligible for decommissioning.
- All active reservations, allocations, operational references/responsibilities and other mandatory dependencies are resolved through their canonical owners.
- Required decommissioning approval is complete under authoritative business authority.
- G10 transitions the resource to `Decommissioned`.
- `InfrastructureDecommissioned` is published by G10.
- Historical identity/references remain preserved and the resource is unavailable for future operational use.

The first, fourth, fifth and sixth criteria are source-grounded, but the cross-domain reservation-resolution hand-off and approval authority remain unresolved.

## 16.2 Postconditions

- No physical deletion occurs.
- Infrastructure remains historically available under G10 ownership.
- New operational use is prohibited.
- Archival, if later eligible, is a separate lifecycle outcome.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 states that Infrastructure owns reservation governance and requires no active reservations before decommissioning, while locked L2-D03 explicitly owns timetable `Resource Reservation` records and resource-reservation governance. | Material ownership/contract conflict for the frozen requirement to review and resolve dependent reservations before decommissioning. L3 cannot choose one owner or invent the synchronization/closure hand-off. | Upstream architecture must reconcile G10 Infrastructure Reservation versus D03 Timetable Resource Reservation ownership and define the decommissioning dependency-resolution contract. |
| G10 requires approval before decommissioning but does not assign the canonical approving business actor/role for the generic Infrastructure decommissioning workflow. | Material business-authority gap: L3 cannot state who may make the mandatory retirement decision. | Upstream G10/Workflow governance must define the approver-resolution authority or the applicable policy contract. |

**Architecture assessment:** `BLOCKED`

The unresolved reservation ownership/mandatory hand-off and mandatory approval authority materially affect ownership, contract and actor-authority correctness.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions beyond the upstream blocking issues.
