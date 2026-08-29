# Examination Registration for a Candidate with Approved Special Examination Arrangements

**Scenario ID:** L3-I-029  
**Participating Domains:** A, C, D, F, I  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I01 Examination Policy & Configuration Workflows; L2-I02 Examination Session Management Workflows; L2-I03 Examination Planning & Scheduling Workflows; L2-I04 Examination Registration Workflows; L2-I05 Hall Ticket Management Workflows; applicable Student, Academic Delivery, Finance, Platform Authorization and Audit architecture  
**Referenced By:** L2-I05 Hall Ticket Management and downstream examination operations

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-029 |
| Canonical Title | Examination Registration for a Candidate with Approved Special Examination Arrangements |
| Primary Owner | Examination |
| Participating Domains | A, C, D, F, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern examination registration for a candidate who has an institutionally approved special examination arrangement, while preserving Examination Registration as the canonical owner of examination participation and keeping the underlying student, academic, financial, policy, authorization, and audit responsibilities with their respective owners.

## 3. Business Outcome

The candidate is registered for the authorised examination scope with the approved special arrangement recognised for the applicable examination participation, and the resulting registration is available for downstream examination processing subject to all remaining registration and examination conditions.

## 4. Scenario Boundary and Scope

**Start Boundary:** A candidate has an approved special examination arrangement applicable to an examination opportunity and requires registration for that opportunity.

**In Scope:** verification of the approved arrangement; candidate and examination eligibility; applicable subject registration; controlled registration; required review/approval; preservation of the arrangement and registration history; and making the confirmed registration available to downstream examination workflows.

**Out of Scope:** creation or adjudication of the underlying special arrangement; changes to examination policy; student-status ownership; curriculum ownership; financial transaction processing; hall-ticket design; examination-conduct procedures; and technical authorization or implementation.

**End Boundary:** The registration is confirmed with the applicable arrangement recognised, or the registration is rejected/withheld because the arrangement or another mandatory registration condition does not support the requested participation.

## 5. Ownership and Participating Domains

**Examination (I):** owns Examination Registration and the resulting examination participation.

**Platform Foundation (A):** provides authorization and audit capabilities; it does not own the examination decision.

**Student Lifecycle (C):** provides candidate/student context.

**Academic Delivery (D):** provides applicable academic/curriculum context.

**Finance (F):** provides financial eligibility where required by examination policy.

Participation does not transfer ownership of any upstream business concept.

## 6. Business Actors, Roles and Responsibilities

- **Candidate (Student):** requests or participates in the registration process.
- **Examination Administrator:** manages examination registration.
- **Controller of Examination:** governs examination registration policy and approval.
- **Department Administration:** verifies departmental requirements where applicable.
- **Academic Administration:** supports academic eligibility decisions where applicable.
- **Student Domain:** provides authoritative student context.
- **Finance Domain:** provides financial eligibility where policy requires it.

The authority that approves the underlying special arrangement is not invented here; it remains governed by the applicable upstream policy.

## 7. Preconditions and Entry Criteria

- An Examination Session is Active.
- An Examination Schedule has been officially published.
- The candidate has an authoritative Student context.
- An approved special examination arrangement exists and applies to the candidate and relevant examination opportunity.
- The candidate satisfies all registration requirements not waived or modified by the approved arrangement.
- Applicable registration-window and examination-policy conditions are satisfied.

## 8. Trigger

An approved special examination arrangement is presented as the basis for registering the candidate for the applicable examination opportunity.

## 9. Main Business Flow

1. Establish the candidate, Examination Session, examination scope, and applicable subjects.
2. Verify that the special examination arrangement is approved and applicable to the candidate and examination opportunity.
3. Evaluate the candidate against the remaining examination registration eligibility requirements.
4. Determine the registration scope and any arrangement-dependent registration conditions.
5. Register the applicable examination subjects under the appropriate registration category.
6. Complete required examination review and approval.
7. Confirm the Examination Registration.
8. Preserve the relationship between the approved arrangement and the registration outcome in the business history.
9. Publish the applicable canonical registration events.
10. Make the confirmed registration available to Hall Ticket Management and other downstream examination workflows.

## 10. Alternate Business Flows

### 10.1 Arrangement Applies Only to Part of the Examination Scope

The registration may cover the eligible examination scope while the approved arrangement applies only to the examinations or subjects for which it was granted.

### 10.2 Administrative Registration

An authorised institutional administrator may perform registration on the candidate's behalf where institutional policy permits.

### 10.3 Additional Approval Required

Where the approved arrangement requires a registration-specific review or approval, the registration remains subject to that approval.

### 10.4 Arrangement Applies to a Special Registration Window

Where policy permits registration under a special or exceptional window, the registration follows the applicable configured registration-window rules.

## 11. Failure, Interruption and Business Recovery

### 11.1 Arrangement Cannot Be Verified

The special arrangement is not applied and the registration does not complete through this scenario until authoritative evidence is available.

### 11.2 Arrangement Does Not Apply

If the approved arrangement does not cover the candidate, session, examination, subject, or relevant circumstance, the registration follows the normal rules or is rejected according to policy.

### 11.3 Another Mandatory Eligibility Condition Fails

The special arrangement does not automatically waive unrelated eligibility requirements. The registration remains pending or is rejected according to the applicable rule.

### 11.4 Registration Is Already Locked

A locked registration is not routinely changed. Any permitted exceptional correction follows the applicable governed exception/amendment process.

## 12. Business Rules and Decision Constraints

- Examination Registration begins only after an Active Examination Session and officially published Examination Schedule exist.
- Only eligible candidates may register.
- Registration must align with applicable programme, curriculum, academic-period, examination-policy, and subject requirements.
- Special examination arrangements are applied only within their approved scope.
- Registration remains candidate-centric.
- Confirmed registrations become authoritative business inputs for downstream examination workflows.
- Significant registration activity is auditable.
- Ownership of student, academic, financial, authorization, and policy information remains with the originating domains.

## 13. State and Lifecycle Impact

The scenario uses the existing Examination Registration lifecycle defined by L2-I04. No special-arrangement lifecycle state is invented.

The registration progresses through the existing initiation, eligibility validation, subject registration, review, approval, confirmation, and locking milestones as applicable.

The special arrangement modifies the applicable business conditions; it does not create a second registration lifecycle.

## 14. Business Events and Cross-Domain Collaboration

Examination Registration is the canonical publisher for registration events defined by L2-I04.

Material events include:

- **Registration Created**
- **Registration Approved**
- **Registration Confirmed**
- **Registration Withdrawn**
- **Registration Locked**

The special arrangement itself is not assigned a new event name in this scenario. If an upstream source defines a canonical arrangement event, that event remains owned by its source domain.

The confirmed registration is consumed by Hall Ticket Management and subsequent examination workflows.

## 15. Business Authorization, Approval and Audit Obligations

- The special arrangement must already be approved under the applicable institutional policy.
- Examination-specific registration approval remains applicable where required.
- Registration activity and the resulting arrangement-sensitive decision must be auditable.
- The underlying arrangement's approval evidence remains owned by its governing business process.
- Platform Foundation retains ownership of authorization and audit infrastructure.
- No technical IAM/RBAC design is introduced.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The approved arrangement is verified as applicable.
- Candidate and examination eligibility are established.
- Required subjects and registration scope are established.
- Required registration approval is complete.
- Registration is confirmed.
- Applicable registration events are published.
- The arrangement-sensitive registration outcome is traceable.

**Postconditions:**
- The candidate has an authoritative Examination Registration for the approved scope.
- Downstream examination workflows may consume the confirmed registration.
- The special arrangement does not transfer ownership of Examination Registration or upstream student/academic policy.

## 17. Open Issues, Assumptions and Deferred Questions

1. The repository source set available for this authoring run does not expose a dedicated canonical event specifically for a special examination arrangement; no new event is invented.
2. The exact authority and business rules for approving the underlying arrangement remain upstream-policy owned and are not redefined here.
3. The exact frozen Stage 3 Type/Priority row was not independently retrievable from the uploaded register during this run; the metadata above follows the current frozen classification used for this examination-registration exception sequence and should be mechanically rechecked against the Stage 3 row before Review.
