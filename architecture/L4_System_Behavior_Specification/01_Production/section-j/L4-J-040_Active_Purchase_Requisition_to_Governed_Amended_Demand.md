# Active Purchase Requisition to Governed Amended Demand

## 1. Specification Metadata
**Specification ID:** L4-J-040  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Purchase Requisition & Approval  
**Realizes Scenario:** L3-J-040  
**Primary Business Owner:** Purchase Requisition & Approval  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-J-040; L2-J03 — Purchase Requisition & Approval Workflows; applicable Locked L1/L2 architecture; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern an authorized amendment to active Purchase Requisition demand while preserving J03 ownership, source-demand provenance, prior requisition history and any validation or approval obligations that become applicable because of the amendment.

## 3. Required Business Context
Required context includes the active Purchase Requisition and its current governed position, authoritative requesting-domain amendment request and rationale, changed purpose/item/quantity context, amendment authority, existing requisition history and applicable policy/validation/approval conditions.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-040-BH-001 | The system shall accept an amendment only when the targeted Purchase Requisition is identifiable, active under J03 governance and the requesting change and authority are valid for its current governed position. | L3-J-040 §§7–9 |
| L4-J-040-BH-002 | Before making the amended demand authoritative, the system shall preserve the prior requisition content and its provenance so the pre-amendment position remains reconstructable. | L3-J-040 §§9,12,15 |
| L4-J-040-BH-003 | J03 shall apply the authorized changed purpose, item and quantity content to the Purchase Requisition without claiming ownership of the requesting domain's source-demand truth. | L3-J-040 §§5,9 |
| L4-J-040-BH-004 | The system shall reapply applicable policy, authorization and required validation appropriate to the amended demand before permitting progression beyond the amendment consequence. | L3-J-040 §§9,12 |
| L4-J-040-BH-005 | Where the amendment requires renewed review or approval, the system shall require the governed Requisition Reviewer / Approver decision and preserve segregation-of-duties constraints. | L3-J-040 §§6,9,12 |
| L4-J-040-BH-006 | The amended requisition shall progress only to source-supported J03 lifecycle stages after the required governance completes; the system shall not create a separate amendment state. | L3-J-040 §§9,13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-040-BH-007 | The system shall not invent or publish an amendment-specific canonical event. Existing J03 submitted, approved or released events shall occur only when their source-defined business facts actually arise after the amendment. | L3-J-040 §14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-040-BH-008 | If amendment authority, amendment context or lifecycle eligibility is insufficient, the system shall not mutate the authoritative requisition; the last valid requisition content shall remain authoritative and the applicable parent-supported withdrawal/cancellation or new-demand path may be used instead. | L3-J-040 §§10–11 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-040-BH-009 | The system shall preserve prior content, amendment request and rationale, requesting/decision authority, amended content, provenance, and any renewed validation or approval evidence. | L3-J-040 §15 |

## 8. Observable Completion Conditions
Complete only when the authoritative Purchase Requisition reflects the governed amended demand, prior content remains reconstructable, required renewed governance is evidenced, and no downstream release occurs unless source-defined release conditions are subsequently satisfied.

## 9. Traceability & Provenance
**Parent:** L3-J-040 — Active Purchase Requisition to Governed Amended Demand  
**Primary verification source:** L2-J03 — Purchase Requisition & Approval Workflows.

## 10. Open Issues / Deferred Design
No material upstream issue blocks derivation. L2-J03 defines neither a discrete amendment state nor a canonical amendment event, so none is introduced here. Technical versioning, persistence, workflow implementation, interfaces and UI/UX remain deferred to L5/L6.
