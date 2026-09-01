# L4-G-113 — Transport Pass Issuance to Active Transport Authorisation

## 1. Specification Metadata
**Specification ID:** L4-G-113  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-113 — Transport Pass Issuance to Active Transport Authorisation  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern an eligible transport pass from issuance through active transport authorisation.

**Start:** An eligible transport requirement is approved for pass issuance.  
**End:** The Transport Pass reaches Active authorisation.

## 3. Required Business Context
- Canonical student/person context is available from its owning domain.
- Transport eligibility and policy are authoritative.
- Transport Management owns the Transport Pass lifecycle.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-113-BH-001 | Establish a Transport Pass request against the applicable authorised transport context. | L3-G-113 |
| L4-G-113-BH-002 | Evaluate eligibility and required approval conditions. | L3-G-113 |
| L4-G-113-BH-003 | Issue the Transport Pass when all authoritative conditions are satisfied. | L3-G-113 |
| L4-G-113-BH-004 | Transition the Transport Pass to Active authorisation. | L3-G-113 |
| L4-G-113-BH-005 | Preserve issuance, authority, validity and lifecycle history. | L3-G-113 |

## 5. Cross-Domain & Event Behavior
Transport Management remains the canonical owner. Participating domains provide authoritative context only; no ownership transfer is introduced. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
Ineligible or incomplete requests shall not become Active. Missing authoritative context shall keep issuance unresolved until corrected.

## 7. Audit & Evidence Behavior
Preserve attributable evidence of eligibility, approval, issuance, validity and activation.

## 8. Observable Completion Conditions
The pass is issued, Active authorisation is recorded, validity is governed, and issuance history is traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-113 — Transport Pass Issuance to Active Transport Authorisation

## 10. Open Issues / Deferred Design
Technical identity, persistence, interfaces, notification and transport mechanisms remain outside L4.