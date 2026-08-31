# Employer Registration to Governed Placement Participation

## 1. Specification Metadata
**Specification ID:** L4-H-022  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Placement  
**Realizes Scenario:** L3-H-022  
**Primary Business Owner:** Placement  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-022; L2-H04 — Placement Workflows; `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern the institutional Placement Employer relationship from registration through verification and active participation to rejection, suspension/reactivation or retirement without modelling the external company's corporate lifecycle.

## 3. Required Business Context
Employer participation intent; mandatory organization/contact facts; uniqueness check; Placement Officer authority; verification evidence.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-022-BH-001 | On an employer participation request recorded by authorized Placement staff, the system shall validate mandatory information and uniqueness before creating the Placement Employer in `Registered`. | L3-H-022 §§7–9 |
| L4-H-022-BH-002 | Employer participation shall follow `Registered → Verification Pending → Verified → Active → Retired`, with `Verification Pending → Rejected`, `Active → Suspended → Active`, and `Suspended → Retired` branches as applicable. | L3-H-022 §§9–13 |
| L4-H-022-BH-003 | Placement Officer shall be the primary business authority for verification, activation, suspension, reactivation and retirement decisions; technical administration shall not replace that authority. | L3-H-022 §§6,12,15 |
| L4-H-022-BH-004 | Incomplete verification shall prevent `Active`; `Retired` shall remain terminal for that participation record and preserve historical recruitment evidence. | L3-H-022 §§11–13,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-022-BH-005 | Placement shall publish the applicable H04 Employer events, including `EmployerActivated`, `EmployerSuspended`, `EmployerReactivated` and `EmployerRetired`, for their corresponding lifecycle facts. | L3-H-022 §14 |

## 6. Failure / Rejection / Continuation
Rejected or suspended participation remains traceable. Future participation after `Retired` requires a new governed relationship where policy permits.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-022-BH-006 | The system shall preserve registration, verification, activation/suspension/reactivation/retirement decisions, actor, rationale and immutable historical status. | L3-H-022 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-022-BH-007 | Completion shall require the Employer to reach the applicable governed participation outcome with decision/history preserved; only `Active` enables current governed Placement participation. | L3-H-022 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-022 — Employer Registration to Governed Placement Participation  
**Primary verification source:** Locked H04 lifecycle-completion amendment.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Employer technical accounts/authentication remain outside this L4 boundary.