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
**Depends On:** L3-H-037; L2-H05 — Research Workflows; `L2-H05_Consultancy_Cancellation_Governance_Amendment_2026-08-27.md`; Finance reconciliation confirmation  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern an approved Research Consultancy through delivery acceptance and authoritative Finance reconciliation to Research closure, or through a governed pre-delivery-acceptance cancellation.

## 3. Required Business Context
Authorized client/agreement; approved Consultancy; valid team references; Research Director authority; deliverables; authoritative Finance reconciliation confirmation where required.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-037-BH-001 | Successful Consultancy progression shall be `Approved → Active → Delivery Submitted → Delivery Accepted → Financial Reconciliation Pending → Closed`. | L3-H-037 §§9,12–13 |
| L4-H-037-BH-002 | Research shall not self-certify Finance reconciliation; `Financial Reconciliation Pending → Closed` shall require the authoritative Finance confirmation/reference where reconciliation is required. | L3-H-037 §§9,11–12,16 |
| L4-H-037-BH-003 | Before `Delivery Accepted`, governed cancellation may follow `Approved / Active / Delivery Submitted → Cancellation Requested → Cancelled`; cancellation after `Delivery Accepted` shall not be permitted. | L3-H-037 §§7,9–13 |
| L4-H-037-BH-004 | Research shall store only Finance-confirmed reference/status and shall not create or alter Finance accounting, receivable, payment, settlement or reconciliation facts. | L3-H-037 §§5,12 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-037-BH-005 | Research shall publish `ResearchConsultancyDeliverySubmitted`, `ResearchConsultancyDeliveryAccepted`, `ResearchConsultancyFinancialReconciliationRequested`, `ResearchConsultancyClosed`, `ResearchConsultancyCancellationRequested` and `ResearchConsultancyCancelled`; Finance retains its own semantic publisher role. | L3-H-037 §14 |

## 6. Failure / Rejection / Continuation
Invalid cancellation authority or missing Finance confirmation leaves the Consultancy in its last valid state with history preserved.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-037-BH-006 | The system shall preserve approval, execution, delivery submission/acceptance, cancellation decision where applicable, Finance confirmation reference, closure decision and immutable history. | L3-H-037 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-037-BH-007 | Completion shall require either accepted delivery plus required authoritative Finance reconciliation and Research `Closed`, or governed pre-acceptance `Cancelled`, with Finance ownership unchanged. | L3-H-037 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-037 — Consultancy Request to Delivered and Financially Reconciled Outcome  
**Primary verification source:** Locked H05 governance and consultancy-cancellation amendments.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation.