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
**Depends On:** L3-H-032; L2-H05 — Research Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`; Finance reconciliation confirmation where required  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern the Research Grant from award through active execution, amendment and closure while Finance remains the sole owner of accounting, payment and reconciliation truth.

## 3. Required Business Context
Eligible proposal/project and PI; funding scheme/award evidence; Research obligations; sponsor evidence; authoritative Finance confirmation where closure/amendment requires it.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-032-BH-001 | Research shall establish a Grant only from valid award/governance context and move it `Awarded → Active` when Research-side activation conditions are satisfied. | L3-H-032 §§7–9,12–13 |
| L4-H-032-BH-002 | An amendment shall follow `Active → Amendment Pending → Active` only after applicable Research approval and any required authoritative Finance confirmation. | L3-H-032 §§9–13 |
| L4-H-032-BH-003 | Closure shall follow `Active → Closure Pending → Closed` only after Research obligations and sponsor evidence are complete and authoritative Finance reconciliation confirmation exists where required. | L3-H-032 §§9–13,16 |
| L4-H-032-BH-004 | Research shall store only Finance-confirmed reference/status needed by Grant governance and shall not create accounting, budget, payment, posting or reconciliation truth. | L3-H-032 §§5,12 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-032-BH-005 | Research shall publish `ResearchGrantAmendmentRequested`, `ResearchGrantAmendmentApproved`, `ResearchGrantClosureInitiated` and `ResearchGrantClosed`; Finance retains semantic ownership of its own financial events. | L3-H-032 §14 |

## 6. Failure / Rejection / Continuation
Invalid award/utilization or missing required Research/Finance evidence shall prevent progression; closure remains pending until authoritative conditions are satisfied.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-032-BH-006 | The system shall preserve application/award, amendment request/decision, Research obligations, Finance confirmation reference, closure decision and immutable Grant history. | L3-H-032 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-032-BH-007 | Completion shall require Grant `Closed`, completed Research obligations, applicable sponsor evidence and consumed authoritative Finance reconciliation confirmation where required. | L3-H-032 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-032 — Grant Opportunity to Award and Funding Closure  
**Primary verification source:** Locked H05 governance/lifecycle amendment.

## 10. Open Issues / Deferred Design
Historical application-event vocabulary differences shall not create a second semantic publisher.