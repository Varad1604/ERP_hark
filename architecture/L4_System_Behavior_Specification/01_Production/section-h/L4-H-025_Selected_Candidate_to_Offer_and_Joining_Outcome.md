# Selected Candidate to Offer and Joining Outcome

## 1. Specification Metadata
**Specification ID:** L4-H-025  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Placement  
**Realizes Scenario:** L3-H-025  
**Primary Business Owner:** Placement  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-025; L2-H04 — Placement Workflows; applicable Locked Student/source-domain architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern a selected Placement candidate through the offer and joining outcome approved by L3-H-025 without changing Student lifecycle ownership.

## 3. Required Business Context
Use the parent-defined selected-candidate reference, offer/joining context, decisions, Placement lifecycle, events/contracts and evidence.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-025-BH-001 | The system shall admit offer/joining progression only for a parent-valid selected candidate and required governing context. | L3-H-025 §§7–8 |
| L4-H-025-BH-002 | The system shall enforce only parent-defined offer and joining constraints. | L3-H-025 §12 |
| L4-H-025-BH-003 | The system shall apply only the exact Placement offer/joining lifecycle effects defined by the parent. | L3-H-025 §13 |
| L4-H-025-BH-004 | The system shall expose the final offer/joining outcome only under L3-H-025 completion conditions. | L3-H-025 §§3,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-025-BH-005 | The system shall preserve canonical Student/source facts outside Placement while recording only H04-owned outcome facts. | L3-H-025 §§5,14 |
| L4-H-025-BH-006 | Any parent-required offer/joining event or hand-off shall retain its exact canonical identity and publisher/owner. | L3-H-025 §14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-025-BH-007 | Declined, withdrawn, invalid or incomplete offer/joining conditions shall follow only the parent-supported outcome and shall not fabricate joining completion. | L3-H-025 §§10–11 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-025-BH-008 | The system shall preserve selected-candidate, offer decision, joining confirmation and final outcome evidence required by the parent. | L3-H-025 §15 |

## 8. Observable Completion Conditions
Complete only when L3-H-025 §16 is objectively satisfied.

## 9. Traceability & Provenance
**Parent:** L3-H-025 — Selected Candidate to Offer and Joining Outcome  
**Primary verification source:** L2-H04 — Placement Workflows.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. L5/L6 design remains deferred.
