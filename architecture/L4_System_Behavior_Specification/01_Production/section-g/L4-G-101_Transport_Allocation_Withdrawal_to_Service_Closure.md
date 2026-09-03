# Transport Allocation Withdrawal to Service Closure

## 1. Specification Metadata

**Specification ID:** L4-G-101  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-101 — Transport Allocation Withdrawal to Service Closure  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-101; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern withdrawal of an active Transport Allocation until it reaches the canonical Withdrawn state while preserving the historical service relationship.

## 3. Required Business Context

- An Active Transport Allocation exists.
- A legitimate withdrawal basis is identifiable.
- Applicable withdrawal and approval policy is available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-101-BH-001 | Identify the Active allocation and the authoritative withdrawal basis. | L3-G-101 §§7–9 |
| L4-G-101-BH-002 | Evaluate the withdrawal using the configured transport withdrawal and approval policy. | L3-G-101 §§9, 12, 15 |
| L4-G-101-BH-003 | When authorised, transition the Transport Allocation from Active to Withdrawn. | L3-G-101 §§9, 13, 16 |
| L4-G-101-BH-004 | Prevent the Withdrawn allocation from remaining an effective transport service relationship. | L3-G-101 §§3, 13, 16 |
| L4-G-101-BH-005 | Preserve withdrawal date, remarks, prior route/stop relationship, and allocation history. | L3-G-101 §§9, 12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-101-BH-006 | On allocation withdrawal, Transport Management shall publish `AllocationWithdrawn`. | L3-G-101 §§9, 14, 16 |
| L4-G-101-BH-007 | The authoritative withdrawal fact may be consumed by Student Lifecycle, People & Workforce, Finance, and Institutional Services for their own governed outcomes without modifying the G04 allocation state. | L3-G-101 §§5, 10–11, 14 |

## 6. Failure / Rejection / Continuation

- Rejected or unresolved withdrawal shall leave the allocation Active.
- An allocation already Withdrawn shall not receive a second lifecycle transition.
- Financial, pass, student/workforce, and communication consequences shall remain with their canonical downstream owners.

## 7. Audit & Evidence Behavior

Preserve the withdrawal basis, approval, effective transition, withdrawal date/remarks, `AllocationWithdrawn` publication, downstream hand-off evidence where material, and complete allocation history.

## 8. Observable Completion Conditions

The allocation is Withdrawn, no longer an effective transport service relationship, `AllocationWithdrawn` is published, and its complete history remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-101 — Transport Allocation Withdrawal to Service Closure  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

No material upstream event or lifecycle gap remains for allocation withdrawal. Downstream financial settlement, pass closure, and communication delivery remain separate outcomes. Technical implementation is deferred to later architecture levels.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
