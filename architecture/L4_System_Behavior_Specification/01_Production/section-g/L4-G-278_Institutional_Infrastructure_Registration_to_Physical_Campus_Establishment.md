# Institutional Infrastructure Registration to Physical Campus Establishment

## 1. Specification Metadata

**Specification ID:** L4-G-278  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-278  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-G-278_Institutional_Infrastructure_Registration_to_Physical_Campus_Establishment.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows; L2-A01 — Platform & Tenant Foundation  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish one canonical G10 Physical Campus from an authorised institutional infrastructure registration while preserving B03 ownership of InstitutionalCampus identity/governance. Registration establishes physical identity and required registration information; it does not make the campus operational.

## 3. Required Business Context

- The InstitutionalCampus context is resolvable from B03 without duplicating its identity.
- The proposed Physical Campus code is unique within the source-defined scope.
- Required physical registration information and authorization are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-278-BH-001 | The system shall prevent registration when the InstitutionalCampus reference, required physical information, authorization, or campus-code uniqueness cannot be validated. | L3-G-278 §§5–9, 12 |
| L4-G-278-BH-002 | On valid registration, the system shall create one G10-owned Physical Campus with immutable canonical identity and preserve the B03 InstitutionalCampus only as an institutional reference. | L3-G-278 §§2–5, 9, 12–13 |
| L4-G-278-BH-003 | The system shall place the newly registered Physical Campus at `Proposed`. | L3-G-278 §§9, 13 |
| L4-G-278-BH-004 | The system shall prevent registration from being treated as operational activation; only a source-defined later lifecycle path may make Infrastructure operational. | L3-G-278 §§3–4, 9, 12–13 |
| L4-G-278-BH-005 | The system shall preserve the campus code and canonical physical identity as immutable after establishment. | L3-G-278 §§4, 12, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-278-BH-006 | On successful Physical Campus registration, the system shall publish `CampusRegistered` as the G10 registration fact and shall not publish `CampusCreated` as an alias. | L3-G-278 §§9, 12, 14 |

B03 retains InstitutionalCampus governance. Section A may supply platform authorization, configuration, event transport and audit capability without becoming owner or publisher of the physical campus fact.

## 6. Failure / Rejection / Continuation

Invalid identity, duplicate code, missing institutional reference, incomplete required physical information or failed authorization shall prevent establishment. Any later operational activation shall occur only through a separately governed source-defined lifecycle.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-278-BH-007 | The system shall preserve registration evidence including canonical Physical Campus identity, InstitutionalCampus reference, campus code, supplied registration information, actor/authority context, resulting `Proposed` position and `CampusRegistered` publication. | L3-G-278 §§15–16 |

## 8. Observable Completion Conditions

Completion requires exactly one canonical G10 Physical Campus to exist at `Proposed`, its immutable identity and B03 reference to be preserved, and `CampusRegistered` to be published. Operational availability is not a completion condition.

## 9. Traceability & Provenance

**Parent:** L3-G-278 — Institutional Infrastructure Registration to Physical Campus Establishment  
**Primary verification sources:** L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows

## 10. Open Issues / Deferred Design

No L4 correctness issue remains for the registration outcome. Implementation mechanisms remain deferred.