# Compliance Finding to Verified CAPA Closure

## 1. Specification Metadata
**Specification ID:** L4-H-067  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accreditation & Compliance  
**Realizes Scenario:** L3-H-067  
**Primary Business Owner:** Accreditation & Compliance  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-067; L2-H09 — Accreditation & Compliance Workflows; applicable Locked source-domain architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern a compliance finding through the verified corrective/preventive-action closure defined by L3-H-067 without re-owning source operational facts.

## 3. Required Business Context
Use the parent-defined finding, corrective/preventive action plan, responsible authority, verification evidence, lifecycle, events/contracts and audit obligations.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-067-BH-001 | The system shall admit CAPA progression only when the parent-defined finding and action-plan prerequisites are valid. | L3-H-067 §§7–8 |
| L4-H-067-BH-002 | The system shall require source-supported completion evidence and the parent-defined verification decision before closure. | L3-H-067 §§9,12,15 |
| L4-H-067-BH-003 | The system shall apply only the CAPA lifecycle effects defined by the parent. | L3-H-067 §13 |
| L4-H-067-BH-004 | The system shall expose CAPA as closed only when L3-H-067 completion conditions are satisfied. | L3-H-067 §§3,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-067-BH-005 | The system shall preserve canonical ownership of operational source facts/evidence used to verify the CAPA. | L3-H-067 §§5,14 |
| L4-H-067-BH-006 | Any parent-required finding/CAPA event or hand-off shall retain its exact identity and publisher/owner. | L3-H-067 §14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-067-BH-007 | Missing or failed verification shall prevent closure and preserve the parent-defined open/unresolved position. | L3-H-067 §§10–11 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-067-BH-008 | The system shall preserve finding, action plan, responsible authority, completion/verification evidence and final closure outcome required by the parent. | L3-H-067 §15 |

## 8. Observable Completion Conditions
Complete only when L3-H-067 §16 is objectively satisfied.

## 9. Traceability & Provenance
**Parent:** L3-H-067 — Compliance Finding to Verified CAPA Closure  
**Primary verification source:** L2-H09 — Accreditation & Compliance Workflows.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Task/workflow/UI implementation remains deferred.
