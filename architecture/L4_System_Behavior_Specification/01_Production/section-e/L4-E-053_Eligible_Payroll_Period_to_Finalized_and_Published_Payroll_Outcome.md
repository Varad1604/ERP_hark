# Eligible Payroll Period to Finalized and Published Payroll Outcome

## 1. Specification Metadata

**Specification ID:** L4-E-053  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-053 — Eligible Payroll Period to Finalized and Published Payroll Outcome  
**Primary Business Owner:** Payroll  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-053; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Execute an eligible payroll period through controlled calculation, validation, approval, finalization and publication of the authoritative payroll outcome. Payment execution, accounting posting, and upstream fact creation remain outside Payroll ownership.

## 3. Required Business Context

- Eligible payroll period and payroll type.
- Authoritative workforce, attendance, leave, compensation, compliance and calendar facts.
- Applicable payroll rules and effective versions.
- Authorized Payroll Approver.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-053-BH-001 | When an eligible payroll period is opened, the system shall establish the controlled Payroll Run for the applicable period and payroll type. | L3-E-053 §§3,7–9 |
| L4-E-053-BH-002 | The system shall determine payroll scope from authoritative upstream facts without assuming ownership of those facts. | L3-E-053 §§4.3,5,9 |
| L4-E-053-BH-003 | The system shall calculate payroll using the applicable approved compensation, attendance, leave, compliance and other governed rules. | L3-E-053 §§3,9 |
| L4-E-053-BH-004 | The system shall validate calculated payroll results before submission for authorization. | L3-E-053 §§3,9 |
| L4-E-053-BH-005 | The system shall record the authorized Payroll Approver decision before finalization. | L3-E-053 §§6,9 |
| L4-E-053-BH-006 | When approval is granted, the system shall finalize the Payroll Run as the authoritative immutable payroll record. | L3-E-053 §§3,9 |
| L4-E-053-BH-007 | Where applicable, the system shall establish the governed payslip outcome. | L3-E-053 §§4.3,9 |
| L4-E-053-BH-008 | The system shall publish the authoritative payroll outcome after finalization. | L3-E-053 §§3,4.2,9 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-053-BH-009 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Payroll ownership. | L3-E-053 §14 |

## 6. Failure / Rejection / Continuation

- When required upstream facts or applicable payroll rules are unavailable, the system shall not establish a valid finalized payroll outcome.
- When validation does not pass, the system shall not submit the payroll as ready for approval.
- When approval is not granted, the Payroll Run shall not progress to Finalized.
- Finalized payroll shall remain immutable; later corrections are handled through separate canonical adjustment scenarios.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of the authoritative upstream facts considered, applicable rules, validation result, approval decision, final payroll outcome and material publication outcome.

## 8. Observable Completion Conditions

- Eligible Payroll Run is established. - Calculation and validation are complete. - Authorized approval is recorded. - Payroll reaches Finalized and is immutable. - Applicable payslip outcome is established. - Finalized payroll outcome is published.

## 9. Traceability & Provenance

**Parent:** L3-E-053 — Eligible Payroll Period to Finalized and Published Payroll Outcome  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
