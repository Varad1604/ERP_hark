# Employee Probation Review to Confirmation Decision

## 1. Specification Metadata

**Specification ID:** L4-E-043  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-043 — Employee Probation Review to Confirmation Decision  
**Primary Business Owner:** HR  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-043; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall process the HR probation/confirmation case through review, appraisal capture and approval until a governed confirmation or non-confirmation decision is recorded. HR shall not represent that case decision as an authoritative employment-state change.

## 3. Required Business Context

- An employee/workforce engagement is identifiable from authoritative Workforce & Engagement context.
- A probation/confirmation case exists or is due for review.
- Applicable institutional review and approval requirements are available.
- HR owns the case; Workforce & Engagement owns authoritative probation/confirmation employment state.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-043-BH-001 | The system shall establish or advance the HR probation/confirmation case into the required review activity. | L3-E-043 §§4, 9 |
| L4-E-043-BH-002 | The system shall capture the applicable probation review evidence and appraisal outcome before a confirmation decision is established. | L3-E-043 §§3, 9, 15 |
| L4-E-043-BH-003 | The system shall route the case to the applicable reviewing/approving business authority and record the resulting decision with actor, timestamp and decision evidence. | L3-E-043 §§6, 9, 12, 15 |
| L4-E-043-BH-004 | The system shall record the HR case outcome as confirmation or non-confirmation without duplicating the authoritative employment fact owned by Workforce & Engagement. | L3-E-043 §§3, 5, 9, 12, 16 |
| L4-E-043-BH-005 | Where the HR decision requires an employment-state transition, the system shall hand the applicable decision to Workforce & Engagement for authoritative lifecycle handling. | L3-E-043 §§5, 11, 14.2 |
| L4-E-043-BH-006 | The system shall preserve review evidence, appraisal outcome, approval evidence and HR case history. | L3-E-043 §§12, 15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-043-BH-007 | The system shall not invent or publish an HR-owned probation/confirmation event name because the Locked parent does not establish an exact scenario-specific canonical event identifier. | L3-E-043 §§14.1, 17.1 |
| L4-E-043-BH-008 | Any authoritative event resulting from an employment-state transition shall remain published by Workforce & Engagement under its canonical employment ownership. | L3-E-043 §§5, 12, 14.1 |
| L4-E-043-BH-009 | The material synchronous handoff, where an employment-state consequence is required, shall be the HR confirmation decision to Workforce & Engagement; the handoff shall not transfer employment ownership to HR. | L3-E-043 §14.2 |

## 6. Failure / Rejection / Continuation

- If required review evidence is incomplete, the system shall keep the case within the governed HR review process and shall not establish a confirmation decision.
- If approval does not support confirmation, the system shall record the non-confirmation outcome and follow applicable upstream employment/probation governance without inventing an employment transition.
- If additional review is required, the case shall remain in the governed review process until a business decision is established.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of review initiation, appraisal/review evidence, approval authority, decision, actor, timestamp, HR case outcome and any handoff to Workforce & Engagement.

## 8. Observable Completion Conditions

Completion requires the probation review and appraisal evidence to be recorded, the required approval decision to be established, the HR case outcome to be recorded, and any required employment-state consequence to be handed to Workforce & Engagement without HR claiming the authoritative employment transition.

## 9. Traceability & Provenance

**Parent:** L3-E-043 — Employee Probation Review to Confirmation Decision  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

The Locked parent states that accessible L2-E04 evidence does not expose a dedicated canonical probation/confirmation event identifier. L4 therefore does not invent one; the exact event identifier remains an upstream architecture question before approval/lock. Technical APIs, persistence schemas, UI, transport and retry mechanics are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
