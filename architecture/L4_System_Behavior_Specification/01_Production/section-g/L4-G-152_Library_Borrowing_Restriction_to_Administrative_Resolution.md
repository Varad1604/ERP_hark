# Library Borrowing Restriction to Administrative Resolution

## 1. Specification Metadata

**Specification ID:** L4-G-152  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-152 — Library Borrowing Restriction to Administrative Resolution  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-152; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authoritative borrowing restriction through validation and Library Management administrative resolution, without inventing a separate restriction aggregate/lifecycle or assuming that Finance payment automatically removes every restriction.

## 3. Required Business Context

- A borrowing request or active borrowing context exists.
- Resource/member context and the authoritative restriction basis are identifiable.
- Applicable borrowing validation rules and Library Management authority are available.
- C/F/H context is consumed only within those domains' ownership boundaries.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-152-BH-001 | Identify the affected borrowing context and the authoritative condition preventing normal borrowing progression. | L3-G-152 §§7–9 |
| L4-G-152-BH-002 | Evaluate Active Membership, Borrowing Limit, Outstanding Dues, Resource Availability, Reservation Priority and Administrative Restrictions according to configurable borrowing validation policy. | L3-G-152 §§2,9,12 |
| L4-G-152-BH-003 | Consume C-owned student context and F-owned financial/payment context where applicable without reinterpreting or taking ownership of those facts. | L3-G-152 §§5–6,9,11–12 |
| L4-G-152-BH-004 | Establish the authoritative administrative resolution: the restriction remains applicable, is resolved/removed under the applicable rule, or normal borrowing remains unable to proceed. | L3-G-152 §§3,9–10,16 |
| L4-G-152-BH-005 | If the borrowing condition becomes satisfied, permit continuation only through the separate authoritative borrowing workflow; do not duplicate the borrowing lifecycle locally. | L3-G-152 §§9–10,13,16 |
| L4-G-152-BH-006 | Treat the restriction as a validation/business condition, not as a new `BorrowingRestriction` lifecycle or invented restriction state. | L3-G-152 §§3,13,17 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-152-BH-007 | The retrieved source defines no dedicated borrowing-restriction event. This specification shall not invent one. Existing circulation events such as `ResourceIssued`, `ResourceReturned`, `ResourceRenewed` and `ResourceOverdue` apply only when their actual business transitions occur. G owns the restriction decision; C and F own their supplied context; H owns notification delivery. | L3-G-152 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- Missing validation information shall not be replaced with invented facts.
- G shall not invent Finance-owned payment/accounting status.
- C remains authoritative for student lifecycle context.
- A pending administrative decision leaves the restriction unresolved.
- Resource unavailability shall follow the applicable availability/reservation workflow rather than being silently absorbed into this scenario.

## 7. Audit & Evidence Behavior

Preserve the affected borrowing/request context, restriction reason, validation findings, C/F context used, administrative decision, resulting restriction condition and H notification hand-off where applicable.

## 8. Observable Completion Conditions

Completion occurs when the restriction has an authoritative administrative resolution, the decision/history is preserved, and any resumed borrowing proceeds only through the existing borrowing workflow.

## 9. Traceability & Provenance

**Parent:** L3-G-152 — Library Borrowing Restriction to Administrative Resolution  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The upstream source defines no dedicated borrowing-restriction lifecycle/event and no universal restriction-cause resolution matrix. These gaps are preserved. Technical implementation remains deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
