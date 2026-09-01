# L4-G-120 — Transport Complaint to Operational Resolution

## 1. Specification Metadata
**Specification ID:** L4-G-120  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-120 — Transport Complaint to Operational Resolution  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern a transport complaint through review and operational resolution.

**Start:** A transport complaint is submitted.  
**End:** The complaint reaches its governed operational resolution.

## 3. Required Business Context
Complaint subject, affected transport context, applicable service rules and resolution authority shall be available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-120-BH-001 | Register the complaint against the applicable transport context. | L3-G-120 |
| L4-G-120-BH-002 | Preserve evidence required for complaint review. | L3-G-120 |
| L4-G-120-BH-003 | Apply the governed operational review and resolution path. | L3-G-120 |
| L4-G-120-BH-004 | Record the authorised operational resolution. | L3-G-120 |
| L4-G-120-BH-005 | Preserve complaint and resolution history. | L3-G-120 |

## 5. Cross-Domain & Event Behavior
Transport Management owns the operational complaint outcome. Other domains remain owners of their records. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
Incomplete evidence shall prevent final resolution. Escalation shall retain the complaint in its governed review position until resolved.

## 7. Audit & Evidence Behavior
Preserve complaint, evidence, review, authority, decision and resolution history.

## 8. Observable Completion Conditions
A governed operational resolution is recorded and all material complaint history is traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-120 — Transport Complaint to Operational Resolution

## 10. Open Issues / Deferred Design
Technical case management, persistence, notification and interfaces remain outside L4.