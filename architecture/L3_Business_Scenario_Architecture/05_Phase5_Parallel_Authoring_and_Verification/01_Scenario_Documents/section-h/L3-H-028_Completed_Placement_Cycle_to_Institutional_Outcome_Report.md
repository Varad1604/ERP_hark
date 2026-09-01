# Completed Placement Cycle to Institutional Outcome Report

# 1. Scenario Metadata

**Scenario ID:** L3-H-028  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L2-H04; L2-C02; L2-E01; `L2-H04_Placement_Outcome_Reporting_Clarification_2026-09-01.md`; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

# 2. Scenario Purpose
Produce institutionally usable Placement outcome reporting from completed, authoritative Placement facts without inventing diversity or publication architecture.

# 3. Business Outcome
Verified Placement statistics and supported Placement outcomes are available for authorized institutional reporting and dashboard consumption.

# 4. Scenario Boundary and Scope
The scenario is intentionally limited to H04-supported Placement statistics/read models. It does not introduce a universal diversity framework, a report-approval/publication lifecycle or a generic reporting-period aggregate.

# 5. Ownership and Participating Domains
H04 owns Placement outcomes/statistics. C and E retain their source facts. H07 owns any later dashboard composition.

# 6. Business Actors, Roles and Responsibilities
Placement governance roles close the Placement cycle and generate/use H04 statistics under authorized institutional scope.

# 7. Preconditions and Entry Criteria
A Placement cycle/drive scope is closed and required Placement-owned outcomes are in governed completed positions.

# 8. Trigger
A Placement cycle/drive scope closes.

# 9. Main Business Flow
1. Resolve the closed Placement scope.  
2. Assemble completed Placement Drive, offer, joining, internship and employer outcomes.  
3. Reference permitted C/E facts without changing their ownership.  
4. Generate H04 Placement statistics/read-model information.  
5. Publish `PlacementStatisticsGenerated`.  
6. Make the supported projection available to authorized institutional/reporting consumers.

# 10. Alternate Business Flows
Incomplete outcomes are excluded/deferred rather than inferred.

# 11. Failure, Interruption and Business Recovery
Unreconciled statistics are not treated as verified; unsupported diversity metrics are not manufactured.

# 12. Business Rules and Decision Constraints
Closed Placement history is immutable; H04 reporting cannot become a competing source of C/E facts.

# 13. State and Lifecycle Impact
Completed Placement lifecycles remain unchanged; the H04 statistics projection becomes available for authorized use.

# 14. Business Events and Cross-Domain Collaboration
H04 uses existing completion facts and publishes `PlacementStatisticsGenerated`; H07 may later compose dashboards from this projection.

# 15. Business Authorization, Approval and Audit Obligations
Preserve reporting scope, source outcome references, excluded/incomplete items and authorization context.

# 16. Completion Criteria and Postconditions
H04-supported Placement statistics are verified and available for authorized institutional consumption.

# 17. Open Issues, Assumptions and Deferred Questions
No blocking issues. Additional diversity analytics require separate future architecture change control and are not assumed here.
