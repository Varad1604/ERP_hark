# Employee Separation to Final Payroll Settlement Result

## 1. Specification Metadata

**Specification ID:** L4-E-056  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-056 — Employee Separation to Final Payroll Settlement Result  
**Primary Business Owner:** Payroll  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-056; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Process the payroll consequences of an authoritative employee separation through the final-settlement payroll lifecycle and establish the finalized final-settlement outcome. Creation or approval of separation, payment execution, and accounting remain outside this behavior.

## 3. Required Business Context

- Authoritative employee separation outcome and effective employment dates.
- Applicable final-settlement payroll rules.
- Salary, deduction, recovery and leave-settlement facts required for calculation.
- Authorized payroll approval authority.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-056-BH-001 | When an authoritative employee separation requires final payroll settlement, the system shall establish the final-settlement payroll scope. | L3-E-056 §§3,7–9 |
| L4-E-056-BH-002 | The system shall apply the applicable salary, deduction, recovery and settlement rules to the final-settlement payroll scope. | L3-E-056 §§3,4.3 |
| L4-E-056-BH-003 | The system shall calculate and validate the final-settlement payroll result. | L3-E-056 §§3,4.2,9 |
| L4-E-056-BH-004 | The system shall obtain the required payroll approval before finalization. | L3-E-056 §§6,9 |
| L4-E-056-BH-005 | When approval is granted, the system shall finalize the final-settlement payroll outcome as authoritative. | L3-E-056 §§3,4.2 |
| L4-E-056-BH-006 | The system shall make the finalized financial outcome available to Finance for downstream settlement and accounting responsibility. | L3-E-056 §§3,5 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-056-BH-007 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Payroll ownership. | L3-E-056 §14 |

## 6. Failure / Rejection / Continuation

- When required separation, employment-date, payroll-rule, or settlement facts are unavailable, the system shall not establish a valid finalized settlement outcome.
- When payroll validation or required approval does not complete successfully, the system shall not finalize the result.
- Payment execution and accounting remain downstream Finance responsibilities.

## 7. Audit & Evidence Behavior

The system shall preserve evidence of the separation context, applicable settlement rules, salary/deduction/recovery facts, approval decision and finalized settlement outcome.

## 8. Observable Completion Conditions

- Final-settlement payroll scope is established. - Applicable settlement rules are applied. - Result is calculated and validated. - Required approval is recorded. - Final-settlement payroll outcome is finalized. - Finalized outcome is available to Finance.

## 9. Traceability & Provenance

**Parent:** L3-E-056 — Employee Separation to Final Payroll Settlement Result  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
