# Workforce Deputation Lifecycle

## 1. Specification Metadata

**Specification ID:** L4-E-005  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-005 — Workforce Deputation Lifecycle  
**Primary Business Owner:** People & Workforce  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-005; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Specify observable behavior for establishing, governing, modifying, and concluding a workforce deputation lifecycle.

## 3. Required Business Context

- Authoritative People & Workforce context exists.
- Applicable upstream business policy and governance conditions are satisfied.
- Relevant cross-domain facts remain source-owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-005-BH-001 | The system shall establish deputation eligibility and authority. | L3-E-005 §§3, 9, 12 |
| L4-E-005-BH-002 | The system shall record the deputation arrangement. | L3-E-005 §§3, 9, 12 |
| L4-E-005-BH-003 | The system shall apply governed lifecycle changes. | L3-E-005 §§3, 9, 12 |
| L4-E-005-BH-004 | The system shall preserve deputation history. | L3-E-005 §§3, 9, 12 |
| L4-E-005-BH-005 | The system shall external organisation master-data maintenance. | L3-E-005 §§3, 9, 12 |
| L4-E-005-BH-006 | The system shall payroll settlement. | L3-E-005 §§3, 9, 12 |
| L4-E-005-BH-007 | The system shall technical implementation details. | L3-E-005 §§3, 9, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-005-BH-008 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under People & Workforce ownership. | L3-E-005 §14 |

## 6. Failure / Rejection / Continuation

- Unresolved business conditions prevent the affected deputation lifecycle change. No technical retry, rollback, queue, or infrastructure behavior is specified.

## 7. Audit & Evidence Behavior

Deputation authority follows applicable workforce governance. Deputation decisions and history remain auditable.

## 8. Observable Completion Conditions

- Applicable deputation conditions and authority are established. - The authoritative deputation lifecycle position is recorded. - History remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-E-005 — Workforce Deputation Lifecycle  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
