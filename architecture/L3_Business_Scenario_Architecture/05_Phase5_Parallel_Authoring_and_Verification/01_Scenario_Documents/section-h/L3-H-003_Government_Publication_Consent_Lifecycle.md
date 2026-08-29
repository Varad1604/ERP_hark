# Government Publication Consent Lifecycle

# 1. Scenario Metadata

**Scenario ID:** L3-H-003  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** A, C, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** L2-H01 — APAAR & DigiLocker Workflows; L2-C02 — Student Lifecycle & Activation Workflows; Section A authorization/audit architecture

# 2. Scenario Purpose

Govern the learner’s H01 government-publication Consent so its effective state controls future eligible publication while prior Consent evidence remains immutable.

# 3. Business Outcome

The effective consent state governs future publication while historical consent evidence remains immutable

# 4. Scenario Boundary and Scope

The scenario starts when the frozen trigger occurs and ends when the frozen business outcome is reached or, for a source-defined unsuccessful branch, when the controlled H01/H02 outcome is recorded. In scope are only the source-owned facts, H-owned integration/communication lifecycle, canonical events, business contracts and evidence materially required for this outcome. Source-domain credential, Student, Institution, identity and originating transaction ownership remain outside H.

# 5. Ownership and Participating Domains

- **A:** retains canonical authorization and audit capability ownership.
- **C:** retains the canonical Student identity/lifecycle referenced by Consent.
- **H:** owns the H01 Consent aggregate, its states and Consent events.

# 6. Business Actors, Roles and Responsibilities

**Student** is source-supported to grant, update or withdraw H01 Consent. Expiry is a policy-governed lifecycle consequence, not a new human decision.

# 7. Preconditions and Entry Criteria

- Canonical Student reference exists.
- Consent scope and requested effective period are valid.
- The action is authorized.
- Existing Consent state/history is available.

# 8. Trigger

A learner grants, changes, withdraws, or allows publication consent to expire

# 9. Main Business Flow

1. Resolve the canonical Student reference and applicable consent scope.
2. Validate authority, scope, dates and current Consent state.
3. Apply grant, update, withdrawal or expiry under locked H01 rules.
4. Preserve the invariant of only one active Consent for the same scope.
5. Publish `ConsentGranted`, `ConsentUpdated`, `ConsentWithdrawn`, or `ConsentExpired` as applicable.
6. Preserve immutable consent history.

# 10. Alternate Business Flows

- Pending Consent may become `Granted`.
- Granted Consent may be updated, withdrawn or expire according to source-defined rules.
- Invalid scope/dates leave the current authoritative Consent unchanged.

# 11. Failure, Interruption and Business Recovery

Invalid Student reference, scope, dates or conflicting active Consent prevent completion until corrected through source-defined business governance; prior Consent history is preserved.

# 12. Business Rules and Decision Constraints

- Allowed Consent statuses are `Pending`, `Granted`, `Withdrawn`, `Expired`.
- Only one active Consent exists for a scope.
- Withdrawn/expired Consent prevents future publication where Consent is required.
- Historical Consent evidence is immutable/auditable.

# 13. State and Lifecycle Impact

Consent follows the locked H01 lifecycle from `Pending` to `Granted`, then to `Withdrawn` or `Expired` as applicable. Student lifecycle remains unchanged.

# 14. Business Events and Cross-Domain Collaboration

H01 Consent is canonical publisher of `ConsentGranted`, `ConsentUpdated`, `ConsentWithdrawn`, and `ConsentExpired`; H01 Publication consumes the effective Consent fact where policy requires it.

# 15. Business Authorization, Approval and Audit Obligations

The Student is the source-supported actor for grant/update/withdraw. No extra approval or separation-of-duty requirement is invented. Audit evidence preserves Student reference, scope, prior/new state, effective dates and actor where applicable.

# 16. Completion Criteria and Postconditions

Completion requires a valid H01 Consent action, resulting source-defined state/event, immutable history and application of the effective Consent to future publication eligibility.

# 17. Open Issues, Assumptions and Deferred Questions

- H01’s dependency table labels Student Lifecycle with a stale section letter; the current dependency map and locked L1/C ownership control.

**Architecture assessment:** `PASS WITH ISSUES`

**Assumptions:** None.

**Deferred questions:** No additional scenario-specific questions beyond the issues recorded above.
