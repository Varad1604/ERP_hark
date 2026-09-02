# Workforce Member Establishment as Active Faculty

## 1. Specification Metadata

**Specification ID:** L4-E-026  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-026 — Workforce Member Establishment as Active Faculty  
**Primary Business Owner:** Faculty Workforce  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-026; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish an eligible Faculty Workforce representation and activate it only after required faculty verification and approval conditions are satisfied. L4 begins when an authoritative workforce/employee context exists and a faculty representation is required. L4 ends when Faculty Workforce is Active or the governed non-active/rejected outcome remains authoritative. HR employment ownership and Position & Assignment ownership remain outside this specification.

## 3. Required Business Context

- Authoritative workforce/employee context.
- Faculty information required for establishment.
- Duplicate Faculty Workforce condition.
- Faculty verification criteria.
- Required institutional approval.
- Organisational assignment context where required.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-026-BH-001 | Before creating a Faculty Workforce representation, the system shall validate the authoritative workforce context and prevent duplicate Faculty Workforce representation for the applicable workforce member. | L3-E-026 §§7, 9, 10, 12 |
| L4-E-026-BH-002 | The system shall establish the Faculty Workforce representation in its initial lifecycle position before faculty activation is attempted. | L3-E-026 §§4.3, 9, 13 |
| L4-E-026-BH-003 | The system shall require successful faculty verification and the applicable institutional approval outcome before activating the Faculty Workforce. | L3-E-026 §§3, 7, 9, 15 |
| L4-E-026-BH-004 | When faculty verification is rejected, the system shall prevent the Faculty Workforce from reaching Active and shall preserve the governed rejection outcome. | L3-E-026 §§10, 11 |
| L4-E-026-BH-005 | On successful activation, the system shall establish Faculty Workforce as Active and recognise `FacultyWorkforceActivated` as the canonical business fact. | L3-E-026 §§3, 9, 13, 14 |
| L4-E-026-BH-006 | The system shall preserve the Faculty Workforce lifecycle history and activation evidence. | L3-E-026 §§9, 11, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-026-BH-007 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Faculty Workforce ownership. | L3-E-026 §14 |

## 6. Failure / Rejection / Continuation

- If required faculty information, verification, or approval conditions are not satisfied, the system shall not establish an Active Faculty Workforce outcome. The applicable non-active or rejection position remains authoritative until a governed correction or subsequent valid progression occurs.

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable: - authoritative workforce context used; - duplicate check result; - faculty information considered; - verification outcome; - approval outcome where required; - resulting Faculty Workforce state; and - canonical activation or rejection fact.

## 8. Observable Completion Conditions

The behavior is complete when: - the Faculty Workforce representation exists; - required verification and approval conditions are satisfied for activation; - Faculty Workforce is Active, or the governed rejection/non-active outcome is authoritative; and - applicable lifecycle history and canonical business facts are preserved.

## 9. Traceability & Provenance

**Parent:** L3-E-026 — Workforce Member Establishment as Active Faculty  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
