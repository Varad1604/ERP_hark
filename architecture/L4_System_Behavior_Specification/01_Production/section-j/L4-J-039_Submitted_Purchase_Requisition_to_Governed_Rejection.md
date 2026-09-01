# Submitted Purchase Requisition to Governed Rejection

## 1. Specification Metadata
**Specification ID:** L4-J-039  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Purchase Requisition & Approval  
**Realizes Scenario:** L3-J-039  
**Primary Business Owner:** Purchase Requisition & Approval  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-J-039; L2-J03 — Purchase Requisition & Approval Workflows; applicable Locked L1/L2 architecture; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern rejection of a submitted Purchase Requisition when demand, validation, policy or authority requirements are not satisfied, ensuring the rejected demand remains auditable and cannot be represented as approved or released procurement demand.

## 3. Required Business Context
Required context includes the submitted Purchase Requisition, originating demand context, applicable policy and authority evidence, Finance-owned budget-validation reference where required, review evidence, reviewer/approver authority and tenant/institution context.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-039-BH-001 | The system shall admit a rejection decision only for an identifiable submitted Purchase Requisition under J03 governance with sufficient evidence for a governed review outcome. | L3-J-039 §§7–9 |
| L4-J-039-BH-002 | The system shall present the applicable demand, policy, authority and validation evidence for the governed rejection decision without modifying source-domain demand or Finance truth. | L3-J-039 §§5,9,12 |
| L4-J-039-BH-003 | The system shall establish rejection only from an authorized Requisition Reviewer / Approver decision and shall preserve the rejection reason and supporting evidence. | L3-J-039 §§6,9,15 |
| L4-J-039-BH-004 | When conclusive rejection is established, the system shall record the source-defined Rejected stage and publish `PurchaseRequisitionRejected`. | L3-J-039 §§9,13–14 |
| L4-J-039-BH-005 | A rejected requisition shall not be represented or released as approved procurement demand and shall not produce `PurchaseRequisitionApproved` or `PurchaseRequisitionReleasedForProcurement` for that rejected outcome. | L3-J-039 §§9,12,14,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-039-BH-006 | J03 shall remain owner of the requisition decision and state; Finance validation remains a referenced Finance-owned fact and the requesting domain retains originating-demand ownership. | L3-J-039 §§5,12,14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-039-BH-007 | If decision authority or required validation evidence is incomplete, the system shall not infer rejection or approval and shall retain the requisition under the last valid governed review position until a valid business decision exists. | L3-J-039 §11 |
| L4-J-039-BH-008 | If the authorized outcome is return for clarification or amendment rather than conclusive rejection, the system shall follow the applicable J03 continuation path and shall not publish `PurchaseRequisitionRejected`. | L3-J-039 §10 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-039-BH-009 | The system shall preserve the submitted requisition reference, originating demand context, validation references, reviewer authority, rejection reason/decision evidence and canonical rejection fact. | L3-J-039 §15 |

## 8. Observable Completion Conditions
Complete only when the requisition has a conclusive authorized rejection, `PurchaseRequisitionRejected` represents that actual outcome, no approved/released fact exists for the rejected outcome, and the decision remains traceable to its demand and review evidence.

## 9. Traceability & Provenance
**Parent:** L3-J-039 — Submitted Purchase Requisition to Governed Rejection  
**Primary verification source:** L2-J03 — Purchase Requisition & Approval Workflows.

## 10. Open Issues / Deferred Design
No material upstream issue blocks derivation. No rejected sub-state beyond the source-defined semantics is introduced. Technical workflow realization, interfaces, persistence, event transport and UI/UX remain deferred to L5/L6.
