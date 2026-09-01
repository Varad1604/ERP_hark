# Eligible Student Application to Selection Decision

## 1. Specification Metadata
**Specification ID:** L4-H-024  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Placement  
**Realizes Scenario:** L3-H-024  
**Primary Business Owner:** Placement  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-024; L2-H04 — Placement Workflows; applicable C/D/I source facts  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern one Student's application to one published Placement Position through eligibility, screening and mandatory recruitment to an authoritative Placement `Selected` or `Eliminated` outcome.

## 3. Required Business Context
Published Position; open registration; active Student; published eligibility rules; recruitment workflow; authoritative academic facts from their canonical owners.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-024-BH-001 | On a Student application to an active Placement opportunity, the system shall create one CandidateApplication for exactly one Student and one Position and validate mandatory eligibility using authoritative source facts. | L3-H-024 §§7–9,12 |
| L4-H-024-BH-002 | Ineligible applications shall not progress; every eligible candidate shall undergo H04 screening, and mandatory recruitment stages shall execute sequentially without being skipped. | L3-H-024 §§9–12 |
| L4-H-024-BH-003 | CandidateApplication shall progress from `Registered` through H04 eligibility/screening/shortlisting/recruitment positions to `Selected` or `Eliminated`; incomplete evaluation shall prevent advancement. | L3-H-024 §§9–13 |
| L4-H-024-BH-004 | An `Eliminated` candidate shall return only through authorized H04 reconsideration/reinstatement with prior history preserved. | L3-H-024 §§10–13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-024-BH-005 | Placement shall publish the applicable H04 events from `CandidateRegistrationInitiated`/`CandidateApplicationCreated` through screening, shortlist, recruitment and final `CandidateSelected` or `CandidateEliminated`; official examination-result ownership remains with Section I, not D. | L3-H-024 §14 |

## 6. Failure / Rejection / Continuation
Unavailable authoritative academic facts prevent guessed eligibility; conflicting selection input remains under governed Placement review without rewriting completed stages.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-024-BH-006 | The system shall preserve Student/Position references, source evidence used, eligibility/policy results, screening/ranking, shortlist approval/publication, recruitment evaluations, employer selection input and final candidate outcome. | L3-H-024 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-024-BH-007 | Completion shall require completed mandatory eligibility/recruitment activity, recorded `Selected` or `Eliminated`, auditable decision history and unchanged source-domain fact ownership. | L3-H-024 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-024 — Eligible Student Application to Selection Decision  
**Primary verification source:** L2-H04 — Placement Workflows; `DEPENDENCY_MAP.md` for official result ownership.

## 10. Open Issues / Deferred Design
The parent records stale H04 examination-result dependency wording and no canonical employer-feedback event; neither is repaired in L4.