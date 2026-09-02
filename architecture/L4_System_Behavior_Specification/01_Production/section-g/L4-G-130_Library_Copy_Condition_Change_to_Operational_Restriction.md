# Library Copy Condition Change to Operational Restriction

## 1. Specification Metadata

**Specification ID:** L4-G-130  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-130 — Library Copy Condition Change to Operational Restriction  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-130; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a material change in library-copy condition and apply the resulting operational restriction where circulation eligibility is affected.

## 3. Required Business Context

- A registered library copy exists.
- Condition assessment is authoritative within Library Management.
- Applicable circulation and restriction policy is available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-130-BH-001 | Register the identified condition change against the authoritative copy. | L3-G-130 |
| L4-G-130-BH-002 | Evaluate the changed condition against applicable circulation criteria. | L3-G-130 |
| L4-G-130-BH-003 | Determine the governed operational restriction where circulation eligibility is affected. | L3-G-130 |
| L4-G-130-BH-004 | Apply the resulting restriction to the copy's operational circulation position. | L3-G-130 |
| L4-G-130-BH-005 | Preserve previous condition and restriction history. | L3-G-130 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-130-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Library Management ownership. | L3-G-130 §14 |

## 6. Failure / Rejection / Continuation

- Incomplete condition evidence shall prevent final restriction determination where evidence is required.
- A copy that remains circulation-eligible shall not be unnecessarily restricted.
- Historical condition changes shall remain traceable.

## 7. Audit & Evidence Behavior

Preserve condition assessment, basis, authority, resulting restriction, effective position, and prior history.

## 8. Observable Completion Conditions

Completion occurs when the condition change is recorded and the resulting operational circulation position is effective and traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-130 — Library Copy Condition Change to Operational Restriction  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
