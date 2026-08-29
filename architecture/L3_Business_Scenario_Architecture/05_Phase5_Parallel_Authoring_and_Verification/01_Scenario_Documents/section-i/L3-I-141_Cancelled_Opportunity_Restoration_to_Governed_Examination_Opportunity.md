# Cancelled Opportunity Restoration to Governed Examination Opportunity

**Scenario ID:** L3-I-141  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I12 Supplementary / Backlog / Improvement Examination Execution; L2-I15 Examination Exception & Contingency; L2-I02 Examination Session Management; L2-I03 Examination Planning & Scheduling  
**Referenced By:** Downstream examination and academic workflows consuming the authoritative outcome

## 1. Scenario Metadata

The frozen Stage 3 row fixes the canonical title, owner, participating domains, Type and Priority shown above. No reclassification is introduced.

## 2. Scenario Purpose

Restore a previously cancelled Supplementary, Backlog or Improvement examination opportunity only after an authoritative institutional decision permits restoration, while preserving the historical cancellation and all canonical ownership boundaries.

## 3. Business Outcome

A cancelled subsequent-examination opportunity is re-established as a governed valid examination opportunity with an applicable session and schedule, and the downstream registration/examination path can resume through canonical workflows.

## 4. Scenario Boundary and Scope

**Start Boundary:** A subsequent-examination opportunity was cancelled and a new authorized decision permits restoration.

**In Scope:** verify restoration decision; determine session usability; establish schedule; re-enter the subsequent-examination opportunity into its valid lifecycle; route registration and Hall Ticket consequences through canonical workflows; preserve cancellation/restoration history.

**Out of Scope:** independently reversing the original cancellation; directly modifying Registration/Hall Ticket; deciding candidate eligibility; examination conduct; results; or technical restoration.

## 5. Ownership and Participating Domains

**Supplementary, Backlog & Improvement Examination:** owns the opportunity and restoration decision within its administrative scope.

**L2-I15:** owns the contingency context where restoration follows an exception.

**L2-I02:** owns Examination Session lifecycle.

**L2-I03:** owns Examination Schedule lifecycle.

Restoration never transfers ownership of session, schedule, registration or Hall Ticket.

## 6. Business Actors, Roles and Responsibilities

- Controller of Examination: governs restoration authority.
- Supplementary Examination Officer: coordinates the restored opportunity.
- Examination Administrator: assesses impact and initiates canonical session/schedule steps.
- L2-I02/L2-I03: establish valid session and schedule.
- L2-I15: provides exception-governance context when applicable.

## 7. Preconditions and Entry Criteria

- A previously cancelled opportunity exists.
- Cancellation history is authoritative and traceable.
- A new authorized restoration decision exists.
- Candidate/examination scope can be established.
- Applicable session and schedule can be revalidated or established through L2-I02/L2-I03.

## 8. Trigger

An authoritative institutional decision permits restoration of the previously cancelled subsequent-examination opportunity.

## 9. Main Business Flow

1. Identify cancelled opportunity and historical cancellation.
2. Establish authorized restoration decision and business basis.
3. Determine whether the original Examination Session remains usable.
4. If needed, route session action through L2-I02.
5. Establish scheduling requirement through L2-I03.
6. Create/revise/publish the authoritative schedule as required.
7. Re-enter the opportunity into its valid L2-I12 lifecycle position.
8. Route candidate registration through L2-I04 where the prior registration consequence requires re-registration.
9. Route Hall Ticket consequences through L2-I05 when the restored opportunity becomes operational.
10. Preserve cancellation and restoration history.
11. Publish applicable session, schedule and subsequent-examination events.
12. End when the restored opportunity is operationally governed for further registration/examination processing.

## 10. Alternate Business Flows

**Original session remains valid:** restoration uses the existing session with the appropriate schedule lifecycle.

**New session required:** L2-I02 establishes the successor session.

**New schedule required:** L2-I03 establishes the authoritative operational timetable; the cancelled schedule remains historical.

**Registration must be re-established:** L2-I04 determines the valid registration path; historical registration is not silently resurrected.

## 11. Failure, Interruption and Business Recovery

**Restoration decision not authoritative:** the cancelled position remains authoritative.

**Session cannot be revalidated:** restoration waits for a valid L2-I02 session position.

**Schedule cannot be established:** restored opportunity is not operationally available.

**Candidate registration invalid:** a valid Registration must be established through L2-I04.

**Restoration later revoked:** the prevailing cancellation/contingency governance applies and the restoration attempt remains historical.

## 12. Business Rules and Decision Constraints

- Cancellation does not disappear merely because conditions improve.
- Restoration requires an authorized institutional decision.
- Session, Schedule and Registration remain owned by L2-I02/L2-I03/L2-I04.
- Historical cancellation must remain preserved.
- Restored opportunity becomes operational only after required session/schedule conditions are satisfied.
- No local restoration state is created.

## 13. State and Lifecycle Impact

The scenario re-enters the existing L2-I12 opportunity lifecycle through a valid governed position. L2-I02, L2-I03 and L2-I04 retain their independent lifecycles. The historical cancelled position remains traceable; no new `Restored` state is invented.

## 14. Business Events and Cross-Domain Collaboration

L2-I15 publishes contingency facts where applicable; L2-I02 publishes session lifecycle facts; L2-I03 publishes schedule events such as `Schedule Published` or `Schedule Revised`; L2-I04 publishes registration events when candidates are re-registered; L2-I12 publishes subsequent-examination lifecycle milestones.

No `OpportunityRestored` event is invented.

## 15. Business Authorization, Approval and Audit Obligations

- Restoration requires applicable institutional authority.
- Original cancellation and restoration authorization must remain auditable.
- Session/schedule/registration changes are traceable in their canonical workflows.
- Restoration must not erase cancellation history.
- Exceptional administrative action requires appropriate authority and justification.

## 16. Completion Criteria and Postconditions

**Complete when:** cancelled opportunity and restoration decision are established; valid session and schedule are available; opportunity is re-entered into the governed lifecycle; required registration path can proceed; cancellation/restoration history is preserved; applicable events are published.

**Postconditions:** a valid governed subsequent-examination opportunity exists; the cancellation remains historical; downstream registration and examination workflows can proceed from the restored opportunity.

## 17. Open Issues, Assumptions and Deferred Questions

1. The frozen Stage 3 outcome is restoration to a governed opportunity. The accessible L2-I12 material does not define a dedicated restoration state/event, so the existing lifecycle is reused.
2. Exact institutional restoration authority and permissible conditions remain governed by Examination Policy/L2-I15.
3. Historical Registration and Hall Ticket consequences depend on the original cancellation path and remain owned by L2-I04/L2-I05.
