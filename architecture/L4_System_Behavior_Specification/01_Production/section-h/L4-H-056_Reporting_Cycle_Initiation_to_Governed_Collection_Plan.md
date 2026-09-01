# Reporting Cycle Initiation to Governed Collection Plan

## 1. Specification Metadata
**Specification ID:** L4-H-056  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** NEP, NAAC & AISHE Reporting  
**Realizes Scenario:** L3-H-056  
**Primary Business Owner:** NEP, NAAC & AISHE Reporting  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-056; L2-H08 — NEP, NAAC & AISHE Reporting Workflows; `L2-H08_Regulatory_Reporting_Authority_and_Lifecycle_Amendment_2026-08-26.md`; applicable Locked source-domain architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall establish and govern the H08 Reporting Collection Plan for a reporting cycle through approval and activation, while preserving every evidence-owning domain's authority over the operational facts requested by the plan.

## 3. Required Business Context
An applicable governed Reporting Framework Definition and Reporting Cycle exist; required collection scope, responsibilities, deadlines, evidence obligations and source contracts can be identified; the Regulatory Reporting Governance Authority owns Collection Plan approval.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-056-BH-001 | The system shall create a Reporting Collection Plan in `Draft` for the governed Reporting Cycle only when the applicable framework, reporting period and source-obligation context are valid. | L3-H-056 §§4,7–9 |
| L4-H-056-BH-002 | The system shall define collection scope, source-domain responsibilities, deadlines, evidence obligations and reporting-contract references using authoritative source ownership without transferring that ownership to H08. | L3-H-056 §§5,9,12 |
| L4-H-056-BH-003 | When the Draft plan is ready for governance review, the system shall transition it `Draft → Under Review`. | L3-H-056 §§9,12–13 |
| L4-H-056-BH-004 | When the Regulatory Reporting Governance Authority approves the plan, the system shall transition it `Under Review → Approved` and publish `ReportingCollectionPlanApproved`. | L3-H-056 §§6,9,12–15 |
| L4-H-056-BH-005 | When an Approved plan is activated, the system shall transition it `Approved → Active`, publish `ReportingCollectionPlanActivated`, and make that Active plan the governed collection context for the associated Reporting Cycle. | L3-H-056 §§3,9,12–14,16 |
| L4-H-056-BH-006 | When review requires correction, the system shall transition `Under Review → Revision Required → Draft` and publish `ReportingCollectionPlanRevisionRequired`. | L3-H-056 §§10,12–14 |
| L4-H-056-BH-007 | A governed temporary halt may transition `Active → Suspended → Active`; the system shall preserve the plan and collection history across suspension/restoration without inventing an additional event not defined by the parent. | L3-H-056 §§10,12–14 |
| L4-H-056-BH-008 | A governed cancellation shall transition `Active` or `Suspended` to `Cancelled` and publish `ReportingCollectionPlanCancelled`; completed collection obligations shall transition `Active → Completed` and publish `ReportingCollectionPlanCompleted`. | L3-H-056 §§10,12–14 |
| L4-H-056-BH-009 | The system shall not create a Reporting Cycle lifecycle state named `Active`; the frozen active-cycle business meaning is satisfied by an `Active` Reporting Collection Plan associated with the governed cycle. | L3-H-056 §§3,12–13,16–17 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-056-BH-010 | H08 shall publish `ReportingCollectionPlanApproved`, `ReportingCollectionPlanRevisionRequired`, `ReportingCollectionPlanActivated`, `ReportingCollectionPlanCompleted`, and `ReportingCollectionPlanCancelled`; source-domain facts/events retain their canonical owners and publishers. | L3-H-056 §14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-056-BH-011 | Incomplete scope, unresolved source responsibility, missing reporting-contract governance or other mandatory plan deficiency shall prevent transition to `Active`. | L3-H-056 §11 |
| L4-H-056-BH-012 | Missing source facts shall be corrected by their canonical owners; H08 shall not fabricate or modify operational facts to complete the Collection Plan. | L3-H-056 §§5,11–12 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-056-BH-013 | The system shall preserve framework/cycle references, collection scope, responsibilities, deadlines, source contracts, review history, approval authority/decision, activation and subsequent Collection Plan lifecycle evidence. | L3-H-056 §15 |

## 8. Observable Completion Conditions
The behavior is complete when an authoritative Reporting Collection Plan has reached `Approved` and `Active`, governs the Reporting Cycle's source-collection obligations, and preserves all source-domain ownership. Later `Completed`, `Cancelled`, or suspended/restored outcomes remain governed by the same H08 Collection Plan lifecycle.

## 9. Traceability & Provenance
**Parent:** L3-H-056 — Reporting Cycle Initiation to Governed Collection Plan  
**Primary verification source:** L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Lifecycle/authority verification:** `L2-H08_Regulatory_Reporting_Authority_and_Lifecycle_Amendment_2026-08-26.md`

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. The `Active` business-readiness description applies to the Reporting Collection Plan, not to a newly invented Reporting Cycle state. Implementation/UI design remains deferred.
