# Government Connector to Governed Active or Retired State

## 1. Specification Metadata
**Specification ID:** L4-H-009  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** APAAR & DigiLocker  
**Realizes Scenario:** L3-H-009  
**Primary Business Owner:** APAAR & DigiLocker  
**Version:** 1.1.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-009; L2-H01 Connector Retirement Amendment; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

## 2. Behavioral Objective & Boundary
Support H01 connector activation, disablement and terminal retirement with exact lifecycle and authority semantics.

## 3. Required Business Context
Connector identity/configuration are H01-owned; validation precedes activation; Platform Administrator owns activation/disablement/retirement authority.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior |
|---|---|
| L4-H-009-BH-001 | Permit activation only after source-defined validation and publish `ConnectorActivated`. |
| L4-H-009-BH-002 | When normal use is withdrawn non-terminally, transition to `Disabled` and publish `ConnectorDisabled`. |
| L4-H-009-BH-003 | Permit terminal retirement only from Disabled under Platform Administrator authority. |
| L4-H-009-BH-004 | Transition `Disabled → Retired` and publish `ConnectorRetired`. |
| L4-H-009-BH-005 | Reject reactivation or configuration use of a Retired Connector. |

## 5. Cross-Domain & Event Behavior
H01 is the sole publisher of Connector lifecycle events; Section A provides authorization/audit capability only.

## 6. Failure / Rejection / Continuation
Failed validation prevents activation; retirement cannot bypass disablement.

## 7. Audit & Evidence Behavior
Preserve connector identity, configuration, validation, acting authority, disablement and retirement evidence.

## 8. Observable Completion Conditions
Connector is either validly active/operational or terminal Retired with the corresponding canonical event evidence.

## 9. Traceability & Provenance
Parent: L3-H-009; governing H01 amendment dated 2026-09-01.

## 10. Open Issues / Deferred Design
None at L4.
