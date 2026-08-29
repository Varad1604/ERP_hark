# Dashboard Change to Versioned Publication or Rollback

# 1. Scenario Metadata

**Scenario ID:** L3-H-052  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Dashboard  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A04 — Authorization & Policy Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern approved dashboard composition changes through immutable H07 version publication and, when required by the frozen semantics, restore a prior governed version without rewriting history.

# 3. Business Outcome

A new immutable version is published or a prior version is restored with full lineage

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an approved dashboard composition changes or a published version must be restored.

## 4.2 End Boundary

The scenario ends when H07 has either published the approved new immutable version or completed the governed rollback to a selected prior immutable version with lineage preserved.

## 4.3 In Scope

- Govern H07 Dashboard Version evolution.
- Validate and approve a candidate version before publication.
- Preserve immutability of published/locked versions.
- Perform H07-governed rollback by selecting a prior valid version rather than editing history.
- Preserve authorization and version lineage evidence.

## 4.4 Out of Scope

- Rewriting an already published dashboard version.
- Changing source-domain business facts because dashboard composition changed.
- Implementation mechanics for rollback.
- Choosing between conflicting H07 publication-event names.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Enterprise Dashboard / H07 | Owns Dashboard Version lifecycle, publication, rollback governance, and version lineage. |
| Platform Foundation / A04 | Owns authorization decisions governing version approval, publication, and rollback actions. |

# 6. Business Actors, Roles and Responsibilities

H07 identifies governance/release authority for publication and rollback activities. A04 remains the canonical owner of authorization decisions. Source-domain owners remain responsible for the business facts represented by the dashboard.

# 7. Preconditions and Entry Criteria

- A governed dashboard exists.
- A composition change is approved for version governance, or a prior published version is eligible for governed restoration.
- The acting participant is authorized for the requested publication or rollback decision.
- Referenced source information remains owned upstream.

# 8. Trigger

An approved dashboard composition changes or a published version must be restored

# 9. Main Business Flow

1. H07 creates a candidate Dashboard Version from the approved composition change.
2. The candidate version is validated against H07 governance constraints and authoritative source references.
3. The authorized H07 governance authority approves the valid version.
4. H07 publishes the approved version as an immutable governed version.
5. The published version becomes the current governed publication without altering prior versions.
6. H07 preserves the complete version lineage and governing evidence.

# 10. Alternate Business Flows

- **Governed rollback:** an authorized rollback decision selects an eligible prior immutable version; H07 validates that version under current governance and records a new rollback action while preserving both the current and historical versions.
- A future change after rollback proceeds through another governed version publication rather than mutation of any historical version.

# 11. Failure, Interruption and Business Recovery

If a candidate version or historical rollback target does not satisfy current governance, authorization, or compatibility conditions, it is not made current. The last valid governed publication remains authoritative until a supported new publication or rollback outcome is approved.

# 12. Business Rules and Decision Constraints

- Published dashboard versions are immutable.
- Rollback does not edit an existing published version; it changes the governed current selection while preserving history.
- Source-domain facts and KPI definitions remain owned by their canonical domains.
- A04 authorization decisions govern publication and rollback authority.

# 13. State and Lifecycle Impact

H07 defines the Dashboard Version lifecycle `Candidate → Validated → Approved → Published → Locked → Superseded → Archived`. Governed rollback uses a prior immutable version without moving an Archived version directly back to Published or rewriting a Locked version.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H07 consistently defines `DashboardVersionCreated` and `DashboardVersionRolledBack`. Publication naming is internally inconsistent: the Dashboard Version event catalogue lists `DashboardVersionPublished`, while the Publish Dashboard workflow/event mapping uses `DashboardPublished` and `DashboardVersionLocked`. H07 is the publisher in all cases; L3 does not choose or rename the publication event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

A04 supplies authorization decisions. H07 keeps version governance and lineage; source-owning domains continue to supply the authoritative facts represented by the dashboard without ownership transfer.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Publication and rollback require the H07 governance/release authority identified upstream and remain subject to A04 authorization.

## 15.2 Separation of Duties, Overrides and Exceptions

No rollback decision may rewrite a historical published version or bypass current authorization/governance conditions. Dashboard version control may not change source-domain business facts.

## 15.3 Business Audit and Evidence Obligations

Preserve candidate-version evidence, validation and approval history, publication evidence, rollback reason and authority, selected historical version, and complete version lineage.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

A new approved immutable version is current, or a governed rollback to an eligible prior immutable version is completed with lineage intact.

## 16.2 Postconditions

All historical H07 versions remain immutable and traceable; source-domain ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

**Non-blocking H07 event-name inconsistency:** the Dashboard Version event catalogue names `DashboardVersionPublished`, while the Publish Dashboard workflow/event mapping names `DashboardPublished` and `DashboardVersionLocked`. The owning domain/publisher and version lifecycle are clear, and `DashboardVersionRolledBack` is consistently defined.

**Architecture impact:** publication-event normalization is required upstream before L4 event behavior is specified; the L3 versioned-publication/rollback business outcome remains source-grounded.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Normalize the canonical H07 event set for Dashboard Version publication.
