# Capital Renovation Proposal to Infrastructure Change Governance

# 1. Scenario Metadata

**Scenario ID:** L3-G-308  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, J, F, B  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-J03 — Purchase Requisition & Approval Workflows; L2-J01 — Procurement Planning & Sourcing Workflows; L2-F06 — Budget & Cost Center Management; L2-B03 — Campus Workflows  
**Referenced By:** L3-G-309 — Capital Renovation Completion to Updated Physical Infrastructure

# 2. Scenario Purpose

Govern a proposed major physical-infrastructure renovation so its intended physical change and impact are recorded against canonical G10 Infrastructure while procurement, budget and InstitutionalCampus responsibilities remain with their canonical owners.

# 3. Business Outcome

The proposed infrastructure change is governed and its impact on physical infrastructure is recorded

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary
The scenario starts when a major infrastructure renovation is proposed.

## 4.2 End Boundary
The intended end boundary is reached when the proposal has passed the source-defined renovation-governance decision and its physical impact is recorded without beginning renovation execution.

## 4.3 In Scope
Physical-scope identification; G10 impact assessment; applicable J/F/B context; source-defined renovation approval.

## 4.4 Out of Scope
Renovation execution/completion; procurement execution; financial execution; InstitutionalCampus lifecycle change; invented renovation aggregates, events or approvers.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| G10 Infrastructure Management | Owns physical Infrastructure, renovation governance and physical-impact facts. |
| Section J | Owns any separately required Purchase Requisition, sourcing and procurement lifecycle. |
| Section F | Owns budget planning, allocation and validation truth. |
| Section B | Owns InstitutionalCampus identity/governance; physical campus remains G10-owned. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates or records the renovation proposal within authorized Infrastructure scope. | Source-supported G10 administration role. |
| Infrastructure Manager | Reviews physical impact and G10 governance context. | Does not acquire J/F/B ownership. |

The mandatory renovation approver is not identified by the reviewed locked sources.

# 7. Preconditions and Entry Criteria

- The affected physical Infrastructure exists under G10 ownership.
- The proposed major modification is identifiable.
- Applicable J/F/B references are available when relevant.
- The business operation is authorized.

# 8. Trigger

A major infrastructure renovation is proposed

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify affected canonical physical Infrastructure. | Infrastructure Management | Physical scope is fixed. |
| 2 | Validate hierarchy, lifecycle and applicable G10 constraints. | Infrastructure Management | Proposal is evaluated against authoritative rules. |
| 3 | Record proposed modification and expected physical impact. | Infrastructure Management | G10-owned impact is recorded without execution. |
| 4 | Preserve applicable InstitutionalCampus, procurement and budget context under B/J/F ownership. | G10 with B/J/F | Ownership remains separated. |
| 5 | Complete the source-defined renovation approval decision. | **Approving business authority unresolved upstream** | Mandatory governance step cannot be completed architecture-safely. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| No procurement requirement | No acquired resources are required. | Omit J transaction processing. | G10 proposal remains validly scoped. |
| No new budget action | Existing Finance context is sufficient. | Do not create/revise a budget. | Finance truth remains unchanged. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Physical impact cannot be established | Proposal cannot be governed reliably. | Resolve authoritative G10 facts before a later decision. | Infrastructure Management |
| Mandatory approval authority is unresolved | Governed outcome cannot complete. | Preserve Draft and obtain upstream authority resolution. | Architecture governance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 owns physical Infrastructure and capital-renovation governance. | Infrastructure Management | J/F/B participation does not transfer physical ownership. | L2-G10 §§1–6, 36 |
| G10 renovation flow contains `Renovation Proposal → Approval` before later execution stages. | Infrastructure Management | Approval cannot be bypassed. | L2-G10 §36 |
| J03/J01 retain requisition and sourcing ownership. | Section J | Procurement remains separate. | L2-J03; L2-J01 |
| Finance owns budget truth. | Section F | Budget references remain Finance-owned. | L2-F06 |
| B03 owns InstitutionalCampus identity while G10 owns physical Infrastructure. | B/G10 | Campus context does not alter physical ownership. | L2-B03 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | Existing G10 lifecycle state | Proposal/impact is evaluated; execution does not begin. | No renovation transition is asserted before mandatory approval. | G10 | L2-G10 §36 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events
G10 defines `RenovationStarted` and `RenovationCompleted` for later facts, but no exact canonical proposal-governance event. None is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs
J/F/B retain their source-owned contracts. No exact mandatory renovation-proposal cross-domain hand-off is defined by the reviewed locked sources.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate/record proposal | Infrastructure Administrator / Infrastructure Manager within authorized scope | Subject to G10 rules and platform authorization. | G10 |
| Approve renovation proposal | **Not defined upstream** | Mandatory G10 §36 step. | G10 decision; actor unresolved |

## 15.2 Separation of Duties, Overrides and Exceptions
No universal renovation SoD rule is defined; no approver is invented.

## 15.3 Business Audit and Evidence Obligations
Proposal, impact, source-domain references and eventual approval decision must remain traceable under their canonical owners.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria
- Canonical physical Infrastructure is identified.
- Proposed physical impact is recorded.
- J/F/B ownership is preserved.
- Mandatory renovation approval is completed by canonical authority.

The final criterion cannot currently be satisfied from locked upstream architecture.

## 16.2 Postconditions
No renovation execution begins; G10 remains physical owner; J/F/B retain their respective responsibilities.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 §36 requires Approval but does not identify the canonical approving business actor/role. | Material actor/authority gap blocks the governed proposal outcome. | Define approval authority or canonical policy-resolution contract upstream. |
| No exact proposal-governance event or mandatory G↔J/F/B renovation contract is defined. | Event/contract details cannot be asserted safely. | Define upstream if required. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions
No assumptions.

## 17.3 Deferred Questions
No additional deferred questions.
