# Medical History Review to Consultation Readiness

## 1. Specification Metadata

**Specification ID:** L4-G-159  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-159 — Medical History Review to Consultation Readiness  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-159; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall support review of authoritative G06 medical history for an identified healthcare visit and establish consultation readiness without modifying historical clinical facts or performing diagnosis/treatment.

## 3. Required Business Context

- Patient and current healthcare visit are identifiable.
- Relevant medical history can be retrieved where it exists.
- Review is performed under applicable healthcare authorization/confidentiality policy.
- C/E remain owners of student/employee context used for identification.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-159-BH-001 | Identify the patient and current visit and preserve the link to authoritative institutional identity context. | L3-G-159 §§5,9 |
| L4-G-159-BH-002 | Retrieve relevant previous visits, allergies, chronic conditions, vaccinations, prescriptions and referrals where available. | L3-G-159 §§3,9,12 |
| L4-G-159-BH-003 | Preserve historical medical information as immutable during review and do not fabricate missing facts. | L3-G-159 §§9,11–12 |
| L4-G-159-BH-004 | Preserve the distinction between historical facts and the upcoming current clinical assessment; do not create diagnosis, treatment or prescription here. | L3-G-159 §§4,9,12 |
| L4-G-159-BH-005 | When required history review is complete, establish the visit as `Consultation Ready`. | L3-G-159 §§3,9,13,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-159-BH-006 | No dedicated `MedicalHistoryReviewed` event is defined upstream; no event shall be invented. Existing patient/consultation events remain G06-owned and are published only at their authoritative boundaries. | L3-G-159 §§14,17 |

## 6. Failure / Rejection / Continuation

Do not proceed against an invented patient or wrongly associated history. Preserve incomplete/conflicting history for professional review. If the required review cannot complete, do not establish `Consultation Ready` unless authoritative policy permits an alternative path.

## 7. Audit & Evidence Behavior

Preserve patient/visit context, history reviewed, responsible healthcare actor, confidentiality boundary and readiness outcome without unnecessary exposure of medical details.

## 8. Observable Completion Conditions

Completion requires relevant history reviewed or absence confirmed, immutable history preserved, and the visit established as `Consultation Ready`; no diagnosis/treatment has been created by this behavior.

## 9. Traceability & Provenance

**Parent:** L3-G-159 — Medical History Review to Consultation Readiness  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

No canonical history-review checklist, dedicated event or universal definition of clinically relevant history is defined upstream. Technical access/storage implementation is deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
