# External Trust Requirement to Active or Revoked Authentication Relationship

## 1. Scenario Metadata

**Scenario ID:** L3-H-074  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** External Education Integration  
**Participating Domains:** A, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-A02_Person_and_Identity_Workflows.md; L2-A03_Authentication_Workflows.md; L2-H10_External_Education_Integration_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Govern the external-provider trust requirement while preserving the canonical Section A ownership of credentials and authentication-provider lifecycles and the H10 ownership of external-provider trust/orchestration.

## 3. Business Outcome

The trust relationship is verified, activated, rotated, suspended, expired, or revoked under security policy

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A provider requires credentials, certificates, consent, or trust establishment.

### 4.2 End Boundary

The external-provider trust/authentication relationship reaches the frozen verified, activated, rotated, suspended, expired or revoked outcome without silently transferring Credential or Authentication Provider ownership from Section A to H10.

### 4.3 In Scope

- H10 external-provider trust verification and provider trust governance.
- Section A-owned credential and authentication-provider lifecycle facts required to support provider trust.
- Security-policy-governed trust activation, suspension and revocation where authoritative ownership and publication are defined.
- Audit evidence for the business trust decision.

### 4.4 Out of Scope

- Person and Identity lifecycle changes.
- Re-definition of Credential or Authentication Provider ownership.
- Connector lifecycle and configuration activation governed by L3-H-073.
- Later architecture-level implementation concerns.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| External Education Integration (H) | Owns external-provider trust orchestration, provider trust status and H10 integration relationship governance. |
| Platform Foundation (A) | L2-A03 exclusively owns Credential, Authentication Provider and their lifecycles; L2-A02 owns canonical identity references. |

The frozen scenario cannot be interpreted as transferring Section A Credential or Authentication Provider lifecycle ownership to H10.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Integration Administrator | Coordinates the H10 provider trust requirement and integration relationship governance. | Initiates / performs |
| Security Administrator | Reviews authentication, credential and trust conditions defined by H10 governance. | Reviews |

L2-A03 does not define a canonical Actors section; no additional A03 business actor is invented here.

## 7. Preconditions and Entry Criteria

- An external provider relationship exists or is in governed onboarding.
- The provider trust requirement and applicable authentication policy are identified.
- Any Person/Identity reference remains canonically owned by L2-A02.
- Any Credential or Authentication Provider state remains canonically owned by L2-A03.

## 8. Trigger

A provider requires credentials, certificates, consent, or trust establishment

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the external-provider trust requirement and the applicable provider relationship. | Integration Administrator / H10 | Trust requirement is established for governance. |
| 2 | Verify provider legitimacy and the H10 provider-trust prerequisites. | External Education Integration | Provider may proceed toward certified/active trust under H10. |
| 3 | Where a credential or Authentication Provider lifecycle action is required, preserve L2-A03 as the canonical owner of that action and resulting state. | Platform Foundation — Authentication | Credential/authentication-provider ownership is not transferred to H10. |
| 4 | Record the H10 provider trust outcome using only the lifecycle positions and events that are authoritative. | External Education Integration | Provider trust can reach supported Verified, Certified, Active or Suspended positions. |

The flow cannot be completed authoritatively for every frozen rotation/expiration/revocation branch because the locked sources conflict on ownership and event publication.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Provider trust suspended | H10 risk, incident or governance conditions require temporary halt | Apply H10 Provider Trust lifecycle suspension. | Provider trust is Suspended. | External Education Integration |
| Credential rotated | Security policy requires credential replacement | A03 owns the Credential lifecycle and publishes `CredentialRotated`; H10 may consume the resulting authentication fact. | Credential rotation remains an A03-owned outcome. | Platform Foundation — Authentication |
| Credential expires | Credential validity ends | A03 owns expiration and publishes `PlatformCredentialExpired`. | Credential expiration remains an A03-owned outcome. | Platform Foundation — Authentication |
| Credential revoked | Credential is permanently invalidated | A03 owns revocation and publishes `CredentialRevoked`. | Credential revocation remains an A03-owned outcome. | Platform Foundation — Authentication |
| Trust relationship revoked | H10 external-provider trust must be removed | H10 lists `TrustRelationshipRevoked`, but its canonical aggregate publisher and corresponding trust-relationship lifecycle state are not defined. | Upstream resolution required. | External Education Integration |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Provider fails trust verification | Provider cannot proceed to governed operational trust. | Remain outside Active trust and resolve through the H10 verification/certification path. | External Education Integration |
| Credential becomes suspended, expired or revoked | Authentication eligibility changes under the A03-owned credential lifecycle. | H10 must respect the authoritative A03 outcome; it must not directly change the Credential lifecycle. | Platform Foundation — Authentication / External Education Integration |
| H10 trust revocation is required | Frozen scenario requires a revoked authentication relationship. | BLOCKED: H10 does not provide a canonical trust-relationship state model or publisher that can be reconciled cleanly with A03 ownership. | Upstream architecture resolution required |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H10 provider trust must not become Active without verification and certification. | External Education Integration | Prevents unverified provider trust activation. | L2-H10 §4.9 |
| H10 suspended providers cannot initiate new synchronization. | External Education Integration | Enforces suspension at the provider-trust boundary. | L2-H10 §4.9 / PV-007 |
| Authentication exclusively owns Credential and Authentication Provider lifecycles. | Platform Foundation — Authentication | H10 cannot create, rotate, expire, suspend or revoke A03-owned credentials as their lifecycle owner. | L2-A03 Canonical Ownership / INT-301 |
| Authentication is responsible for credential rotation, expiration, suspension and revocation. | Platform Foundation — Authentication | These lifecycle facts must originate from A03. | L2-A03 Domain Responsibilities / event model |
| Person and Identity remain owned by L2-A02. | Platform Foundation — Person & Identity | Trust establishment may reference identity but cannot alter identity ownership. | L2-A02 Canonical Ownership |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Provider Trust | Pending Verification / Verified / Certified / Active | Verify, certify, activate, restrict, suspend or retire provider trust. | Verified / Certified / Active / Restricted / Suspended / Retired | External Education Integration | L2-H10 §4.9 |
| Credential | A03 credential lifecycle | Rotation, suspension, expiration and revocation are A03-owned credential state changes. | Authoritative A03 credential position | Platform Foundation — Authentication | L2-A03 Canonical Ownership / Credential lifecycle |
| Authentication Provider | A03 Authentication Provider lifecycle | Any Authentication Provider lifecycle effect remains outside H10 ownership. | Authoritative A03 provider position | Platform Foundation — Authentication | L2-A03 Canonical Ownership |

H10 defines no separate canonical lifecycle for a `Trust Relationship` aggregate containing all frozen outcomes `rotated`, `expired` and `revoked`.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CredentialRotated | Credential aggregate (L2-A03) | H10 and authentication consumers where applicable | Records A03-owned credential replacement. | L2-A03 Aggregate-to-Event Matrix |
| CredentialSuspended | Credential aggregate (L2-A03) | H10 and authentication consumers where applicable | Records A03-owned credential suspension. | L2-A03 Aggregate-to-Event Matrix |
| PlatformCredentialExpired | Credential aggregate (L2-A03) | H10 and authentication consumers where applicable | Records A03-owned credential expiration. | L2-A03 Aggregate-to-Event Matrix |
| CredentialRevoked | Credential aggregate (L2-A03) | H10 and authentication consumers where applicable | Records A03-owned credential revocation. | L2-A03 Aggregate-to-Event Matrix |

### 14.2 Business Contracts and Cross-Domain Hand-offs

- H10 must consume Section A authentication/credential outcomes without assuming their lifecycle ownership.
- H10 lists `TrustRelationshipEstablished` and `TrustRelationshipRevoked`, but the source does not identify a canonical aggregate publisher or a corresponding trust-relationship aggregate lifecycle. No publisher or missing contract is invented here.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate H10 trust establishment | Integration Administrator | H10 enterprise role for integration governance. | External Education Integration |
| Review H10 trust/security readiness | Security Administrator | H10 enterprise role for authentication and credential governance context. | External Education Integration |
| Rotate, suspend, expire or revoke Credential | No A03 actor is defined in the A03 canonical Actors section | The lifecycle authority belongs to L2-A03, not H10. | Platform Foundation — Authentication |

### 15.2 Separation of Duties, Overrides and Exceptions

No H10 role may be interpreted as transferring A03 Credential or Authentication Provider lifecycle authority. The exact cross-domain decision/actor hand-off for the frozen trust-revocation outcome is not defined upstream.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| H10 provider trust verification/activation/suspension | Provider trust governance and decision history | External Education Integration |
| Credential rotation/suspension/expiration/revocation | A03 credential lifecycle and event history | Platform Foundation — Authentication |
| Identity reference | Canonical identity history remains with A02 | Platform Foundation — Person & Identity |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- H10 provider trust has reached a source-supported governed trust position.
- Any credential lifecycle change remains authoritative in A03.
- No H10 content re-owns A03 Credential or Authentication Provider lifecycle.
- The unresolved frozen trust-revocation/relationship-lifecycle conflict is recorded rather than invented.

### 16.2 Postconditions

- H10 may act only on authoritative authentication outcomes supplied by Section A.
- A suspended H10 provider trust cannot initiate new exchanges.
- The scenario cannot be considered architecture-lockable until the ownership/lifecycle/event conflict is resolved upstream.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-A03, under the approved INT-301 ownership ruling, exclusively owns Credential and Authentication Provider lifecycles and states that no other domain may directly create, modify, expire or revoke them. H10 simultaneously claims authentication orchestration responsibilities including credential rotation and publishes H10-side credential-expiration/rotation events. | The frozen H074 outcome includes rotation, suspension, expiration and revocation, but the owning domain and publisher cannot be assigned consistently from the locked sources without violating canonical ownership. | Reconcile H10 authentication/trust responsibilities with L2-A03 INT-301 ownership and define the authoritative cross-domain contract. |
| H10 lists `TrustRelationshipEstablished` and `TrustRelationshipRevoked` in its authentication event catalogue, but does not define a Trust Relationship aggregate, lifecycle, or canonical aggregate publisher. | The frozen final outcome requires an Active or Revoked authentication relationship, but the authoritative lifecycle and event publisher for that relationship are incomplete. | Define the canonical trust-relationship owner, lifecycle and event publisher upstream. |
| H10 Provider Trust lifecycle has Verified, Certified, Active, Restricted, Suspended and Retired positions, but no Expired or Revoked trust-relationship states; A03 supplies expiration/revocation for Credentials instead. | The frozen final outcome cannot be mapped to one authoritative lifecycle without conflating H10 provider trust with A03 credential lifecycle. | Upstream architecture must explicitly separate and connect the provider-trust lifecycle and credential lifecycle for H074. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
