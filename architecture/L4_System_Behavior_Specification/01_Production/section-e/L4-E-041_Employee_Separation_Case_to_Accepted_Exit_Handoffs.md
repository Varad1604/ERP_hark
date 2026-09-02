# Employee Separation Case to Accepted Exit Handoffs

## 1. Specification Metadata

**Specification ID:** L4-E-041  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-041 — Employee Separation Case to Accepted Exit Handoffs  
**Primary Business Owner:** HR  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-041; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an employee separation through the HR separation case, including exit approval, clearance and final verification, until the accepted cleared separation is handed to Workforce & Engagement for authoritative employment closure. HR shall not claim or duplicate the resulting employment-closure fact.

## 3. Required Business Context

- The employee/workforce engagement is identifiable from authoritative workforce context.
- A resignation or separation request exists.
- Required exit approval and clearance responsibilities can be determined.
- Workforce & Engagement remains authoritative for employment closure and its resulting employment event.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-041-BH-001 | The system shall establish the separation case from the resignation or separation request under HR ownership. | L3-E-041 §§2, 5, 9 |
| L4-E-041-BH-002 | The system shall obtain and record the applicable exit approval decision with attributable decision evidence. | L3-E-041 §§6, 9, 12, 15 |
| L4-E-041-BH-003 | The system shall complete and record the required business clearance outcomes before final verification. | L3-E-041 §§9, 10, 11, 16 |
| L4-E-041-BH-004 | The system shall perform final verification only after the applicable exit approval and required clearance outcomes are complete. | L3-E-041 §§9, 10, 16 |
| L4-E-041-BH-005 | The system shall hand the cleared separation from HR to Workforce & Engagement for authoritative employment closure without transferring employment-lifecycle ownership to HR. | L3-E-041 §§3, 5, 9, 14.2 |
| L4-E-041-BH-006 | The system shall record the HR separation-case outcome without duplicating the authoritative employment-closure fact. | L3-E-041 §§9, 12, 16 |
| L4-E-041-BH-007 | The system shall preserve exit approval, clearance, final verification, handoff status and HR separation-case history. | L3-E-041 §§12, 15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-041-BH-008 | The system shall treat **EmployeeSeparated** as the authoritative employment-separation event published by Workforce & Engagement following its successful employment-closure transition; HR shall not publish or represent that event as HR-owned. | L3-E-041 §§12, 14.1, 15 |
| L4-E-041-BH-009 | If the locked HR workflow publishes **HRSeparationCaseClosed** for HR case closure, the system shall keep that HR case event distinct from **EmployeeSeparated** and shall not use it as a substitute for the authoritative employment event. | L3-E-041 §14.1 |
| L4-E-041-BH-010 | The material synchronous business handoff shall be the accepted, cleared separation from HR to Workforce & Engagement for closure of the Workforce Engagement; the handoff shall not transfer ownership. | L3-E-041 §14.2 |

## 6. Failure / Rejection / Continuation

- If exit approval is rejected, the system shall record the decision and shall not establish an accepted separation handoff.
- If required clearance is incomplete, the system shall keep the separation case unresolved for final verification until the applicable business clearance outcome is established.
- If Workforce & Engagement has not yet completed authoritative employment closure, the system shall preserve the accepted handoff and shall not claim the employment closure or **EmployeeSeparated** outcome as complete.
- Technical retries, queues, rollback and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of separation initiation, exit approval, clearance outcomes, final verification, accepted HR-to-E01 handoff, HR case outcome and the distinction between HR case closure and Workforce & Engagement-owned employment closure.

## 8. Observable Completion Conditions

Completion requires the exit approval decision to be recorded, required clearance and final verification to be complete, the accepted separation to be handed to Workforce & Engagement, and the HR separation-case outcome to be recorded without duplicating authoritative employment closure. The authoritative **EmployeeSeparated** event remains dependent on the successful E01 employment-closure transition.

## 9. Traceability & Provenance

**Parent:** L3-E-041 — Employee Separation Case to Accepted Exit Handoffs  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval and clearance roles remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
