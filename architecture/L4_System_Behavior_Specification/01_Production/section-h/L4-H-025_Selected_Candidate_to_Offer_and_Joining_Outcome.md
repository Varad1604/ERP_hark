# Selected Candidate to Offer and Joining Outcome

## 1. Specification Metadata
**Specification ID:** L4-H-025  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Placement  
**Realizes Scenario:** L3-H-025  
**Primary Business Owner:** Placement  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-025; L2-H04 — Placement Workflows; `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern a selected candidate through PlacementOffer response and, where accepted, verified JoiningConfirmation before declaring Placement completion.

## 3. Required Business Context
Candidate `Selected`; completed recruitment; hiring capacity where applicable; offer terms/validity; Student response; employer/joining evidence.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-025-BH-001 | Only a `Selected` candidate with complete required recruitment may receive a PlacementOffer; offer details and validity shall validate before publication. | L3-H-025 §§7–9,12 |
| L4-H-025-BH-002 | A published PlacementOffer shall reach one governed response outcome: `Accepted`, `Rejected`, `Withdrawn` or `Expired`; an expired offer shall no longer be actionable. | L3-H-025 §§9–13 |
| L4-H-025-BH-003 | `Accepted` shall not itself prove joining; JoiningConfirmation shall remain a separate lifecycle and shall require Student/employer confirmation and verified joining evidence. | L3-H-025 §§9,12–13 |
| L4-H-025-BH-004 | `PlacementCompleted` shall occur only after the applicable joining evidence is verified; employer withdrawal before joining may reopen candidate eligibility only where H04 policy permits. | L3-H-025 §§9–12,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-025-BH-005 | Placement shall publish its exact offer/withdrawal/joining events, including `PlacementOfferExpired` when validity lapses and `PlacementCompleted` only after verified joining. | L3-H-025 §14 |

## 6. Failure / Rejection / Continuation
Invalid offer details prevent publication; expired offers cannot later be newly accepted; missing joining evidence prevents Placement completion.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-025-BH-006 | The system shall preserve offer terms/validity, publication, Student response, expiry/withdrawal reason, reopened eligibility where permitted, joining confirmations, verification and completion evidence. | L3-H-025 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-025-BH-007 | Completion shall require one authoritative offer response outcome or, for `Accepted`, verified joining through the governed Placement-completion outcome, with immutable offer/joining history. | L3-H-025 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-025 — Selected Candidate to Offer and Joining Outcome  
**Primary verification source:** Locked H04 lifecycle-completion amendment.

## 10. Open Issues / Deferred Design
The prior offer-expiry gap is resolved upstream; no new authority or state is introduced.