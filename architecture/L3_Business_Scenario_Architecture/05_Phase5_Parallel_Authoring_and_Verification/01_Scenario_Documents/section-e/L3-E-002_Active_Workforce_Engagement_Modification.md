# Active Workforce Engagement Modification

**Scenario ID:** L3-E-002  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** E  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  
**Authoritative L2:** L2-E01 — Workforce & Engagement Workflows

## 1. Scenario Metadata
Frozen Section E identity is preserved. Workforce Engagement is the canonical E01 aggregate. E02 retains exclusive ownership of assignment aggregates.

## 2. Scenario Purpose
Apply an authorised business modification to an existing active Workforce Engagement while preserving effective dating and historical continuity.

## 3. Business Outcome
The Workforce Engagement reflects the authorised modification from its applicable effective boundary while prior business history remains traceable.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An active Workforce Engagement requires a valid business modification.
### 4.2 End Boundary
The modified Workforce Engagement position is authoritative for its applicable effective context and the resulting business fact is available to consumers.
### 4.3 In Scope
- Workforce Engagement modification.
- Effective dating.
- Historical continuity.
- Canonical workforce business publication.
### 4.4 Out of Scope
- E02 assignment lifecycle.
- Person/organisation master ownership.
- Payroll, leave calculation, IAM and technical implementation.
### 4.5 Related Scenarios
E001 — Workforce Engagement Establishment and Activation; E003 — Workforce Transfer Between Organisational Contexts; E004 — Workforce Engagement Suspension and Operational Restoration.

## 5. Ownership and Participating Domains
Workforce & Engagement owns Workforce Engagement. E02 owns Position Assignment, Responsibility Assignment, Reporting Relationship and Assignment History. No participating domain gains ownership through collaboration.

## 6. Business Actors, Roles and Responsibilities
Authorised workforce business participants act according to upstream Workforce/Policy/Workflow governance. Exact role names are not invented where not exposed by the retrieved L2 source.

## 7. Preconditions and Entry Criteria
- Active Workforce Engagement exists.
- Valid modification decision/request exists.
- Applicable Person, organisation and workforce context is authoritative.
- Any E02-owned change is separately governed by E02.

## 8. Trigger
An authorised change to an active Workforce Engagement is established.

## 9. Main Business Flow
1. Identify the active Workforce Engagement.
2. Validate the modification against applicable governance.
3. Determine its effective business boundary.
4. Record the modified Workforce Engagement position.
5. Preserve the previous effective/history position.
6. Publish the canonical modification business fact.
7. Make resulting workforce information available to consumers.

## 10. Alternate Business Flows
- Unauthorised changes are not applied.
- Future-effective changes are not treated as current before their boundary.
- E02-owned assignment changes are not performed in E01.
- Historical business facts are not overwritten.

## 11. Failure, Interruption and Business Recovery
If modification cannot validly complete, the existing authoritative engagement position remains in force. Upstream Person/organisation corrections remain upstream-owned. No technical retry or rollback is defined.

## 12. Business Rules and Decision Constraints
- Workforce Engagement remains E01-owned.
- Significant changes are effective-dated.
- Historical business records remain traceable.
- Corrections use governed business workflows rather than deletion.
- E02 exclusively owns assignment-state changes.

## 13. State and Lifecycle Impact
The Workforce Engagement remains within its E01 lifecycle while a new effective business position/history is established. No new aggregate or local state is introduced.

## 14. Business Events and Cross-Domain Collaboration
The retrieved L2-E01 material confirms event publication after successful state transitions but does not expose a verified exact modification-event name. No event is invented. Verify the detailed L2-E01 event catalogue before lock.

## 15. Business Authorization, Approval and Audit Obligations
Modification authority follows applicable Workforce/Policy/Workflow governance. Exact approval-role names are not invented. Business decisions remain auditable.

## 16. Completion Criteria and Postconditions
- Valid modification established.
- Effective context authoritative.
- Prior history preserved.
- Canonical modification fact published.
- Consumers can rely on the resulting workforce position.

## 17. Open Issues, Assumptions and Deferred Questions
**OI-E002-01 — Event vocabulary:** Exact modification event requires verification against detailed L2-E01 event catalogue.

**OI-E002-02 — Detailed rule/state vocabulary:** Exact modification state/decision entries require verification before lock.

**Classification: PASS WITH ISSUES — source-grounded authoring; not lock-ready until detailed upstream vocabulary is verified.**
