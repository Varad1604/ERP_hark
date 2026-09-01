# L4-I-041 — Duplicate or Conflicting Hall Ticket to Single Valid Hall Ticket

**Architecture Level:** L4 — Implementation Architecture  
**Parent Scenario:** `L3-I-041` — Duplicate or Conflicting Hall Ticket to Single Valid Hall Ticket  
**Canonical Owner:** Hall Ticket Management  
**Governing L2:** `L2-I05` — Hall Ticket Management Workflows  
**Status:** Draft for governed L4 authoring

## 1. Purpose

Define implementation-independent behaviour for resolving duplicate or conflicting Hall Ticket positions so that exactly one operational Hall Ticket remains valid for the applicable candidate and Examination Session, while preserving historical traceability.

## 2. Required Behaviour

The system shall:

1. identify the candidate, Examination Registration and Examination Session;
2. identify all Hall Ticket positions relevant to that context;
3. distinguish the current authoritative operational Hall Ticket from superseded, withdrawn, cancelled or otherwise non-authorising positions;
4. resolve an unresolved duplicate/conflict under the authority and lifecycle rules of L2-I05;
5. ensure only one Hall Ticket represents operational examination-entry authorization for the applicable context;
6. preserve historical Hall Ticket versions and their business relationships;
7. publish only canonical Hall Ticket lifecycle events associated with actual governed transitions.

## 3. Ownership

Hall Ticket Management is the canonical owner of Hall Ticket validity and conflict resolution. Examination Registration remains authoritative for registration context. No downstream examination workflow becomes owner of Hall Tickets by consuming the resolution.

## 4. Resolution Rules

| Situation | Required Behaviour |
|---|---|
| One valid current Hall Ticket and historical superseded versions | Retain the valid current Hall Ticket; historical versions remain non-operational. |
| Multiple apparently valid Hall Tickets | Do not expose competing operational authorization; resolve through L2-I05 authority before treating the context as valid. |
| One Hall Ticket replaced by another | Replacement is operationally authoritative; replaced position is not a second authorization. |
| One Hall Ticket reissued | Current governed reissue position is operational; prior position remains historical/non-competing. |
| Hall Ticket withdrawn/cancelled | It cannot authorize examination entry. |
| Conflict cannot be safely resolved | Do not silently choose a Hall Ticket; preserve the unresolved business condition and escalate through existing governance. |

## 5. Lifecycle Constraints

L4 shall use the existing Hall Ticket lifecycle. It must not create a generic `ConflictResolved`, `Duplicate`, or `Canonical` Hall Ticket state unless such a state is established by the governing L2 architecture.

The resolution is a business outcome supported by existing lifecycle transitions.

## 6. Cross-Domain Behaviour

Authoritative registration and candidate context are consumed from their owning domains. Hall Ticket Management resolves the Hall Ticket-specific consequence. Cross-domain interaction remains technology-neutral.

No new API, message schema, event, endpoint, or persistence design is defined.

## 7. Audit Behaviour

The implementation architecture shall retain evidence of:

- all Hall Ticket positions considered;
- the authoritative registration/session context;
- the conflict or duplication identified;
- the governing decision and authority;
- the lifecycle transition used to resolve it;
- the resulting single operational Hall Ticket;
- historical versions excluded from operational authorization.

## 8. Recovery and Exceptions

If a conflict arises after publication, the current Hall Ticket validity must be re-evaluated under L2-I05. If an upstream registration withdrawal or other authoritative change affects the context, the corresponding canonical lifecycle consequence applies.

No technical retry or storage-recovery behaviour is prescribed.

## 9. Validation Criteria

Conformance requires that:

1. competing operational Hall Ticket authorization is prevented;
2. exactly one valid operational Hall Ticket exists where the business context requires one;
3. historical versions remain traceable;
4. Hall Ticket Management remains canonical owner;
5. no new lifecycle state/event is invented;
6. resolution authority is auditable;
7. implementation technology remains unspecified.

## 10. Traceability

`L3-I-041` → `L2-I05` Hall Ticket Management; upstream registration context → `L2-I04`; session context → `L2-I02`; applicable planning context → `L2-I03`.
