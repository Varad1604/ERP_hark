# Compliance Failure to Controlled Infrastructure Restriction

## 1. Specification Metadata
**Specification ID:** L4-G-302  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-302  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-302_Compliance_Failure_to_Controlled_Infrastructure_Restriction.md`; L2-G10 — Infrastructure Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall evaluate compliance failure under G10 policy and apply only source-supported restriction consequences without inventing Compliance states or automatically deactivating Infrastructure.

## 3. Required Business Context
The canonical Compliance Record, affected Infrastructure, compliance condition, applicable policy/restriction rule and authorization shall be available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-302-BH-001 | On source-defined expiry, the system shall use the canonical `Expired` Compliance lifecycle position; a separate `Invalid` lifecycle state shall not be created. | L3-G-302 §§12–13 |
| L4-G-302-BH-002 | A non-expiry invalid compliance condition shall be represented as a business condition only unless an authoritative lifecycle state exists. | L3-G-302 §§12–13,17 |
| L4-G-302-BH-003 | The system shall evaluate configured compliance policy before applying any Infrastructure restriction or review consequence. | L3-G-302 §§9,12 |
| L4-G-302-BH-004 | Compliance failure shall not automatically disable or change Infrastructure lifecycle unless a separate source-defined rule authorizes that consequence. | L3-G-302 §§3–4,9,12–13 |
| L4-G-302-BH-005 | Where policy requires a controlled availability/restriction change, the system shall use the applicable authoritative G10 availability mechanism rather than inventing a compliance restriction state. | L3-G-302 §§9,12–14 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-302-BH-006 | On compliance expiry, the system shall publish `ComplianceExpired`; the system shall not invent a generic invalid-compliance or restriction event. | L3-G-302 §§12,14,17 |

## 6. Failure / Rejection / Continuation
If the compliance condition, policy or restriction authority cannot be established, the system shall not claim a restriction outcome and shall preserve the prior authoritative Infrastructure condition pending governed resolution.

## 7. Audit & Evidence Behavior
The system shall preserve the compliance condition, policy basis, resulting review/restriction decision, actor/authority and any source-defined event.

## 8. Observable Completion Conditions
The compliance failure is represented using source-defined semantics; any restriction is policy-supported; no invented state/event exists; Infrastructure lifecycle is changed only when independently authorized.

## 9. Traceability & Provenance
**Parent:** L3-G-302 — Compliance Failure to Controlled Infrastructure Restriction  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows

## 10. Open Issues / Deferred Design
The parent explicitly distinguishes non-expiry invalid conditions from lifecycle states. L4 preserves that distinction.
