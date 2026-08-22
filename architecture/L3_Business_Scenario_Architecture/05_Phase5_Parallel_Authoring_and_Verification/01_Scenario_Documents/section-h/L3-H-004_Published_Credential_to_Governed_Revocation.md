# Published Credential to Governed Revocation

# 1. Scenario Metadata

**Scenario ID:** L3-H-004  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** C, D, F, H, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H01 — APAAR & DigiLocker Workflows; applicable canonical credential-owning source-domain architecture; Section A Authorization and Audit architecture  
**Referenced By:** Credential correction, republication, reconciliation, government publication history, and audit activities

# 2. Scenario Purpose

Apply a source-authorised revocation to an H01 government publication without allowing H01 to revoke or rewrite the canonical source credential itself, and preserve the terminal government-publication history.

# 3. Business Outcome

The external publication is revoked where supported and the terminal revocation outcome is recorded without deleting its history

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authoritative source establishes that a government-published credential must be revoked.

## 4.2 End Boundary

The scenario ends when H01 records the government revocation outcome for the previously Published H01 Publication, or records that revocation cannot be completed because the connector does not support it, while preserving publication history.

## 4.3 In Scope

- Consuming the authoritative source-owned revocation decision/reference.
- Confirming the credential was previously published through H01.
- Validating H01 revocation authorization and connector revocation capability.
- Using the H01 Revocation Contract at the government boundary.
- Recording `Revoked` where government revocation is supported and completed.
- Publishing the canonical H01 revocation events.
- Preserving original publication and revocation history.

## 4.4 Out of Scope

- Making the source-domain academic/financial/student decision to invalidate or correct the credential.
- Deleting or rewriting the source credential.
- Treating unsupported government revocation as if it succeeded.
- Corrected credential republication, which is a separate scenario.
- Technical integration or implementation mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Retains source-owned student/qualification context where applicable. |
| Academic Delivery / Section D | Retains source-owned academic context where applicable. |
| Finance / Section F | Frozen participating domain; no universal Finance revocation decision or hand-off is defined by H01 for this generic scenario. |
| APAAR & DigiLocker / Section H | Owns the H01 Publication revocation lifecycle, government revocation transaction/status, and H01 revocation events. |
| Examination / Section I | Retains examination/credential authority where the source credential is examination-owned. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Institution Administrator | Initiates H01 credential revocation after the authoritative source establishes the revocation requirement. | Locked H01 authorization material assigns the revocation request to Institution Administrator. |

The source-domain actor who establishes the credential revocation decision is not universally defined by H01 and remains source-owned.

# 7. Preconditions and Entry Criteria

- The source credential and its revocation decision/reference are authoritative.
- The corresponding H01 Publication exists and is `Published`.
- Institutional authorization for the H01 revocation request is valid.
- The selected connector's revocation capability is known.
- Original publication history and government reference remain available.

# 8. Trigger

An authoritative source establishes that a government-published credential must be revoked

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the authoritative source-owned revocation requirement for a credential already published through H01. | APAAR & DigiLocker consuming source-domain decision | H01 receives a revocation basis without taking source credential ownership. |
| 2 | Resolve the existing `Published` H01 Publication and its government reference. | APAAR & DigiLocker | Revocation targets the correct publication history. |
| 3 | Validate institutional authorization and connector revocation capability. | Institution Administrator / APAAR & DigiLocker | Unsupported or unauthorised revocation is not represented as complete. |
| 4 | Record the H01 revocation request and publish `CredentialRevocationRequested`. | APAAR & DigiLocker | The H01 revocation lifecycle begins. |
| 5 | Submit the governed revocation request to the government platform and receive the revocation response. | APAAR & DigiLocker | External revocation outcome is available. |
| 6 | On successful supported revocation, transition the H01 Publication from `Published` to `Revoked`. | APAAR & DigiLocker | H01 government-publication lifecycle reaches its terminal revocation state. |
| 7 | Publish `CredentialRevoked` and preserve the original publication plus revocation evidence. | APAAR & DigiLocker / Platform Audit | The terminal revocation outcome is traceable without deleting history. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Connector does not support revocation | Government connector capability is absent. | Do not claim `Revoked`; retain the authoritative source revocation requirement and H01 publication history for governed follow-up. | APAAR & DigiLocker |
| Publication is not currently Published | H01 cannot locate an eligible published publication. | Reject the revocation request for this H01 lifecycle. | APAAR & DigiLocker |
| Government rejects revocation | External platform does not accept the governed request. | Preserve the existing H01 publication state and record the unsuccessful revocation evidence for follow-up. | APAAR & DigiLocker |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Source revocation authority is unclear | H01 cannot determine whether revocation is legitimate. | Do not initiate government revocation until an authoritative source decision is available. | Source-owning domain / APAAR & DigiLocker |
| Government revocation cannot be completed | H01 cannot represent the external publication as revoked. | Preserve source revocation evidence and route the publication to governed follow-up without deleting history. | APAAR & DigiLocker |
| Historical government reference is missing | Correct external publication cannot be proven. | Recover authoritative H01 publication evidence before revocation proceeds. | APAAR & DigiLocker |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H01 never owns or modifies academic credentials; source domains remain authoritative. | Source domain / APAAR & DigiLocker | Revocation of the external publication does not rewrite the source credential. | L2-H01 §§2–7, 16 |
| Revocation requires a previously published credential/publication. | APAAR & DigiLocker | Only an eligible `Published` Publication can enter the revocation path. | L2-H01 §31, §60 |
| Connector must support revocation. | APAAR & DigiLocker | Unsupported revocation cannot be claimed successful. | L2-H01 §§31, 43.1 |
| Completed publications cannot be repeated except through approved republish or revocation workflows. | APAAR & DigiLocker | Existing publication identity/history is preserved. | L2-H01 §40 |
| H01 publishes `CredentialRevocationRequested` and `CredentialRevoked` for the defined revocation lifecycle. | APAAR & DigiLocker | Event names and publisher remain canonical. | L2-H01 §§35, 37 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Source credential | Source-owned state establishing revocation/correction requirement | Referenced only. | Source lifecycle remains controlled by its canonical owner. | Applicable source domain | L2-H01 source-authority principle |
| H01 Publication | `Published` | Revocation is requested and completed where supported. | `Revoked` on successful supported government revocation. | APAAR & DigiLocker | L2-H01 §60 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H01 canonically publishes `CredentialRevocationRequested` when the H01 revocation request is established and `CredentialRevoked` when government revocation completes. The exact source-domain event/publisher that establishes the generic credential revocation requirement is not mapped by H01 and is not invented here.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The source-owning domain supplies an authoritative revocation decision/reference. H01 uses its documented Revocation Contract with the government platform and records the resulting government revocation status/reference. Source credential ownership remains upstream.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish credential revocation requirement | Source-domain authority | Must be authoritative before H01 acts; exact role varies by source and is not defined by H01. | Canonical credential-owning source domain |
| Request H01 government revocation | Institution Administrator | Publication must be eligible and connector must support revocation. | APAAR & DigiLocker |

## 15.2 Separation of Duties, Overrides and Exceptions

H01 does not define a universal cross-domain separation-of-duty model between the source revocation decision and H01 Institution Administrator request. None is invented.

## 15.3 Business Audit and Evidence Obligations

Preserve the original publication reference, source credential reference, revocation reason, government reference/status, authorization context, `Published`→`Revoked` lifecycle evidence where successful, and complete immutable history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An authoritative source establishes the credential revocation requirement.
- The corresponding H01 Publication is confirmed as eligible for revocation.
- H01 authorization and connector capability are validated.
- Government revocation is completed where supported.
- H01 records `Revoked` and publishes `CredentialRevoked` on successful completion.
- Original publication history remains preserved.

## 16.2 Postconditions

- The H01 government-publication history shows the terminal revocation outcome where supported.
- The source credential remains owned and governed by its source domain.
- Original publication and revocation evidence remain auditable and historically reconstructable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H01 does not map the generic source-domain revocation trigger to one exact canonical upstream event and publisher across C/D/F/I. | L3 cannot assign a universal source event publisher for all credential types. | Register the source-domain revocation event/contract mapping for each credential type consumed by H01. |
| The frozen participant set includes Finance, but H01 defines no universal Finance-owned revocation decision or hand-off for the generic credential scenario. | Finance authority cannot be invented. | Clarify Finance participation upstream if mandatory for a specific credential class. |
| H01 §65 uses stale section-letter labels for Examination/Document Platform. | Literal use would misstate current current-section ownership. | Current `DEPENDENCY_MAP.md` and locked L1 ownership control; correct H01 metadata upstream. |

**Architecture assessment:** `PASS WITH ISSUES`

The H01 Published→Revoked lifecycle, connector-capability condition, revocation contract, canonical H01 events, and history-preservation outcome are explicit. The unresolved generic source event/publisher and participant metadata do not require local invention to express the H01-owned revocation outcome.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
