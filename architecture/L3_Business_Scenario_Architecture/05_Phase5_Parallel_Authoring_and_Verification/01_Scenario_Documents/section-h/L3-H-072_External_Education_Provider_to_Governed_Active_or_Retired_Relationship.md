# External Education Provider to Governed Active or Retired Relationship

# 1. Scenario Metadata

**Scenario ID:** L3-H-072  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** External Education Integration  
**Participating Domains:** A, B, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H10 — External Education Integration Workflows  
**Referenced By:** External connector, trust, synchronization, enrollment, learning, credit-transfer, verification, reconciliation, and exchange scenarios

# 2. Scenario Purpose

Govern an external education provider relationship from proposed onboarding through verification, certification, approval and activation, with controlled restriction or suspension where required and terminal retirement when the institutional relationship ends, while preserving upstream platform and institutional ownership.

# 3. Business Outcome

Due diligence and approval establish, reject, suspend, or retire the provider relationship; retired providers require a new governed onboarding to return

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An institution proposes or changes an external education provider relationship.

## 4.2 End Boundary

The scenario ends when H10 records a governed provider outcome of activation, onboarding rejection, suspension/restriction, or terminal retirement. A retired provider does not transition back into service through the retired lifecycle; any future relationship requires a new governed onboarding.

## 4.3 In Scope

- Provider registration and due-diligence verification.
- Technical/security certification and institutional approval.
- Provider activation.
- Restriction or suspension under governed risk or incident conditions.
- Terminal provider retirement.
- Preservation of provider relationship and approval history.

## 4.4 Out of Scope

- Connector configuration and lifecycle beyond what is required to establish provider readiness.
- Ownership of institution, identity, student, academic, examination, finance, or workforce records.
- Re-activating a `Retired` provider through an unsupported state transition.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| External Education Integration / H | Owns External Provider lifecycle, provider onboarding orchestration, certification coordination, activation, restriction/suspension, retirement, and integration relationship history. |
| Platform Foundation / A | Retains ownership of canonical platform identity, security and authorization capabilities consumed by the governed integration boundary. |
| Institution & Structure / B | Retains ownership of canonical institutional context referenced by the external provider relationship. |

Participation by A or B does not transfer their canonical entities into H10.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Integration Administrator | Registers and governs the external provider relationship and has source-defined provider activation capability. | Performs / governs |
| Security Administrator | Participates in provider verification, security/trust review and has source-defined provider activation capability. | Reviews / governs |
| Registrar | Provides institutional collaboration oversight where applicable. | Reviews |
| Compliance Officer | Provides regulatory and audit oversight where applicable. | Reviews |

H10 states that governance owns certification, approval, deprecation and retirement decisions, but does not bind every such decision to one singular named enterprise role.

# 7. Preconditions and Entry Criteria

- The proposed provider can be identified as an external education organization.
- Institutional context and applicable integration governance are available.
- No `Retired` H10 provider record is being directly returned to an active state.

# 8. Trigger

An institution proposes or changes an external education provider relationship

# 9. Main Business Flow

1. H10 registers the proposed External Provider.
2. H10 performs the source-defined legal, identity and basic technical verification required before provider trust may advance.
3. Security, protocol and governance checks are completed for provider certification.
4. Required connector, configuration and trust evidence is considered without transferring ownership from the respective canonical domains.
5. The provider relationship proceeds through institutional certification and approval.
6. When operational activation is approved, H10 records the provider as `Active` and publishes the canonical provider activation outcome.
7. If governance cannot approve onboarding, H10 records the source-defined provider-onboarding rejection outcome and does not activate the provider.
8. If risk or incident conditions arise after activation, H10 may move the provider to `Restricted` or `Suspended` according to the locked provider trust lifecycle.
9. If the relationship ends permanently, H10 moves the provider to `Retired` and records the canonical retirement event.
10. Because `Retired` is terminal upstream, any future relationship must begin through a new governed onboarding rather than a direct reactivation.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Onboarding rejected | Verification, security, certification or governance review does not support activation | Record `ProviderOnboardingRejected`; do not activate | Rejected onboarding outcome | H10 |
| Provider restricted | Provider may operate only within constrained scope | Move provider to `Restricted` | Restricted governed relationship | H10 |
| Provider suspended | Risk, incident or governance condition temporarily halts communication | Move provider to `Suspended` | New governed operation halted | H10 |
| Provider retired | Relationship is permanently ended | Move provider to terminal `Retired` | Retirement recorded | H10 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Verification or certification fails | Provider cannot become operational | Retain outside `Active` and complete governed remediation/review or reject onboarding | H10 |
| Active provider becomes unsafe or non-compliant | Continued normal exchange is not permitted | Apply the source-defined restriction or suspension path | H10 |
| Retired provider seeks return | Direct return is prohibited by terminal lifecycle | Begin a new governed onboarding relationship | H10 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H10 owns external-provider lifecycle management and integration orchestration only | H10 | Provider governance does not transfer ownership of institutional or academic facts | L2-H10 §§1–3 |
| Provider cannot reach production operation without verification, certification and approval | H10 | Activation is gated | L2-H10 Provider Onboarding Saga; Provider Trust Lifecycle |
| `Retired` is terminal | H10 | A retired provider cannot transition directly back to active service | L2-H10 §4.9 Provider Trust Lifecycle |
| Every external interaction remains governed and auditable | H10 | Relationship decisions and history remain traceable | L2-H10 core principles |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| External Provider | Pending Verification | Complete provider verification | Verified | H10 | L2-H10 §4.9 |
| External Provider | Verified | Complete certification review | Certified | H10 | L2-H10 §4.9 |
| External Provider | Certified | Record approved operational relationship | Active | H10 | L2-H10 §4.9 |
| External Provider | Active / Restricted | Apply governed temporary halt | Suspended | H10 | L2-H10 §4.9 |
| External Provider | Non-terminal state | Permanently end relationship | Retired | H10 | L2-H10 §4.9 |

H10 represents onboarding rejection through `ProviderOnboardingRejected` rather than a persistent `Rejected` provider state.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Scenario Purpose | Authoritative Source |
|---|---|---|---|
| ProviderRegistered | External Provider / H10 | Records provider registration | L2-H10 Provider Events |
| ProviderVerified | External Provider / H10 | Records completed verification | L2-H10 §4.9 |
| ProviderCertified | External Provider / H10 | Records certification | L2-H10 §4.9 |
| ProviderActivated | External Provider / H10 | Records active governed availability | L2-H10 §4.9 / Provider Events |
| ProviderSuspended | External Provider / H10 | Records temporary suspension | L2-H10 §4.9 / Provider Events |
| ProviderRetired | External Provider / H10 | Records terminal retirement | L2-H10 §4.9 / Provider Events |
| ProviderOnboardingRejected | Provider Onboarding / H10 | Records unsuccessful onboarding decision | L2-H10 Provider Onboarding Saga |

## 14.2 Business Contracts and Cross-Domain Hand-offs

A and B provide their canonical platform/institutional context through approved enterprise boundaries. H10 owns only the external-provider relationship and orchestration record.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The source-defined permission matrix allows Integration Administrator and Security Administrator participation in provider registration/activation. H10 separately assigns certification, approval and retirement decisions to governance without naming one singular role for every decision.

## 15.2 Separation of Duties, Overrides and Exceptions

No external provider becomes active solely because an external party requests access. Verification, certification and institutional approval remain mandatory. No direct transition out of `Retired` is permitted.

## 15.3 Business Audit and Evidence Obligations

Preserve provider identity, institutional relationship context, verification evidence, certification evidence, approval decision, activation history, restriction/suspension decisions, retirement evidence and complete provider history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A provider relationship reaches a source-supported governed outcome.
- Activation occurs only after verification/certification/approval.
- Rejection, suspension/restriction and retirement remain traceable.
- Retirement remains terminal.

## 16.2 Postconditions

An active provider may participate in later H10 integration scenarios, while a rejected or retired provider cannot be treated as active without a new valid governance path.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

H10 defines onboarding rejection as an event but not as a persistent External Provider lifecycle state. It also states that governance owns provider approval/retirement decisions while the enterprise-role table does not identify one singular decision-maker for all provider lifecycle decisions. These gaps do not prevent the frozen relationship outcomes from being represented because the provider lifecycle, activation/suspension/retirement events and onboarding rejection event are explicit.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H10 may clarify whether rejected onboarding has a persistent aggregate state and may bind each provider governance decision to a named enterprise role or explicit authority rule.
