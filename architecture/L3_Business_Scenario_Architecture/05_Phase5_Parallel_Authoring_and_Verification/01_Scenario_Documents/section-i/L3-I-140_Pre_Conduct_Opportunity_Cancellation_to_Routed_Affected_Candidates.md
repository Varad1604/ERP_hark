# Pre-Conduct Opportunity Cancellation to Routed Affected Candidates

**Scenario ID:** L3-I-140  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I12 Supplementary / Backlog / Improvement Examination Execution; L2-I15 Examination Exception & Contingency; L2-I03 Examination Planning & Scheduling; L2-I04 Examination Registration; L2-I05 Hall Ticket Management  
**Referenced By:** Downstream examination and academic workflows consuming the authoritative outcome

## 1. Scenario Metadata

The frozen Stage 3 row fixes the canonical title, owner, participating domains, Type and Priority shown above. No reclassification is introduced.

## 2. Scenario Purpose

Govern cancellation of a Supplementary, Backlog or Improvement examination opportunity before conduct begins and route every affected candidate into the appropriate authoritative institutional consequence.

## 3. Business Outcome

The affected subsequent-examination opportunity is authoritatively cancelled before conduct, all affected participants are identified, and each candidate receives a governed downstream route such as replacement/rescheduling or another policy-defined outcome. The cancelled opportunity cannot proceed into conduct.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authorized decision cancels a subsequent-examination opportunity before examination conduct begins.

**In Scope:** establish the cancellation decision; identify affected candidates/registrations/schedule; route schedule, registration and Hall Ticket consequences to their canonical owners; establish candidate-level routing; preserve history; and publish applicable events.

**Out of Scope:** deciding the contingency policy itself, directly changing Registration/Hall Ticket lifecycles, examination conduct, results, or technical communication.

**End Boundary:** the opportunity is no longer active for conduct and every affected candidate has a routed authoritative outcome.

## 5. Ownership and Participating Domains

**Supplementary, Backlog & Improvement Examination:** owns the opportunity and its administration.

**L2-I15:** owns the exception/contingency decision when cancellation arises from an operational disruption.

**L2-I03:** owns Examination Schedule cancellation.

**L2-I04:** owns Registration withdrawal/amendment.

**L2-I05:** owns Hall Ticket withdrawal/cancellation.

The scenario coordinates these outcomes without transferring ownership.

## 6. Business Actors, Roles and Responsibilities

- Controller of Examination: governs cancellation authority.
- Supplementary Examination Officer: manages the affected opportunity.
- Examination Administrator: identifies affected candidates and coordinates routing.
- L2-I15: provides contingency basis where applicable.
- L2-I03/L2-I04/L2-I05: execute their respective canonical consequences.

## 7. Preconditions and Entry Criteria

- A subsequent-examination opportunity exists.
- Affected candidate/registration population can be identified.
- The opportunity has not reached or completed conduct.
- An authoritative cancellation decision exists.
- Downstream consequences can be routed to their owning workflows.

## 8. Trigger

An authorized institutional decision cancels the subsequent-examination opportunity before conduct begins.

## 9. Main Business Flow

1. Identify the opportunity and authoritative cancellation decision.
2. Establish whether the cancellation is routine or contingency-driven.
3. Identify affected candidate registrations and participation records.
4. Coordinate schedule cancellation through L2-I03.
5. Route registration withdrawal/amendment through L2-I04 where applicable.
6. Route Hall Ticket withdrawal/cancellation through L2-I05 where applicable.
7. Determine the approved candidate-level route, such as replacement opportunity or rescheduling.
8. Preserve affected population and cancellation history.
9. Publish canonical events owned by the applicable workflows.
10. Close when every affected candidate has a routed authoritative outcome.

## 10. Alternate Business Flows

**Replacement opportunity:** route affected candidates into a new governed opportunity.

**No immediate replacement:** retain the institutionally governed pending/unresolved route without inventing a local state.

**Partial cancellation:** apply only to the affected subjects/candidate groups.

**Published Hall Tickets:** L2-I05 handles the Hall Ticket consequence; this scenario coordinates but does not own it.

## 11. Failure, Interruption and Business Recovery

**Decision not authoritative:** no downstream cancellation is applied until the decision is final.

**Population cannot be determined:** cancellation is not complete until the authoritative affected population is known.

**Registration consequence blocked:** opportunity remains cancelled while candidate-level registration consequence remains owned by L2-I04.

**Hall Ticket consequence unresolved:** Hall Ticket remains governed by L2-I05.

**Replacement later authorized:** candidates enter the distinct replacement opportunity rather than silently reviving the cancelled one.

## 12. Business Rules and Decision Constraints

- A cancelled subsequent-examination opportunity cannot proceed into conduct.
- Cancellation requires an authorized institutional decision.
- Schedule, Registration and Hall Ticket ownership remains with L2-I03/L2-I04/L2-I05.
- Candidate routing uses canonical downstream workflows.
- Historical cancellation decisions remain traceable.
- A replacement opportunity is a new governed opportunity, not an automatic revival.

## 13. State and Lifecycle Impact

The subsequent-examination lifecycle remains governed by L2-I12. Schedule, Registration and Hall Ticket lifecycles remain governed independently by L2-I03/L2-I04/L2-I05. No new cross-domain `Cancelled` state is created by this scenario.

## 14. Business Events and Cross-Domain Collaboration

L2-I12 publishes its subsequent-examination lifecycle events. L2-I03 publishes schedule cancellation facts; L2-I04 publishes `Registration Withdrawn` when applicable; L2-I05 publishes `Hall Ticket Withdrawn` or `Hall Ticket Cancelled` where applicable; L2-I15 publishes the canonical exception/contingency facts where it owns the decision.

No `SubsequentOpportunityCancelled` event is invented.

## 15. Business Authorization, Approval and Audit Obligations

- Cancellation authority follows the applicable subsequent-examination/contingency policy.
- Decision and affected population must be auditable.
- Each routed downstream consequence is auditable in its canonical workflow.
- Exceptional administrative action requires appropriate authority and justification.

## 16. Completion Criteria and Postconditions

**Complete when:** authorized cancellation exists; affected population is established; schedule/registration/Hall Ticket consequences are routed; each candidate has an authoritative route; history and rationale are preserved; applicable canonical events are published.

**Postconditions:** the cancelled opportunity cannot conduct; affected candidates are not active participants in it; any replacement/rescheduling follows its own lifecycle.

## 17. Open Issues, Assumptions and Deferred Questions

1. The frozen Stage 3 outcome is candidate routing; exact routing categories remain policy/configuration governed.
2. L2-I12 owns subsequent-examination administration, while L2-I03/L2-I04/L2-I05 own their operational consequences.
3. The accessible L2-I12 material does not expose a dedicated cancellation event name; none is invented.
