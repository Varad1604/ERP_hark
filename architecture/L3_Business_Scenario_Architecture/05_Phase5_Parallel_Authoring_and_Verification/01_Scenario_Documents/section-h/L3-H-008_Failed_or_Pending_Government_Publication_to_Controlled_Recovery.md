# Failed or Pending Government Publication to Controlled Recovery

# 1. Scenario Metadata

**Scenario ID:** L3-H-008  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** A, C, D, F, H, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** L2-H01 — APAAR & DigiLocker Workflows; applicable locked source-domain credential architecture; Section A authorization/audit architecture; `L2-H01_Publication_Recovery_and_Republishing_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern recovery of an H01 publication attempt that fails, remains pending, or requires cancellation while preserving source credential ownership, immutable attempt history and complete recovery lineage.

# 3. Business Outcome

The publication is safely retried, cancelled, or escalated without duplicate external records and with complete recovery lineage

The 2026-08-26 locked H01 amendment establishes the authoritative recovery semantics required for this frozen outcome.

# 4. Scenario Boundary and Scope

The scenario starts when the frozen trigger occurs and ends when the applicable governed recovery decision and resulting publication outcome have been recorded. Source-domain credential, Student, Institution, identity and originating transaction ownership remain outside H01. H01 owns only publication-attempt recovery and publication history.

# 5. Ownership and Participating Domains

- **A:** retains authorization/audit capability ownership.
- **C/D/F/I:** retain source credential and related authoritative facts.
- **H:** owns publication recovery governance, recovery decisions, recovery attempts and publication history.
- A recovery action does not transfer source credential ownership to H01.

# 6. Business Actors, Roles and Responsibilities

**Institution Administrator** is the canonical business authority for deciding whether an institution-controlled failed or pending publication shall be retried, manually recovered, or cancelled where a business decision is required.

Integration Administrator remains limited to connector/integration administration. Platform Administrator remains limited to platform availability and may not override institutional publication decisions or consent.

# 7. Preconditions and Entry Criteria

- An H01 publication attempt exists with preserved identity/history.
- The source credential remains authoritative outside H01.
- The publication attempt is failed, pending/stalled, or cancellation-eligible.
- Applicable consent, eligibility, source-credential validity and connector-governance conditions remain satisfied for any recovery attempt.

# 8. Trigger

A publication fails, stalls, or must be cancelled before terminal completion

# 9. Main Business Flow

1. Resolve the existing publication attempt and preserve its immutable identity, correlation and history.
2. Classify the condition for governed retry/recovery, cancellation or manual recovery.
3. Where a business decision is required, the Institution Administrator authorizes the permitted recovery path.
4. If recovery is authorized after a failed or pending outcome, publish `CredentialPublicationRecoveryAuthorized`.
5. Create a new auditable publication attempt linked to the prior attempt; do not reopen or mutate the prior failed attempt.
6. If cancellation of a pending attempt is authorized, preserve its history and prevent it from later being treated as successful.
7. Complete the resulting governed publication path according to H01 rules.
8. Preserve complete decision, predecessor-attempt and outcome evidence.

# 10. Alternate Business Flows

- A failed attempt remains terminal for that attempt even when a later recovery attempt is authorized.
- A pending attempt may be cancelled where the governing business decision permits cancellation.
- If recovery eligibility, consent, source-credential validity or connector-governance requirements fail, no new recovery attempt proceeds.

# 11. Failure, Interruption and Business Recovery

Recovery never rewrites a failed attempt. Each later retry/recovery is a new auditable attempt linked to the prior attempt. Cancellation preserves the cancelled/pending-attempt history. No recovery path may bypass consent, eligibility, source-credential validity or connector-governance requirements.

# 12. Business Rules and Decision Constraints

- `Failed` is terminal for the individual unsuccessful publication attempt.
- A later governed recovery attempt may be created for the same institutional credential when authorized.
- Recovery creates a new attempt and preserves lineage; it never mutates the failed attempt.
- Institution Administrator owns the applicable institution-controlled recovery/cancellation decision.
- Recovery must not create duplicate current successful publication records for the same governed context.

# 13. State and Lifecycle Impact

A failed attempt remains `Failed` and immutable. Recovery creates a successor attempt rather than transitioning the failed record out of `Failed`. A cancelled pending attempt remains preserved as cancelled historical evidence. Successful later attempts follow normal H01 completion semantics.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

- `CredentialPublicationRecoveryAuthorized` — H01 publication fact that the Institution Administrator authorized governed recovery.
- Existing H01 publication-attempt and completion events remain applicable to the new recovery attempt according to its outcome.
- Source-domain credential events remain owned by their source domains.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H01 consumes authoritative source credential facts by reference and records only H01 publication/recovery facts. Recovery does not authorize writes to source-domain aggregates.

# 15. Business Authorization, Approval and Audit Obligations

The Institution Administrator is the canonical business authority for institution-controlled retry, manual recovery or cancellation decisions where such a decision is required. Audit evidence must preserve prior attempt identity/state, recovery authorization, cancellation decision where applicable, successor-attempt lineage, external references and final outcome.

# 16. Completion Criteria and Postconditions

Completion requires one governed outcome:

- a new authorized recovery attempt has completed according to H01 rules; or
- a pending attempt has been governedly cancelled with history preserved; or
- a governed manual-recovery decision has been recorded and followed through its applicable publication outcome.

No prior failed attempt is reopened or overwritten, source credential ownership is unchanged, and complete recovery lineage is preserved.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former conflict between Failed-state terminality and retry/recovery, together with the missing manual recovery/cancellation authority, was resolved by locked `L2-H01_Publication_Recovery_and_Republishing_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

The frozen Finance participation remains participation only; no generic H01 recovery ownership is assigned to Finance.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
