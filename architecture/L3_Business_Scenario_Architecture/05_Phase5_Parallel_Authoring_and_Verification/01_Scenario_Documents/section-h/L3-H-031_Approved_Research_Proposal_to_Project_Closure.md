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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 Faculty Workforce; applicable locked Finance architecture  
**Referenced By:** Grant and funding activity, ethics governance, research outputs, portfolio reporting, and institutional research audit

# 2. Scenario Purpose

Govern the complete Research Project lifecycle arising from an approved Research Proposal through project activation, execution, permitted suspension/resumption, completion, and institutional closure while preserving workforce and Finance ownership boundaries.

# 3. Business Outcome

The project executes, may be suspended or resumed under policy, and closes with verified deliverables and preserved evidence

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a proposal is approved and activated as a research project.

## 4.2 End Boundary

The scenario ends when the Research Project reaches `Closed` after completion and closure validation, with mandatory deliverables satisfied, applicable closure obligations resolved according to policy, and project evidence preserved.

## 4.3 In Scope

- Initiating a Research Project from an approved Research Proposal.
- Establishing the Research team using source-owned workforce identities and eligibility.
- Managing milestones and progress reporting.
- Applying permitted `Active` / `Suspended` project status changes.
- Completing the project after mandatory deliverables are satisfied.
- Performing institutional closure validation and moving the project to `Closed`.
- Preserving project, milestone, progress, status, completion, and closure evidence.

## 4.4 Out of Scope

- Proposal approval itself, governed by L3-H-029.
- Proposal withdrawal, governed by L3-H-030.
- Detailed Grant lifecycle, governed by L3-H-032.
- Finance accounting or posting decisions.
- Publication, patent, ethics, collaboration, or consultancy lifecycles except as closure obligations referenced by the Project lifecycle.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| People & Workforce / Section E | Owns Faculty identity, research capability, eligibility, availability, capacity, and workforce state referenced by Research. |
| Finance / Section F | Owns financial accounting, budgets, ledgers, payments, and financial facts referenced for funded projects and closure obligations. |
| Research / Section H | Owns Research Project lifecycle, project team references, milestones, progress reports, deliverables, project status, completion, and closure. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Research Office / Research Administrator | Initiates the project from an approved proposal and performs Research governance actions supported by H05. | Initiates/administers Research Project lifecycle. |
| Principal Investigator | Leads project execution, manages milestones, submits progress, requests completion, and participates in permitted status changes. | Performs project execution responsibilities. |
| Research Office | Approves/performs final project closure under H05 closure governance. | Closes completed project. |

# 7. Preconditions and Entry Criteria

- The originating Research Proposal is `Approved`.
- Principal Investigator remains valid according to L2-E03 Faculty Workforce.
- Required project identifiers and source-domain participant references are available.
- Funding references exist where funding is assigned.
- Ethics approval exists where mandatory.
- Project initiation does not duplicate an existing project from the same approved proposal except where institution configuration explicitly permits multiple implementation projects.

# 8. Trigger

A proposal is approved and activated as a research project

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Consume the approved proposal outcome and initiate the Research Project. | Research Office / Research Administrator / Research | Project enters `Initiated`; `ResearchProjectCreated` is published. |
| 2 | Validate and assign the project team using authoritative workforce/participant references. | Principal Investigator / Research Administrator / Research with source-domain validation | Project enters `Active`. |
| 3 | Establish and maintain governed milestones and progress reporting during execution. | Principal Investigator / Research | Project execution remains traceable. |
| 4 | Where policy permits, suspend or resume the project through the defined project status lifecycle. | Principal Investigator / Research Administrator / Research | `Active` / `Suspended` status changes remain governed and auditable. |
| 5 | Validate mandatory deliverables and completion criteria, then complete the Research Project. | Principal Investigator / Research | Project enters `Completed`; `ResearchProjectCompleted` is published. |
| 6 | Validate institutional closure conditions, including applicable financial, publication, and compliance obligations according to policy. | Research Office / Research with relevant source-domain evidence | Project is eligible for closure. |
| 7 | Close the Research Project and preserve closure evidence. | Research Office / Research | Project enters `Closed`; `ResearchProjectClosed` is published. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Temporary suspension | A valid project status condition requires suspension. | Move `Active` → `Suspended` using the governed status transition. | Project execution pauses without closure. | Research |
| Project resume | A suspended project receives approved resume decision. | Move `Suspended` → `Active`. | Project execution resumes with history preserved. | Research |
| Completion criteria not met | Mandatory deliverables remain incomplete. | Reject completion and keep the project in the permitted execution state. | Project does not become `Completed`. | Research |
| Closure checks not satisfied | Required closure obligations remain unresolved. | Reject premature closure and retain `Completed` state until closure validation passes. | Project does not become `Closed`. | Research |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Investigator/team source status becomes invalid | Project participation cannot be treated as current without authoritative source validation. | Apply only an upstream-defined Research project/team governance path using current workforce facts. | Research / People & Workforce |
| Required ethics or funding reference is absent | Project execution or closure may be ineligible. | Hold the affected Research transition until required source evidence exists. | Research / Source Domain |
| Completed project fails closure validation | Project cannot be formally closed. | Retain `Completed`, resolve outstanding governed obligations, then re-evaluate closure. | Research |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every project originates from an approved proposal. | Research | No project may be initiated without proposal approval. | L2-H05 PJ-01 |
| Every project has exactly one Principal Investigator. | Research with workforce source identity from E | Project team retains one PI reference. | L2-H05 PJ-02 |
| Project team members are referenced from authoritative enterprise domains. | Source domains / Research | Research does not duplicate workforce/student master data. | L2-H05 PJ-04 |
| Completion requires mandatory deliverables to be satisfied. | Research | `Active` → `Completed` requires completion validation. | L2-H05 PJ-08; §26.10 |
| Closure requires institutional closure validation. | Research | `Completed` → `Closed` is governed. | L2-H05 PJ-09 |
| Financial, publication, and compliance obligations should be resolved before closure according to institutional policy. | Research with respective source ownership | Closure references external obligations without transferring ownership. | L2-H05 §26.7 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Research Project | None | Approved proposal initiates project. | `Initiated` | Research | L2-H05 §26.8 |
| Research Project | `Initiated` | Valid team assignment begins execution. | `Active` | Research | L2-H05 §26.8 |
| Research Project | `Active` | Permitted status change may suspend project. | `Suspended` | Research | L2-H05 §26.8 |
| Research Project | `Suspended` | Approved resume returns project to execution. | `Active` | Research | L2-H05 §26.8 |
| Research Project | `Active` | Mandatory deliverables and completion criteria satisfied. | `Completed` | Research | L2-H05 §26.8 |
| Research Project | `Completed` | Closure validation passes. | `Closed` | Research | L2-H05 §26.8 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

The Research Project consumes `ResearchProposalApproved` and publishes exact H05 events including `ResearchProjectCreated`, `ResearchTeamAssigned`, `ProjectMilestoneUpdated`, `ProjectProgressReported`, `ResearchProjectStatusChanged`, `ResearchProjectCompleted`, and `ResearchProjectClosed`.

Relevant source-domain facts/events such as Faculty verification, funding references, and ethics approval retain their original owners.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Research uses workforce eligibility/capacity information from L2-E03 and Finance-owned financial references where applicable. Finance remains authoritative for accounting/posting; Research owns only project-level funding references and Research lifecycle state.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Research Office/Research Administrator initiates the project from an approved proposal. Principal Investigator performs project execution actions. Research Office owns the Research-side project closure action after closure validation.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is defined. Closed projects cannot be resumed under normal operating conditions.

## 15.3 Business Audit and Evidence Obligations

Preserve originating proposal reference, project identity, team assignment history, milestone history, progress reports, every project status change, completion summary, mandatory deliverable completion evidence, closure validation, closure authority, and published project events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The project originated from an approved Research Proposal.
- Project team and required source-domain references were validated.
- Execution history and permitted suspension/resumption are preserved where applicable.
- Mandatory deliverables satisfy completion validation.
- Project reaches `Completed`.
- Institutional closure validation passes.
- Project reaches `Closed` and closure evidence is preserved.

## 16.2 Postconditions

- The Research Project is read-only in normal operation after closure.
- Research retains project lifecycle ownership.
- Workforce and Finance source ownership remains unchanged.
- `ResearchProjectClosed` is available for downstream governed Research workflows.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome says the project closes with **verified deliverables**. H05 requires mandatory deliverables to be completed and closure validation to pass, but does not define a distinct Deliverable Verification lifecycle/event/role separate from project completion validation. | L3 can rely on H05 completion validation but cannot invent a separate deliverable-verification authority or event. | Clarify upstream only if a distinct deliverable verification lifecycle is required. |
| H05 states financial obligations should be resolved before closure according to policy, but does not define an exact Finance reconciliation event/contract for project closure. | Finance participation can be referenced through authoritative financial evidence, but L3 cannot invent a new reconciliation hand-off. | Define an explicit Finance closure contract/event if project closure requires one as a mandatory cross-domain condition. |

**Architecture assessment:** `PASS WITH ISSUES`

The complete project lifecycle, suspension/resumption, completion, closure states, roles, rules, and events are explicitly defined by H05. The gaps above do not prevent the frozen project-closure outcome from being expressed at L3.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
