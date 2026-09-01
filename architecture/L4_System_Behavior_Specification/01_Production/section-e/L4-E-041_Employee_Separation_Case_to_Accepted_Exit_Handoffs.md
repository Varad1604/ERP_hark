# L4-E-041 — Employee Separation Case to Accepted Exit Handoffs

**Scenario ID:** L4-E-041  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-041 — Employee Separation Case to Accepted Exit Handoffs  
**Primary Owner:** HR  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-041. HR owns the separation case; Workforce & Engagement owns the authoritative employment closure.

## 2. System Behavior Purpose
Specify observable behavior for coordinating an employee separation through HR approval, clearance, verification, and accepted handoff to Workforce & Engagement.

## 3. System Behavior Outcome
The HR separation case reaches an accepted exit handoff with required approval, clearance, and verification evidence recorded, without duplicating employment closure ownership.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
A resignation or separation request enters the HR separation workflow.
### 4.2 End Boundary
Exit approval, clearance, and final verification are complete and the accepted separation handoff to Workforce & Engagement is recorded.
### 4.3 In Scope
- Establish separation case.
- Obtain exit approval.
- Complete clearance activities.
- Perform final verification.
- Hand accepted separation to Workforce & Engagement.
- Record HR case outcome and evidence.
### 4.4 Out of Scope
Authoritative employment closure, employee master/profile ownership, position assignment, payroll settlement calculation, and technical implementation.

## 5. Ownership and Participating Domains
HR owns the separation case and case outcome. Workforce & Engagement receives the accepted handoff and owns the authoritative employment lifecycle fact.

## 6. Business Actors, Roles and Responsibilities
Employee initiates or is subject to separation; HR case owner manages the separation workflow; approving authority establishes exit approval; Workforce & Engagement establishes authoritative employment closure.

## 7. Preconditions and Entry Criteria
- Workforce engagement is identifiable.
- Separation request exists.
- Required approval and clearance responsibilities are determinable.

## 8. Trigger
A resignation or separation request is submitted.

## 9. Main System Behavior
1. Establish the separation case.
2. Resolve the applicable exit approval requirement.
3. Record the exit approval decision.
4. Complete and record required clearance outcomes.
5. Perform final verification.
6. Hand the cleared separation to Workforce & Engagement for authoritative closure.
7. Record the accepted HR case outcome and evidence.

## 10. Alternate System Behaviors
- Rejected exit approval prevents accepted handoff.
- Incomplete clearance prevents final verification.
- Pending workforce closure does not allow HR to claim authoritative employment closure.

## 11. Failure, Interruption and Business Recovery
If approval or clearance is incomplete, the HR case remains unresolved until the applicable business outcome is established. If the workforce closure is pending, the accepted handoff is preserved while authoritative employment closure remains with Workforce & Engagement. No technical recovery mechanics are defined.

## 12. Business Rules and Decision Constraints
- HR owns the separation case.
- Employment closure is owned by Workforce & Engagement.
- The authoritative employment-separation event is published by Workforce & Engagement.
- Approval decisions are attributable and traceable.
- Cross-domain updates use governed business handoffs/events.

## 13. State and Lifecycle Impact
The HR separation case advances through its authoritative case lifecycle toward accepted exit handoff. No new HR employment-closure state is introduced.

## 14. Business Events and Cross-Domain Collaboration
`EmployeeSeparated` remains the authoritative employment event published by Workforce & Engagement. HR may publish its own case outcome event where governed, but it must not represent that event as authoritative employment separation.

## 15. Business Authorization, Approval and Audit Obligations
Exit approval, clearance evidence, final verification, accepted handoff, and HR case history remain attributable and auditable. Technical authorization mechanisms are outside L4.

## 16. Completion Criteria and Postconditions
- Exit approval recorded.
- Required clearance complete.
- Final verification complete.
- Accepted handoff recorded with Workforce & Engagement.
- HR case outcome recorded without duplicating employment closure.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary, lifecycle, or outcome was identified in the locked L3 parent.

**Authoring Classification:** PASS — source-grounded Draft.
