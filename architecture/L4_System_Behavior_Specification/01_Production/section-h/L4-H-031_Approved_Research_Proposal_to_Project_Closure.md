# Approved Research Proposal to Project Closure

## 1. Specification Metadata
**Specification ID:** L4-H-031  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Research  
**Realizes Scenario:** L3-H-031  
**Primary Business Owner:** Research  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-031; L2-H05 — Research Workflows; E03 workforce and Finance-owned evidence where applicable  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall turn an `Approved` Research Proposal into a governed Research Project, execute it through completion and close it only after required deliverables and institutional closure conditions are satisfied.

## 3. Required Business Context
Approved Proposal; valid PI/team references; Ethics approval where mandatory; funding references where applicable; milestones/progress/deliverables; closure evidence.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-031-BH-001 | Research shall create a Project only from an `Approved` Proposal, validate PI/team and required Ethics/funding references, and prevent duplicate creation except where upstream configuration explicitly permits multiple implementation projects. | L3-H-031 §§7–9,12 |
| L4-H-031-BH-002 | The Project shall follow `None → Initiated → Active`, permit `Active ↔ Suspended`, and progress `Active → Completed → Closed` only through the locked H05 conditions. | L3-H-031 §§9–13 |
| L4-H-031-BH-003 | Mandatory deliverables shall be satisfied before `Completed`; failed closure validation shall leave the Project `Completed` until the outstanding governed obligations are satisfied. | L3-H-031 §§10–13 |
| L4-H-031-BH-004 | Research shall consume authoritative Finance/compliance evidence by reference where closure policy requires it and shall not create Finance-owned monetary or reconciliation facts. | L3-H-031 §§5,9,12 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-031-BH-005 | Research shall consume `ResearchProposalApproved` and publish `ResearchProjectCreated`, `ResearchTeamAssigned`, `ProjectMilestoneUpdated`, `ProjectProgressReported`, `ResearchProjectStatusChanged`, `ResearchProjectCompleted` and `ResearchProjectClosed` as the corresponding facts occur. | L3-H-031 §14 |

## 6. Failure / Rejection / Continuation
Invalid participant status, missing mandatory Ethics/funding references or incomplete deliverables shall prevent the affected transition; prior Project evidence remains preserved.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-031-BH-006 | The system shall preserve originating Proposal, team assignments, milestone/progress history, state changes, completion summary, deliverable evidence, closure checks, closure authority and Project events. | L3-H-031 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-031-BH-007 | Completion shall require origin from an Approved Proposal, satisfaction of mandatory deliverables and closure validation, Project `Closed`, and preserved evidence with E03/Finance ownership unchanged. | L3-H-031 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-031 — Approved Research Proposal to Project Closure  
**Primary verification source:** L2-H05 — Research Workflows.

## 10. Open Issues / Deferred Design
The parent notes no distinct Deliverable Verification aggregate/event/role and no universal Finance reconciliation event/contract for project closure. L4 uses only the defined Project closure outcome and does not invent those facts.