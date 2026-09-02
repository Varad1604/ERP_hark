# Retrospective Change to Retroactive Pay Result

## 1. Specification Metadata

**Specification ID:** L4-E-055  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-055 — Retrospective Change to Retroactive Pay Result  
**Primary Business Owner:** Payroll  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-055; L2-E06 — Payroll Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Process the financial effect of an authoritative retrospective compensation change across affected prior payroll periods without editing historical finalized payroll. Compensation remains upstream-owned; Payroll owns the retro-pay calculation and resulting payroll outcome; Finance owns downstream payment and accounting.

## 3. Required Business Context

- Authoritative compensation change and effective date.
- Identifiable previously processed payroll periods affected by that change.
- Applicable payroll rule/version and calculation context.
- Required Payroll approval authority.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-055-BH-001 | The system shall identify every previously processed payroll period affected by the authoritative retrospective compensation change. | L3-E-055 §§3, 7–9, 12 |
| L4-E-055-BH-002 | The system shall calculate the difference between the prior processed outcome and the outcome applicable under the retrospective change for each affected period. | L3-E-055 §§3, 9, 12 |
| L4-E-055-BH-003 | The system shall establish the resulting difference through the governed arrear/retro-pay transaction rather than directly editing historical finalized payroll. | L3-E-055 §§4.3, 9, 12 |
| L4-E-055-BH-004 | The system shall validate the resulting retroactive payroll calculation before approval. | L3-E-055 §§9, 11, 16 |
| L4-E-055-BH-005 | The system shall record the authorized Payroll Approver decision before finalization. | L3-E-055 §§6, 9, 15 |
| L4-E-055-BH-006 | When approval is granted, the system shall include the retroactive pay result in the applicable governed payroll outcome and finalize that outcome without changing the original finalized historical payroll record. | L3-E-055 §§3, 9, 13, 16 |
| L4-E-055-BH-007 | The system shall preserve impacted-period, input, rule/version, calculation, approval and resulting payroll history sufficient for reproducibility. | L3-E-055 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-055-BH-008 | The system shall publish `PayrollAdjustmentCreated` when the controlled retrospective correction workflow is initiated. | L3-E-055 §14.1 |
| L4-E-055-BH-009 | The system shall publish `PayrollCalculationCompleted`, `PayrollValidationCompleted`, `PayrollApproved`, and `PayrollFinalized` at their corresponding canonical Payroll milestones. | L3-E-055 §14.1 |
| L4-E-055-BH-010 | The authoritative compensation change shall be consumed from its upstream owner; the finalized payroll financial outcome shall be handed to Finance without transferring Payroll calculation ownership. | L3-E-055 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- If the affected period or authoritative compensation change cannot be established, no retroactive pay result shall be finalized.
- If calculation validation fails or approval is not granted, the resulting payroll outcome shall remain unfinalized.
- Broader correction remains governed by the Payroll Adjustment boundary; historical finalized payroll shall not be directly edited.

## 7. Audit & Evidence Behavior

The retrospective change, effective date, impacted periods, calculation basis, applicable rule/version, approval decision and resulting retroactive amount shall remain attributable and traceable.

## 8. Observable Completion Conditions

Completion requires impacted periods to be identified, the retroactive difference to be calculated and represented through the governed arrear/retro-pay process, validation and approval to complete, the resulting payroll outcome to be finalized, and the original historical finalized payroll record to remain unchanged.

## 9. Traceability & Provenance

**Parent:** L3-E-055 — Employee Retrospective Change to Retroactive Pay Result  
**Primary verification source:** L3-E-055 §§3, 5, 9, 11–16 and authoritative L2-E06 Payroll architecture.

## 10. Open Issues / Deferred Design

No parent-defined architecture issue remains unresolved for the L4 behavior. Technical APIs, persistence, queues, transport, UI and infrastructure mechanics remain deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
