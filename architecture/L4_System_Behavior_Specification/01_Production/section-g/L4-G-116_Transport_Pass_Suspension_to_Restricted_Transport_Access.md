# L4-G-116 — Transport Pass Suspension to Restricted Transport Access

## 1. Specification Metadata
**Specification ID:** L4-G-116  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-116 — Transport Pass Suspension to Restricted Transport Access  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern suspension of a Transport Pass and the resulting restriction of transport access.

**Start:** A governed condition requires pass suspension.  
**End:** Suspension is authorised and restricted transport access is recorded.

## 3. Required Business Context
The Transport Pass, holder context, suspension reason and applicable authority shall be authoritative.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-116-BH-001 | Identify the Transport Pass subject to suspension. | L3-G-116 |
| L4-G-116-BH-002 | Evaluate the suspension condition and applicable authority. | L3-G-116 |
| L4-G-116-BH-003 | Record the governed suspension decision and effective context. | L3-G-116 |
| L4-G-116-BH-004 | Restrict transport access in accordance with the suspended pass outcome. | L3-G-116 |
| L4-G-116-BH-005 | Preserve prior active-pass history and suspension history. | L3-G-116 |

## 5. Cross-Domain & Event Behavior
Transport Management retains ownership of the Transport Pass. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
Unauthorised suspension shall not be applied. If the condition or authority cannot be established, the pass remains in its prior governed position.

## 7. Audit & Evidence Behavior
Preserve reason, authority, effective period, restricted-access outcome and lifecycle history.

## 8. Observable Completion Conditions
The pass is governed as suspended, restricted access is recorded, and the decision is traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-116 — Transport Pass Suspension to Restricted Transport Access

## 10. Open Issues / Deferred Design
Technical access enforcement, persistence, interfaces and notification mechanisms remain outside L4.