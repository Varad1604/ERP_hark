# Approved Participation to Governed External Learning Registration

# 1. Scenario Metadata

**Scenario ID:** L3-D-037  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0758 / CAN-L3-D-038 (final L3-D-037)

---

# 2. Scenario Purpose

Govern registration of an institution-approved learner with an approved external learning provider while preserving D06 institutional ownership and avoiding duplicate institutional registration outcomes.

# 3. Business Outcome

The External Learning Registration is accepted with a provider-side enrollment reference linked to the D06 institutional enrollment, or remains in a governed Rejected/Failed/Recovery-required business position.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A D06 Flexible Learning Enrollment is Approved for an external learning opportunity and the provider is approved for institutional use.

## 4.2 End Boundary

Provider-side registration acknowledgement establishes the governed D06 registration outcome and `LearnerRegisteredWithProvider` is published where accepted.

## 4.3 In Scope

- Establish learner/enrollment/provider business context.
- Request provider-side registration through the agreed business hand-off.
- Record provider acknowledgement and provider enrollment reference.
- Govern rejection/failure/recovery without duplicate institutional registration outcome.
- Preserve D06 ownership of institutional enrollment/registration facts.

## 4.4 Out of Scope

- Authentication, credentials, API payloads, transport, retries or integration-service design.
- Provider ownership of institutional enrollment.
- Credit recognition/transcript posting.

## 4.5 Related Scenarios — Optional

L3-D-034 supplies Approved enrollment; later D06 scenarios govern progress/completion.

# 5. Ownership and Participating Domains

D06 owns institutional External Learning Registration, provider reference and synchronization history. External Learning Provider owns provider-side registration acknowledgement only.

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| External Learning Provider | Accepts or rejects provider-side learner registration and supplies its provider enrollment reference/status facts. | External-provider authority over provider-side registration only. |
| Academic Affairs, where institutional exception decision is required | Governs any business exception under D06 policy. | Policy-bound institutional authority. |

No `Integration Service`, `Synchronization Engine` or `Integration Administrator` is treated as an L3 business actor.

# 7. Preconditions and Entry Criteria

D06 enrollment is Approved; provider is approved/Active; learner/provider correlation context is available.

# 8. Trigger

Approved external-learning participation requires provider-side registration.

# 9. Main Business Flow

1. D06 validates Approved enrollment, provider status and learner/provider business context.
2. D06 issues the governed registration hand-off to the provider.
3. External Learning Provider accepts or rejects the provider-side registration and returns acknowledgement/reference.
4. D06 validates the acknowledgement as business evidence.
5. For acceptance, D06 records the provider reference and publishes `LearnerRegisteredWithProvider`.
6. For a governed failure/rejection, D06 records the business position and follows the applicable recovery/exception path without creating duplicate institutional registration outcome.
7. Subsequent provider enrollment status facts may be reconciled/synchronized under D06 ownership using the canonical D06 events.

# 10. Alternate Business Flows

No separate alternate path reaches the Accepted registration outcome. Provider rejection/failure is a blocking/terminal business condition governed under Section 11.

# 11. Failure, Interruption and Business Recovery

Invalid provider/enrollment context blocks the hand-off until corrected. Provider rejection/failure retains the D06 governed unsuccessful/recovery-required position. Interrupted acknowledgement leaves completion uncertain; D06 preserves the last authoritative business position and reconciles provider evidence before deciding whether registration has occurred.

# 12. Business Rules and Decision Constraints

D06 remains canonical institutional owner; provider owns only provider-side acknowledgement; repeated same business fact must not create duplicate institutional registration outcome; technical idempotency/retry/authentication is deferred.

# 13. State and Lifecycle Impact

External Learning Registration: Requested → Submitted/handed off → Accepted → Active; governed alternate Rejected / Recovery Required / Failed as defined by L2-D06 Aggregate 4; W33–W34.

# 14. Business Events and Cross-Domain Collaboration

D06 remains publisher of `LearnerRegisteredWithProvider`, `ProviderRegistrationFailed`, `EnrollmentStatusSynchronized`, `SynchronizationRecovered`, and `SynchronizationFailureResolved` under L2-D06 W33–W39/Event Catalogue.

Provider acknowledgement is a business hand-off; no ownership transfer occurs.

# 15. Business Authorization, Approval and Audit Obligations

Provider decides provider-side acceptance/rejection; D06 records the institutional outcome; any institutional exception requires D06-policy business authority. Audit retains Approved enrollment, provider identity/status, hand-off, acknowledgement/reference, failure/recovery evidence and final institutional registration position.

# 16. Completion Criteria and Postconditions

Accepted provider registration is linked to the D06 enrollment and remains traceable; unsuccessful/recovery-required positions are explicit; no provider ownership of institutional enrollment is created.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. APIs, authentication, credentials, transport, retry policy, synchronization implementation and technical idempotency are deferred to L4/L5.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
