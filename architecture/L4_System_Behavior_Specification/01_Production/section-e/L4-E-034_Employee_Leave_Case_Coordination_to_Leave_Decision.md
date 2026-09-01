# Employee Leave Case Coordination to Leave Decision

## 1. Specification Metadata
**Specification ID:** L4-E-034  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** HR  
**Realizes Scenario:** L3-E-034  
**Primary Business Owner:** HR  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-034 — Employee Leave Case Coordination to Leave Decision; authoritative HR and leave workflows

## 2. Behavioral Objective & Boundary
SutraERP shall coordinate an HR leave case through the information, review and handoff necessary to reach an authoritative leave decision, without assuming ownership of the canonical Leave Request or its decision where that ownership resides upstream.

## 3. Required Business Context
An HR case requires sufficient employee, employment and leave context. HR owns case coordination; the authoritative leave decision remains with the canonical owner identified by upstream architecture.

## 4. Behavioural Specification
1. Identify the employee and associated HR leave case.
2. Validate required leave information and applicable policy context.
3. Coordinate the request with the authoritative leave-decision responsibility.
4. Evaluate returned decision information and required business consequences.
5. Record the decision against the HR case.
6. Resolve missing information or rejected coordination through the governed correction path.
7. Complete the case when the authoritative leave decision is available and required HR actions are recorded.
8. Preserve decision history and audit evidence.

## 5. Alternate and Exceptional Behaviour
Incomplete information prevents final case completion. A decision that is not yet available leaves the case open or pending according to the canonical workflow. HR does not override the authoritative Leave Request/decision owner.

## 6. Business Rules
- HR coordinates the case but does not acquire ownership of the canonical Leave Request or decision.
- Only an authoritative leave decision may close the decision-dependent portion of the case.
- Rejected/incomplete information follows governed correction.
- Historical decisions remain traceable.

## 7. Events and Collaboration
Use only canonical leave and HR business events/contracts defined upstream. No undefined event name is introduced. Cross-domain collaboration preserves canonical ownership.

## 8. Authorization, Audit & Traceability
Leave decisions and HR case actions follow applicable institutional policy. Decision actor, reason and relevant case history remain auditable where required by the upstream model.

## 9. Completion & Validation
The specification completes when the authoritative leave decision is recorded, required HR coordination is complete, and the case outcome is historically traceable.

## 10. Out of Scope
APIs, databases, implementation services, UI, IAM implementation, infrastructure, queues and technical retry/rollback behaviour.