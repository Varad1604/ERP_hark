# Library Membership Suspension to Restricted Library Access

**Scenario ID:** L3-G-125  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-125 |
| Canonical Title | Library Membership Suspension to Restricted Library Access |
| Primary Owner | Library Management |
| Participating Domains | G, C, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional outcome in which an already-active Library Membership is suspended and the member's library access is consequently restricted.

Library Management owns the membership lifecycle and the operational membership status. The suspension decision may be driven by configurable library policy, outstanding obligations, administrative restrictions, or another authoritative business condition. The scenario does not invent a suspension reason or policy threshold.

The scenario also establishes the correct cross-domain boundary. Where the member is a student or employee, C or E remains authoritative for identity and lifecycle information. H may consume the established membership outcome for governed communication. None of those domains acquires ownership of Library Membership.

## 3. Business Outcome

An Active Library Membership is authoritatively suspended, the member's library access is restricted according to the applicable configured policy, the suspension reason and decision history are preserved, and downstream library operations respect the resulting membership status.

Suspension does not imply closure. It is a distinct lifecycle outcome from permanent membership closure.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when a valid business condition requires review of an Active Library Membership for suspension.

### 4.2 End Boundary

The scenario ends when the Library Membership is in the authoritative Suspended state and the resulting restriction is available to subsequent library operations.

### 4.3 In Scope

- Identification of the active membership.
- Identification of the business reason/condition for suspension.
- Review against configured membership policy.
- Administrative review/approval where required.
- Membership suspension.
- Restriction of library-service eligibility.
- Preservation of membership history.
- Relevant business communication outcome.

### 4.4 Out of Scope

- Student status change.
- Employee disciplinary lifecycle.
- Financial accounting or payment processing.
- Creation of new fines.
- Technical authentication or authorization implementation.
- Notification delivery implementation.
- Database/API/UI/infrastructure mechanics.

## 5. Ownership and Participating Domains

### Library Management — G

G05 owns Library Membership, including membership status, borrowing eligibility and membership history.

### Student Lifecycle — C

C remains authoritative for student identity/status. A suspension of library access does not suspend the student's institutional identity or student lifecycle.

### People & Workforce — E

E remains authoritative for employee identity/personnel lifecycle. A library suspension does not create an employee lifecycle state.

### Communication — H

H may consume the membership suspension outcome for notification. H owns delivery, not the suspension decision.

Cross-domain participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

### Library Administrator / Librarian

Reviews the business condition and establishes the membership suspension under configured library authority.

### Library Assistant / Operational Library Staff

Supports membership review and operational application of the membership outcome.

### Member

May be the subject of the suspension and may receive the resulting business communication.

### Section C Authority

Provides authoritative student context when the member is a student.

### Section E Authority

Provides authoritative employee/personnel context when required.

### H Communication Consumer

Consumes the established suspension fact where a communication is required.

## 7. Preconditions and Entry Criteria

1. An Active Library Membership exists.
2. A business condition exists that may justify suspension under configured policy.
3. The member and membership can be identified authoritatively.
4. Required review/approval authority is available.
5. The proposed suspension does not contradict a locked upstream decision.

## 8. Trigger

A configured library business condition causes an Active Library Membership to be reviewed for suspension.

## 9. Main Business Flow

### Phase A — Identify and Review

1. Identify the affected Library Membership.
2. Confirm that its current lifecycle status is Active.
3. Establish the business condition requiring suspension review.
4. Obtain authoritative member identity/status context where necessary.
5. Evaluate the condition against configured membership eligibility/restriction policy.
6. Determine whether suspension is warranted.

### Phase B — Authorise the Suspension

7. Establish the required operational approval according to configured library authority.
8. Record the suspension decision and applicable business reason.
9. Confirm the effective scope of the restriction.

### Phase C — Apply the Membership Outcome

10. Move the Library Membership from Active to Suspended.
11. Restrict the member's library access according to the applicable policy.
12. Ensure subsequent borrowing/reservation/renewal decisions respect the Suspended membership status.
13. Preserve the membership history and suspension decision.
14. Make the business outcome available to H where communication is required.

### Phase D — Complete

15. Publish the canonical membership-suspension event.
16. End the scenario with the membership in Suspended status.

The scenario does not convert suspension into closure or alter the member's upstream institutional lifecycle.

## 10. Alternate Business Flows

### 10.1 Suspension Review Determines No Suspension

If the review finds that the configured conditions are not satisfied, the membership remains Active and the review outcome is recorded.

### 10.2 Suspension Requires Additional Administrative Review

Where policy requires a higher authority, the membership remains Active until the required decision is established.

### 10.3 Member Has Outstanding Library Obligations

Outstanding obligations may be considered as part of eligibility/restriction policy because G05 explicitly permits eligibility validation to consider outstanding obligations. The scenario does not invent a new penalty or fine.

### 10.4 Student Status Is Relevant

C remains authoritative for student status. G05 consumes that information for the membership decision; it does not change the student status.

### 10.5 Employee Status Is Relevant

E remains authoritative for employee/personnel status. G05 does not create a personnel consequence.

## 11. Failure, Interruption and Business Recovery

### 11.1 Current Membership Status Cannot Be Established

The suspension is not applied until the authoritative membership status is known.

### 11.2 Eligibility/Restriction Condition Cannot Be Established

The membership remains Active until the required business information is available and a valid decision can be made.

### 11.3 Approval Is Rejected

The membership remains Active and the proposed suspension is not treated as authoritative.

### 11.4 Suspension Later Becomes No Longer Applicable

Any restoration/reactivation must follow the authoritative membership lifecycle and policy. This scenario does not invent a separate reactivation state.

### 11.5 Communication Is Required

The established suspension fact is handed to H; G05 does not own communication delivery.

No technical retry, queue, rollback or exception implementation is specified.

## 12. Business Rules and Decision Constraints

- Library Membership owns membership status.
- Membership categories and eligibility policies remain configurable.
- Eligibility may consider user status, institution, campus, programme, outstanding obligations and administrative approval.
- Supported membership status values include Pending, Active, Suspended, Expired and Cancelled, with status values remaining configurable.
- Suspension restricts library access according to the effective policy.
- Student ownership remains C-owned.
- Employee ownership remains E-owned.
- Finance owns payment processing.
- Historical membership information remains preserved.
- Authorization policies remain configurable.
- Every lifecycle transition is audited.

No unsupported suspension duration, automatic expiry period, fine, disciplinary consequence, or student/employee status change is introduced.

## 13. State and Lifecycle Impact

The G05 state transition matrix defines:

`Library Membership: Applied → Approved → Active → Suspended → Closed`

This scenario covers:

`Active → Suspended`

Suspended is the canonical membership state. No local states such as `Restricted`, `Blocked`, `Under Review`, or `Pending Suspension` are invented.

The scenario's title uses “Restricted Library Access” as the business outcome, while the authoritative aggregate state remains **Suspended**.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The G05 Membership Events catalogue defines:

- `MembershipRequested` — Membership application submitted.
- `MembershipApproved` — Membership activated.
- `MembershipSuspended` — Membership suspended.
- `MembershipClosed` — Membership closed.

The applicable event is:

**`MembershipSuspended` — published by Library Management.**

No `LibraryAccessRestricted` event is invented.

### 14.2 Synchronous Business Collaboration / Hand-offs

- C provides authoritative student context where required.
- E provides authoritative personnel context where required.
- H consumes the suspension outcome for governed communication.
- Library Management remains the canonical publisher and owner of membership status.

These collaborations do not transfer membership ownership.

## 15. Business Authorization, Approval and Audit Obligations

The G05 authorization model identifies Library Administrator and Librarian roles among the typical operational authorities and explicitly states that authorization policies remain configurable.

The suspension decision must therefore use the applicable configured library authority.

The membership history should retain the relevant business decision and its context. The G05 architecture requires immutable operational audit history and audit of lifecycle transitions.

No technical access control/IAM mechanism is defined here.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Active membership identified.
- Suspension condition established.
- Applicable eligibility/restriction policy evaluated.
- Required approval completed.
- Membership moved to Suspended.
- Library access is restricted according to policy.
- Membership history preserved.
- `MembershipSuspended` published.
- Required communication business outcome made available.

### Postconditions

- Membership status is Suspended.
- Subsequent library eligibility checks recognise the suspended membership.
- The member's C/E institutional lifecycle remains unchanged.
- Membership history remains traceable and immutable.
- H may communicate the outcome without becoming membership owner.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership, lifecycle or publisher conflict identified.

### Assumptions

- Exact suspension criteria are configuration driven.
- The exact duration of suspension is not defined by the retrieved L2 and therefore is not invented.
- Access restrictions are determined by the effective library policy.

### Deferred Questions

Any automatic reactivation rule, suspension duration, or specialised restriction state must be established upstream before being introduced into L3.
