# Research Collaboration Request to Active or Closed Agreement

## 1. Specification Metadata
**Specification ID:** L4-H-036  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Research  
**Realizes Scenario:** L3-H-036  
**Primary Business Owner:** Research  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-036; L2-H05 — Research Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern the institutional Research Collaboration relationship from request/review through approval/activation to rejection, amendment, termination or closure while external agreement, workforce and Finance facts remain externally owned.

## 3. Required Business Context
Partner/agreement context; due-diligence evidence; participant references; institutional approval requirements; Research Director authority.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-036-BH-001 | Collaboration shall follow `Requested → Under Review → Approved → Active → Closed`, with `Under Review → Rejected`, `Active → Amendment Pending → Active`, and `Active / Amendment Pending → Terminated` branches. | L3-H-036 §§9–13 |
| L4-H-036-BH-002 | Unverified partner/agreement context shall prevent approval; Research Director shall own the applicable strategic approval/rejection/amendment/termination/closure decision. | L3-H-036 §§6–7,11–12,15 |
| L4-H-036-BH-003 | Rejected, Closed and Terminated Collaboration records shall remain historical/immutable; a later relationship requires a new governed Collaboration rather than rewriting prior history. | L3-H-036 §§11–13 |
| L4-H-036-BH-004 | Research shall govern only the Research Collaboration relationship and shall not re-own external legal agreement, E03 workforce or Finance monetary facts. | L3-H-036 §§5,12 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-036-BH-005 | Research shall publish `ResearchCollaborationRejected`, `ResearchCollaborationActivated`, `ResearchCollaborationAmendmentApproved`, `ResearchCollaborationTerminated` and `ResearchCollaborationClosed` for their corresponding facts. | L3-H-036 §14 |

## 6. Failure / Rejection / Continuation
Invalid partner/agreement or approval evidence prevents progression; no rejected/terminated/closed record is silently reopened.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-036-BH-006 | The system shall preserve partner/agreement references, review, decision authority, amendment history, termination/closure rationale and complete lifecycle history. | L3-H-036 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-036-BH-007 | Completion shall require the Collaboration to reach `Rejected`, `Active`, `Terminated` or `Closed` under the governed branch with all evidence preserved and external ownership unchanged. | L3-H-036 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-036 — Research Collaboration Request to Active or Closed Agreement  
**Primary verification source:** Locked H05 governance/lifecycle amendment.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation.