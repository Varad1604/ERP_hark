# L4-G-143 — Reading Room Opening to Operational Availability

## 1. Specification Metadata
**Specification ID:** L4-G-143  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-143 — Reading Room Opening to Operational Availability  
**Primary Business Owner:** Library Management  
**Participating Domain:** People & Workforce (E) where authoritative personnel context is required  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern establishment of a configured Reading Room as operationally available for its authorised operating period.

**Start:** A configured Reading Room reaches the authorised business condition to commence operation.  
**End:** Reading Room Available is established and the room is ready for Operational Monitoring.

The behavior shall not extend into monitoring, usage supervision, reading sessions, seat reservation, physical security, or technical access control.

## 3. Required Business Context
- Reading Room exists as a configured institutional Reading Room.
- Category and authorised operating period are established.
- Applicable Reading Room rules are available.
- Required Library Management authority is available.
- Required personnel context is available from E where applicable.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-143-BH-001 | Identify the applicable configured Reading Room. | L3-G-143 §§2, 7, 9 |
| L4-G-143-BH-002 | Establish the configured Reading Room category and authorised operating period. | L3-G-143 §§3, 7, 12 |
| L4-G-143-BH-003 | Establish the applicable Reading Room operating rules without inventing local defaults. | L3-G-143 §§3, 12 |
| L4-G-143-BH-004 | Validate that no authoritative operational restriction prevents availability. | L3-G-143 §§9, 10, 11 |
| L4-G-143-BH-005 | Where required, obtain authoritative personnel context from E without transferring Reading Room ownership. | L3-G-143 §§5, 10 |
| L4-G-143-BH-006 | When availability conditions are satisfied, establish Reading Room Available. | L3-G-143 §§3, 9, 13, 16 |
| L4-G-143-BH-007 | Preserve the availability outcome and applicable operating context in Library Management history. | L3-G-143 §§3, 15, 16 |
| L4-G-143-BH-008 | Make the Reading Room ready for the downstream Operational Monitoring stage without executing that stage. | L3-G-143 §§9, 13, 16 |

## 5. Cross-Domain & Event Behavior

The locked L3 states that no dedicated canonical Reading Room opening/availability event is exposed by the retrieved L2. Therefore this specification does not invent an event name. E may provide authoritative personnel context where required; G remains the canonical owner of Reading Room availability.

## 6. Failure / Rejection / Continuation
- Outside the configured operating period, the system shall not establish Reading Room Available.
- If required operating hours or rules cannot be established, the system shall not falsely assert availability.
- If an authoritative operational restriction exists, the system shall retain the restriction and prevent contradictory availability.
- If mandatory personnel context is unavailable, the system shall not substitute an invented authority or person.
- If the room is already authoritatively available, the system shall not create a duplicate opening outcome.

## 7. Audit & Evidence Behavior
Preserve attributable evidence of Reading Room identity, category, operating period, applicable rules, availability decision, relevant personnel context, and operational history. Lifecycle transition evidence shall remain auditable.

## 8. Observable Completion Conditions
Completion occurs when:

- the correct Reading Room is identified;
- category and operating period are established;
- applicable rules are established;
- no authoritative restriction prevents availability;
- Reading Room Available is established;
- availability history is preserved; and
- the room is ready for Operational Monitoring.

No seat reservation or individual reading session is created.

## 9. Traceability & Provenance
**Parent:** L3-G-143 — Reading Room Opening to Operational Availability  
**Primary ownership:** Library Management (G)  
**Participating context:** People & Workforce (E)

## 10. Open Issues / Deferred Design
The canonical event register should be consulted if a dedicated Reading Room availability event is required. Technical scheduling, authentication, access control, persistence, interfaces, and infrastructure remain outside L4.