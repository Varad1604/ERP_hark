# Periodic Compliance Review to Governed Compliance Status

## 1. Specification Metadata
**Specification ID:** L4-H-070  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accreditation & Compliance  
**Realizes Scenario:** L3-H-070  
**Primary Business Owner:** Accreditation & Compliance  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-070; L2-H09 — Accreditation & Compliance Workflows; `L2-H09_Framework_and_Compliance_Review_Normalization_Amendment_2026-09-01.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern a periodic Compliance Review from scheduled evaluation through evidence review, action assignment and closure while recording only source-supported compliance-status outcomes.

## 3. Required Business Context
Applicable Compliance Requirements, authoritative evidence, review scope/schedule and H09 governance context are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-070-BH-001 | The system shall progress the Compliance Review only through `Scheduled → In Progress → Reviewed → Action Assigned → Closed`. | L3-H-070 §§9,13; L2-H09 normalization amendment §2 |
| L4-H-070-BH-002 | `Compliance Review Completed` shall be recorded when evidence evaluation reaches the `Reviewed` milestone and shall not be treated as final closure. | L2-H09 normalization amendment §2 |
| L4-H-070-BH-003 | Required governed actions shall be recorded before final closure where applicable; `Compliance Review Assigned` records action assignment and `Compliance Review Closed` records final closure. | L2-H09 normalization amendment §2 |
| L4-H-070-BH-004 | The system shall record `Compliant` only as a governed compliance-status outcome for an applicable Active Compliance Requirement when updated evidence and review confirmation satisfy BR-H09-035. | L3-H-070 §12; L2-H09 normalization amendment §3 |
| L4-H-070-BH-005 | `Compliant` shall not replace the canonical Compliance Requirement lifecycle, and unsupported or incomplete evidence shall prevent an unsupported compliant status. | L2-H09 normalization amendment §3 |

## 5. Cross-Domain & Event Behavior
Source-domain evidence remains owned by its canonical owner. H09 shall publish `Compliance Status Updated` from the Compliance Requirement aggregate when the governed assessment changes, while Compliance Review events remain H09 review facts.

## 6. Failure / Rejection / Continuation
Missing or conflicting evidence shall prevent unsupported status advancement or closure and keep the review in its appropriate governed position until action/evidence conditions are satisfied.

## 7. Audit & Evidence Behavior
The system shall preserve review scope/schedule, evidence provenance, requirement evaluation, deviations/actions, status decision and review milestone/closure history.

## 8. Observable Completion Conditions
Completion requires the review to reach `Closed` after evidence evaluation and required action assignment, with any recorded compliance-status outcome supported by authoritative evidence and BR-H09-035.

## 9. Traceability & Provenance
**Parent:** L3-H-070 — Periodic Compliance Review to Governed Compliance Status  
**Primary verification sources:** L2-H09 — Accreditation & Compliance Workflows; L2-H09 Framework and Compliance Review Normalization Amendment — 2026-09-01.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Technical workflow realization and UI remain deferred.
