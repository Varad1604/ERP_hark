# Compliance Finding to Verified CAPA Closure

## 1. Specification Metadata
**Specification ID:** L4-H-067  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accreditation & Compliance  
**Realizes Scenario:** L3-H-067  
**Primary Business Owner:** Accreditation & Compliance  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-067; L2-H09 — Accreditation & Compliance Workflows; `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md`; applicable Locked source-domain architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern an H09 compliance Finding/CAPA through planning, source-domain remediation evidence, effectiveness verification, closure and governed reopening where effectiveness does not hold, without taking ownership of source operational remediation.

## 3. Required Business Context
A governed observation/nonconformity supports a Finding; CAPA is required; remediation ownership/evidence obligations are assigned; H09 owns Finding/CAPA governance; the Accreditation and Compliance Governance Authority owns governed closure/reopening decisions where institutional authority is required.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-067-BH-001 | The system shall establish CAPA in `Opened` from a governed Finding and progress it through `Opened → Action Planned → In Progress` while the accountable source domain performs operational remediation. | L3-H-067 §§7–9,12–13 |
| L4-H-067-BH-002 | When authoritative remediation evidence is received, the system shall transition `In Progress → Verification Pending` without re-owning the source operational facts. | L3-H-067 §§5,9,12–13 |
| L4-H-067-BH-003 | When effectiveness verification succeeds, the system shall transition `Verification Pending → Verified` and publish `CAPAVerified`. | L3-H-067 §§9,12–14 |
| L4-H-067-BH-004 | After the applicable authoritative closure decision, the system shall transition `Verified → Closed` and publish `CAPAClosed`. | L3-H-067 §§6,9,12–15 |
| L4-H-067-BH-005 | If authoritative post-closure evidence shows that the corrective outcome did not hold or the finding remains unresolved, a governed reopening decision may transition `Closed → Reopened` and shall publish `CAPAReopened`. | L3-H-067 §§6,10,12–15 |
| L4-H-067-BH-006 | A reopened CAPA shall progress `Reopened → In Progress` and repeat the remediation, verification and closure path; prior closure evidence shall remain immutable. | L3-H-067 §§10–13,15–16 |
| L4-H-067-BH-007 | CAPA shall not reach `Closed` before successful effectiveness verification. | L3-H-067 §§11–12,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-067-BH-008 | H09 shall publish `CAPAVerified`, `CAPAClosed`, and `CAPAReopened` for the corresponding H09-owned CAPA outcomes; source-domain remediation evidence shall return through the governed CAPA hand-off without transferring operational ownership. | L3-H-067 §§5,14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-067-BH-009 | Missing or failed effectiveness verification shall prevent `Closed` and keep the CAPA in its applicable governed pre-closure/rework position. | L3-H-067 §§11–12 |
| L4-H-067-BH-010 | Reopening shall preserve the original closure evidence and record the new reason/authority; no prior history shall be erased. | L3-H-067 §§10–11,15 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-067-BH-011 | The system shall preserve Finding/root cause, action plan, remediation owner, source execution evidence, verification basis, original closure, reopening reason/authority, later remediation and final closure history. | L3-H-067 §15 |

## 8. Observable Completion Conditions
The behavior is complete when CAPA effectiveness is `Verified` and the CAPA reaches final `Closed`; if reopening was required, the reopened cycle must complete through renewed remediation and verification without erasing prior closure history. Source operational ownership remains unchanged.

## 9. Traceability & Provenance
**Parent:** L3-H-067 — Compliance Finding to Verified CAPA Closure  
**Primary verification source:** L2-H09 — Accreditation & Compliance Workflows  
**Reopening verification:** `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md`

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Task/workflow/UI implementation remains deferred.
