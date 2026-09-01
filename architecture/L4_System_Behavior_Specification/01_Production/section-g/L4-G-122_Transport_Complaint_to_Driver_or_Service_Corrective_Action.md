# L4-G-122 — Transport Complaint to Driver or Service Corrective Action

## 1. Specification Metadata
**Specification ID:** L4-G-122  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-122 — Transport Complaint to Driver or Service Corrective Action  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern a transport complaint through assessment and an authorised corrective action concerning driver conduct or transport service.

**Start:** A transport complaint identifies a possible driver/service deficiency.  
**End:** The governed corrective action or supported alternative resolution is recorded.

## 3. Required Business Context
Complaint, affected transport context, authoritative driver/personnel context and corrective-action authority shall be available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-122-BH-001 | Register the complaint and affected transport context. | L3-G-122 |
| L4-G-122-BH-002 | Preserve evidence required to assess the alleged driver or service deficiency. | L3-G-122 |
| L4-G-122-BH-003 | Evaluate the complaint through the governed review path. | L3-G-122 |
| L4-G-122-BH-004 | Record an authorised corrective action where the review establishes that action is warranted. | L3-G-122 |
| L4-G-122-BH-005 | Preserve the complaint, decision, corrective-action and hand-off history. | L3-G-122 |

## 5. Cross-Domain & Event Behavior
Transport Management owns the transport complaint outcome. Driver/personnel records remain owned by Section E; corrective-action participation does not transfer workforce ownership. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
Insufficient evidence shall prevent unsupported corrective action. If the complaint is not substantiated, the governed non-corrective resolution shall be recorded. Required authority must exist before corrective action is finalised.

## 7. Audit & Evidence Behavior
Preserve complaint evidence, assessment, authority, decision, corrective action and cross-domain hand-off evidence.

## 8. Observable Completion Conditions
The complaint reaches a governed resolution, any authorised corrective action is recorded, and relevant history remains traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-122 — Transport Complaint to Driver or Service Corrective Action

## 10. Open Issues / Deferred Design
Technical HR case handling, persistence, notification, interfaces and implementation mechanics remain outside L4.