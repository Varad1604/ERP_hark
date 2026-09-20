# Financial Definition Change to Approved KPI or Analytical Model

## 1. Specification Metadata

**Specification ID:** L4-F-076  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Financial Reporting & Analytics  
**Realizes Scenario:** L3-F-076  
**Primary Business Owner:** Financial Reporting & Analytics  
**Version:** 1.1.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-20  
**Depends On:** `L3-F-076_Financial_Definition_Change_to_Approved_KPI_or_Analytical_Model.md`; L2-F13 — Financial Reporting & Analytics Workflows; `L2-F13_KPI_and_Analytical_Model_Governance_Amendment_2026-09-20.md`; ADR-FIN-ANALYTICS-001  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an existing KPI Definition or Analytical Model through version-controlled successor creation, validation, approval, publication, activation and supersession while preserving canonical Finance-fact ownership and the exact historical definition/model version used by prior results.

The behavior begins when a governed definition/model change is initiated and ends when the approved successor is Active for new eligible calculations/projections, any replaced predecessor is Superseded, required governance events are published, and historical results remain reproducible.

## 3. Required Business Context

- The predecessor KPI Definition or Analytical Model and its historical configuration context are identifiable.
- The proposed change remains inside Financial Reporting & Analytics ownership.
- Canonical Finance inputs and source ownership are identifiable.
- Historical results must remain reproducible under their original definition/configuration context.
- The Financial Analytics Definition Approval Authority is available for the governed approval decision.
- The canonical Finance configuration lifecycle is available for KPI Definition / Analytical Model governance.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-076-BH-001 | The system shall preserve the predecessor KPI Definition or Analytical Model and its historical calculation/configuration context without in-place replacement. | L3-F-076 §§2–4, 9, 12, 16 |
| L4-F-076-BH-002 | The system shall represent a changed financial definition as a distinct version-controlled successor candidate. | L3-F-076 §§2–4, 9, 12 |
| L4-F-076-BH-003 | The system shall validate that the successor definition references canonical Finance information without modifying or re-owning the underlying financial facts. | L3-F-076 §§4–5, 7, 9, 11–12 |
| L4-F-076-BH-004 | The system shall preserve the information needed to reproduce historical results under the predecessor definition/configuration context after a successor candidate exists. | L3-F-076 §§2–4, 9, 12, 16 |
| L4-F-076-BH-005 | The system shall progress KPI Definition / Analytical Model versions only through the canonical `Draft → Validation → Approval → Published → Active → Superseded → Archived` governance lifecycle. | L3-F-076 §§9, 12–13; locked F13 amendment §1 |
| L4-F-076-BH-006 | The system shall recognize approval only when the Financial Analytics Definition Approval Authority records the governed decision; an unapproved candidate shall not become Published or Active. | L3-F-076 §§6, 9, 15; locked F13 amendment §2 |
| L4-F-076-BH-007 | Only an Active KPI Definition or Analytical Model version shall govern new eligible KPI calculations or analytical projections for its applicable scope/effective boundary. | L3-F-076 §§3, 9, 12–13; locked F13 amendment §1 |
| L4-F-076-BH-008 | When an Active successor replaces an Active predecessor for the same governed scope, the system shall transition the predecessor to Superseded without rewriting historical results. | L3-F-076 §§3, 9, 12–13, 16; locked F13 amendment §1 |
| L4-F-076-BH-009 | Historical KPI results, analytical projections and governed reports shall retain the exact definition/model version used when they were produced. | L3-F-076 §§2–4, 12, 16; locked F13 amendment §§1,5 |

## 5. Cross-Domain & Event Behavior

Applicable Finance capabilities retain ownership of source financial facts. Financial Reporting & Analytics owns the KPI Definition/Analytical Model meaning and version-controlled reporting configuration.

KPI Definition Aggregate / F13 shall publish `KPIDefinitionApproved`, `KPIDefinitionActivated`, `KPIDefinitionSuperseded`, and `KPIDefinitionArchived` for their corresponding successful business transitions.

Analytical Model Aggregate / F13 shall publish `AnalyticalModelApproved`, `AnalyticalModelActivated`, `AnalyticalModelSuperseded`, and `AnalyticalModelArchived` for their corresponding successful business transitions.

`FinancialKPICalculated` and `FinancialDashboardProjectionUpdated` remain calculation/projection completion facts and shall not be used as substitutes for governance lifecycle events.

## 6. Failure / Rejection / Continuation

If the proposed definition would modify source-owned Finance facts, lose historical configuration context, fail validation, or lack required approval, the system shall keep it outside Active use and preserve the current Active version. If activation would break predecessor/successor lineage or historical reproducibility, activation shall not occur.

## 7. Audit & Evidence Behavior

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-076-BH-010 | The system shall preserve immutable evidence of definition/model identity and version, governed scope, predecessor/successor lineage, validation result, approving authority and approval reference, Published/Active effective boundary, Superseded/Archived disposition, canonical source references and historical result-version references. | L3-F-076 §§12, 15–16; locked F13 amendment §5 |

## 8. Observable Completion Conditions

Completion requires a distinct successor version to pass validation, receive Financial Analytics Definition Approval Authority approval, become Published then Active, publish the applicable aggregate-specific governance events, supersede any replaced Active predecessor, preserve canonical Finance ownership, and retain historical results under their original governing version.

## 9. Traceability & Provenance

**Parent:** L3-F-076 — Financial Definition Change to Approved KPI or Analytical Model  
**Primary verification sources:** L2-F13 — Financial Reporting & Analytics Workflows; `L2-F13_KPI_and_Analytical_Model_Governance_Amendment_2026-09-20.md`; ADR-FIN-ANALYTICS-001

The locked 2026-09-20 F13 amendment resolves the former lifecycle, authority and event-publisher gap for KPI Definition / Analytical Model governance.

## 10. Open Issues / Deferred Design

No material upstream architecture issue remains. Implementation-specific UI, API, persistence, technical authorization and event-transport design remain deferred to later architecture levels.
