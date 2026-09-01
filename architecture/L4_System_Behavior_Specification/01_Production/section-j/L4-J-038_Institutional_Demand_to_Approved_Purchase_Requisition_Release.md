# Institutional Demand to Approved Purchase Requisition Release

## 1. Specification Metadata
**Specification ID:** L4-J-038  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Purchase Requisition & Approval  
**Realizes Scenario:** L3-J-038  
**Primary Business Owner:** Purchase Requisition & Approval  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-J-038; L2-J03 — Purchase Requisition & Approval Workflows; applicable Locked L1/L2 architecture; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall convert an authorized institutional demand into a governed Purchase Requisition that is submitted, validated, approved and released for downstream procurement while preserving requesting-domain, Finance and J03 ownership boundaries.

## 3. Required Business Context
Required context includes the authoritative requesting-domain demand, purpose and organizational scope, requested items and quantities, requester identity/authority, applicable policy and approval context, Finance-owned budget-validation reference where required, and tenant/institution context.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-038-BH-001 | The system shall establish the Purchase Requisition only from identifiable institutional demand supplied by an authorized requesting context and shall preserve the source-demand association. | L3-J-038 §§5–9 |
| L4-J-038-BH-002 | The system shall require sufficient purpose, scope, item and quantity information before treating the requisition as submitted and shall record `PurchaseRequisitionSubmitted` only when submission is complete. | L3-J-038 §§7–9,14 |
| L4-J-038-BH-003 | Where policy requires budget validation, the system shall consume the authoritative Finance-owned budget-validation reference without altering or recreating Finance budget truth. | L3-J-038 §§5,9,12 |
| L4-J-038-BH-004 | The system shall evaluate the requisition against applicable policy, authorization and segregation-of-duties constraints, including `Requester != Final Approver`, before approval. | L3-J-038 §§6,9,12,15 |
| L4-J-038-BH-005 | The system shall establish approval only from the governed Requisition Reviewer / Approver decision and shall record `PurchaseRequisitionApproved` only when that approval fact exists. | L3-J-038 §§6,9,14 |
| L4-J-038-BH-006 | The system shall release the approved requisition for procurement only after all parent-defined release conditions are satisfied and shall record `PurchaseRequisitionReleasedForProcurement` when release occurs. | L3-J-038 §§9,14,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-038-BH-007 | J03 shall remain canonical owner of the Purchase Requisition and its business state; requesting domains retain demand-context ownership, Finance retains budget truth, and J01/J04 consume released demand without ownership transfer. | L3-J-038 §§5,12,14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-038-BH-008 | Missing demand information, failed required validation, insufficient authority or incomplete approval shall prevent release; the system shall retain the last valid J03-governed position and continue only through a parent-supported correction, return, rejection, withdrawal or cancellation path. | L3-J-038 §§10–11 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-038-BH-009 | The system shall preserve evidence of demand provenance, requisition creation/submission, validation references, policy/authorization outcomes, approval authority and decision, release outcome, tenant/institution context and canonical event facts. | L3-J-038 §15 |

## 8. Observable Completion Conditions
Complete only when an authorized Purchase Requisition is approved and released for procurement, required validations and SoD controls are evidenced, the canonical submitted/approved/released facts correspond to actual business outcomes, and J03 remains the authoritative requisition owner.

## 9. Traceability & Provenance
**Parent:** L3-J-038 — Institutional Demand to Approved Purchase Requisition Release  
**Primary verification source:** L2-J03 — Purchase Requisition & Approval Workflows.

## 10. Open Issues / Deferred Design
No material upstream issue blocks derivation. Technical workflow realization, interfaces, persistence, integration transport, access-control implementation and UI/UX remain deferred to L5/L6.
