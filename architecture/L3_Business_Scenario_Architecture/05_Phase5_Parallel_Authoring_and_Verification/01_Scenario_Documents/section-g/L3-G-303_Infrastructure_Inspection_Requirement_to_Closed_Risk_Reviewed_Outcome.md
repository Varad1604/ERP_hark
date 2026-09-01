# Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-G-303  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L2-G10; L2-G01; L2-G00; `L2-G10_Infrastructure_Blocker_Remediation_Amendment_2026-09-01.md`; `L2-G00_Governance_Resolution_and_Maturity_Amendment_2026-09-01.md`; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

# 2. Scenario Purpose
Govern a G10 Infrastructure Inspection from requirement through closure while keeping Facilities operational inspection and maintenance execution separate.

# 3. Business Outcome
The G10 Inspection Record is completed and closed, Infrastructure condition is reconciled, and significant findings enter the appropriate maintenance or risk path.

# 4. Scenario Boundary and Scope
G10 Infrastructure Inspection concerns physical Infrastructure condition/compliance/criticality. G01 Facilities Inspection remains a separate operational aggregate.

# 5. Ownership and Participating Domains
G10 owns Infrastructure Inspection and physical Infrastructure facts. G01 owns Facilities Inspection and maintenance execution. G00 owns Operational Risk when a risk referral is accepted.

# 6. Business Actors, Roles and Responsibilities
Infrastructure Administrator establishes/schedules; Infrastructure Inspector conducts; Infrastructure Manager approves closure.

# 7. Preconditions and Entry Criteria
Canonical Infrastructure exists and an inspection is due/authorized.

# 8. Trigger
An Infrastructure inspection becomes due or is authorized.

# 9. Main Business Flow
1. Establish/schedule the G10 Inspection Record; publish `InfrastructureInspectionScheduled`.  
2. Conduct the inspection and record findings.  
3. Reconcile authoritative Infrastructure condition.  
4. For corrective-maintenance findings, hand the requirement to G01.  
5. For material risk-only findings, refer to G00; G00 alone may register Operational Risk.  
6. Complete/approve/close the G10 Inspection Record and publish `InfrastructureInspectionCompleted`.

# 10. Alternate Business Flows
No significant finding permits direct inspection closure after approval.

# 11. Failure, Interruption and Business Recovery
Unresolved material findings prevent closure until an appropriate governed follow-up exists.

# 12. Business Rules and Decision Constraints
G10 does not own G01 maintenance execution; G01 Facilities Inspection events do not substitute for G10 Infrastructure Inspection events.

# 13. State and Lifecycle Impact
G10 Inspection Record follows `Scheduled → In Progress → Completed → Approved → Closed`.

# 14. Business Events and Cross-Domain Collaboration
G10 publishes `InfrastructureInspectionScheduled`, `FindingRecorded`, `InfrastructureInspectionCompleted`; G01/G00 publish their own downstream facts.

# 15. Business Authorization, Approval and Audit Obligations
Preserve inspection authority, criteria, findings, condition changes, referrals and closure approval.

# 16. Completion Criteria and Postconditions
Inspection is Closed and every significant finding has a traceable source-supported follow-up.

# 17. Open Issues, Assumptions and Deferred Questions
No blocking issues. No assumptions.
