# Duplicate Workforce Profiles to Canonical Workforce Resolution

**Scenario ID:** L3-E-009  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E01 — Workforce & Engagement Workflows; L2-E02 — Position & Assignment Workflows where assignment consequences are involved  
**Referenced By:** Future L4 System Behavior Specifications and materially participating downstream workforce consumers

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-009 |
| Canonical Title | Duplicate Workforce Profiles to Canonical Workforce Resolution |
| Primary Owner | Workforce & Engagement |
| Participating Domains | E |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose
Resolve duplicate Workforce Profiles through the canonical Workforce governance workflow while preserving the complete workforce history associated with the canonical profile and avoiding deletion of workforce history.

## 3. Business Outcome
Duplicate Workforce Profiles are resolved into the canonical workforce representation through the governed merge workflow, with engagement and related historical continuity preserved and the authoritative merge fact published.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
Duplicate Workforce Profiles have been identified and a governed profile-resolution decision is available.

### 4.2 End Boundary
The duplicate-profile resolution is completed, canonical workforce continuity is preserved, and the authoritative merge fact has been published.

### 4.3 In Scope
- Identification of duplicate Workforce Profiles.
- Controlled Workforce Profile Merge Workflow.
- Preservation of engagement history.
- Preservation of Position Assignment history.
- Preservation of Responsibility history.
- Preservation of audit/history.
- Publication of the canonical merge event.
- Canonical workforce resolution.

### 4.4 Out of Scope
- Canonical Person ownership.
- Authentication or identity-account lifecycle.
- Creating or modifying Position Assignments.
- Responsibility Assignment ownership.
- Payroll or leave processing.
- Physical database deletion or technical data migration mechanics.

### 4.5 Related Scenarios
E001 Workforce Engagement Establishment and Activation; E007 Workforce Engagement Closure and Institutional Exit; E008 Former Workforce Member Rejoining.

## 5. Ownership and Participating Domains
Workforce & Engagement owns Workforce Profile governance and the controlled merge workflow. L2-E01 explicitly assigns duplicate-profile resolution to **WF-016 — Merge Workforce Profiles** and requires preservation of complete audit, engagement, Position Assignment, and Responsibility history.

## 6. Business Actors, Roles and Responsibilities
- Workforce & Engagement owns the canonical Workforce Profile resolution outcome.
- The authorised institutional governance process determines the merge decision.
- Position & Assignment remains the canonical owner of Position Assignment and Responsibility Assignment history; E01 preserves/references that history as part of the merge outcome without re-owning it.
- No unsupported technical role or approval authority is introduced.

## 7. Preconditions and Entry Criteria
- More than one Workforce Profile has been identified as representing duplicate workforce identity within the applicable institutional context.
- A canonical profile can be determined through the governed resolution process.
- The merge is authorized through the applicable business governance.
- Associated Workforce Engagement history is available for preservation.

## 8. Trigger
A governed duplicate-Workforce-Profile resolution decision authorizes the merge.

## 9. Main Business Flow
1. Identify the duplicate Workforce Profiles.
2. Determine the canonical Workforce Profile under the governed resolution process.
3. Review the associated workforce history that must remain continuous.
4. Execute **WF-016 — Merge Workforce Profiles**.
5. Consolidate the workforce representation under the canonical profile.
6. Preserve engagement history.
7. Preserve Position Assignment history.
8. Preserve Responsibility history.
9. Preserve complete audit history.
10. Publish **`WorkforceProfilesMerged`**.
11. Downstream consumers use the authoritative merge fact and canonical workforce representation.

## 10. Alternate Business Flows
- If duplicate status is not confirmed, no merge is performed.
- If the canonical profile cannot be determined, the resolution remains unresolved and the authoritative profiles remain unchanged.
- If governance rejects the proposed merge, no merge outcome is completed.

## 11. Failure, Interruption and Business Recovery
If the governed merge cannot validly complete, the existing canonical workforce records and their historical relationships remain authoritative. Resolution may be reconsidered through the governed merge workflow after the underlying business issue is resolved. No technical data-migration, rollback, deletion, or retry mechanics are defined at L3.

## 12. Business Rules and Decision Constraints
- A Workforce Profile shall not be duplicated for the same canonical person within the applicable tenant.
- Duplicate Workforce Profiles are resolved through the controlled Workforce Profile Merge Workflow.
- Workforce Profiles are not deleted as a business lifecycle action.
- Engagement history must be preserved.
- Position Assignment history must be preserved.
- Responsibility history must be preserved.
- Complete audit history must be preserved.
- The merge event is published by Workforce & Engagement.

## 13. State and Lifecycle Impact
The business outcome is a **canonical Workforce Profile resolution** rather than a new workforce engagement lifecycle.

Historical Workforce Engagements and associated assignment/responsibility history remain preserved. The scenario does not invent or rename a Workforce Profile state transition.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `WorkforceProfilesMerged` | Workforce & Engagement | Duplicate Workforce Profiles have been resolved through the canonical merge workflow |

L2-E01 explicitly defines `WorkforceProfilesMerged` for WF-016.

## 15. Business Authorization, Approval and Audit Obligations
Profile merging is a controlled governance workflow. The resolution decision and resulting canonical representation must remain auditable, including preservation of the histories explicitly required by L2-E01. The scenario does not invent a specific approver, separation-of-duty rule, or evidence artefact.

## 16. Completion Criteria and Postconditions
- A canonical Workforce Profile has been established for the duplicate-resolution outcome.
- Workforce Engagement history is preserved.
- Position Assignment history is preserved.
- Responsibility history is preserved.
- Complete audit history is preserved.
- `WorkforceProfilesMerged` has been published.
- No competing workforce lifecycle ownership is created.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, final outcome, lifecycle, event publisher, or boundary was identified in the locked L2-E01 material used for this scenario.

**Authoring classification: PASS — source-grounded.**
