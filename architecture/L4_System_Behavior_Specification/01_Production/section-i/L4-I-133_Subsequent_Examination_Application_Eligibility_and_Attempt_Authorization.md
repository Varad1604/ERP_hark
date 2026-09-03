# Subsequent Examination Application, Eligibility and Attempt Authorization

## 1. Specification Metadata

**Specification ID:** L4-I-133  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-133 — Subsequent Examination Application, Eligibility and Attempt Authorization  
**Primary Business Owner:** Supplementary, Backlog & Improvement Examination  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-133; L2-I12 Supplementary / Backlog / Improvement Examination Execution; L2-I04 Examination Registration; applicable Student Lifecycle, result, Finance, and examination-policy architecture  
**Referenced By:** Applicable downstream Examination workflows

## 2. Behavioral Objective & Boundary

Govern a candidate's subsequent-examination application, eligibility determination, and attempt authorization under I12. The behavior begins when a supplementary, backlog, or improvement opportunity is available and a candidate applies, and ends when an authorised I12 attempt exists with the required I04 registration initiated, or when no attempt is authorised. The locked I-RC-03 ownership resolution is controlling: I12 owns subsequent-examination administration and attempt lifecycle; I04 exclusively owns Examination Registration and its lifecycle.

## 3. Required Business Context

- Authoritative candidate/student and academic context.
- Published examination result relevant to subsequent-examination eligibility.
- Available supplementary, backlog, or improvement opportunity and governing policy.
- Prior-attempt history and applicable attempt limits.
- Finance-owned eligibility/status where required.
- Examination authority for required approval.
- I04 capability for canonical Examination Registration initiation.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-133-BH-001 | Identify the candidate, requested subsequent-examination opportunity, examination scope, and application. | L3-I-133 §§3,7–9 |
| L4-I-133-BH-002 | Consume authoritative academic context and published-result information without redefining Student Lifecycle or result ownership. | L3-I-133 §§5,7,9,12 |
| L4-I-133-BH-003 | Determine eligibility under the applicable supplementary, backlog, or improvement policy and verify prior attempts and permitted attempt limits. | L3-I-133 §§9,11–13 |
| L4-I-133-BH-004 | Consume Finance-owned eligibility/status where applicable and prevent bypass of an unsatisfied authoritative Finance condition. | L3-I-133 §§5,9–12,15 |
| L4-I-133-BH-005 | Validate the application and obtain the required examination approval before authorising the governed subsequent-examination attempt. | L3-I-133 §§9,12,15–16 |
| L4-I-133-BH-006 | Initiate the required Examination Registration through I04; I12 shall not create, own, or redefine the Examination Registration aggregate or lifecycle. | L3-I-133 §§3,5,9,12–13,16 |
| L4-I-133-BH-007 | Establish the attempt within the existing I12 lifecycle and preserve complete application, approval, registration-initiation, and attempt history. | L3-I-133 §§9,13,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-133-BH-008 | When I12 eligibility is authoritatively established, the applicable I12 lifecycle fact such as `Supplementary Eligibility Confirmed` shall be published according to the locked I12 workflow. Subsequent I12 facts, including `Supplementary Registration Completed`, shall be published only when their corresponding I12 milestone actually occurs. | L3-I-133 §§14,16; L2-I12 |
| L4-I-133-BH-009 | I04 remains the canonical publisher of Examination Registration lifecycle events. No `AttemptAuthorized` event is invented and no I12 event may replace an I04 registration event. | L3-I-133 §§5,12–14; L2-I04; L2-I12 |
| L4-I-133-BH-010 | Student Lifecycle and Finance retain ownership of their authoritative facts; downstream scheduling, Hall Ticket, conduct, evaluation, and result workflows remain with their canonical owners. | L3-I-133 §§4–5,14,16 |

## 6. Failure / Rejection / Continuation

- Missing published result, conflicting Student context, exhausted attempt limits, unresolved Finance eligibility, failed examination approval, or failure to establish the required I04 registration shall prevent downstream participation.
- A rejected or withheld application shall create no authorised I12 attempt.
- Application approval alone shall not create or own Examination Registration.
- An exception to attempt limits shall require authoritative institutional policy/approval and shall not be inferred locally.

## 7. Audit & Evidence Behavior

The system shall preserve the application, candidate/opportunity context, published-result and academic basis, eligibility decision, prior-attempt evidence, Finance status where applicable, approval authority, I04 registration initiation, I12 lifecycle position, published lifecycle facts, and resulting authorised/rejected outcome.

## 8. Observable Completion Conditions

Completion occurs when eligibility and applicable Finance conditions are satisfied, required approval is complete, the application is authorised for the subsequent attempt, required Examination Registration is initiated through I04, the attempt is established under I12, and applicable I12/I04 lifecycle facts are preserved; or when a governed rejected/withheld outcome exists with no authorised attempt.

## 9. Traceability & Provenance

**Parent:** L3-I-133 — Subsequent Examination Application, Eligibility and Attempt Authorization  
**Primary verification source:** L2-I12 — Supplementary / Backlog / Improvement Examination Execution Workflows.  
**Registration ownership constraint:** L2-I04 — Examination Registration Workflows; locked I-RC-03 ownership resolution.

## 10. Open Issues / Deferred Design

Institution-specific eligibility values, attempt limits, fee conditions, and approval values remain policy/configuration governed. I12's existing lifecycle begins with `Eligible`, progresses through `Applied` and `Registered`, and continues through governed execution stages; no separate `Authorized` state is introduced. No `AttemptAuthorized` event is invented. Technical orchestration, APIs, persistence, UI, transport, and infrastructure remain outside L4.

## Canonical Validation Note

This specification is a canonical L4 System Behavior Specification adhering to the locked I-RC-03 ownership resolution and is not self-locked.
