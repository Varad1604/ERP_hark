# L4-G-094 — Transport Route Modification to Effective Route Version

## 1. Specification Metadata
**Specification ID:** L4-G-094  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-094 — Transport Route Modification to Effective Route Version  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-094; L2-G04 Transport Management Workflows

## 2. Behavioral Objective & Boundary
SutraERP shall govern an authorised modification to an existing Transport Route and make the resulting route version effective while preserving prior route history.

**Start:** A route modification is requested.  
**End:** The authorised revised route version is effective.

## 3. Required Business Context
- An existing Transport Route is authoritative.
- Proposed route changes are identifiable.
- Applicable validation and authorization rules are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-094-BH-001 | Identify the authoritative Transport Route and requested modification. | L3-G-094 §§2–5 |
| L4-G-094-BH-002 | Evaluate the proposed route modification against applicable operational constraints. | L3-G-094 §§2–3 |
| L4-G-094-BH-003 | Validate affected route stops and sequencing before the revised route becomes effective. | L3-G-094 §2 |
| L4-G-094-BH-004 | Complete the required review and authorization for the modification. | L3-G-094 §§3, 8–9 |
| L4-G-094-BH-005 | Make the authorised revised route version effective. | L3-G-094 §§3, 16 |
| L4-G-094-BH-006 | Preserve the prior route version and modification history. | L3-G-094 §§3, 16 |

## 5. Cross-Domain & Event Behavior
Transport Management remains the canonical owner and publisher of the route modification outcome. No unsupported event name is introduced.

## 6. Failure / Rejection / Continuation
- Invalid modifications shall not become effective.
- Incomplete validation or authorization shall retain the current effective route version.
- A failed modification shall preserve the prior authoritative route history.

## 7. Audit & Evidence Behavior
Preserve the requested modification, validation, review, authorization, effective version and historical route evidence.

## 8. Observable Completion Conditions
The revised route version is authorised and effective, affected sequencing is valid, and the previous route history remains traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-094 — Transport Route Modification to Effective Route Version  
**Authoritative workflow:** L2-G04 — Transport Management Workflows

## 10. Open Issues / Deferred Design
Exact event names and implementation mechanics remain outside L4 where not frozen upstream.