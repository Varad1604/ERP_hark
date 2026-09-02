# Employee Separation to Final Payroll Settlement Result

## 1. Specification Metadata

**Specification ID:** L4-E-056  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-056 — Employee Separation to Final Payroll Settlement Result  
**Primary Business Owner:** Payroll  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-056; L2-E06 — Payroll Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Process an authoritative employee separation through the dedicated Final Settlement Payroll path and establish an immutable finalized payroll outcome. Workforce owns the separation fact; Payroll owns calculation/finalization; Finance owns payment and accounting.

## 3. Required Business Context

- Authoritative employee separation outcome and effective employment dates.
- Applicable final-settlement payroll rules/configuration.
- Required salary, deduction, recovery and leave-settlement facts.
- Authorized Payroll Approver.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-056-BH-001 | The system shall establish a Final Settlement Payroll Run for the separated employee using the supported final-settlement payroll type. | L3-E-056 §§3, 9, 12–13 |
| L4-E-056-BH-002 | The system shall determine applicable pending salary, deductions, recoveries and settlements from authoritative source facts and effective employment dates. | L3-E-056 §§7–9, 12 |
| L4-E-056-BH-003 | The system shall calculate and validate the final-settlement payroll result before approval. | L3-E-056 §§9, 11, 13 |
| L4-E-056-BH-004 | The system shall record the authorized Payroll Approver decision before finalization. | L3-E-056 §§6, 9, 15 |
| L4-E-056-BH-005 | When approval is granted, the system shall transition the applicable Payroll Run/Result to `Finalized`, establishing an immutable final-settlement payroll record. | L3-E-056 §§9, 12–13, 16 |
| L4-E-056-BH-006 | The system shall hand the finalized financial outcome to Finance without transferring Payroll-result ownership. | L3-E-056 §§5, 9, 14.2 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-056-BH-007 | The system shall publish `PayrollRunCreated`, `PayrollCalculationCompleted`, `PayrollValidationCompleted`, `PayrollApproved`, and `PayrollFinalized` at the corresponding canonical payroll milestones. | L3-E-056 §14.1 |
| L4-E-056-BH-008 | The authoritative separation/effective-employment facts shall remain owned by their upstream workforce source; Finance shall own downstream payment/accounting after the finalized Payroll outcome is handed off. | L3-E-056 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- Incomplete separation/effective-date information or missing settlement inputs prevents valid calculation/finalization.
- Unresolved validation findings prevent approval/finalization.
- Rejected approval leaves the settlement unfinalized under the governed Payroll review/correction path.
- A downstream Finance issue does not alter the authoritative Payroll Result.

## 7. Audit & Evidence Behavior

The system shall preserve applicable workforce/source references, settlement inputs, payroll rules/calculation references, validation result, approval history and finalized settlement outcome.

## 8. Observable Completion Conditions

A Final Settlement Payroll Run exists; applicable settlement components are calculated and validated; Payroll Approver approval is recorded; the Payroll Result reaches `Finalized`; the result is immutable; and Finance receives the finalized financial outcome.

## 9. Traceability & Provenance

**Parent:** L3-E-056 — Employee Separation to Final Payroll Settlement Result  
**Primary verification source:** L3-E-056 §§3, 5, 9, 11–16 and authoritative L2-E06 Payroll architecture.

## 10. Open Issues / Deferred Design

No parent-defined architecture issue remains unresolved for this L4 behavior. Technical payment mechanisms, accounting implementation, APIs, persistence and infrastructure are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
