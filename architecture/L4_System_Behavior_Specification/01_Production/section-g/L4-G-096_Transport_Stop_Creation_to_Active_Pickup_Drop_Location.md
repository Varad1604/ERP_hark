# L4-G-096 — Transport Stop Creation to Active Pickup/Drop Location

## 1. Specification Metadata
**Specification ID:** L4-G-096  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-096 — Transport Stop Creation to Active Pickup/Drop Location  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-096; L2-G04 Transport Management Workflows

## 2. Behavioral Objective & Boundary
SutraERP shall establish and activate a valid Transport Stop as an operational pickup/drop location under Transport Management ownership.

**Start:** A requirement for a transport stop is established.  
**End:** The stop is Active and available for route use.

## 3. Required Business Context
- Transport Management owns the Transport Stop lifecycle.
- Location context and intended operational use are identifiable.
- Applicable stop validation and authorization rules are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-096-BH-001 | Establish the proposed Transport Stop within Transport Management ownership. | L3-G-096 |
| L4-G-096-BH-002 | Validate the stop's operational location and required business context. | L3-G-096 |
| L4-G-096-BH-003 | Apply required review and authorization before activation. | L3-G-096 |
| L4-G-096-BH-004 | Activate the Transport Stop when required business conditions are satisfied. | L3-G-096 |
| L4-G-096-BH-005 | Preserve the stop's activation history and evidence. | L3-G-096 |

## 5. Cross-Domain & Event Behavior
Transport Management remains the canonical owner and publisher of the Transport Stop outcome. Exact event names are not fabricated.

## 6. Failure / Rejection / Continuation
- Invalid location/context shall prevent activation.
- Failed review shall leave the stop non-active.
- Duplicate or conflicting operational use shall be resolved under the applicable business process before activation.

## 7. Audit & Evidence Behavior
Preserve stop definition, validation, authorization, activation and historical evidence.

## 8. Observable Completion Conditions
The stop is valid, authorised, Active, available for operational route use, and its activation evidence is preserved.

## 9. Traceability & Provenance
**Parent:** L3-G-096 — Transport Stop Creation to Active Pickup/Drop Location  
**Authoritative workflow:** L2-G04 — Transport Management Workflows

## 10. Open Issues / Deferred Design
Technical location services, persistence, APIs, interfaces and implementation mechanics remain outside L4.