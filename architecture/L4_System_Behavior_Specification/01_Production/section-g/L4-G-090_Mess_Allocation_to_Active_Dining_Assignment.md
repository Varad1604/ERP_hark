# Mess Allocation to Active Dining Assignment

## 1. Specification Metadata

**Specification ID:** L4-G-090  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-090 — Mess Allocation to Active Dining Assignment  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-090; L2-G03 — Hostel Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Govern assignment of an eligible hostel resident to an effective active Mess Allocation while preserving Hostel Management ownership and downstream Notification separation.

## 3. Required Business Context

- Resident is within applicable hostel residency scope.
- Mess allocation is required.
- Applicable configured allocation rules and approval conditions are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-090-BH-001 | The system shall identify the eligible resident and applicable mess/dining allocation context. | L3-G-090 §§7–9 |
| L4-G-090-BH-002 | The system shall evaluate permitted mess assignment options under configured Hostel Management allocation rules. | L3-G-090 §§9, 12 |
| L4-G-090-BH-003 | Where configured policy requires approval, the system shall require the applicable hostel authority before activation. | L3-G-090 §§6, 9, 15 |
| L4-G-090-BH-004 | When a permitted assignment is established, the system shall record the Mess Allocation, effective period and allocation status so the dining assignment becomes active. | L3-G-090 §§3, 9, 12–13, 16 |
| L4-G-090-BH-005 | If no permitted assignment or required approval exists, the system shall not establish an invalid active Mess Allocation. | L3-G-090 §§10–11 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-090-BH-006 | G03 Hostel Management remains the canonical publisher of the active Mess Allocation fact; no event identifier shall be invented because the Locked parent records the exact event name as unresolved. | L3-G-090 §§14, 17 |
| L4-G-090-BH-007 | H02 may consume the authoritative allocation outcome for governed communication without acquiring Mess Allocation ownership. | L3-G-090 §§5, 14 |

## 6. Failure / Rejection / Continuation

- Incomplete allocation context prevents activation.
- No permitted assignment means no active allocation is created.
- Notification failure does not invalidate an already-authoritative Mess Allocation.

## 7. Audit & Evidence Behavior

Preserve resident context, rules considered, approval where applicable, assignment, effective period, allocation status and outcome history.

## 8. Observable Completion Conditions

Completion requires an eligible resident, permitted assignment, required approval where applicable, and an effective active Mess Allocation that remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-090 — Mess Allocation to Active Dining Assignment  
**Primary verification source:** L3-G-090 §§3, 5, 9, 11–17 and authoritative L2-G03 architecture.

## 10. Open Issues / Deferred Design

Exact canonical Mess Allocation event naming remains unresolved upstream. Technical notification and allocation implementation are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
