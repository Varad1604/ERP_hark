# L4-G-101 — Transport Allocation Withdrawal to Service Closure

## 1. Specification Metadata
**Specification ID:** L4-G-101  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-101 — Transport Allocation Withdrawal to Service Closure  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-101; L2-G04 Transport Management Workflows

## 2. Behavioral Objective & Boundary
SutraERP shall govern withdrawal of an active Transport Allocation through authorised closure while preserving historical allocation evidence.

**Start:** Withdrawal of an active transport allocation is initiated.  
**End:** The allocation is closed and no longer effective for future service use.

## 3. Required Business Context
- An active Transport Allocation exists.
- The withdrawal basis is identifiable.
- Applicable authority and closure rules are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-101-BH-001 | Identify the active allocation and withdrawal basis. | L3-G-101 |
| L4-G-101-BH-002 | Evaluate the withdrawal under applicable transport rules and authority. | L3-G-101 |
| L4-G-101-BH-003 | Record the authorised withdrawal decision. | L3-G-101 |
| L4-G-101-BH-004 | Close the Transport Allocation and prevent further effective use. | L3-G-101 |
| L4-G-101-BH-005 | Preserve the prior allocation and closure history. | L3-G-101 |

## 5. Cross-Domain & Event Behavior
Transport Management remains the canonical owner and publisher of the allocation closure outcome. Exact event names are not fabricated.

## 6. Failure / Rejection / Continuation
- Unauthorised withdrawal shall not close the active allocation.
- Incomplete withdrawal information shall retain the allocation until resolved.
- Historical allocation evidence shall remain intact after closure.

## 7. Audit & Evidence Behavior
Preserve withdrawal request/basis, authority, decision, closure and historical allocation evidence.

## 8. Observable Completion Conditions
The authorised Transport Allocation is closed, is no longer effective, and its complete history remains traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-101 — Transport Allocation Withdrawal to Service Closure  
**Authoritative workflow:** L2-G04 — Transport Management Workflows

## 10. Open Issues / Deferred Design
Technical closure mechanics, persistence, APIs, interfaces and notification delivery remain outside L4.