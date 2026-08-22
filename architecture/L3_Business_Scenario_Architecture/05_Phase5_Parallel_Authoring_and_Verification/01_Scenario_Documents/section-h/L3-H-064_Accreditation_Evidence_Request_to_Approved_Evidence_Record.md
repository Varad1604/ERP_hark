# Accreditation Evidence Request to Approved Evidence Record

# 1. Scenario Metadata

**Scenario ID:** L3-H-064  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** Readiness assessment, audit, external assessment, compliance review, and SSR scenarios

# 2. Scenario Purpose

Govern an H09 accreditation evidence record from source reference through validation, approval or rejection, version preservation, and retention while the underlying source-domain fact remains owned by its canonical business domain.

# 3. Business Outcome

A source-owned reference is collected, validated, approved or rejected, versioned, and retained without copying canonical business ownership into H09

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An active criterion or compliance requirement needs source-domain evidence.

## 4.2 End Boundary

The scenario ends when H09 has a traceable evidence decision: an approved evidence record progresses through the governed evidence lifecycle and is versioned/retained, or a rejection/correction outcome is recorded without replacing the source-domain system of record.

## 4.3 In Scope

- Mapping the evidence request to an accreditation criterion or compliance requirement.
- Linking the authoritative source-domain record or approved institutional document.
- Evidence completeness, authenticity, traceability, metadata, and version review.
- H09 evidence approval or rejection outcome.
- Version preservation, locking of approved evidence, and retention history.

## 4.4 Out of Scope

- Duplicating operational source-domain transactions into H09.
- Changing source-domain business records.
- Treating H09's evidence item as the owner of the underlying operational fact.
- Readiness or assessment decisions that consume approved evidence later.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance / H | Owns the accreditation Evidence Repository Item, its metadata/reference, validation/approval status, version history, approved evidence snapshot, locking, and accreditation retention lifecycle. |
| VAR:Evidence-Owning Domain | Owns the authoritative operational fact or source record referenced by the H09 evidence item and supplies corrected/current evidence under its own governance. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC Reviewer | Participates in evidence validation and review. | Reviews |
| Accreditation Coordinator | Coordinates evidence collection and accreditation mapping. | Coordinates |
| Department Representative | Supplies or corrects source-linked evidence where applicable. | Performs |
| Evidence Reviewer | Reviews evidence completeness, authenticity, metadata, and readiness under H09 evidence governance. | Reviews |
| Compliance Officer | Participates where the evidence supports a compliance requirement. | Reviews |

H09's evidence workflows define the approval/rejection action but do not bind the final decision to one singular actor from this set.

# 7. Preconditions and Entry Criteria

- An active criterion or Compliance Requirement requires evidence.
- The originating source record or approved institutional document can be referenced.
- H09 can create an accreditation evidence record without changing source-domain ownership.

# 8. Trigger

An active criterion or compliance requirement needs source-domain evidence

# 9. Main Business Flow

1. H09 establishes the Evidence Repository Item for the required criterion or compliance requirement.
2. H09 links the evidence item to its originating source-domain record or approved institutional document.
3. The participating source domain supplies the current authoritative reference or supporting evidence under its own governance.
4. H09 validates source ownership and traceability.
5. H09 checks completeness, authenticity, metadata, approval status, and version currency.
6. If gaps are found, H09 records the gap and requests corrected or updated evidence from the authoritative source participant.
7. When evidence satisfies the H09 validation requirements, the evidence workflow records the approval decision and preserves the approval history.
8. H09 versions the evidence record, publishes it where required, locks approved evidence, and preserves its retention/history.
9. If evidence is rejected, H09 records `Evidence Rejected`, preserves the decision history, and does not substitute a locally invented source fact.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Evidence rejected | Evidence fails validation or approval | Record rejection and request correction/update from the authoritative source participant | Rejected decision retained | Accreditation & Compliance / evidence owner |
| Approved evidence changes later | A source-linked fact or supporting evidence changes after H09 approval | Create a new H09 evidence version rather than modifying approved history | Successor evidence version governed | Accreditation & Compliance |
| Evidence becomes archival | Evidence has completed active use under retention governance | Preserve version lineage and move through the source-defined archival path | Historical evidence remains traceable | Accreditation & Compliance |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Source reference cannot be verified | Evidence cannot be accepted as official accreditation evidence | Resolve the source reference with its canonical owner | Evidence-owning domain / Accreditation & Compliance |
| Mandatory evidence information is incomplete | Evidence cannot reach approved readiness | Obtain corrected/complete evidence and repeat validation | Evidence-owning domain / Accreditation & Compliance |
| Approved evidence requires change | Approved evidence is read-only | Create a new evidence version | Accreditation & Compliance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every evidence item references its originating record or approved institutional document | H09 / source owner boundary | Traceability is mandatory | L2-H09 BR-H09-009 |
| Reference-based linking avoids evidence duplication | H09 | H09 must not copy source transactional ownership | L2-H09 BR-H09-010, BR-H09-036/037 |
| Evidence versions are preserved permanently | H09 | Approval history and later versions remain traceable | L2-H09 BR-H09-011 |
| Approved evidence becomes read-only | H09 | Approved evidence cannot be altered in place | L2-H09 BR-H09-012 |
| Post-approval changes require a new version | H09 | Corrections create successor evidence history | L2-H09 BR-H09-013 |
| Deleted evidence remains recoverable according to retention policy | H09 | Retention obligations survive normal deletion handling | L2-H09 BR-H09-014 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Evidence Repository Item | Created | Establish and attach/source-link the evidence item | Uploaded | H09 | L2-H09 H09-A04 |
| Evidence Repository Item | Uploaded | Validate and review evidence | Validated → Reviewed | H09 | L2-H09 H09-A04 |
| Evidence Repository Item | Reviewed | Record approval outcome | Approved on successful branch | H09 | L2-H09 H09-A04; H09-05 |
| Evidence Repository Item | Approved | Publish and preserve approved evidence | Published → Locked | H09 | L2-H09 H09-A04 |

H09 §7 uses the labels `Validation`, `Review`, and `Approval`, while H09-A04 uses `Validated`, `Reviewed`, and `Approved`. H09 also defines an `Evidence Rejected` event but no explicit `Rejected` lifecycle state.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Evidence Linked | Evidence Repository aggregate / H09 | Readiness, review and audit consumers | Records source linkage | L2-H09 §11 |
| Evidence Validation Requested | Evidence Repository aggregate / H09 | Evidence governance participants | Records validation entry | L2-H09 §11 |
| Evidence Approved | Evidence Repository aggregate / H09 | Readiness, audit and assessment consumers | Records approved evidence | L2-H09 §11 |
| Evidence Rejected | Evidence Repository aggregate / H09 | Evidence governance participants | Records rejection outcome | L2-H09 §11 |
| Evidence Published | Evidence Repository aggregate / H09 | Readiness, audit and assessment consumers | Records governed publication | L2-H09 §11 |
| Evidence Archived | Evidence Repository aggregate / H09 | Historical governance consumers | Records archival | L2-H09 §11 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

The source-owning domain supplies or confirms the authoritative record/reference. H09 owns only the accreditation evidence record, metadata, approval/version lifecycle, and governed snapshot; the source-domain business fact remains externally owned.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H09-04 and H09-05 define the business actor set for evidence governance and explicitly define approval/rejection outcomes. The source does not identify one singular final evidence approver among those actors, so this Draft preserves the collective H09 evidence-governance action without assigning unsupported individual authority.

## 15.2 Separation of Duties, Overrides and Exceptions

The evidence-owning domain may supply/correct the authoritative source fact but does not thereby acquire ownership of H09's accreditation evidence approval lifecycle. H09 may not alter the source fact to achieve an approval outcome.

## 15.3 Business Audit and Evidence Obligations

Preserve source reference, criterion/requirement mapping, metadata, validation findings, rejection/correction history, approval history, version lineage, publication/locking history, and retention status.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The evidence record remains traceable to its authoritative source.
- Validation and approval/rejection history is preserved.
- Approved evidence is versioned, read-only, and retained under H09 governance.
- Rejected evidence does not cause H09 to substitute a source-domain fact.

## 16.2 Postconditions

Approved evidence is available for later readiness/audit/assessment use, or the rejection/correction outcome remains governed and traceable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H09-A04 uses `Validated`, `Reviewed`, and `Approved`; §7 uses `Validation`, `Review`, and `Approval` for the same lifecycle positions.
- H09 defines the canonical `Evidence Rejected` event but no explicit `Rejected` Evidence Repository Item state.
- H09 evidence workflows define multiple actors but do not designate a singular final evidence approver.

These issues do not prevent the evidence approval/rejection business outcome from being represented because the governing workflow, event, source boundary, and approved lifecycle are otherwise explicit.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Normalize the evidence-state labels, clarify rejected-state treatment, and optionally identify the canonical final approval role within H09 evidence governance.
