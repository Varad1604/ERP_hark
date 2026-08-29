# Published Credential to Governed Revocation

# 1. Scenario Metadata

**Scenario ID:** L3-H-004  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** C, D, F, H, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** L2-H01 — APAAR & DigiLocker Workflows; applicable locked source-domain credential architecture; L2-C05; L2-I13; Section A audit architecture

# 2. Scenario Purpose

Apply an authoritative source revocation requirement to a previously published H01 government publication without allowing H01 to revoke, erase or rewrite the source credential.

# 3. Business Outcome

The external publication is revoked where supported and the terminal revocation outcome is recorded without deleting its history

# 4. Scenario Boundary and Scope

The scenario starts when the frozen trigger occurs and ends when the frozen business outcome is reached or, for a source-defined unsuccessful branch, when the controlled H01/H02 outcome is recorded. In scope are only the source-owned facts, H-owned integration/communication lifecycle, canonical events, business contracts and evidence materially required for this outcome. Source-domain credential, Student, Institution, identity and originating transaction ownership remain outside H.

# 5. Ownership and Participating Domains

- **C/D/F/I:** retain source facts and any source-domain decision that makes revocation necessary; frozen Finance participation is preserved without invented generic authority.
- **H:** owns only the H01 government-publication revocation lifecycle and events.

# 6. Business Actors, Roles and Responsibilities

**Institution Administrator** is source-supported to request H01 credential revocation after an authoritative source basis exists. The source-domain role that makes the underlying credential decision varies by credential owner and is not invented.

# 7. Preconditions and Entry Criteria

- Authoritative source revocation basis exists.
- Corresponding H01 Publication is `Published`.
- H01 revocation authorization is valid.
- Connector revocation capability is known.
- Original publication/government reference is available.

# 8. Trigger

An authoritative source establishes that a government-published credential must be revoked

# 9. Main Business Flow

1. Consume the authoritative source revocation requirement.
2. Resolve the corresponding `Published` H01 Publication.
3. Validate Institution Administrator authority and Connector revocation support.
4. Record the H01 revocation request and publish `CredentialRevocationRequested`.
5. Obtain the government revocation outcome.
6. On successful supported revocation, transition the H01 Publication to `Revoked`.
7. Publish `CredentialRevoked` and preserve original publication plus revocation evidence.

# 10. Alternate Business Flows

- Unsupported Connector revocation does not become `Revoked`; the unresolved condition remains governed.
- A non-Published H01 record is ineligible for this revocation flow.
- Government rejection preserves the existing Publication state and unsuccessful revocation evidence.

# 11. Failure, Interruption and Business Recovery

Unclear source authority, missing prior government reference, or unsupported/rejected government revocation prevents a false `Revoked` outcome. History remains preserved for governed follow-up.

# 12. Business Rules and Decision Constraints

- H01 never modifies the source credential.
- H01 revocation requires a previously `Published` Publication.
- Connector revocation capability is mandatory for successful external revocation.
- Completed Publication history remains preserved.

# 13. State and Lifecycle Impact

H01 Publication moves from `Published` to `Revoked` only on successful supported government revocation. Source credential lifecycle is unchanged by H01.

# 14. Business Events and Cross-Domain Collaboration

H01 publishes `CredentialRevocationRequested` and `CredentialRevoked`. The generic upstream source event/publisher for the underlying credential revocation basis is not universally mapped, so none is invented.

# 15. Business Authorization, Approval and Audit Obligations

Institution Administrator authority is limited to the H01 revocation request; the source-domain revocation decision remains upstream. Evidence preserves source basis, original Publication/government reference, H01 lifecycle change and revocation outcome.

# 16. Completion Criteria and Postconditions

Completion requires an authoritative source basis, eligible Published H01 Publication, supported/authorized government revocation, `Revoked` state/event on success, and preserved history.

# 17. Open Issues, Assumptions and Deferred Questions

- Generic source revocation event/publisher across C/D/F/I is undefined.
- Frozen Finance participation has no universal H01 revocation responsibility.
- H01 dependency labels contain stale section mappings.

**Architecture assessment:** `PASS WITH ISSUES`

**Assumptions:** None.

**Deferred questions:** No additional scenario-specific questions beyond the issues recorded above.
