# Selected Candidate to Offer and Joining Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-025  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-C02 — Student Lifecycle & Activation Workflows; L2-E01 — Workforce & Engagement Workflows  
**Referenced By:** Placement completion and reporting

# 2. Scenario Purpose

Govern a Placement-selected candidate through offer publication, candidate response, employer withdrawal where permitted, and verified joining while preserving immutable offer/joining history.

# 3. Business Outcome

The offer is issued and accepted, declined, withdrawn, expired, or completed through verified joining

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a placement selection is confirmed.

## 4.2 End Boundary

Accepted, declined, withdrawn, and verified-joining paths are defined by H04. H04 names `Expired` in the PlacementOffer lifecycle and rules but provides no transition into `Expired`, no expiry command/event, and no expiry authority. The full frozen outcome is unresolved.

## 4.3 In Scope

Offer generation/publication; Student acceptance/rejection; employer-originated withdrawal recorded by Placement staff; candidate eligibility reopening where policy permits; joining confirmation and verification.

## 4.4 Out of Scope

Application-to-selection (H024), invented offer-expiry mechanics, and notification delivery.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / C | Owns Student identity/lifecycle facts. |
| People & Workforce / E | Owns workforce context for Placement staff. |
| Placement / H | Owns PlacementOffer, JoiningConfirmation, offer status, withdrawal consequences and `PlacementCompleted`. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Student | Accepts/rejects a published offer and confirms joining where applicable. |
| Placement Officer | Governs offer publication, records/reviews employer withdrawal input, and verifies joining evidence. |

Employer-originated selection, withdrawal and joining confirmation are external submissions recorded by authorized Placement staff.

# 7. Preconditions and Entry Criteria

Candidate is `Selected`; required recruitment is complete; hiring capacity remains available; offer details/validity are defined; joining may proceed only from an accepted offer.

# 8. Trigger

A placement selection is confirmed

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Generate, validate and publish the PlacementOffer. | Placement Officer / H | Governed offer becomes available. |
| 2 | Record Student acceptance or rejection. | Student / H | Offer becomes `Accepted` or `Declined`. |
| 3 | If employer withdrawal occurs before joining confirmation, govern the withdrawal and reopen candidate eligibility where policy permits. | Placement Officer / H | Offer becomes `Withdrawn`; history remains auditable. |
| 4 | If an accepted offer proceeds to joining, record Student and employer confirmation, verify evidence, and complete Placement. | Student / Placement Officer / H | `PlacementCompleted` records verified joining completion. |
| 5 | Apply offer expiry when validity lapses. | H | **BLOCKED:** H04 defines no canonical transition/event/authority into `Expired`. |

# 10. Alternate Business Flows

Student decline, valid employer withdrawal, and verified joining follow explicit H04 paths. Offer expiry is a frozen outcome branch but remains undefined upstream.

# 11. Failure, Interruption and Business Recovery

Invalid offer details prevent publication. Withdrawal after joining confirmation is not permitted. Missing joining evidence prevents completion. Validity lapse cannot be converted into an invented expiry transition.

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Only selected candidates may receive offers. | H | Offer generation starts after selection. |
| Offer validity period is mandatory; expired offers cannot be accepted. | H | Validity constrains acceptance. |
| Accepted offers are immutable and cannot return to pending. | H | Decision history is preserved. |
| Withdrawn offers remain auditable and cannot generate joining confirmation. | H | Withdrawal is terminal for that offer. |
| Joining requires accepted offer and employer confirmation. | H | Placement completion requires verified evidence. |

# 13. State and Lifecycle Impact

PlacementOffer supports `Draft → Generated → Published → Accepted/Declined/Withdrawn` and accepted offer to joining confirmation. `Expired` appears in the diagram but has no transition table entry. JoiningConfirmation progresses `Pending → Student Confirmed → Employer Confirmed → Institution Verified → Completed`.

# 14. Business Events and Cross-Domain Collaboration

Supported exact events include `CandidateSelected`, `PlacementOfferGenerated`, `PlacementOfferPublished`, `CandidateOfferStatusUpdated`, `PlacementOfferAccepted`, `PlacementOfferRejected`, `OfferWithdrawalRequested`, `PlacementOfferWithdrawn`, `CandidateEligibilityReopened`, `StudentJoiningRecorded`, `EmployerJoiningConfirmed`, `JoiningVerified`, and `PlacementCompleted`. No `PlacementOfferExpired` event exists.

# 15. Business Authorization, Approval and Audit Obligations

Student decides acceptance/rejection. Placement Officer performs supported offer/withdrawal/joining governance. H04 defines no expiry authority. Evidence preserves offer details/validity, publication, Student decision, withdrawal reason/approval, eligibility reopening, joining confirmations, verification evidence, completion, and history.

# 16. Completion Criteria and Postconditions

Accepted, declined, withdrawn and verified-joining branches can complete. The full frozen scenario is blocked because the required `Expired` branch lacks lifecycle and event semantics. C/E ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `Expired` is named in the PlacementOffer lifecycle/rules, but H04 defines no transition, command, event, or authority that enters `Expired`. | Frozen outcome explicitly requires expiry. | Define the canonical expiry trigger/transition/event/publisher upstream. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
