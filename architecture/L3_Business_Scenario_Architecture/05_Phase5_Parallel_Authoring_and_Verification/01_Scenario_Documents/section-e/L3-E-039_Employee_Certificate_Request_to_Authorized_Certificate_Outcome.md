# Employee Certificate Request to Authorized Certificate Outcome

**Scenario ID:** L3-E-039  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** E  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E04 — HR Workflows; Document Platform; applicable Employee Profile context  
**Referenced By:** None yet.

## 1. Scenario Metadata

The canonical identity and frozen Stage 3 semantics are preserved exactly. This document applies the locked upstream architecture and does not redefine ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose

Process an employee's official HR certificate request through eligibility validation and HR approval so that an authorised certificate outcome is established and the document is made available through the governed document process.

## 3. Business Outcome

An eligible certificate request is approved, the certificate is generated through the governed document process, and the certificate becomes available as the authorised HR outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An employee certificate request is submitted to HR.

### 4.2 End Boundary

The certificate is available following validation, HR approval, generation and governed document storage.

### 4.3 In Scope

Certificate request; eligibility validation; HR approval; generation request; document-platform storage; certificate availability.

### 4.4 Out of Scope

The scenario does not own the resulting employment fact, employee master data, organisational structure, position assignment, document storage, or any other specialised state owned by another canonical domain.

### 4.5 Related Scenarios

None yet.

## 5. Ownership and Participating Domains

| Domain | Business responsibility |
|---|---|
| HR (L2-E04) | Owns the HR case, approval process, decision record and accepted handoff outcome. |
| Workforce & Engagement (L2-E01) | Owns authoritative workforce/employment state and performs the employment lifecycle transition where applicable. |
| Position & Assignment (L2-E02) | Owns position/assignment state when the approved organisational change affects assignment placement. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

- Employee / requester.
- HR Administrator / HR certificate process owner.
- HR approver where approval is required.
- Document Platform as the document-storage participant.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- A certificate request has been received.
- The requester and relevant employee context are identifiable.
- Certificate eligibility can be validated.
- The required HR approval can be obtained.

## 8. Trigger

An employee certificate request is submitted.

## 9. Main Business Flow

| Step | Business behaviour | Responsible owner / participant | Business result |
|---|---|---|---|
| 1 | Create the certificate request and identify the requested certificate type. | HR | Certificate request is established. |
| 2 | Validate eligibility for the requested certificate. | HR | Eligibility decision is established. |
| 3 | Obtain and record HR approval. | HR / Approver | Certificate request is authorised for generation. |
| 4 | Request certificate generation through the governed document process. | HR / Document Platform | Certificate document is generated. |
| 5 | Store the generated certificate through Document Platform. | Document Platform | Certificate is preserved in the authoritative document platform. |
| 6 | Record the certificate as available to the requester. | HR | Authorised certificate outcome is complete. |

The flow stops when the institutional business outcome and end boundary are reached.

## 10. Alternate Business Flows

- If eligibility is not satisfied, the certificate is not authorised for generation; the request receives the applicable business outcome.
- If HR approval is not granted, generation does not proceed and the request is resolved according to the certificate workflow.

## 11. Failure, Interruption and Business Recovery

| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Eligibility validation fails | Certificate cannot be authorised. | Record the eligibility outcome and resolve the request according to HR case governance. | HR |
| Approval is rejected | No authorised certificate outcome exists. | Record the rejection and close/resolve the certificate request according to its lifecycle. | HR |
| Document storage is not yet complete | Certificate is not yet available as the final business outcome. | Keep the request in its applicable processing state until governed document storage completes. | HR / Document Platform |

No technical retry, queue, rollback, infrastructure failure or implementation recovery is defined.

## 12. Business Rules and Decision Constraints

- Certificate Request Aggregate is owned by HR.
- HR certificate examples include Experience Certificate, Employment Certificate, Relieving Certificate and NOC.
- The certificate workflow is Request → Validate Eligibility → HR Approval → Document Generation Request → Document Platform Storage → Certificate Available.
- Document storage is owned by Document Platform.
- HR owns the business workflow and approval, not the underlying document-storage lifecycle.

These rules remain owned by their authoritative upstream domains.

## 13. State and Lifecycle Impact

The Certificate Request Aggregate uses the applicable HR workflow lifecycle. The generic E04 workflow lifecycle includes Initiated, PendingApproval, Approved, Processing, Completed, Failed, Rejected, Closed and Archived; the scenario does not create a new certificate-specific state.

No new state or transition is introduced by this scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

The locked E04 downstream model explicitly identifies **CertificateGenerated** as an example event consumed by Notification Service.

The event remains a business event from the HR certificate workflow; Document Platform owns document storage.

### 14.2 Synchronous Business Contracts / Hand-offs

The synchronous handoff is the certificate generation request from HR to the Document Platform, followed by the business outcome that the certificate is available. This does not transfer certificate-request ownership from HR or document-storage ownership from Document Platform.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

HR approval is mandatory in the certificate workflow. Approval records actor, timestamp and decision. Certificate request history is maintained as business process history. Technical access control and delivery mechanics are outside L3.

## 16. Completion Criteria and Postconditions

- Eligibility is validated.
- HR approval is recorded.
- Certificate generation is requested and the resulting document is stored through Document Platform.
- Certificate becomes available as the authorised business outcome.
- Request history remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified that materially block this scenario after applying the locked ownership clarification in L2-E01 and the HR workflow boundaries in L2-E04.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.

**Authoring Classification:** PASS — source-grounded Draft.
