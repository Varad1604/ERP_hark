# Library Borrowing Renewal to Extended Due Date

## 1. Specification Metadata

**Specification ID:** L4-G-133  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-133 — Library Borrowing Renewal to Extended Due Date  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-133; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall evaluate renewal of an Active borrowing and, when configured renewal conditions are satisfied, extend its due date without creating a new borrowing or changing ownership.

## 3. Required Business Context

- Active borrowing exists.
- Membership is Active.
- Library Copy remains associated with the borrowing.
- Reservation, renewal-limit, resource-eligibility and applicable renewal-policy context is available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-133-BH-001 | Identify the Active borrowing and associated member/resource context. | L3-G-133 §§7,9 |
| L4-G-133-BH-002 | Evaluate Active Membership, reservation existence, renewal limit and resource eligibility against configured renewal policy. | L3-G-133 §§7,12 |
| L4-G-133-BH-003 | Prevent renewal when a source-defined eligibility condition is not satisfied. | L3-G-133 §§10,11 |
| L4-G-133-BH-004 | When renewal is approved, establish the extended due date on the existing Active borrowing. | L3-G-133 §§3,9,13 |
| L4-G-133-BH-005 | Preserve the prior due date and renewal history. | L3-G-133 §§3,11,15 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-133-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Library Management ownership. | L3-G-133 §14 |

## 6. Failure / Rejection / Continuation

- If Membership is not Active, renewal shall be rejected.
- If a prohibited reservation exists, renewal shall be rejected.
- If the configured renewal limit is exceeded, renewal shall be rejected.
- If the resource is not eligible, renewal shall be rejected.
- On rejection, the existing borrowing and due date remain authoritative.

## 7. Audit & Evidence Behavior

Preserve renewal request, eligibility basis, decision, previous and effective due-date information, authority and renewal history.

## 8. Observable Completion Conditions

Completion occurs when an eligible renewal has an authoritative extended due date and `ResourceRenewed` is published, or a governed rejection is recorded without altering the existing due date.

## 9. Traceability & Provenance

**Parent:** L3-G-133 — Library Borrowing Renewal to Extended Due Date  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
