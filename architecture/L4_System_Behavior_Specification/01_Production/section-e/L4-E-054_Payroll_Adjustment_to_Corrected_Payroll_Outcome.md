# Payroll Adjustment to Corrected Payroll Outcome

## 1. Specification Metadata

**Specification ID:** L4-E-054  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-054 — Payroll Adjustment to Corrected Payroll Outcome  
**Primary Business Owner:** Payroll  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-054; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Process a governed payroll correction and establish a corrected payroll outcome without editing the finalized payroll record. Upstream fact correction, payment execution and accounting remain outside this behavior.

## 3. Required Business Context

- Identified payroll discrepancy or correction requirement.
- Business reason and basis for adjustment.
- Applicable source facts and payroll rules.
- Required approval authority.
- Existing finalized payroll result, which remains immutable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-054-BH-001 | When a payroll correction requirement is identified, the system shall establish the payroll adjustment case and record its business reason. | L3-E-054 §§3,7–9 |
| L4-E-054-BH-002 | The system shall determine the affected payroll result and adjustment impact. | L3-E-054 §9 |
| L4-E-054-BH-003 | The system shall obtain the required authorized adjustment decision before applying the correction. | L3-E-054 §§6,9 |
| L4-E-054-BH-004 | When the adjustment is approved, the system shall apply the controlled correction or recalculation without editing the finalized payroll record. | L3-E-054 §§3,4.2,9 |
| L4-E-054-BH-005 | The system shall validate the corrected payroll outcome before establishing it as authoritative within the adjustment workflow. | L3-E-054 §9 |
| L4-E-054-BH-006 | The system shall preserve the corrected payroll result and adjustment history. | L3-E-054 §§3,9,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-054-BH-007 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Payroll ownership. | L3-E-054 §14 |

## 6. Failure / Rejection / Continuation

- When the adjustment basis or required authority is unavailable, the system shall not establish an approved adjustment outcome.
- When approval is rejected, the system shall not apply the proposed adjustment.
- The finalized payroll record shall remain unchanged by the adjustment behavior.
- A source-fact correction remains the responsibility of its canonical upstream owner.

## 7. Audit & Evidence Behavior

The system shall preserve the adjustment reason, affected payroll result, authority/approval decision, correction outcome and adjustment history as attributable business evidence.

## 8. Observable Completion Conditions

- Adjustment case and business reason are established. - Required authority and approval are recorded. - Controlled correction is applied without editing finalized payroll. - Corrected outcome is validated and recorded. - Adjustment history is preserved and outcome is made available to authorized consumers.

## 9. Traceability & Provenance

**Parent:** L3-E-054 — Payroll Adjustment to Corrected Payroll Outcome  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
