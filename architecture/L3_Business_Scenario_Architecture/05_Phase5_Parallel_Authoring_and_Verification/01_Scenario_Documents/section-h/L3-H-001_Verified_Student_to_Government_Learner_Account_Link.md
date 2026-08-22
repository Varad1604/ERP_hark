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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H01 — APAAR & DigiLocker Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows; applicable Institution and Student Lifecycle contracts  
**Referenced By:** Government learner-account integration, credential-publication eligibility, and institutional audit activities

# 2. Scenario Purpose

Establish or reject a governed link between a verified SutraERP student and the applicable government learner account while preserving Student, Institution, identity, consent, and external-reference ownership boundaries.

# 3. Business Outcome

A governed learner-account link is established or rejected with the external reference, consent basis, and audit history preserved

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a verified student or authorized institutional process requests an APAAR or DigiLocker link.

## 4.2 End Boundary

The scenario ends when H01 records either the successful learner-account link with its government reference or a controlled rejected/failed link outcome, with consent basis and audit evidence preserved.

## 4.3 In Scope

- Resolution of the canonical Student and Institution references required by H01.
- Verification that the student identity is already verified by the owning source capability.
- Validation of publication/linking consent where policy requires it.
- Use of an active APAAR or DigiLocker connector under H01 ownership.
- Recording of the government reference and link outcome under H01.
- Publication of the target-specific canonical H01 link outcome event.
- Preservation of audit evidence.

## 4.4 Out of Scope

- Creating or modifying the canonical Student, Institution, or identity record.
- Inventing a separate learner-account-link aggregate or lifecycle not defined by H01.
- Technical connector, queue, transport, retry, authentication-protocol, or payload design.
- Credential publication, which is a separate H01 business outcome.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Supplies canonical identity/authorization and audit capabilities used by the governed linking operation. |
| Institution & Structure / Section B | Supplies the authoritative Institution context referenced by the link. |
| Student Lifecycle / Section C | Retains canonical Student ownership; H01 consumes the verified Student reference without modifying it. |
| APAAR & DigiLocker / Section H | Owns the government-linking workflow, H01 link status/external reference, applicable consent record, connector, acknowledgement and H01 outcome events. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student | Initiates an APAAR or DigiLocker linking request where the learner initiates the process; grants or updates consent where applicable. | H01 explicitly supports Student initiation for APAAR/DigiLocker linking and consent actions. |

H01 also permits institution-initiated linking, but its locked architecture does not define the canonical human business role that performs that institutional initiation. No placeholder role is introduced.

# 7. Preconditions and Entry Criteria

- The canonical Student exists.
- The Student identity has already been verified by the authoritative identity/student capabilities.
- The applicable Institution context is valid.
- The target APAAR or DigiLocker connector is active.
- Applicable institutional policy checks pass.
- Consent is valid where policy requires consent for linking/publication.

# 8. Trigger

A verified student or authorized institutional process requests an APAAR or DigiLocker link

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the authorised learner-account linking request and identify whether the target is APAAR or DigiLocker. | APAAR & DigiLocker / Student where learner-initiated | The H01 linking journey is established. |
| 2 | Resolve the canonical Student and Institution references without modifying either source-owned record. | H01 consuming Sections B/C | The link is bound to authoritative institutional context. |
| 3 | Confirm the source-owned student identity verification and applicable authorization/policy conditions. | H01 consuming Section A/C facts | Unverified or unauthorised linking is prevented. |
| 4 | Validate the applicable H01 consent basis where policy requires consent. | APAAR & DigiLocker / Student | Publication/linking authority is established or refused. |
| 5 | Use the active target connector to submit the governed link request to the external government platform. | APAAR & DigiLocker | The external linking decision is requested without changing source Student ownership. |
| 6 | Record the external government response and government reference where supplied. | APAAR & DigiLocker | H01 retains the authoritative integration outcome. |
| 7 | Complete the scenario as a successful link or a controlled rejected/failed outcome and publish the target-specific H01 event. | APAAR & DigiLocker | Link outcome, consent basis and audit history are preserved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| DigiLocker rather than APAAR is requested | The verified learner selects/uses DigiLocker. | Execute the H01 DigiLocker linking workflow under the same source-ownership constraints. | `DigiLockerLinked` or `DigiLockerLinkFailed` outcome is recorded. | APAAR & DigiLocker |
| Government platform rejects link | Government validation does not accept the request. | Record the failed/rejected outcome and preserve the government response/reference where available. | Controlled rejection; Student ownership is unchanged. | APAAR & DigiLocker |
| Consent is missing or invalid where required | Required consent is absent, expired or withdrawn. | Do not establish the link; preserve the controlled refusal/failure evidence. | No unauthorised link is created. | APAAR & DigiLocker |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Student or identity reference does not validate | Link would not be trustworthy. | Reject/stop the linking outcome until authoritative source data is valid. | Student Lifecycle / Platform Foundation / H01 |
| Target connector is not active | H01 cannot validly conduct the government link. | Preserve the non-linked state and resume only through an authorised H01 connector lifecycle outcome. | APAAR & DigiLocker |
| Government response is a business rejection | Link cannot be established. | Record the target-specific failed link outcome with audit history. | APAAR & DigiLocker |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H01 owns integration, not Student, Institution, academic, or identity business data. | APAAR & DigiLocker | Linking cannot modify source-owned business records. | L2-H01 §§2–7 |
| APAAR linking requires Student existence, verified identity, policy checks, and an active APAAR connector. | APAAR & DigiLocker / source owners | These conditions gate the H01 linking outcome. | L2-H01 §27 |
| DigiLocker linking requires a verified Student, active DigiLocker connector and satisfied consent. | APAAR & DigiLocker / source owners | These conditions gate DigiLocker linking. | L2-H01 §28 |
| Where policy requires consent, consent validation must precede publication/linking. | APAAR & DigiLocker | Missing/withdrawn consent prevents the governed link. | L2-H01 Principles and Consent Aggregate |
| Every business operation must be authorized before execution. | Platform Foundation — Authorization | Unauthorized linking is prohibited. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Student | Existing verified source-owned state | Referenced for linking only. | Unchanged by H01. | Student Lifecycle | `DEPENDENCY_MAP.md`; L2-H01 non-ownership rules |
| Institution | Existing source-owned state | Referenced as institutional context. | Unchanged by H01. | Institution & Structure | `DEPENDENCY_MAP.md`; L2-H01 |
| H01 linking status within Publication integration model | Link requested / governed H01 processing | Government reference and link result are recorded. | Successful link or controlled failed/rejected outcome; H01 does not define a separate Link aggregate lifecycle. | APAAR & DigiLocker | L2-H01 §§27–28, 35, 37 |
| Consent | Pending/Granted/Expired/Withdrawn as applicable | Consent is validated; a Student action may grant/update/withdraw it in the separate H01 consent lifecycle. | Existing source-defined consent state governs linking. | APAAR & DigiLocker | L2-H01 §§11.2, 62 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

For APAAR, H01 defines `APAARLinkRequested`, `APAARLinked`, and `APAARLinkFailed`. For DigiLocker, H01 defines `DigiLockerLinkRequested`, `DigiLockerLinked`, and `DigiLockerLinkFailed`. These events are produced by the H01 Publication integration model. Consent events such as `ConsentGranted`, `ConsentUpdated`, `ConsentWithdrawn`, and `ConsentExpired` remain H01-owned.

No generic `GovernmentLearnerAccountLinked` event is introduced.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H01 consumes authoritative Student, Institution and identity references and uses its documented Linking Contract with the external government platform. The linking response provides link status and government reference. Source Student/Institution records remain externally owned.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate learner link | Student where learner-initiated | H01 explicitly permits Student initiation for APAAR/DigiLocker link requests. | APAAR & DigiLocker |
| Initiate institution-driven link | Canonical human role not defined in H01 | H01 permits institution initiation but L3 cannot invent the institutional role. | APAAR & DigiLocker / upstream governance |
| Grant/update/withdraw H01 consent | Student | Applies to the source-defined H01 consent scope and lifecycle. | APAAR & DigiLocker |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal additional approval or separation-of-duty rule is defined for normal learner-account linking. None is invented.

## 15.3 Business Audit and Evidence Obligations

The link attempt/outcome must preserve the Student and Institution references, consent basis where applicable, government reference/acknowledgement where supplied, H01 outcome, actor/source of initiation, and immutable audit history. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Canonical Student and Institution references are resolved.
- Student identity verification is confirmed from the authoritative source.
- Applicable H01 policy/consent conditions are satisfied or the link is controlledly rejected.
- The target government platform returns a business outcome.
- H01 records the government reference where available.
- H01 records and publishes the target-specific successful or failed link outcome.
- Audit evidence is preserved.

## 16.2 Postconditions

- A successful APAAR/DigiLocker learner-account link or controlled rejection/failure is recorded under H01.
- Student, Institution, and identity ownership remain unchanged.
- Consent and audit history remain traceable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H01's canonical-section crosswalk records no Actors section, although its command authorization matrix names `Student` and permits institution-initiated linking without defining the corresponding institutional human role. | Student initiation is source-supported, but institution-driven initiation cannot be assigned to a new descriptive role. | Define the canonical institutional linking actor upstream if explicit human authority is required. |
| H01 §65 contains stale section-letter labels (`B — Student Lifecycle`, `C — Academic Management`) that conflict with higher-authority current `DEPENDENCY_MAP.md`, where Section B is Institution & Structure and Section C is Student Lifecycle. | Participant ownership could be misread if the stale labels were followed literally. | `DEPENDENCY_MAP.md` controls; H01 dependency labels should be corrected upstream. |
| H01 implements APAAR/DigiLocker linking through its Publication integration model but does not define a distinct learner-account-link aggregate or state machine. | L3 can record link status and canonical link events but must not invent an additional lifecycle. | Upstream H01 may define a dedicated link aggregate/lifecycle if later architecture requires one. |

**Architecture assessment:** `PASS WITH ISSUES`

The complete link/rejection outcome is source-supported. Actor, stale-label, and dedicated-link-model gaps do not prevent a grounded Draft.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
