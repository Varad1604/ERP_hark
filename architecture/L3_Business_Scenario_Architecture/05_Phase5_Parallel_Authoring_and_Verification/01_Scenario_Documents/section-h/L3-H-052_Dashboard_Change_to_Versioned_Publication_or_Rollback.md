# Dashboard Change to Versioned Publication or Rollback

# 1. Scenario Metadata

**Scenario ID:** L3-H-052  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Dashboard  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A04 — Authorization & Policy Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Dashboard publication, dashboard restoration, audit, and retirement scenarios

# 2. Scenario Purpose

Govern changes to an H07 Dashboard through immutable versioning and provide a controlled restoration path for a prior published composition without changing source-domain facts or weakening Section A authorization.

# 3. Business Outcome

A new immutable version is published or a prior version is restored with full lineage

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An approved dashboard composition changes or a published version must be restored.

## 4.2 End Boundary

For a normal governed change, the scenario can end when a new immutable version is validated, approved, and published. The rollback/restoration branch is materially ambiguous because H07 defines both creation of a new rollback version and direct switching of the active publication pointer to a prior immutable version.

## 4.3 In Scope

- H07 Dashboard metadata change and version governance.
- Validation, approval, and publication of a new immutable version.
- H07 rollback/restoration semantics only where unambiguous.
- Preservation of version lineage and Section A authorization constraints.

## 4.4 Out of Scope

- Choosing between contradictory H07 rollback models.
- Editing a published version in place.
- Modifying source-domain business facts.
- Technical deployment, caching, UI, or persistence mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Owns authorization policies and decisions governing update, publication, and rollback authority. |
| Enterprise Dashboard / H | Owns Dashboard metadata, Dashboard Version, publication metadata, version history, and rollback governance. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Dashboard Owner | Initiates governed dashboard changes and publication where authorized. | Initiates |
| Administrator | Performs H07 dashboard update/publication and rollback administration where authorized. | Performs |
| Governance Reviewer | Reviews changed dashboard metadata where H07 requires governance review. | Reviews |
| Approver | Provides H07-required publication approval. | Approves |
| Release Manager | Initiates the H07-defined rollback workflow where authorized. | Initiates rollback |

# 7. Preconditions and Entry Criteria

- The Dashboard exists under H07 ownership.
- A normal change is applied only to an editable H07 position and never mutates a published immutable version.
- For rollback, a prior published version exists and is rollback-eligible.
- Applicable Section A authorization and H07 governance authority are satisfied.

# 8. Trigger

An approved dashboard composition changes or a published version must be restored

# 9. Main Business Flow

1. Receive the approved dashboard-composition change.
2. Preserve the current published version as immutable historical evidence.
3. Apply the change to H07-owned dashboard metadata through the editable/versioned path.
4. Validate structure, widget compatibility, policy, authorization, and dependency compatibility.
5. Complete H07 review and approval for the successor publication.
6. Publish the new immutable Dashboard Version and preserve lineage to prior versions.
7. Stop at the successful new-version publication outcome for the normal change branch.
8. For a rollback request, verify the selected historical version is rollback-eligible and stop before selecting the final restoration mechanism because H07 contains contradictory authoritative instructions.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Rollback/restoration | A prior published version must be restored | H07 requires compatibility and authorization validation, but the final version action is disputed upstream | BLOCKED pending authoritative rollback-model reconciliation | Enterprise Dashboard |
| Publication rejection | Changed version fails validation or approval | Do not publish; preserve existing published history | Existing published version remains unchanged | Enterprise Dashboard |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Changed composition is invalid or unapproved | Successor version cannot be published | Correct and resubmit the editable version through H07 governance | Enterprise Dashboard |
| Rollback target is incompatible or unauthorized | Prior version cannot be restored | Reject the rollback request | Enterprise Dashboard / Platform Foundation |
| Conflicting rollback semantics apply | Frozen restoration branch cannot be completed unambiguously | Upstream H07 architecture must select the canonical rollback/version behaviour | Enterprise Dashboard |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Published dashboard versions are immutable | Enterprise Dashboard | Historical published versions are never edited in place | L2-H07 BR-VER-001 |
| Every configuration modification is traceable | Enterprise Dashboard | New-version lineage must be preserved | L2-H07 BR-VER-003 |
| Version history is never physically deleted | Enterprise Dashboard | Historical lineage remains available | L2-H07 BR-VER-004 |
| Rollback creates a new version and historical versions remain unchanged | Enterprise Dashboard | One authoritative H07 rule points to a successor rollback version | L2-H07 BR-VER-002; §12.6 |
| Rollback switches the active pointer to a prior immutable version | Enterprise Dashboard | Another authoritative H07 workflow/state rule points to direct restoration of the historical version | L2-H07 §51.24; §52.7 |
| Authorization remains governed by Section A | Platform Foundation | Update/publication/rollback cannot expand authorization | L2-A04; L2-H07 §17 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Dashboard Version | Candidate/Created and other pre-publication positions as defined upstream | Validate, approve and publish normal successor version | New immutable Published/Locked version with lineage | Enterprise Dashboard | L2-H07 §§12.6, 52.7 |
| Dashboard Version | Prior published historical version | Rollback/restoration requested | Ambiguous: new rollback version versus active-pointer switch to prior version | Enterprise Dashboard | L2-H07 BR-VER-002; §§12.6, 51.24, 52.7 |

H07 also differs internally on the version lifecycle itself: §12.6 begins with `Created` and omits `Locked`, while §52.7 begins with `Candidate` and includes `Locked`.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| DashboardUpdated | Enterprise Dashboard | H07 governance participants | Records supported metadata update | L2-H07 §23.2 |
| DashboardPublished | Enterprise Dashboard | Authorized dashboard consumers | Records dashboard publication | L2-H07 §23.2; §51.7 |
| DashboardVersionRolledBack | Enterprise Dashboard | H07 version governance participants | Records rollback action | L2-H07 §23.8; §51.24 |

H07 additionally conflicts on `DashboardVersionLocked` versus `DashboardVersionPublished`; L3 does not rename or reconcile those events.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A supplies authorization decisions and constraints. H07 owns only dashboard/version metadata. Source-domain business facts remain read-only to H07 and are not versioned or rolled back by this scenario.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Update dashboard | Dashboard Owner / Administrator under H07 governance | Must operate on an editable governed version and satisfy Section A authorization | Enterprise Dashboard |
| Approve/publish successor | Approver / Dashboard Owner / Administrator as defined by H07 | Requires validation and publication authority | Enterprise Dashboard |
| Rollback | Administrator / Release Manager | Requires rollback eligibility, compatibility, policy clearance, and release authority | Enterprise Dashboard |

## 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is defined beyond H07 approval/release authority and Section A authorization. Rollback must not be used to bypass current policy.

## 15.3 Business Audit and Evidence Obligations

Preserve before/after change history, approval evidence, version lineage, publication history, rollback reason, target-version reference, and the resulting governed action. The exact rollback-version consequence remains unresolved upstream.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The normal change branch completes when a new immutable H07 version is validated, approved, published, and linked to its lineage.
- The full frozen scenario cannot be completed while the rollback branch has contradictory upstream version semantics.

## 16.2 Postconditions

Normal successor publication leaves historical versions immutable and source-domain business facts unchanged. No rollback outcome is fabricated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H07 BR-VER-002 and §12.6 state that rollback creates a **new version** based on the selected historical version.
- H07 §51.24 and §52.7 instead state that rollback **switches/repoints the active publication to the prior immutable version**, with the original versions unchanged.
- H07's Dashboard Version lifecycle also conflicts: §12.6 is `Created → Validated → Approved → Published → Superseded → Archived`, while §52.7 is `Candidate → Validated → Approved → Published → Locked → Superseded → Archived`.
- H07 uses `DashboardVersionLocked` in publication workflow/state mapping while its version-event catalogue defines `DashboardVersionPublished`.

These conflicts materially affect the frozen rollback outcome, lifecycle position, and event semantics.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H07 architecture must select one canonical rollback model and reconcile the Dashboard Version lifecycle/event vocabulary before the rollback branch can be completed.
