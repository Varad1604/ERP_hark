# Physical Zone Establishment to Infrastructure Grouping

# 1. Scenario Metadata

**Scenario ID:** L3-G-285  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L2-G10; `L2-G10_Infrastructure_Blocker_Remediation_Amendment_2026-09-01.md`; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

# 2. Scenario Purpose
Establish a G10 Zone as an operational logical grouping without changing the canonical physical hierarchy.

# 3. Business Outcome
A canonical Zone is established in `Proposed` and linked to applicable physical infrastructure.

# 4. Scenario Boundary and Scope
Zone establishment and membership are G10-owned. Zone is not a Campus/Building/Floor/Space hierarchy level.

# 5. Ownership and Participating Domains
Infrastructure Management owns Zone, Zone Membership and their lifecycle/rules.

# 6. Business Actors, Roles and Responsibilities
Infrastructure Administrator creates the Zone and manages membership.

# 7. Preconditions and Entry Criteria
Zone Type is configured, Zone Name satisfies uniqueness rules and referenced infrastructure exists.

# 8. Trigger
The institution requires physical areas to be grouped into an operational Zone.

# 9. Main Business Flow
1. Validate Zone Type and Zone Name.  
2. Create the Zone in `Proposed`.  
3. Publish `ZoneCreated`.  
4. Validate infrastructure memberships without re-parenting physical hierarchy.  
5. Establish each membership and publish `AssignedToZone`.

# 10. Alternate Business Flows
Infrastructure may participate in multiple valid Zones.

# 11. Failure, Interruption and Business Recovery
Invalid type/name or invalid membership prevents the affected action.

# 12. Business Rules and Decision Constraints
Zone membership never changes physical hierarchy; multiple Zone memberships are allowed.

# 13. State and Lifecycle Impact
New Zone enters `Proposed`; the Zone-specific lifecycle remains authoritative. Membership does not alter member lifecycle.

# 14. Business Events and Cross-Domain Collaboration
G10 publishes `ZoneCreated` and `AssignedToZone`. `ZoneRegistered` is legacy vocabulary for the same establishment fact and is not published.

# 15. Business Authorization, Approval and Audit Obligations
Preserve Zone definition, membership changes and acting Infrastructure Administrator.

# 16. Completion Criteria and Postconditions
Zone exists canonically in Proposed with valid memberships and complete event/audit evidence.

# 17. Open Issues, Assumptions and Deferred Questions
No blocking issues. No assumptions.
