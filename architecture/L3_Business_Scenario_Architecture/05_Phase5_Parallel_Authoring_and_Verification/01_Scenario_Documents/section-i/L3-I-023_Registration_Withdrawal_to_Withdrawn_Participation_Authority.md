# Registration Withdrawal to Withdrawn Participation Authority

**Scenario ID:** L3-I-023  
**Participating Domains:** C, I  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2-I04 Examination Registration Workflows; applicable Student Lifecycle architecture  
**Referenced By:** Downstream examination workflows consuming Examination Registration; None yet

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-023 |
| Canonical Title | Registration Withdrawal to Withdrawn Participation Authority |
| Primary Owner | Examination Registration |
| Participating Domains | C, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the controlled withdrawal of an Examination Registration or registered subject where institutional policy permits withdrawal, establishing the withdrawn participation position while preserving registration ownership and history.

## 3. Business Outcome

The permitted withdrawal is completed as an authoritative Examination Registration outcome, the affected examination participation is withdrawn from downstream examination processing, and the withdrawal remains traceable.

## 4. Scenario Boundary and Scope

**Start:** An existing Examination Registration or registered subject is identified for permitted withdrawal.  
**In scope:** withdrawal eligibility, deadline/policy validation, candidate or authorised institutional initiation, required review/approval, authoritative withdrawal, audit, and the canonical withdrawal event.  
**Out of scope:** Student lifecycle withdrawal, examination-policy change, Finance transaction/refund processing, hall-ticket mechanics, and technical implementation.  
**End:** The registration reflects the approved withdrawal and withdrawn participation is no longer an eligible downstream registration input.

## 5. Ownership and Participating Domains

**Examination Registration (I):** owns the registration and withdrawal outcome.  
**Student Lifecycle (C):** provides candidate/academic context and retains ownership of student information. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

**Candidate (Student):** may request withdrawal where policy permits.  
**Examination Administrator:** manages withdrawal operations.  
**Controller of Examination:** governs registration policy/approval where required.  
**Department Administration:** verifies departmental requirements where applicable.

## 7. Preconditions and Entry Criteria

A valid registration exists; candidate and examination context are identifiable; applicable policy and withdrawal deadline/exception can be established; required authority is available; withdrawal is not otherwise prohibited.

## 8. Trigger

A candidate or authorised institutional actor requests withdrawal of an existing Examination Registration or registered subject.

## 9. Main Business Flow

1. Identify the authoritative registration and affected scope.  
2. Establish candidate, session, category and subject context.  
3. Evaluate withdrawal eligibility, policy and deadline.  
4. Obtain required institutional review/approval.  
5. Establish the withdrawal.  
6. Preserve audit history.  
7. Publish **Registration Withdrawn**.  
8. Downstream examination workflows use the withdrawn position.

## 10. Alternate Business Flows

**Subject-level withdrawal:** withdraw only permitted subjects while retaining the remaining registration scope.  
**Full registration withdrawal:** withdraw the complete applicable participation.  
**Administrative withdrawal:** authorised administration performs the same governed process.  
**Withdrawal after approval:** permitted approved registrations may be withdrawn without erasing prior approval history.

## 11. Failure, Interruption and Business Recovery

**Not permitted:** reject and retain the authoritative registration.  
**Approval not granted:** do not establish withdrawal.  
**Context cannot be verified:** resolve the business discrepancy before completion.  
**Later restoration:** use the separate governed reinstatement scenario rather than silently reversing withdrawal.

## 12. Business Rules and Decision Constraints

Withdrawal follows institutional policy and applicable deadlines; may be subject-level or complete-registration according to policy; withdrawn registrations do not participate downstream unless subsequently reinstated through an approved process; complete audit history is required; Student and Finance ownership remains external to Examination Registration.

## 13. State and Lifecycle Impact

Uses the withdrawal capability defined by L2-I04. No new lifecycle state is invented. A withdrawn participation is no longer an eligible downstream registration input unless restored through an approved process.

## 14. Business Events and Cross-Domain Collaboration

**Registration Withdrawn** is the canonical event published by Examination Registration. Student Lifecycle provides candidate/academic context. No new withdrawal event is introduced.

## 15. Business Authorization, Approval and Audit Obligations

Withdrawal complies with policy; required examination authority approval is obtained where applicable; administrative action is attributable to the authorised process; significant withdrawal activity is auditable. No technical authorization model is defined.

## 16. Completion Criteria and Postconditions

**Complete when:** policy/deadline checks pass, required approval exists, authoritative withdrawal is established, audit history is preserved, and **Registration Withdrawn** is published. **Afterward:** withdrawn participation is excluded downstream unless reinstated; unaffected scope remains governed.

## 17. Open Issues, Assumptions and Deferred Questions

L2-I04 defines withdrawal and reinstatement but no separate reinstatement event. Exact approval authority is policy-dependent. Financial consequences remain Finance-owned.

