# Patient Identification to Healthcare Visit Registration

## 1. Specification Metadata

**Specification ID:** L4-G-157  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-157 — Patient Identification to Healthcare Visit Registration  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-157; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall validate an individual against authoritative institutional identity/context, retrieve relevant G06 medical history where available, register a healthcare visit and establish `Consultation Ready` without transferring person-identity ownership to G06.

## 3. Required Business Context

- A person is seeking institutional healthcare.
- Applicable identity source, campus and healthcare eligibility context can be evaluated.
- C remains authoritative for student identity/status; E remains authoritative for employee/personnel context.
- Existing medical history is available where it exists.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-157-BH-001 | Identify the individual and applicable patient category without creating a new canonical person identity in G06. | L3-G-157 §§5,9,12 |
| L4-G-157-BH-002 | Validate institutional identity, campus, Active Status, Emergency Classification and Healthcare Eligibility according to configurable G06 rules. | L3-G-157 §§9,12 |
| L4-G-157-BH-003 | Consume authoritative student context from C and employee/personnel context from E where applicable without changing source ownership. | L3-G-157 §§5,9 |
| L4-G-157-BH-004 | Retrieve relevant existing medical history where available and preserve historical information as immutable. | L3-G-157 §§9,12 |
| L4-G-157-BH-005 | Register the healthcare visit under G06 and associate it with exactly one validated patient. | L3-G-157 §§9,12,16 |
| L4-G-157-BH-006 | Establish the registered visit as `Consultation Ready` and stop before consultation execution. | L3-G-157 §§3,9,13,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-157-BH-007 | On successful visit registration, publish `PatientRegistered`; G06 is the canonical publisher. C and E provide identity/personnel facts but do not publish the G06 registration event. | L3-G-157 §§5,14,16 |

## 6. Failure / Rejection / Continuation

Do not register against invented identity, campus, status or eligibility facts. If normal identity validation cannot complete, hand off to the authoritative registration-exception path. Missing medical history shall not be fabricated.

## 7. Audit & Evidence Behavior

Preserve patient/visit identity, source-domain context consumed, validation results, medical-history retrieval, registration outcome, `Consultation Ready` condition, responsible actor and `PatientRegistered` publication.

## 8. Observable Completion Conditions

Completion requires validated patient context, healthcare visit registration, `Consultation Ready`, immutable history preservation and `PatientRegistered` published by G06.

## 9. Traceability & Provenance

**Parent:** L3-G-157 — Patient Identification to Healthcare Visit Registration  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Healthcare eligibility matrices, exceptional identification evidence and patient categories remain configurable/upstream governed. Technical identity matching, IAM, APIs, persistence and UI are deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
