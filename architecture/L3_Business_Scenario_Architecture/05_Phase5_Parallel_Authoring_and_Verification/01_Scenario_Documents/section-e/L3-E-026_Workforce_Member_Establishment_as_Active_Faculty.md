# Workforce Member Establishment as Active Faculty

**Scenario ID:** L3-E-026  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Faculty Workforce  
**Participating Domains:** A, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20  
**Depends On:** L2-E03 — Faculty Workforce Workflows; HR Domain Architecture; L2-E02 — Position & Assignment Workflows  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
This scenario preserves the frozen canonical identity, title, owner, participating domains, Scenario Type and Priority. It applies the locked upstream architecture without transferring ownership or introducing new architecture.

## 2. Scenario Purpose
Establish an eligible workforce member as an active Faculty Workforce participant after the authoritative faculty verification and activation conditions are satisfied.

## 3. Business Outcome
The Faculty Workforce representation exists, is verified and reaches Active, establishing the faculty member's institutional academic participation readiness.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A valid upstream workforce/employee context exists and a faculty workforce representation is required.

### 4.2 End Boundary
The Faculty Workforce is Active and `FacultyWorkforceActivated` has been published.

### 4.3 In Scope
- Establish the Faculty Workforce representation.
- Validate the authoritative workforce/employee context.
- Perform faculty verification.
- Satisfy required institutional approval.
- Activate the Faculty Workforce.
- Preserve the lifecycle history and publish the activation fact.

### 4.4 Out of Scope
Implementation mechanics, APIs, databases, services, UI, infrastructure, technical access control/IAM, queues, technical retries and other L4+ concerns are outside L3.

### 4.5 Related Scenarios
Only canonical dependencies relevant to this outcome are referenced; no new scenario ownership is created.

## 5. Ownership and Participating Domains
**Canonical owner:** Faculty Workforce.

Participating domains remain authoritative for their own concepts and provide business context only within those ownership boundaries. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Faculty Workforce owns the academic faculty representation and lifecycle.
- HR owns employee identity/employment lifecycle and supplies authoritative workforce context.
- Position & Assignment supplies organisational assignment context where required.
- Institutional academic authority participates in faculty verification/approval.

No technical permission model is introduced.

## 7. Preconditions and Entry Criteria
- Authoritative workforce/employee context exists.
- Required faculty information is available.
- Duplicate faculty representation is not permitted.
- Required verification and approval conditions are satisfied before activation.

## 8. Trigger
An authorised institutional process requires an eligible workforce member to become an active Faculty Workforce participant.

## 9. Main Business Flow
1. Resolve the authoritative workforce/employee identity.
2. Validate that a duplicate Faculty Workforce representation does not already exist.
3. Capture the required faculty workforce information.
4. Create the Faculty Workforce representation in its initial lifecycle state.
5. Generate the required business audit/evidence record.
6. Perform the **WF-E03-002 — Faculty Verification & Activation** workflow.
7. Validate faculty workforce readiness and required institutional approval.
8. Activate the Faculty Workforce.
9. Publish **`FacultyWorkforceActivated`**.
10. Stop when Active faculty participation is authoritative.

The flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
- If required information is incomplete, the Faculty Workforce is not activated until the information is completed.
- If verification is rejected, the Faculty Workforce does not reach Active and the canonical rejection outcome is recorded.
- If a Faculty Workforce representation already exists, the creation path is not duplicated.

## 11. Failure, Interruption and Business Recovery
If verification or required approval fails, the Faculty Workforce remains outside Active participation and the rejection/required correction follows the authoritative E03 lifecycle. A non-Active Faculty Workforce cannot participate in academic activities. No technical retry or rollback is described.

Only business-level recovery is described.

## 12. Business Rules and Decision Constraints
- Faculty Workforce is the authoritative academic workforce capability domain.
- HR employee identity/employment ownership is not transferred to Faculty Workforce.
- A non-Active Faculty Workforce SHALL NOT participate in academic activities.
- Teaching Eligibility SHALL NOT remain operational when the Faculty Workforce is not Active.
- Faculty Availability SHALL NOT indicate availability for a non-Active Faculty Workforce.
- Workload SHALL NOT accept new allocations unless the Faculty Workforce is Active.
- Lifecycle transitions preserve historical records and auditability.

The rules remain owned by the authoritative upstream architecture.

## 13. State and Lifecycle Impact
**Faculty Workforce lifecycle:** the creation workflow establishes the faculty workforce representation in **Draft**; the verification and activation workflow validates readiness and establishes **Active**.

Canonical workflows:
- WF-E03-001 — Faculty Workforce Creation
- WF-E03-002 — Faculty Verification & Activation

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `FacultyWorkforceCreated` | Faculty Workforce | Faculty workforce profile has been created. |
| `FacultyWorkforceActivated` | Faculty Workforce | Faculty workforce has been activated for institutional academic participation. |
| `FacultyVerificationRejected` | Faculty Workforce | Faculty verification has been rejected. |

### 14.2 Business Contracts / Hand-offs
HR provides authoritative employee/workforce context; Position & Assignment provides organisational assignment context. These dependencies do not transfer Faculty Workforce ownership.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
Faculty verification and activation require the applicable institutional approval and policy validation. The locked L2 establishes approval as part of the readiness decision; no unsupported named approving role is invented.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Faculty Workforce representation exists.
- Required verification is complete.
- Faculty Workforce is Active.
- Academic participation readiness is established.
- `FacultyWorkforceActivated` is published.
- Historical/audit evidence is preserved.

### 16.2 Postconditions
The completed business outcome remains authoritative for subsequent institutional behaviour and preserves required historical traceability.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
