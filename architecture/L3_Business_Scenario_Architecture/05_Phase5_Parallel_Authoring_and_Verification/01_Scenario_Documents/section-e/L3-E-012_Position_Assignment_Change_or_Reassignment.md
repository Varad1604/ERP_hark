# Position Assignment Change or Reassignment

**Scenario ID:** L3-E-012
**Primary Owner:** Position & Assignment
**Participating Domains:** B, E
**Scenario Type:** Critical
**Priority:** P0
**Architecture Level:** L3 — Business Scenario Architecture
**Status:** Locked  
**Version:** 1.0
**Owner:** Varad
**Last Updated:** 2026-08-20

**Depends On:** L2-E02 — Position & Assignment Workflows; L2-E01 — Workforce & Engagement Workflows; Organisation Management
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
Frozen Section E identity is preserved. Position Assignment remains exclusively E02-owned.

## 2. Scenario Purpose
Govern an authorised change or reassignment of an existing Position Assignment while preserving stable assignment identity, historical lineage, effective dating, and organisational authority boundaries.

## 3. Business Outcome
The Position Assignment has an authoritative successor business position reflecting the approved change, with the prior position preserved as immutable history and the resulting business facts published.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An active Position Assignment is subject to an authorised organisational change or reassignment.
### 4.2 End Boundary
The approved successor Position Assignment outcome is authoritative at its effective boundary.
### 4.3 In Scope
- Existing assignment validation.
- Target Position/organisation resolution.
- E02 transfer/change workflow.
- Successor Assignment Version.
- Authority/reporting consequences.
- Historical lineage.
- Completed assignment-change events.
### 4.4 Out of Scope
Workforce Engagement lifecycle, Position Definition lifecycle, payroll, leave administration, recruitment, and technical implementation.
### 4.5 Related Scenarios
E003 — Workforce Transfer Between Organisational Contexts; E011 — Workforce Member Assignment to Active Institutional Position; E013 — Position Assignment Completion or Termination.

## 5. Ownership and Participating Domains
Position & Assignment owns Position Assignment. Workforce & Engagement remains authoritative for Workforce Engagement. Organisation Management remains authoritative for Position Definition and organisational structure.

## 6. Business Actors, Roles and Responsibilities
The authorised institutional authority governs reassignment; Position & Assignment executes the Position Assignment lifecycle; Workforce & Engagement supplies the valid engagement; Organisation Management supplies target organisational context.

## 7. Preconditions and Entry Criteria
- Existing Position Assignment is valid and operational.
- Workforce Engagement remains valid.
- Target Position/organisation is authoritative.
- Applicable change policy is satisfied.
- Required approval is complete.

## 8. Trigger
An authorised organisational change to an existing Position Assignment is established.

## 9. Main Business Flow
1. Identify the existing Position Assignment.
2. Validate the assignment and Workforce Engagement.
3. Resolve the target Position/organisational context.
4. Validate the change under E02 rules.
5. Execute **W-004 — Transfer Position Assignment** where the change is an organisational transfer/reassignment.
6. Create the successor Assignment Version while preserving stable assignment identity.
7. Establish the target effective organisational position.
8. Update applicable reporting and authority outcomes.
9. Preserve the prior version as immutable history.
10. Publish the canonical completed assignment-change facts.

## 10. Alternate Business Flows
- Future-effective change does not alter current state before its effective boundary.
- Invalid target Position/organisation rejects the change.
- Unauthorised change leaves existing assignment authoritative.
- A change that is actually a Workforce Engagement lifecycle change remains E01-owned.

## 11. Failure, Interruption and Business Recovery
If reassignment cannot validly complete, the existing Position Assignment remains authoritative. Historical versions are not overwritten. Business correction follows canonical E02 workflow. Technical retry/rollback is outside L3.

## 12. Business Rules and Decision Constraints
- Stable Assignment Identity is preserved through transfer.
- Transfers create successor Assignment Versions.
- Historical versions remain immutable.
- Authority transitions at the effective boundary.
- Reporting hierarchy is recalculated where applicable.
- Operational scope is revalidated.
- Position Assignment is exclusively E02-owned.

## 13. State and Lifecycle Impact
The authoritative E02 transfer path is **W-004 — Transfer Position Assignment**. Successor-version and effective-date semantics remain governed by E02.

## 14. Business Events and Cross-Domain Collaboration
| Event | Publisher |
|---|---|
| `PositionAssignmentTransferred` | Position & Assignment |
| `AssignmentVersionCreated` | Position & Assignment |
| `OrganizationalAuthorityChanged` | Position & Assignment |
| `ReportingHierarchyChanged` | Position & Assignment |
| `OperationalScopeChanged` | Position & Assignment |

## 15. Business Authorization, Approval and Audit Obligations
Reassignment follows E02 approval/governance, including applicable organisational authority and effective-date policy. No fixed approver is invented where policy is configurable.

## 16. Completion Criteria and Postconditions
- Approved Position Assignment change is complete.
- Successor position is authoritative for its effective period.
- Stable identity/version lineage is preserved.
- Applicable authority/reporting outcomes are updated.
- Prior history remains immutable.
- Canonical assignment-change events are published.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, outcome, lifecycle, event publisher or boundary was identified in the authoritative sources read for this batch.

**Authoring classification: PASS — source-grounded production-quality Draft.**
