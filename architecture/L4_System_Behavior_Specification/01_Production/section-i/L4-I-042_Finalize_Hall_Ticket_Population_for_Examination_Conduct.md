# L4-I-042 — Finalize Hall Ticket Population for Examination Conduct

**Architecture Level:** L4 — Implementation Architecture  
**Parent Scenario:** `L3-I-042` — Finalize Hall Ticket Population for Examination Conduct  
**Canonical Owner:** Hall Ticket Management  
**Governing L2:** `L2-I05` — Hall Ticket Management Workflows  
**Downstream Consumers:** `L2-I06` — Practical & Viva Examination Conduct; `L2-I07` — Theory Examination Conduct  
**Status:** Draft for governed L4 authoring

## 1. Purpose

Define implementation-independent behaviour for establishing the authoritative population of candidates holding valid Hall Tickets for examination conduct for an applicable Examination Session.

## 2. Required Behaviour

The system shall:

1. establish the applicable Examination Session and Published Examination Schedule;
2. identify Hall Tickets relevant to the examination population;
3. evaluate current Hall Ticket validity against the L2-I05 lifecycle;
4. exclude withdrawn, cancelled, superseded or otherwise non-authorising Hall Tickets;
5. resolve remaining duplicate or conflicting operational positions through the governed I041 behaviour;
6. establish one valid operational Hall Ticket per applicable candidate/session context;
7. preserve the final business decision and exclusions as auditable evidence;
8. make the resulting valid population available as a business hand-off to examination conduct workflows.

## 3. Ownership Boundary

Hall Ticket Management owns Hall Ticket validity and the final Hall Ticket population. Examination Registration remains the canonical owner of registration authority. Examination Session Management and Planning remain owners of session and schedule facts. Examination conduct workflows consume the population and do not redefine Hall Ticket validity.

## 4. Inclusion and Exclusion Behaviour

| Hall Ticket Position | Population Behaviour |
|---|---|
| Valid current Hall Ticket | Include. |
| Withdrawn | Exclude. |
| Cancelled | Exclude. |
| Superseded by valid replacement | Exclude superseded position; include valid replacement. |
| Historical/reissued predecessor | Do not treat as competing operational authorization. |
| Validity unresolved | Exclude pending authoritative resolution. |
| Duplicate/conflicting positions | Resolve under L3-I-041/L2-I05 before inclusion. |

The population is an outcome, not a new Hall Ticket lifecycle state.

## 5. Finalization Behaviour

At the governed examination-conduct boundary, the architecture shall support a definitive business determination of which Hall Tickets authorize participation. The determination must be based on current authoritative lifecycle positions rather than merely on existence of a Hall Ticket record.

Where the examination schedule or upstream registration position changes before finalization, the affected Hall Ticket population must be re-evaluated using the authoritative business consequences.

## 6. Cross-Domain Hand-Off

The final population consumes authoritative facts from Examination Registration, Examination Session Management, Examination Planning & Scheduling and Student Lifecycle where applicable. The resulting population is handed to examination conduct as a business capability/output.

No API, database query, batch implementation, queue, message schema, endpoint or transport mechanism is prescribed.

## 7. Event Constraints

L2-I05 remains the canonical publisher of Hall Ticket lifecycle events. Population finalization must not invent a `HallTicketPopulationFinalized` event. Existing lifecycle events are published only when an actual governed Hall Ticket transition occurs.

## 8. Audit Behaviour

The architecture shall preserve evidence sufficient to establish:

- the Examination Session and schedule considered;
- the Hall Tickets evaluated;
- the inclusion/exclusion decision for each relevant position;
- duplicate/conflict resolution where applicable;
- the resulting authoritative valid population;
- the authority and business basis for exceptional decisions.

## 9. Exceptional Behaviour

If no valid Hall Tickets exist, the authoritative population is empty. If an authorized late Hall Ticket reaches a valid lifecycle position before finalization, it may be included according to L2-I05. If a registration is withdrawn before finalization, the resulting Hall Ticket consequence is applied and the non-authorising Hall Ticket is excluded.

No new exception authority is created.

## 10. Downstream Consumption Constraint

L2-I06, L2-I07 and subsequent examination conduct workflows may consume the final valid population but must not independently redefine Hall Ticket validity or admit candidates based on superseded, withdrawn or cancelled Hall Tickets.

## 11. Validation Criteria

Conformance requires that:

1. the final population contains only valid examination-entry authorizations;
2. non-authorising Hall Tickets are excluded;
3. duplicate/conflicting operational authorizations are resolved before final inclusion;
4. each included candidate has one valid operational Hall Ticket for the applicable session;
5. Hall Ticket Management remains canonical owner;
6. no new lifecycle state or population-finalization event is invented;
7. exclusions and finalization are auditable;
8. downstream conduct consumes rather than redefines the authorization boundary;
9. the design remains implementation-independent.

## 12. Traceability

`L3-I-042` → `L2-I05`; conflict resolution → `L3-I-041`; registration authority → `L2-I04`; session → `L2-I02`; schedule → `L2-I03`; downstream consumption → `L2-I06` / `L2-I07`.
