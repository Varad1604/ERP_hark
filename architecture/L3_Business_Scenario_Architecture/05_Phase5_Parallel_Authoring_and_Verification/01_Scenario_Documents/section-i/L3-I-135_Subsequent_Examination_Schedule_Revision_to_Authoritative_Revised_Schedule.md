# Subsequent Examination Schedule Revision to Authoritative Revised Schedule

**Scenario ID:** L3-I-135  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** D — Academic Delivery; I — Examination  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; applicable locked L2 Examination workflows  
**Referenced By:** Applicable downstream Examination workflows

## 2. Scenario Purpose
Govern a necessary revision to the Examination Schedule for a subsequent-examination opportunity so that the revised timetable becomes the single authoritative schedule while the prior schedule remains historically traceable.

## 3. Business Outcome
An authorized schedule revision is approved and published as the authoritative revised Examination Schedule for the affected subsequent-examination opportunity, with the superseded schedule preserved and downstream workflows able to consume the revised business position.

## 4. Scenario Boundary and Scope
**Start Boundary:** An approved or published subsequent-examination schedule requires a governed change.

**In Scope:** affected opportunity and schedule; authorized revision reason; candidate/academic/resource impact; revised planning; conflict validation; approval; publication; historical preservation; downstream business coordination.

**Out of Scope:** policy changes; candidate eligibility changes; Examination Registration ownership; independent Hall Ticket modification; conduct; marks/results; technical rescheduling; new lifecycle creation.

**End Boundary:** The revised schedule is published as the sole current authoritative timetable.

## 5. Ownership and Participating Domains
**Supplementary, Backlog & Improvement Examination:** owns the subsequent-examination opportunity and identifies the scheduling consequence.

**Examination Planning & Scheduling (L2-I03):** owns the Examination Schedule and revision lifecycle.

**Academic Delivery:** supplies authoritative academic-delivery information where affected.

**Examination Exception & Contingency (L2-I15):** governs exceptional disruption decisions when applicable.

No participating domain takes schedule ownership from L2-I03.

## 6. Business Actors, Roles and Responsibilities
- **Supplementary Examination Officer:** identifies and coordinates scheduling impact.
- **Examination Administrator:** manages schedule revision activities.
- **Controller of Examination:** governs revised schedule approval.
- **Academic Administration:** validates academic implications where required.
- **Academic Delivery:** supplies authoritative academic context.
- **Examination Planning & Scheduling:** prepares, validates, approves and publishes the revised schedule.
- **Examination Exception & Contingency:** governs qualifying disruption decisions.

## 7. Preconditions and Entry Criteria
- A subsequent-examination opportunity exists.
- An approved or published Examination Schedule exists.
- A material business reason for revision is authorized.
- Where applicable, an exception/contingency decision has been established.
- The Examination Session remains valid or has undergone its own governed lifecycle change.
- Business impact can be assessed.

## 8. Trigger
An authorized business decision determines that the current subsequent-examination schedule must be revised.

## 9. Main Business Flow
1. Identify the opportunity and current Examination Schedule.
2. Establish the authorized reason for revision and whether it is routine planning or an exception.
3. Assess impact on candidate scope, subjects, academic context, venues, resources and downstream workflows.
4. Provide the revised requirement to L2-I03.
5. Prepare the revised timetable under the existing schedule lifecycle.
6. Validate conflicts.
7. Resolve material conflicts.
8. Complete review and obtain required approval.
9. Publish the revised Examination Schedule.
10. Preserve the superseded schedule as historical evidence.
11. Associate the revised schedule with the opportunity.
12. Provide the revised schedule as the authoritative business input to affected downstream workflows.
13. Publish applicable canonical schedule and subsequent-examination events.

## 10. Alternate Business Flows
### 10.1 Routine Planning Revision
L2-I03 governs the revision without invoking the full exception process.

### 10.2 Examination Disruption
L2-I15 governs the exception/contingency decision while L2-I03 remains the schedule owner.

### 10.3 Revision Before Hall Ticket Use
The revised schedule becomes the downstream authoritative input before candidate-facing operational consequences occur.

### 10.4 Revision After Hall Ticket Publication
L2-I03 publishes the revised schedule; Hall Ticket consequences remain governed by L2-I05.

### 10.5 Revision After Conduct Begins
Applicable contingency and conduct workflows govern candidate/examination consequences; L2-I03 remains schedule owner.

## 11. Failure, Interruption and Business Recovery
### 11.1 Revised Schedule Cannot Be Approved
The current schedule remains authoritative until a valid successor is approved and published.

### 11.2 No Viable Replacement Schedule
The applicable contingency process determines whether the opportunity is postponed, cancelled, restored or otherwise governed; no new schedule state is created.

### 11.3 Candidate Impact Cannot Be Reconciled
The revision remains unresolved until the business impact is governed; candidate eligibility and registration ownership remain canonical upstream responsibilities.

### 11.4 Exception Decision Is Not Final
The schedule is not changed on an unapproved exception position.

### 11.5 Proposed Revision Superseded Before Publication
The obsolete proposal does not become operational; the current authoritative schedule remains unchanged.

## 12. Business Rules and Decision Constraints
- L2-I03 owns the Examination Schedule and revision lifecycle.
- Published schedules are authoritative operational timetables.
- Schedule revisions preserve historical schedule versions.
- Only approved schedules may be published.
- Material conflicts must be resolved before approval.
- L2-I15 owns exception/contingency decisions; L2-I03 owns the resulting schedule.
- A schedule revision does not automatically modify registration, Hall Ticket, marks or results.
- The revised schedule becomes authoritative only after the existing L2-I03 approval/publication lifecycle completes.
- No new `Rescheduled` state is introduced.

## 13. State and Lifecycle Impact
The Examination Schedule uses the existing L2-I03 lifecycle:

`Draft → Under Review → Approved → Published → Active → Revised → Completed → Archived`

The existing **Revised** position expresses the outcome; no new `Rescheduled` state is created.

The subsequent-examination opportunity remains governed by L2-I12 and any exception remains governed by L2-I15.

## 14. Business Events and Cross-Domain Collaboration
L2-I03 publishes:
- **Schedule Updated**
- **Schedule Approved**
- **Schedule Published**
- **Schedule Revised**
- **Schedule Cancelled**
- **Schedule Archived**

L2-I12 identifies **Examination Scheduled** as a subsequent-examination lifecycle event.

Where an exception causes the revision, L2-I15 publishes its own canonical exception/contingency facts; it does not become the schedule publisher.

No `SubsequentExaminationScheduleRevised` event is invented.

## 15. Business Authorization, Approval and Audit Obligations
- The revision reason must be authorized under planning or contingency governance.
- Revised schedule approval follows L2-I03 authority.
- Exception-caused revisions retain L2-I15 approval traceability.
- Superseded and revised schedule positions remain auditable.
- Downstream consequences remain attributable to their canonical owners.

## 16. Completion Criteria and Postconditions
**Completion Criteria:** current opportunity/schedule identified; authorized reason established; impact assessed; revised schedule prepared; conflicts resolved; approval complete; revised schedule Published; superseded schedule preserved; opportunity associated with revised authoritative schedule; applicable events published.

**Postconditions:** one current authoritative schedule exists; the superseded schedule is historical; L2-I03 remains schedule owner; downstream Hall Ticket/conduct/result consequences remain with their canonical workflows.

## 17. Open Issues, Assumptions and Deferred Questions
1. L2-I03 explicitly supports Schedule Revision and historical schedule preservation.
2. L2-I15 governs exceptional disruption, postponement, rescheduling, suspension and cancellation while excluding routine Examination Scheduling ownership; this scenario preserves that boundary.
3. Downstream consequences depend on the state of Hall Tickets and examination conduct and therefore remain within their owning workflows.
4. The available L2 sources do not define a dedicated subsequent-examination schedule-revision event; none is invented.
