# L4-G-128 — Catalogue Update to Effective Resource Information

## 1. Specification Metadata
**Specification ID:** L4-G-128  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-128 — Catalogue Update to Effective Resource Information  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern an authorised catalogue update through validation and effective adoption while preserving the prior authoritative resource information.

**Start:** A catalogue update is proposed.  
**End:** The validated update becomes effective catalogue information or reaches a governed rejection outcome.

## 3. Required Business Context
- An authoritative catalogue record exists.
- Applicable cataloguing governance is available.
- Library Management owns catalogue information.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-128-BH-001 | Register the proposed catalogue update against the authoritative catalogue record. | L3-G-128 |
| L4-G-128-BH-002 | Validate changed information against applicable cataloguing rules. | L3-G-128 |
| L4-G-128-BH-003 | Apply the governed review and authorisation path. | L3-G-128 |
| L4-G-128-BH-004 | On approval, establish the changed information as effective catalogue information. | L3-G-128 |
| L4-G-128-BH-005 | Preserve prior information and update history for traceability. | L3-G-128 |

## 5. Cross-Domain & Event Behavior
The effective catalogue update shall be available to authorised library consumers without transferring Library Management ownership. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
- Invalid updates shall not become effective.
- Missing authorisation shall retain the update in its applicable governance position.
- Effective changes shall preserve prior historical information.

## 7. Audit & Evidence Behavior
Preserve proposal, validation, authority, effective outcome, and historical versions required for traceability.

## 8. Observable Completion Conditions
Completion occurs when the authorised catalogue update is effective and historical traceability is preserved, or a governed terminal rejection is recorded.

## 9. Traceability & Provenance
**Parent:** L3-G-128 — Catalogue Update to Effective Resource Information

## 10. Open Issues / Deferred Design
Technical storage, indexing, interfaces, and search mechanisms remain outside L4.