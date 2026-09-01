# Operational Maturity Assessment

# 1. Scenario Metadata

**Scenario ID:** L3-G-010  
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
Evaluate campus operational maturity through the G00-owned, versioned maturity framework while preserving source-domain evidence ownership.

# 3. Business Outcome
Campus operational maturity is evaluated and the resulting maturity assessment is recorded for governance and improvement planning.

# 4. Scenario Boundary and Scope
The scenario owns assessment governance, not operational source facts. The maturity framework is institutionally configurable and versioned.

# 5. Ownership and Participating Domains
G00 owns Operational Maturity Assessment, framework use, result and history. Source domains own contributing evidence.

# 6. Business Actors, Roles and Responsibilities
Governance Coordinator schedules/coordinates; Campus Operations Manager evaluates; Executive Reviewer confirms the final result and closes the assessment.

# 7. Preconditions and Entry Criteria
An active maturity framework version exists and required evidence can be resolved from canonical owners.

# 8. Trigger
An operational maturity assessment cycle is initiated.

# 9. Main Business Flow
1. Create the assessment in `Planned`.  
2. Enter `Evidence Collection` and assemble referenced evidence.  
3. Evaluate against the active framework and record the derived result; publish `OperationalMaturityEvaluated`.  
4. Executive Reviewer reviews/confirms the result.  
5. Close the assessment and publish `OperationalMaturityAssessmentClosed`.

# 10. Alternate Business Flows
Incomplete evidence keeps the assessment in Evidence Collection. A revised framework applies only through its governed effective version.

# 11. Failure, Interruption and Business Recovery
Do not manufacture evidence or maturity classifications outside the active framework.

# 12. Business Rules and Decision Constraints
Each assessment records framework version, criteria and evidence. Universal maturity-level names are not mandated by the architecture.

# 13. State and Lifecycle Impact
`Planned → Evidence Collection → Evaluated → Reviewed → Closed`.

# 14. Business Events and Cross-Domain Collaboration
G00 publishes `OperationalMaturityAssessmentScheduled`, `OperationalMaturityEvaluated`, and `OperationalMaturityAssessmentClosed`.

# 15. Business Authorization, Approval and Audit Obligations
Preserve framework version, evidence references, calculated result, Executive Reviewer decision and closure history.

# 16. Completion Criteria and Postconditions
The assessment is Closed with an authoritative, reproducible maturity result; source facts remain source-owned.

# 17. Open Issues, Assumptions and Deferred Questions
No blocking issues. Institutional framework content remains configuration, not repository-invented policy.
