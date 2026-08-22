# Quality Improvement Proposal to Verified Institutional Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-068  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Accountable Domain  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** Quality governance, readiness, audit, compliance, and institutional-improvement scenarios

# 2. Scenario Purpose

Govern a measurable institutional quality-improvement initiative from proposal through approval, accountable execution, monitoring, outcome evaluation, and governed completion while participating domains retain ownership of the business changes they execute.

# 3. Business Outcome

The initiative is approved, executed by accountable domains, measured, verified, and closed or discontinued with lessons retained

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A trend, finding, benchmark, or stakeholder input identifies an improvement opportunity.

## 4.2 End Boundary

H09 supports proposal, approval, execution, monitoring, evaluation, completion, and a workflow-level close action/event. It does not define a `Closed` lifecycle state for the Quality Improvement Initiative, a `Discontinued` state/path, or any explicit lessons-retained outcome. The complete frozen end boundary is therefore unresolved.

## 4.3 In Scope

- Quality objective and initiative definition.
- Measurable KPI definition.
- Approval and ownership assignment.
- Accountable-domain execution.
- Progress monitoring and outcome measurement.
- Outcome evaluation before completion/closure.
- Institutional improvement history.

## 4.4 Out of Scope

- Inventing a discontinued initiative lifecycle.
- Treating workflow `Close Initiative` as a lifecycle state when H09-A09 ends at `Completed`.
- Inventing a lessons-learned artifact or retention rule.
- Taking ownership of operational changes executed by the accountable domain.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance / H | Owns the Quality Improvement Initiative governance lifecycle, KPI expectation, monitoring, evaluation, and institutional improvement history. |
| VAR:Accountable Domain | Owns and executes the underlying business changes or institutional improvements assigned to its canonical scope. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Coordinates institutional quality governance and review. | Reviews / coordinates |
| Institutional Leadership | Participates in institutional improvement governance. | Sponsors / reviews |
| Registrar | Participates in institutional quality initiatives where applicable. | Coordinates / reviews |
| Deans | Participate for school-level improvement initiatives. | Accountable participation |
| Department Heads | Participate for departmental improvement initiatives. | Accountable participation |

# 7. Preconditions and Entry Criteria

- A source-supported improvement opportunity exists.
- The initiative can define measurable success indicators.
- Accountable institutional ownership can be assigned without transferring the underlying business lifecycle into H09.

# 8. Trigger

A trend, finding, benchmark, or stakeholder input identifies an improvement opportunity

# 9. Main Business Flow

1. H09 records the improvement opportunity and defines the quality objective.
2. H09 creates the Quality Improvement Initiative.
3. The initiative identifies accountable leadership and measurable success indicators.
4. The initiative is approved under H09 quality governance.
5. Required responsibilities/resources are allocated within the accountable institutional domains.
6. The accountable domain executes the approved improvement work under its own business ownership.
7. H09 monitors progress and records KPI updates.
8. H09 measures the institutional outcome and conducts the source-defined review.
9. H09 evaluates the outcome before completion as required by BR-H09-030.
10. H09 records the source-defined `Initiative Completed` outcome where successful.
11. Stop before asserting a canonical `Closed` state, `Discontinued` outcome, or retained-lessons result because those semantics are not defined upstream.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Outcome target is not met | Evaluation shows the initiative has not achieved its objective | Continue governed monitoring/execution or revise through upstream-approved governance | Initiative remains unresolved | Accreditation & Compliance / accountable domain |
| Discontinuation is required | Governance determines the initiative should not continue | No source-defined discontinuation path exists | Frozen alternate outcome blocked | Accreditation & Compliance |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| KPI is not defined | BR-H09-029 is not satisfied | Define measurable KPI before relying on the initiative outcome | Accreditation & Compliance |
| Outcome evaluation is absent | Closure/completion evidence is insufficient | Complete the source-defined outcome evaluation | Accreditation & Compliance |
| Discontinuation is required | H09 lacks the lifecycle branch | Escalate upstream; do not invent a local state | Accreditation & Compliance |
| Lessons must be retained | Frozen outcome requires evidence not defined in H09 | Record the architecture gap; do not invent a lessons artifact | Accreditation & Compliance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Quality Improvement Initiatives define measurable KPIs | H09 | Outcome must be measurable | L2-H09 BR-H09-029 |
| Completed initiatives undergo outcome evaluation before closure | H09 | Evaluation precedes completion/closure | L2-H09 BR-H09-030 |
| Operational source domains retain ownership of their business facts and lifecycles | Source domains | H09 governance does not transfer execution ownership | L2-H09 Principle 2; ownership boundaries |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Quality Improvement Initiative | Proposed | Approve the initiative | Approved | H09 | L2-H09 H09-A09 |
| Quality Improvement Initiative | Approved | Begin governed work | Initiated | H09 | L2-H09 H09-A09 |
| Quality Improvement Initiative | Initiated | Execute and monitor improvement | Execution → Monitoring | H09 | L2-H09 H09-A09 |
| Quality Improvement Initiative | Monitoring | Evaluate measurable outcome | Evaluation | H09 | L2-H09 H09-A09 |
| Quality Improvement Initiative | Evaluation | Record supported completion | Completed | H09 | L2-H09 H09-A09; H09-11 |

H09-11 includes `Close Initiative` and defines `Initiative Closed`, but H09-A09 lifecycle ends at `Completed`; no `Closed` or `Discontinued` state is defined.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Improvement Initiative Created | Quality Improvement aggregate / H09 | Governance participants | Records initiative creation | L2-H09 §11 |
| Improvement Initiative Approved | Quality Improvement aggregate / H09 | Accountable participants | Records governance approval | L2-H09 §11 |
| KPI Updated | Quality Improvement aggregate / H09 | Governance participants | Records measured progress | L2-H09 §11 |
| Initiative Completed | Quality Improvement aggregate / H09 | Governance participants | Records completion | L2-H09 §11 |
| Initiative Closed | Quality Improvement aggregate / H09 | Governance participants | Records workflow-level closure | L2-H09 §11 |

No discontinued event or retained-lessons event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The accountable domain executes the underlying improvement work and supplies outcome evidence. H09 governs the institutional initiative, KPI, review, and outcome record without assuming ownership of the accountable domain's operational lifecycle.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The H09 continuous-quality workflow includes initiative approval and assigns leadership/accountable participation. No authoritative discontinuation authority is defined.

## 15.2 Separation of Duties, Overrides and Exceptions

No outcome is treated as verified without measurable evaluation. No local discontinued state or lessons artifact is invented.

## 15.3 Business Audit and Evidence Obligations

Preserve the improvement opportunity, objective, initiative approval, accountable ownership, KPIs, progress, outcome measurements, evaluation, and institutional improvement history supported by H09.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The source-supported successful path can reach `Completed` after outcome evaluation. The complete frozen outcome cannot be satisfied because closed/discontinued lifecycle semantics and lessons-retention semantics are incomplete.

## 16.2 Postconditions

Supported initiative history remains traceable; no unsupported discontinued or lessons-retained result is asserted.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen semantics require `closed or discontinued with lessons retained`.
- H09-A09 lifecycle ends at `Completed`.
- H09-11 includes a `Close Initiative` action and `Initiative Closed` event without defining a corresponding `Closed` lifecycle state.
- No `Discontinued` state, transition, rule, authority, or event exists.
- No explicit lessons-retention artifact, rule, or event exists.

These gaps materially affect the frozen final outcome.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H09 must reconcile `Completed` versus `Closed`, define discontinuation governance, and define how lessons are retained for the frozen outcome.
