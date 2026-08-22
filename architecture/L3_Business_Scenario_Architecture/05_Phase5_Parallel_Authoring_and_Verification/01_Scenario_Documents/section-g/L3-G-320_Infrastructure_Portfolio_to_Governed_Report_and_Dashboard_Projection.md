# Infrastructure Portfolio to Governed Report and Dashboard Projection

# 1. Scenario Metadata

**Scenario ID:** L3-G-320  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B, D, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows; L2-D03 — Timetable Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Infrastructure management, institutional governance, operational consumers, and Enterprise Dashboard composition

# 2. Scenario Purpose

Produce governed Infrastructure reports, KPIs, and dashboard-ready projections from G10-owned portfolio, capacity, availability, compliance, condition, reservation, allocation, and lifecycle facts while preserving source ownership and allowing H07 to compose those projections without becoming the system of record.

# 3. Business Outcome

G10 produces authoritative portfolio, capacity, availability, compliance, condition, reservation and allocation reports, KPIs and dashboard-ready projections; H07 may compose them without owning infrastructure facts

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when management or an authorised institutional consumer requires current infrastructure insight.

## 4.2 End Boundary

The scenario ends when G10 has produced the authoritative governed infrastructure report/KPI/projection and the result is available to authorised consumers, including H07 composition where applicable, without transferring ownership of underlying facts.

## 4.3 In Scope

- Selection of source-owned G10 portfolio and Infrastructure facts relevant to the requested insight.
- G10-owned report/KPI/projection production under the controlling dependency-map ownership rule.
- Preservation of B03 InstitutionalCampus context where institutional campus identity is needed.
- Use of G10 Infrastructure information by D-owned operational consumers without transfer of scheduling/operational ownership.
- H07 composition of G10-owned projections without KPI or source-fact ownership transfer.

## 4.4 Out of Scope

- H07 recomputing Infrastructure KPIs or becoming the Infrastructure system of record.
- Modifying Infrastructure facts through a dashboard.
- Reassigning B03 InstitutionalCampus governance to G10 or H07.
- Reassigning D-owned academic scheduling or timetable lifecycle to G10.
- Inventing KPI formulas, thresholds, dashboard layouts, widgets, or presentation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns Infrastructure source facts and, under `DEPENDENCY_MAP.md`, the computation, report, KPI, and governed projection derived from those G-owned facts. |
| Institution & Structure / Section B | Owns InstitutionalCampus identity/governance context referenced by Infrastructure reporting where applicable. |
| Academic Delivery / Section D | Consumes authoritative physical-resource information for its own scheduling/operational decisions; it does not own Infrastructure facts. |
| Institutional Services & Intelligence / Section H | H07 composes approved G10 reports/KPIs/projections into dashboards without owning source facts, report logic, or KPI definitions. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Maintains the authoritative Infrastructure facts from which governed reporting is produced. | Does not transfer source ownership to consuming domains. |
| Infrastructure Manager | Requests or reviews Infrastructure operational/portfolio insight within authorised scope. | Uses G10 authoritative information for Infrastructure governance. |
| Read-Only Auditor | Reviews report lineage and Infrastructure history where authorised. | Read-only evidence participation. |

H07's locked L2 document does not define a canonical business-actor section; no dashboard-specific actor is invented here.

# 7. Preconditions and Entry Criteria

- The relevant G10 Infrastructure facts exist and are current under their canonical lifecycles.
- Any referenced Physical Campus is associated with the correct B03 InstitutionalCampus context.
- The requesting management or institutional consumer is authorised for the requested information.
- Report/KPI/projection semantics use only source-supported G10 facts and governed definitions; no unsupported formula is introduced.

# 8. Trigger

Management or an authorised institutional consumer requires current infrastructure insight

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the authorised reporting scope and relevant Infrastructure portfolio or physical-resource population. | Infrastructure Management | The requested insight is bounded to G10-owned facts. |
| 2 | Resolve the authoritative G10 portfolio, capacity, availability, compliance, condition, reservation, allocation and lifecycle information required by the request. | Infrastructure Management | Source facts remain canonical and current. |
| 3 | Preserve B03 InstitutionalCampus context and any D-owned operational references without importing their lifecycles or rules into G10. | Infrastructure Management / B / D | Cross-domain references remain ownership-safe. |
| 4 | Produce the governed report, KPI, and dashboard-ready projection from source-owned Infrastructure facts according to the controlling repository ownership rule. | Infrastructure Management | G10 owns the Infrastructure-derived report/KPI/projection. |
| 5 | Make the governed result available through approved G10 read/reporting contracts to authorised institutional consumers. | Infrastructure Management | Consumers receive a stable Infrastructure-owned information product. |
| 6 | Where dashboard composition is required, H07 composes the G10-owned projection without recalculating or owning the Infrastructure facts/KPI definition. | Enterprise Dashboard | Dashboard composition preserves source-domain ownership. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Requested insight requires a fact owned outside G10 | A report request includes a B-, D-, or other domain fact. | Keep the external fact source-owned and consume it only through an approved reference/contract; do not convert it into a G10 fact. | Ownership remains canonical. | Infrastructure Management / source domain |
| H07 composition is not required | The authorised consumer needs only the G10 report/projection. | Deliver the governed G10 information product without dashboard composition. | Frozen G10-owned reporting outcome still exists. | Infrastructure Management |
| Requested KPI definition is not authoritative | No governed source defines the requested KPI semantics. | Do not invent the formula; return the available source facts/report and record the definition gap for governance. | Unsupported KPI is not produced. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Source Infrastructure facts are incomplete or inconsistent | Report/KPI would be unreliable. | Correct or reconcile the G10 source facts before treating the result as authoritative. | Infrastructure Management |
| Consumer attempts to modify G10 facts through the projection | Canonical ownership would be violated. | Reject the ownership transfer; changes must occur through G10-owned workflows. | Infrastructure Management / H07 |
| H07 composition is unavailable | Dashboard presentation is interrupted but G10 source ownership and report/KPI remain intact. | Preserve the G10 information product for later authorised composition. | Enterprise Dashboard |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 owns canonical physical Infrastructure and its authoritative facts. | Infrastructure Management | All Infrastructure report content must originate from G10-owned facts. | L2-G10 §§1–8 |
| G10 provides operational information for reporting, including capacity, utilisation, reservation, compliance, inspection, sustainability and lifecycle reports. | Infrastructure Management | Reporting is a source-supported G10 responsibility. | L2-G10 §50 |
| `DEPENDENCY_MAP.md` states H07 consumes financial and campus-operational reports, KPIs and governed projections from Sections F and G; source domains retain fact, computation, report and KPI ownership. | Repository governance | Resolves reporting/KPI ownership in favour of G for G-owned Infrastructure facts. | `DEPENDENCY_MAP.md` Runtime Contract and Event Consumption Register |
| H07 owns dashboard composition and does not own business computation, reporting logic, or KPI calculation. | Enterprise Dashboard | H07 cannot become an alternative Infrastructure reporting owner. | L2-H07 §§1.1–1.8 |
| Portfolio membership supports governance/reporting and never changes physical hierarchy, lifecycle, or ownership. | Infrastructure Management | Portfolio reporting cannot redefine Infrastructure ownership. | L2-G10 §§157–176 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure aggregates | Existing source-defined states | Read for authoritative reporting; no state change is required. | Unchanged. | Infrastructure Management | L2-G10 |
| Infrastructure Portfolio / membership | Existing G10 portfolio state/membership | Used as a strategic reporting view; no physical hierarchy or lifecycle change. | Unchanged. | Infrastructure Management | L2-G10 §§168–176 |
| Dashboard | Existing H07 lifecycle where composition is requested | Composes source-owned projection only. | H07 lifecycle remains independently governed. | Enterprise Dashboard | L2-H07 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

This scenario is primarily a governed read/reporting outcome and does not require inventing a new reporting event. Under G10, source changes continue to publish their existing canonical events (for example portfolio membership, capacity, availability, compliance, reservation, allocation, or lifecycle events). H07 may refresh its composition from approved source projections without changing source ownership.

## 14.2 Business Contracts and Cross-Domain Hand-offs

- G10 provides authoritative read/reporting information and governed projections.
- B03 provides InstitutionalCampus identity/governance context when needed.
- D consumes physical Infrastructure information for D-owned operational scheduling decisions.
- H07 consumes G-owned reports/KPIs/projections and owns only dashboard composition.

No direct business-state modification is transferred through these read hand-offs.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Request/review Infrastructure insight | Infrastructure Manager or other authorised institutional consumer | Access is limited to the authorised Infrastructure scope and governed information product. | Infrastructure Management |
| Compose G10 projection into dashboard | H07-owned dashboard workflow | Composition cannot change source facts or KPI ownership. | Enterprise Dashboard |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific report approval or separation-of-duty requirement is defined by the reviewed sources. None is invented.

## 15.3 Business Audit and Evidence Obligations

Underlying G10 source changes remain fully auditable. Where the governed report/KPI/projection definition or publication is itself governed, its lineage must identify the authoritative G10 source facts and applicable definition/version without rewriting historical source records.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Reporting scope is authorised.
- Source facts are resolved from their canonical G10 records.
- B/D contextual references remain externally owned.
- G10 produces the required authoritative report, KPI, and dashboard-ready projection using governed source-domain semantics.
- H07, if used, composes the projection without source-fact or KPI ownership transfer.

## 16.2 Postconditions

- G10 remains authoritative for Infrastructure facts and Infrastructure-derived report/KPI/projection ownership under the controlling dependency map.
- H07 retains dashboard-composition ownership only.
- B03 and D retain their respective institutional and academic-delivery ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 §44A.198 says consumers are responsible for Analytics and Reporting and §44A.186 excludes calculated KPIs/consumer projections from event payloads, while higher-authority `DEPENDENCY_MAP.md` explicitly states H07 consumes Section G reports, KPIs and governed projections and that source domains retain computation/report/KPI ownership. | There is a wording conflict inside the lower-authority G10 integration section. Repository precedence resolves ownership to Section G for G-owned reports/KPIs, but the L2 wording remains inconsistent. | Reconcile G10 §44A.186/198 with the controlling dependency-map ruling without changing the frozen L3 outcome. |
| G10 defines reporting responsibilities but does not define one canonical end-to-end report/KPI workflow or publication event for this frozen scenario. | L3 can describe the business outcome and read hand-off but cannot invent formulas, report lifecycle states, or a reporting event. | Future upstream refinement may define report/KPI governance contracts where greater specificity is required. |
| H07's locked canonical-section crosswalk records no Actors section. | A dashboard-specific business actor cannot be added solely for this scenario. | Define H07 actors upstream if later L3/L4 work requires explicit human authority. |

**Architecture assessment:** `PASS WITH ISSUES`

The higher-authority dependency map resolves report/KPI ownership and the frozen outcome is usable. Remaining L2 wording and actor/workflow gaps are non-blocking for this Draft.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
