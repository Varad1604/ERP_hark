# Selected Candidate to Offer and Joining Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-025  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-C02 — Student Lifecycle & Activation Workflows; L2-E01 — Workforce & Engagement Workflows; `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md`  
**Referenced By:** Placement completion and reporting; L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern a Placement-selected candidate through offer publication, candidate response, employer withdrawal where permitted, expiry, and verified joining while preserving immutable offer/joining history.

# 3. Business Outcome

The offer is issued and accepted, declined, withdrawn, expired, or completed through verified joining

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a placement selection is confirmed.

## 4.2 End Boundary

The scenario ends when the PlacementOffer reaches an authoritative response outcome—Accepted, Rejected, Withdrawn or Expired—or an accepted offer proceeds through the separate JoiningConfirmation lifecycle to verified Placement completion.

## 4.3 In Scope

Offer generation/publication; Student acceptance/rejection; governed expiry; employer-originated withdrawal recorded by Placement staff; candidate eligibility reopening where policy permits; joining confirmation and verification.

## 4.4 Out of Scope

Application-to-selection (H024), notification delivery, and Student/workforce lifecycle changes.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / C | Owns Student identity/lifecycle facts. |
| People & Workforce / E | Owns workforce context for Placement staff. |
| Placement / H | Owns PlacementOffer, JoiningConfirmation, response outcomes, withdrawal/expiry consequences and `PlacementCompleted`. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Student | Accepts or rejects a published offer and confirms joining where applicable. |
| Placement Officer | Primary business authority for PlacementOffer and JoiningConfirmation decisions and records employer-originated withdrawal facts. |

Employer-originated withdrawal and joining confirmations remain external inputs recorded through the governed H04 process.

# 7. Preconditions and Entry Criteria

- Candidate is `Selected`.
- Required recruitment is complete.
- Hiring capacity remains available where applicable.
- Offer details and validity period are defined.
- Joining may proceed only from an Accepted offer.

# 8. Trigger

A placement selection is confirmed

# 9. Main Business Flow

1. Generate, validate and publish the PlacementOffer.
2. Record the Student response as Accepted or Rejected.
3. If the governed response period ends without valid acceptance, move the offer to `Expired` and publish `PlacementOfferExpired`.
4. If employer withdrawal occurs before joining confirmation, govern the withdrawal and reopen candidate eligibility where policy permits.
5. If an Accepted offer proceeds to joining, record Student and employer confirmation.
6. Verify joining evidence under the JoiningConfirmation lifecycle.
7. Publish `PlacementCompleted` only when the applicable joining evidence has been verified and the joining outcome is complete.
8. Preserve immutable offer, response, expiry/withdrawal and joining history.

# 10. Alternate Business Flows

- Student rejection: offer reaches the authoritative rejected/declined outcome.
- Employer withdrawal before joining: offer reaches `Withdrawn`; candidate eligibility may reopen where policy permits.
- No valid response within the governed period: offer reaches `Expired` and is no longer actionable.
- Accepted offer without verified joining: Placement is not yet complete.

# 11. Failure, Interruption and Business Recovery

Invalid offer details prevent publication. Expired offers cannot be newly accepted. Withdrawal after joining confirmation is not treated as a valid offer-withdrawal path. Missing joining evidence prevents `PlacementCompleted`.

# 12. Business Rules and Decision Constraints

- Only selected candidates may receive offers.
- Offer validity period is mandatory; expired offers are no longer actionable.
- Published offers may reach Accepted, Rejected, Withdrawn or Expired.
- An Accepted offer does not itself prove joining.
- Joining remains governed by the separate JoiningConfirmation aggregate.
- Placement completion requires verified joining evidence.

# 13. State and Lifecycle Impact

PlacementOffer supports governed response outcomes:

- Accepted
- Rejected
- Withdrawn
- Expired

JoiningConfirmation remains a separate lifecycle and governs the evidence required for verified joining. Expiry preserves the offer record and does not mean institutional rejection of the candidate.

# 14. Business Events and Cross-Domain Collaboration

Canonical H04 events include the existing offer/withdrawal/joining facts and the locked `PlacementOfferExpired` event. H04 remains the semantic publisher of PlacementOffer and JoiningConfirmation facts. C/E source facts remain externally owned.

# 15. Business Authorization, Approval and Audit Obligations

Student owns the acceptance/rejection response. Placement Officer is the primary business authority for PlacementOffer and JoiningConfirmation decisions. Evidence preserves offer terms/validity, publication, Student response, expiry/withdrawal reason, reopened eligibility where applicable, joining confirmations, verification and completion.

# 16. Completion Criteria and Postconditions

The scenario completes when the offer reaches one authoritative response outcome or, for an Accepted offer, when verified joining reaches the governed Placement-completion outcome. All offer and joining history remains immutable and C/E ownership is unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing offer-expiry transition/event semantics were resolved by locked `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md`, which defines `Expired` and `PlacementOfferExpired`; the scenario passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
