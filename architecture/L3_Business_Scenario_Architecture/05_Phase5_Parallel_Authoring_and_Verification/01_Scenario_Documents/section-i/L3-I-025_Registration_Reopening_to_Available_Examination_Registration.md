# Registration Reopening to Available Examination Registration

**Scenario ID:** L3-I-025  
**Participating Domains:** A, B, I  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2-I01 Examination Policy and Configuration Workflows; L2-I02 Examination Session Management Workflows; L2-I04 Examination Registration Workflows; Section A Authorization and Audit architecture  
**Referenced By:** Examination Registration and downstream examination workflows; None yet

## 2. Scenario Purpose

Govern institutional reopening of an examination registration opportunity after it has closed or become unavailable, where governing authority permits reopening.

## 3. Business Outcome

An authorised registration opportunity is reopened for the applicable examination context, making valid registration available again while preserving policy, session, ownership and audit boundaries.

## 4. Scenario Boundary and Scope

**Start:** A registration opportunity is closed/unavailable and an authorised need to reopen exists.  
**In scope:** reason, policy/session validation, institutional decision, reopened availability, audit, and resumed registration.  
**Out of scope:** redesigning policy, creating a session, revising the examination schedule, candidate-specific registration, or technical implementation.  
**End:** opportunity is reopened and available, or remains closed after a governed decision.

## 5. Ownership and Participating Domains

**Examination Registration (I):** owns registration availability and reopening outcome. **Institution & Structure (B):** provides institutional/academic-calendar context where applicable. **Platform Foundation (A):** provides authoritative authorization and audit capabilities without owning the business decision.

## 6. Business Actors, Roles and Responsibilities

Controller of Examination governs registration policy and decisions where applicable; Examination Administrator manages registration operations; Academic Administration may provide institutional/academic context; an Authorised Institutional Officer may provide required exceptional authority where defined upstream.

## 7. Preconditions and Entry Criteria

An applicable Examination Session exists and permits registration; registration context is identifiable; opportunity is closed/unavailable; a business reason exists; policy permits or governs reopening; required authority is available; reopening does not require an unapproved change to a locked upstream decision.

## 8. Trigger

An authorised institutional request is raised to reopen a closed or unavailable examination registration opportunity.

## 9. Main Business Flow

1. Identify the closed registration opportunity and its session/policy context.  2. Establish business reason and affected registration period/scope.  3. Evaluate against Examination Policy and Examination Session governance.  4. Obtain authorised institutional decision.  5. Reopen the opportunity for the approved scope.  6. Establish effective registration availability and conditions.  7. Preserve prior closure and reopening decision in audit history.  8. Resume normal Examination Registration for valid candidates.

## 10. Alternate Business Flows

**Defined additional period:** reopen for an approved additional registration period.  
**Restricted scope:** reopen only for the approved institutional/examination context.  
**Exceptional reopening:** apply required authority and justification.  
**Existing registrations:** valid registrations remain authoritative; reopening does not invalidate them.

## 11. Failure, Interruption and Business Recovery

If policy does not permit reopening, retain closure. If authority is not granted, retain closure. If the session no longer supports registration, do not reopen through this scenario. If reopening would require an unapproved policy change, route through the upstream policy governance process rather than silently changing policy.

## 12. Business Rules and Decision Constraints

Registration normally occurs only within approved windows; reopening must remain within policy and session governance; reopening does not bypass candidate eligibility, subject applicability, financial eligibility where required, or other registration requirements; existing valid registrations remain authoritative; Section A retains authorization/audit ownership; Section B retains institutional/calendar ownership.

## 13. State and Lifecycle Impact

Changes the business availability of the registration opportunity within registration-window governance. No new Examination Registration lifecycle state is invented. Candidate registrations created after reopening use the existing lifecycle. Existing valid registrations are not retroactively changed.

## 14. Business Events and Cross-Domain Collaboration

L2-I04 defines registration lifecycle events but no canonical **Registration Reopened** event. Therefore none is invented. Policy/session information is consumed from L2-I01/L2-I02 while ownership remains there. Subsequent candidate registration events are published by Examination Registration.

## 15. Business Authorization, Approval and Audit Obligations

Reopening requires institutional authority under applicable policy; exceptional reopening requires business justification; prior closure and reopening decision remain traceable and auditable; Section A retains enterprise authorization/audit infrastructure ownership; no technical access control/IAM is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** policy/session requirements pass, required authority approves, opportunity is available for valid registration, and decision/history are auditable. **Afterward:** candidates may enter normal registration; existing registrations remain authoritative; all normal eligibility requirements continue to apply.

## 17. Open Issues, Assumptions and Deferred Questions

L2-I04 defines registration windows and late/extended/special registration, while Stage 3 explicitly defines reopening. Available L2-I04 material does not define a dedicated reopened event/state; reopening is therefore treated as governed availability of the existing registration workflow. Exact authority/circumstances remain institution-specific. If reopening requires a locked upstream policy/session change, that upstream workflow must govern it first.

