# Transport Pass Suspension to Restricted Transport Access

## 1. Specification Metadata

**Specification ID:** L4-G-116  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-116 — Transport Pass Suspension to Restricted Transport Access  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-116; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern suspension of a Transport Pass and the resulting restriction of transport access without transferring student or financial ownership.

## 3. Required Business Context

- An Active Transport Pass exists.
- Suspension basis, policy and authority are authoritative.
- Transport Management owns pass status and validity.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-116-BH-001 | Identify the Active Transport Pass and holder subject to suspension. | L3-G-116 §§3, 7, 9 |
| L4-G-116-BH-002 | Evaluate the suspension basis against configured policy and required authority. | L3-G-116 §§9, 12, 15 |
| L4-G-116-BH-003 | Move the pass to the authoritative `Suspended` state only after the required business decision is established. | L3-G-116 §§9, 13, 16 |
| L4-G-116-BH-004 | Restrict operational transport access in accordance with the Suspended pass outcome without creating a separate Restricted lifecycle state. | L3-G-116 §§3, 13, 16 |
| L4-G-116-BH-005 | Preserve prior Active-pass history, suspension basis, effective period and decision evidence. | L3-G-116 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-116-BH-006 | On successful pass suspension, Transport Management shall publish `PassSuspended`. | L3-G-116 §§14, 16 |

## 6. Failure / Rejection / Continuation

- Unauthorised or rejected suspension shall not change the pass from its current authoritative state.
- If the pass is already Expired, Cancelled or Replaced, suspension shall not be fabricated as an additional lifecycle outcome.
- Restoration/reactivation remains outside this scenario.

## 7. Audit & Evidence Behavior

Preserve suspension basis, authority, effective period, restricted-access outcome and pass lifecycle history.

## 8. Observable Completion Conditions

The pass is `Suspended`, restricted transport access is effective, `PassSuspended` is published, and the suspension decision remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-116 — Transport Pass Suspension to Restricted Transport Access  
**Primary verification source:** L3-G-116 §§3, 9–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

No unresolved event or lifecycle identifier remains for the core suspension outcome. Technical implementation remains deferred to L5/L6.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
