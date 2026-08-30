# Operational Risk Identification to Risk Closure

## 1. Specification Metadata

**Specification ID:** L4-G-005  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations Governance  
**Realizes Scenario:** L3-G-005  
**Primary Business Owner:** Campus Operations Governance  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-G-005_Operational_Risk_Identification_to_Risk_Closure.md`; L2-G00 — Campus Operations Review & Governance  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern one campus Operational Risk from registration through assessment, mitigation governance, monitoring and closure while preserving responsible operational-domain ownership of incidents and corrective execution.

The behavior begins when a qualifying multi-domain operational risk is identified and ends when the Operational Risk reaches `Closed` under configured governance closure conditions with mitigation, monitoring and audit evidence preserved.

## 3. Required Business Context

- The condition qualifies for G00 operational-risk governance.
- At least one responsible operational domain can be identified.
- Initial assessment evidence exists or its absence can be governed as an exception.
- Applicable risk category, severity, mitigation, escalation and closure rules are resolvable from active G00 configuration.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-005-BH-001 | The system shall prevent valid Operational Risk registration when no responsible operational domain can be referenced. | L3-G-005 §§7, 11–12, 16 |
| L4-G-005-BH-002 | On valid registration, the system shall place the Operational Risk at `Identified` and preserve its responsible-domain reference. | L3-G-005 §§9, 12–13 |
| L4-G-005-BH-003 | The system shall classify and assess the risk using active configured category and severity rules and shall place a validly assessed risk at `Assessed`. | L3-G-005 §§7, 9, 12–13 |
| L4-G-005-BH-004 | The system shall establish mitigation governance, including responsible owner/domain, target and applicable verification expectations, before progressing the risk into `Mitigation`. | L3-G-005 §§9, 12–13 |
| L4-G-005-BH-005 | The system shall preserve source-domain execution ownership while G00 records mitigation progress, dependencies, blockers and evidence. | L3-G-005 §§2, 4–5, 9, 12–13 |
| L4-G-005-BH-006 | The system shall progress the risk to `Monitoring` only through the source-defined risk lifecycle and shall evaluate residual exposure using governed risk rules. | L3-G-005 §§9–10, 12–13 |
| L4-G-005-BH-007 | When configured escalation conditions apply, the system shall preserve the governed escalation without changing Operational Risk ownership. | L3-G-005 §§9–10, 12, 14 |
| L4-G-005-BH-008 | The system shall prevent closure when mitigation evidence is incomplete or residual exposure does not satisfy configured closure conditions. | L3-G-005 §§10–12, 16 |
| L4-G-005-BH-009 | When configured closure conditions are satisfied, the system shall progress the Operational Risk from `Monitoring` to `Closed`. | L3-G-005 §§9, 12–13, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-005-BH-010 | On the corresponding canonical facts, the system shall publish `OperationalRiskRegistered`, `MitigationPlanCreated`, `OperationalRiskEscalated` where applicable, and `OperationalRiskClosed` under G00 ownership. | L3-G-005 §14 |
| L4-G-005-BH-011 | Any risk status or closure information made available to Section H shall remain G00-owned governance information; underlying incidents and remediation remain source-domain owned. | L3-G-005 §§5, 12, 14 |

## 6. Failure / Rejection / Continuation

Invalid classification shall not be treated as a valid assessment and must be reassessed using governed rules. Missing responsible-domain attribution blocks valid progression. Incomplete mitigation evidence or unacceptable residual exposure keeps the risk open in its valid lifecycle position. G00 shall not perform the underlying operational remediation.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-005-BH-012 | The system shall preserve risk registration, classification, severity, responsible domain, mitigation, monitoring, escalation, closure conditions, lifecycle transitions and canonical event history as auditable governance evidence. | L3-G-005 §§12, 15–16 |

## 8. Observable Completion Conditions

Completion requires the Operational Risk to reference a responsible operational domain and to have progressed `Identified → Assessed → Mitigation → Monitoring → Closed` under configured governance rules, with sufficient mitigation/monitoring evidence and applicable canonical events preserved. Source-domain incident and remediation ownership remains unchanged.

## 9. Traceability & Provenance

**Parent:** L3-G-005 — Operational Risk Identification to Risk Closure  
**Primary verification source:** L2-G00 — Campus Operations Review & Governance

## 10. Open Issues / Deferred Design

No L4 correctness issue remains. Detailed technical monitoring, notification, persistence, authorization implementation and event transport are downstream concerns.