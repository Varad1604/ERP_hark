# Transport Request to Active Allocation

**Scenario ID:** L3-G-099  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** C, E, G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose
This scenario governs the complete business outcome in which a transport request is submitted and evaluated until Transport Management establishes an active transport allocation. G04 defines the allocation workflow as Application Submitted → Eligibility Validation → Route Selection → Approval → Transport Activated.

## 3. Business Outcome
A valid transport request passes the applicable eligibility and route-selection decisions, receives required approval, and results in an **Active** Transport Allocation.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
A student, staff member, or authorised guest has a legitimate requirement for institutional transport and submits a transport request.
### 4.2 End Boundary
The Transport Allocation reaches the canonical **Active** state.
### 4.3 In Scope
- Receive the transport request.
- Validate eligibility using configured G04 policy.
- Select an available route/stop context.
- Obtain required approval.
- Establish and activate the allocation.
- Preserve allocation history and audit evidence.
- Publish the canonical allocation event.
### 4.4 Out of Scope
- Vehicle ownership or maintenance execution.
- Student/workforce lifecycle ownership.
- Financial accounting.
- Notification delivery mechanics.
- APIs, databases, services, queues, UI, deployment or infrastructure.
- Separate downstream transport-pass, trip, financial or communication outcomes.

## 5. Ownership and Participating Domains
Transport Management (G04) is the canonical owner of the Transport Allocation outcome. Participation does not transfer ownership.

G04 defines Transport Allocation as owning Student Allocation, Staff Allocation, Guest Allocation, Allocation Status and Allocation History. Student information remains externally owned by Section C; workforce information remains externally owned by Section E. Finance consumes transport allocation but does not modify transport operations. Parent Communication consumes transport information. None of these relationships transfers allocation ownership.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Administrator | Manages transport allocation decisions. |
| Transport Manager / configured authority | Performs approval responsibility where required. |
| Student / Staff / authorised guest | Provides the transport requirement/request. |

## 7. Preconditions and Entry Criteria
- A legitimate transport requirement exists.
- Required applicant context is available from the authoritative domain.
- Applicable route/stop information exists for evaluation.
- Configured eligibility and approval policy can be applied.

## 8. Trigger
A transport request is submitted for institutional transport service.

## 9. Main Business Flow
1. **Receive and identify the request.** Establish the applicant and requested transport context.
2. **Validate eligibility.** Apply the configured transport allocation eligibility rules.
3. **Select route/stop.** Identify an active route and applicable stop context.
4. **Review and approve.** Complete the configured approval workflow.
5. **Establish the allocation.** Record the approved route/stop relationship.
6. **Activate transport.** Move the allocation to the canonical Active state.
7. **Preserve history.** Record the allocation history and audit evidence.
8. **Publish the business outcome.** Publish `TransportAllocated`.

## 10. Alternate Business Flows
- Eligibility is not satisfied: no Active Allocation is created.
- No suitable active route is available: allocation cannot be activated against an unavailable service.
- Approval is rejected: the allocation remains non-active.
- Required student/workforce information is unavailable: validation cannot complete until the authoritative information is available.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| Eligibility cannot be established | Allocation cannot become Active | Resolve the business information/policy condition |
| No active suitable route/stop | Allocation cannot be activated | Reassess route availability |
| Approval rejected | No Active Allocation | Close or revise according to configured workflow |
| Upstream applicant context unavailable | Validation cannot complete | Obtain authoritative upstream information |

No technical retries, queues, rollback or infrastructure recovery mechanics are introduced.

## 12. Business Rules and Decision Constraints
| Rule | Effect |
|---|---|
| Every Allocation shall reference an active Route. | An Active Allocation cannot reference an inactive route. |
| Validation rules remain configuration driven. | No local eligibility rule is invented. |
| Approval policies remain configurable. | No local approval authority is invented. |
| Allocation history is immutable and preserves route, stop, dates, change history and remarks. | Activation remains historically traceable. |

## 13. State and Lifecycle Impact
`Applied → Approved → Allocated → Active → Withdrawn`. The authoritative workflow is `Application Submitted → Eligibility Validation → Route Selection → Approval → Transport Activated`; workflow labels are not promoted into extra lifecycle states.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
**`TransportAllocated`** — transport allocation completed. **Canonical publisher: Transport Management (G04).** No additional activation event is invented.
### 14.2 Synchronous Business Collaboration / Hand-offs
Frozen participating domains are C, E and G. C provides authoritative student context where applicable; E provides workforce context where applicable; G04 owns the allocation. Downstream communication is outside this scenario boundary.

## 15. Business Authorization, Approval and Audit Obligations
G04 identifies Transport Administrator as a typical allocation authority, with configurable approval policy. Request, eligibility decision, selected route/stop, approval, activation and audit history must remain traceable.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Request received.
- Eligibility validated.
- Active route/stop selected.
- Approval completed.
- Allocation reaches **Active**.
- `TransportAllocated` is published by G04.
- History remains auditable.
### 16.2 Postconditions
- The named allocation outcome is authoritative within G04.
- Allocation history remains traceable.
- No unrelated domain lifecycle is implicitly completed.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material source conflict identified. Exact institutional eligibility criteria remain configuration-driven and are therefore not fabricated.
### 17.2 Assumptions
- G04 configuration-driven policies remain authoritative.
- No additional actor, rule, state, event, publisher, owner or dependency is invented.
### 17.3 Deferred Questions
Any material upstream ambiguity must be resolved through architecture governance before Architecture Approval/Lock.
