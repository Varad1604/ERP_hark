# Infrastructure Portfolio to Governed Report and Dashboard Projection

# 1. Scenario Metadata

**Scenario ID:** L3-G-320  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B, D, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows; L2-D03 — Timetable Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Authorised Infrastructure management, institutional governance, scheduling and dashboard consumers

# 2. Scenario Purpose

Produce authoritative Infrastructure reports, KPIs and dashboard-ready projections from G10-owned portfolio and operational facts while preserving B03 institutional-campus governance, D03 scheduling ownership and H07 dashboard-composition ownership.

# 3. Business Outcome

G10 produces authoritative portfolio, capacity, availability, compliance, condition, reservation and allocation reports, KPIs and dashboard-ready projections; H07 may compose them without owning infrastructure facts

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when management or an authorised institutional consumer requires current infrastructure insight.

## 4.2 End Boundary

The scenario ends when G10 has produced the governed Infrastructure report, KPI or dashboard-ready projection from authoritative source facts and made it available to authorised consumers, with H07 composition where applicable and without transferring source ownership.

## 4.3 In Scope

- Establishment of the authorised Infrastructure reporting scope.
- Resolution of G10-owned portfolio, capacity, availability, compliance, condition, reservation, allocation and lifecycle facts required by the requested insight.
- Preservation of B03 InstitutionalCampus context where institutional campus identity is required.
- Preservation of D03 ownership of timetable/scheduling decisions that consume Infrastructure context.
- G10 ownership of Infrastructure-derived reports, KPIs and governed projections under the controlling dependency-map ruling.
- H07 composition of source-owned projections without ownership of Infrastructure facts, KPI definitions or report logic.

## 4.4 Out of Scope

- H07 becoming an alternative source of Infrastructure truth or recomputing G10-owned KPIs as independent business facts.
- Changing Infrastructure state through a dashboard or report.
- Reassigning B03 institutional-campus governance or D03 timetable ownership.
- Inventing KPI formulas, thresholds, report lifecycles, dashboard layouts, widgets or presentation design.
- APIs, query technologies, storage, caches or implementation mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns canonical Infrastructure facts and, under `DEPENDENCY_MAP.md`, the computation, report, KPI and governed projection derived from G-owned facts. |
| Institution & Structure / Section B | Owns InstitutionalCampus identity/governance context referenced where applicable. |
| Academic Delivery / Section D | Consumes authoritative Infrastructure context for D-owned timetable/scheduling decisions; it does not own physical Infrastructure facts. |
| Institutional Services & Intelligence / Section H | H07 owns dashboard composition and presentation metadata while consuming source-owned G10 reports/KPIs/projections. |

Participation never transfers the underlying business fact or lifecycle away from its canonical owner.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Manager | Requests or reviews governed Infrastructure insight within authorised scope. | Uses G10-owned information for Infrastructure governance. |
| Infrastructure Administrator | Maintains authoritative Infrastructure facts from which reports/projections are produced. | Does not transfer source ownership to consumers. |
| Read-Only Auditor | Reviews source lineage and historical evidence where authorised. | Read-only evidence participation. |

H07's locked L2 crosswalk records no canonical Actors section, so no dashboard-specific human actor is invented.

# 7. Preconditions and Entry Criteria

- The relevant G10 Infrastructure source facts exist and are current according to their authoritative lifecycles.
- Any referenced Physical Campus is associated with the correct B03 InstitutionalCampus context where required.
- The requesting consumer is authorised for the requested information scope.
- Any report/KPI/projection semantics used are already governed by source architecture; unsupported formulas or thresholds are not introduced locally.

# 8. Trigger

Management or an authorised institutional consumer requires current infrastructure insight

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the authorised reporting scope and relevant G10 Infrastructure portfolio or resource population. | Infrastructure Management | The information request is bounded to authoritative source facts. |
| 2 | Resolve the required G10-owned portfolio, capacity, availability, compliance, condition, reservation, allocation and lifecycle information. | Infrastructure Management | Authoritative source facts are assembled without ownership transfer. |
| 3 | Preserve B03 InstitutionalCampus identity context and any D03 scheduling references as source-owned contextual inputs. | Infrastructure Management / Institution & Structure / Academic Delivery | Cross-domain references remain canonically owned. |
| 4 | Produce the governed Infrastructure report, KPI or dashboard-ready projection from G10-owned facts under the controlling repository ownership rule. | Infrastructure Management | G10 owns the resulting Infrastructure-derived information product. |
| 5 | Make the governed G10 information product available to authorised institutional consumers. | Infrastructure Management | Consumers receive current authoritative Infrastructure insight. |
| 6 | Where dashboard composition is required, H07 composes the G10-owned projection without recalculating or owning the underlying Infrastructure facts or KPI definition. | Enterprise Dashboard | Dashboard composition preserves source-domain ownership. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Requested insight includes facts owned outside G10 | B-, D-, H- or another-domain facts are required. | Consume those facts through their authoritative source context and do not relabel them as G10 facts. | Mixed-domain insight preserves canonical ownership. | Infrastructure Management / source-owning domain |
| Dashboard composition is not required | The consumer needs only the G10 report/KPI/projection. | Deliver the governed G10 information product directly. | Frozen G10 reporting outcome remains satisfied. | Infrastructure Management |
| Requested KPI definition is not governed upstream | No authoritative definition exists. | Do not invent the KPI; provide supported source facts/reporting and record the gap. | Unsupported business meaning is avoided. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Source Infrastructure facts are incomplete or inconsistent | The resulting insight would not be authoritative. | Reconcile or correct the source-owned G10 facts before treating the report/projection as governed. | Infrastructure Management |
| A consumer attempts to change G10 facts through the information product | Canonical ownership would be violated. | Reject the attempted state change; changes must occur through the applicable G10-owned business workflow. | Infrastructure Management / consuming domain |
| H07 composition is unavailable | Dashboard presentation is unavailable while the G10 information product remains valid. | Preserve the source-owned report/projection for later authorised composition. | Enterprise Dashboard |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 is the canonical owner of physical Infrastructure and its authoritative business facts. | Infrastructure Management | Report content and Infrastructure-derived meaning remain source-owned by G10. | L2-G10 §§1–8 |
| G10 provides operational information for reporting. | Infrastructure Management | Infrastructure reporting is a source-supported domain responsibility. | L2-G10 §50 |
| H07 consumes financial and campus-operational reports, KPIs and governed projections from Sections F and G; source domains retain fact, computation, report and KPI ownership. | Repository governance | The controlling repository dependency ruling assigns G10 ownership of G-derived reports/KPIs/projections. | `DEPENDENCY_MAP.md` Runtime Contract and Event Consumption Register |
| H07 owns dashboard composition and does not own business computation, reporting logic or KPI calculation. | Enterprise Dashboard | H07 cannot become a second Infrastructure reporting owner. | L2-H07 §§1.1–1.8 |
| D03 is the exclusive authoritative owner of timetable creation and scheduling operations. | Academic Delivery — Timetable | G10 Infrastructure insight may inform D03, but cannot take over timetable decisions. | L2-D03 §1 |
| B03 owns InstitutionalCampus identity/governance while G10 owns the physical Campus. | Institution & Structure / Infrastructure Management | Campus context in reporting must preserve the institutional/physical ownership split. | L2-B03 §§1–4; L2-G10 §§1–2 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure aggregates | Existing source-defined states | Read for authoritative reporting/projection. | No state change is required by the reporting outcome. | Infrastructure Management | L2-G10 |
| Infrastructure Portfolio / membership | Existing G10 portfolio context | Used as a governed reporting grouping without altering physical hierarchy or ownership. | Unchanged. | Infrastructure Management | L2-G10 portfolio sections |
| InstitutionalCampus | Existing B03 state | Referenced for institutional context where needed. | Unchanged. | Institution & Structure | L2-B03 |
| Timetable | Existing D03 state | May consume Infrastructure insight for D-owned scheduling decisions. | D03 lifecycle remains independently governed. | Academic Delivery — Timetable | L2-D03 |
| Dashboard | Existing H07 lifecycle where composition is requested | Composes source-owned projection only. | H07 lifecycle remains independently governed. | Enterprise Dashboard | L2-H07 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

This frozen scenario is a governed reporting/projection outcome and does not require a new report-publication event. Underlying G10 source changes continue to use their existing canonical events. H07 may use approved G10 projections for composition without acquiring source ownership.

No `InfrastructureReportGenerated`, `InfrastructureKPIUpdated` or similar event is invented because the reviewed sources do not define one as mandatory.

## 14.2 Business Contracts and Cross-Domain Hand-offs

- G10 provides the authoritative Infrastructure report/KPI/projection.
- B03 provides InstitutionalCampus identity/governance context where required.
- D03 consumes relevant physical-resource context for D-owned timetable/scheduling decisions.
- H07 consumes G-owned reports/KPIs/projections and owns only dashboard composition.

These hand-offs do not transfer state ownership or lifecycle authority.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Request/review Infrastructure insight | Infrastructure Manager or other authorised institutional consumer | Access remains limited to permitted information scope. | Infrastructure Management |
| Maintain underlying Infrastructure facts | Infrastructure Administrator | Must act within G10 business rules and authority. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific universal report approval, KPI approval or separation-of-duty rule is defined in the reviewed locked sources. None is invented.

## 15.3 Business Audit and Evidence Obligations

Underlying G10 source changes remain auditable. Where report/KPI/projection governance is itself source-defined, lineage must preserve the authoritative G10 source population and governed definition/version without altering historical business facts.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The reporting scope and consumer authorization are established.
- Required Infrastructure facts are resolved from canonical G10 sources.
- B03 and D03 contextual facts remain externally owned.
- G10 produces the authoritative Infrastructure report, KPI or dashboard-ready projection using governed source semantics.
- H07, where used, composes the source-owned projection without taking ownership of Infrastructure facts, computation or KPI definitions.

## 16.2 Postconditions

- G10 remains authoritative for Infrastructure facts and G-derived reports/KPIs/projections under the controlling dependency-map ruling.
- H07 retains dashboard-composition ownership only.
- B03 retains InstitutionalCampus governance and D03 retains timetable/scheduling ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Lower-level G10 integration wording includes statements that consumers are responsible for Analytics and Reporting, while the higher-authority `DEPENDENCY_MAP.md` explicitly states that H07 consumes Section G reports, KPIs and governed projections and that source domains retain computation/report/KPI ownership. | Repository precedence resolves the frozen scenario in favour of G10 ownership for G-derived reports/KPIs, but lower-level wording remains internally inconsistent. | Reconcile the lower-authority G10 wording with the controlling dependency-map ruling through architecture change control. |
| G10 defines reporting responsibility but no single canonical end-to-end report/KPI lifecycle or mandatory publication event for this scenario. | L3 can state the business outcome and ownership-safe hand-offs but cannot invent report states, formulas or events. | Future upstream refinement may define report/KPI governance where additional lifecycle precision is required. |
| H07's locked canonical crosswalk has no Actors section. | No canonical H07 human actor can be assigned in this L3 scenario. | Define H07 actors upstream if later architecture requires explicit human authority. |

**Architecture assessment:** `PASS WITH ISSUES`

The higher-authority dependency map resolves report/KPI/projection ownership and supports the frozen outcome. Remaining lower-level wording and actor/report-lifecycle gaps are non-blocking.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
