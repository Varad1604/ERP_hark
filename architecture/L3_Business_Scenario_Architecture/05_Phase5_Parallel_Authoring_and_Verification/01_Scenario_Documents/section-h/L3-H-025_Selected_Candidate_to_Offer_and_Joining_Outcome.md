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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; applicable locked Student Lifecycle and People & Workforce architecture  
**Referenced By:** Placement offer management, joining verification, placement completion, candidate eligibility reopening, placement reporting, and audit activities

# 2. Scenario Purpose

Govern a selected candidate from confirmed Placement selection through offer issuance and the permitted offer/joining outcomes while preserving Placement-owned offer history, Student ownership, employer-originated input boundaries, and verified joining evidence.

# 3. Business Outcome

The offer is issued and accepted, declined, withdrawn, expired, or completed through verified joining

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a placement selection is confirmed for a candidate who has completed the required recruitment process.

## 4.2 End Boundary

The scenario ends when the PlacementOffer reaches a governed accepted, declined, withdrawn, expired, or joining-completed outcome and the applicable evidence/history is preserved. The `Expired` branch is not fully specified by locked H04 and therefore prevents authoritative completion of all frozen outcome branches.

## 4.3 In Scope

- Finalizing the selected candidate under Placement governance.
- Generating, validating, and publishing the PlacementOffer.
- Recording Student acceptance or rejection.
- Processing an employer-originated withdrawal before joining confirmation.
- Reopening candidate eligibility where H04 policy permits after withdrawal.
- Recording Student and employer joining confirmation and institutional verification.
- Publishing `PlacementCompleted` after verified joining.
- Preserving offer and joining history.

## 4.4 Out of Scope

- Candidate application-to-selection processing, governed by L3-H-024.
- Employer verification or drive creation/closure.
- Defining a missing offer-expiry transition, event, or authority locally.
- Enterprise notification delivery mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Retains Student identity/lifecycle information referenced by Placement. |
| People & Workforce / Section E | Retains Placement Officer/Coordinator workforce information used for Placement responsibilities. |
| Placement / Section H | Owns PlacementOffer, JoiningConfirmation, candidate offer status, withdrawal/reopening consequences, and placement-completion evidence. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student | Accepts or rejects a published PlacementOffer and confirms joining where applicable. | Decides the candidate response supported by H04. |
| Placement Officer | Reviews employer selection/withdrawal input, governs offer publication, and verifies joining evidence. | Performs H04-assigned Placement decisions/actions. |

Employer-originated selection, withdrawal, and joining confirmation are recorded by authorized placement staff from external employer submissions; no employer system actor is assumed.

# 7. Preconditions and Entry Criteria

- The candidate is in the Placement-owned `Selected` position.
- All mandatory recruitment stages were successfully completed.
- Hiring capacity remains available.
- Offer details and validity period are available for publication.
- Student identity remains authoritative in Section C.
- Joining verification, if reached, follows an accepted offer.

# 8. Trigger

A placement selection is confirmed

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Finalize the selected candidate and generate/validate the PlacementOffer. | Placement Officer / Placement | `PlacementOfferGenerated` records the governed offer creation. |
| 2 | Publish the offer with its validity period and update the candidate's Placement-owned offer status. | Placement | `PlacementOfferPublished` makes the governed offer available to the candidate. |
| 3 | If the Student accepts, validate applicable placement policy and record `PlacementOfferAccepted`. | Student / Placement | The offer reaches `Accepted`. |
| 4 | If the Student declines, record `PlacementOfferRejected`. | Student / Placement | The offer reaches `Declined`. |
| 5 | If the employer withdraws before joining confirmation, validate the external withdrawal request and record `PlacementOfferWithdrawn`; reopen candidate eligibility where H04 permits. | Placement Officer / Placement | Withdrawal remains auditable and candidate eligibility may reopen. |
| 6 | If an accepted offer proceeds to joining, record Student joining, employer confirmation, Placement verification, and `PlacementCompleted`. | Student / Placement Officer / Placement | Verified joining completes the placement lifecycle. |
| 7 | Preserve immutable offer decisions, withdrawal history, joining evidence, and completion history. | Placement | The supported branches remain fully traceable. |

The frozen `Expired` branch cannot be completed authoritatively because H04 names the state but defines no transition, event, or authority for entering it.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Student declines | Published offer is valid but not accepted. | Record `PlacementOfferRejected`. | Offer reaches `Declined`. | Placement |
| Employer withdraws | Offer exists and joining is not confirmed. | Validate withdrawal, publish `PlacementOfferWithdrawn`, and reopen candidate eligibility where policy permits. | Offer reaches `Withdrawn`; history is preserved. | Placement |
| Joining verification succeeds | Accepted offer has Student and employer confirmation with valid evidence. | Verify joining and complete Placement. | `PlacementCompleted` records the terminal placement-completion outcome. | Placement |
| Offer validity lapses | Offer is no longer valid. | H04 states that expired offers cannot be accepted, but no authoritative transition/event is defined. | **Blocked branch — no local expiry semantics invented.** | Placement |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Offer details are incomplete or invalid | Offer cannot be safely published. | Correct Placement-owned offer details before publication. | Placement |
| Withdrawal is requested after joining confirmation | H04 withdrawal precondition is not satisfied. | Do not apply the withdrawal path; preserve the completed joining position. | Placement |
| Joining evidence or employer confirmation is missing | Placement completion cannot be verified. | Do not publish completion until H04 joining requirements are satisfied. | Placement |
| Offer reaches its validity end | Frozen outcome requires expiry but H04 does not define the governing transition/event. | Record as an unresolved upstream lifecycle gap; do not invent expiry mechanics. | Placement |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only selected candidates may receive offers. | Placement | Offer generation begins only after selection. | L2-H04 §17.6 / §19.12 |
| Offer validity period is mandatory and expired offers cannot be accepted. | Placement | Offer validity constrains candidate acceptance. | L2-H04 §§17.6, 19.12–19.13 |
| Accepted offers cannot return to pending status and become immutable. | Placement | Acceptance history is preserved. | L2-H04 §§17.6, 19.13 |
| Withdrawn offers remain permanently auditable and cannot generate joining confirmations. | Placement | Withdrawal is a governed terminal branch before joining. | L2-H04 §§17.6, 17.8, 19.14 |
| Joining confirmation requires an accepted offer and employer confirmation is mandatory. | Placement | Placement completion requires verified joining evidence. | L2-H04 §§17.8, 19.15 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| PlacementOffer | `Draft` / `Generated` / `Published` | Offer is generated, published, then accepted, declined, withdrawn, or intended to expire. | `Accepted`, `Declined`, `Withdrawn`, `Expired` (undefined transition), or `Joining Confirmed` | Placement | L2-H04 §17.6 |
| JoiningConfirmation | `Pending` | Student and employer confirmations are recorded and institutionally verified. | `Completed` | Placement | L2-H04 §17.8 |
| CandidateApplication | `Selected` / offer-related position | Candidate offer status reflects supported offer outcome. | Supported H04 offer/joining position | Placement | L2-H04 §17.4 |

The `Expired` state appears in the PlacementOffer lifecycle but is absent from its transition table and the event catalogue.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Supported exact Placement events include `CandidateSelected`, `PlacementOfferGenerated`, `PlacementOfferPublished`, `CandidateOfferStatusUpdated`, `PlacementOfferAccepted`, `PlacementOfferRejected`, `PlacementPolicyEvaluated`, `OfferWithdrawalRequested`, `PlacementOfferWithdrawn`, `CandidateEligibilityReopened`, `StudentJoiningRecorded`, `EmployerJoiningConfirmed`, `JoiningVerified`, and `PlacementCompleted`.

No canonical `PlacementOfferExpired` event is defined by H04.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Employer selection/withdrawal/joining input remains external and is recorded by authorized placement staff under H04. Student identity remains owned by Section C. No new employer-authentication or expiry contract is invented.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Student decides offer acceptance/rejection. Placement Officer performs supported offer/withdrawal/joining governance. H04 does not define the authority or decision mechanism that places an offer into `Expired`.

## 15.2 Separation of Duties, Overrides and Exceptions

Offer withdrawal requires Placement review/approval under H04 and cannot occur after joining confirmation. No additional expiry override is defined.

## 15.3 Business Audit and Evidence Obligations

Evidence must preserve candidate/offer identity, offer details and validity period, publication, Student decision, withdrawal request/reason/approval, candidate eligibility reopening where applicable, Student joining, employer confirmation, verification evidence, completion, and immutable history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

Supported branches complete when the offer is accepted, declined, withdrawn, or completed through verified joining with all required evidence preserved. The full frozen completion set cannot be established because the `Expired` branch lacks an authoritative H04 transition/event/authority.

## 16.2 Postconditions

- Supported offer and joining history remains immutable/auditable.
- Placement remains owner of PlacementOffer and JoiningConfirmation.
- Student source ownership remains unchanged.
- Expiry remains unresolved rather than locally invented.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H04's PlacementOffer lifecycle includes `Expired`, and its rules state expired offers cannot be accepted, but the transition table has no transition into `Expired`; the event catalogue has no expiry event; and no expiry authority is defined. | The frozen final outcome explicitly requires an `expired` branch. L3 cannot complete that branch or assign an event publisher without inventing lifecycle semantics. | Define the canonical PlacementOffer expiry transition, trigger/authority, and event/publisher upstream. |

**Architecture assessment:** `BLOCKED`

The accepted, declined, withdrawn, and verified-joining branches are source-grounded, but the frozen `Expired` outcome is materially undefined. Upstream lifecycle resolution is required.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
