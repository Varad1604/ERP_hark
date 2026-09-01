# L4-G-090 — Mess Allocation to Active Dining Assignment

## 1. Specification Metadata
**Specification ID:** L4-G-090  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Hostel Management  
**Realizes Scenario:** L3-G-090 — Mess Allocation to Active Dining Assignment  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad

## 2. Behavioral Objective & Boundary
SutraERP shall govern assignment of an eligible hostel resident to an effective mess/dining allocation.

**Start:** Mess allocation is initiated.  
**End:** Mess Allocation is active and the dining assignment is effective.

## 3. Required Business Context
- Resident is within applicable hostel residency scope.
- Mess/dining allocation applies.
- Applicable allocation rules and operational constraints are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior |
|---|---|
| L4-G-090-BH-001 | Identify the resident and applicable dining allocation context. |
| L4-G-090-BH-002 | Evaluate applicable mess allocation rules and availability. |
| L4-G-090-BH-003 | Complete required hostel approval where configured. |
| L4-G-090-BH-004 | Establish Mess Allocation with effective period and allocation status. |
| L4-G-090-BH-005 | Preserve the active dining assignment as the authoritative G03 outcome. |
| L4-G-090-BH-006 | Preserve allocation decision and effective-period evidence. |

## 5. Cross-Domain & Event Behavior
Hostel Management remains owner of Mess Allocation. Notifications may consume the authoritative outcome without acquiring ownership. The canonical Mess Allocation event name is not invented where not exposed by authoritative sources.

## 6. Failure / Rejection / Continuation
- Incomplete allocation information shall prevent activation.
- No permitted assignment shall result in no invalid active allocation.
- Unavailable downstream notification shall not invalidate the authoritative hostel allocation.

## 7. Audit & Evidence Behavior
Preserve allocation context, rules considered, approval where applicable, effective period, status, and resulting authoritative outcome.

## 8. Observable Completion Conditions
Eligible resident is identified, applicable rules evaluated, required approval completed, and an effective active Mess Allocation is established and traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-090 — Mess Allocation to Active Dining Assignment  
**Workflow:** L2-G03 — Hostel Management Workflows

## 10. Open Issues / Deferred Design
Exact canonical Mess Allocation event/state names require confirmation before Architecture Approval/Lock. Technical persistence, interfaces, notification transport, APIs, databases, and UI remain outside L4.