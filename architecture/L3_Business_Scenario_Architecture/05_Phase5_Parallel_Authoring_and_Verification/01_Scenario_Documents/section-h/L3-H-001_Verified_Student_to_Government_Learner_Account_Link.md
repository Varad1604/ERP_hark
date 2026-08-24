# Verified Student to Government Learner Account Link

# 1. Scenario Metadata

**Scenario ID:** L3-H-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** A, B, C, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** L2-H01 — APAAR & DigiLocker Workflows; L2-A02 — Person & Identity Workflows; L2-B01 — Institution Workflows; L2-C02 — Student Lifecycle & Activation Workflows; applicable Section A authorization and audit architecture

# 2. Scenario Purpose

Establish or reject a governed association between a verified canonical Student and an APAAR or DigiLocker learner account while preserving Student, Institution and identity ownership outside H01 and preserving the H01 consent and government-link history.

# 3. Business Outcome

A governed learner-account link is established or rejected with the external reference, consent basis, and audit history preserved

# 4. Scenario Boundary and Scope

The scenario starts when the frozen trigger occurs and ends when H01 records the successful government learner-account link or a controlled rejected/failed outcome. In scope are authoritative Student, Institution and identity references; applicable consent; H01 connector eligibility; the government-link outcome; and H01 link events. Credential publication and any change to Student, Institution or Person/Identity lifecycles are out of scope.

# 5. Ownership and Participating Domains

- **A — Platform Foundation:** owns Person/Identity references, authorization capability and audit evidence capability used by this scenario.
- **B — Institution & Structure:** owns the Institution context referenced by H01.
- **C — Student Lifecycle:** owns the Student Aggregate and Student lifecycle.
- **H — APAAR & DigiLocker:** owns the government-integration link outcome, applicable H01 Consent, Connector and H01 link events.

Participation does not transfer Student, Institution or Person/Identity ownership to H01.

# 6. Business Actors, Roles and Responsibilities

- **Student:** may initiate APAAR or DigiLocker linking and may perform the source-defined H01 consent actions.
- H01 also permits institution-initiated linking, but the locked H01 architecture does not identify the canonical human role for that branch. No role is invented locally.

# 7. Preconditions and Entry Criteria

- The canonical Student exists and is associated with valid institutional context.
- Required Person/Identity verification is already established by its canonical owner.
- The applicable government Connector is in a source-defined active/usable state.
- Applicable authorization and policy conditions are satisfied.
- Consent is valid where governing policy requires it.

# 8. Trigger

A verified student or authorized institutional process requests an APAAR or DigiLocker link

# 9. Main Business Flow

1. H01 accepts the governed linking request and identifies the target government platform.
2. H01 resolves the canonical Student, Institution and identity references without modifying them.
3. H01 verifies the source-owned identity/student eligibility and applicable authorization conditions.
4. H01 validates the applicable Consent basis where policy requires consent.
5. H01 conducts the governed government-linking interaction through an eligible H01 Connector.
6. H01 records the government response and external reference where supplied.
7. H01 records the successful or rejected/failed link outcome and publishes the target-specific H01 event.
8. Required audit evidence remains available without changing source-domain ownership.

# 10. Alternate Business Flows

- **APAAR success:** H01 records the link and publishes `APAARLinked`.
- **DigiLocker success:** H01 records the link and publishes `DigiLockerLinked`.
- **Government rejection/failure:** H01 records the controlled outcome and publishes `APAARLinkFailed` or `DigiLockerLinkFailed` as applicable.
- **Consent unavailable where required:** no governed link is established.

# 11. Failure, Interruption and Business Recovery

If the Student/identity reference is not authoritative, the link does not proceed. If the Connector is not eligible, the H01 link remains unestablished until a source-defined connector lifecycle outcome permits use. A government rejection is recorded as the controlled link outcome; no source Student or identity record is changed to force success.

# 12. Business Rules and Decision Constraints

- H01 owns integration, not Student, Institution, academic or Person/Identity business data.
- Student and identity verification must come from their canonical owners.
- Where policy requires consent, H01 must validate Consent before the governed sharing/linking outcome.
- Only source-defined H01 Connector states may permit government use.
- H01 link events retain H01 as canonical publisher.

# 13. State and Lifecycle Impact

- **Student / C:** referenced only; lifecycle unchanged.
- **Institution / B:** referenced only; lifecycle unchanged.
- **Person/Identity / A:** referenced only; lifecycle unchanged.
- **H01 linking outcome:** H01 records the platform-specific successful or failed/rejected link fact. Locked H01 does not define a separate learner-link aggregate/state machine, so none is introduced.
- **Consent / H01:** existing `Pending`, `Granted`, `Withdrawn` or `Expired` state is applied according to locked H01 rules.

# 14. Business Events and Cross-Domain Collaboration

H01 defines `APAARLinkRequested`, `APAARLinked`, `APAARLinkFailed`, `DigiLockerLinkRequested`, `DigiLockerLinked`, and `DigiLockerLinkFailed`. No generic learner-account-link event is invented. B, C and A supply authoritative references; they do not become publishers of H01 link events.

# 15. Business Authorization, Approval and Audit Obligations

Student initiation is source-supported. The human role for institution-initiated linking is not defined upstream. No additional approval or separation-of-duty rule is invented. Audit evidence must preserve the authoritative references, consent basis where applicable, government reference/acknowledgement where available, initiating context and final H01 link outcome.

# 16. Completion Criteria and Postconditions

Completion requires authoritative source references, applicable policy/consent validation, an H01 government-link decision, recorded external reference where available, the exact applicable H01 outcome event, and preserved audit history. After completion, H01 owns only the government-integration link outcome; A, B and C retain their canonical ownership.

# 17. Open Issues, Assumptions and Deferred Questions

**Open issues:**
- Locked H01 has no canonical Actors section and does not identify the institutional human role for institution-initiated linking.
- H01 dependency labels contain stale section-letter descriptions that conflict with the higher-authority current dependency map.
- H01 models linking within its Publication integration model but defines no distinct learner-link aggregate/state machine.

**Architecture assessment:** `PASS WITH ISSUES`

These gaps do not prevent the frozen link-or-rejection outcome from being expressed with source-defined H01 events and preserved canonical ownership.

**Assumptions:** None.

**Deferred questions:** Upstream architecture may define the institutional linking actor and a dedicated link lifecycle if later levels require them.
