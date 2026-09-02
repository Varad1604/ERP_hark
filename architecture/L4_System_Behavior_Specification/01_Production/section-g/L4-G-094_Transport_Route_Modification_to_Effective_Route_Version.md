# Transport Route Modification to Effective Route Version

## 1. Specification Metadata

**Specification ID:** L4-G-094  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-094 — Transport Route Modification to Effective Route Version  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-094; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised modification to an existing Transport Route and make the resulting route version effective while preserving prior route history.

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

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-094-BH-007 | The effective revised Transport Route shall remain authoritative under Transport Management and may be consumed by authorised downstream route consumers. The exact canonical event identifier for effective route modification is unresolved upstream; this specification shall not invent one. | L3-G-094 §§14, 17 |

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
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The exact canonical route-modification event identifier remains unresolved upstream and is deliberately not invented here. Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
