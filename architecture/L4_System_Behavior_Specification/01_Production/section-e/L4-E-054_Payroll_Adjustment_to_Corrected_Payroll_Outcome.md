# Payroll Adjustment to Corrected Payroll Outcome

## 1. Specification Metadata

**Specification ID:** L4-E-054  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-054 — Payroll Adjustment to Corrected Payroll Outcome  
**Primary Business Owner:** Payroll  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-054; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall process a governed payroll correction through the Payroll Adjustment workflow, establish a corrected payroll outcome through controlled recalculation and validation, and preserve the original finalized payroll record as immutable. Source-fact correction, payment execution and accounting remain with their canonical owners.

## 3. Required Business Context

- A payroll discrepancy or correction requirement has been identified.
- The business reason and basis for adjustment are available.
- Applicable authoritative source facts and payroll rules are available.
- Required payroll approval authority is available.
- The existing finalized payroll record remains immutable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-054-BH-001 | The system shall establish the Payroll Adjustment case and record the business reason for the correction. | L3-E-054 §§3, 7–9 |
| L4-E-054-BH-002 | The system shall identify the affected payroll result and determine the adjustment scope and business impact. | L3-E-054 §9 |
| L4-E-054-BH-003 | The system shall obtain and record the required authorized payroll-adjustment decision before applying the correction. | L3-E-054 §§6, 9, 12, 15 |
| L4-E-054-BH-004 | When approved, the system shall perform the controlled correction/recalculation without editing the existing finalized payroll record. | L3-E-054 §§3, 4.2, 9, 12–13 |
| L4-E-054-BH-005 | The system shall validate the corrected payroll result before establishing it as the authoritative adjustment outcome. | L3-E-054 §§9, 11, 16 |
| L4-E-054-BH-006 | The system shall preserve the corrected payroll outcome, affected payroll reference, calculation references, applicable rule/version evidence and complete adjustment history. | L3-E-054 §§12, 15–16 |
| L4-E-054-BH-007 | If the correction depends on an incorrect upstream business fact, the system shall consume the corrected fact only after the canonical source domain has corrected it; Payroll shall not take ownership of that source fact. | L3-E-054 §§5, 10–12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-054-BH-008 | When the adjustment case is established, the system shall publish **PayrollAdjustmentCreated** under Payroll ownership. | L3-E-054 §14.1 |
| L4-E-054-BH-009 | When the corrected payroll calculation completes, the system shall publish **PayrollCalculationCompleted** under Payroll ownership. | L3-E-054 §14.1 |
| L4-E-054-BH-010 | When the corrected payroll outcome passes validation, the system shall publish **PayrollValidationCompleted** under Payroll ownership. | L3-E-054 §14.1 |
| L4-E-054-BH-011 | Where the adjustment produces a new finalized payroll outcome, the system shall publish **PayrollFinalized** for that resulting immutable payroll record. | L3-E-054 §14.1 |
| L4-E-054-BH-012 | The corrected payroll outcome may be made available to Finance and other authorized downstream consumers, while Finance remains owner of payment and accounting consequences. | L3-E-054 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- If adjustment authority is insufficient, the system shall not apply the correction and shall preserve the original finalized payroll.
- If the adjustment is rejected, the system shall record the rejection and shall not establish a corrected payroll outcome.
- If an upstream source fact is incorrect, its canonical owner shall correct that fact before Payroll recalculates from the corrected authoritative input.
- If the corrected calculation remains invalid, the adjustment shall remain within the governed correction workflow until the discrepancy is resolved and validation succeeds.
- The original finalized payroll record shall never be edited by this behavior.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of the adjustment reason, authority, approval decision, affected payroll result, source-fact references where applicable, calculation references, rule versions, validation outcome, corrected payroll result and all relevant Payroll events.

## 8. Observable Completion Conditions

Completion requires the adjustment case and business reason to be recorded, required approval to be established, controlled recalculation/correction to complete, the corrected outcome to pass validation, the original finalized payroll to remain immutable, and the applicable canonical Payroll adjustment/result events to be published.

## 9. Traceability & Provenance

**Parent:** L3-E-054 — Payroll Adjustment to Corrected Payroll Outcome  
**Primary verification source:** Authoritative People & Workforce L2-E06 Payroll workflow architecture.

## 10. Open Issues / Deferred Design

No L3 open issue materially affects ownership, lifecycle, events or the correction boundary. Technical APIs, persistence schemas, UI, transport and retry mechanics are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
