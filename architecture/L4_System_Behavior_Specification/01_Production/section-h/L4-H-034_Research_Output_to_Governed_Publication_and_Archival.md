# Research Output to Governed Publication and Archival

## 1. Specification Metadata
**Specification ID:** L4-H-034  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Research  
**Realizes Scenario:** L3-H-034  
**Primary Business Owner:** Research  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-034; L2-H05 — Research Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern institutional Research Publication metadata from submission/review through publication, successor correction and archival without acquiring source document or E03 workforce ownership.

## 3. Required Business Context
Verified Research output; valid author/project references; required metadata; duplicate/DOI/source-verification evidence; H05 review authority.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-034-BH-001 | Research Publication shall follow `Draft → Submitted → Under Review → Approved → Published → Archived`, with `Under Review → Revision Required → Submitted` and `Under Review → Rejected` branches. | L3-H-034 §§9–13 |
| L4-H-034-BH-002 | Missing metadata, failed review or duplicate/source conflict shall prevent approval/publication; prior review history shall remain immutable. | L3-H-034 §§10–12 |
| L4-H-034-BH-003 | A material correction after `Published` shall create a successor Publication version; the predecessor shall remain immutable and may become `Superseded` for current-use purposes. | L3-H-034 §§9–13 |
| L4-H-034-BH-004 | Archival shall preserve historical availability/evidence and shall not transfer source document or workforce ownership into Research. | L3-H-034 §§5,9,12,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-034-BH-005 | Research shall publish `ResearchPublicationRevisionRequired`, `ResearchPublicationRejected`, `ResearchPublicationPublished`, `ResearchPublicationSuperseded` and `ResearchPublicationArchived` for the corresponding governed facts. | L3-H-034 §14 |

## 6. Failure / Rejection / Continuation
Review failure or incomplete/duplicate metadata leaves the Publication in its last valid state; correction never overwrites a Published predecessor.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-034-BH-006 | The system shall preserve submission/review, revision/rejection/approval decisions, publication, successor lineage, supersession and archival history. | L3-H-034 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-034-BH-007 | Completion shall require the Publication to reach its governed final/current outcome with all review/publication/correction/archival evidence preserved and external source ownership unchanged. | L3-H-034 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-034 — Research Output to Governed Publication and Archival  
**Primary verification source:** Locked H05 governance/lifecycle amendment.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation.