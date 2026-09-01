# Infrastructure Recovery Condition to Controlled Recovery State

# 1. Scenario Metadata

**Scenario ID:** L3-G-313  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A, B  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L2-G10; L2-G00; L2-B03; `L2-G10_Infrastructure_Blocker_Remediation_Amendment_2026-09-01.md`; `L2-G00_Governance_Resolution_and_Maturity_Amendment_2026-09-01.md`; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

# 2. Scenario Purpose
Govern physical-Infrastructure recovery after disaster/major disruption without creating a new Disaster Management bounded context.

# 3. Business Outcome
Infrastructure recovery status is recorded and affected resources follow authorized existing G10 recovery/lifecycle paths.

# 4. Scenario Boundary and Scope
G10 owns physical Infrastructure state/recovery metadata. G00 owns cross-campus recovery governance when the disruption meets the configured governance threshold. B03 retains InstitutionalCampus context.

# 5. Ownership and Participating Domains
G10: Infrastructure facts/lifecycle. G00: cross-campus governance. B03: InstitutionalCampus identity/context. Section A: authorization/audit capability only.

# 6. Business Actors, Roles and Responsibilities
Infrastructure Administrator/Manager assess and govern G10 facts; G00 governance roles coordinate material cross-campus governance.

# 7. Preconditions and Entry Criteria
Affected Infrastructure exists and a disaster/major disruption has created a recovery condition.

# 8. Trigger
A disaster or major Infrastructure disruption affects physical resources.

# 9. Main Business Flow
1. Identify affected Infrastructure and current authoritative condition/availability/lifecycle.  
2. Record/update G10 recovery metadata.  
3. If temporary closure/restoration is appropriate, use existing G10 permitted transitions and events.  
4. If unrecoverable, use the separately governed decommissioning path.  
5. If the disruption crosses the G00 governance threshold, establish cross-campus governance while G10 retains physical state ownership.  
6. Preserve B03 InstitutionalCampus context without ownership transfer.

# 10. Alternate Business Flows
Localized recoverable disruption remains G10/operational-owner scoped; institution-wide disruption invokes G00 governance.

# 11. Failure, Interruption and Business Recovery
No generic `Recovered` state may be invented; unsupported requested transitions are rejected.

# 12. Business Rules and Decision Constraints
Only existing G10 lifecycle/availability transitions may change physical Infrastructure state.

# 13. State and Lifecycle Impact
Applicable existing transitions such as temporary closure/restoration or decommissioning are used; recovery metadata records status independently.

# 14. Business Events and Cross-Domain Collaboration
G10 publishes the canonical event for the actual lifecycle/availability transition used; G00 publishes its governance facts when involved.

# 15. Business Authorization, Approval and Audit Obligations
Preserve affected scope, recovery facts, state decisions, governance threshold result and cross-domain hand-offs.

# 16. Completion Criteria and Postconditions
Recovery status is recorded and each affected resource is in an authorized G10 lifecycle/availability position with ownership preserved.

# 17. Open Issues, Assumptions and Deferred Questions
No blocking issues. No new Disaster Management domain is assumed.
