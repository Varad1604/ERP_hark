# Employee Grievance to Investigated Resolution

**Scenario ID:** L3-E-036  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** E  
**Scenario Type:** Critical  
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

Provide a governed HR grievance process that takes an employee concern through investigation to an authoritative, recorded resolution while preserving case history and evidence.

## 3. Business Outcome

The grievance is investigated through the HR case workflow and reaches a recorded resolution with the HR case closed and its history preserved.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An employee grievance is formally received within HR case-management scope.

### 4.2 End Boundary

The grievance case reaches its governed resolution and closure after investigation.

### 4.3 In Scope

- Grievance intake and case creation.
- Case classification and assignment.
- Investigation.
- Resolution decision.
- Closure and preservation of case history.

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

- Employee / grievance submitter.
- HR Administrator / HR case owner.
- Other business participants only where supported by the governed grievance investigation; no unsupported role is named.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- A grievance has been received.
- The grievance falls within HR case-management scope.
- The case can be classified and assigned for investigation.

## 8. Trigger

An employee grievance is received and requires formal HR investigation.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create and classify the grievance as an HR case. | HR | A governed grievance case is established. |
| 2 | Assign the grievance to the responsible HR case owner. | HR | Investigation responsibility is established. |
| 3 | Investigate the grievance and collect the business evidence required for the decision. | HR | The grievance is sufficiently investigated for resolution. |
| 4 | Determine and record the grievance resolution. | HR | An authoritative business resolution is established. |
| 5 | Close the grievance case while preserving the case history. | HR | The grievance reaches its investigated resolution and closure. |

The Main Business Flow stops once the Business Outcome and End Boundary are achieved.

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Investigation cannot yet establish the necessary business facts | A final resolution cannot responsibly be reached. | Continue the governed investigation until sufficient evidence exists or another authorised case outcome is established. | HR |
| The grievance is not substantiated or cannot be granted | The requested grievance outcome is not established. | Record the applicable resolution and close the case according to HR case governance. | HR |

No technical retry, queue, rollback or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

- Employee grievances are explicitly within HR Case Management scope.
- HR owns the HR case, including intake, investigation, resolution tracking and closure.
- Every workflow has a defined lifecycle.
- Every approval records actor, timestamp and decision.
- HR maintains business process history.
- Evidence and case history are part of the HR case responsibility.

The scenario applies these upstream-owned rules; it does not become their owner.

## 13. State and Lifecycle Impact

The HR Case lifecycle applies: Request Created → Case Classification → Assign HR Owner → Investigation → Resolution Decision → Case Closed.

No lifecycle state is invented or renamed.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

No scenario-specific grievance event name is identified in the authoritative L2 material reviewed for this scenario. Do not invent one.

### 14.2 Synchronous Business Contracts / Hand-offs

Grievance investigation and resolution remain within HR case ownership. If a resolution requires a decision or state owned by another domain, that domain remains authoritative and HR records the case outcome rather than duplicating the external state.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Grievance handling requires traceable case history and decision evidence. Any approval or decision follows the applicable HR business governance; technical access control/IAM is outside L3.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Grievance is formally classified and assigned.
- Investigation is completed to the extent required for a governed resolution.
- Resolution is recorded.
- HR case is Closed.
- Grievance history and evidence remain traceable.

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
