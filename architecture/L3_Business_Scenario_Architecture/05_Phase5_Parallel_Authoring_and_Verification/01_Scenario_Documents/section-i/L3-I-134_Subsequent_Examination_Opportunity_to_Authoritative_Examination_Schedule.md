# Subsequent Examination Opportunity to Authoritative Examination Schedule

**Scenario ID:** L3-I-134  
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
Convert an authorized subsequent-examination opportunity into an authoritative Examination Schedule that downstream examination workflows can consume. Scheduling is coordinated with L2-I03 without transferring schedule ownership away from Examination Planning & Scheduling.

## 3. Business Outcome
An authorized subsequent-examination opportunity is associated with an approved and authoritative Examination Schedule for its applicable Examination Session, subject scope and examination mode.

## 4. Scenario Boundary and Scope
**Start Boundary:** A subsequent-examination opportunity has been authorized and requires operational scheduling.

**In Scope:** opportunity and subject scope; governing Examination Session; academic context; scheduling requirement; timetable/resource planning; conflict validation; approval; publication; association of the schedule with the opportunity.

**Out of Scope:** candidate eligibility; Examination Registration ownership; Examination Session lifecycle ownership; Hall Ticket issuance; conduct; marks/results; policy definition.

**End Boundary:** An authoritative published Examination Schedule exists for the subsequent-examination opportunity.

## 5. Ownership and Participating Domains
**Supplementary, Backlog & Improvement Examination:** owns the subsequent-examination opportunity and its scheduling requirement.

**Academic Delivery:** supplies authoritative academic-delivery context where required.

**Examination Planning & Scheduling (L2-I03):** owns Examination Planning and the authoritative Examination Schedule.

**Examination Session Management (L2-I02):** owns the governing Examination Session.

No schedule ownership transfers to L2-I12.

## 6. Business Actors, Roles and Responsibilities
- **Supplementary Examination Officer:** establishes the authorized scope and coordinates the scheduling requirement.
- **Examination Administrator:** manages planning coordination and review.
- **Controller of Examination:** governs schedule approval.
- **Academic Administration:** validates academic applicability where required.
- **Academic Delivery:** supplies authoritative academic context.
- **Examination Planning & Scheduling:** prepares, validates, approves and publishes the schedule.

## 7. Preconditions and Entry Criteria
- The subsequent-examination opportunity is authorized.
- Candidate/subject scope is established.
- A valid governing Examination Session exists.
- Required academic context is authoritative.
- L2-I03 scheduling governance applies.
- Required examination policy is established.

## 8. Trigger
An authorized subsequent-examination opportunity requires operational scheduling.

## 9. Main Business Flow
1. Identify the authorized opportunity and subject/candidate scope.
2. Establish the applicable Examination Session under L2-I02.
3. Determine academic and examination context required for scheduling.
4. Hand the scheduling requirement to L2-I03.
5. Prepare the schedule using dates, time slots, examination mode, venues and resources.
6. Validate student, faculty, venue, resource and calendar conflicts as applicable.
7. Resolve material planning conflicts.
8. Complete institutional review.
9. Approve the Examination Schedule.
10. Publish the schedule as the official timetable.
11. Associate the published schedule with the subsequent-examination opportunity.
12. Make it available to downstream Hall Ticket and conduct workflows.

## 10. Alternate Business Flows
### 10.1 Practical or Viva Opportunity
The schedule uses the applicable examination mode and resource context; conduct remains owned by L2-I06.

### 10.2 Multiple Subjects
L2-I03 produces the authoritative schedule covering the permitted subject scope.

### 10.3 Multiple Candidate Groups
The authoritative schedule may contain necessary candidate/venue distinctions without creating competing schedules.

### 10.4 Schedule Not Yet Approveable
The schedule remains within the existing L2-I03 planning lifecycle until governing conditions are resolved.

## 11. Failure, Interruption and Business Recovery
### 11.1 No Valid Examination Session
Scheduling does not proceed; L2-I02 remains authoritative for session lifecycle.

### 11.2 Academic Applicability Conflict
Academic Delivery remains authoritative for its own information and the affected schedule remains unresolved.

### 11.3 Resource Conflict
L2-I03 applies its conflict-resolution process; the opportunity is not treated as scheduled until an authoritative schedule is approved.

### 11.4 Candidate Scope Changes
The schedule is re-evaluated against the updated authoritative scope before publication.

### 11.5 Schedule Cancelled Before Operational Use
The schedule consequence is governed by L2-I03; the opportunity remains an administrative outcome and may require a new schedule.

## 12. Business Rules and Decision Constraints
- Every Examination Schedule belongs to an approved Examination Session.
- L2-I03 owns Examination Planning and Scheduling.
- Only approved schedules may be published for operational use.
- Published schedules are the authoritative institutional timetable.
- Material scheduling conflicts must be resolved before approval.
- Academic Delivery retains ownership of its information.
- L2-I12 does not create a parallel schedule lifecycle.
- Schedule publication does not independently establish candidate eligibility.
- No new schedule rule or state is created.

## 13. State and Lifecycle Impact
The subsequent-examination opportunity uses the existing L2-I12 lifecycle, including its **Scheduled** position.

The Examination Schedule uses the L2-I03 lifecycle:

`Draft → Under Review → Approved → Published → Active → Revised → Completed → Archived`

These are separate lifecycles; no cross-domain `Scheduled` state is introduced.

## 14. Business Events and Cross-Domain Collaboration
L2-I03 publishes:
- **Examination Planning Initiated**
- **Examination Schedule Created**
- **Schedule Updated**
- **Conflict Validation Completed**
- **Schedule Approved**
- **Schedule Published**
- **Schedule Revised**
- **Schedule Cancelled**
- **Schedule Archived**

L2-I12 identifies **Examination Scheduled** as a significant subsequent-examination event. Each publisher remains canonical to its own lifecycle.

No new scheduling contract or event name is invented.

## 15. Business Authorization, Approval and Audit Obligations
- Schedule approval follows L2-I03 authority.
- The subsequent-examination opportunity must already be authorized.
- Academic applicability remains attributable to Academic Delivery.
- Planning, conflict resolution, approval and publication are auditable.
- Exceptional scheduling decisions follow existing examination governance.

## 16. Completion Criteria and Postconditions
**Completion Criteria:** authorized scope established; valid session identified; planning completed; conflicts resolved; review/approval complete; schedule Published; opportunity associated with the authoritative schedule.

**Postconditions:** L2-I03 owns the schedule; L2-I12 owns the opportunity; downstream Hall Ticket and conduct workflows can consume the published timetable.

## 17. Open Issues, Assumptions and Deferred Questions
1. L2-I03 defines the authoritative schedule lifecycle and downstream consumption boundary.
2. L2-I12 identifies scheduling as part of subsequent-examination administration while leaving detailed scheduling ownership to L2-I03.
3. The available L2 sources do not expose a dedicated synchronous scheduling-contract name; none is invented.
