# Notification Template Request to Governed Published Template

# 1. Scenario Metadata

**Scenario ID:** L3-H-012  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Notification  
**Participating Domains:** H  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L2-H02; `L2-H02_Notification_Template_Approval_Amendment_2026-09-01.md`; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

# 2. Scenario Purpose
Govern Notification Template versions through validation, review, approval, publication, deprecation and retirement.

# 3. Business Outcome
A validated Template version is approved and published or returned for revision, with later deprecation/retirement preserving prior use.

# 4. Scenario Boundary and Scope
H02 owns Template lifecycle/versioning. Source domains retain the business meaning/rules represented in communication content.

# 5. Ownership and Participating Domains
Notification owns Template identity, validation, lifecycle, publication and history.

# 6. Business Actors, Roles and Responsibilities
Notification Template Approval Authority decides review/approval/publication eligibility under institutional communication policy.

# 7. Preconditions and Entry Criteria
Required metadata, variables, localization/channel context and business content are available for validation.

# 8. Trigger
New or revised communication content requires governance.

# 9. Main Business Flow
1. Create new Template version in `Draft`.  
2. Validate metadata/variables and move to `Under Review`.  
3. Approval Authority either returns it `Under Review → Draft` or approves `Under Review → Approved`.  
4. Authorized publication transitions `Approved → Published`.  
5. Later governance may move `Published → Deprecated → Retired`.

# 10. Alternate Business Flows
Published content correction creates a new version; published versions remain immutable.

# 11. Failure, Interruption and Business Recovery
Invalid Template data or absent approval prevents publication.

# 12. Business Rules and Decision Constraints
Templates contain no executable business logic; published versions are immutable; revisions create new versions.

# 13. State and Lifecycle Impact
`Draft → Under Review → Approved → Published → Deprecated → Retired`, with source-defined return/reconsideration paths.

# 14. Business Events and Cross-Domain Collaboration
No new Template lifecycle event is required for this scenario; Template decisions remain H02-owned and auditable.

# 15. Business Authorization, Approval and Audit Obligations
Preserve Template version, validation evidence, decision authority, publication/deprecation/retirement history.

# 16. Completion Criteria and Postconditions
Template reaches Published or the governed revision path; later retirement preserves historical use.

# 17. Open Issues, Assumptions and Deferred Questions
No blocking issues. No assumptions.
