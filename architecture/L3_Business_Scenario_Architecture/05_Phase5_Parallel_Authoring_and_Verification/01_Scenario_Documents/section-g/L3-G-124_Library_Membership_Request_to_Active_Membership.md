# Library Membership Request to Active Membership

**Scenario ID:** L3-G-124  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** Locked L1 Business Domain Architecture; L2-G05 Library Management Workflows  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-G-124` |
| Canonical Title | Library Membership Request to Active Membership |
| Primary Owner | Library Management |
| Participating Domains | G, C, E, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |
| Authoritative L2 | `L2-G05` — Library Management Workflows |

The final Phase 5 identity is `L3-G-124`. Earlier candidate-number or sequential Stage-3 identifiers do not replace the approved final L3 identity.

## 2. Scenario Purpose

Establish an active Library Membership for an eligible applicant while preserving Library Management ownership and consuming authoritative identity/status context from participating domains without transferring ownership.

## 3. Business Outcome

An eligible library membership request is approved and the Library Membership becomes `Active`, establishing the applicant's authorised library-service relationship.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A configured library membership request is submitted.

### 4.2 End Boundary
The membership reaches `Active` and its operational history is established.

### 4.3 In Scope
- Membership request.
- Identity/status validation.
- Eligibility determination.
- Approval where required.
- Membership activation.
- Membership history.

### 4.4 Out of Scope
- Student or personnel identity creation.
- Finance accounting or payment processing.
- Circulation transactions.
- Technical implementation or notification delivery mechanics.

### 4.5 Related Scenarios — Optional
Subsequent circulation, reservation, fine, and clearance outcomes remain separate Library Management scenarios.

## 5. Ownership and Participating Domains

Library Management owns the Library Membership aggregate, including membership registration, membership status, borrowing eligibility, and membership history.

Student Lifecycle and People & Workforce provide authoritative identity/status context where applicable. Section H may consume the membership outcome for authorised communication/reporting use. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

- Applicant — submits the membership request.
- Librarian / Library Assistant — performs operational membership review.
- Library Administrator / configured authority — approves membership where required.
- Student Lifecycle / People & Workforce — provide authoritative identity or status context.
- Section H — consumes the authoritative outcome where applicable; it does not own membership.

## 7. Preconditions and Entry Criteria

- Applicant identity is authoritative.
- Applicable membership category is identifiable.
- Membership policy is available.
- No governed membership conflict prevents activation.

## 8. Trigger

A library membership request is submitted.

## 9. Main Business Flow

1. Register the membership request.
2. Establish authoritative applicant identity and status context.
3. Determine the applicable membership category.
4. Validate membership eligibility under configured policy.
5. Perform the required operational review.
6. Obtain approval where required.
7. Establish the Library Membership.
8. Move the membership through the applicable lifecycle to `Active`.
9. Preserve membership history.
10. Publish the applicable canonical membership events.
11. Make the active membership available to subsequent Library Management operations.

## 10. Alternate Business Flows

- Eligibility fails → membership does not become `Active`.
- Approval is not granted → request remains outside active membership according to policy.
- Identity context is unavailable → obtain authoritative context from its owner before proceeding.
- A conflicting membership exists → resolve through the governed membership process without creating duplicate ownership.

## 11. Failure, Interruption and Business Recovery

The business recovery outcome is correction, authoritative-source resolution, or renewed review of the membership request. No technical retry, queue, rollback, database, or infrastructure mechanics are defined at L3.

## 12. Business Rules and Decision Constraints

- Membership categories and eligibility are configuration driven.
- Library Membership owns its operational status and history.
- Student and workforce identity/status facts remain with their canonical owners.
- Lifecycle transitions are auditable.
- Historical membership information remains preserved.
- Finance owns accounting and payment processing.

## 13. State and Lifecycle Impact

Locked L2-G05 defines the Membership lifecycle:

`Applied → Approved → Active → Suspended → Closed`

This scenario covers establishment through `Applied → Approved → Active`. No additional lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

Locked L2-G05 defines the membership events:

| Business Event | Canonical Publisher | Scenario Purpose |
|---|---|---|
| `MembershipRequested` | G05 Library Management | Records submission of the membership request. |
| `MembershipApproved` | G05 Library Management | Records the approved/activated membership outcome. |

Student Lifecycle and People & Workforce provide authoritative context; Section H may consume the resulting membership fact. G05 remains the sole publisher and owner of the membership lifecycle.

## 15. Business Authorization, Approval and Audit Obligations

Configured Library Management approval governs activation. The request, eligibility basis, approval, activation, lifecycle transition, and membership history must remain auditable. No technical IAM or document-delivery design is introduced.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Request registered.
- Eligibility validated.
- Approval completed where required.
- Membership is `Active`.
- `MembershipApproved` is published by G05.
- Membership history is preserved.

### 16.2 Postconditions
The applicant has an authoritative Active Library Membership available to subsequent governed library operations.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No material final-ID, ownership, lifecycle, or membership-event ambiguity remains after reconciliation with the Phase 5 filename authority and locked L2-G05.

### 17.2 Assumptions
Institution-specific membership categories, approval paths, and eligibility values remain configuration governed.

### 17.3 Deferred Questions
Technical delivery, access-control enforcement, and implementation mechanics remain outside L3.
