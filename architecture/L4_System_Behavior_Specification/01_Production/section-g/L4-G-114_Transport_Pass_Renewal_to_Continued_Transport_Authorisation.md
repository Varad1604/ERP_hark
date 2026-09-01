# L4-G-114 — Transport Pass Renewal to Continued Transport Authorisation

## 1. Specification Metadata
**Specification ID:** L4-G-114  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-114 — Transport Pass Renewal to Continued Transport Authorisation  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern renewal of an existing Transport Pass so that eligible transport authorisation continues without creating an ownership change.

**Start:** An active or renewable Transport Pass approaches its governed renewal point.  
**End:** Renewal is authorised and the pass continues in its governed Active authorisation.

## 3. Required Business Context
- Existing Transport Pass is identifiable.
- Renewal eligibility and validity policy are authoritative.
- Transport Management owns the pass lifecycle.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-114-BH-001 | Identify the existing Transport Pass and its renewal context. | L3-G-114 |
| L4-G-114-BH-002 | Evaluate continued eligibility and applicable renewal conditions. | L3-G-114 |
| L4-G-114-BH-003 | Record the governed renewal decision. | L3-G-114 |
| L4-G-114-BH-004 | Extend or establish the governed validity period when renewal is authorised. | L3-G-114 |
| L4-G-114-BH-005 | Preserve prior validity and renewal history. | L3-G-114 |

## 5. Cross-Domain & Event Behavior
Transport Management remains owner of Transport Pass. Authoritative student/person context remains with its owning domains. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
Failed eligibility or approval shall prevent continued authorisation and record the applicable non-renewal outcome. Missing authoritative context shall prevent final renewal.

## 7. Audit & Evidence Behavior
Preserve renewal request, eligibility assessment, authority, validity change and prior-pass history.

## 8. Observable Completion Conditions
The renewal decision is recorded, authorised validity is established where approved, and historical validity remains traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-114 — Transport Pass Renewal to Continued Transport Authorisation

## 10. Open Issues / Deferred Design
Technical persistence, interfaces, notifications and implementation mechanics remain outside L4.