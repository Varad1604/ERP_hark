# Occupational Health Assessment to Medical Outcome

## 1. Specification Metadata

**Specification ID:** L4-G-179  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-179 — Occupational Health Assessment to Medical Outcome  
**Primary Business Owner:** Institutional Health Services  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-179; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall govern an occupational health assessment through a recorded G06 medical outcome while consuming authoritative workforce context from E. Employment, payroll and workforce decisions remain outside this behavior.

## 3. Required Business Context

- E is authoritative for workforce context.
- G06 owns occupational healthcare assessment and medical outcome.
- Assessment requires authorised healthcare responsibility.
- Resulting follow-up, referral, observation, prescription or certificate uses its canonical G06 workflow.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-179-BH-001 | When an occupational health assessment is due, the system shall associate it with authoritative E workforce context and establish the G06 healthcare assessment. | L3-G-179 §§7,9 |
| L4-G-179-BH-002 | The system shall record relevant medical findings and the authorised medical outcome without changing E-owned workforce information. | L3-G-179 §§9,12 |
| L4-G-179-BH-003 | When the assessment requires further healthcare action, the system shall establish or route the appropriate canonical G06 healthcare activity. | L3-G-179 §§9,10 |
| L4-G-179-BH-004 | The system shall prevent an unsupported final medical outcome when workforce context, assessment evidence or healthcare authority is insufficient. | L3-G-179 §§7,11 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-179-BH-005 | Where the resulting healthcare activity has a canonical G06 event, the system shall use that authoritative event with G06 as publisher. | L3-G-179 §14 |
| L4-G-179-BH-006 | The system shall consume authoritative workforce context from E without transferring workforce ownership. | L3-G-179 §§5,14 |

## 6. Failure / Rejection / Continuation

- If workforce identity/context cannot be established, the system shall not record the medical outcome against uncertain workforce context.
- If assessment is inconclusive, the system shall not create an unsupported final outcome.
- If further healthcare action is required, the system shall route into the appropriate G06 workflow.
- Workforce changes shall remain governed by E and shall not be locally rewritten by G06.

## 7. Audit & Evidence Behavior

The system shall preserve workforce participant reference, assessment, assessing professional, material findings, medical outcome, resulting healthcare action, applicable lifecycle position and canonical event evidence.

## 8. Observable Completion Conditions

Completion occurs when the occupational assessment has an authoritative medical outcome recorded and any required downstream G06 healthcare activity is established. No employment decision is implied.

## 9. Traceability & Provenance

**Parent:** L3-G-179 — Occupational Health Assessment to Medical Outcome  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

No dedicated occupational-health event, universal fitness classification or employment consequence is defined upstream. Any such future requirement must be owned by the appropriate upstream architecture. Technical implementation remains deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
