# Course Offering Revision, Replacement and Rollback

**Scenario ID:** L3-D-009  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0729 / CAN-L3-D-009

---

# 2. Scenario Purpose

Govern version-based change to a Course Offering through revision and successor replacement, with restoration of a previous approved version when an authorized rollback is required.

# 3. Business Outcome

The Course Offering has a governed current version—either an authorized successor replacement or a restored previously approved version—while all prior versions and lineage remain immutable.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized version-change or rollback requirement exists for a Course Offering with preserved version history.

## 4.2 End Boundary

The authorized successor version becomes current or the approved rollback target is restored, and the applicable exact version/rollback events are published.

## 4.3 In Scope

- Create and validate a new Draft version.
- Publish and activate an authorized successor replacement.
- Request and complete rollback to an existing approved target.
- Preserve immutable version lineage and history.

## 4.4 Out of Scope

- Merge or split.
- Technical transaction rollback.
- Direct modification of downstream registrations, assignments or timetables.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns Course Offering versions, successor replacement, rollback decision and immutable lineage. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Authorized administrator | May perform governed offering change under applicable version and rollback policy. | Initiates / performs. |

The Course Offering Governance Authority approves material successor replacement and rollback; the Authorized administrator initiates and executes the governed action.

# 7. Preconditions and Entry Criteria

- The Course Offering exists and is not Archived.
- Version lineage is available and immutable.
- For rollback, the target version exists and is approved.

# 8. Trigger

An authorized revision/replacement requirement or rollback request is raised for the Course Offering.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Classify the requirement as successor revision/replacement or rollback. | D01 | The governed path is selected. |
| 2 | Validate lifecycle eligibility, version policy, effective dates and historical integrity. | D01 | The requested version action is valid or rejected. |
| 3 | For revision/replacement, create a new Draft version linked to its predecessor. | D01 | `CourseOfferingVersionCreated` records the successor Draft. |
| 4 | Validate and authorize the successor version for publication. | D01 / authorized administrator | The successor may replace the current version. |
| 5 | Publish and activate the authorized successor without deleting the previous version. | D01 | `CourseOfferingVersionPublished` records the new current version. |
| 6 | For rollback, validate the approved target and record the rollback request. | D01 | `CourseOfferingRollbackRequested` records the governed request. |
| 7 | Restore the approved target as the current governed version while preserving all intervening history. | D01 | `CourseOfferingRollbackCompleted` records the restored current position. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Successor replacement | A governed change requires a new version. | Create, authorize and publish a successor version. | New current version with preserved predecessor. | D01 |
| Rollback | A previous approved version must be restored. | Validate target, authorize and restore it without deleting history. | Restored current version with preserved lineage. | D01 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Offering is Archived or lifecycle prohibits version change | The change cannot proceed. | Reject the request and preserve the current version. | D01 |
| Rollback target does not exist or is not approved | Restoration is invalid. | Reject rollback; preserve current version. | D01 |
| Successor version fails validation or authorization | Replacement cannot become current. | Retain it outside the published/current position or return it for correction according to policy. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Version history is immutable and lineage must be preserved. | D01 | Prohibits overwrite or deletion. | L2-D01 §§43,55 |
| Only one version is Active at a time. | D01 | Controls successor replacement. | L2-D01 §43 |
| Rollback restores a previous approved version and never deletes history. | D01 | Defines business rollback. | L2-D01 §51 |
| Archived offerings cannot change. | D01 | Blocks version actions after archive. | L2-D01 §§12,52 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Course Offering version lineage | One designated current version plus immutable historical versions | Create and approve a successor or redesignate an approved historical version as current. | Exactly one designated current version; all other versions remain immutable historical versions | D01 | L2-D01 §§43–45,51; accepted Batch 1 decision 7 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CourseOfferingVersionCreated | D01 | Interested D domains | Records successor version creation. | L2-D01 §§56,70 |
| CourseOfferingVersionPublished | D01 | Interested D domains | Records successor becoming current. | L2-D01 §§56,70 |
| CourseOfferingRollbackRequested | D01 | Interested D domains | Records governed rollback request. | L2-D01 §§56,70 |
| CourseOfferingRollbackCompleted | D01 | Interested D domains | Records restored approved version. | L2-D01 §§56,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain business contract or hand-off materially participates in this scenario.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate revision/replacement or rollback | Authorized Course Offering administrator | Must satisfy version, lifecycle and rollback policy. | D01 |
| Approve material successor replacement or rollback | Course Offering Governance Authority | Requires validated target or successor, preserved immutable lineage and recorded rationale. | D01 |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Version lineage | All predecessor/successor references, effective positions and preserved versions. | D01 |
| Replacement or rollback decision | Request, validation basis, authorization and resulting current version. | D01 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The applicable version action is authorized.
- Exactly one governed current version exists.
- All prior versions and lineage remain preserved.
- The applicable exact event is published.

## 16.2 Postconditions

- No historical version is deleted or rewritten.
- Downstream domains may react to the completed version fact without D01 modifying their aggregates.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** `Section_D_Canonical_Register.md` and `L2-D01_Course_and_Course_Offering_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500; accepted Batch 1 resolution decisions 1–7 (2026-08-20).
