# Employee Leave Case Coordination to Leave Decision

## 1. Specification Metadata

**Specification ID:** L4-E-034  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-034 — Employee Leave Case Coordination to Leave Decision  
**Primary Business Owner:** HR  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-034; L2-E04; L2-E05; applicable employee/organisation context  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall coordinate an employee leave-related HR case with authoritative Leave Management so that HR records the resulting leave decision and reaches its governed HR case outcome without taking ownership of the Leave Request, leave approval, leave policy, leave balance, Leave Ledger, or leave lifecycle state.

## 3. Required Business Context

- An employee context is available.
- A leave-related matter requires HR case coordination.
- Leave Management is authoritative for the Leave Request, leave policy, eligibility, approval and leave lifecycle.
- Relevant cross-domain facts remain source-owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-034-BH-001 | The system shall establish the HR case and identify the employee leave/absence matter requiring coordination. | L3-E-034 §§4.1, 7, 8, 9 |
| L4-E-034-BH-002 | The system shall hand the leave matter to authoritative Leave Management for leave lifecycle and decision processing without transferring ownership to HR. | L3-E-034 §§3, 4.3, 9, 12, 14.2 |
| L4-E-034-BH-003 | The system shall preserve Leave Management authority to evaluate leave policy, eligibility, balance and approval conditions and to establish the leave decision. | L3-E-034 §§9, 12, 15 |
| L4-E-034-BH-004 | When Leave Management establishes an authoritative leave outcome, the system shall make that outcome available to the HR case for coordination without transferring Leave Request ownership. | L3-E-034 §§3, 9, 14.1–14.2 |
| L4-E-034-BH-005 | The system shall record the authoritative leave decision against the HR case as a domain-owned outcome reference, not as an HR-owned leave lifecycle state. | L3-E-034 §§4.2, 9, 12, 13 |
| L4-E-034-BH-006 | If Leave Management rejects the requested leave outcome, the system shall record that authoritative rejection in the HR case and apply the applicable HR case resolution. | L3-E-034 §§10, 11 |
| L4-E-034-BH-007 | If required leave information or eligibility is not satisfied, the HR case shall not infer approval; Leave Management shall retain the Leave Request in its governed lifecycle until correction or the applicable terminal outcome. | L3-E-034 §§11, 12, 13 |
| L4-E-034-BH-008 | The system shall complete the HR case only after the authoritative leave decision has been recorded and the corresponding HR case outcome has been established. | L3-E-034 §§4.2, 9, 16.1 |
| L4-E-034-BH-009 | The system shall preserve attributable history of HR coordination and the authoritative leave outcome while leaving domain-owned decision evidence with Leave Management. | L3-E-034 §§15, 16.1–16.2 |

## 5. Cross-Domain & Event Behavior

The material synchronous business collaboration is HR → Leave Management for authoritative leave processing and Leave Management → HR for the resulting business outcome. `LeaveApproved`, where applicable to the authoritative decision, is published by Leave Management and may be consumed by HR; HR shall not republish it as an HR event or acquire leave ownership. No additional generic HR completion event is asserted by this specification.

## 6. Failure / Rejection / Continuation

- A rejected leave decision shall remain a Leave Management decision; HR shall record the rejection and apply the applicable HR case resolution.
- If required leave information or eligibility is not satisfied, Leave Management shall retain the Leave Request in its governed lifecycle until correction or the applicable terminal outcome; HR shall not infer approval.
- The HR case shall remain incomplete until an authoritative leave decision needed for the case is available.
- Technical retry, database rollback, queue recovery, and infrastructure mechanisms are excluded.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of HR case initiation, the hand-off to Leave Management, the authoritative leave outcome received, the HR case resolution based on that outcome, and the completed HR business-case history. Approval evidence remains governed by the authoritative Leave Management workflow.

## 8. Observable Completion Conditions

Completion requires the HR case to contain the authoritative leave decision, Leave Management ownership of the Leave Request and leave lifecycle to remain unchanged, the HR case to reach its governed outcome, and required business history to remain traceable.

## 9. Traceability & Provenance

**Parent:** L3-E-034 — Employee Leave Case Coordination to Leave Decision  
**Primary verification sources:** L2-E04 HR Workflows; L2-E05 Leave Management Workflows; applicable employee/organisation context.

## 10. Open Issues / Deferred Design

Institution-specific HR case handling and configurable leave policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, technical authorization mechanisms, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
