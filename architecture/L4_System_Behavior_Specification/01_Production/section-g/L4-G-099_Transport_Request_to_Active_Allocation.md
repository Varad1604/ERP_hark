# Transport Request to Active Allocation

## 1. Specification Metadata

**Specification ID:** L4-G-099  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-099 — Transport Request to Active Allocation  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-099; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a transport request through eligibility, route/stop selection, required approval, and activation of the authorised Transport Allocation.

## 3. Required Business Context

- Authoritative applicant context is available from Student Lifecycle or People & Workforce where applicable.
- Active route/stop context is available from Transport Management.
- Configured eligibility and approval rules are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-099-BH-001 | Register the transport request against its authoritative applicant and transport context. | L3-G-099 §§7–9 |
| L4-G-099-BH-002 | Evaluate the request using the configured transport eligibility rules and authoritative applicant context. | L3-G-099 §§7, 9, 12 |
| L4-G-099-BH-003 | Select an active Route and applicable Stop before allocation activation. | L3-G-099 §§9, 12, 16 |
| L4-G-099-BH-004 | Complete the configured approval before the allocation becomes Active. | L3-G-099 §§9, 15–16 |
| L4-G-099-BH-005 | Establish the authorised Transport Allocation as Active while retaining Transport Management ownership. | L3-G-099 §§3, 5, 13, 16 |
| L4-G-099-BH-006 | Preserve request, eligibility, route/stop, approval, activation, and allocation history. | L3-G-099 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-099-BH-007 | On completed transport allocation, Transport Management shall publish `TransportAllocated`. | L3-G-099 §§9, 14, 16 |
| L4-G-099-BH-008 | Student Lifecycle and People & Workforce facts used for eligibility shall remain owned by their source domains; Transport Management shall use them without re-owning them. | L3-G-099 §§5, 14 |

## 6. Failure / Rejection / Continuation

- Ineligible requests shall not receive an Active allocation.
- If no active suitable route/stop exists, activation shall not occur until a valid transport context is available.
- If authoritative applicant context is unavailable or approval is rejected, the request shall remain non-active until the parent-supported condition is resolved.

## 7. Audit & Evidence Behavior

Preserve the request, authoritative applicant context used, eligibility assessment, selected route/stop, approval, activation, event publication, and allocation history.

## 8. Observable Completion Conditions

The Transport Allocation is Active, references an active Route and applicable Stop, `TransportAllocated` is published, and the originating request remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-099 — Transport Request to Active Allocation  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Exact institutional eligibility criteria and approval policy remain configuration governed. Technical APIs, persistence, UI, notification delivery, and retry mechanics are deferred to later architecture levels.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
