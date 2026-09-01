# Preventive Maintenance Governance to Infrastructure Maintenance Requirement

# 1. Scenario Metadata

**Scenario ID:** L3-G-304  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L2-G10; L2-G01; `L2-G10_Infrastructure_Blocker_Remediation_Amendment_2026-09-01.md`; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

# 2. Scenario Purpose
Record a G10 preventive-maintenance requirement and hand it to G01 without duplicating G01 plans, schedules or execution ownership.

# 3. Business Outcome
The Infrastructure preventive-maintenance requirement is governed under G10 and operational maintenance remains owned by G01.

# 4. Scenario Boundary and Scope
G10 ends at the governed requirement and hand-off. G01 owns later Preventive Maintenance Plan, Maintenance Schedule, Work Order and execution.

# 5. Ownership and Participating Domains
G10 owns Infrastructure and the preventive-maintenance requirement; G01 owns operational maintenance planning/scheduling/execution.

# 6. Business Actors, Roles and Responsibilities
Maintenance Coordinator / Infrastructure Administrator records the G10 requirement; Facilities roles govern later G01 work.

# 7. Preconditions and Entry Criteria
Canonical Infrastructure exists and condition/policy establishes a preventive need.

# 8. Trigger
Infrastructure condition or policy identifies a preventive maintenance requirement.

# 9. Main Business Flow
1. Validate the preventive-maintenance need and affected Infrastructure.  
2. Record the G10 requirement.  
3. Publish `InfrastructurePreventiveMaintenanceRequired`.  
4. Hand the requirement to G01 for its own planning/scheduling/execution lifecycle.

# 10. Alternate Business Flows
Existing defect uses corrective maintenance; urgent condition uses emergency maintenance.

# 11. Failure, Interruption and Business Recovery
If requirement classification is invalid, no G10 requirement is published.

# 12. Business Rules and Decision Constraints
G10 never creates the G01 Preventive Maintenance Plan or Maintenance Schedule.

# 13. State and Lifecycle Impact
Physical Infrastructure lifecycle is unchanged solely by recording the requirement; G01 later governs its own aggregates.

# 14. Business Events and Cross-Domain Collaboration
G10 publishes `InfrastructurePreventiveMaintenanceRequired`. G01 alone publishes `PreventiveMaintenanceScheduled` for its schedule fact.

# 15. Business Authorization, Approval and Audit Obligations
Preserve requirement basis, affected Infrastructure and hand-off evidence.

# 16. Completion Criteria and Postconditions
The G10 requirement exists and the G01 hand-off is traceable; execution ownership remains G01.

# 17. Open Issues, Assumptions and Deferred Questions
No blocking issues. No assumptions.
