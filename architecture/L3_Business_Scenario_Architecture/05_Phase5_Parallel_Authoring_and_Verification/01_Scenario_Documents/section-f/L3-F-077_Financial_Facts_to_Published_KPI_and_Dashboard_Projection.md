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
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics Workflows; applicable Finance source domains; L2-H07 — Enterprise Dashboard Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Enterprise dashboards and authorized consumers of governed financial KPIs and projections

---

# 2. Scenario Purpose

Calculate authoritative financial KPIs and dashboard-ready projections from governed Finance facts under the applicable versioned definitions, then publish those outputs for H07 consumption without transferring ownership of Finance facts, KPI meaning, or analytical calculation to the dashboard domain.

---

# 3. Business Outcome

Finance calculates and publishes authoritative KPIs and dashboard-ready projections for H07 consumption

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a reporting or dashboard refresh requires current governed financial information.

## 4.2 End Boundary

The scenario ends when:

- the applicable KPI Definition, Analytical Model and reporting configuration versions have been resolved;
- canonical Finance facts have been consumed without mutation;
- the required financial KPIs have been calculated and published;
- dashboard-ready financial projections have been produced and published;
- H07 can consume the governed projections while retaining ownership of dashboard composition and publication; and
- calculation inputs, definition versions, outputs and publication facts are auditable and reproducible.

## 4.3 In Scope

- Authorization of the refresh or reporting context.
- Resolution of active KPI Definitions, Analytical Models and configuration versions.
- Read-only consumption of canonical Finance facts.
- KPI calculation.
- Creation of dashboard-ready projections and metrics.
- Validation of traceability and reproducibility.
- Publication of `FinancialKPICalculated` and `FinancialDashboardProjectionUpdated`.
- Governed hand-off to Enterprise Dashboard Workflows.
- Audit evidence for calculations and publications.

## 4.4 Out of Scope

- Creating or changing operational Finance transactions.
- Closing or reopening a Financial Period.
- Creating or approving a new KPI Definition or Analytical Model; that belongs to L3-F-076.
- Creating a Financial Report through the report lifecycle.
- Defining H07 dashboard identity, composition, layout, widgets, audience or publication.
- Treating a dashboard as the authoritative source of financial truth.
- Inventing a shared ownership model between F13 and H07.
- Technical ETL, cache, database, API, visualization-library or refresh-scheduling design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Upstream | L3-F-076 — Financial Definition Change to Approved KPI or Analytical Model | Governs the intended definition-change outcome; F077 only consumes effective definitions. |
| Related | L3-F-075 — Management Reporting Request to Governed Financial Report | Produces a governed Financial Report rather than KPI and dashboard projection publications. |
| Downstream | L2-H07 — Enterprise Dashboard Workflows | Consumes governed projections and owns dashboard composition and presentation. |
| Related | L3-F-079 — Financial Evidence to Audit and Historical Reconstruction | Uses preserved inputs, definitions and outputs for later reconstruction. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Reporting & Analytics / Section F | Primary owner. Owns KPI Definition, Analytical Model, KPI calculation, dashboard-ready financial projections and the canonical financial calculation events. |
| Applicable Finance source domains / Section F | Own canonical operational financial facts consumed read-only by reporting and analytics. |
| Enterprise Dashboard / Section H | Consumes governed financial projections and owns dashboard identity, composition, layout, widgets, audience and publication. |
| Authorization & Policy / Section A | Supplies centralized authorization decisions for protected calculation, publication and consumption activities. |
| Event Infrastructure / Section A | Distributes canonical calculation and projection events without owning their business facts. |
| Audit Infrastructure / Section A | Preserves immutable evidence and its governed lifecycle. |

The frozen participating-domain value remains exactly `F, H`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authorized Reporting Consumer | Initiates or relies on an authorized reporting or dashboard refresh. | May request the outcome; cannot redefine KPI meaning or change canonical Finance facts. |
| Financial Reporting & Analytics | Resolves governed definitions, calculates KPIs, produces projections and publishes canonical financial outputs. | Owns the business calculation and publication outcome. |
| Enterprise Dashboard | Consumes the governed projection for dashboard composition and presentation. | Does not own Finance facts, KPI calculation or analytical meaning. |

L2-F13 and L2-H07 do not name canonical human actor titles for this workflow. The role labels above describe domain responsibilities and do not introduce an approval authority.

---

# 7. Preconditions and Entry Criteria

- The refresh request belongs to a resolved Tenant and authorized reporting context.
- The requested KPI or projection is within the Financial Reporting & Analytics boundary.
- The applicable KPI Definition and Analytical Model can be resolved.
- The calculation and projection configuration versions are available.
- Required canonical Finance facts are available from their owning domains.
- Source data is consumed read-only and remains authoritative in its originating Finance domain.
- Applicable access, scope and confidentiality policies can be evaluated.
- H07 has an authorized consumption context where dashboard presentation is required.
- Correlation and audit context are available.

---

# 8. Trigger

A reporting or dashboard refresh requires current governed financial information

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the reporting or dashboard refresh requirement and establish Tenant, purpose, scope and correlation context. | Financial Reporting & Analytics | A governed refresh execution begins. |
| 2 | Authorize the requested calculation, publication and intended consumption scope. | Authorization & Policy; Financial Reporting & Analytics | Unauthorized or out-of-scope processing is excluded. |
| 3 | Resolve the applicable KPI Definition, Analytical Model and reporting configuration versions. | Financial Reporting & Analytics | Calculation meaning and configuration context are fixed for the execution. |
| 4 | Identify the required canonical Finance facts and their owning domains. | Financial Reporting & Analytics; applicable Finance source domains | Authoritative input sources are established. |
| 5 | Consume the required Finance facts without changing operational records or ownership. | Financial Reporting & Analytics | Governed calculation inputs are available read-only. |
| 6 | Validate input completeness, scope, period and traceability against the selected definitions. | Financial Reporting & Analytics | The execution is eligible for governed calculation. |
| 7 | Calculate the required financial KPI values under the resolved KPI Definition. | Financial Reporting & Analytics — KPI Definition | An authoritative KPI result is produced. |
| 8 | Preserve the KPI result with its input, definition-version, period, scope and correlation references. | Financial Reporting & Analytics | The KPI result is reproducible and auditable. |
| 9 | Publish `FinancialKPICalculated` from the KPI Definition business responsibility after successful calculation. | Financial Reporting & Analytics — KPI Definition; Event Infrastructure | Consumers receive the canonical KPI calculation fact. |
| 10 | Apply the resolved Analytical Model and projection rules to create the required dashboard-ready financial projection. | Financial Reporting & Analytics — Analytical Model | A governed projection suitable for H07 consumption is produced. |
| 11 | Validate that the projection remains traceable to canonical Finance facts, KPI results and configuration versions. | Financial Reporting & Analytics | The projection is not treated as an independent financial source of truth. |
| 12 | Publish `FinancialDashboardProjectionUpdated` from the Analytical Model business responsibility. | Financial Reporting & Analytics — Analytical Model; Event Infrastructure | The canonical projection update becomes available to governed consumers. |
| 13 | Make the projection available to H07 under applicable authorization and audience controls. | Financial Reporting & Analytics; Enterprise Dashboard | H07 can compose and publish the dashboard without assuming Finance ownership. |
| 14 | Preserve immutable evidence of authorization, inputs, definition versions, calculations, projections and publications. | Financial Reporting & Analytics; Audit Infrastructure | The completed refresh is traceable and reproducible. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| KPI-only refresh | The request requires a KPI but no dashboard projection. | Complete the governed KPI calculation and publish `FinancialKPICalculated`; do not create a projection. | The KPI is available to authorized consumers. | Financial Reporting & Analytics |
| Projection refresh using existing KPI | A valid current KPI result already exists for the required scope and configuration. | Validate its currency and lineage before applying the Analytical Model. | A new governed projection may be published without recalculating an unchanged KPI. | Financial Reporting & Analytics |
| Multiple Finance sources | The calculation requires facts from several Finance domains. | Resolve each canonical source and preserve all input lineage. | One governed result remains traceable to every authoritative input. | Financial Reporting & Analytics; Finance source domains |
| H07 presentation change | Dashboard layout or audience changes without a Finance calculation change. | H07 handles the presentation change under its own workflows. | Finance definitions and projection facts remain unchanged. | Enterprise Dashboard |
| No material value change | Recalculation produces the same governed value. | Preserve the execution and publication decision according to configuration and audit requirements. | No unsupported “changed” state is inferred. | Financial Reporting & Analytics |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization fails. | The requested calculation or publication cannot proceed. | Deny the protected activity and preserve the decision evidence. | Authorization & Policy |
| KPI Definition or Analytical Model cannot be resolved. | Calculation meaning is not governed. | Stop the execution; do not publish a success event. | Financial Reporting & Analytics |
| Required Finance facts are unavailable or incomplete. | A reliable result cannot be produced. | Retain the unsuccessful execution evidence and rerun only after authoritative inputs are available. | Finance source domain; Financial Reporting & Analytics |
| Input scope or period is inconsistent. | The result would not be comparable or reproducible. | Reject or correct the execution context before calculation. | Financial Reporting & Analytics |
| KPI calculation fails. | No authoritative KPI result exists. | Do not publish `FinancialKPICalculated` or derive a dependent projection. | Financial Reporting & Analytics |
| Projection calculation fails. | H07 cannot receive a governed updated projection. | Preserve the failure evidence; do not publish `FinancialDashboardProjectionUpdated`. | Financial Reporting & Analytics |
| Event publication condition is not satisfied. | Consumers must not receive an uncommitted success fact. | Publish only after the corresponding successful business outcome. | Financial Reporting & Analytics; Event Infrastructure |
| H07 consumption is unavailable. | The financial projection may exist but is not yet presented. | Preserve Finance ownership and publication evidence; H07 recovery remains within H07. | Enterprise Dashboard |

No technical retry, rollback, queue, cache or transport recovery design is introduced.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Financial Reporting & Analytics consumes canonical Finance facts read-only. | Financial Reporting & Analytics; Finance source domains | Calculation cannot mutate or replace operational records. | L2-F13 — Foundational Principles |
| Reports, KPIs and projections are not sources of operational financial truth. | Financial Reporting & Analytics | Every output must remain traceable to canonical facts. | L2-F13 — Foundational Principles |
| KPI definitions and analytical behaviour are configuration-driven and version controlled. | Financial Reporting & Analytics | Each result must bind to the configuration version used. | L2-F13 — Configuration-Driven Behaviour |
| `FinancialKPICalculated` is the canonical KPI calculation event. | Financial Reporting & Analytics | It is published only after successful calculation. | L2-F13 — Event Catalogue and Aggregate-to-Event Matrix |
| KPI Definition is the canonical publisher of `FinancialKPICalculated`. | Financial Reporting & Analytics | Event ownership is not transferred to H07 or Event Infrastructure. | L2-F13 — Aggregate-to-Event Matrix |
| `FinancialDashboardProjectionUpdated` is the canonical projection event. | Financial Reporting & Analytics | It represents a successful governed projection update. | L2-F13 — Event Catalogue and Aggregate-to-Event Matrix |
| Analytical Model is the canonical publisher of `FinancialDashboardProjectionUpdated`. | Financial Reporting & Analytics | H07 consumes but does not publish the Finance calculation fact. | L2-F13 — Aggregate-to-Event Matrix |
| H07 owns dashboard identity, composition, layout, widgets, audience and publication. | Enterprise Dashboard | Finance cannot define dashboard presentation as part of F077. | L2-F13 — H07 integration boundary; L2-H07 |
| Historical results must remain reproducible. | Financial Reporting & Analytics | Inputs, definitions, models and outputs require version lineage. | L2-F13 — Audit and Traceability; Configuration-Driven Behaviour |
| Protected processing remains subject to centralized authorization. | Authorization & Policy | A refresh request does not itself confer access. | L1; L2-A04 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Canonical Finance facts | Available in source domains | Consumed read-only. | Unchanged. | Applicable Finance source domains |
| KPI Definition | Applicable governed version available | Used to calculate the KPI. | Definition unchanged; a calculation result is produced. | Financial Reporting & Analytics |
| Analytical Model | Applicable governed version available | Used to produce the projection. | Model unchanged; a projection result is produced. | Financial Reporting & Analytics |
| KPI result | Not yet calculated for this execution | Calculated, traced and published. | Authoritative calculated result available. | Financial Reporting & Analytics |
| Dashboard-ready projection | Absent or requires refresh | Calculated, validated and published. | Current governed projection available for H07 consumption. | Financial Reporting & Analytics |
| Enterprise Dashboard | Existing H07-governed dashboard context | Consumes the projection; Finance does not change its lifecycle directly. | H07 determines its own presentation state. | Enterprise Dashboard |
| Audit evidence | Correlation context available | Receives calculation and publication evidence. | Immutable evidence available under audit governance. | Audit Infrastructure |

L2-F13 specifies events for successful calculation and projection update but does not define separate lifecycle state machines for KPI results or projections. This scenario therefore describes business outcomes without inventing additional state names.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Publication Condition | Business Effect / Consumer |
|---|---|---|---|
| `FinancialKPICalculated` | KPI Definition | The governed KPI calculation succeeds under the resolved definition. | Authorized reporting, analytics, audit and monitoring consumers receive the calculated KPI fact. |
| `FinancialDashboardProjectionUpdated` | Analytical Model | The governed dashboard-ready projection is successfully produced and validated. | H07 and authorized consumers receive the updated financial projection fact. |

Event Infrastructure governs delivery but does not become the publisher of either business fact.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Canonical Finance facts | Applicable Finance source domains | Financial Reporting & Analytics | Read-only authoritative inputs with period, scope and source references. |
| KPI Definition and configuration | Financial Reporting & Analytics | KPI calculation | Governed formula, dimensional, scope and version context. |
| Analytical Model and projection rules | Financial Reporting & Analytics | Projection calculation | Governed analytical and projection context. |
| Dashboard-ready financial projection | Financial Reporting & Analytics | Enterprise Dashboard | Governed projection values, dimensions, scope, period and lineage. |
| Dashboard presentation | Enterprise Dashboard | Authorized dashboard audience | H07-owned composition, layout, widgets, audience and publication. |
| Audit input | Financial Reporting & Analytics | Audit Infrastructure | Authorization, input, definition, calculation, projection, event and correlation evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- The refresh, calculation, publication and consumption must be authorized for the applicable Tenant, financial scope and audience.
- Access to an H07 dashboard does not transfer ownership of the underlying Finance facts or KPI definitions.
- A published projection does not bypass confidentiality, scope or explicit-deny rules.
- No technical role, claim, ACL or enforcement design is specified.

## 15.2 Approval and Separation of Duty

- L2-F13 requires governed definitions and version-controlled configuration but does not define a separate human approval step for each routine KPI or projection refresh.
- No approval role is invented for the refresh execution.
- Where configured approval policy applies, publication must satisfy it before a success event is emitted.
- Applicable SoD controls remain enforceable through Authorization & Policy.

## 15.3 Audit Obligations

The following must be auditable:

- Tenant, requester or initiating context and authorized purpose.
- KPI Definition, Analytical Model and configuration versions.
- Canonical source facts and their owning-domain references.
- Period, scope, dimensions and calculation time.
- KPI result and projection result.
- Validation and publication outcomes.
- `FinancialKPICalculated` and `FinancialDashboardProjectionUpdated` publication facts.
- H07 hand-off context.
- Actor, timestamp, correlation and final outcome.

---

# 16. Completion Criteria and Postconditions

The scenario is complete when:

- the refresh request and scope were authorized;
- the applicable KPI Definition, Analytical Model and configuration versions were resolved;
- canonical Finance facts were consumed read-only;
- input completeness and traceability were validated;
- required KPI values were calculated;
- `FinancialKPICalculated` was published by KPI Definition where a KPI calculation was required;
- the dashboard-ready projection was calculated and validated;
- `FinancialDashboardProjectionUpdated` was published by Analytical Model where a projection update was required;
- H07 could consume the governed projection without assuming Finance ownership;
- calculation and projection lineage was preserved; and
- immutable audit evidence was available.

Postconditions:

- Canonical operational Finance facts remain unchanged.
- KPI and Analytical Model definitions remain unchanged by the refresh.
- Finance retains ownership of KPI meaning, calculation and projection facts.
- H07 retains ownership of dashboard composition and presentation.
- Historical calculation results remain reproducible from their preserved versions and inputs.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Type | Issue / Deferred Question | Precise Scenario Impact | Disposition |
|---|---|---|---|---|
| F077-ISSUE-01 | Upstream granularity gap | L2-F13 defines the two canonical success events and their publishers but no explicit lifecycle states for KPI results or dashboard projections. | F077 can specify the calculation and publication outcomes but cannot name intermediate or terminal aggregate states beyond those outcomes. | Preserve outcome-based wording; define states upstream before L4 if stateful contracts are required. |
| F077-ISSUE-02 | Actor gap | L2-F13 and L2-H07 do not name canonical human actors for requesting or approving a refresh. | This Draft uses functional business-role labels and introduces no approval authority. | Define actors upstream if a human approval workflow is required. |

**Source Assessment:** PASS

No unsupported actor, state, transition, event, publisher, calculation method, dashboard ownership transfer or technical mechanism is assumed by this Draft.
