# Source Data Collection to Validated Regulatory Snapshot

# 1. Scenario Metadata

**Scenario ID:** L3-H-057  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** Regulatory approval, submission, correction, audit, and reporting-closure scenarios

# 2. Scenario Purpose

Collect authoritative institutional reporting facts from their canonical owners, evaluate reporting completeness and regulatory validity, preserve evidence references, and establish an immutable H08 Reporting Snapshot without changing ownership of the source facts.

# 3. Business Outcome

Evidence is collected by reference, validated, reconciled, and frozen as a traceable reporting snapshot or routed for correction

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An active reporting cycle requests authoritative institutional facts.

## 4.2 End Boundary

The frozen outcome requires a validated and frozen Reporting Snapshot or a correction route. H08 supports data collection, validation, snapshot generation, evidence mapping, and correction concepts, but its locked rules and aggregate definitions contradict one another on whether validation occurs before or against a Reporting Snapshot. The final validated-snapshot sequence therefore cannot be asserted authoritatively.

## 4.3 In Scope

- H08 collection of reporting information from canonical evidence-owning domains through approved reporting relationships.
- Completeness assessment and identification of missing reporting data.
- Structural, business, multi-domain, and regulatory validation.
- Reporting Snapshot generation, locking, and version preservation where the authoritative ordering can be established.
- Evidence references and reporting traceability without transfer of source ownership.
- Routing of deficient reporting information for governed correction.

## 4.4 Out of Scope

- Modification of source-domain operational records by H08.
- Creation of substitute source facts inside H08.
- Choosing one of H08's contradictory validation/snapshot sequences.
- Inventing a pre-snapshot validation object when the H08 Validation Result invariant requires a Reporting Snapshot.
- Technical data-staging, persistence, transport, or implementation mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| NEP, NAAC & AISHE Reporting / H | Owns Reporting Cycle reporting orchestration, Validation Result, Reporting Snapshot, Compliance Evidence Mapping, reporting version history, and correction routing. |
| VAR:Evidence-Owning Domain | Owns each authoritative institutional fact/evidence item referenced by H08 and remains the system of record for that fact. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Reporting Coordinator | Coordinates the H08 reporting cycle and collection progress where authorized. | Coordinates |
| Compliance Officer | Reviews reporting readiness and compliance outcomes where source-defined. | Reviews |
| Evidence Manager | Manages H08 evidence mappings and evidence completeness where authorized. | Performs evidence governance |

No actor is invented for the variable evidence-owning domain; its authoritative business facts are supplied under that domain's own upstream governance.

# 7. Preconditions and Entry Criteria

- A governed Reporting Cycle is ready for Data Collection under H08.
- The Reporting Framework and Report Template applicable to the cycle are published.
- Required source-domain reporting information can be referenced through approved reporting relationships.
- Evidence-owning domains retain ownership of their source facts.

# 8. Trigger

An active reporting cycle requests authoritative institutional facts

# 9. Main Business Flow

1. H08 begins the source-defined Data Collection phase for the Reporting Cycle.
2. H08 obtains the required reporting information from the applicable evidence-owning domains while preserving their canonical ownership.
3. H08 verifies reporting-data completeness and identifies any missing source information.
4. If required information is missing, H08 records the deficiency and routes the reporting work for governed correction or later recollection without creating substitute source facts.
5. For a complete reporting dataset, H08 applies the source-defined structural, business, multi-domain, and regulatory validation requirements.
6. H08 preserves the validation outcome and the source references required for reporting traceability.
7. Stop before asserting the creation and validation/locking order of the Reporting Snapshot because H08 simultaneously requires validation before snapshot generation and requires the Validation Result to reference a Reporting Snapshot.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Missing reporting information | Required source fact is absent | Record the missing-data outcome and route the reporting work back to source correction/collection without changing the source fact | Correction route established | NEP, NAAC & AISHE Reporting / evidence-owning domain within its own ownership |
| Validation failure | Collected reporting information fails a required validation | Record the failed validation and return the Reporting Cycle to the source-defined Data Collection path | Reporting work routed for correction | NEP, NAAC & AISHE Reporting |
| Validation passes | Collected reporting information satisfies all required validation stages | H08 would proceed toward Reporting Snapshot generation under W05, but the authoritative ordering conflict remains unresolved | Snapshot completion branch blocked pending upstream resolution | NEP, NAAC & AISHE Reporting |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Source information is unavailable or incomplete | Reporting readiness cannot be established | Preserve the deficiency and obtain corrected/complete facts from the canonical owner later | Evidence-owning domain / H08 |
| Validation fails | Snapshot-ready outcome cannot be claimed | Preserve the Validation Result and route the Reporting Cycle back to Data Collection as H08 defines | NEP, NAAC & AISHE Reporting |
| Validation/snapshot ordering is ambiguous | Frozen final snapshot outcome cannot be completed authoritatively | Escalate H08 rule/lifecycle conflict rather than selecting a local sequence | NEP, NAAC & AISHE Reporting |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Reporting data originates from canonical operational domains | Evidence-owning domains | H08 may consume but not re-own source facts | L2-H08 BR-003; O-002 |
| H08 must not modify operational business data | Evidence-owning domains / H08 boundary | Correction must occur under the source owner's governance | L2-H08 BR-004; O-006 |
| Validation evaluates structural, business, multi-domain, and compliance requirements | H08 | All required reporting validation categories apply | L2-H08 BR-006; H08-W04 |
| Validation failure prevents institutional approval | H08 | Failed information routes to correction rather than approval | L2-H08 BR-007 |
| BR-005 requires validation success before Reporting Snapshot generation | H08 | One H08 authority says no snapshot exists until validation succeeds | L2-H08 BR-005; H08-W05 |
| Validation Result must reference one Reporting Snapshot | H08 | Another H08 authority requires a snapshot to exist for validation | L2-H08 §6.9 invariant |
| Locked Reporting Snapshots are immutable and corrections create new versions | H08 | No correction may overwrite a locked historical snapshot | L2-H08 BR-008, BR-009 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Reporting Cycle | DataCollection | Complete collection and enter validation | Validation, or back to DataCollection on failed/deficient processing | H08 | L2-H08 §§6.6, 18.1 |
| Validation Result | Created | Execute H08 validation | Executing → Completed → Passed or Failed | H08 | L2-H08 §§6.9, 18.4 |
| Reporting Snapshot | Building | Source state model expects validation before lock | Validated → Locked, but H08 does not reconcile when this aggregate is created relative to Validation Result | H08 | L2-H08 §§6.8, 18.2 |
| Compliance Evidence Mapping | Created | H08 maps and verifies evidence after a locked snapshot under W06 | Mapped → Verified → Approved where applicable | H08 | L2-H08 §6.10; H08-W06 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H08 defines source-supported collection events `DataCollectionStarted`, `ReportingDataCollected`, `DataCollectionCompleted`, and `ReportingDataMissing`; validation events `ValidationStarted`, `ValidationCompleted`, `ValidationPassed`, `ValidationFailed`, and `ComplianceGapDetected`; and snapshot events `ReportingSnapshotGenerated`, `ReportingSnapshotLocked`, and `ReportingSnapshotPublished`. L3 does not impose an event order where the underlying H08 aggregate sequence is contradictory.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Evidence-owning domains supply authoritative reporting information through approved reporting relationships and retain ownership. H08 returns identified data deficiencies for correction rather than directly modifying source facts. Exact source-specific contracts remain owned upstream and are not invented by this scenario.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Reporting Coordinator, Compliance Officer, and Evidence Manager are source-listed H08 roles for reporting-cycle, readiness, and evidence responsibilities. H08's formal Actors section is absent, so no additional role or source-domain authority is synthesized.

## 15.2 Separation of Duties, Overrides and Exceptions

No validation exception may bypass mandatory validation before institutional approval. No local exception may override source-domain ownership or immutable snapshot rules.

## 15.3 Business Audit and Evidence Obligations

Preserve the Reporting Cycle reference, source-data/evidence references, missing-data findings, validation results, correction-routing history, snapshot version/history where validly created, and all evidence-mapping lineage.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The correction branch can be source-grounded when missing or failed information is routed back for correction. The frozen successful branch cannot be declared complete until H08 reconciles whether validation precedes snapshot generation or operates against an existing Reporting Snapshot.

## 16.2 Postconditions

Source-domain facts remain canonically owned and unchanged by H08. Reporting deficiencies and validation outcomes remain traceable. No unsupported validated-snapshot ordering is asserted.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H08 BR-005 and W04/W05 state that validation completes successfully **before Reporting Snapshot generation**.
- H08 §6.9 simultaneously requires every Validation Result to reference **one Reporting Snapshot**, implying that a Reporting Snapshot already exists during validation.
- H08's Reporting Snapshot state machine starts at `Building → Validated → Locked`, which also implies a snapshot exists before validation completes.
- H08-W06 performs evidence mapping only after a Reporting Snapshot is generated/locked, while the frozen outcome describes evidence being collected by reference and validated as part of the path to the frozen reporting snapshot.

These conflicts materially affect the frozen successful outcome, aggregate lifecycle ordering, and validation relationship.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H08 architecture must define one canonical sequence for reporting-data collection, Reporting Snapshot creation, Validation Result evaluation, evidence mapping, and snapshot locking.
