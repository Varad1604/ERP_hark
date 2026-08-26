# Verified Student to Government Learner Account Link

# 1. Scenario Metadata

**Scenario ID:** L3-H-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** A, B, C, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture and Locked Amendment 2026-08-26; `DEPENDENCY_MAP.md`; L2-H01 — APAAR & DigiLocker Workflows; L2-H01 Locked Amendment 2026-08-26; ADR-H-GOVLINK-001; L2-A02 — Person & Identity; authoritative Institution & Structure and Student Lifecycle workflows; applicable Platform Foundation authorization/event/audit architecture  
**Referenced By:** Government learner-link and credential-publication scenarios

# 2. Scenario Purpose

Establish or reject a governed association between a verified canonical Student and an APAAR or DigiLocker learner account while preserving Student, Institution and Person/Identity ownership outside H01 and preserving H01 consent and link history.

# 3. Business Outcome

A governed platform-specific learner-account link reaches either `Linked` or `Failed`, with authoritative source references, consent basis where required, external acknowledgement/reference where available, canonical H01 event and audit history preserved.

# 4. Scenario Boundary and Scope

**Start:** A Student or authorized Institution Administrator requests APAAR/DigiLocker linking.  
**End:** The platform-specific H01 link request reaches `Linked` or `Failed`.

In scope: authoritative Student/Institution/Person references, consent, H01 connector eligibility, request outcome and canonical link events. Out of scope: credential publication, changes to source-domain lifecycles, technical APIs/queues/retries/persistence or creation of a separate learner-link aggregate.

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
| 6 | Preserve audit evidence. | H01 / Platform audit capability | Link outcome remains traceable. |

# 10. Alternate Business Flows

- Institution-initiated linking follows the same H01 business rules as Student initiation and cannot bypass required consent.
- If the government platform conclusively rejects/fails the link, H01 records `Failed` and publishes `APAARLinkFailed` or `DigiLockerLinkFailed`.
- If required consent is not valid, no link is established.

# 11. Failure, Interruption and Business Recovery

- Invalid/non-authoritative Student, Institution or Person/Identity references prevent the request.
- Ineligible connector prevents establishment of the link.
- A `Failed` outcome is terminal for that link request; a later authorized attempt is a new governed request.
- No source Student, Institution or Person/Identity fact is changed to force link success.

# 12. Business Rules and Decision Constraints

- H01 owns integration/link outcome, not Student, Institution or Person/Identity truth.
- Student or Institution Administrator may initiate where policy permits.
- Required consent remains mandatory regardless of initiating actor.
- Link lifecycle is `Requested → Linked` or `Requested → Failed`.
- Consent lifecycle remains separately `Pending`, `Granted`, `Withdrawn`, `Expired`.
- H01 retains canonical publication of all six existing link events.

# 13. State and Lifecycle Impact

| Concept | Impact | Result | Owner |
|---|---|---|---|
| Student | Referenced only | Unchanged | C |
| Institution | Referenced only | Unchanged | B |
| Person/Identity | Referenced only | Unchanged | A |
| Platform-specific learner link request | Governed H01 path | `Requested → Linked` or `Requested → Failed` | H01 integration/Publication boundary |
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

Student and Institution Administrator are the authorized business initiating roles subject to governing policy. Institution initiation cannot bypass required Student consent. Audit evidence preserves initiating actor/context, source references, consent basis, target platform, government acknowledgement/reference where available, and final H01 outcome.

# 16. Completion Criteria and Postconditions

- Authoritative source references are validated.
- Applicable consent/authorization/connector conditions are satisfied or a controlled `Failed` outcome is recorded.
- Link request reaches `Linked` or `Failed`.
- Exact applicable H01 outcome event is published.
- Audit history is preserved.
- A/B/C retain canonical ownership of Person/Identity, Institution and Student.

# 17. Open Issues, Assumptions and Deferred Questions

**Open Issues:** None.  
**Assumptions:** None.  
**Deferred Questions:** None.  
**Architecture assessment:** `PASS`
