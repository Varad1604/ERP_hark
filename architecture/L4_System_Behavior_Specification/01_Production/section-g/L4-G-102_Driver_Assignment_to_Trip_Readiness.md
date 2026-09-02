# Driver Assignment to Trip Readiness

## 1. Specification Metadata

**Specification ID:** L4-G-102  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-102 — Driver Assignment to Trip Readiness  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-102; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern assignment of an eligible driver to a transport Trip so that the Trip is ready from the driver-assignment perspective while preserving Section E ownership of driver/personnel context.

## 3. Required Business Context

- A Trip requiring driver assignment exists within Transport Management.
- Authoritative driver/personnel context is available from People & Workforce.
- Applicable operational driver eligibility and availability conditions can be evaluated.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-102-BH-001 | Identify the Trip requiring an operational driver assignment. | L3-G-102 §§7–9 |
| L4-G-102-BH-002 | Use authoritative People & Workforce driver/personnel context to evaluate candidate eligibility and availability without re-owning that context. | L3-G-102 §§5, 7, 9, 12 |
| L4-G-102-BH-003 | Establish the operational Driver Assignment only for a candidate satisfying the applicable transport conditions. | L3-G-102 §§9, 12, 16 |
| L4-G-102-BH-004 | Preserve assignment responsibility, status and history under Transport Management ownership. | L3-G-102 §§3, 5, 12, 15–16 |
| L4-G-102-BH-005 | Mark the driver-assignment readiness contribution complete only when the required assignment is valid and established. | L3-G-102 §§3, 9, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-102-BH-006 | People & Workforce remains the authoritative owner of driver/personnel records; Transport Management owns only the operational Driver Assignment. | L3-G-102 §§5, 14, 16 |
| L4-G-102-BH-007 | The exact canonical Driver Assignment event identifier is unresolved upstream; this specification shall not invent one. | L3-G-102 §§14, 17 |

## 6. Failure / Rejection / Continuation

- An ineligible or unavailable driver shall not be assigned.
- If no suitable driver exists, the Trip shall remain not ready from the driver-assignment perspective.
- Workforce-record correction shall remain with People & Workforce.

## 7. Audit & Evidence Behavior

Preserve Trip context, authoritative driver context considered, eligibility/availability assessment, assignment outcome, operational responsibility and readiness evidence.

## 8. Observable Completion Conditions

An eligible driver is operationally assigned, assignment history is preserved, the Trip is ready from the driver-assignment perspective, and People & Workforce ownership remains unchanged.

## 9. Traceability & Provenance

**Parent:** L3-G-102 — Driver Assignment to Trip Readiness  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Exact Driver Assignment lifecycle state labels and event identifier remain unresolved upstream and are deliberately not invented. Technical implementation is deferred to later architecture levels.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
