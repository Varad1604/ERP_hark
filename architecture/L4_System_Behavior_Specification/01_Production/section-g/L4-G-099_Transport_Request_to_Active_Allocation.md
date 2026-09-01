# L4-G-099 — Transport Request to Active Allocation

## 1. Specification Metadata
**Specification ID:** L4-G-099  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-099 — Transport Request to Active Allocation  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-099; L2-G04 Transport Management Workflows

## 2. Behavioral Objective & Boundary
SutraERP shall govern a transport request through eligibility, allocation and activation of the authorised transport allocation.

**Start:** A transport request is submitted for an eligible subject.  
**End:** A valid Transport Allocation is Active.

## 3. Required Business Context
- The requesting subject and transport context are authoritative.
- An eligible route/stop context is available.
- Applicable allocation rules and authority are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-099-BH-001 | Register the transport request against its authoritative subject and operational context. | L3-G-099 |
| L4-G-099-BH-002 | Evaluate the request against applicable transport eligibility and allocation rules. | L3-G-099 |
| L4-G-099-BH-003 | Identify an eligible transport allocation context. | L3-G-099 |
| L4-G-099-BH-004 | Complete required approval/authorization before activation. | L3-G-099 |
| L4-G-099-BH-005 | Establish the authorised Transport Allocation as Active. | L3-G-099 |
| L4-G-099-BH-006 | Preserve allocation evidence and history. | L3-G-099 |

## 5. Cross-Domain & Event Behavior
Transport Management owns the Transport Allocation outcome. Exact event names are not fabricated where absent from authoritative source material.

## 6. Failure / Rejection / Continuation
- Ineligible requests shall not receive an active allocation.
- No available compatible allocation shall prevent activation until a valid business resolution exists.
- Failed approval shall retain the request in its applicable non-active position.

## 7. Audit & Evidence Behavior
Preserve request, eligibility assessment, allocation decision, authorization and active allocation evidence.

## 8. Observable Completion Conditions
An authorised valid Transport Allocation is Active and traceable to the originating transport request.

## 9. Traceability & Provenance
**Parent:** L3-G-099 — Transport Request to Active Allocation  
**Authoritative workflow:** L2-G04 — Transport Management Workflows

## 10. Open Issues / Deferred Design
Technical matching, persistence, APIs, notification and implementation mechanics remain outside L4.