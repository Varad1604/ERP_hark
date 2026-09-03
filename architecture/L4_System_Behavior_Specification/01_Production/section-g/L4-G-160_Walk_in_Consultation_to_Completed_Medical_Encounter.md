# Walk in Consultation to Completed Medical Encounter

## 1. Specification Metadata

**Specification ID:** L4-G-160  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-160 — Walk-in Consultation to Completed Medical Encounter  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-160; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall support the G06 walk-in consultation from a consultation-ready registered patient through clinical assessment, diagnosis, treatment and authoritative consultation closure while leaving professional clinical judgement with the authorised medical professional.

## 3. Required Business Context

- Patient and Medical Encounter are authoritative and consultation-ready.
- An authorised healthcare professional is established from E-owned personnel context.
- Relevant medical history and configurable consultation policy are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-160-BH-001 | Identify the patient, Medical Encounter, responsible healthcare professional and walk-in consultation context. | L3-G-160 §§7–9 |
| L4-G-160-BH-002 | Record clinical assessment, symptoms/context, diagnosis and treatment as established by authorised professional judgement. | L3-G-160 §§9,12,15 |
| L4-G-160-BH-003 | Determine whether prescription, follow-up, referral or observation is required and hand off each resulting outcome to its authoritative G06 workflow. | L3-G-160 §§9–10,12 |
| L4-G-160-BH-004 | Do not represent medicine dispensing, referral completion or observation completion as part of the consultation lifecycle. | L3-G-160 §§4,9–10 |
| L4-G-160-BH-005 | When required consultation information is established, complete/close the consultation according to the Medical Encounter lifecycle. | L3-G-160 §§9,13,16 |
| L4-G-160-BH-006 | Preserve the complete auditable Medical Encounter history and relevant downstream handoffs. | L3-G-160 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-160-BH-007 | Publish `ConsultationStarted` at consultation initiation and `ConsultationCompleted` at authoritative completion. Publish `FollowUpRecommended` only when follow-up is actually advised. G06 is the canonical publisher; H owns notification delivery. | L3-G-160 §§9,14,16 |

## 6. Failure / Rejection / Continuation

Do not close the encounter against an invented patient, professional, diagnosis, treatment or incomplete clinical record. If emergency handling becomes applicable, hand off to the emergency workflow instead of continuing an ordinary consultation incorrectly.

## 7. Audit & Evidence Behavior

Preserve patient/encounter context, professional, consultation type, symptoms, assessment, diagnosis, treatment, follow-up/downstream outcomes, lifecycle transitions and canonical events.

## 8. Observable Completion Conditions

Completion requires authoritative consultation closure, `ConsultationCompleted` published by G06, audit history preserved and any prescription/follow-up/referral/observation outcomes handed to their own workflows.

## 9. Traceability & Provenance

**Parent:** L3-G-160 — Walk-in Consultation to Completed Medical Encounter  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Consultation types, follow-up policy and detailed clinical assessment/diagnosis/treatment rules remain configurable/professional rather than invented at L4. Technical implementation is deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
