# Grant Opportunity to Award and Funding Closure

## 1. Specification Metadata
**Specification ID:** L4-H-032  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Research  
**Realizes Scenario:** L3-H-032  
**Primary Business Owner:** Research  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-032; L2-H05 — Research Workflows; applicable Locked Finance/source-domain architecture; H05 governance amendments  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern a Research grant opportunity through award and H05 funding closure while preserving Finance ownership of monetary truth.

## 3. Required Business Context
Use the parent-defined grant opportunity/application/award context, funding source facts, H05 lifecycle, Finance confirmations where required, authority, events and evidence.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-032-BH-001 | The system shall admit grant progression only when the parent-defined opportunity/application/award prerequisites are satisfied. | L3-H-032 §§7–8 |
| L4-H-032-BH-002 | The system shall enforce only parent-defined grant/funding governance constraints and shall not create Finance-owned settlement facts. | L3-H-032 §12 |
| L4-H-032-BH-003 | The system shall apply only the exact H05 grant/funding lifecycle effects defined by the parent. | L3-H-032 §13 |
| L4-H-032-BH-004 | The system shall expose funding closure only when L3-H-032 completion conditions and required authoritative confirmations are satisfied. | L3-H-032 §§3,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-032-BH-005 | The system shall consume monetary facts only from Finance and shall keep H05 limited to Research-owned grant/funding context. | L3-H-032 §§5,14 |
| L4-H-032-BH-006 | Any parent-required grant/funding event or hand-off shall retain its exact identity and publisher/owner. | L3-H-032 §14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-032-BH-007 | Missing award/Finance confirmation or other parent-defined invalid condition shall prevent unsupported closure and preserve the last valid H05 position. | L3-H-032 §§10–11 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-032-BH-008 | The system shall preserve opportunity/application/award references, Finance-confirmed facts where required, authority and final funding outcome evidence required by the parent. | L3-H-032 §15 |

## 8. Observable Completion Conditions
Complete only when L3-H-032 §16 is objectively satisfied; Finance ownership remains unchanged.

## 9. Traceability & Provenance
**Parent:** L3-H-032 — Grant Opportunity to Award and Funding Closure  
**Primary verification source:** L2-H05 — Research Workflows.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. L5/L6 design remains deferred.
