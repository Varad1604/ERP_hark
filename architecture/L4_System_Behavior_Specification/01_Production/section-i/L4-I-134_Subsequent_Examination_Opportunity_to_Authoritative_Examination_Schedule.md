# L4-I-134 — Subsequent Examination Opportunity to Authoritative Examination Schedule

## 1. Specification Metadata
- Specification ID: L4-I-134
- Architecture Level: L4 — System Behavior Specification
- Realizes: L3-I-134 — Subsequent Examination Opportunity to Authoritative Examination Schedule
- Primary Owner: Supplementary, Backlog & Improvement Examination
- Participating Domains: D — Academic Delivery; I — Examination
- Scenario Type: Cross-domain; Priority: P1
- Status: Draft; Version: 1.0

## 2. Behavioral Objective & Boundary
Govern establishment of an authoritative examination schedule for an established subsequent-examination opportunity. Begin with an eligible opportunity requiring scheduling and end with an approved/published schedule or a governed non-scheduling outcome.

Out of scope: owning the Examination Schedule lifecycle, candidate registration, examination conduct, or technical scheduling implementation.

## 3. Required Business Context
- Subsequent examination opportunity is authoritative.
- Applicable academic-delivery/calendar context is available.
- Examination Scheduling owns the schedule lifecycle.
- Required examination planning constraints are known.

## 4. Required System Behaviors
1. Identify the subsequent-examination opportunity requiring a schedule.
2. Establish the scheduling requirements and applicable examination window.
3. Hand scheduling responsibility to the canonical Examination Scheduling capability.
4. Validate the returned schedule against the opportunity's authoritative scope.
5. Recognize an approved/published schedule as the authoritative scheduling outcome.
6. Preserve the relationship between opportunity and resulting schedule.
7. Prevent local creation of competing schedule ownership or lifecycle states.

## 5. Cross-Domain Behavior
Academic Delivery provides authoritative academic context. Examination Scheduling owns preparation, approval and publication of the Examination Schedule. Subsequent Examination administration coordinates the requirement but does not take schedule ownership.

## 6. Failure / Rejection / Recovery
An opportunity without sufficient authoritative context shall not be scheduled. An invalid or rejected schedule remains non-authoritative. Material conflicts follow the existing scheduling governance rather than local exception logic.

## 7. Audit & Evidence
Preserve opportunity identity, scheduling requirement, governing academic context, schedule decision, approval/publication evidence and superseded scheduling context where applicable.

## 8. Observable Completion
A valid authoritative schedule is associated with the subsequent-examination opportunity, or a governed non-scheduling outcome is recorded.

## 9. Traceability
Parent: L3-I-134 — Subsequent Examination Opportunity to Authoritative Examination Schedule.

## 10. Deferred Design
Exact scheduling algorithms, resource allocation, interfaces and storage remain outside L4.