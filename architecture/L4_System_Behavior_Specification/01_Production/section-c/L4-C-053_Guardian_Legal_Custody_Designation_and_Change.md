# Guardian Legal Custody Designation and Change

## 1. Specification Metadata
**Specification ID:** L4-C-053  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Guardian  
**Realizes Scenario:** L3-C-053 — Guardian Legal Custody Designation and Change  
**Primary Business Owner:** Guardian  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-053; L2-C06; L2-A02; L2-C01 where applicable

## 2. Behavioral Objective & Boundary
The system shall support authorized designation or change of legal custody information within an existing Guardian Relationship while preserving authoritative Person/Student ownership, historical custody information and Guardian aggregate consistency.

## 3. Required Business Context
- Existing Guardian Relationship.
- Authoritative custody evidence where applicable.
- Applicable Guardian policy.
- Required institutional authority.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-053-BH-001 | The system shall identify the authoritative Guardian Relationship affected by the custody designation or change. | L3-C-053 §§4,9 |
| L4-C-053-BH-002 | The system shall validate custody information against applicable policy and available authoritative evidence. | L3-C-053 §§7,9,15 |
| L4-C-053-BH-003 | The system shall prevent application of a conflicting active custody assignment while the conflict remains unresolved. | L3-C-053 §§3,9,11-12 |
| L4-C-053-BH-004 | The system shall preserve prior custody information as immutable business history. | L3-C-053 §§3,11-13 |
| L4-C-053-BH-005 | The system shall process custody changes through the governed Guardian Relationship modification path. | L3-C-053 §13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-053-BH-006 | The system shall preserve Person & Identity and Student Lifecycle ownership of their authoritative facts. | L3-C-053 §5 |
| L4-C-053-BH-007 | On successful custody change, the system shall publish `LegalCustodyUpdated` from the Guardian Relationship Aggregate. | L3-C-053 §14 |

## 6. Failure / Rejection / Continuation
Incomplete evidence, missing authority or unresolved custody conflict shall prevent the change. Existing custody state shall remain intact until a governed decision permits continuation. Historical custody information shall not be overwritten.

## 7. Audit & Evidence Behavior
Preserve custody request, evidence, policy basis, authority, conflict evaluation, resulting decision and prior custody history.

## 8. Observable Completion Conditions
Completion requires valid custody evidence/policy evaluation, no unresolved conflicting active custody, authorized recording of the designation/change, preserved history and `LegalCustodyUpdated` publication.

## 9. Traceability & Provenance
**Parent:** L3-C-053 — Guardian Legal Custody Designation and Change  
**Primary verification source:** L2-C06 — Guardian Workflows.

## 10. Open Issues / Deferred Design
No universal legal-document taxonomy or external legal-fact owner is defined by the source. Exact authority, persistence and interface mechanics remain deferred.