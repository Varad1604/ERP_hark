# Reschedule Supplementary, Backlog, or Improvement Examination Following Authorized Disruption

**Scenario ID:** L3-I-136  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** G — Campus Operations; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I03 Examination Planning & Scheduling Workflows; L2-I12 Supplementary / Backlog / Improvement Examination Execution Workflows; L2-I15 Examination Exception & Contingency Workflows  
**Referenced By:** L2-I05 Hall Ticket Management; L2-I06 Practical & Viva Examination Conduct; L2-I07 Theory Examination Conduct; L2-I08 Marks Collection & Evaluation; L2-I09 Result Processing & Computation

## 1. Scenario Metadata

This scenario is the final L3-I-136 canonical output. The frozen Stage 3 semantics define it as a Critical P1 scenario owned by Supplementary, Backlog & Improvement Examination, with material participation from Campus Operations and Examination.

The scenario is specifically the business outcome of obtaining a valid replacement schedule for a subsequent examination opportunity after an authorized disruption has made the existing schedule unusable or requires governed rescheduling.

## 2. Scenario Purpose

Provide a governed business path from an authorized examination disruption to a valid revised schedule for a Supplementary, Backlog, or Improvement examination.

The scenario preserves the separation between contingency governance and schedule ownership: L2-I15 governs the authorized disruption and contingency decision, while L2-I03 owns the Examination Schedule and its revision/rescheduling lifecycle. L2-I12 remains the owner of subsequent-examination administration.

## 3. Business Outcome

A new authoritative Examination Schedule is approved and published for the affected Supplementary, Backlog, or Improvement examination opportunity following the authorized disruption, with the affected operational context and relevant resource implications reconciled.

The superseded schedule remains historically traceable, and downstream examination workflows can consume the revised schedule.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authorized disruption has resulted in an approved contingency decision requiring a Supplementary, Backlog, or Improvement examination to be rescheduled.

**In Scope:**
- establish the affected subsequent-examination opportunity and current schedule;
- use the authorized contingency decision as the business basis for rescheduling;
- assess affected academic/calendar/resource context;
- prepare a replacement schedule through L2-I03;
- validate conflicts and resource feasibility;
- obtain required schedule approval;
- publish the revised schedule;
- preserve the superseded schedule history;
- coordinate the revised schedule as the authoritative input for downstream examination workflows.

**Out of Scope:**
- registering or re-registering candidates;
- redefining examination eligibility;
- owning the Examination Schedule;
- deciding whether the original disruption qualifies as an exception;
- examination conduct;
- Hall Ticket lifecycle;
- marks/evaluation;
- result processing or publication.

**End Boundary:** The replacement schedule is published as the authoritative timetable for the affected subsequent-examination opportunity.

## 5. Ownership and Participating Domains

**Supplementary, Backlog & Improvement Examination:** owns the affected subsequent-examination opportunity and its administration. It coordinates the need for rescheduling but does not own the Examination Schedule.

**Campus Operations (G):** participates where the disruption affects campus/venue or operational resource context. It retains ownership of its own campus operational information and decisions.

**Examination Planning & Scheduling (L2-I03):** owns preparation, validation, approval and publication of the revised Examination Schedule.

**Examination Exception & Contingency (L2-I15):** owns the Examination Exception Case and the contingency decision that authorizes postponement/rescheduling.

No participation in the scenario transfers ownership of an upstream aggregate or lifecycle.

## 6. Business Actors, Roles and Responsibilities

| Actor / Domain | Responsibility |
|---|---|
| Examination Controller | Governs examination contingency and schedule decisions within the applicable institutional authority |
| Examination Administrator | Coordinates subsequent-examination rescheduling |
| Supplementary Examination Officer | Maintains the affected subsequent-examination context and coordinates its revised schedule requirement |
| Campus Administration / Infrastructure Administration | Provides authoritative venue and operational availability where the disruption affects campus resources |
| Examination Planning & Scheduling | Prepares, validates, approves and publishes the authoritative revised Examination Schedule |
| Examination Exception & Contingency | Establishes and governs the authorized contingency decision |

Only actors supported by the locked L2 sources are used; no new rescheduling authority is introduced.

## 7. Preconditions and Entry Criteria

- A Supplementary, Backlog, or Improvement examination opportunity exists.
- An Examination Schedule already exists for the opportunity.
- A disruption affecting the examination has been registered and assessed under L2-I15.
- The contingency decision authorizing postponement/rescheduling has been approved.
- The affected Examination Session remains applicable, or its own governed lifecycle position has been established.
- The business context required for replacement scheduling is available.

## 8. Trigger

An authorized contingency decision determines that the affected subsequent examination must be rescheduled.

## 9. Main Business Flow

1. Identify the affected subsequent-examination opportunity and its current Examination Schedule.
2. Establish the authorized contingency decision and the business reason requiring rescheduling.
3. Assess the impact of the disruption on examination date, time, venue, resources, academic calendar and affected operational context.
4. Establish the replacement scheduling requirement within the subsequent-examination administration context.
5. Hand the scheduling requirement to L2-I03 Examination Planning & Scheduling.
6. Prepare a replacement Examination Schedule using the applicable examination window, venue/resource availability and examination requirements.
7. Validate relevant student, academic-calendar, venue and resource conflicts.
8. Resolve material planning conflicts through the existing L2-I03 governance.
9. Complete the required institutional schedule review and approval.
10. Publish the replacement Examination Schedule.
11. Preserve the superseded schedule as historical schedule context.
12. Associate the published replacement schedule with the affected subsequent-examination opportunity.
13. Make the revised schedule available to downstream examination workflows.
14. Publish the applicable canonical schedule and subsequent-examination lifecycle events.

The flow ends at publication of the authoritative replacement schedule. It does not continue into Hall Ticket regeneration or examination conduct.

## 10. Alternate Business Flows

### 10.1 Venue-Driven Rescheduling

Where the disruption affects a campus venue or facility, Campus Operations supplies the authoritative operational availability. L2-I03 incorporates that information into schedule planning without taking ownership of the campus resource.

### 10.2 Calendar-Constrained Rescheduling

Where the authorized disruption changes the viable examination window, the replacement schedule is prepared within the applicable institutional calendar and examination-window constraints.

### 10.3 Multiple Affected Examination Items

Where the disruption affects multiple scheduled examination items within the subsequent-examination opportunity, L2-I03 may revise the affected schedule scope together while preserving the single authoritative schedule lifecycle.

### 10.4 Downstream Candidate Consequences

Where the revised schedule has consequences for already-issued Hall Tickets or other downstream examination activities, those consequences are handled by the owning downstream workflows. This scenario ends at the authoritative schedule outcome.

## 11. Failure, Interruption and Business Recovery

### 11.1 No Viable Replacement Schedule

If no acceptable replacement schedule can be established, the rescheduling outcome is not treated as complete. The authorized contingency governance determines the next permitted institutional course, such as further postponement or cancellation, without inventing a new L3 state.

### 11.2 Resource Availability Remains Unresolved

If required venue or operational resources remain unavailable, the schedule remains unresolved until the business resource position is established. Campus Operations remains authoritative for its own operational context.

### 11.3 Schedule Approval Is Not Obtained

If the replacement schedule is not approved, the prior authoritative schedule remains the current schedule until an authorized successor is published or another contingency decision changes that position.

### 11.4 Contingency Decision Is Withdrawn or Superseded

If the authorized contingency decision is subsequently changed before rescheduling completes, the scheduling activity follows the new authoritative business position rather than publishing a superseded contingency outcome.

### 11.5 Previously Published Schedule Has Downstream Consequences

If Hall Tickets or examination arrangements already exist, the revised schedule does not silently overwrite their owning lifecycles. L2-I05, L2-I06 and L2-I07 govern their respective consequences.

## 12. Business Rules and Decision Constraints

- Examination rescheduling following a disruption must be based on an authorized contingency decision.
- L2-I15 owns examination exception and contingency governance; L2-I03 owns the Examination Schedule.
- L2-I03 governs the preparation, validation, approval and publication of the replacement schedule.
- Only an approved schedule may be published for operational use.
- Schedule revision must preserve historical schedule versions.
- The affected subsequent-examination opportunity remains owned by L2-I12.
- Campus operational information remains owned by Campus Operations.
- Rescheduling does not itself change candidate eligibility or Examination Registration ownership.
- No local rule, schedule state, event or authority is introduced by this scenario.

## 13. State and Lifecycle Impact

The Examination Schedule remains within the L2-I03 lifecycle. The relevant schedule progression uses the existing governed positions, including **Revised**, rather than introducing a new `Rescheduled` state.

The subsequent-examination lifecycle remains governed by L2-I12, whose framework includes the **Scheduled** position.

The Examination Exception Case remains governed by L2-I15. The scenario does not merge the exception lifecycle with the schedule lifecycle.

## 14. Business Events and Cross-Domain Collaboration

L2-I15 governs the exception/contingency business facts associated with the disruption.

L2-I03 is the canonical publisher for schedule milestones, including:
- **Schedule Updated**
- **Schedule Approved**
- **Schedule Published**
- **Schedule Revised**
- **Schedule Cancelled**
- **Schedule Archived**

L2-I12 identifies subsequent-examination milestones including:
- **Examination Scheduled**
- **Attempt Closed**

The scenario does not invent a `ReschedulingCompleted` or `SubsequentExaminationRescheduled` event. Event publisher ownership remains with the canonical owning workflow.

## 15. Business Authorization, Approval and Audit Obligations

- The contingency decision authorizing rescheduling must be institutionally approved under L2-I15.
- Revised schedule approval follows the authority defined by L2-I03.
- Where campus resources are affected, the relevant operational decision remains attributable to Campus Operations.
- The reason for rescheduling, affected schedule, approval decision, revised schedule and superseded schedule must remain auditable.
- Candidate-facing and downstream operational consequences must remain attributable to their canonical owning workflows.
- No technical authorization mechanism is defined at L3.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- Authorized contingency decision is established.
- Affected subsequent-examination opportunity and schedule are identified.
- Replacement scheduling requirements are defined.
- Required operational/resource context is authoritative.
- Replacement schedule is prepared and validated.
- Material conflicts are resolved.
- Required schedule approval is complete.
- Replacement Examination Schedule is Published.
- Superseded schedule remains historically traceable.
- Revised schedule is associated with the subsequent-examination opportunity.
- Applicable canonical events are published.

**Postconditions:**
- The replacement Examination Schedule is the current authoritative timetable.
- L2-I03 remains the schedule owner.
- L2-I12 remains the subsequent-examination administration owner.
- L2-I15 retains ownership of the underlying exception/contingency case.
- Downstream examination workflows can consume the revised schedule.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I15 explicitly owns examination rescheduling as a contingency capability while L2-I03 explicitly owns Examination Scheduling. This scenario preserves that split rather than treating contingency participation as schedule ownership.
2. The available L2-I15 source does not define a dedicated cross-domain contract name for handing a contingency decision to L2-I03. No new contract name is invented.
3. The exact downstream effect of rescheduling depends on the current state of Hall Ticket and examination-conduct workflows. Those effects remain outside this scenario's end boundary.
4. Institution-specific rescheduling windows, approval levels and resource policies remain governed by the applicable upstream examination policies and workflows.
