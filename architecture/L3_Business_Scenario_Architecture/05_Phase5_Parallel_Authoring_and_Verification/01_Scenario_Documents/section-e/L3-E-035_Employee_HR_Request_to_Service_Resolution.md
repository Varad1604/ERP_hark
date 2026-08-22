# Employee HR Request to Service Resolution

**Scenario ID:** L3-E-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** E  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E04 — HR Workflows; applicable Employee Profile and organisational context  
**Referenced By:** None yet.

## 1. Scenario Metadata

This scenario preserves the frozen Stage 3 canonical title, Primary Owner, Participating Domains, Scenario Type and Priority. It applies the locked upstream architecture without changing ownership, rules, lifecycle, events, contracts or dependency direction.

## 2. Scenario Purpose

Resolve an employee-initiated HR service request through the authoritative HR case workflow, producing a recorded business resolution and closure.

## 3. Business Outcome

The employee HR request is investigated and resolved through the HR case workflow, with the resolution recorded and the case closed.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An employee or manager submits an HR service request that falls within HR case management scope.

### 4.2 End Boundary

The HR case reaches the Closed outcome with a recorded resolution.

### 4.3 In Scope

- HR request intake.
- Case classification.
- Assignment to an HR owner.
- Investigation and resolution.
- Case closure and history preservation.

### 4.4 Out of Scope

The scenario does not own or redefine business data, lifecycle, rules or decisions belonging to another canonical domain. Technical implementation concerns are outside L3.

### 4.5 Related Scenarios

None yet.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| HR | Owns the final HR business-case outcome defined by this scenario. |
| Participating People & Workforce domain(s) | Provides the authoritative domain-owned business decision or fact required by the HR case. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

- Employee.
- Manager where the request is manager-initiated.
- HR Administrator / HR case owner.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- The requester is identifiable.
- The request falls within HR case-management scope.
- The information required for investigation is available or can be obtained through the governed case process.

## 8. Trigger

An employee or manager HR request is received by HR Workflows.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create and classify the HR case for the employee request. | HR | A governed HR case exists. |
| 2 | Assign the case to the appropriate HR business owner. | HR | Responsibility for the case is established. |
| 3 | Investigate the request and establish the applicable business facts. | HR | The case is ready for a resolution decision. |
| 4 | Determine and record the resolution. | HR | A business resolution is established. |
| 5 | Close the HR case with its resolution and history preserved. | HR | The employee HR request is resolved and the case is closed. |

The Main Business Flow stops once the Business Outcome and End Boundary are achieved.

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required information is insufficient | The case cannot be resolved correctly. | Keep the case in the applicable investigation state until the required business information is available. | HR |
| The request cannot be granted | The requested service outcome is not available. | Record the applicable resolution/rejection and close the case according to HR case governance. | HR |

No technical retry, queue, rollback or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

- HR Workflows owns HR operational cases, including employee requests.
- Every workflow has a defined lifecycle.
- Every approval records actor, timestamp and decision.
- HR maintains business process history.
- HR does not become owner of employee master data or other specialised domain data merely because that data is considered during the request.

The scenario applies these upstream-owned rules; it does not become their owner.

## 13. State and Lifecycle Impact

The HR Case lifecycle is the applicable lifecycle. The locked L2 workflow describes the business path as Request Created → Case Classification → Assign HR Owner → Investigation → Resolution Decision → Case Closed.

No lifecycle state is invented or renamed.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

No scenario-specific HR-request domain event is identified in the authoritative L2 material reviewed for this scenario. Do not invent one.

### 14.2 Synchronous Business Contracts / Hand-offs

Internal HR case progression is owned by HR. If the resolution requires another canonical domain's business state or fact, the HR case coordinates with that domain without acquiring ownership.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

HR case handling follows the business approval and audit requirements defined by L2-E04. The resolution records the responsible business actor and decision evidence where applicable; no technical access-control design is introduced.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Request is classified and assigned.
- Required investigation is completed.
- A business resolution is recorded.
- HR case is Closed.
- Case history remains traceable.

### 16.2 Postconditions

The completed HR case outcome remains historically traceable. Any domain-owned state or decision remains authoritative in its canonical owning domain.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified from the authoritative sources reviewed for this scenario.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.

**Authoring Classification:** PASS — source-grounded Draft.
