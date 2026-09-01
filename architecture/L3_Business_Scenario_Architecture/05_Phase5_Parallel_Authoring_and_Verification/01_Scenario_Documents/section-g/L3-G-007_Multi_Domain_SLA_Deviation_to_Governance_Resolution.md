# Multi-Domain SLA Deviation to Governance Resolution

# 1. Scenario Metadata

**Scenario ID:** L3-G-007  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Campus Operations Governance  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L2-G00; `L2-G00_Governance_Resolution_and_Maturity_Amendment_2026-09-01.md`; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

# 2. Scenario Purpose
Govern a material SLA deviation using existing G00 review and corrective-action lifecycles without creating a separate SLA aggregate.

# 3. Business Outcome
The SLA deviation is reviewed, responsibility and corrective action are established where required, and governance resolution is completed.

# 4. Scenario Boundary and Scope
The SLA deviation remains a source-owned performance fact. G00 owns governance review, findings and action plans only.

# 5. Ownership and Participating Domains
G00 owns the governance matter; the publishing operational domain retains the underlying SLA fact.

# 6. Business Actors, Roles and Responsibilities
Campus Operations Manager conducts review; Governance Coordinator creates required Action Plans; Quality Assurance Officer verifies action completion.

# 7. Preconditions and Entry Criteria
A material source-owned SLA deviation and sufficient governance evidence exist.

# 8. Trigger
A significant SLA deviation is identified across campus operations.

# 9. Main Business Flow
1. Validate deviation and governance scope.  
2. Conduct the Operational Review and record findings.  
3. If corrective action is required, create and govern a Multi-Domain Action Plan.  
4. Verify required action completion.  
5. Close the Operational Review.  
6. Treat the SLA governance matter as complete only when the review is Closed and every required Action Plan is Verified/Closed. If no plan is required, review closure is sufficient.

# 10. Alternate Business Flows
Below-materiality deviations remain with the source owner. Material risk may be separately registered under the Operational Risk lifecycle.

# 11. Failure, Interruption and Business Recovery
Missing evidence or unverified required action prevents governance completion.

# 12. Business Rules and Decision Constraints
G00 never modifies operational source data and does not execute source-domain corrective work.

# 13. State and Lifecycle Impact
Operational Review follows `Planned → Active → Findings Recorded → Completed → Closed`; required Action Plans follow their existing lifecycle. No SLA lifecycle is created.

# 14. Business Events and Cross-Domain Collaboration
`GovernanceReviewClosed`, correlated to the SLA matter, is the canonical completion fact; Action Plan events remain authoritative when corrective work is required.

# 15. Business Authorization, Approval and Audit Obligations
Preserve SLA source evidence, findings, assigned responsibility, action evidence and review closure.

# 16. Completion Criteria and Postconditions
Review is Closed and every required Action Plan is verified/closed; source ownership is unchanged.

# 17. Open Issues, Assumptions and Deferred Questions
No blocking issues. No assumptions.
