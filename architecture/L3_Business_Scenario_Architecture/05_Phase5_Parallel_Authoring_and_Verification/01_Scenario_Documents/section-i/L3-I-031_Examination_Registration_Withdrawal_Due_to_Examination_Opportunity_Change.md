# Examination Registration Withdrawal Due to Examination Opportunity Change

**Scenario ID:** L3-I-031  
**Participating Domains:** A, C, D, I  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I01 Examination Policy & Configuration Workflows; L2-I02 Examination Session Management Workflows; L2-I03 Examination Planning & Scheduling Workflows; L2-I04 Examination Registration Workflows; L2-I05 Hall Ticket Management Workflows; L2-I15 Examination Exception & Contingency Workflows; applicable Student and Academic Delivery architecture  
**Referenced By:** L2-I05 Hall Ticket Management and downstream examination operations

## 2. Scenario Purpose

Govern the controlled withdrawal or invalidation of examination participation when an authorised change to the underlying examination opportunity makes an existing registration no longer valid or no longer applicable.

The scenario preserves the Examination Registration history and ensures that a changed examination opportunity does not silently leave obsolete candidate participation available to downstream examination processing.

## 3. Business Outcome

The affected Examination Registration is withdrawn or otherwise receives the source-defined governed outcome for the changed examination opportunity, its historical participation remains traceable, and downstream examination workflows no longer treat the affected participation as an eligible current registration.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authorised change to an examination opportunity affects an existing Examination Registration.

**In Scope:** identify affected registrations; establish the authoritative opportunity change; determine impact; withdraw affected registration scope where required; preserve history; publish canonical registration withdrawal facts; and coordinate the resulting registration position with downstream examination workflows.

**Out of Scope:** changing examination policy; independently changing the examination schedule; changing the Student record; hall-ticket mechanics; examination conduct; financial settlement; and technical recovery.

**End Boundary:** All affected registrations have the source-defined authoritative withdrawal outcome and are excluded from downstream processing where required.

## 5. Ownership and Participating Domains

**Examination (I):** owns Examination Registration and the withdrawal outcome.

**Platform Foundation (A):** provides audit/authorization capabilities.

**Student Lifecycle (C):** provides candidate context and remains owner of student state.

**Academic Delivery (D):** provides academic context where the opportunity change affects academic applicability.

The opportunity change itself remains owned by the authoritative examination planning/session/policy workflow that produced it. Examination Registration owns the registration consequence.

## 6. Business Actors, Roles and Responsibilities

- **Examination Administrator:** identifies and manages affected registrations.
- **Controller of Examination:** governs examination opportunity changes and registration decisions.
- **Academic Administration:** supplies academic context where required.
- **Candidate (Student):** receives or participates in the resulting registration outcome.
- **Student Domain:** supplies authoritative candidate context.
- **Platform Audit Infrastructure:** preserves business audit evidence.

The scenario does not invent an actor for the underlying schedule/session decision.

## 7. Preconditions and Entry Criteria

- An Examination Registration exists and is authoritative.
- The relevant examination opportunity has been authoritatively changed, withdrawn, cancelled, rescheduled, or otherwise made inapplicable.
- The affected candidate and registration scope can be identified.
- The applicable Examination Registration policy defines the required consequence.
- Downstream examination processing has not been allowed to continue against an obsolete participation position without the applicable exception governance.

## 8. Trigger

An authoritative examination opportunity change is established and its impact assessment identifies one or more existing Examination Registrations as affected.

## 9. Main Business Flow

1. Identify the authoritative examination opportunity change.
2. Identify the affected Examination Session, examination scope, subjects, and candidate registrations.
3. Determine which registrations are no longer valid or applicable under the changed opportunity.
4. Apply the Examination Registration withdrawal or other source-defined consequence.
5. Preserve the prior registration and the reason for the changed-outcome decision.
6. Publish **Registration Withdrawn** when withdrawal is established.
7. Ensure downstream examination workflows use the resulting registration position rather than the obsolete registration.
8. Where policy provides an alternative examination opportunity or rescheduling path, route the candidate through that governed opportunity without silently reusing the withdrawn registration.

## 10. Alternate Business Flows

### 10.1 Entire Opportunity Withdrawn

All registrations affected by the withdrawn opportunity receive the applicable governed withdrawal outcome.

### 10.2 Partial Opportunity Change

Only registrations tied to the affected portion of the examination opportunity are withdrawn; unaffected registrations remain authoritative.

### 10.3 Rescheduled Opportunity

Where the underlying opportunity is rescheduled rather than cancelled, registration remains subject to the applicable policy and may continue if the authoritative registration remains valid.

### 10.4 Replacement Opportunity

Where policy establishes a replacement examination opportunity, affected candidates follow the separate governed registration path for that opportunity.

## 11. Failure, Interruption and Business Recovery

### 11.1 Opportunity Change Is Not Authoritative

No registration is withdrawn on the basis of a proposed or unconfirmed opportunity change.

### 11.2 Affected Registration Cannot Be Determined

If the affected registration scope cannot be established, the authoritative registration remains unchanged until the business relationship is resolved.

### 11.3 Registration Already Locked

A locked registration is not silently changed. The applicable exceptional or contingency process governs the authorised withdrawal/correction.

### 11.4 Hall Ticket Already Exists

Where downstream hall-ticket processing has already occurred, the registration withdrawal must be coordinated with the applicable Hall Ticket Management and exception workflows. This scenario does not redefine hall-ticket lifecycle ownership.

### 11.5 Candidate Requires a New Opportunity

The candidate does not automatically inherit the withdrawn registration. A new or replacement opportunity follows its own governed registration process.

## 12. Business Rules and Decision Constraints

- Registration is authoritative only for an applicable examination opportunity.
- Significant registration activities, including withdrawal, are auditable.
- Withdrawal must follow the applicable registration and examination-opportunity policy.
- Existing registration history is preserved.
- A withdrawn registration is not treated as an eligible current registration for downstream examination processing.
- Examination Registration remains the canonical owner of registration state.
- The underlying examination session/schedule/opportunity remains owned by its canonical examination workflow.
- A changed opportunity does not transfer ownership of Student or Academic Delivery information.

## 13. State and Lifecycle Impact

The scenario applies the existing Registration Withdrawal capability in L2-I04.

No new registration state is created.

Where the registration is already locked or the change occurs after downstream processing has begun, the applicable exceptional/contingency workflow governs the business consequence. The scenario does not invent a new lifecycle transition.

## 14. Business Events and Cross-Domain Collaboration

**Registration Withdrawn** is the canonical registration event identified by L2-I04 and is published by Examination Registration.

Other registration events remain applicable only when their corresponding business milestones occur.

Downstream Hall Ticket Management consumes the resulting registration position. The examination session/schedule workflow remains responsible for publishing or establishing the underlying opportunity change.

No new `OpportunityChanged` or `RegistrationInvalidated` event is invented in this scenario.

## 15. Business Authorization, Approval and Audit Obligations

- The underlying opportunity change must be authoritative before registration consequences are applied.
- Withdrawal follows the applicable Examination Registration policy and authorised examination governance.
- The business reason and affected scope must remain auditable.
- Any downstream hall-ticket consequence remains subject to Hall Ticket Management authority.
- Platform Foundation provides audit and authorization infrastructure without owning the registration decision.
- No technical IAM/RBAC design is specified.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The authoritative opportunity change is established.
- All affected registrations are identified.
- The applicable withdrawal/outcome is established.
- Registration history and business reason are preserved.
- **Registration Withdrawn** is published for withdrawn registrations.
- Downstream workflows can identify the affected participation as no longer current.

**Postconditions:**
- The affected registration is no longer an eligible current registration for the changed opportunity.
- Historical registration information remains traceable.
- Any replacement or rescheduled opportunity follows its own governed lifecycle.
- Ownership of the underlying opportunity, student context, and registration remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 explicitly supports Registration Withdrawal and exceptional workflows, while L2-I15 provides examination exception/contingency governance. The exact post-opportunity-change treatment is policy-dependent where the source does not define one universal path.
2. No dedicated `RegistrationInvalidated` event is introduced; **Registration Withdrawn** is the canonical withdrawal fact available from L2-I04.
3. The exact frozen Stage 3 Type/Priority row was not independently retrievable from the uploaded register during this run; the metadata above should be mechanically rechecked against the frozen Stage 3 row before Review.
