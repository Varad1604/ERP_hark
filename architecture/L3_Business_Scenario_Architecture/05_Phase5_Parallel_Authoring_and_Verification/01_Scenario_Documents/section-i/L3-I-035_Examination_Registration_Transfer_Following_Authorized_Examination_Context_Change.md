# Examination Registration Transfer Following Authorized Examination Context Change

**Scenario ID:** L3-I-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination  
**Participating Domains:** C — Student Lifecycle, I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I02 Examination Session Management Workflows; L2-I04 Examination Registration Workflows; applicable Student Lifecycle architecture  
**Referenced By:** L2-I05 Hall Ticket Management and subsequent Examination workflows where the transferred registration is consumed

## 2. Scenario Purpose

Govern an authorised change to the examination context of an existing candidate Examination Registration so that the candidate's valid examination participation follows the authorised context change without creating competing or stale registration authority.

The scenario preserves the distinction between Student Lifecycle ownership of the underlying academic context and Examination ownership of Examination Registration.

## 3. Business Outcome

The candidate's Examination Registration is established against the authorised examination context, with the superseded registration position no longer acting as the current authority for examination participation and the complete business history preserved.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authorised business change establishes that an existing Examination Registration must be associated with a different examination context within the permitted institutional process.

**In Scope:** identification of the existing registration; validation of the authorised context change; confirmation of the candidate and affected examination context; determination of the resulting registration position; application of the existing Examination Registration lifecycle; preservation of the superseded position; audit; and downstream availability of the resulting valid registration.

**Out of Scope:** changing Student Lifecycle ownership; changing Examination Policy; redefining Examination Session governance; changing curriculum ownership; financial transaction processing; Hall Ticket issuance itself; examination conduct; and technical implementation mechanics.

**End Boundary:** The candidate has one authoritative Examination Registration position for the authorised examination context and the superseded position cannot independently authorise downstream examination processing.

## 5. Ownership and Participating Domains

**Student Lifecycle (C):** owns the authoritative student/academic context that may give rise to the authorised change.

**Examination (I):** owns Examination Registration and determines the registration consequence of the authorised context change.

No context transfer changes ownership of Student Lifecycle information. Examination consumes the authoritative upstream change and governs the resulting registration.

## 6. Business Actors, Roles and Responsibilities

- **Examination Administrator:** reviews the affected registration and applies the authorised registration consequence.
- **Controller of Examination:** provides examination governance and required approval for material registration changes.
- **Candidate (Student):** may participate in the authorised context-change process where required by institutional rules.
- **Student Lifecycle / Academic Administration:** establishes the authoritative upstream context change.
- **Audit Infrastructure:** preserves the registration history and decision evidence.

No new permanent actor or technical role is introduced.

## 7. Preconditions and Entry Criteria

- A valid Examination Registration exists.
- An authorised examination-context change has been established by the relevant business authority.
- The candidate identity is unambiguous.
- The affected Examination Session and registration scope can be identified.
- The resulting context is valid for examination registration under applicable policy.
- The existing L2-I04 registration lifecycle remains applicable.

## 8. Trigger

An authorised change to the examination context is established and requires the candidate's existing Examination Registration to reflect the new context.

## 9. Main Business Flow

1. Identify the candidate and the current Examination Registration affected by the authorised context change.
2. Establish the authoritative upstream context change and its effective business meaning.
3. Determine the Examination Session, registration items, examination category and other registration scope affected by the change.
4. Validate that the resulting examination context is eligible for registration under applicable Examination Policy and Student Lifecycle rules.
5. Determine whether the existing registration can be amended within the permitted lifecycle or whether the existing registration must reach another authorised disposition before the resulting registration position is established.
6. Apply the applicable existing L2-I04 registration transition or amendment.
7. Preserve the prior registration position and the business reason for the context change.
8. Where required, obtain the applicable examination approval before the resulting registration becomes confirmed.
9. Confirm the resulting Examination Registration when all required conditions are satisfied.
10. Publish the applicable canonical registration lifecycle event(s).
11. Ensure downstream examination workflows use the resulting authoritative registration position and do not continue from the superseded context.

## 10. Alternate Business Flows

### 10.1 Context Change Does Not Affect Examination Registration

If the authorised change does not alter the candidate's examination participation context, the existing registration remains authoritative and no transfer is performed.

### 10.2 Context Change Requires Only Registration Amendment

Where L2-I04 permits the affected change as an amendment before locking, the registration is amended through the existing amendment lifecycle.

### 10.3 Context Change Occurs Before Confirmation

Where the registration has not yet been confirmed, the change is incorporated before confirmation, subject to the existing eligibility and approval requirements.

### 10.4 Context Change Requires Reopening

Where the authorised process permits reopening of an otherwise unavailable registration position, the existing registration-reopening lifecycle is used. No new reopening state is created by this scenario.

### 10.5 Context Change Cannot Be Applied

If the resulting context is not valid for examination registration, the existing registration is not transferred to the invalid context. The candidate remains governed by the applicable existing registration disposition.

## 11. Failure, Interruption and Business Recovery

### 11.1 Upstream Change Is Not Authoritative

If the purported context change has not been established by the owning Student Lifecycle authority, Examination does not act on it.

### 11.2 Resulting Context Is Ineligible

If the new context fails authoritative eligibility requirements, the registration does not become valid in that context.

### 11.3 Registration Is Already Locked

A locked registration is not routinely modified. Any change follows the authorised exceptional process applicable to locked registrations.

### 11.4 Multiple Resulting Registration Positions

If the context change would create more than one current valid registration for the same examination participation scope, the duplicate/conflicting registration controls apply and one authoritative registration position must remain.

### 11.5 Downstream Examination Processing Has Begun

If the superseded registration has already been consumed by Hall Ticket or examination operations, the registration correction is coordinated with those owning workflows. This scenario does not take ownership of their lifecycle.

## 12. Business Rules and Decision Constraints

- Student Lifecycle remains authoritative for the underlying academic context change.
- Examination remains authoritative for Examination Registration.
- A registration transfer must not create competing current registration authority.
- Existing L2-I04 registration rules govern eligibility, amendment, confirmation, locking and withdrawal.
- The resulting registration must satisfy applicable examination and academic conditions.
- Historical registration information must remain traceable.
- No new transfer-specific lifecycle state or business event is introduced without upstream architecture authority.
- A superseded registration position must not independently authorise downstream examination participation.

## 13. State and Lifecycle Impact

The scenario uses the existing L2-I04 Registration State Model and its governed transitions:

`Draft → Pending Validation → Pending Review → Approved → Confirmed → Locked`

with existing amendment, withdrawal, reopening, completion and archival positions where applicable.

The word **transfer** describes the institutional outcome. It is not introduced as a new lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

Student Lifecycle is the authoritative source for the underlying academic/context change.

Examination Registration remains the canonical publisher of registration lifecycle events defined by L2-I04, including:

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

No `RegistrationTransferred` event is invented.

The collaboration is a business hand-off: the authoritative context change is consumed by Examination, which determines and governs the registration consequence.

## 15. Business Authorization, Approval and Audit Obligations

- The underlying context change must be authorised by the owning Student Lifecycle process.
- Examination registration changes must be performed by an authorised Examination business actor.
- Required examination approval remains applicable.
- The reason for the context change, prior registration position, resulting registration position and responsible business authority must remain auditable.
- If an exceptional post-lock change is required, the applicable institutional approval must be obtained.
- No technical access control/IAM design is specified.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The upstream context change is authoritative.
- The affected registration is identified.
- The resulting examination context is valid.
- The applicable existing registration lifecycle transition is completed.
- Required approval is complete.
- The resulting registration is confirmed or otherwise reaches the applicable valid terminal/current position.
- The superseded position is no longer current examination participation authority.
- Relevant registration events and audit evidence are preserved.
- Downstream workflows have one authoritative registration position.

**Postconditions:**
- Examination Registration reflects the authorised examination context.
- Student Lifecycle ownership remains unchanged.
- The prior registration position remains historically traceable.
- Downstream examination processing can rely on the resulting authoritative registration.

## 17. Open Issues, Assumptions and Deferred Questions

1. **Upstream semantic gap:** L2-I04 defines registration amendment, withdrawal, reopening and related lifecycle controls, but the available locked L2 source does not define a dedicated `Registration Transfer` lifecycle transition or event. This scenario therefore treats “transfer” as the business outcome and applies only existing registration transitions.
2. If a locked upstream decision defines a specific transfer mechanism, that upstream definition takes precedence.
3. The Stage 3 row fixes Type = **Critical**, Priority = **P1**, Primary Owner = **Examination Registration**, and Participating Domains = **C, I**. These frozen semantics are preserved. 
4. If the authorised context change materially affects Hall Ticket or downstream examination records already created, those consequences belong to their owning workflows and must not be absorbed into this scenario.
