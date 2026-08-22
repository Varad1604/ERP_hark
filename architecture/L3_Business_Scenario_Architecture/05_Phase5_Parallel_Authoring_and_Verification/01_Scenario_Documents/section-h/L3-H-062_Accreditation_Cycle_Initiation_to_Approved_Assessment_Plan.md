# Accreditation Cycle Initiation to Approved Assessment Plan

# 1. Scenario Metadata

**Scenario ID:** L3-H-062  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** Evidence, internal review, readiness, external assessment, SSR, and compliance-governance scenarios

# 2. Scenario Purpose

Establish a governed H09 Accreditation Cycle and institution-wide assessment plan that identifies responsibilities, milestones, applicable framework criteria, evidence sources, reviews, and downstream submission obligations while evidence-owning domains retain ownership of their source facts.

# 3. Business Outcome

Responsibilities, milestones, criteria, evidence sources, reviews, and submission obligations are established in an approved plan

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An accreditation or compliance cycle is opened for an institutional scope.

## 4.2 End Boundary

The scenario ends when the Accreditation Cycle has a governed institution-wide plan with participating units, responsibilities, deliverables, milestones, applicable criteria/evidence expectations, review checkpoints, and downstream SSR/submission obligations established under H09 governance.

## 4.3 In Scope

- Selection of one approved Accreditation Framework.
- Creation of the Accreditation Cycle.
- Assessment timeline, participating units, responsibilities, deliverables, owners, and milestones.
- Framework-derived criteria and evidence expectations relevant to the cycle.
- Review/readiness checkpoints.
- Identification of H09 SSR preparation/handoff and H08 submission ownership where applicable.
- Governance approval of the major review cycle before the plan is treated as approved.

## 4.4 Out of Scope

- Collecting or validating evidence.
- Executing audits or external assessments.
- SSR drafting or external submission.
- Changing source-domain business records or their ownership.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance / H | Owns Accreditation Cycle, accreditation planning, responsibilities, milestones, readiness checkpoints, review governance, and the institutional accreditation plan. |
| VAR:Evidence-Owning Domain | Retains ownership of operational facts and evidence sources referenced by the plan and later requested by H09. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Coordinates accreditation governance and readiness planning. | Reviews / coordinates |
| Accreditation Coordinator | Creates and coordinates the institution-wide accreditation plan. | Performs / coordinates |
| Department Coordinators | Receive source-defined responsibilities for participating departmental evidence and deliverables where applicable. | Performs |
| Registrar | Participates in institutional accreditation planning. | Reviews / coordinates |
| Deans | Participate in planning for their institutional units. | Reviews / performs |
| Steering Committee | Approves major review cycles under the H09 governance structure. | Approves cycle governance |

# 7. Preconditions and Entry Criteria

- One approved Accreditation Framework governs the cycle.
- The institutional scope and assessment period are known.
- Participating units and applicable framework criteria/evidence expectations can be identified without transferring source ownership.

# 8. Trigger

An accreditation or compliance cycle is opened for an institutional scope

# 9. Main Business Flow

1. H09 selects the approved Accreditation Framework that governs the cycle.
2. H09 creates the Accreditation Cycle for the institutional scope.
3. The Accreditation Coordinator defines the assessment timeline and review calendar.
4. H09 identifies participating units and allocates source-defined responsibilities.
5. H09 defines the cycle deliverables, responsible owners, and milestone schedule.
6. The plan references the governing framework criteria, metrics, and evidence expectations that later evidence collection must satisfy.
7. H09 establishes readiness/internal-review checkpoints and the downstream H09 SSR preparation/handoff obligation where the accreditation path requires an SSR package.
8. H09 preserves H08 as the owner of external SSR submission rather than assigning that responsibility to H09 or an evidence-owning domain.
9. The Steering Committee approves the major review cycle under the H09 governance structure.
10. H09 publishes the Accreditation Plan and closes the planning phase for progression into preparation.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Participating unit changes | Institutional scope changes before planning closure | Update the responsibility/deliverable allocation before approval and publication | Revised plan remains governed | Accreditation & Compliance |
| Evidence source changes | The authoritative source for a required evidence item changes | Update the plan reference while preserving source-domain ownership | Plan points to current authoritative evidence source | Accreditation & Compliance / evidence owner |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| No approved framework is available | Accreditation Cycle cannot be validly governed | Complete framework governance before later cycle planning | Accreditation & Compliance |
| Responsibilities or milestones are incomplete | Assessment plan is not ready for approval | Complete the planning content before governance approval/publication | Accreditation & Compliance |
| Evidence source cannot be authoritatively identified | Plan would contain an unsupported ownership assumption | Record the source gap and resolve it with the canonical evidence owner before relying on it | Accreditation & Compliance / evidence-owning domain |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Accreditation Cycle is associated with exactly one approved Accreditation Framework | H09 | Cycle planning cannot proceed against an unapproved framework | L2-H09 BR-H09-002 |
| Operational source domains remain systems of record | Evidence-owning domains | The plan references evidence sources without changing ownership | L2-H09 Principle 2; domain ownership matrix |
| H09 owns SSR preparation/review/approval; H08 owns SSR submission | H09 / H08 | Submission obligations are recorded without transferring outbound submission ownership into H09 | L2-H09 §2.6; BR-H09-038/039 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Accreditation Cycle | New cycle | Create and govern the assessment plan | Planning | H09 | L2-H09 H09-A02 |
| Accreditation Cycle | Planning | Complete planning and publish the approved plan | Preparation | H09 | L2-H09 H09-A02; H09-02 |

No separate Accreditation Plan aggregate or lifecycle is defined upstream; the plan is an H09-02 workflow output governed through the Accreditation Cycle.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Accreditation Cycle Created | Accreditation Cycle aggregate / H09 | Readiness, compliance and governance consumers | Records creation of the cycle | L2-H09 §11 |
| Accreditation Cycle Started | Accreditation Cycle aggregate / H09 | Readiness, dashboard and governance consumers | Records cycle start | L2-H09 §11 |

H09 defines no dedicated `Accreditation Plan Approved` or `Accreditation Plan Published` domain event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Evidence-owning domains remain authoritative for their source facts. The plan records evidence-source responsibilities and later H09 evidence requests. Where SSR submission is required, H09 owns preparation and approval before handoff; H08 owns the external submission.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Steering Committee is source-supported to approve major review cycles. H09-02 itself publishes an Accreditation Plan but does not define a separate plan approval state/event, so this Draft uses the Steering Committee's major-cycle approval as the business approval gate without creating a new plan lifecycle.

## 15.2 Separation of Duties, Overrides and Exceptions

Evidence-owning domains do not approve H09's institutional accreditation plan merely because their facts are referenced. External submission authority remains with H08 where applicable.

## 15.3 Business Audit and Evidence Obligations

Preserve framework version, cycle scope, assessment timeline, participating units, responsibility assignments, deliverables, milestones, evidence-source references, governance approval evidence, plan publication, and downstream handoff/submission obligations.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The cycle references one approved Accreditation Framework.
- Responsibilities, milestones, criteria/evidence expectations, review checkpoints, and downstream submission ownership are established.
- The major review cycle is approved under H09 governance and the plan is published.

## 16.2 Postconditions

The Accreditation Cycle can progress from planning into preparation under the governed assessment plan, while evidence-owning domains retain ownership of source facts.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

H09-02 defines publication of an `Accreditation Plan`, while the H09 governance structure defines Steering Committee approval of major review cycles. H09 does not define a separate Accreditation Plan aggregate, approval lifecycle, or plan-specific approval/publication event. The frozen approved-plan outcome is therefore representable through the approved cycle-governance gate, but the plan's standalone lifecycle semantics remain incomplete.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H09 may clarify whether Accreditation Plan is only a workflow output of Accreditation Cycle or requires its own explicit approval/event semantics.
