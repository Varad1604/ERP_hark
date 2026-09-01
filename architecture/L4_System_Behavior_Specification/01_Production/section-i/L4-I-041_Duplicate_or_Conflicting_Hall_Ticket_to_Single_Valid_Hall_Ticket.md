# L4-I-041 — Duplicate or Conflicting Hall Ticket to Single Valid Hall Ticket

**Architecture Level:** L4 — System Behavior Specification  
**Parent Scenario:** `L3-I-041` — Duplicate or Conflicting Hall Ticket to Single Valid Hall Ticket  
**Canonical Owner:** Hall Ticket Management  
**Governing L2:** `L2-I05` — Hall Ticket Management Workflows  
**Status:** Draft for governed L4 authoring

## 1. Purpose
Define implementation-independent behaviour for resolving duplicate or conflicting Hall Ticket positions so exactly one operational Hall Ticket remains valid for the applicable candidate and Examination Session while historical traceability is preserved.

## 2. Required Behaviour
The system shall identify the candidate, registration and session; identify all relevant Hall Ticket positions; distinguish current authoritative and historical/superseded positions; resolve genuine duplication under L2-I05 authority; retain one operational authorization; apply existing withdrawal, replacement or cancellation transitions where applicable; preserve relationships and history; and publish only canonical lifecycle events associated with actual transitions.

## 3. Ownership Constraints
Hall Ticket Management owns Hall Ticket validity and resolution. Examination Registration remains authoritative for registration context. Student Lifecycle supplies candidate context where required. No participating or downstream workflow acquires Hall Ticket ownership.

## 4. Resolution Behaviour
| Situation | Required Behaviour |
|---|---|
| Current valid Hall Ticket with historical predecessors | Retain current operational position. |
| Multiple apparently valid Hall Tickets | Resolve under L2-I05 authority before treating the context as valid. |
| Permitted replacement | Replacement is operationally authoritative; replaced version is non-operational. |
| Permitted reissue | Current reissued position is operational; prior position remains historical. |
| Withdrawn/cancelled Hall Ticket | It cannot authorize examination entry. |
| Conflict cannot be safely resolved | Preserve unresolved business condition; do not select arbitrarily. |

## 5. Lifecycle and Event Constraints
Use only existing Hall Ticket states and canonical events. Do not introduce `Duplicate`, `ConflictResolved`, `Canonical`, or similar states/events.

## 6. Cross-Domain Behaviour
Consume authoritative registration, candidate, session and planning context from owning domains. Cross-domain interaction remains technology-neutral.

## 7. Audit Behaviour
Preserve all positions considered, authoritative context, conflict basis, decision authority, lifecycle transition, resulting operational Hall Ticket, and excluded historical versions.

## 8. Recovery Behaviour
Conflicts arising after publication must be re-evaluated under L2-I05. Upstream registration withdrawal or other authoritative change applies through its canonical lifecycle consequences.

## 9. Validation Criteria
Conformance requires one operational authorization, historical traceability, canonical ownership, existing lifecycle usage, auditable resolution, and no implementation-specific technology.

## 10. Traceability
`L3-I-041` → `L2-I05`; registration context → `L2-I04`; session context → `L2-I02`; planning context → `L2-I03`.