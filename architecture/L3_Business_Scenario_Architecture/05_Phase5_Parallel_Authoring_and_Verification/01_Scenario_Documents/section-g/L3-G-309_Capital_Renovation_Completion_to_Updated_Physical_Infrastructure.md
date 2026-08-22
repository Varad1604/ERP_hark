# Capital Renovation Completion to Updated Physical Infrastructure

# 1. Scenario Metadata

**Scenario ID:** L3-G-309  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, J, F, B  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-J00 — Enterprise Resource Management Architecture & Governance; L2-F06 — Budget & Cost Center Management; L2-B03 — Campus Workflows  
**Referenced By:** Authorized consumers requiring the post-renovation authoritative physical Infrastructure state

# 2. Scenario Purpose

Reconcile an authorized completed capital renovation into the canonical physical Infrastructure model so that affected hierarchy, attributes, capacity or capabilities are updated with historical traceability and the physical resource returns to its authorized operational position only after the source-defined inspection and compliance conditions are satisfied.

# 3. Business Outcome

The affected physical infrastructure hierarchy, attributes, capacity, or capability is updated with historical traceability

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised capital renovation is completed.

## 4.2 End Boundary

The scenario ends when the completed renovation has been reconciled into the authoritative G10 physical Infrastructure facts, required inspection/compliance validation has been satisfied, historical identity remains traceable, and the affected Infrastructure is returned to its authorized operational position where the G10 lifecycle permits it.

## 4.3 In Scope

- Confirmation that the authorized renovation work has completed for the affected physical Infrastructure.
- Reconciliation of the physical hierarchy, attributes, capacity or capability that actually changed.
- Preservation of unchanged physical identity where G10 rules require continuity.
- Inspection and compliance validation required by the G10 renovation workflow before Operational restoration.
- Publication of applicable G10 lifecycle facts such as `RenovationCompleted` and `InfrastructureOperational` when those transitions occur.
- Reference to J/F/B outcomes only to the extent their canonical facts are relevant to reconciliation.

## 4.4 Out of Scope

- Renovation execution itself.
- Procurement, goods receipt, inventory or asset-management execution.
- Budget revision, accounting, capitalization, payment or financial posting.
- InstitutionalCampus lifecycle change.
- Creation of a new physical Infrastructure identity where the real-world physical identity remains unchanged.
- Invented renovation-completion contracts between G, J, F or B.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-G-308 — Capital Renovation Proposal to Infrastructure Change Governance | The proposal-governance outcome precedes authorized execution and later completion. |
| Related | L3-G-298 — Infrastructure Validation to Physical Consistency Confirmation | Reconciled physical facts remain subject to G10 validation rules. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the physical hierarchy, attributes, capacity, capabilities, lifecycle and historical Infrastructure truth being reconciled. |
| Enterprise Resource Management / Section J | Owns any procurement, received resource, stock or operational-asset facts arising from separately executed resource workflows; it does not own the physical Infrastructure model. |
| Finance / Section F | Owns budget/financial truth arising from separately governed financial processes; it does not own the physical renovation result. |
| Institution & Structure / Section B | Owns InstitutionalCampus identity/governance context and does not own the physical hierarchy or Infrastructure lifecycle. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Reconciles authorized completed renovation results into G10-owned physical Infrastructure within scope. | Must satisfy authorization and G10 validation/lifecycle rules. |
| Infrastructure Manager | Reviews the affected physical facts and authorized restoration position. | Does not own execution, procurement or financial transactions. |
| Compliance Officer | Participates where the G10 renovation workflow requires compliance validation. | No universal approval authority beyond source-defined responsibility is inferred. |
| Auditor | Reviews historical reconstruction and evidence. | Read-only participation. |

No J, F or B renovation-completion actor is canonically specified by the reviewed sources; none is invented.

# 7. Preconditions and Entry Criteria

- The renovation was authorized and has completed outside this scenario's execution boundary.
- The affected physical Infrastructure remains identifiable under G10 ownership.
- The pre-renovation physical facts and historical identity are available for reconciliation.
- Required inspection and compliance information for post-renovation validation is available or can be obtained through their governed paths.
- The reconciliation operation is authorized.

# 8. Trigger

An authorised capital renovation is completed

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the completed authorized renovation and the canonical physical Infrastructure affected. | Infrastructure Management | The reconciliation scope is fixed. |
| 2 | Compare the completed physical result with the authoritative pre-renovation hierarchy, attributes, capacity and capabilities. | Infrastructure Management | Actual physical changes are distinguished from unchanged facts. |
| 3 | Preserve existing Infrastructure identity wherever physical identity remains unchanged. | Infrastructure Management | Historical continuity is maintained. |
| 4 | Apply only the source-supported physical hierarchy, attribute, capacity or capability changes resulting from the completed renovation. | Infrastructure Management | G10 becomes authoritative for the updated physical facts. |
| 5 | Complete the G10-required inspection and compliance validation for the renovated Infrastructure. | Infrastructure Management | Post-renovation physical readiness is evaluated. |
| 6 | Where the lifecycle transition applies, complete the renovation state and restore the affected Infrastructure to `Operational` only when G10 rules permit. | Infrastructure Management | The physical lifecycle is reconciled. |
| 7 | Publish the applicable completed G10 lifecycle facts. | Infrastructure Management | Authorized consumers can resolve the completed renovation and operational restoration. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Renovation changed only attributes/capabilities | No hierarchy or identity change occurred. | Update only the affected authoritative facts and preserve the existing hierarchy/identity. | No artificial restructuring occurs. | Infrastructure Management |
| Renovation changed physical hierarchy | Authorized physical structure changed. | Apply the relevant G10 hierarchy rules and preserve historical traceability of the prior structure. | Canonical hierarchy reflects the completed physical reality. | Infrastructure Management |
| Inspection/compliance validation not yet satisfactory | The completed work cannot yet support Operational restoration. | Preserve the reconciled physical facts as permitted but do not assert an unauthorized Operational outcome. | Infrastructure remains in the applicable non-operational G10 position until separately resolved. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Completed physical result conflicts with G10 hierarchy/validation rules | Authoritative reconciliation cannot complete. | Resolve the physical inconsistency through the appropriate governed Infrastructure path before Operational restoration. | Infrastructure Management |
| Required inspection or compliance condition is unresolved | Return to Operational cannot be confirmed. | Keep the Infrastructure in the applicable controlled lifecycle/availability position until the requirement is resolved. | Infrastructure Management |
| J/F/B source facts are unavailable | Their separate resource, financial or institutional context cannot be referenced reliably. | Do not infer those facts; complete only the G10-owned reconciliation that is source-supported. | Respective canonical domain |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Capital renovation is governed by G10 and preserves historical physical identity where identity remains unchanged. | Infrastructure Management | Reconciliation updates facts without creating a new identity by default. | L2-G10 §36 |
| The G10 renovation workflow requires Inspection and Compliance Validation before Infrastructure Operational. | Infrastructure Management | Operational restoration cannot be asserted early. | L2-G10 §36 |
| Infrastructure lifecycle transitions must follow the approved lifecycle and remain auditable. | Infrastructure Management | Renovation completion/restoration follows G10 lifecycle rules. | L2-G10 §11A, §47 |
| Section J owns physical-resource procurement/inventory/asset truth but not Campus Operations' operational outcome. | Enterprise Resource Management | Renovation reconciliation cannot absorb Section J aggregates. | L2-J00 §§3.2, 5, 7 |
| Finance owns budget/financial truth. | Finance | Physical reconciliation does not perform financial accounting. | L2-F06 §§1–5 |
| B03 owns InstitutionalCampus identity; G10 owns physical hierarchy, sustainability, criticality and lifecycle. | Institution & Structure / Infrastructure Management | Institutional and physical campus facts remain separate. | L2-B03 §§1–4 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure aggregate | `Under Renovation` where the G10 lifecycle applies | Completed renovation is reconciled and, after required validation, the lifecycle may return to `Operational`. | `Operational` when source-defined conditions are met; otherwise the applicable current G10 state remains. | Infrastructure Management | L2-G10 §11A, §36 |
| Physical hierarchy/attributes/capacity/capability | Pre-renovation authoritative facts | Updated to reflect the completed physical outcome with history preserved. | New authoritative current facts; prior history retained. | Infrastructure Management | L2-G10 §§8, 18–25, 36 |
| InstitutionalCampus | Existing B03 state | Referenced only. | Unchanged. | Institution & Structure | L2-B03 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `RenovationCompleted` | Infrastructure Management | Authorized consumers | Records completion of the G10 renovation lifecycle fact. | L2-G10 §11A.58 |
| `InfrastructureOperational` | Infrastructure Management | Authorized consumers | Records return to the G10 Operational lifecycle state when the post-renovation conditions are satisfied. | L2-G10 §11A.58 |

Other attribute-specific G10 events may apply only when their corresponding owned fact changes; this Draft does not assert them universally.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The reviewed J/F/B sources establish separate canonical ownership for procurement/resource, financial/budget and InstitutionalCampus facts. No exact mandatory completion hand-off among these domains is defined by G10 §36, so the scenario references their authoritative outcomes where available without inventing a contract.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Reconcile completed renovation into canonical physical Infrastructure | Infrastructure Administrator or Infrastructure Manager within authorized scope | Must satisfy G10 validation/lifecycle constraints and applicable authorization. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

The source renovation workflow contains approval, inspection and compliance stages but does not define one universal actor separation rule for this scenario. None is invented.

## 15.3 Business Audit and Evidence Obligations

The reconciliation must preserve the pre-renovation and post-renovation physical facts, effective change history, lifecycle transition evidence, inspection/compliance evidence and actor/authority attribution required by the applicable G10 and platform audit rules.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The completed renovation is linked to the authoritative physical Infrastructure.
- Actual affected hierarchy, attributes, capacity or capabilities are reconciled.
- Historical physical identity is preserved where required.
- Required inspection and compliance validation is satisfied for any Operational restoration.
- Applicable G10 lifecycle events are published only for completed facts that actually occurred.

## 16.2 Postconditions

- G10 exposes the updated authoritative physical Infrastructure facts.
- Historical reconstruction remains possible.
- J, F and B continue to own their separate resource, financial and institutional-campus facts.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Stage 3 freezes J, F and B participation, while G10 §36 provides no exact renovation-completion contracts to those domains. | The ownership boundary is clear but the precise collaboration contract is not. | Define explicit cross-domain completion contracts upstream if required. |
| G10 allows multiple attribute-specific event names depending on the physical fact changed. | A single universal post-renovation attribute event cannot be asserted. | Consumers should rely on the applicable canonical G10 event for each fact actually changed. |

**Architecture assessment:** `PASS WITH ISSUES`

The core completion outcome and lifecycle/event semantics are source-grounded; remaining issues are non-blocking contract granularity gaps.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
