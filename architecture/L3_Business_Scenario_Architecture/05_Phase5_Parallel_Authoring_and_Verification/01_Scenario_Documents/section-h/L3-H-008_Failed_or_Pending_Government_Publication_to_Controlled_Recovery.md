# Failed or Pending Government Publication to Controlled Recovery

# 1. Scenario Metadata

**Scenario ID:** L3-H-008  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** A, C, D, F, H, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H01 — APAAR & DigiLocker Workflows; applicable canonical credential-owning source-domain architecture; Section A Authorization and Audit architecture  
**Referenced By:** Government credential publication recovery, reconciliation, operational governance, and audit activities

# 2. Scenario Purpose

Govern recovery of an H01 government-publication transaction that has failed, stalled before completion, or must be cancelled, while preserving the original publication identity, source-domain credential ownership, external-reference evidence, and recovery lineage.

# 3. Business Outcome

The publication is safely retried, cancelled, or escalated without duplicate external records and with complete recovery lineage

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a publication fails, stalls, or must be cancelled before terminal completion.

## 4.2 End Boundary

The frozen end boundary requires H01 to reach a governed retry, cancellation, or escalation outcome without duplicate external records and with complete recovery lineage. Locked H01 contains contradictory lifecycle statements about whether `PublicationFailed` is terminal while also permitting retry after failure; therefore the failed-publication recovery end boundary cannot be fully resolved in this Draft.

## 4.3 In Scope

- Classifying the H01-owned publication condition for recovery eligibility.
- Preserving the same Publication identity and correlation lineage during governed recovery.
- Applying a source-defined controlled retry where H01 policy permits.
- Cancelling a publication that is still eligible for cancellation.
- Routing a non-recoverable condition to H01 manual recovery or escalation without inventing a new lifecycle state or event.
- Preserving government acknowledgement, failure, cancellation, and recovery evidence.

## 4.4 Out of Scope

- Changing, correcting, approving, or reissuing the source credential.
- Defining new retry limits, technical retry timing, infrastructure recovery, or transport behavior.
- Republishing a corrected credential, which is a separate frozen scenario.
- Changing source-domain ownership of Student, academic, Finance, or Examination facts.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains canonical Authorization and Audit ownership supporting governed recovery actions and immutable evidence. |
| Student Lifecycle / Section C | Retains Student identity and lifecycle facts referenced by the publication where applicable. |
| Academic Delivery / Section D | Retains source-owned academic-delivery facts where applicable. |
| Finance / Section F | Frozen participant; H01 does not define a mandatory Finance-owned recovery decision or hand-off for this generic publication recovery. |
| APAAR & DigiLocker / Section H | Owns Publication recovery governance, Publication identity/status, retry/cancellation events, connector interaction, and recovery history. |
| Examination / Section I | Retains official examination/credential facts where the recovered publication concerns an Examination-owned credential. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Institution Administrator | May cancel a pending publication before completion where H01 permits cancellation. | Source-defined H01 command authority for `CancelPublication`. |

Locked H01 does not define a canonical business actor for the manual-recovery/escalation branch. No actor is invented for that responsibility.

# 7. Preconditions and Entry Criteria

- An H01 Publication already exists and retains its canonical Publication identity.
- The source credential remains owned by its canonical business domain.
- The publication is failed, pending/stalled before completion, or otherwise cancellation-eligible under H01 governance.
- Existing publication status, external references, and recovery history are available for evaluation.
- Any recovery action remains subject to H01 publication, connector, consent, and authorization rules.

# 8. Trigger

A publication fails, stalls, or must be cancelled before terminal completion

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the existing H01 Publication and preserve its source credential reference, Publication identity, correlation lineage, and prior external evidence. | APAAR & DigiLocker | Recovery is attached to the existing governed publication context. |
| 2 | Classify the publication condition using the locked H01 recovery rules and determine whether a controlled retry, cancellation, or manual recovery path is permitted. | APAAR & DigiLocker | A source-defined recovery path is selected without changing source credential meaning. |
| 3 | If recovery is retry-eligible, preserve the original Publication identity and initiate the H01-controlled retry path. | APAAR & DigiLocker | Recovery proceeds without creating a duplicate publication identity. |
| 4 | If the publication remains cancellation-eligible, an authorized cancellation records the H01 cancellation outcome. | Institution Administrator / APAAR & DigiLocker | `PublicationCancelled` records the controlled cancellation fact. |
| 5 | If the condition is not recoverable through the defined retry path, route it to H01 manual recovery or escalation while preserving complete evidence. | APAAR & DigiLocker | The unresolved condition remains governed and traceable. |
| 6 | Preserve all prior and resulting publication, acknowledgement, failure, cancellation, and recovery evidence. | APAAR & DigiLocker / Platform Audit | Recovery lineage remains reconstructable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Retry-eligible failed publication | H01 classifies the failure as eligible for retry. | Apply the source-defined retry path using the existing Publication identity and correlation lineage. | H01 may publish `PublicationRetryScheduled` and `PublicationRetryStarted`, followed by the applicable publication outcome. | APAAR & DigiLocker |
| Cancellation before completion | Publication is not completed and cancellation is authorized. | Cancel the pending publication. | H01 publishes `PublicationCancelled`. | APAAR & DigiLocker |
| Non-recoverable condition | H01 does not permit controlled retry. | Preserve the failed publication and route for manual recovery/escalation. | Recovery remains governed but no new event or state is invented. | APAAR & DigiLocker |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Recovery would duplicate an already completed or in-flight publication | A duplicate external publication could result. | Do not create a second completed publication identity; retain the existing H01 publication and duplicate-prevention rules. | APAAR & DigiLocker |
| Consent or connector eligibility is no longer valid | Publication cannot safely continue. | Preserve the existing publication and route to the permitted H01 recovery/cancellation path. | APAAR & DigiLocker |
| Manual recovery is required | Automatic/source-defined recovery cannot complete the business outcome. | Preserve all evidence and route the condition to governed manual recovery; exact business actor is unresolved upstream. | APAAR & DigiLocker |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Retry execution shall never create duplicate government records. | APAAR & DigiLocker | Recovery must preserve Publication identity and duplicate prevention. | L2-H01 §16 |
| Every publication request has a unique Publication identity and retry reuses the original correlation lineage. | APAAR & DigiLocker | Recovery remains tied to the original publication context. | L2-H01 §40 |
| `PublicationRetryScheduled` follows `PublicationFailed`. | APAAR & DigiLocker | Failed-publication recovery uses the exact canonical H01 retry event sequence where applicable. | L2-H01 §39 |
| A pending publication may be cancelled before completion and publishes `PublicationCancelled`. | APAAR & DigiLocker | Cancellation is a governed terminal recovery outcome where permitted. | L2-H01 §§35, 37 |
| Source credential ownership and meaning remain outside H01. | Canonical source domain | Recovery cannot modify the credential itself. | L2-H01 §§2–3, 16 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| H01 Publication | `Failed`, or a pre-completion publication state eligible for cancellation/recovery | H01 applies the permitted retry, cancellation, or manual-recovery branch. | `Cancelled`, later `Published`/`Failed` after permitted retry, or unresolved manual recovery according to locked H01. | APAAR & DigiLocker | L2-H01 §§33, 35, 39, 60 |
| Source credential | Existing authoritative credential state | Referenced only. | Unchanged. | Applicable canonical source domain | L2-H01 ownership boundary |

The H01 lifecycle is internally ambiguous: §39 calls `PublicationFailed` terminal while the same section requires retry scheduling after `PublicationFailed`, and §60 depicts retry/manual recovery from `Failed`. L3 does not silently choose one interpretation.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Applicable exact H01 events are `PublicationFailed`, `PublicationRetryScheduled`, `PublicationRetryStarted`, `PublicationCancelled`, and—if recovery later completes the existing publication—`GovernmentAcknowledgementRecorded` followed by `PublicationCompleted`. H01 remains the publisher of these publication events.

No source-domain credential event is republished or re-owned by H01 in this recovery scenario.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H01 retains the existing Credential Reference and Publication identity while interacting with the government boundary under its locked Publication Contract. Any source-domain credential correction or approval remains outside this recovery scenario and under its canonical owner.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Cancel an eligible pending publication | Institution Administrator | H01 permits cancellation only before publication completion. | APAAR & DigiLocker |
| Apply source-defined retry eligibility | No business actor defined | H01 recovery policy determines whether retry is permitted. | APAAR & DigiLocker |
| Manual recovery / escalation | Not defined upstream | Locked H01 requires manual recovery for non-recoverable conditions but does not define the canonical business actor. | APAAR & DigiLocker |

## 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty or override rule is defined for this scenario. The missing manual-recovery authority is recorded as an architecture gap rather than filled locally.

## 15.3 Business Audit and Evidence Obligations

Recovery evidence must preserve Publication ID, source credential reference, prior and resulting publication status, external transaction/government references when available, failure evidence, cancellation or retry lineage, correlation lineage, and the actor for any authorized cancellation. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen scenario completes only when the failed/pending publication reaches a source-supported controlled retry, cancellation, or escalation outcome without duplicate external records and with complete recovery lineage. Because the failed-state lifecycle and manual-recovery authority are unresolved upstream, the full completion criteria cannot be established authoritatively for every branch.

## 16.2 Postconditions

- Source credential ownership and meaning remain unchanged.
- Existing H01 Publication identity and prior evidence remain traceable.
- No duplicate completed publication is intentionally introduced.
- Any completed cancellation or retry outcome uses exact H01 events.
- Unresolved manual recovery remains visible rather than being given invented authority or lifecycle semantics.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-H01 §39 states that `PublicationCompleted` and `PublicationFailed` are terminal events, but §39 also requires `PublicationRetryScheduled` to follow `PublicationFailed`, W07 explicitly retries failed publications, and §60 depicts retry/manual recovery from `Failed`. | The lifecycle meaning of `Failed` is material to the frozen trigger and retry outcome. L3 cannot determine whether `Failed` is terminal for an attempt, terminal for the Publication aggregate, or recoverable without an upstream clarification. | Clarify the canonical Publication lifecycle and the meaning of terminality for `PublicationFailed`. |
| H01 requires manual recovery for non-retryable conditions but its canonical Actors section is absent and no business actor is assigned to the manual-recovery/escalation decision. | The frozen escalation branch lacks authoritative business authority. | Define the canonical H01 manual-recovery/escalation business actor and decision authority. |
| L2-H01 §65 uses stale section-letter labels that conflict with the current `DEPENDENCY_MAP.md`. | Participating-domain labels could be misread if H01 §65 is followed literally. | Correct H01 dependency labels upstream; current locked L1/Dependency Map ownership controls. |
| The frozen participant list includes Finance, but H01 recovery does not define a mandatory Finance-owned recovery hand-off. | Finance participation cannot be expanded with invented responsibilities. | Clarify Finance participation upstream if a mandatory recovery responsibility is intended. |

**Architecture assessment:** `BLOCKED`

The retry/cancellation mechanisms are substantially defined, but the contradictory terminality of `PublicationFailed` and the missing manual-recovery authority materially affect the frozen recovery outcome and lifecycle. Upstream clarification is required before this scenario can become a source-grounded complete Draft.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
