# Internal Audit Plan to Closed Audit Report

## 1. Specification Metadata
**Specification ID:** L4-H-065  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accreditation & Compliance  
**Realizes Scenario:** L3-H-065  
**Primary Business Owner:** Accreditation & Compliance  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-065; L2-H09 — Accreditation & Compliance Workflows; applicable Locked source-domain and Platform Audit architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall execute a governed H09 internal audit from an authorized risk-based plan through scheduling, evidence review, findings/reporting, mandatory management acknowledgement and formal closure while preserving audited-domain operational ownership and Platform Audit infrastructure ownership.

## 3. Required Business Context
An approved Audit Plan, defined scope/team and required evidence exist; the Internal Auditor performs the audit; Management acknowledgement is mandatory before closure; unresolved Critical findings prohibit closure; H09 owns the compliance audit business outcome.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-065-BH-001 | The system shall admit the internal Audit only from an approved Audit Plan with defined scope/team and shall establish the Audit from the parent-supported `Planned` position. | L3-H-065 §§7–9,13 |
| L4-H-065-BH-002 | The system shall require approval before publishing the audit schedule and shall progress the Audit from `Planned` to `Scheduled` only after that governed approval condition is satisfied. | L3-H-065 §§9,12,15 |
| L4-H-065-BH-003 | During audit execution, the system shall preserve the rule that an auditor must not audit an organisational unit for which the auditor has operational ownership. | L3-H-065 §§9,12,15 |
| L4-H-065-BH-004 | The system shall record source-supported audit evidence, observations, findings and non-conformities and shall support progression into the parent-described `Reporting` position without altering audited-domain operational facts. | L3-H-065 §§5,9,12–13 |
| L4-H-065-BH-005 | The system shall require Management acknowledgement of the Audit Report before closure and shall preserve that acknowledgement as closure authority evidence. | L3-H-065 §§6,9,12,15–16 |
| L4-H-065-BH-006 | The system shall prevent closure while any unresolved Critical finding exists; required CAPA work shall continue under H09 governance until the closure prerequisite is satisfied. | L3-H-065 §§9,11–12,16 |
| L4-H-065-BH-007 | When all closure prerequisites are satisfied, the system shall place the Audit in `Closed` and preserve the formally closed Audit Report and governed audit history. | L3-H-065 §§9,13,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-065-BH-008 | For the corresponding H09 Audit facts, the system shall preserve the exact canonical event identities `Audit Planned`, `Audit Scheduled`, `Audit Started`, `Audit Completed`, `Audit Report Generated`, and `Audit Closed`; it shall not rename them into implementation-style event identifiers. | L3-H-065 §14 |
| L4-H-065-BH-009 | `VAR:Audited Domain` shall retain ownership of audited operational facts and provide governed evidence references; H09 owns the compliance audit outcome, while Platform Audit infrastructure ownership remains separate. | L3-H-065 §§5,14–15 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-065-BH-010 | Missing mandatory evidence, missing Management acknowledgement or an unresolved Critical finding shall keep the Audit in its applicable pre-closure position and prevent `Closed`. | L3-H-065 §§11–12,16 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-065-BH-011 | The system shall preserve Audit Plan/scope, schedule approval, auditor independence evidence, source evidence provenance, observations/findings, Audit Report, Management acknowledgement, Critical-finding/CAPA closure condition and complete Audit event/history evidence. | L3-H-065 §15 |

## 8. Observable Completion Conditions
The behavior is complete when the audit scope has been executed and documented, the Audit Report has Management acknowledgement, no unresolved Critical finding prevents closure, and the Audit is `Closed`; any CAPA created from findings continues independently under the H09 CAPA lifecycle.

## 9. Traceability & Provenance
**Parent:** L3-H-065 — Internal Audit Plan to Closed Audit Report  
**Primary verification source:** L2-H09 — Accreditation & Compliance Workflows.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Logging, storage and UI implementation remain deferred to later architecture levels.
