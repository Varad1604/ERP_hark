# Health Programme Planning to Programme Completion

## 1. Specification Metadata

**Specification ID:** L4-G-175  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-175 — Health Programme Planning to Programme Completion  
**Primary Business Owner:** Institutional Health Services  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Approved L3-G-175; locked L2-G06 Health Program lifecycle/event definitions  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

The system shall support establishment, activation, conduct and completion of a G06 Health Programme while preserving programme outcomes and separating individual follow-up activities from the programme lifecycle.

## 3. Required Business Context

- Health Program is G06-owned.
- Authoritative lifecycle: `Planned → Active → Completed`.
- Participant and workforce context may be consumed from C/E.
- Programme configuration governs applicable conduct/completion conditions.
- Individual follow-up, referral, observation and prescription remain separately governed.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-175-BH-001 | When an authorised programme requirement is established, the system shall establish the Health Programme with its applicable purpose, category, scope and participant context in `Planned`. | L3-G-175 §§8,9 |
| L4-G-175-BH-002 | When the authoritative activation condition is satisfied, the system shall transition the Health Programme from `Planned` to `Active`. | L3-G-175 §§9,13 |
| L4-G-175-BH-003 | During an active programme, the system shall record materially required participant activity and programme findings/outcomes within G06 ownership. | L3-G-175 §§9,16 |
| L4-G-175-BH-004 | The system shall prevent transition to `Completed` until the authoritative programme completion condition and materially required outcomes are satisfied. | L3-G-175 §§11,16 |
| L4-G-175-BH-005 | The system shall preserve individual follow-up outcomes as separate healthcare activities rather than creating a local Health Programme lifecycle state. | L3-G-175 §§10,12,13 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-175-BH-006 | For a Health Programme specifically governed as a health camp, the system shall publish the canonical `HealthCampCreated` and `HealthCampCompleted` facts at the corresponding authoritative outcomes. | L3-G-175 §14 |
| L4-G-175-BH-007 | Where the programme produces participant/workforce context requirements, the system shall consume authoritative C/E context without transferring ownership. | L3-G-175 §§5,14 |

## 6. Failure / Rejection / Continuation

- If required participant or workforce context is unavailable, the system shall not record unsupported affected outcomes.
- If programme activity is interrupted, the system shall retain the last valid lifecycle position.
- If completion criteria are not satisfied, the system shall not mark the programme `Completed`.
- If an individual finding requires follow-up, the system shall route it to the applicable canonical G06 healthcare workflow.

## 7. Audit & Evidence Behavior

The system shall preserve programme identity, purpose/type, lifecycle positions, responsible healthcare personnel, participant context, material findings/outcomes, completion decision and applicable canonical event evidence.

## 8. Observable Completion Conditions

Completion occurs when the Health Programme has satisfied its authoritative completion criteria, reached `Completed`, and its materially required outcomes are recorded. Individual follow-up may continue separately without extending the programme lifecycle.

## 9. Traceability & Provenance

**Parent:** L3-G-175 — Health Programme Planning to Programme Completion  
**Primary verification source:** L2-G06 — Health Program lifecycle and event catalogue  

## 10. Open Issues / Deferred Design

The exact canonical event for programme categories other than explicitly defined event types remains upstream-governed. Technical scheduling, persistence, APIs, notification delivery and infrastructure are deferred.