# Employee Grievance to Investigated Resolution

## 1. Specification Metadata

**Specification ID:** L4-E-036  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-036 — Employee Grievance to Investigated Resolution  
**Primary Business Owner:** HR  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-036; L2-E04; applicable Employee Profile and organisational context  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an employee grievance from formal receipt through HR investigation to an authoritative recorded resolution and Closed HR case, while preserving case history and evidence and without duplicating authoritative state owned by another domain.

## 3. Required Business Context

- A grievance has been formally received.
- The grievance falls within HR case-management scope.
- The grievance can be classified and assigned for investigation.
- HR owns the grievance case, investigation, resolution tracking and closure.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-036-BH-001 | On formal receipt of an employee grievance within HR case-management scope, the system shall create and classify the grievance as an HR case. | L3-E-036 §§4.1, 7, 8, 9 |
| L4-E-036-BH-002 | The system shall assign the grievance case to the responsible HR case owner before the governed investigation proceeds. | L3-E-036 §§7, 9, 12, 13 |
| L4-E-036-BH-003 | The system shall investigate the grievance and collect the business evidence required to establish a governed resolution. | L3-E-036 §§9, 12, 16.1 |
| L4-E-036-BH-004 | If investigation has not established sufficient business facts, the system shall continue the governed investigation and shall not infer a final resolution. | L3-E-036 §11 |
| L4-E-036-BH-005 | The system shall determine and record the grievance resolution under HR case governance once sufficient evidence exists or another authorised case outcome is established. | L3-E-036 §§9, 11, 12, 15 |
| L4-E-036-BH-006 | If the grievance is not substantiated or the requested outcome cannot be granted, the system shall record the applicable resolution rather than represent the requested outcome as established. | L3-E-036 §11 |
| L4-E-036-BH-007 | If resolution requires a decision or state owned by another canonical domain, the system shall consume that authoritative outcome without duplicating or transferring ownership of the external state. | L3-E-036 §§5, 14.2, 16.2 |
| L4-E-036-BH-008 | The system shall close the grievance case only after the governed resolution has been recorded. | L3-E-036 §§4.2, 9, 13, 16.1 |
| L4-E-036-BH-009 | The system shall preserve grievance history, collected business evidence and attributable decision evidence after case closure. | L3-E-036 §§12, 15, 16.1–16.2 |

## 5. Cross-Domain & Event Behavior

No scenario-specific grievance event name is established by the Locked parent; this specification shall not invent one. Grievance investigation and resolution remain within HR case ownership. Where another domain must provide an authoritative decision or state, HR shall consume that outcome for case resolution without copying ownership of the external state.

## 6. Failure / Rejection / Continuation

- If investigation has not established the necessary business facts, the system shall continue the governed investigation until sufficient evidence exists or another authorised case outcome is established.
- If the grievance is not substantiated or cannot be granted, the system shall record the applicable resolution and close the case according to HR case governance.
- The system shall not infer another domain's decision or authoritative state to force grievance completion.
- Technical retry, queue, rollback, database recovery and infrastructure mechanisms are excluded.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of grievance intake, classification, case assignment, investigation activity, collected business evidence, recorded resolution, applicable responsible business actor/decision evidence, case closure and retained grievance history.

## 8. Observable Completion Conditions

Completion requires the grievance to be formally classified and assigned, investigation to be completed to the extent required for a governed resolution, a resolution to be recorded, the HR case to be Closed, and grievance history and evidence to remain traceable. Any externally owned decision or state remains authoritative in its canonical owning domain.

## 9. Traceability & Provenance

**Parent:** L3-E-036 — Employee Grievance to Investigated Resolution  
**Primary verification source:** L2-E04 — HR Workflows, with applicable Employee Profile and organisational context remaining source-owned.

## 10. Open Issues / Deferred Design

No scenario-specific grievance completion event is established upstream. Institution-specific grievance governance and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, technical access control, network transport and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
