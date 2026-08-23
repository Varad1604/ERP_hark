# Approved Research Proposal to Project Closure

# 1. Scenario Metadata

**Scenario ID:** L3-H-031  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; locked Finance architecture

# 2. Scenario Purpose

Govern an approved proposal through Research Project initiation, team assignment, execution, permitted suspension/resumption, completion, and institutional closure while preserving workforce and Finance ownership.

# 3. Business Outcome

The project executes, may be suspended or resumed under policy, and closes with verified deliverables and preserved evidence

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A proposal is approved and activated as a research project

## 4.2 End Boundary

The scenario ends when the Research Project reaches `Closed` after completion and closure validation, with mandatory deliverables satisfied and the required evidence preserved.

## 4.3 In Scope

Project initiation, team assignment, milestones, progress reporting, `Active`/`Suspended` governance, completion, closure validation, and closure evidence.

## 4.4 Out of Scope

Proposal approval/withdrawal; Grant, Ethics, Publication, IP, Collaboration, or Consultancy lifecycles except as referenced closure obligations; Finance accounting/posting; E03 workforce lifecycle changes.

# 5. Ownership and Participating Domains

E03 owns faculty capability, eligibility, availability, capacity, and workforce status. Finance owns accounting and monetary facts. Research owns Research Project state, project-team references, milestones, progress, deliverables, completion, and closure.

# 6. Business Actors, Roles and Responsibilities

Research Office / Research Administrator initiates the project. Principal Investigator governs execution, milestones, progress, and completion. Research Office performs the H05 closure action after closure validation.

# 7. Preconditions and Entry Criteria

Proposal is `Approved`; PI remains valid from E03; required participant references exist; funding references exist where applicable; Ethics approval exists where mandatory; duplicate project creation is not allowed except where institution configuration permits multiple implementation projects.

# 8. Trigger

A proposal is approved and activated as a research project

# 9. Main Business Flow

1. Consume `ResearchProposalApproved` and create the project; publish `ResearchProjectCreated`.
2. Validate and assign the research team; project becomes `Active` and `ResearchTeamAssigned` is published.
3. Maintain milestones and progress evidence through the active execution period.
4. Apply source-defined suspension/resumption where needed using `ResearchProjectStatusChanged`.
5. Validate mandatory deliverables and complete the project; publish `ResearchProjectCompleted`.
6. Validate applicable institutional closure conditions, including source-owned financial/compliance evidence where policy requires it.
7. Close the project and publish `ResearchProjectClosed`.

# 10. Alternate Business Flows

`Active → Suspended` and approved `Suspended → Active` are permitted. Incomplete deliverables block completion. Failed closure validation retains the project in `Completed` until governed obligations are satisfied.

# 11. Failure, Interruption and Business Recovery

Invalid source participant status, missing mandatory Ethics/funding references, or incomplete deliverables prevent the affected transition. A completed project that fails closure validation remains `Completed`; prior evidence is preserved.

# 12. Business Rules and Decision Constraints

Every project originates from an approved proposal; exactly one PI is required; team members are referenced from authoritative domains; completion requires mandatory deliverables; closure requires institutional validation; Finance remains owner of financial obligations and accounting.

# 13. State and Lifecycle Impact

`None → Initiated → Active`; `Active ↔ Suspended`; `Active → Completed → Closed`. Closed Projects are normally immutable.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Research consumes `ResearchProposalApproved` and publishes `ResearchProjectCreated`, `ResearchTeamAssigned`, `ProjectMilestoneUpdated`, `ProjectProgressReported`, `ResearchProjectStatusChanged`, `ResearchProjectCompleted`, and `ResearchProjectClosed`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Research consumes E03 workforce facts and Finance-owned financial evidence/references where applicable. No ownership transfers to Research.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Research Office/Research Administrator initiates; PI performs source-defined execution/completion actions; Research Office closes after validation.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional L3 separation-of-duty or reopen authority is invented.

## 15.3 Business Audit and Evidence Obligations

Preserve originating Proposal reference, team assignment history, milestone/progress history, project-state changes, completion summary, deliverable evidence, closure checks, closure authority, and project events.

# 16. Completion Criteria and Postconditions

A project completes this scenario when it originates from an Approved Proposal, executes through the defined lifecycle, satisfies mandatory deliverables, passes closure validation, reaches `Closed`, and preserves the required evidence.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- The frozen outcome says “verified deliverables”; H05 defines completion validation and required deliverables but no distinct Deliverable Verification aggregate/state/event/role.
- H05 says financial obligations should be resolved before closure according to policy but defines no single canonical Finance reconciliation event/contract for Project closure.

These do not prevent the H05-defined Project closure outcome, but should be clarified upstream if separate verification/reconciliation facts are mandatory.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
