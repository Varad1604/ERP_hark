# Annual Health Checkup to Recorded Assessment Outcome

## 1. Specification Metadata

**Specification ID:** L4-G-177  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-177 — Annual Health Checkup to Recorded Assessment Outcome  
**Primary Business Owner:** Institutional Health Services  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Approved L3-G-177; locked L2-G06 Annual Health Checkup lifecycle/event definitions  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

The system shall support establishment, conduct and completion of an Annual Health Checkup while recording assessment findings and recommendations and keeping resulting healthcare activities separately governed.

## 3. Required Business Context

- Annual Health Checkup lifecycle: `Scheduled → Conducted → Completed`.
- G06 owns the checkup and assessment outcome.
- Participant context is authoritative from C/E as applicable.
- Healthcare professionals retain clinical authority.
- Follow-up/referral/observation/prescription remain separate G06 workflows.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-177-BH-001 | When an annual assessment requirement is established, the system shall create the Annual Health Checkup with authoritative participant and responsible-professional context in `Scheduled`. | L3-G-177 §§8,9 |
| L4-G-177-BH-002 | When the assessment activity has been conducted under authorised healthcare responsibility, the system shall record the relevant findings and transition the checkup to `Conducted`. | L3-G-177 §§9,13 |
| L4-G-177-BH-003 | The system shall preserve recommendations and establish separately governed follow-up healthcare activities when findings require further action. | L3-G-177 §§9,12 |
| L4-G-177-BH-004 | The system shall prevent `Completed` until the authoritative checkup completion condition and required assessment outcome are satisfied. | L3-G-177 §§11,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-177-BH-005 | On authoritative completion, the system shall publish `HealthCheckCompleted` with G06 as canonical publisher. | L3-G-177 §14 |
| L4-G-177-BH-006 | The system shall consume authoritative participant context from C/E where required without transferring ownership. | L3-G-177 §§5,14 |

## 6. Failure / Rejection / Continuation

- If participant context is unavailable, the system shall not record an unsupported assessment outcome.
- If assessment is interrupted, the system shall retain the valid business position.
- If findings are incomplete, the system shall not produce a final unsupported outcome.
- If urgent or further care is required, the system shall route the need to the applicable G06 healthcare workflow.
- If completion criteria are unmet, the system shall not transition to `Completed`.

## 7. Audit & Evidence Behavior

The system shall preserve checkup identity, participant context, responsible professional, assessment activity, findings, recommendations, lifecycle transitions, completion outcome and `HealthCheckCompleted` evidence.

## 8. Observable Completion Conditions

Completion occurs when the Annual Health Checkup reaches `Completed`, its required assessment outcome is recorded, and `HealthCheckCompleted` is published by G06. Individual follow-up may remain separately active.

## 9. Traceability & Provenance

**Parent:** L3-G-177 — Annual Health Checkup to Recorded Assessment Outcome  
**Primary verification source:** L2-G06 — Annual Health Checkup lifecycle and event catalogue  

## 10. Open Issues / Deferred Design

Clinical assessment criteria and regulatory reporting are not invented. Technical scheduling, APIs, persistence, notification delivery and infrastructure are deferred.