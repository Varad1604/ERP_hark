# L4-G-091 — Mess Allocation Change to Effective Reallocation

## 1. Specification Metadata
**Specification ID:** L4-G-091  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Hostel Management  
**Realizes Scenario:** L3-G-091 — Mess Allocation Change to Effective Reallocation  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad

## 2. Behavioral Objective & Boundary
SutraERP shall govern an authorised change to an existing Mess Allocation so the revised dining assignment becomes effective while prior allocation history remains traceable.

**Start:** Change to an existing Mess Allocation is initiated.  
**End:** Revised allocation is effective and history records the change.

## 3. Required Business Context
- Existing Mess Allocation is authoritative.
- Requested change is permitted by configured hostel policy.
- Valid revised assignment can be established.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior |
|---|---|
| L4-G-091-BH-001 | Identify the current Mess Allocation and requested change. |
| L4-G-091-BH-002 | Evaluate the proposed change against configured allocation rules. |
| L4-G-091-BH-003 | Complete required hostel approval where configured. |
| L4-G-091-BH-004 | Make the revised Mess Allocation effective for its applicable period. |
| L4-G-091-BH-005 | Retain the previous allocation as historical information. |
| L4-G-091-BH-006 | Preserve evidence of the decision and effective reallocation. |

## 5. Cross-Domain & Event Behavior
Hostel Management retains Mess Allocation ownership. Notifications may consume the revised authoritative outcome. Exact canonical reallocation event names are not invented where not exposed upstream.

## 6. Failure / Rejection / Continuation
- Policy-ineligible changes shall not be activated.
- If no valid revised assignment can be established, preserve the current authoritative allocation.
- Notification failure shall not invalidate the effective reallocation.

## 7. Audit & Evidence Behavior
Preserve current allocation, requested change, evaluation, approval, revised effective period, resulting assignment, and historical traceability.

## 8. Observable Completion Conditions
Required decision is complete, revised Mess Allocation is effective, prior allocation remains traceable, and the authoritative outcome is preserved.

## 9. Traceability & Provenance
**Parent:** L3-G-091 — Mess Allocation Change to Effective Reallocation  
**Workflow:** L2-G03 — Hostel Management Workflows

## 10. Open Issues / Deferred Design
Exact canonical Mess Allocation change event/state names require confirmation before Architecture Approval/Lock. Technical persistence, interfaces, APIs, databases, and UI remain outside L4.