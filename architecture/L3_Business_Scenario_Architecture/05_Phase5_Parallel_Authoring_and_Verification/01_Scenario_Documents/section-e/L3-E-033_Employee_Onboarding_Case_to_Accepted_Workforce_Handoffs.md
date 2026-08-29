# Employee Onboarding Case to Accepted Workforce Handoffs

**Scenario ID:** L3-E-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** A, B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20  
**Depends On:** L2-E04 — HR Workflows; L2-E01 — Workforce & Engagement Workflows; L2-E07 — Employee Profile; Recruitment and Identity dependencies

## 1. Scenario Metadata

This document preserves the frozen Phase 4 / Stage 3 scenario identity, title, Primary Owner, Participating Domains, Scenario Type and Priority. It applies the locked upstream architecture and does not redesign ownership.

## 2. Scenario Purpose

Orchestrate an approved hiring outcome through the HR onboarding case to a completed onboarding outcome and the required accepted hand-offs to authoritative workforce, employee-profile and access-related domains.

## 3. Business Outcome

The HR onboarding case completes its document, verification, approval and readiness responsibilities, and the resulting accepted business hand-offs are made to the authoritative domains without HR taking ownership of their lifecycle data.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An approved hiring outcome is received from the Recruitment domain.

### 4.2 End Boundary

The onboarding case reaches Completed and the accepted downstream business hand-offs have been made, including the authoritative employment transition being performed by L2-E01 rather than by HR.

### 4.3 In Scope

- Onboarding case creation and lifecycle.
- Document collection and verification.
- Approval routing and readiness check.
- Coordination with the authoritative employment lifecycle.
- Employee-profile and access-related business hand-offs.
- Completion and audit history of the HR case.

### 4.4 Out of Scope

Implementation mechanics, APIs, databases, services, Business Data Structures, code, UI, infrastructure, technical access control/IAM, queues, technical retries, rollback and deployment concerns are outside L3.

### 4.5 Related Scenarios

Only canonical upstream/downstream dependencies explicitly supported by the locked architecture are referenced.

## 5. Ownership and Participating Domains

**Canonical Owner:** HR

Participating domains remain authoritative for their own concepts. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

- HR Administrator.
- Hiring Manager.
- Employee.
- Verification Team.
- L2-E01 Workforce & Engagement as owner of employment state.
- L2-E07 Employee Profile as owner of employee master/profile data.
- Identity Domain as owner of authentication/access provisioning.
- Recruitment as source of the approved hiring outcome.

No technical role or unsupported authority is introduced.

## 7. Preconditions and Entry Criteria

- Hiring outcome is approved.
- Required employee identity/profile context is available from the authoritative upstream source.
- Required onboarding documentation can be collected.
- Required verification and approval conditions can be evaluated.

## 8. Trigger

`HiringApproved` initiates the Employee Onboarding Workflow.

## 9. Main Business Flow

1. Receive the approved hiring outcome.
2. Create the Employee Onboarding case.
3. Collect required employee documents.
4. Verify the submitted information.
5. Perform the onboarding readiness and approval checks.
6. Coordinate the required employment transition with **L2-E01 Workforce & Engagement**, which owns the authoritative employment state.
7. Coordinate the required employee-profile hand-off with **L2-E07 Employee Profile**, which owns employee master/profile data.
8. Coordinate required access-provisioning requirements with the Identity Domain.
9. Complete the HR onboarding case when the required onboarding outcome and hand-offs are accepted.
10. Publish `EmployeeOnboardingCompleted`.
11. Stop when the HR case is Completed and the accepted hand-offs are established.

The Main Business Flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows

- Missing/incomplete onboarding information keeps the case from reaching readiness until the required business information is available.
- Verification failure prevents onboarding completion until the authoritative correction/verification path succeeds.
- HR does not directly create or own the authoritative employment state; L2-E01 performs that transition.
- HR does not directly modify employee master/profile data; L2-E07 remains authoritative.
- Identity remains authoritative for access provisioning.

## 11. Failure, Interruption and Business Recovery

If required onboarding conditions are not satisfied, the HR case remains in the applicable workflow state and does not complete. The authoritative external domain state remains unchanged until its own accepted business transition occurs. No technical retry or rollback is described.

Only business-level recovery is described.

## 12. Business Rules and Decision Constraints

- HR Workflows owns the HR onboarding case, not employment state.
- Employment lifecycle state is owned exclusively by L2-E01.
- Employee master/profile data is owned by L2-E07.
- Onboarding includes document collection, verification, approval routing and readiness check.
- Every approval records actor, timestamp and decision.
- HR SHALL maintain business process history.
- External domains remain authoritative for their owned data.

These rules remain owned by the authoritative upstream architecture.

## 13. State and Lifecycle Impact

**HR Onboarding Case:** Initiated → documentation/verification processing → readiness/approval → Completed, using the HR case lifecycle.

**Employment lifecycle:** the resulting employment transition is performed by L2-E01; HR records the case outcome rather than owning the employment state.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `EmployeeOnboardingCompleted` | HR Workflows / Employee Onboarding case | HR onboarding case has completed. |

The locked HR workflow explicitly defines `HiringApproved` as the trigger and `EmployeeOnboardingCompleted` as the completion event. The authoritative employment event remains owned and published by L2-E01.

### 14.2 Business Contracts / Hand-offs

| Business Hand-off | Canonical Owner | HR Responsibility |
|---|---|---|
| Approved hiring outcome | Recruitment | Initiates onboarding case |
| Employment state transition | L2-E01 Workforce & Engagement | Validate/route/approve case, then invoke the authoritative employment transition |
| Employee master/profile | L2-E07 Employee Profile | Coordinate accepted profile hand-off; do not own employee master data |
| Access provisioning | Identity Domain | Submit/coordinate business access requirement; Identity remains authoritative |
| Documents | Document Platform | Provide document business context; platform owns storage lifecycle |

Events are distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

HR approval routing and readiness checks are governed by L2-E04. Every approval records actor, timestamp and decision. HR does not invent or assume technical authorization. The employment transition remains subject to L2-E01 authority.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- HR onboarding case is completed.
- Required documentation and verification are complete.
- Required approval/readiness outcome is complete.
- Accepted hand-offs to authoritative domains are established.
- `EmployeeOnboardingCompleted` is published.
- HR retains the case outcome and business process history without taking ownership of external domain data.

### 16.2 Postconditions

The completed business outcome remains authoritative for subsequent institutional behaviour and required historical information is preserved.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified from the authoritative sources reviewed for this scenario.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.

**Authoring Classification:** PASS — source-grounded Draft.
