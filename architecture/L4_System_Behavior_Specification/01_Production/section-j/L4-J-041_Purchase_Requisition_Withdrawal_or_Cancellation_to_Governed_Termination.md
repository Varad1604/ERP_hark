# Purchase Requisition Withdrawal or Cancellation to Governed Termination

## 1. Specification Metadata
**Specification ID:** L4-J-041  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Purchase Requisition & Approval  
**Realizes Scenario:** L3-J-041  
**Primary Business Owner:** Purchase Requisition & Approval  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-J-041; L2-J03 — Purchase Requisition & Approval Workflows; applicable Locked L1/L2 architecture; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern withdrawal or cancellation of a Purchase Requisition so the terminated demand cannot subsequently be released as active procurement demand, while preserving J03 ownership and complete reconstructable requisition history without reversing downstream domain-owned facts.

## 3. Required Business Context
Required context includes the Purchase Requisition and its current governed position, withdrawal request or authorized cancellation decision, termination basis and rationale, requesting/decision authority, prior validation and approval evidence, originating demand context and tenant/institution context.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-041-BH-001 | The system shall admit governed termination only for an identifiable Purchase Requisition when withdrawal/cancellation is permitted for its current J03-governed position and applicable authority is established. | L3-J-041 §§7–9 |
| L4-J-041-BH-002 | The system shall preserve all prior requisition, validation, approval and release-related evidence before applying the termination consequence. | L3-J-041 §§9,12,15 |
| L4-J-041-BH-003 | J03 shall apply the conclusive withdrawal/cancellation consequence under its own requisition ownership and shall preserve the termination basis, rationale and authority. | L3-J-041 §§5,9,12 |
| L4-J-041-BH-004 | After governed termination, the system shall make the requisition ineligible for a new J03 release as active procurement demand and shall not publish `PurchaseRequisitionReleasedForProcurement` for a post-termination release. | L3-J-041 §§9,14,16 |
| L4-J-041-BH-005 | The system shall represent termination through the parent-defined business consequence without inventing a separate withdrawal/cancellation state name. | L3-J-041 §13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-041-BH-006 | The system shall not invent a canonical withdrawal/cancellation event because L2-J03 defines none for this fact. | L3-J-041 §14 |
| L4-J-041-BH-007 | J03 termination shall not silently cancel, reverse or rewrite purchase orders, Finance facts or other downstream domain-owned lifecycle outcomes; any such action remains with the canonical downstream owner. | L3-J-041 §§4,10,12 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-041-BH-008 | If termination authority or lifecycle eligibility is unresolved, the system shall not infer termination and shall retain the last valid requisition position until governance resolves the request. | L3-J-041 §11 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-041-BH-009 | The system shall preserve the full requisition history, termination request/decision, basis, authority, prior lifecycle evidence and final procurement-ineligibility consequence so the outcome is reconstructable. | L3-J-041 §15 |

## 8. Observable Completion Conditions
Complete only when the withdrawal/cancellation consequence is conclusively authorized under J03, the requisition is no longer eligible for a new release as active procurement demand, its prior history remains reconstructable, and no downstream-owned fact has been silently reversed.

## 9. Traceability & Provenance
**Parent:** L3-J-041 — Purchase Requisition Withdrawal or Cancellation to Governed Termination  
**Primary verification source:** L2-J03 — Purchase Requisition & Approval Workflows.

## 10. Open Issues / Deferred Design
No material upstream issue blocks derivation. No new cancellation/withdrawal state, event or actor is introduced. Downstream reversal behavior remains outside this specification unless defined by the relevant owning architecture. Technical realization and UI/UX remain deferred to L5/L6.
