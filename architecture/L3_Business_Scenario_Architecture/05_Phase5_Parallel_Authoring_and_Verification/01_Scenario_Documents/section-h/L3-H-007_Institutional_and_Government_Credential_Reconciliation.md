# Institutional and Government Credential Reconciliation

# 1. Scenario Metadata

**Scenario ID:** L3-H-007  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** C, D, F, H, I  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** L2-H01 — APAAR & DigiLocker Workflows; applicable locked source-domain credential architecture; L2-C05; L2-I13; Section A audit architecture

# 2. Scenario Purpose

Compare authoritative institutional publication references with government records through H01 Synchronization so discrepancies are classified, resolved through existing governed source paths where possible, or routed to governed follow-up without rewriting either history.

# 3. Business Outcome

Discrepancies are classified and resolved or routed to governed follow-up while both systems’ histories are preserved

# 4. Scenario Boundary and Scope

The scenario starts when the frozen trigger occurs and ends when the frozen business outcome is reached or, for a source-defined unsuccessful branch, when the controlled H01/H02 outcome is recorded. In scope are only the source-owned facts, H-owned integration/communication lifecycle, canonical events, business contracts and evidence materially required for this outcome. Source-domain credential, Student, Institution, identity and originating transaction ownership remain outside H.

# 5. Ownership and Participating Domains

- **C/D/F/I:** retain source credential/student/academic/finance/examination facts.
- **H:** owns Synchronization/reconciliation execution, discrepancy classification, reconciliation evidence and H01 follow-up routing; it does not become source credential owner.

# 6. Business Actors, Roles and Responsibilities

H01 does not define a canonical human actor for generic reconciliation. Scheduled, audit, recovery or authorized administrative initiation remains under its source governance.

# 7. Preconditions and Entry Criteria

- Relevant H01 Publication/government references exist.
- Source credential facts remain available from their canonical owners.
- Reconciliation request is authorized.
- H01 Synchronization can enter a source-defined execution.

# 8. Trigger

A scheduled review, audit, or recovery request requires comparison with government records

# 9. Main Business Flow

1. Establish the H01 Synchronization scope.
2. Compare authoritative institutional/H01 publication references with government records.
3. Classify discrepancies using H01-supported outcomes.
4. Record reconciliation status/report and preserve both histories.
5. Resolve only through an existing authorized source/H01 path; otherwise route the discrepancy to governed follow-up.
6. Complete H01 Synchronization as `Completed` or `Failed` under its locked lifecycle.

# 10. Alternate Business Flows

- Fully synchronized closes without source changes.
- Missing publication, government mismatch, duplicate publication or revoked-credential discrepancy is classified and routed to the applicable governed path.
- Manual-review-required remains governed follow-up rather than local correction.

# 11. Failure, Interruption and Business Recovery

If comparison cannot complete, record the source-defined failed Synchronization. If source truth is disputed, H01 does not decide it; the canonical source owner must resolve the fact.

# 12. Business Rules and Decision Constraints

- H01 Synchronization owns comparison/reconciliation status, not credential truth.
- Supported outcomes include fully synchronized, missing publication, government mismatch, duplicate publication, revoked credential detected and manual review required.
- Histories remain preserved.

# 13. State and Lifecycle Impact

H01 Synchronization progresses from its source-defined start through comparison to `Completed` or `Failed`. Source credentials and existing Publications are not silently changed by reconciliation.

# 14. Business Events and Cross-Domain Collaboration

H01 publishes `ExternalProviderSynchronizationStarted`, `ApaarSynchronizationCompleted`, or `SynchronizationFailed`. The only named success-completion event is APAAR-specific despite the generic scenario; no generic event is invented.

# 15. Business Authorization, Approval and Audit Obligations

No universal H01 reconciliation human actor is defined. Source corrections/revocations remain with canonical owners. Evidence preserves comparison scope, discrepancy classifications, report, references, follow-up and history.

# 16. Completion Criteria and Postconditions

Completion requires completed comparison, supported discrepancy classification, authoritative resolution where already defined or governed follow-up, preserved histories and a source-defined Synchronization outcome.

# 17. Open Issues, Assumptions and Deferred Questions

- Only `ApaarSynchronizationCompleted` is defined as success event for a generic government reconciliation scope.
- No universal cross-domain discrepancy-resolution contract exists.
- H01 stale dependency labels and frozen Finance participation remain unresolved metadata/participation issues.

**Architecture assessment:** `PASS WITH ISSUES`

**Assumptions:** None.

**Deferred questions:** No additional scenario-specific questions beyond the issues recorded above.
