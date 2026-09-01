# Infrastructure Capacity Definition to Capacity Profile Activation

## 1. Specification Metadata
**Specification ID:** L4-G-288  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-288  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-288_Infrastructure_Capacity_Definition_to_Capacity_Profile_Activation.md`; L2-G10 — Infrastructure Management Workflows; L2-A04; L2-A06; L2-A07  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall establish or change the authoritative G10 Capacity Profile for applicable physical infrastructure without inventing a Capacity Profile lifecycle or changing the physical Infrastructure lifecycle.

## 3. Required Business Context
The infrastructure, supported capacity type, valid non-negative capacity value, governing effective context and authorization shall be resolvable.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-288-BH-001 | The system shall reject a capacity change when the infrastructure is ineligible, capacity type unsupported, value negative/invalid, policy invalid or authorization denied. | L3-G-288 §§7,9,11–12 |
| L4-G-288-BH-002 | The system shall preserve prior capacity history and make the valid successor Capacity Profile the authoritative current capacity fact. | L3-G-288 §§2–4,9,12–13 |
| L4-G-288-BH-003 | The system shall support source-defined effective dating, multiple capacity types and bounded temporary overrides without rewriting prior capacity history. | L3-G-288 §§4,9–12 |
| L4-G-288-BH-004 | The system shall not create an `Active` Capacity Profile lifecycle state merely because the effective profile is authoritative. | L3-G-288 §§12–13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-288-BH-005 | On establishment or change of the authoritative Capacity Profile, the system shall publish `CapacityUpdated` from G10. | L3-G-288 §§9,12,14 |
| L4-G-288-BH-006 | The system shall not publish legacy `CapacityChanged` or `RoomCapacityChanged` as separate semantic events. | L3-G-288 §§12,14 |

## 6. Failure / Rejection / Continuation
Failed validation shall leave the prior Capacity Profile authoritative.

## 7. Audit & Evidence Behavior
Evidence shall preserve prior/new capacity, capacity type, effective dates, override lineage where applicable, reason, actor/authority and `CapacityUpdated` publication.

## 8. Observable Completion Conditions
A valid Capacity Profile is authoritative, prior history is retained, physical Infrastructure lifecycle is unchanged and `CapacityUpdated` is published.

## 9. Traceability & Provenance
**Parent:** L3-G-288 — Infrastructure Capacity Definition to Capacity Profile Activation  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows

## 10. Open Issues / Deferred Design
No Capacity Profile `Active` state is implied. Implementation mechanics are deferred.
