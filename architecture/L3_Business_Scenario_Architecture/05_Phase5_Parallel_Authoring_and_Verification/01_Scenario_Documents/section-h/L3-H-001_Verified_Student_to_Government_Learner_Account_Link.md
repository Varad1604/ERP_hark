# Verified Student to Government Learner Account Link

# 1. Scenario Metadata

**Scenario ID:** L3-H-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** A, B, C, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.2  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture and Locked Amendment 2026-08-26; `DEPENDENCY_MAP.md`; L2-H01 — APAAR & DigiLocker Workflows; L2-H01 Locked Amendment 2026-08-26; ADR-H-GOVLINK-001; L2-A02 — Person & Identity; authoritative Institution & Structure and Student Lifecycle workflows; applicable Platform Foundation authorization/event/audit architecture  
**Referenced By:** Government learner-link and credential-publication scenarios

# 2. Scenario Purpose

Establish a governed association between a verified canonical Student and an APAAR or DigiLocker learner account while preserving Student, Institution and Person/Identity ownership outside H01 and preserving H01 consent and link history.

# 3. Business Outcome

A governed platform-specific learner-account link reaches `Linked`, with authoritative source references, consent basis where required, government acknowledgement/reference where available, the applicable canonical H01 link event and audit history preserved.

# 4. Scenario Boundary and Scope

**Start:** A Student or authorized Institution Administrator requests APAAR/DigiLocker linking.  
**End:** The platform-specific H01 link request reaches `Linked`.

In scope: authoritative Student/Institution/Person references, consent, H01 connector eligibility, request processing, successful government acknowledgement and canonical link events. Out of scope: credential publication, changes to source-domain lifecycles, technical APIs/queues/retries/persistence or creation of a separate learner-link aggregate.

# 5. Ownership and Participating Domains

- **A — Platform Foundation:** owns Person/Identity and governed authorization/event/audit capabilities used by the scenario.
- **B — Institution & Structure:** owns Institution and institutional context.
- **C — Student Lifecycle:** owns Student and learner lifecycle.
- **H — APAAR & DigiLocker:** owns the government-integration link outcome, applicable H01 Consent/Connector context and H01 link events.

# 6. Business Actors, Roles and Responsibilities

| Actor | Responsibility |
|---|---|
| Student | May initiate linking for their own learner context and performs H01 consent actions assigned to Student. |
| Institution Administrator | May initiate institution-authorized linking where governing policy permits it. |
| Integration Administrator | Governs connector configuration only; does not own learner-link decisions. |
| Platform Administrator | Governs connector operational availability only; does not own learner-link decisions. |

Institution initiation does not bypass consent where consent is required.

# 7. Preconditions and Entry Criteria

- Canonical Student, Institution and Person/Identity references exist and are valid.
- Required identity verification is established by its canonical owner.
- Applicable H01 Connector is eligible for use.
- Authorization conditions are satisfied.
- Valid consent exists where governing policy requires it.

# 8. Trigger

A verified Student or authorized Institution Administrator requests an APAAR or DigiLocker learner-account link.

# 9. Main Business Flow

| Step | Behaviour | Responsible Actor / Domain | Result |
|---|---|---|---|
| 1 | Initiate the platform-specific link request. | Student or Institution Administrator / H01 boundary | Link lifecycle enters `Requested`; corresponding `...LinkRequested` event is published by H01. |
| 2 | Resolve authoritative Student, Institution and Person/Identity references without modifying them. | H01 with A/B/C references | Source ownership is preserved. |
| 3 | Validate authorization, connector eligibility and consent where required. | H01 | Request may proceed only if governed prerequisites are satisfied. |
| 4 | Submit the governed linking request to the target government platform through H01's integration boundary. | H01 | Government outcome is awaited/received at business level. |
| 5 | Record successful government acknowledgement/reference. | H01 | Link reaches `Linked`; `APAARLinked` or `DigiLockerLinked` is published. |
| 6 | Preserve audit evidence. | H01 / Platform audit capability | Successful link outcome remains traceable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Institution-initiated linking | Governing policy permits an Institution Administrator to initiate the link | The Institution Administrator initiates the same governed linking path; required Student consent remains mandatory where applicable. | The same `Linked` business outcome is reached if all governing conditions are satisfied. | H01 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Government platform conclusively rejects or fails the link request. | The current request cannot reach the successful `Linked` outcome and enters `Failed`. | H01 records `Failed` and publishes `APAARLinkFailed` or `DigiLockerLinkFailed`. A later authorized attempt is a new governed request. | H01 |
| Required consent is absent, withdrawn, expired or otherwise invalid. | Linking cannot validly proceed. | No link is established. Progression requires a separately governed valid consent condition before a new or continuing permitted request may proceed. | H01 |
| Student, Institution or Person/Identity reference is invalid or non-authoritative. | The request cannot validly proceed. | The canonical owner must establish or correct the authoritative source fact before linking can proceed; H01 does not alter the source fact. | A/B/C canonical owner with H01 consuming the corrected fact |
| Applicable Connector is ineligible for use. | The link cannot be established through that Connector. | Linking remains blocked until the Connector is governed back to an eligible state under H01 connector governance. | H01 |

# 12. Business Rules and Decision Constraints

- H01 owns integration/link outcome, not Student, Institution or Person/Identity truth.
- Student or Institution Administrator may initiate where policy permits.
- Required consent remains mandatory regardless of initiating actor.
- Link lifecycle is `Requested → Linked` or `Requested → Failed`.
- `Failed` is terminal for that request and is not successful completion of this scenario.
- Consent lifecycle remains separately `Pending`, `Granted`, `Withdrawn`, `Expired`.
- H01 retains canonical publication of all six existing link events.

# 13. State and Lifecycle Impact

| Concept | Impact | Result | Owner |
|---|---|---|---|
| Student | Referenced only | Unchanged | C |
| Institution | Referenced only | Unchanged | B |
| Person/Identity | Referenced only | Unchanged | A |
| Platform-specific learner link request | Governed H01 path | Successful path: `Requested → Linked`; failure path: `Requested → Failed` | H01 integration/Publication boundary |
| Consent | Referenced/validated under existing lifecycle | Unchanged unless separate consent action occurs | H01 |

# 14. Business Events and Cross-Domain Collaboration

Canonical H01 events remain exactly:

- `APAARLinkRequested`
- `APAARLinked`
- `APAARLinkFailed`
- `DigiLockerLinkRequested`
- `DigiLockerLinked`
- `DigiLockerLinkFailed`

H01 is the publisher. A/B/C supply authoritative references and platform capabilities without acquiring H01 event ownership.

# 15. Business Authorization, Approval and Audit Obligations

Student and Institution Administrator are the authorized business initiating roles subject to governing policy. Institution initiation cannot bypass required Student consent. Audit evidence preserves initiating actor/context, source references, consent basis, target platform, government acknowledgement/reference where available, and final H01 request outcome.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Authoritative source references are validated.
- Applicable consent, authorization and connector conditions are satisfied.
- The platform-specific learner link reaches `Linked`.
- `APAARLinked` or `DigiLockerLinked` is published by H01 as applicable.
- Audit history for the successful link outcome is preserved.

## 16.2 Postconditions

- The governed learner-account association is established.
- A/B/C retain canonical ownership of Person/Identity, Institution and Student.
- H01 retains ownership of the learner-link outcome and its canonical events.
- A request that reaches `Failed` is preserved as failure history and does not satisfy successful scenario completion.

# 17. Open Issues, Assumptions and Deferred Questions

**Open Issues:** None.  
**Assumptions:** None.  
**Deferred Questions:** None.
