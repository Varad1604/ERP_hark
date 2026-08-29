# Source Data Collection to Validated Regulatory Snapshot

# 1. Scenario Metadata

**Scenario ID:** L3-H-057  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Collect authoritative institutional facts for an H08 reporting cycle, validate and reconcile them without changing source ownership, and preserve the accepted reporting facts as an immutable regulatory snapshot.

# 3. Business Outcome

Evidence is collected by reference, validated, reconciled, and frozen as a traceable reporting snapshot or routed for correction

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an active reporting cycle requests authoritative institutional facts.

## 4.2 End Boundary

The scenario ends when H08 has either produced the governed validated/locked Reporting Snapshot required for subsequent approval, or returned the reporting work to source-data collection/correction because validation cannot succeed.

## 4.3 In Scope

- Request and collect authoritative facts from the applicable `VAR:Evidence-Owning Domain` through governed reporting contracts.
- Verify collection completeness and identify missing reporting data.
- Execute the H08 validation stages applicable to the reporting dataset.
- Reconcile reporting inconsistencies without changing source-owned facts.
- Generate, version, validate, and lock the H08 Reporting Snapshot when validation succeeds.
- Route failed validation back to governed data collection/correction.

## 4.4 Out of Scope

- Modifying operational facts in any evidence-owning domain.
- Approving a Submission Package.
- Submitting information to an external regulatory authority.
- Replacing `VAR:Evidence-Owning Domain` with an invented concrete domain.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| NEP, NAAC & AISHE Reporting / H08 | Owns reporting collection orchestration, Validation Result, Reporting Snapshot, snapshot versioning, reporting validation outcome, and reporting history. |
| VAR:Evidence-Owning Domain | Owns the authoritative institutional facts/evidence referenced by the reporting cycle and any source-side correction required to make those facts authoritative. |

# 6. Business Actors, Roles and Responsibilities

Locked H08 does not define canonical business actors for data collection and validation. H08 owns the reporting process and outcome; each applicable evidence-owning domain retains responsibility for the correctness of its own source facts.

# 7. Preconditions and Entry Criteria

- The applicable Reporting Cycle is ready for `DataCollection` under H08 governance.
- A published Reporting Framework and Report Template govern the cycle.
- Applicable source reporting contracts are available.
- H08 does not own or modify the operational facts being collected.

# 8. Trigger

An active reporting cycle requests authoritative institutional facts

# 9. Main Business Flow

1. H08 begins the Reporting Cycle `DataCollection` activity.
2. H08 obtains the required reporting facts by reference from the applicable authoritative evidence-owning domains.
3. H08 verifies completeness and records any missing reporting data.
4. When collection is complete, H08 evaluates structural, business, multi-domain, and regulatory validation requirements.
5. H08 records the Validation Result without modifying the authoritative source facts.
6. If validation succeeds, H08 generates the Reporting Snapshot from the validated reporting dataset, assigns the governed version, and locks the snapshot against in-place change.
7. The locked snapshot becomes the traceable reporting record used by the subsequent approval/submission lifecycle.

# 10. Alternate Business Flows

- If required reporting data is missing, H08 records the missing-data condition and collection remains incomplete.
- If validation fails, the Reporting Cycle returns to `DataCollection` for governed correction/recollection.
- A correction to source-owned information is performed by its canonical owner; H08 subsequently recollects the corrected authoritative fact.
- Validation may complete with warnings where H08 permits that outcome, but only successful validation permits snapshot generation under BR-005.

# 11. Failure, Interruption and Business Recovery

Missing, inconsistent, or invalid reporting information prevents successful snapshot completion. Recovery is achieved by obtaining corrected authoritative information from its owning domain and repeating the governed H08 collection/validation path. Existing locked snapshots are never overwritten.

# 12. Business Rules and Decision Constraints

- BR-003 requires reporting data to originate from canonical operational domains.
- BR-004 prohibits H08 from modifying operational business data.
- BR-005 requires successful validation before Reporting Snapshot generation.
- BR-006 requires structural, business, multi-domain, and compliance validation.
- BR-008 requires Reporting Snapshots to remain immutable after locking.
- BR-009 requires corrections to create new Reporting Snapshot versions.
- Evidence references must preserve authoritative source ownership.

# 13. State and Lifecycle Impact

The Reporting Cycle progresses through `DataCollection → Validation → SnapshotCreated` when the path succeeds; failed validation returns the cycle to `DataCollection`. The Reporting Snapshot lifecycle is `Building → Validated → Locked → Submitted → Archived`. Validation Result progresses through `Created → Executing → Completed → Passed` or `Failed`.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Relevant H08 events include `DataCollectionStarted`, `ReportingDataCollected`, `DataCollectionCompleted`, `ReportingDataMissing`, `ValidationStarted`, `ValidationCompleted`, `ValidationPassed`, `ValidationFailed`, `ComplianceGapDetected`, `ReportingSnapshotGenerated`, and `ReportingSnapshotLocked`.

No event is invented for a variable evidence-owning domain; its authoritative facts/events remain defined by the actual owning source when resolved for a concrete reporting cycle.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H08 consumes authoritative source information through the applicable approved reporting contract. The hand-off supplies source-owned facts/evidence references for reporting use; it does not transfer ownership or authorize H08 to alter the source record.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H08 governs collection and validation of reporting data. The locked source does not identify a canonical business actor for these activities; no local actor or approval authority is invented.

## 15.2 Separation of Duties, Overrides and Exceptions

H08 validation cannot override or rewrite a source-domain fact. When a source fact requires correction, the owning domain remains responsible for that correction before H08 recollects it.

## 15.3 Business Audit and Evidence Obligations

Preserve source references, collection completeness results, validation findings, correction/recollection lineage, snapshot version lineage, snapshot lock evidence, and evidence mappings needed to reconstruct the reporting outcome.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

Either a validated and locked H08 Reporting Snapshot has been created from authoritative source facts, or failed validation has routed the reporting work back to governed correction/data collection without changing source ownership.

## 16.2 Postconditions

The current Reporting Snapshot and Validation Result remain H08-owned. Operational facts remain owned by the applicable `VAR:Evidence-Owning Domain`; any future correction creates new reporting history rather than rewriting the locked snapshot.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

**Non-blocking actor-definition gap:** the H08 canonical crosswalk records that an Actors section is not present. This scenario does not depend on a discretionary approval decision, so the source-grounded owner/process boundary can still be authored, but named business responsibility remains incomplete for later approval.

**Non-blocking lifecycle terminology issue:** frozen H057 uses “active reporting cycle,” and H08-W03 also uses “Reporting Cycle active” as a precondition, while the canonical Reporting Cycle state machine contains no `Active` state. This Draft anchors the actual lifecycle work to the defined `DataCollection` state and does not create an `Active` state.

**Architecture impact:** business actor naming and the adjective “active” require upstream clarification before the scenario can become source-complete, but ownership, collection, validation, correction, and snapshot outcomes are defined.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions. `VAR:Evidence-Owning Domain` is preserved exactly as frozen.

## 17.3 Deferred Questions

Upstream H08 should normalize the business-actor model and clarify whether “active reporting cycle” is a non-state business condition or a missing canonical lifecycle state.
