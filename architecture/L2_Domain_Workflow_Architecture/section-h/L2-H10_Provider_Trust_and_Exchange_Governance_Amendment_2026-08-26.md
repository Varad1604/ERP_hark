# L2-H10 — External Education Integration Workflows — Provider Trust and Exchange Governance Amendment

| Attribute | Value |
|---|---|
| Target | L2-H10 — External Education Integration Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-EXTEDU-TRUST-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-H10 architecture. It resolves H10 lifecycle gaps while preserving Section A ownership of authentication credentials and authentication-provider lifecycle facts.

## 1. Authentication Ownership Boundary

Section A / L2-A03 remains the sole canonical owner of authentication credentials, authentication-provider identity/configuration and authentication-provider lifecycle facts.

H10 SHALL NOT create, activate, revoke, retire or publish lifecycle events for an A03 Authentication Provider or Credential.

H10 owns a separate **External Education Provider Trust** relationship. Provider Trust records whether H10 is institutionally permitted to exchange governed education information with a referenced external provider under the applicable institutional/legal/business context.

Provider Trust references, but does not re-own, A03 authentication facts used to establish secure interaction.

## 2. External Connector Configuration Lifecycle

Canonical H10 External Connector Configuration lifecycle:

`Draft → Under Review → Approved → Active → Retired`

Branches:

- `Under Review → Rejected`
- `Active → Suspended → Active`
- an invalid or harmful newly effective version may undergo governed rollback to a prior eligible approved configuration; rollback creates a new effective configuration decision and preserves immutable version history.

Canonical H10 events added:

- `ExternalConnectorConfigurationApproved`
- `ExternalConnectorActivated`
- `ExternalConnectorSuspended`
- `ExternalConnectorRollbackAuthorized`
- `ExternalConnectorRolledBack`
- `ExternalConnectorRetired`

## 3. External Education Provider Trust Lifecycle

Canonical H10 Provider Trust lifecycle:

`Requested → Under Review → Approved → Active → Revoked`

Branches:

- `Under Review → Rejected`
- `Active → Suspended → Active`
- `Active / Suspended → Expired` when the governed trust period ends without renewal.

`Revoked` and `Expired` end the current Provider Trust record. Re-establishment requires a new governed trust decision. Neither state changes A03 authentication-provider or credential lifecycle.

Canonical H10 events added:

- `ExternalProviderTrustApproved`
- `ExternalProviderTrustActivated`
- `ExternalProviderTrustSuspended`
- `ExternalProviderTrustRevoked`
- `ExternalProviderTrustExpired`

## 4. External Enrollment Exchange Outcome

H10 owns only the external exchange transaction/relationship, not the canonical Student enrollment fact.

Canonical Enrollment Exchange lifecycle:

`Requested → Validation Pending → Accepted / Rejected`

If an accepted external request has a bounded validity period and the owning Student/Academic domain has not established the corresponding canonical enrollment before that period ends, the H10 exchange may become `Expired` for exchange purposes.

Canonical H10 events added:

- `ExternalEnrollmentExchangeAccepted`
- `ExternalEnrollmentExchangeRejected`
- `ExternalEnrollmentExchangeExpired`

Acceptance of the exchange does not itself create or mutate Student enrollment; the owning domain decides its own canonical outcome.

## 5. External Credential Verification Result

H10 may govern an external verification request and record the resulting external verification evidence, while the source credential remains owned by its canonical source domain and authentication credentials remain A03-owned where applicable.

Canonical verification lifecycle:

`Requested → Verification Pending → Verified / Rejected`

An external verification result is immutable evidence for that request. A later verification creates a successor request/result rather than overwriting the prior result.

Canonical H10 events added:

- `ExternalCredentialVerificationVerified`
- `ExternalCredentialVerificationRejected`

These events mean H10 recorded the governed external verification outcome; they do not change the lifecycle of the source credential.

## 6. Governed External Education Data Exchange

Canonical Exchange Transfer lifecycle:

`Prepared → Validation Pending → Sent → Receipt Pending → Accepted`

Governed alternative outcomes:

- `Validation Pending → Rejected`
- `Receipt Pending → Rejected`
- `Validation Pending / Receipt Pending → Quarantined` when the exchange cannot be safely accepted or rejected until a governed discrepancy is resolved.
- `Quarantined → Validation Pending` after correction/revalidation.

Canonical H10 events added:

- `ExternalEducationTransferAccepted`
- `ExternalEducationTransferRejected`
- `ExternalEducationTransferQuarantined`

Acceptance of a transfer means the H10 exchange was accepted under the integration contract. It does not by itself mutate canonical Student, Academic, Examination, Finance or other source-domain records; each owning domain processes accepted facts through its own governed contract/workflow.

## 7. Business Authority

The **External Education Integration Governance Authority** is the canonical institutional business authority for H10 connector approval/rollback, Provider Trust approval/revocation, and governed exchange exception decisions defined by this amendment.

Integration administration may execute recorded decisions but does not acquire ownership of A03 authentication or source-domain business facts.

## 8. Downstream Effect

This amendment resolves the H10 gaps identified by L3-H-073, L3-H-074, L3-H-076, L3-H-079 and L3-H-081, while explicitly resolving the A03/H10 ownership conflict in favor of A03 for authentication-provider and credential lifecycle.

## 9. Locked Decisions

The A03/H10 ownership boundary, H10 lifecycles, authority and events in §§1–7 are Locked L2 decisions under ADR-H-EXTEDU-TRUST-001.
