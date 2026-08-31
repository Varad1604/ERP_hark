# Guardian Relationship Termination

## 1. Specification Metadata
**Specification ID:** L4-C-061  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Guardian  
**Realizes Scenario:** L3-C-061 — Guardian Relationship Termination  
**Primary Business Owner:** Guardian  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-061; authoritative Guardian L2 workflow; materially applicable L1/L2 sources

## 2. Behavioral Objective & Boundary
The system shall support the authoritative termination of an existing Guardian Relationship when governed termination conditions, authority and evidence are satisfied. It shall preserve the historical relationship record and shall not treat termination as identity deletion, student lifecycle termination, suspension or reinstatement.

## 3. Required Business Context
- Existing Guardian Relationship.
- Applicable Guardian termination policy and condition.
- Required institutional authority/evidence.
- Existing Guardian relationship history.
- Distinction between termination and temporary suspension.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-061-BH-001 | The system shall identify the authoritative Guardian Relationship subject to termination. | L3-C-061 §§4,7,9 |
| L4-C-061-BH-002 | The system shall validate the applicable termination condition, authority and evidence before applying termination. | L3-C-061 §§7,9,15 |
| L4-C-061-BH-003 | The system shall record the Guardian Relationship's canonical terminal/inactive outcome when governed conditions are satisfied. | L3-C-061 §§3,9,13 |
| L4-C-061-BH-004 | The system shall preserve prior Guardian relationship history and shall not delete historical evidence. | L3-C-061 §§3,10-11 |
| L4-C-061-BH-005 | The system shall keep termination distinct from suspension and reinstatement. | L3-C-061 §§4,12 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-061-BH-006 | The system shall publish `GuardianRelationshipTerminated` from the Guardian Relationship Aggregate after successful termination. | L3-C-061 §14 |
| L4-C-061-BH-007 | The system shall make the completed termination fact available to downstream consumers without transferring Guardian ownership. | L3-C-061 §§3,9,14 |

Downstream Finance, Communication, Transport, Health and other domains remain responsible for their own consequences and shall not redefine Guardian Relationship lifecycle ownership.

## 6. Failure / Rejection / Continuation
If termination authority or conditions are insufficient, the system shall not apply termination and shall preserve the existing relationship state and history. Technical retry, queue, rollback and infrastructure behavior are outside this specification.

## 7. Audit & Evidence Behavior
The system shall preserve termination decision evidence, applicable policy basis, authority, prior relationship history, resulting relationship state and published business-event history to the extent required by the authoritative business architecture.

## 8. Observable Completion Conditions
Completion requires a valid termination decision, canonical terminal/inactive relationship outcome, cessation of active Guardian responsibilities, preserved history and publication of `GuardianRelationshipTerminated`.

## 9. Traceability & Provenance
**Parent:** L3-C-061 — Guardian Relationship Termination  
**Primary verification source:** Authoritative Guardian L2 workflow and materially applicable L1/L2 sources.

## 10. Open Issues / Deferred Design
The exact upstream terminal-state vocabulary and transition detail were not exposed in the accessible L2 extraction. No local state name is invented. Technical IAM, persistence, interfaces and implementation mechanics remain deferred to later technical design levels.