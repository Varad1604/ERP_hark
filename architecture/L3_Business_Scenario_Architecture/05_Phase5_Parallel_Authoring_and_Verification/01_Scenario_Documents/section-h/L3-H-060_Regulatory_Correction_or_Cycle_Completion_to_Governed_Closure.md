# Regulatory Correction or Cycle Completion to Governed Closure

# 1. Scenario Metadata

**Scenario ID:** L3-H-060  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** Historical reporting, audit, compliance evidence, and future reporting-cycle scenarios

# 2. Scenario Purpose

Govern either correction/resubmission of an H08 regulatory reporting outcome or formal closure and archival of a completed Reporting Cycle while preserving immutable snapshot/submission lineage and canonical ownership of source-domain corrections.

# 3. Business Outcome

A corrected package is resubmitted where required and the cycle is formally closed and archived with full lineage

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A submitted package requires correction, or all cycle obligations are complete.

## 4.2 End Boundary

The completion branch can end when the accepted/completed Reporting Cycle is closed and archived with immutable history. The correction branch cannot be authored to a conclusive corrected resubmission because H08-W11 orders corrected snapshot generation before repeated validation, while H08 BR-005 and W04/W05 require successful validation before Reporting Snapshot generation.

## 4.3 In Scope

- H08 correction initiation after validation, institutional, or regulatory rejection.
- Collection/reference of corrected source information from its canonical evidence-owning domain.
- New snapshot/version and resubmission governance only to the extent source ordering is unambiguous.
- Formal Reporting Cycle closure and archival after obligations are complete.
- Preservation of reporting, snapshot, submission, acknowledgement, and compliance history.

## 4.4 Out of Scope

- Direct correction of operational source facts by H08.
- Overwriting a locked Reporting Snapshot or historical Submission Record.
- Choosing between contradictory H08 correction/validation/snapshot rules.
- Technical retry, rollback, transport, or storage mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| NEP, NAAC & AISHE Reporting / H | Owns Reporting Cycle correction/closure governance, Reporting Snapshot versions, Submission Package/Record history, reporting archival, and H08 lineage. |
| VAR:Evidence-Owning Domain | Owns any authoritative source fact that must be corrected or re-issued before H08 can consume the corrected value/reference. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Reporting Coordinator | Coordinates H08 correction or Reporting Cycle completion where authorized. | Coordinates |
| Regulatory Submission Officer | Performs a later corrected submission where the package becomes validly ready and authorized. | Submits |
| Auditor | Reviews preserved reporting and submission history. | Read-only review |

No source-domain correction actor is invented for `VAR:Evidence-Owning Domain`.

# 7. Preconditions and Entry Criteria

For correction:
- An existing Reporting Cycle and prior reporting/submission history exist.
- A validation failure, approval rejection, regulatory rejection, resubmission requirement, or authorized institutional correction identifies the issue.
- Source-domain facts requiring correction remain owned by their canonical domains.

For closure:
- Reporting obligations are complete.
- Submission history is finalized as required by H08.
- The Reporting Cycle is eligible for source-defined closure/archival.

# 8. Trigger

A submitted package requires correction, or all cycle obligations are complete

# 9. Main Business Flow

1. H08 determines whether the trigger concerns correction/resubmission or completed-cycle closure.
2. For correction, H08 records `CorrectionInitiated` and identifies the reporting issue without altering prior immutable snapshots or Submission Records.
3. Where the issue originates in a source-domain fact, the evidence-owning domain corrects or re-establishes its authoritative fact under its own governance; H08 later consumes the corrected reference/value.
4. H08 assembles the corrected reporting information required for a successor reporting version.
5. Stop the correction branch before deciding whether the corrected Reporting Snapshot is generated before or after repeated validation because locked H08 sources contradict one another on that sequence.
6. For the completion branch, H08 verifies that reporting obligations and submission history are complete.
7. H08 closes the Reporting Cycle in the source-defined `Closed` state and preserves it as immutable.
8. H08 archives the Reporting Snapshot and submission/compliance history as defined upstream.
9. H08 moves the Reporting Cycle from `Closed` to `Archived` and publishes the source-defined closure/archival outcomes.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Regulatory rejection | Authority rejects prior submission | Enter H08 correction governance while preserving the prior Submission Record | Correction path initiated | NEP, NAAC & AISHE Reporting |
| Institutional correction request | Institution identifies an authorized reporting correction | Enter correction governance; source facts remain corrected by their canonical owners | Correction path initiated | NEP, NAAC & AISHE Reporting / evidence-owning domain |
| Obligations complete without correction | Reporting lifecycle has reached its accepted/completed position | Close and archive according to H08-W12 | Governed closure and archival completed | NEP, NAAC & AISHE Reporting |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Corrected source information is not authoritative or complete | Corrected reporting cannot progress | Obtain a valid correction from the canonical evidence-owning domain | Evidence-owning domain / H08 |
| Corrected snapshot/validation ordering remains unresolved | Resubmission cannot be completed authoritatively | Escalate H08 rule conflict; preserve existing history | NEP, NAAC & AISHE Reporting |
| Reporting obligations remain incomplete | Cycle cannot close | Keep the cycle in its source-supported current position until obligations complete | NEP, NAAC & AISHE Reporting |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H08 does not modify operational business data | Evidence-owning domains / H08 boundary | Source corrections occur under source ownership | L2-H08 BR-004; O-002 |
| Corrections create new Reporting Snapshot versions | H08 | Prior locked snapshots are never overwritten | L2-H08 BR-009; O-003 |
| Validation success is required before Reporting Snapshot generation | H08 | BR-005 conflicts with the W11 corrected-snapshot sequence | L2-H08 BR-005; H08-W04/W05 |
| Every resubmission creates a new Submission Record | H08 | Prior submission history remains immutable | L2-H08 O-007; §6.12 |
| Closed Reporting Cycles are immutable | H08 | Closure prevents rewriting completed cycle history | L2-H08 BR-020 |
| Historical submissions are never deleted and history remains reconstructable | H08 | Full lineage must survive correction and closure | L2-H08 BR-021, BR-022 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Reporting Cycle | Rejected / RejectedByAuthority | Correction routing | DataCollection under H08 lifecycle | H08 | L2-H08 §§6.6, 18.1 |
| Reporting Snapshot | Locked historical version | Correction requires successor version | Historical snapshot unchanged; successor ordering unresolved | H08 | L2-H08 §6.8; BR-008/009 |
| Submission Record | Prior submitted/acknowledged record | Correction/resubmission must preserve prior history | Prior record unchanged; new record required for valid resubmission | H08 | L2-H08 §6.12; O-007 |
| Reporting Cycle | Accepted | Formal completion | Closed → Archived | H08 | L2-H08 §§6.6, 18.1; H08-W12 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

For correction, H08 defines `CorrectionInitiated`, `SnapshotVersionCreated`, `ResubmissionPrepared`, and `ReportResubmitted`. For closure/archival, H08 defines `ReportingCycleClosed`, `ReportingArchived`, `ComplianceHistoryArchived`, and `ReportingLifecycleCompleted`. L3 preserves these names but does not assert the disputed corrected-snapshot ordering.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Where correction requires a changed source fact, H08 communicates the reporting deficiency through the approved source-domain reporting relationship and later consumes the authoritative corrected fact/reference. H08 never takes ownership of the source correction. Closure is H08-owned and does not transfer reporting-history ownership to consumers.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Reporting Coordinator is source-listed for Reporting Cycle management; Regulatory Submission Officer is source-listed for submission. The evidence-owning domain retains its own authority for correcting source facts. No additional cross-domain actor is invented.

## 15.2 Separation of Duties, Overrides and Exceptions

Correction cannot bypass validation, approval, evidence, or immutable-history requirements. Closure cannot occur while reporting obligations remain incomplete.

## 15.3 Business Audit and Evidence Obligations

Preserve correction reason, affected snapshot/package/submission references, source correction references, validation history, successor version lineage, resubmission history where validly completed, cycle closure evidence, archival evidence, acknowledgement history, and the complete immutable chain of prior reporting outcomes.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The closure branch completes when obligations are complete and the Reporting Cycle is closed and archived with reporting/submission history retained.
- The correction/resubmission branch cannot be declared complete until H08 resolves the corrected-snapshot versus validation ordering conflict.

## 16.2 Postconditions

Completed reporting history is immutable and reconstructable. Source-domain corrections remain owned upstream. No corrected resubmission is fabricated from contradictory H08 sequencing.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H08-W11 defines correction as `Collect Corrected Data → Generate New Snapshot → Repeat Validation → Generate New Submission Package → Resubmit`.
- H08 BR-005 and W04/W05 instead require validation to complete successfully **before Reporting Snapshot generation**.
- H08 §6.9 also requires Validation Result to reference a Reporting Snapshot, creating the broader validation/snapshot conflict already exposed by H057.
- Because the frozen outcome requires a corrected package to be resubmitted **where required**, this contradiction affects a material branch of this scenario rather than a peripheral detail.

The closure/archival branch is source-supported, but the full frozen scenario remains materially incomplete.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H08 architecture must reconcile correction, snapshot-generation, validation, package-generation, and resubmission sequencing before the correction branch can be completed.
