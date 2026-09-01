# Aid Funding and Utilization to Reconciled Programme Outcome

## 1. Specification Metadata
**Specification ID:** L4-F-066  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Scholarship & Financial Aid  
**Realizes Scenario:** L3-F-066  
**Primary Business Owner:** Scholarship & Financial Aid  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-F-066; L2-F11 — Scholarship & Financial Aid; L2-F00 — Finance Domain Model & Foundation  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
Reconcile authoritative financial-aid funding and utilization facts into a traceable programme-level outcome without modifying historical facts or absorbing Budget, Treasury or General Ledger ownership.

## 3. Required Business Context
- Valid Funding Source and funding history.
- Recorded Benefit Utilization against active assistance.
- Identifiable programme context.
- Applicable funding and utilization constraints.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-066-BH-001 | The system shall establish the applicable Funding Source history for the programme reconciliation context. | L3-F-066 §§3,7,9 |
| L4-F-066-BH-002 | The system shall establish recorded Benefit Utilization relevant to the programme. | L3-F-066 §§3,7,9 |
| L4-F-066-BH-003 | The system shall validate that utilization references active assistance and funding assignments satisfy applicable invariants. | L3-F-066 §§9,12 |
| L4-F-066-BH-004 | The system shall determine the programme-level reconciliation outcome from authoritative funding and utilization facts. | L3-F-066 §§3,9 |
| L4-F-066-BH-005 | The system shall represent material variance or correction as a new business fact without overwriting historical funding or utilization. | L3-F-066 §§10–12 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-066-BH-006 | On applicable completed funding or utilization transitions, the system shall publish `FundingSourceAssigned`, `FundingSourceChanged`, `BenefitUtilizationRecorded`, or `BenefitUtilizationUpdated` as defined by the authoritative lifecycle. | L3-F-066 §14; L2-F11 |
| L4-F-066-BH-007 | The system shall make authoritative funding and utilization facts available to reporting and audit consumers without transferring ownership. | L3-F-066 §§3,14 |

## 6. Failure / Rejection / Continuation
- Invalid Funding Source shall prevent a reconciled funding outcome.
- Utilization not referencing active assistance shall be rejected.
- Incomplete historical evidence shall prevent claiming a complete reconciliation.
- Variance correction shall create a new business fact rather than overwrite history.
- Accounting consequences remain under General Ledger ownership.

## 7. Audit & Evidence Behavior
Preserve programme context, Funding Source history, Benefit Allocation, utilization history, reconciliation result, variance/correction facts and relevant published business facts.

## 8. Observable Completion Conditions
- Funding and utilization inputs are established.
- Applicable invariants are satisfied.
- Programme-level reconciliation outcome is determined.
- Any variance/correction is represented as a new fact.
- Historical funding and utilization remain reconstructable.
- Reporting and audit consumers can consume the authoritative outcome.

## 9. Traceability & Provenance
**Parent:** L3-F-066 — Aid Funding and Utilization to Reconciled Programme Outcome  
**Primary verification sources:** L2-F11 — Scholarship & Financial Aid; L2-F00 — Finance Domain Model & Foundation

## 10. Open Issues / Deferred Design
L2-F11 does not define a dedicated `Programme Reconciled` state or `ProgrammeReconciled` event. Reconciliation is therefore treated as a business outcome derived from canonical funding and utilization facts, without inventing a new lifecycle state or event.
