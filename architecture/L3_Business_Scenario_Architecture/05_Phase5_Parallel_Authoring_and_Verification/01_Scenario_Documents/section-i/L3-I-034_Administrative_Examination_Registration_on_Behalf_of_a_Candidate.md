# Administrative Examination Registration on Behalf of a Candidate

**Scenario ID:** L3-I-034  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination  
**Participating Domains:** Examination, Student Lifecycle, Academic Delivery, Finance  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I01 Examination Policy & Configuration Workflows; L2-I02 Examination Session Management Workflows; L2-I03 Examination Planning & Scheduling Workflows; L2-I04 Examination Registration Workflows; applicable Student Lifecycle, Academic Delivery, Finance and Platform Foundation architecture  
**Referenced By:** L2-I05 through L2-I16 where the resulting Examination Registration is consumed

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-034 |
| Canonical Title | Administrative Examination Registration on Behalf of a Candidate |
| Primary Owner | Examination |
| Participating Domains | Examination, Student Lifecycle, Academic Delivery, Finance |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Enable an authorised institutional administrator to complete Examination Registration on behalf of a candidate where institutional policy permits assisted or administrative registration, while preserving the candidate's examination participation as the authoritative Examination Registration owned by Examination.

## 3. Business Outcome

A valid Examination Registration is established for the candidate through an authorised administrative action, with the same eligibility, scope, approval, confirmation, audit and downstream requirements that govern candidate-initiated registration.

## 4. Scenario Boundary and Scope

**Start Boundary:** A candidate requires or is permitted to receive administrative assistance with Examination Registration and an authorised institutional actor is available to perform the registration.

**In Scope:** candidate identification; establishment of the applicable examination session and subjects; eligibility validation; registration entry on the candidate's behalf; required review/approval; confirmation; audit; and downstream availability.

**Out of Scope:** candidate identity creation; student-status changes; examination policy definition; academic curriculum changes; financial transaction processing; hall-ticket generation; examination conduct; and technical authorization design.

**End Boundary:** The candidate's valid Examination Registration is confirmed and available for downstream examination processing, or the registration is rejected/withheld because an applicable business condition is not satisfied.

## 5. Ownership and Participating Domains

**Examination:** owns Examination Registration and the resulting examination participation.

**Student Lifecycle:** provides authoritative candidate/student context.

**Academic Delivery / Curriculum:** provides academic and subject applicability.

**Finance:** provides financial eligibility where required.

The administrator acts on behalf of the candidate but does not become the owner of the candidate's Examination Registration.

## 6. Business Actors, Roles and Responsibilities

- **Examination Administrator:** performs the registration on behalf of the candidate where permitted.
- **Candidate (Student):** is the subject of the registration and provides/authorises the required information where institutional policy requires candidate participation.
- **Controller of Examination:** governs examination registration and required approvals.
- **Academic Administration:** validates academic applicability where required.
- **Student Domain:** provides authoritative student context.
- **Finance Domain:** provides financial eligibility where required.

The exact delegation/assisted-registration authority remains governed by the applicable institutional process; no new role is created here.

## 7. Preconditions and Entry Criteria

- An Active Examination Session exists.
- An approved Examination Schedule has been published.
- The candidate has an authoritative Student context.
- The administrator is authorised under institutional policy to act on behalf of the candidate.
- Applicable examination subjects and category can be established.
- Eligibility requirements are available.
- Registration occurs within an approved registration window or authorised exception.

## 8. Trigger

A candidate's examination registration requirement is routed to an authorised institutional administrator for completion on the candidate's behalf.

## 9. Main Business Flow

1. Identify the candidate and establish the authority for administrative registration.
2. Establish the applicable Examination Session, Schedule, academic context, registration category and subjects.
3. Validate the candidate's examination eligibility.
4. Validate any required financial eligibility.
5. Enter the Examination Registration on behalf of the candidate within the permitted registration scope.
6. Complete the required registration review.
7. Obtain required institutional approval.
8. Confirm the Examination Registration.
9. Record that the registration was completed administratively and preserve the responsible business actor and applicable business justification.
10. Publish the applicable canonical registration events.
11. Make the confirmed registration available to Hall Ticket Management and other downstream examination workflows.

## 10. Alternate Business Flows

### 10.1 Candidate Provides Required Confirmation

Where institutional policy requires candidate confirmation before administrative completion, the administrator obtains that confirmation before the registration is confirmed.

### 10.2 Institutionally Assisted Registration

Where an institution provides registration assistance as a standard service, the administrator completes the registration within the normal registration window and normal eligibility rules.

### 10.3 Administrative Registration Under an Approved Exception

Where policy permits administrative registration under a special circumstance, the administrator follows the applicable exception approval while retaining the standard Examination Registration lifecycle.

### 10.4 Candidate Information Requires Correction

If candidate or registration information is incomplete or incorrect, the registration remains in the appropriate existing lifecycle position until the authoritative information is corrected.

## 11. Failure, Interruption and Business Recovery

### 11.1 Administrative Authority Cannot Be Established

Registration does not proceed until the administrator's authority to act on behalf of the candidate is established under institutional policy.

### 11.2 Candidate Eligibility Fails

The administrative channel does not bypass eligibility. The registration remains rejected or pending according to the applicable policy.

### 11.3 Registration Window Closed

Administrative registration after the normal window requires an authorised late, extended or special registration path where policy permits.

### 11.4 Candidate or Academic Context Is Inconsistent

The administrator does not override authoritative Student or Academic information. The relevant owning domain must establish the authoritative context.

### 11.5 Registration Is Already Locked

A locked registration is not routinely modified. Any required administrative correction follows the authorised exceptional process.

### 11.6 Registration Is Duplicate

The duplicate-registration workflow applies; administrative origin does not create a second valid participation record.

## 12. Business Rules and Decision Constraints

- Administrative registration is permitted only where institutional policy authorises it.
- Acting on behalf of a candidate does not transfer Examination Registration ownership.
- The same eligibility, subject, category, registration-window and approval requirements apply to administrative registration.
- An administrator must not bypass financial eligibility where financial eligibility is required.
- Administrative registration must remain auditable.
- The candidate remains the registered examination participant.
- Existing L2-I04 lifecycle states and events govern the registration.
- No separate administrative-registration lifecycle is created.

## 13. State and Lifecycle Impact

Administrative registration uses the standard L2-I04 lifecycle:

`Draft → Pending Validation → Pending Review → Approved → Confirmed → Locked`

with existing `Amended`, `Withdrawn`, `Completed` and `Archived` positions as applicable.

The fact that an administrator performed the registration is business provenance within the registration activity; it does not create a new registration state.

## 14. Business Events and Cross-Domain Collaboration

Examination Registration publishes the existing canonical events defined by L2-I04:

- **Registration Initiated**
- **Eligibility Validated**
- **Registration Submitted**
- **Registration Approved**
- **Registration Rejected**
- **Registration Confirmed**
- **Registration Amended**
- **Registration Withdrawn**
- **Registration Locked**
- **Registration Archived**

No new `AdministrativeRegistrationCompleted` event is invented.

Student Lifecycle, Academic Delivery/Curriculum and Finance provide their authoritative business information without assuming ownership of Examination Registration.

## 15. Business Authorization, Approval and Audit Obligations

- The administrator must have source-supported authority to act on behalf of the candidate.
- Required examination approval remains applicable.
- Where policy requires candidate confirmation, that confirmation must be obtained before the relevant business milestone.
- Administrative origin, responsible actor and business justification must remain auditable.
- Platform Foundation supplies authorization and audit infrastructure but does not own the registration decision.
- No technical access control/IAM implementation is specified.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- Candidate identity and authority for administrative action are established.
- Examination Session and Schedule are valid.
- Eligibility is satisfied.
- Applicable subjects and registration category are established.
- Required approval is complete.
- Registration is confirmed.
- Administrative provenance and audit evidence are preserved.
- Applicable registration events are published.
- Confirmed registration is available to downstream examination workflows.

**Postconditions:**
- The candidate has one authoritative Examination Registration.
- The registration is indistinguishable in lifecycle authority from a valid candidate-initiated registration.
- Administrative action is traceable.
- Downstream examination workflows can consume the confirmed registration.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 explicitly names authorised institutional administrators as participants in registration withdrawal and exceptional administration, but the precise institutional delegation model for initiating a full registration on behalf of a candidate is not separately defined in the available L2 source. This scenario therefore relies on the locked policy principle that institutional workflows may be configured while avoiding invention of a new permanent role.
2. The exact frozen Stage 3 Type/Priority values are not exposed in the available canonical-register search results. The values in Section 1 are therefore a Phase 5 classification interpretation and must be mechanically reconciled against the frozen Stage 3 row before the document can advance beyond Draft.
3. If Stage 3 defines a specific actor or approval authority for administrative registration, that frozen upstream definition takes precedence over this scenario wording.
