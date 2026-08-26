# L2-H01 — Government Learner Account and Credential Publication — Recovery and Republishing Amendment

| Attribute | Value |
|---|---|
| Target | L2-H01 — APAAR and DigiLocker Workflows and locked amendments |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-GOVPUB-RECOVERY-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-H01 architecture. Where it conflicts with earlier H01 wording only on publication failure recovery and republication, this amendment is authoritative. All unaffected H01 decisions remain unchanged.

## 1. Canonical Publication Failure Meaning

`Failed` represents a completed unsuccessful publication attempt. It is terminal for that attempt, but it does not prohibit a later governed recovery attempt for the same institutional credential when recovery is authorized.

A failed attempt is never rewritten or reopened. Recovery establishes a new publication attempt linked to the failed attempt for traceability.

## 2. Recovery Authority

The **Institution Administrator** is the canonical business authority for deciding whether an institution-controlled failed or pending publication shall be retried, manually recovered, or cancelled where a business decision is required.

- Integration Administrator remains limited to connector/integration administration.
- Platform Administrator remains limited to platform availability and shall not override institutional publication decisions or consent.
- No actor may bypass source-domain credential authority or consent requirements.

## 3. Controlled Republishing

A corrected credential that is eligible for republication creates a new immutable publication attempt/version. It shall reference the prior publication record as its predecessor.

The prior publication record remains historical and immutable. It is not overwritten.

For one credential, target publication service and applicable governing context, only one publication record may be treated as the current successful publication at a time. A later successful republication supersedes the prior current publication for current-use purposes while preserving the prior record historically.

## 4. Canonical H01 Events

H01 is the sole semantic publisher of the following additional publication facts:

- `CredentialPublicationRepublished` — a corrected eligible credential has been successfully published as a successor publication.
- `CredentialPublicationSuperseded` — an earlier successful publication is no longer the current publication because a governed successor was successfully published.
- `CredentialPublicationRecoveryAuthorized` — the Institution Administrator authorized a new governed recovery attempt after a failed or pending publication outcome.

These events describe H01 publication facts only. They do not transfer ownership of the source credential.

## 5. Recovery and Cancellation Rules

- Retry/recovery creates a new auditable attempt; it never mutates the prior failed attempt.
- Cancellation of a pending publication preserves the pending-attempt history and prevents it from later being treated as a successful publication.
- Recovery cannot bypass consent, eligibility, source-credential validity, or connector-governance rules.
- A correction to the source credential remains owned by the source domain; H01 consumes the corrected authoritative credential by reference.

## 6. Downstream Effect

This amendment closes the L2 blockers identified by L3-H-005 and L3-H-008 for successor republication, failure recovery, cancellation authority, immutable lineage and current-publication semantics.

## 7. Locked Decisions

The attempt-level meaning of `Failed`, recovery authority, immutable successor republication, current-publication rule and events in §§1–5 are Locked L2-H01 decisions under ADR-H-GOVPUB-RECOVERY-001.
