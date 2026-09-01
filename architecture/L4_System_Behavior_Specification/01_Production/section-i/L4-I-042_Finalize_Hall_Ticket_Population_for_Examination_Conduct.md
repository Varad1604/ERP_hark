# L4-I-042 — Finalize Hall Ticket Population for Examination Conduct

**Architecture Level:** L4 — System Behavior Specification  
**Parent Scenario:** `L3-I-042` — Finalize Hall Ticket Population for Examination Conduct  
**Canonical Owner:** Hall Ticket Management  
**Governing L2:** `L2-I05` — Hall Ticket Management Workflows  
**Downstream Consumers:** `L2-I06` — Practical & Viva Examination Conduct; `L2-I07` — Theory Examination Conduct  
**Status:** Draft for governed L4 authoring

## 1. Purpose
Define implementation-independent behaviour for establishing the authoritative population of candidates holding valid Hall Tickets for examination conduct for the applicable Examination Session.

## 2. Required Behaviour
The system shall establish the Examination Session and Published Examination Schedule, identify relevant Hall Tickets, evaluate current Hall Ticket validity under L2-I05, exclude withdrawn/cancelled/superseded/non-authorising positions, resolve duplicate/conflicting positions through the governed I041 behaviour, establish one valid operational Hall Ticket per applicable candidate/session context, preserve the final decision and exclusions as auditable evidence, and make the valid population available to examination conduct as a business hand-off.

## 3. Ownership Boundary
Hall Ticket Management owns Hall Ticket validity and the final population. Examination Registration remains owner of registration authority. Examination Session Management and Planning retain their respective authorities. Examination conduct consumes the population and does not redefine validity.

## 4. Inclusion and Exclusion Behaviour
| Hall Ticket Position | Population Behaviour |
|---|---|
| Valid current Hall Ticket | Include. |
| Withdrawn | Exclude. |
| Cancelled | Exclude. |
| Superseded by valid replacement | Exclude superseded position; include replacement. |
| Historical/reissued predecessor | Do not treat as competing operational authorization. |
| Validity unresolved | Exclude pending authoritative resolution. |
| Duplicate/conflicting positions | Resolve under L3-I-041/L2-I05 before inclusion. |

The final population is an outcome, not a new Hall Ticket lifecycle state.

## 5. Finalization Behaviour
At the governed examination-conduct boundary, determine which current Hall Tickets authorize participation. If schedule or registration authority changes before finalization, re-evaluate affected positions against authoritative business consequences.

## 6. Cross-Domain Hand-Off
Consume authoritative Examination Registration, Examination Session, Examination Planning & Scheduling and Student Lifecycle facts where applicable. Hand the resulting population to downstream examination conduct without prescribing API, database, batch, queue, message, or transport mechanisms.

## 7. Event Constraints
L2-I05 remains the canonical publisher of Hall Ticket lifecycle events. Do not invent `HallTicketPopulationFinalized`; existing events are published only when an actual governed Hall Ticket transition occurs.

## 8. Audit Behaviour
Preserve the session/schedule considered, Hall Tickets evaluated, inclusion/exclusion decisions, conflict resolution, final population, and authority/business basis for exceptional decisions.

## 9. Exceptional Behaviour
If no valid Hall Tickets exist, the final population is empty. An authorised late Hall Ticket reaching a valid lifecycle position before finalization may be included. A registration withdrawal before finalization causes the related non-authorising Hall Ticket to be excluded through the existing lifecycle consequence.

## 10. Downstream Consumption Constraint
L2-I06 and L2-I07 may consume the final valid population but must not independently redefine Hall Ticket validity or admit candidates through superseded, withdrawn, or cancelled positions.

## 11. Validation Criteria
Conformance requires only valid authorization in the final population, exclusion of non-authorising positions, conflict resolution before inclusion, one operational Hall Ticket per applicable candidate/session context, canonical ownership, canonical events, auditability, and implementation independence.

## 12. Traceability
`L3-I-042` → `L2-I05`; conflict resolution → `L3-I-041`; registration → `L2-I04`; session → `L2-I02`; schedule → `L2-I03`; downstream consumption → `L2-I06` / `L2-I07`.