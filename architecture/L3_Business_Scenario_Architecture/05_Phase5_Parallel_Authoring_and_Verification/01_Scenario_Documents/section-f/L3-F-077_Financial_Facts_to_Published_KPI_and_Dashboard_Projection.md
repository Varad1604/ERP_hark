# Financial Facts to Published KPI and Dashboard Projection

# 1. Scenario Metadata

**Scenario ID:** L3-F-077  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Enterprise dashboard and financial analytics consumption

# 2. Scenario Purpose

Calculate and publish authoritative financial KPIs and dashboard-ready financial projections from canonical Finance facts, then make those governed outputs available to H07 while Finance retains ownership of the calculations, KPI meaning and source financial information.

# 3. Business Outcome

Finance calculates and publishes authoritative KPIs and dashboard-ready projections for H07 consumption

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A reporting or dashboard refresh requires current governed financial information.

## 4.2 End Boundary

The scenario ends when the required financial KPI calculation and/or dashboard-ready projection has completed successfully and the canonical F13 business events have made the governed outcome available for H07 consumption.

## 4.3 In Scope

- Resolution of the applicable KPI Definition, Analytical Model and reporting configuration.
- Read-only consumption of canonical Finance facts.
- Financial KPI calculation.
- Dashboard-ready financial projection generation.
- Publication of the exact F13 calculation/projection events.
- H07 consumption without ownership transfer.

## 4.4 Out of Scope

- Creating, approving or activating a new KPI Definition or Analytical Model.
- Modifying canonical financial facts.
- Dashboard identity, composition, layout, widgets, audience, workspace or publication lifecycle.
- Inventing lifecycle states for KPI results or projections where F13 defines only completed business events.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Reporting & Analytics / F | Owns KPI Definition, Analytical Model, KPI calculation, dashboard-ready financial projections and their canonical business events. |
| Applicable Finance capabilities / F | Retain ownership of canonical source financial facts consumed by the calculation. |
| Enterprise Dashboard / H07 | Consumes governed Finance reports, KPIs and projections and owns dashboard composition/presentation only. |

# 6. Business Actors, Roles and Responsibilities

L2-F13 and L2-H07 both record Actors as absent in their canonical crosswalks. No human business actor is required to establish the frozen calculation/publication outcome, so no actor is invented. The actor gap is retained as a non-blocking issue for future governance clarification.

# 7. Preconditions and Entry Criteria

- A governed KPI Definition and/or Analytical Model applicable to the requested financial information is available.
- Required canonical Finance facts are available from their owning capabilities.
- Reporting configuration and financial scope are defined.
- H07 consumption, where required, is limited to governed Finance outputs.

# 8. Trigger

A reporting or dashboard refresh requires current governed financial information

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the required financial KPI/projection scope and current reporting context. | Financial Reporting & Analytics / F | Calculation context is established. |
| 2 | Resolve the applicable KPI Definition, Analytical Model and configuration versions. | Financial Reporting & Analytics / F | Governed calculation meaning is fixed. |
| 3 | Consume required canonical Finance facts without changing their source-owned state. | Financial Reporting & Analytics / F | Authoritative inputs are available. |
| 4 | Calculate the required financial KPI under the governed KPI Definition. | KPI Definition / F13 | `FinancialKPICalculated` records the completed financial KPI calculation. |
| 5 | Produce the required dashboard-ready financial projection under the governed Analytical Model. | Analytical Model / F13 | `FinancialDashboardProjectionUpdated` records the completed authoritative projection update. |
| 6 | Make the governed KPI/projection available to H07 under the approved read-model/projection boundary. | F13 → H07 | H07 may refresh its governed dashboard presentation. |
| 7 | Preserve calculation/projection traceability to source facts and configuration context. | Financial Reporting & Analytics / F | The frozen outcome remains reproducible and auditable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| KPI-only requirement | No dashboard projection is required | Complete the governed KPI calculation and publish `FinancialKPICalculated` | Authoritative KPI available to permitted consumers | F13 |
| Projection-only refresh from already current governed metrics | H07 requires a refreshed Finance projection without a new KPI calculation | Produce the governed projection under the applicable Analytical Model | `FinancialDashboardProjectionUpdated` makes the projection available | F13 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required canonical Finance fact is unavailable or inconsistent | Authoritative calculation cannot complete | Do not publish the affected successful calculation/projection event until authoritative inputs are available | Applicable Finance owner; F13 |
| Governed KPI Definition or Analytical Model cannot be resolved | Calculation meaning is not authoritative | Do not complete or publish the affected outcome | F13 |
| KPI calculation does not complete successfully | No authoritative KPI result exists | Do not publish `FinancialKPICalculated` | F13 |
| Projection generation does not complete successfully | No updated governed projection exists | Do not publish `FinancialDashboardProjectionUpdated` | F13 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Financial Reporting consumes canonical financial facts read-only | F13 / source Finance owners | Calculations cannot alter operational financial truth | L2-F13 FRP-01, FRP-02 |
| Financial Reporting owns KPI meaning and analytical models | F13 | H07 cannot become owner of the calculation | L2-F13 §§5, 16–17 |
| H07 owns dashboard identity, composition, workspace, widgets, audience and publication | H07 | Finance stops at the governed projection hand-off | L2-F13 accepted H07 boundary; L2-H07 |
| Configuration changes are version controlled and historical context preserved | F13 | Calculation/projection meaning remains reproducible | L2-F13 CP-04 |
| Successful KPI Calculation Workflow publishes `FinancialKPICalculated` | F13 | Exact completed business fact is used | L2-F13 §§28–29 |
| Successful Dashboard Projection Refresh Workflow publishes `FinancialDashboardProjectionUpdated` | F13 | Exact completed projection fact is used | L2-F13 §§28–29 |

# 13. State and Lifecycle Impact

F13 does not define separate lifecycle state machines for calculated KPI results or dashboard-ready projections. Canonical Finance facts and the governing KPI Definition/Analytical Model remain in their source-owned state; this scenario produces the exact completed business facts represented by `FinancialKPICalculated` and `FinancialDashboardProjectionUpdated` without inventing local lifecycle states.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| FinancialKPICalculated | KPI Definition / F13 | Dashboards, executive reporting, analytics | Records completed authoritative financial KPI calculation | L2-F13 §29 |
| FinancialDashboardProjectionUpdated | Analytical Model / F13 | H07 Enterprise Dashboard and other permitted consumers | Records an authoritative financial projection available for dashboard refresh | L2-F13 §29 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Applicable Finance capabilities | F13 | Canonical Finance facts | Provide authoritative calculation inputs without ownership transfer | L2-F13 upstream dependencies |
| F13 | H07 | Financial reports, KPIs and governed dashboard-ready projections | Supports dashboard composition while Finance retains fact/computation/report/KPI ownership | DEPENDENCY_MAP runtime/read-model register; L2-F13/H07 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

No scenario-specific human initiation or approval authority is established in F13/H07 for routine calculation/projection refresh. No role is invented.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception requirement is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

F13 requires workflow execution and configuration changes to preserve complete audit evidence; calculation/projection results retain traceability to their governed configuration and canonical inputs.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Required authoritative Finance inputs were used read-only.
- The applicable governed KPI Definition and/or Analytical Model was used.
- The required KPI/projection completed and the exact canonical event was published by its sole F13 publisher.
- H07 can consume the governed Finance output without ownership transfer.

## 16.2 Postconditions

Finance remains owner of financial facts, KPI meaning, analytical models and projections. H07 remains owner of dashboard composition and presentation.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| F13/H07 canonical crosswalks have no Actors sections | Human initiation/responsibility cannot be stated more specifically, but the event-defined frozen outcome remains complete | Clarify actors upstream if needed for later approval/authority design |
| F13 defines no separate KPI-result or projection lifecycle state machine | L3 must represent these as completed event-backed business facts rather than locally named states | Upstream lifecycle clarification only if future scenarios require state transitions |

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
