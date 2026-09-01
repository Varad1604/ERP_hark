# L4-G-115 — Lost or Damaged Transport Pass to Replacement Pass

## 1. Specification Metadata
**Specification ID:** L4-G-115  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-115 — Lost or Damaged Transport Pass to Replacement Pass  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern a lost or damaged Transport Pass through validation and replacement issuance.

**Start:** Loss or damage of an active pass is reported.  
**End:** A governed replacement pass is issued or a supported rejection is recorded.

## 3. Required Business Context
The existing pass, holder context, validity and applicable replacement policy shall be authoritative.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-115-BH-001 | Register the loss or damage against the authoritative Transport Pass. | L3-G-115 |
| L4-G-115-BH-002 | Validate the report and applicable replacement eligibility. | L3-G-115 |
| L4-G-115-BH-003 | Restrict or supersede the affected pass as required by the governed lifecycle. | L3-G-115 |
| L4-G-115-BH-004 | Issue the replacement pass when authorised. | L3-G-115 |
| L4-G-115-BH-005 | Preserve the original and replacement pass history. | L3-G-115 |

## 5. Cross-Domain & Event Behavior
Transport Management owns the Transport Pass lifecycle. No unsupported event name is introduced.

## 6. Failure / Rejection / Continuation
Unverified loss/damage or failed eligibility shall prevent replacement issuance. Duplicate replacement requests shall be governed against the existing pass history.

## 7. Audit & Evidence Behavior
Preserve report, validation, authority, affected-pass status and replacement issuance evidence.

## 8. Observable Completion Conditions
The affected pass outcome and replacement outcome are recorded with complete historical traceability.

## 9. Traceability & Provenance
**Parent:** L3-G-115 — Lost or Damaged Transport Pass to Replacement Pass

## 10. Open Issues / Deferred Design
Technical identity, persistence, interfaces and notifications remain outside L4.