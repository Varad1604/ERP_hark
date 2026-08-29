# Candidate Examination Registration to Confirmed Registration

**Scenario ID:** L3-I-017  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** C, D, F, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** L2-I01 Examination Policy & Configuration; L2-I02 Examination Session Management; L2-I03 Examination Planning & Scheduling; L2-I04 Examination Registration; Student Lifecycle; Curriculum Framework; Academic Calendar; Finance  
**Referenced By:** L2-I05 Hall Ticket Management; downstream examination workflows

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-I-017` |
| Canonical Title | Candidate Examination Registration to Confirmed Registration |
| Primary Owner | Examination Registration |
| Participating Domains | C, D, F, I |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |
| Authoritative L2 | `L2-I04` — Examination Registration Workflows |

Frozen Phase 5 identity is preserved. L2-I04 owns the Examination Registration aggregate and its lifecycle.

## 2. Scenario Purpose

Establish an authoritative Examination Registration for an eligible candidate so the applicable examination subjects are formally registered within an active Examination Session and the registration reaches the canonical `Confirmed` position for downstream examination operations.

## 3. Business Outcome

The candidate's examination registration is validated against the active examination context, academic applicability, financial eligibility where applicable, and institutional registration rules; required review and approval are completed; and the registration becomes the authoritative Confirmed participation record.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An eligible candidate initiates registration during an approved registration window for a published examination schedule.

### 4.2 End Boundary
The Examination Registration reaches `Confirmed` and is available to downstream Hall Ticket and examination workflows.

### 4.3 In Scope
- Examination Session and Schedule identification.
- Candidate and academic-context validation.
- Examination eligibility validation.
- Subject and category registration.
- Financial eligibility consumption where required.
- Institutional review/approval where required.
- Registration confirmation and canonical event publication.
- Auditability.

### 4.4 Out of Scope
- Examination planning and timetable publication.
- Hall Ticket generation.
- Examination conduct, marks, or results.
- Student, curriculum, or Finance ownership.
- Finance transaction processing.
- Technical APIs, persistence, queues, or infrastructure.

### 4.5 Related Scenarios — Optional
Hall Ticket processing begins only after the required I04 registration outcome exists.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Ownership Boundary |
|---|---|---|
| Examination Registration (I04) | Owns registration and lifecycle. | Canonical registration owner. |
| Student Lifecycle (C) | Provides authoritative candidate/student context. | Student ownership remains C. |
| Academic/Curriculum context (D) | Provides applicable academic and subject context. | Academic-delivery/curriculum ownership remains upstream. |
| Finance (F) | Provides financial eligibility/payment status where applicable. | Financial truth remains F. |

## 6. Business Actors, Roles and Responsibilities

L2-I04 supports Candidate, Examination Administrator, Controller of Examination, Department Administration, Academic Administration, Finance Domain, Curriculum Framework Domain, Academic Calendar Domain, Student Domain, and Audit Infrastructure participation. Exact institution-specific approval sequencing remains policy governed.

## 7. Preconditions and Entry Criteria

- Examination Policy is established.
- Examination Session is `Active`.
- Examination Schedule is `Published`.
- Registration window is open or an authorised exception applies.
- Candidate satisfies applicable eligibility requirements.
- Applicable examination subjects are valid.
- Financial eligibility is satisfied where required.

## 8. Trigger

The candidate initiates examination registration for the applicable scheduled examination opportunity.

## 9. Main Business Flow

1. Identify the active Examination Session and published schedule.
2. Consume authoritative candidate and academic context.
3. Validate registration eligibility and applicable restrictions.
4. Register valid examination subjects and category.
5. Complete required institutional review.
6. Obtain required approval.
7. Confirm the Examination Registration.
8. Publish the applicable I04 registration events.
9. Preserve registration history and audit evidence.

## 10. Alternate Business Flows

- Late/special registration follows authorised policy.
- Supplementary, Backlog, or Improvement categories use the same I04 registration ownership while their administration may be coordinated by the applicable examination workflow.
- Financial eligibility failure does not transfer financial decision ownership to Examination.
- Correctable registration issues return through the governed review/amendment path.

## 11. Failure, Interruption and Business Recovery

Registration does not proceed when the Examination Session is not Active, the schedule is not Published, eligibility fails, applicable subjects are invalid, or required approval is absent. Duplicate or conflicting registration is resolved through I04 governance. Recovery remains business-level; no technical retry or rollback is defined.

## 12. Business Rules and Decision Constraints

- Registration requires an Active Examination Session and Published Examination Schedule.
- Candidate eligibility must be satisfied.
- Registration must occur within an approved window unless an authorised exception applies.
- Every registration includes one or more valid examination subjects.
- Only approved registrations may become `Confirmed`.
- Confirmed registrations are authoritative downstream inputs.
- Student, academic/curriculum, and Finance facts remain with their canonical owners.

## 13. State and Lifecycle Impact

L2-I04 defines the governed registration lifecycle from Registration Initiation through Eligibility Validation, Subject Registration, Review, Approval, Confirmation, Locking, Operational Consumption, Completion, and Archival, with controlled amendment/withdrawal paths.

This scenario ends at `Registration Confirmation`; it introduces no new registration state.

## 14. Business Events and Cross-Domain Collaboration

L2-I04 defines registration events including `Registration Initiated`, `Eligibility Validated`, `Registration Submitted`, `Registration Approved`, `Registration Rejected`, `Registration Confirmed`, `Registration Amended`, `Registration Withdrawn`, `Registration Locked`, and `Registration Archived`.

The Examination Registration domain is the canonical publisher. This scenario's completion fact is `Registration Confirmed`. Upstream domains supply authoritative context; downstream Hall Ticket processing consumes the confirmed registration without acquiring ownership.

## 15. Business Authorization, Approval and Audit Obligations

Examination Administrator, Controller of Examination, Department Administration, and Academic Administration participate as governed by L2-I04 and institutional policy. Significant registration activities—including eligibility, subject scope, review, approval, confirmation, amendment, and withdrawal where applicable—must remain auditable. No technical IAM design is introduced.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Active Examination Session confirmed.
- Published Examination Schedule identified.
- Eligibility satisfied.
- Applicable subjects registered.
- Required review/approval complete.
- Registration is `Confirmed`.
- `Registration Confirmed` is published by I04.
- Registration history is auditable.

### 16.2 Postconditions
- Candidate has an authoritative Confirmed Examination Registration.
- Hall Ticket and downstream examination workflows may consume it.
- Student, academic/curriculum, and Finance ownership remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No material ownership, lifecycle, publisher, or source-access issue remains for the core I017 outcome after direct reconciliation with locked L2-I04.

### 17.2 Assumptions
Institution-specific approval chains and registration-window values remain policy/configuration governed.

### 17.3 Deferred Questions
Implementation mechanics remain outside L3.
