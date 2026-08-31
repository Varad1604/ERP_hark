# Guardian Relationship Reinstatement

## 1. Specification Metadata
**Specification ID:** L4-C-060  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Guardian  
**Realizes Scenario:** L3-C-060 — Guardian Relationship Reinstatement  
**Primary Business Owner:** Guardian  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-060; authoritative Guardian L2 workflow; materially applicable L1/L2 sources

## 2. Behavioral Objective & Boundary
The system shall restore a previously Suspended Guardian Relationship to Active when applicable institutional conditions and authority are satisfied. It shall preserve suspension history and shall not treat reinstatement as creation or termination.

## 3. Required Business Context
- Existing Guardian Relationship in Suspended state.
- Applicable reinstatement condition and Guardian policy.
- Required authority/evidence.
- Prior suspension history.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-060-BH-001 | The system shall identify the suspended Guardian Relationship eligible for reinstatement. | L3-C-060 §§4,7,9 |
| L4-C-060-BH-002 | The system shall validate reinstatement conditions and required authority. | L3-C-060 §§7,9,15 |
| L4-C-060-BH-003 | The system shall transition the relationship from Suspended to Active when governed conditions are satisfied. | L3-C-060 §§3,9,13 |
| L4-C-060-BH-004 | The system shall preserve the prior suspension and reinstatement history. | L3-C-060 §§3,10-11 |
| L4-C-060-BH-005 | The system shall not create a new Guardian Relationship as a substitute for reinstatement. | L3-C-060 §§2,4 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-060-BH-006 | The system shall publish `GuardianRelationshipReinstated` from the Guardian Relationship Aggregate after successful reinstatement. | L3-C-060 §14 |
| L4-C-060-BH-007 | The system shall make the restored Active relationship fact available to downstream consumers without transferring Guardian ownership. | L3-C-060 §§3,9,14 |

## 6. Failure / Rejection / Continuation
If reinstatement conditions or authority are insufficient, the relationship shall remain Suspended and prior history shall remain intact. A relationship requiring termination shall not be silently reinstated.

## 7. Audit & Evidence Behavior
Preserve reinstatement request, prior suspension, policy basis, evidence, authority, resulting decision and event history.

## 8. Observable Completion Conditions
Completion requires validated reinstatement eligibility, required authority, Active state, retained suspension history and `GuardianRelationshipReinstated` publication.

## 9. Traceability & Provenance
**Parent:** L3-C-060 — Guardian Relationship Reinstatement  
**Primary verification source:** Authoritative Guardian L2 workflow and materially applicable L1/L2 sources.

## 10. Open Issues / Deferred Design
Institution-specific reinstatement conditions and approval authority remain upstream/configurable. Technical IAM, persistence, interfaces and implementation mechanics remain deferred.