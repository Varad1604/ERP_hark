# Consultancy Request to Delivered and Financially Reconciled Outcome

## 1. Specification Metadata
**Specification ID:** L4-H-037  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Research  
**Realizes Scenario:** L3-H-037  
**Primary Business Owner:** Research  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-037; L2-H05 — Research Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`; `L2-H05_Consultancy_Cancellation_Governance_Amendment_2026-08-27.md`; Locked Finance architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall realize the Research-owned Consultancy lifecycle through `Closed` after delivery acceptance and Finance-confirmed reconciliation, or through the governed pre-delivery `Cancelled` path, without creating Finance truth.

## 3. Required Business Context
The Consultancy must use the parent-defined client/agreement/team context, Research Director authority, delivery acceptance, Finance reconciliation confirmation/reference and locked cancellation boundary.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-037-BH-001 | The system shall permit Consultancy progression only when the parent-defined approval, agreement and workforce prerequisites are valid. | L3-H-037 §§7–9 |
| L4-H-037-BH-002 | After authoritative delivery acceptance, the system shall require Finance reconciliation confirmation/reference before Research closure. | L3-H-037 §§9,12,16 |
| L4-H-037-BH-003 | The system shall apply only `Approved → Active → Delivery Submitted → Delivery Accepted → Financial Reconciliation Pending → Closed` on the successful path. | L3-H-037 §§12–13 |
| L4-H-037-BH-004 | The system shall permit `Cancellation Requested → Cancelled` only from the parent-supported pre-`Delivery Accepted` positions and only on the authoritative cancellation decision. | L3-H-037 §§10,12–13,15 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-037-BH-005 | The system shall consume Finance reconciliation truth only by authoritative Finance confirmation/reference and shall not create or alter Finance accounting facts. | L3-H-037 §§5,9,12,14 |
| L4-H-037-BH-006 | The system shall publish only the exact Research Consultancy events enumerated by the parent for delivery, reconciliation request, closure and cancellation. | L3-H-037 §14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-037-BH-007 | Missing Finance confirmation shall block closure; invalid or post-`Delivery Accepted` cancellation shall leave the current Consultancy state unchanged and preserve history. | L3-H-037 §11 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-037-BH-008 | The system shall preserve Research approval/execution/delivery/cancellation evidence and the authoritative Finance reconciliation reference used for closure. | L3-H-037 §15 |

## 8. Observable Completion Conditions
Complete only when the Consultancy reaches `Closed` after accepted delivery and required Finance confirmation, or reaches `Cancelled` through the governed pre-delivery branch, as defined by L3-H-037 §16.

## 9. Traceability & Provenance
**Parent:** L3-H-037 — Consultancy Request to Delivered and Financially Reconciled Outcome  
**Primary verification source:** L2-H05 — Research Workflows plus the locked 2026-08-26 and 2026-08-27 H05 amendments.

## 10. Open Issues / Deferred Design
The historical cancellation blocker is resolved upstream. No material L4 blocker remains. Finance mechanisms and all implementation design remain deferred.
