# Retrospective Change to Retroactive Pay Result

## 1. Specification Metadata

**Specification ID:** L4-E-055  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-055 — Retrospective Change to Retroactive Pay Result  
**Primary Business Owner:** Payroll  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-055; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Process the payroll impact of an authoritative retrospective compensation change and establish the resulting retroactive pay outcome. Compensation policy and originating change remain owned upstream; payment and accounting remain downstream.

## 3. Required Business Context

- Authoritative compensation change and effective date.
- Affected previously processed payroll periods.
- Applicable payroll rules and calculation context.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-055-BH-001 | When an authoritative compensation change affects previously processed payroll, the system shall identify the impacted payroll periods. | L3-E-055 §§3,7–9 |
| L4-E-055-BH-002 | The system shall calculate the financial difference resulting from the retrospective change for the impacted periods. | L3-E-055 §§3,4.3 |
| L4-E-055-BH-003 | The system shall establish the resulting difference through the governed arrear or retroactive-pay process. | L3-E-055 §§3,4.3 |
| L4-E-055-BH-004 | The system shall include the established retroactive pay result in the applicable governed payroll outcome. | L3-E-055 §§3,4.2,9 |
| L4-E-055-BH-005 | The system shall preserve the calculation and business audit history for the retrospective change and resulting pay outcome. | L3-E-055 §§3,4.3 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-055-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Payroll ownership. | L3-E-055 §14 |

## 6. Failure / Rejection / Continuation

- When the authoritative compensation change or applicable calculation context is unavailable, the system shall not establish a valid retroactive pay result.
- Historical finalized payroll shall not be directly edited to apply the retrospective change.
- Where an affected result requires broader correction, the governed Payroll Adjustment scenario remains the applicable correction boundary.

## 7. Audit & Evidence Behavior

The system shall preserve evidence of the authoritative compensation change, effective date, impacted payroll periods, calculation basis, resulting retroactive difference and resulting payroll outcome.

## 8. Observable Completion Conditions

- Impacted historical payroll periods are identified. - Retroactive difference is calculated. - Governed arrear/retro-pay result is established. - Result is included in the applicable payroll outcome. - Calculation and audit history are preserved.

## 9. Traceability & Provenance

**Parent:** L3-E-055 — Retrospective Change to Retroactive Pay Result  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
