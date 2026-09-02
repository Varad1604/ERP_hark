# Eligible Payroll Period to Finalized and Published Payroll Outcome

## 1. Specification Metadata

**Specification ID:** L4-E-053  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-053 — Eligible Payroll Period to Finalized and Published Payroll Outcome  
**Primary Business Owner:** Payroll  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-053; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall execute an eligible payroll period through the authoritative Payroll Run lifecycle from creation through calculation, validation, approval and finalization, then publish the governed payroll outcomes. Payroll shall consume authoritative upstream facts without taking ownership of them, and Finance-owned payment/accounting remains outside this behavior.

## 3. Required Business Context

- An eligible payroll period and payroll type/scope are established.
- Authoritative Workforce, Attendance, Leave, Compensation, Compliance and Calendar facts are available.
- Applicable payroll rules and effective versions are available.
- An authorized Payroll Approver is available.
- The unique controlled Payroll Run condition is satisfied.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-053-BH-001 | The system shall establish the controlled Payroll Run for the eligible period and payroll type/scope in the canonical `Created` state. | L3-E-053 §§7–9, 13 |
| L4-E-053-BH-002 | The system shall determine payroll scope from authoritative upstream facts without acquiring ownership of those Workforce, Attendance, Leave, Compensation, Compliance or Calendar facts. | L3-E-053 §§4.3–4.4, 5, 9 |
| L4-E-053-BH-003 | The system shall calculate payroll using the applicable governed rules and progress the Payroll Run through `Calculating` to `Calculated`. | L3-E-053 §§9, 12–13 |
| L4-E-053-BH-004 | The system shall validate the calculated result and, when validation passes, progress the Payroll Run through `Validating` to `PendingApproval`. | L3-E-053 §§9, 11, 13 |
| L4-E-053-BH-005 | The system shall submit the validated Payroll Run to the authorized Payroll Approver and record the governed approval decision. | L3-E-053 §§6, 9, 12, 15 |
| L4-E-053-BH-006 | When approval is granted, the system shall progress the Payroll Run to `Approved` and then `Finalized`, preserving the finalized payroll as an immutable payroll record. | L3-E-053 §§3, 9, 12–13, 16 |
| L4-E-053-BH-007 | Where applicable, the system shall establish the governed payslip generation outcome after payroll finalization. | L3-E-053 §§4.3, 9, 10, 16 |
| L4-E-053-BH-008 | The system shall preserve calculation references, applicable rule versions, validation evidence, approval history and the finalized payroll snapshot required for reproducibility and auditability. | L3-E-053 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-053-BH-009 | When the Payroll Run is established, the system shall publish **PayrollRunCreated** under Payroll ownership. | L3-E-053 §14.1 |
| L4-E-053-BH-010 | On completed calculation, the system shall publish **PayrollCalculationCompleted** under Payroll ownership. | L3-E-053 §14.1 |
| L4-E-053-BH-011 | On completed validation, the system shall publish **PayrollValidationCompleted** under Payroll ownership. | L3-E-053 §14.1 |
| L4-E-053-BH-012 | On authorized approval, the system shall publish **PayrollApproved** under Payroll ownership. | L3-E-053 §14.1 |
| L4-E-053-BH-013 | On finalization, the system shall publish **PayrollFinalized** under Payroll ownership. | L3-E-053 §§4.2, 14.1 |
| L4-E-053-BH-014 | Where an applicable payslip outcome is generated, the system shall publish **PayslipGenerated** under Payroll ownership. | L3-E-053 §14.1 |
| L4-E-053-BH-015 | Finalized payroll outcomes may be consumed by Finance and other governed downstream participants, but payment execution and accounting posting remain owned by Finance and do not transfer to Payroll. | L3-E-053 §§4.4, 5, 14.2 |

## 6. Failure / Rejection / Continuation

- If a duplicate or invalid Payroll Run condition exists, the system shall not establish a second controlled run for the same governed scope.
- If calculation or validation findings remain unresolved, the Payroll Run shall not progress to authoritative approval/finalization.
- If validation does not pass, the Payroll Run shall not progress to `PendingApproval`.
- If approval is rejected or finalization is not authorized, the Payroll Run shall remain unfinalized and follow the governed Payroll review/correction path.
- Finalized payroll shall remain immutable; later corrections shall use the canonical Payroll Adjustment workflow.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of payroll scope, authoritative inputs consumed, applicable rules and versions, calculation references, validation result, approval decision, Payroll Run lifecycle transitions, finalized payroll snapshot and published payroll outcomes.

## 8. Observable Completion Conditions

Completion requires the Payroll Run to reach `Finalized`, remain immutable, have the applicable finalized Payroll events published, and have any required payslip outcome established. Payment execution and accounting remain downstream Finance responsibilities.

## 9. Traceability & Provenance

**Parent:** L3-E-053 — Eligible Payroll Period to Finalized and Published Payroll Outcome  
**Primary verification source:** Authoritative People & Workforce L2-E06 Payroll workflow architecture.

## 10. Open Issues / Deferred Design

No L3 open issue materially affects the event, lifecycle, ownership or scenario boundary. Institution-specific optional Finance review remains configuration governed and does not transfer Payroll ownership. Technical APIs, persistence schemas, UI, transport and retry mechanics are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
