# Mess Allocation Change to Effective Reallocation

## 1. Specification Metadata

**Specification ID:** L4-G-091  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-091 — Mess Allocation Change to Effective Reallocation  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-091; L2-G03 — Hostel Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Govern an authorised change to an existing Mess Allocation so a valid revised dining assignment becomes effective while the prior allocation remains historically traceable.

## 3. Required Business Context

- Existing Mess Allocation is authoritative.
- Requested change is permitted by configured hostel policy.
- A valid revised assignment can be established.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-091-BH-001 | The system shall identify the current Mess Allocation and the requested change. | L3-G-091 §§7–9 |
| L4-G-091-BH-002 | The system shall evaluate the proposed change against configured allocation rules and applicable dining context. | L3-G-091 §§9, 12 |
| L4-G-091-BH-003 | Where required, the system shall require the configured hostel approval before making the revised allocation effective. | L3-G-091 §§6, 9, 15 |
| L4-G-091-BH-004 | When the change is authorised and a valid revised assignment exists, the system shall make the successor Mess Allocation effective for its approved period. | L3-G-091 §§3, 9, 13, 16 |
| L4-G-091-BH-005 | The system shall preserve the previous allocation as historical information and retain one authoritative current assignment. | L3-G-091 §§3, 12, 16 |
| L4-G-091-BH-006 | If the proposed change is not permitted or no valid revised assignment exists, the current allocation shall remain authoritative. | L3-G-091 §§10–11 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-091-BH-007 | G03 Hostel Management remains the canonical publisher of the effective reallocation fact; no exact event name shall be invented because the Locked parent records it as unresolved. | L3-G-091 §§14, 17 |
| L4-G-091-BH-008 | H02 may consume the revised allocation outcome for governed communication without acquiring Mess Allocation ownership. | L3-G-091 §§5, 14 |

## 6. Failure / Rejection / Continuation

- Policy-ineligible changes shall not replace the current allocation.
- No valid revised assignment means the existing allocation remains effective.
- Notification failure does not invalidate an effective reallocation.

## 7. Audit & Evidence Behavior

Preserve current allocation, requested change, rule evaluation, approval, revised effective period, successor assignment and prior history.

## 8. Observable Completion Conditions

Completion requires either an authorised effective successor Mess Allocation with prior history preserved, or a supported non-change outcome retaining the current allocation.

## 9. Traceability & Provenance

**Parent:** L3-G-091 — Mess Allocation Change to Effective Reallocation  
**Primary verification source:** L3-G-091 §§3, 5, 9, 11–17 and authoritative L2-G03 architecture.

## 10. Open Issues / Deferred Design

Exact canonical Mess Allocation reallocation state/event identifiers remain unresolved upstream. Technical implementation remains deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
