# L4-G-093 — Transport Route Creation to Operational Activation

## 1. Specification Metadata
**Specification ID:** L4-G-093  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-093 — Transport Route Creation to Operational Activation  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-093; L2-G04 Transport Management Workflows

## 2. Behavioral Objective & Boundary
SutraERP shall govern establishment, validation, review, authorization and activation of a Transport Route. Route activation is the terminal outcome; downstream allocation, trip planning, driver assignment and vehicle assignment remain separate scenarios.

**Start:** A requirement to establish a new operational route is received.  
**End:** The authorised Transport Route is Active and available for transport operations.

## 3. Required Business Context
- Transport Management owns the Transport Route aggregate.
- The intended route context and required stops are identifiable.
- Applicable operational route rules and review authority are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-093-BH-001 | Establish the proposed Transport Route within Transport Management ownership. | L3-G-093 §§2–5 |
| L4-G-093-BH-002 | Establish the required operational route context and associated stops. | L3-G-093 §2 |
| L4-G-093-BH-003 | Establish and validate the required stop sequence before activation. | L3-G-093 §2 |
| L4-G-093-BH-004 | Apply the applicable operational review and authorization path. | L3-G-093 §§3, 8–9 |
| L4-G-093-BH-005 | Activate the route only after required business conditions are satisfied. | L3-G-093 §§3, 16 |
| L4-G-093-BH-006 | Preserve route history and evidence supporting activation. | L3-G-093 §§3, 16 |

## 5. Cross-Domain & Event Behavior
Transport Management remains the sole publisher/owner of the Transport Route business outcome. The exact canonical event name is not asserted where absent from the authoritative L3 material; no event name is fabricated.

## 6. Failure / Rejection / Continuation
- Incomplete route definition shall prevent activation.
- Invalid or incomplete stop sequencing shall prevent activation until corrected.
- Failed review or authorization shall leave the route non-active.

## 7. Audit & Evidence Behavior
Preserve attributable evidence of route definition, stop assignment, sequencing, review, authorization and activation outcome.

## 8. Observable Completion Conditions
- Valid route definition exists.
- Required stops and sequence are established.
- Required review/authorization is complete.
- Transport Route is Active and operationally available.
- Activation evidence is preserved.

## 9. Traceability & Provenance
**Parent:** L3-G-093 — Transport Route Creation to Operational Activation  
**Authoritative workflow:** L2-G04 — Transport Management Workflows

## 10. Open Issues / Deferred Design
Exact event names, technical persistence, APIs, services, interfaces and deployment mechanics remain outside L4 and are deferred where not explicitly frozen upstream.