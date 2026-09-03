# Library Copy Condition Change to Operational Restriction

## 1. Specification Metadata

**Specification ID:** L4-G-130  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-130 — Library Copy Condition Change to Operational Restriction  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-130; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a material physical-condition change for a Library Copy and establish the appropriate operational status/restriction without inventing a `Restricted` copy lifecycle state.

## 3. Required Business Context

- The affected Library Copy exists.
- Physical condition and current operational status can be established.
- Applicable configured copy/circulation policy and authority are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-130-BH-001 | Identify the Library Copy and record the observed physical-condition change. | L3-G-130 §§7–9 |
| L4-G-130-BH-002 | Assess whether the changed condition affects ordinary circulation eligibility. | L3-G-130 §§9,12 |
| L4-G-130-BH-003 | Establish the applicable authoritative/configured operational status, such as `Under Verification`, `Damaged`, `Lost`, `Withdrawn`, or `Reference Only`, only when supported by the business assessment. | L3-G-130 §§9,12–13 |
| L4-G-130-BH-004 | Prevent ordinary circulation while the resulting operational status does not permit it. | L3-G-130 §§3,9,16 |
| L4-G-130-BH-005 | Preserve prior condition, resulting status/restriction and copy history. | L3-G-130 §§12,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-130-BH-006 | The retrieved G05 event catalogue exposes no dedicated Library Copy condition/status-change event; no `LibraryCopyDamaged`, `LibraryCopyRestricted`, or `CopyConditionChanged` event shall be invented. The resulting business fact remains Library Management-owned. | L3-G-130 §§14,17 |

## 6. Failure / Rejection / Continuation

- Insufficient condition evidence shall prevent unsupported status assignment.
- A copy that remains circulation-eligible shall not be unnecessarily restricted.
- Repair, replacement, fine, or later withdrawal outcomes remain separate governed processes where applicable.
- No new `Restricted` lifecycle state shall be introduced.

## 7. Audit & Evidence Behavior

Preserve condition observation, assessment, authority, resulting operational status/restriction and prior history.

## 8. Observable Completion Conditions

Completion occurs when the condition change and resulting operational status are authoritative, subsequent circulation respects that status, and history is traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-130 — Library Copy Condition Change to Operational Restriction  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Dedicated Library Copy condition/status event vocabulary remains unresolved upstream. Exact status selection criteria remain configuration governed. Technical inventory/device mechanics, APIs, databases, UI and infrastructure are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
