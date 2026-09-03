# Reading Room Operational Usage to Session Closure

## 1. Specification Metadata

**Specification ID:** L4-G-145  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-145 — Reading Room Operational Usage to Session Closure  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-145; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern permitted Reading Room usage under configured rules and supervision to an individual session closure outcome. Ending an individual session shall not automatically transition the Reading Room to **Operational Closure**.

## 3. Required Business Context

- Reading Room is operationally available.
- Usage is permitted under authoritative Reading Room rules.
- Applicable operating period and monitoring/supervision context are established.
- E personnel context and H communication participation are used only where required.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-145-BH-001 | Identify the Reading Room, confirm operational availability and establish the permitted usage context and applicable rules. | L3-G-145 §§7,9,12 |
| L4-G-145-BH-002 | Maintain applicable operational monitoring and usage supervision while the permitted reading activity is underway. | L3-G-145 §§2–3,9,12 |
| L4-G-145-BH-003 | Establish the authoritative individual session closure condition and record the session closure outcome. | L3-G-145 §§3,8–9,16 |
| L4-G-145-BH-004 | Preserve relevant operational history and any required E personnel context or H communication hand-off without transferring G ownership. | L3-G-145 §§5,9,15–16 |
| L4-G-145-BH-005 | Keep the broader Reading Room lifecycle unchanged solely because one session ended; do not invent `Session Started`, `Session Active`, `Session Closed` or similar formal lifecycle states. | L3-G-145 §§12–13,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-145-BH-006 | The retrieved source does not expose dedicated canonical Reading Session start/closure events. This specification shall not invent `ReadingSessionStarted`, `ReadingSessionClosed` or `ReadingUsageCompleted`. G owns the usage/session outcome; E provides personnel context and H owns communication delivery where applicable. | L3-G-145 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- If session context or Reading Room availability cannot be established, no unsupported session outcome shall be recorded.
- Missing rules shall not be replaced with invented usage policy.
- An unclear closure condition keeps the session under applicable supervision until authoritative.
- A room-level closure, if separately reached, remains governed by the Reading Room closure workflow.

## 7. Audit & Evidence Behavior

Preserve Reading Room identity, operating period, applicable rules, supervision context, session closure outcome, relevant personnel context and communication outcome where required.

## 8. Observable Completion Conditions

Completion occurs when the individual usage session reaches its authoritative closure condition and its history is preserved while the broader Reading Room remains in its independently authoritative operational condition.

## 9. Traceability & Provenance

**Parent:** L3-G-145 — Reading Room Operational Usage to Session Closure  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The upstream source defines no separate Reading Session aggregate lifecycle, dedicated session events or detailed closure matrix. None are invented at L4. Technical implementation remains deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
