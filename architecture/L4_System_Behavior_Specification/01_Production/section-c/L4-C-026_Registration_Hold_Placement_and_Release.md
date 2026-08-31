# L4-C-026 — Registration Hold Placement and Release

**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** `L3-C-026` — Registration Hold Placement and Release  
**Primary Owner:** Academic Registration  
**Participating Domains:** C, F  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad

## 1. Purpose
Specify behavior for placing or releasing a Registration Hold that affects registration eligibility while preserving source-domain ownership of the underlying condition.

## 2. Preconditions and Trigger
Placement begins when an authoritative business condition requires registration restriction. Release begins when the authoritative condition has been resolved and release is permitted.

## 3. Main Behavior — Placement
1. Receive/establish the authoritative business condition.
2. Confirm applicability to the Student/registration context.
3. Validate that institutional policy permits the restriction.
4. Record the Registration Hold within the Academic Registration ownership boundary.
5. Make the hold available to registration eligibility evaluation.
6. Audit the hold placement.

## 4. Main Behavior — Release
1. Identify the existing hold.
2. Validate the authoritative release basis.
3. Confirm required authority.
4. Release the hold.
5. Make the changed hold condition available for subsequent eligibility evaluation.
6. Audit the release.

## 5. Cross-Domain Ownership
Student Lifecycle and Finance may provide authoritative conditions; they retain ownership of those conditions. Academic Registration owns the Registration Hold and its effect on registration eligibility.

## 6. Lifecycle
`No Applicable Hold → Hold Placed → Hold Released`. This represents the Registration Hold lifecycle only; it does not create or modify an Academic Registration Aggregate lifecycle state.

## 7. Events
The locked L3/L2 source does **not** define canonical `RegistrationHoldPlaced` or `RegistrationHoldReleased` events. Therefore this specification deliberately introduces neither. The resulting hold condition participates in the established registration eligibility behavior.

## 8. Authorization and Audit
Placement/release must satisfy institutional authorization. Evidence includes the underlying business basis, affected registration context, action, authority where required, resulting hold status and release basis where applicable.

## 9. Business Failure Handling
Invalid source evidence, unsatisfied release condition, absent authority or disputed source condition prevents the corresponding action. Resolution occurs through the authoritative owning domain/process.

## 10. Traceability
L3-C-026; L2-C03 Student Academic Registration Workflows; participating domains C and F.

## 11. Open Issues
Exact hold categories, source-specific triggers, severity/precedence and canonical hold-specific events remain unresolved upstream. L4 does not invent them.

## 12. Implementation Boundary
No API, database, queue, retry, service, IAM or infrastructure behavior is defined.

**Classification: PASS WITH ISSUES — Draft**
