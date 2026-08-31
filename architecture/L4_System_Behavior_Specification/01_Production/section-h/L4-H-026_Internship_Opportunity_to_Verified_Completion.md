# Internship Opportunity to Verified Completion

## 1. Specification Metadata
**Specification ID:** L4-H-026  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Placement  
**Realizes Scenario:** L3-H-026  
**Primary Business Owner:** Placement  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-026; L2-H04 — Placement Workflows; `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern an approved internship opportunity assigned to an eligible Student through participation, operational completion, evaluation and verified completion, with controlled pre-verification interruption/reopening.

## 3. Required Business Context
Approved Internship opportunity; eligible Student; one Employer and one Student assignment; recruitment/offer context; completion evidence; Placement Officer authority.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-026-BH-001 | Placement shall admit the Internship only for an approved opportunity and eligible Student and shall preserve one Employer and one Student assignment. | L3-H-026 §§7–9,12 |
| L4-H-026-BH-002 | The successful Internship lifecycle shall follow `Published → Candidate Registered → Offered → Accepted → Started → Completed → Evaluated → Verified`; operational `Completed` shall not be treated as final verified completion. | L3-H-026 §§9,12–13 |
| L4-H-026-BH-003 | `Withdrawn` or `Cancelled` may occur before `Verified` only where policy permits; reopening shall be permitted only before `Verified` and shall preserve prior interruption history. | L3-H-026 §§10–13 |
| L4-H-026-BH-004 | A `Verified` Internship shall remain immutable historical evidence and shall not be locally reopened. | L3-H-026 §§10–13,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-026-BH-005 | Placement shall publish `InternshipStarted`, `InternshipCompleted`, `InternshipEvaluated`, `InternshipCompletionVerified`, `InternshipWithdrawn`, `InternshipCancelled` and `InternshipReopened` only for their corresponding governed facts. | L3-H-026 §14 |

## 6. Failure / Rejection / Continuation
Incomplete or invalid evidence prevents verification; withdrawal/cancellation/reopening shall preserve prior participation history and cannot reverse `Verified` completion.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-026-BH-006 | The system shall preserve Internship, Employer, Student, participation, completion evidence, evaluation, verification, exception/reopening decisions and optional conversion history. | L3-H-026 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-026-BH-007 | Successful completion shall require operational completion evidence, governed evaluation and final `Verified`; alternative governed pre-verification outcomes may end in `Withdrawn` or `Cancelled`. | L3-H-026 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-026 — Internship Opportunity to Verified Completion  
**Primary verification source:** Locked H04 lifecycle-completion amendment.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation; stale H04 official-examination ownership wording does not alter Section I ownership.