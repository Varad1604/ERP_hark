# Accreditation Cycle Initiation to Approved Assessment Plan

## 1. Scenario Metadata

**Scenario ID:** L3-H-062  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Establish the institutional assessment plan for an opened accreditation cycle, including responsibilities, milestones, evidence sources, review points, and submission obligations.

## 3. Business Outcome

Responsibilities, milestones, criteria, evidence sources, reviews, and submission obligations are established in an approved plan

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An accreditation or compliance cycle is opened for an institutional scope.

### 4.2 End Boundary

Responsibilities, milestones, criteria, evidence sources, reviews, and submission obligations are established in an approved plan.

### 4.3 In Scope

- H09-owned governance behaviour materially required to reach the frozen business outcome.
- Applicable source-domain references, evidence and governed hand-offs without ownership transfer.
- Only upstream-defined rules, lifecycle positions, events, approvals and evidence obligations relevant to this scenario.

### 4.4 Out of Scope

- Operational business-state ownership belonging to participating source domains.
- Adjacent downstream institutional outcomes after this scenario's End Boundary.
- Later architecture-level implementation concerns.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance (H) | Owns the H09 governance outcome and applicable H09 lifecycles. |
| VAR:Evidence-Owning Domain | Retains ownership of authoritative source evidence/business facts while supplying governed references or readiness evidence. |

Participation and evidence contribution do not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Coordinates accreditation planning and readiness review. | Reviews |
| Accreditation Coordinator | Creates and coordinates the accreditation plan. | Performs |
| Department Coordinators | Provide departmental responsibility and evidence-source planning. | Performs |
| Registrar | Participates in institutional planning. | Participates |
| Deans | Participate in institutional planning for applicable units. | Participates |

## 7. Preconditions and Entry Criteria

- An approved Accreditation Framework exists and governs the cycle.
- The institutional scope for the accreditation or compliance cycle is established.

## 8. Trigger

An accreditation or compliance cycle is opened for an institutional scope

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create the Accreditation Cycle against exactly one approved framework. | Accreditation & Compliance | The cycle is in Planning. |
| 2 | Define the assessment timeline, participating units, deliverables, responsibilities and owners. | Accreditation Coordinator / IQAC | Institutional responsibilities and milestones are established. |
| 3 | Identify criterion coverage, evidence sources and required review points, including contributions from `VAR:Evidence-Owning Domain`. | Accreditation & Compliance / VAR:Evidence-Owning Domain | Evidence-source and review planning is established without transferring source ownership. |
| 4 | Publish the Accreditation Plan. | Accreditation & Compliance | The H09-defined planning workflow reaches its published-plan output. |

> The authoritative flow cannot be extended to the frozen End Boundary without resolving the material issue in Section 17.1.

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The plan requires the frozen outcome of an approved assessment plan | H09 defines publication of the Accreditation Plan but no plan approval lifecycle, plan-approval event, or explicit approval decision for this plan. | Do not infer approval from publication; upstream architecture resolution is required. | Accreditation & Compliance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-H09-002 — every Accreditation Cycle references exactly one approved Accreditation Framework. | Accreditation & Compliance | Constrains cycle planning to one approved framework. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Accreditation Cycle | Planning | Planning activities establish cycle responsibilities and milestones. | Preparation | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A02 / §7 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Accreditation Cycle Created | Accreditation Cycle aggregate | Readiness Assessment; Compliance Review | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Accreditation Cycle Started | Accreditation Cycle aggregate | Readiness Assessment; Compliance Review | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| VAR:Evidence-Owning Domain | Accreditation & Compliance | Source Evidence Reference Contract | Identify authoritative source evidence and ownership for the plan. | L2-H09_Accreditation_and_Compliance_Workflows.md §23.1 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

The exact authority required to complete the frozen outcome is not established upstream; see Section 17.1.

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule for plan approval is defined by H09.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Cycle and plan establishment | Accreditation Cycle, published Accreditation Plan, responsibility matrix and milestone schedule | Accreditation & Compliance |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The frozen outcome requires an approved assessment plan containing responsibilities, milestones, criteria, evidence sources, reviews and submission obligations.
- This criterion cannot be fully proven from locked H09 until plan approval is defined upstream.

### 16.2 Postconditions

- The Accreditation Cycle remains governed by one approved framework.
- Source evidence ownership remains with `VAR:Evidence-Owning Domain`.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome requires an **approved** assessment plan, but H09-02 only defines `Publish Accreditation Plan` and does not define a plan lifecycle, plan-approval event, or explicit plan-approval authority. | The scenario cannot reach its frozen End Boundary without inventing an approval decision/authority. | Define the authoritative assessment-plan approval semantics in locked upstream architecture. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
