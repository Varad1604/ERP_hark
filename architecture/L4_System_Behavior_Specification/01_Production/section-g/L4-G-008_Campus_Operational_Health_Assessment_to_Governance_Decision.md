# Campus Operational Health Assessment to Governance Decision

## 1. Specification Metadata

**Specification ID:** L4-G-008  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations Governance  
**Realizes Scenario:** L3-G-008  
**Primary Business Owner:** Campus Operations Governance  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-G-008_Campus_Operational_Health_Assessment_to_Governance_Decision.md`; L2-G00 — Campus Operations Review & Governance  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall evaluate the G00-owned Campus Operational Health view from governed source-domain readiness, SLA, risk, action and KPI information, support governance review of material drivers and preserve an appropriate decision or recommendation without inventing a Campus Operational Health lifecycle.

The behavior begins when consolidated operational information requires an institutional health assessment and ends when a supported governance decision or recommendation and its evidence lineage are preserved.

## 3. Required Business Context

- Authorized campus, period and assessment purpose can be resolved.
- Required source readiness, SLA, risk, action and KPI information is attributable to canonical owners.
- Applicable Campus Operational Health definition and governance configuration can be resolved.
- Any governance decision or recommendation is made only within authority actually available from upstream governance.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-008-BH-001 | The system shall establish the authorized assessment scope, period and purpose before calculating Campus Operational Health. | L3-G-008 §§7–9 |
| L4-G-008-BH-002 | The system shall obtain required readiness, SLA, risk, action and KPI information only from authoritative source references and shall preserve source ownership. | L3-G-008 §§4–5, 9, 12 |
| L4-G-008-BH-003 | The system shall validate source lineage, scope, currency and comparability and shall not produce a supported health outcome when material required information is missing or inconsistent. | L3-G-008 §§7, 9–12 |
| L4-G-008-BH-004 | When valid inputs and the configured governance definition are available, the system shall calculate or recalculate the G00-owned Campus Operational Health result without modifying source facts. | L3-G-008 §§2, 5, 9, 12–13 |
| L4-G-008-BH-005 | The system shall not assign lifecycle states to Campus Operational Health because no dedicated source-defined health lifecycle exists. | L3-G-008 §§4, 13, 17 |
| L4-G-008-BH-006 | The system shall support governance review of the calculated health result and its material drivers using source-defined G00 review behavior. | L3-G-008 §§6, 9, 13 |
| L4-G-008-BH-007 | The system shall record an appropriate governance decision or recommendation only within available source-supported authority and shall not infer a universal final decision role. | L3-G-008 §§6, 9, 15, 17 |
| L4-G-008-BH-008 | When adverse health requires follow-up, the system shall link only source-supported findings, risks, Action Plans or escalations and shall keep those lifecycles separate. | L3-G-008 §10 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-008-BH-009 | On successful recalculation, the system shall publish `CampusOperationalHealthUpdated` as the G00 health fact. | L3-G-008 §14 |
| L4-G-008-BH-010 | Where the corresponding source-defined outcome occurs, the system shall publish `GovernanceRecommendationPublished` and, where an applicable multi-domain review completes, `MultiDomainReviewCompleted`; neither event creates a health lifecycle. | L3-G-008 §§13–14 |
| L4-G-008-BH-011 | Any health result or governance outcome made available to Section H shall remain G00-owned and shall preserve source-domain ownership of contributing information. | L3-G-008 §§5, 14 |

## 6. Failure / Rejection / Continuation

Missing or inconsistent required source information shall cause the affected assessment to be deferred, corrected or escalated without an invented health result. If required governance authority for a proposed decision is unresolved, the system shall not represent that decision as institutionally authoritative. Follow-up shall use only canonical G00 aggregates and lifecycles.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-008-BH-012 | The system shall preserve assessment scope, source references, governing health definition/configuration, calculated result, material drivers, review evidence, decision or recommendation and applicable canonical event history. | L3-G-008 §§15–16 |

## 8. Observable Completion Conditions

Completion requires Campus Operational Health to have been calculated from valid governed sources, reviewed, and accompanied by a supported governance decision or recommendation within available authority, with source ownership and audit lineage intact. No dedicated health lifecycle state is required or permitted by this parent.

## 9. Traceability & Provenance

**Parent:** L3-G-008 — Campus Operational Health Assessment to Governance Decision  
**Primary verification source:** L2-G00 — Campus Operations Review & Governance

## 10. Open Issues / Deferred Design

The parent records two non-blocking gaps: no dedicated Campus Operational Health aggregate/lifecycle and no universal authority mapping for choosing between a decision and recommendation. This L4 specification preserves both gaps and does not invent either construct. Downstream implementation details remain deferred.