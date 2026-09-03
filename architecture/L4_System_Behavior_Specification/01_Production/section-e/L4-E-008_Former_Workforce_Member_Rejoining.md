# Former Workforce Member Rejoining

## 1. Specification Metadata

**Specification ID:** L4-E-008  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-008 — Former Workforce Member Rejoining  
**Primary Business Owner:** Workforce & Engagement  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-008; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Specify observable behavior for rejoining a former workforce member while preserving the existing Workforce Profile and historical engagements.

## 3. Required Business Context

- Authoritative Workforce & Engagement context exists.
- Applicable upstream business policy and governance conditions are satisfied.
- Relevant cross-domain facts remain source-owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-008-BH-001 | The system shall identify canonical Workforce Profile. | L3-E-008 §§3, 9, 12 |
| L4-E-008-BH-002 | The system shall verify authorised rejoining outcome. | L3-E-008 §§3, 9, 12 |
| L4-E-008-BH-003 | The system shall create new Workforce Engagement. | L3-E-008 §§3, 9, 12 |
| L4-E-008-BH-004 | The system shall preserve historical engagements. | L3-E-008 §§3, 9, 12 |
| L4-E-008-BH-005 | The system shall publish canonical rejoining event. | L3-E-008 §§3, 9, 12 |
| L4-E-008-BH-006 | The system shall coordinate subsequent E02 placement where required. | L3-E-008 §§3, 9, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-008-BH-007 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Workforce & Engagement ownership. | L3-E-008 §14 |

## 6. Failure / Rejection / Continuation

- If the new engagement cannot validly complete, the canonical profile and historical engagements remain unchanged. No technical retry or rollback behavior is specified.

## 7. Audit & Evidence Behavior

Rejoining follows applicable policy-driven appointment/workflow governance. The decision and new engagement remain auditable.

## 8. Observable Completion Conditions

- Existing Workforce Profile remains canonical. - New Workforce Engagement exists beneath it. - Historical engagements remain preserved. - `WorkforceRejoined` is published.

## 9. Traceability & Provenance

**Parent:** L3-E-008 — Former Workforce Member Rejoining  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
