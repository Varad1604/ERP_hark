# External Education Provider to Governed Active or Retired Relationship

## 1. Scenario Metadata

**Scenario ID:** L3-H-072  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** External Education Integration  
**Participating Domains:** A, B, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-A03_Authentication_Workflows.md; L2-B01_Institution_Workflow.md; L2-H10_External_Education_Integration_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Govern an institutional relationship with an external education provider from proposal or change through due diligence, approval or rejection, activation, suspension, and permanent retirement while preserving institutional, identity, and authentication ownership outside H10.

## 3. Business Outcome

Due diligence and approval establish, reject, suspend, or retire the provider relationship; retired providers require a new governed onboarding to return

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An institution proposes or changes an external education provider relationship.

### 4.2 End Boundary

The provider relationship is governed to an approved Active position, rejected onboarding outcome, temporary Suspended position, or terminal Retired position; a retired relationship is not reactivated in place.

### 4.3 In Scope

- External provider registration, verification, certification, approval, activation, suspension and retirement governed by H10.
- Institutional reference and collaboration context supplied by Section B without ownership transfer.
- Authentication and credential facts supplied by Section A under their canonical ownership.
- Preservation of provider relationship and governance history.

### 4.4 Out of Scope

- Institution lifecycle changes owned by Section B.
- Person, Identity, Credential, Authentication Session or Authentication Provider lifecycle ownership in Section A.
- Connector configuration and lifecycle outcomes belonging to L3-H-073.
- Any external enrollment, learning, credit-transfer or credential-verification exchange after provider relationship establishment.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| External Education Integration (H) | Owns the External Provider and provider-relationship governance lifecycle and final H10 relationship outcome. |
| Platform Foundation (A) | Retains canonical ownership of identity and authentication artifacts referenced during trust and authentication checks. |
| Institution & Structure (B) | Retains authoritative institutional and organisation context referenced by the provider relationship. |

Cross-domain validation and reference consumption do not transfer ownership to H10.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Integration Administrator | Coordinates provider registration, governance progression and provider lifecycle administration. | Initiates / performs |
| Security Administrator | Reviews authentication, credential and trust conditions required before operational use. | Reviews |
| Registrar | Provides institutional collaboration oversight where the provider relationship affects institutional partnership. | Reviews |
| Compliance Officer | Reviews applicable regulatory and audit obligations in provider governance. | Reviews |

## 7. Preconditions and Entry Criteria

- The institution and relevant organisational context are authoritative and available from Section B.
- Any identity or authentication references required for the provider relationship remain available from their canonical Section A owners.
- The provider proposal contains the metadata required by H10 provider validation rules.

## 8. Trigger

An institution proposes or changes an external education provider relationship

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register or identify the proposed external education provider relationship. | Integration Administrator / H10 | A governed provider onboarding record exists. |
| 2 | Validate provider identity, institutional legitimacy, provider category, required metadata and relevant institutional references. | H10 with A/B references | Provider due-diligence inputs are established without transferring source ownership. |
| 3 | Complete required connector certification, security assessment, compliance review and trust verification. | H10 / Security Administrator / Compliance Officer | Provider is eligible for the governed approval decision or rejection path. |
| 4 | Record the institutional provider approval decision. | H10 governance | Approved providers may proceed to activation; rejected onboarding does not proceed to operational use. |
| 5 | Where approved, activate the provider relationship for the permitted scope. | External Education Integration | Provider relationship reaches Active. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Onboarding rejected | Validation, security, certification or approval does not satisfy governance requirements | Stop activation and preserve the rejected onboarding decision and evidence. | Provider relationship is not established for operational use. | External Education Integration |
| Provider restricted or suspended | Risk, policy, incident or governance conditions require limitation or halt | Apply the H10 provider trust lifecycle restriction or suspension controls. | Relationship is Restricted or Suspended without creating a new provider owner. | External Education Integration |
| Provider retired | Governance permanently ends the relationship | Transition the provider to Retired and preserve historical relationship evidence. | Retired is terminal for the existing provider relationship. | External Education Integration |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Provider onboarding cannot satisfy required verification or certification | Operational activation is prohibited. | Preserve the failed/rejected onboarding outcome and permit remediation only through governed onboarding review. | External Education Integration |
| Suspended provider is remediated | Existing communication remains halted until governance conditions are restored. | Re-validation and governed approval may restore Active status where H10 permits. | External Education Integration |
| A retired provider seeks a future relationship | The retired relationship cannot return to an active state. | Begin a new governed onboarding rather than reactivating the retired relationship in place. | External Education Integration |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| PV-001 — Provider Code must be unique. | External Education Integration | Prevents duplicate provider registration. | L2-H10 §12.3 |
| PV-004 — Active providers require at least one operational endpoint. | External Education Integration | Prevents activation without a usable governed provider endpoint. | L2-H10 §12.3 |
| PV-005 — Authentication configuration is mandatory before activation. | External Education Integration | Requires authentication readiness before provider activation. | L2-H10 §12.3 |
| PV-006 — Provider certification must be completed before production use. | External Education Integration | Prevents operational use before governed certification. | L2-H10 §12.3 |
| PV-007 — Suspended providers cannot initiate new synchronization jobs. | External Education Integration | Enforces suspension as a business prohibition on new exchanges. | L2-H10 §12.3 |
| PV-008 — Retired providers cannot receive new requests. | External Education Integration | Makes retirement terminal for the existing provider relationship. | L2-H10 §12.3 |
| Credential and Authentication Provider lifecycles are owned exclusively by L2-A03. | Platform Foundation — Authentication | H10 may reference authentication outcomes but must not take over credential/authentication-provider ownership. | L2-A03 Canonical Ownership |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| External Provider / Provider Trust | Pending Verification / Verified / Certified / Active | Verification, certification, activation, restriction, suspension or permanent retirement. | Active / Restricted / Suspended / Retired | External Education Integration | L2-H10 §4.9 |
| Credential / Authentication Provider | Existing authoritative A03 lifecycle position | Referenced for provider authentication/trust prerequisites only; H10 does not own state change. | Unchanged by H10 | Platform Foundation — Authentication | L2-A03 Canonical Ownership |
| Institution / organisation context | Existing authoritative institutional position | Referenced to establish the institutional relationship context. | Unchanged by H10 | Institution & Structure | Locked L1 / L2-B01 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| ProviderRegistered | External Provider aggregate | H10 connector, configuration, monitoring and security capabilities | Records governed provider registration. | L2-H10 §6.3 / §10.3 |
| ProviderActivated | External Provider aggregate | H10 synchronization and external-exchange capabilities | Records operational provider activation. | L2-H10 §6.3 / §10.3 |
| ProviderSuspended | External Provider aggregate | H10 external-exchange capabilities | Records temporary halt of provider communication. | L2-H10 §6.3 / §10.3 |
| ProviderRetired | External Provider aggregate | H10 consumers of provider relationship state | Records permanent provider retirement. | L2-H10 §6.3 / §10.3 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

- H10 consumes authoritative institutional and organisation references without changing Section B ownership.
- H10 consumes Section A identity/authentication outcomes where required; Credential and Authentication Provider lifecycles remain owned by L2-A03.
- No additional named synchronous business contract is defined upstream for this scenario.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register provider | Integration Administrator | H10 enterprise role and provider command authority. | External Education Integration |
| Review authentication/trust readiness | Security Administrator | H10 enterprise role for authentication and credential governance; A03 retains credential lifecycle ownership. | External Education Integration / Platform Foundation — Authentication |
| Activate provider | Integration Administrator / Security Administrator | Provider must satisfy verification, certification, authentication and governance prerequisites. | External Education Integration |
| Retire provider | Integration Administrator under governance decision | Retired is terminal for the existing provider relationship. | External Education Integration |

### 15.2 Separation of Duties, Overrides and Exceptions

Provider governance separates operational activation from the preceding validation, security and certification evidence. No scenario-specific override authority beyond the upstream H10 governance model is invented here.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Provider due diligence and certification | Provider validation, certification, security and approval evidence | External Education Integration |
| Provider lifecycle decision | Registration, activation, suspension/restriction or retirement history | External Education Integration |
| Authentication evidence | Canonical authentication/credential evidence retained by its owning Section A domain | Platform Foundation — Authentication |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The provider proposal has reached an authoritative approved/activated, rejected, suspended/restricted, or retired business outcome supported by the H10 governance path.
- Section A and Section B ownership remains unchanged.
- Provider governance history and decision evidence are retained.

### 16.2 Postconditions

- An approved relationship may be used only within its governed provider scope.
- Suspended providers cannot initiate new H10 exchanges.
- Retired is terminal for the existing provider relationship; return requires new governed onboarding under the frozen scenario semantics.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `ProviderOnboardingRejected` is defined in the H10 Provider Onboarding Saga publication list, but it is absent from the canonical H10 Provider Event catalogue and no aggregate publisher is identified there. | Rejection behaviour is supported, but the canonical event catalogue/publisher for the rejection fact is internally incomplete. The scenario therefore does not invent a rejection event publisher. | Reconcile the H10 provider-event catalogue and publisher assignment before locking this scenario. |
| H10 describes both an External Provider aggregate lifecycle and a Provider Trust lifecycle using different intermediate state vocabulary. | The end positions Active, Suspended and Retired remain aligned, but intermediate vocabulary is inconsistent. | Reconcile H10 lifecycle terminology before lock. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
