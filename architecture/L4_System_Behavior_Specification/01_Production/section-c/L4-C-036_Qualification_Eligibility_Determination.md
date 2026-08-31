# Qualification Eligibility Determination

## 1. Specification Metadata

**Specification ID:** L4-C-036  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Progression  
**Realizes Scenario:** L3-C-036 — Qualification Eligibility Determination  
**Primary Business Owner:** Academic Progression  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-036; L2-C04 Student Academic Progression Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall determine whether a Student has satisfied the applicable academic requirements to enter the Qualification Completion & Student Exit boundary. The behavior ends with **Qualification Eligible** or **Qualification Pending** and does not perform qualification completion or degree conferment.

## 3. Required Business Context

- Authoritative Academic Progression record.
- Programme credits and academic completion information.
- Mandatory curriculum requirements.
- Institution-specific completion requirements.
- Applicable exit/NEP requirements.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-036-BH-001 | The system shall gather the authoritative programme-credit and academic-completion information required for eligibility evaluation. | L3-C-036 §§3, 8–9 |
| L4-C-036-BH-002 | The system shall verify all mandatory curriculum requirements relevant to qualification eligibility. | L3-C-036 §§4.3, 9, 12 |
| L4-C-036-BH-003 | The system shall verify that minimum programme credits are satisfied. | L3-C-036 §§4.3, 9, 12 |
| L4-C-036-BH-004 | The system shall verify applicable institution-specific completion requirements and exit/NEP requirements. | L3-C-036 §§4.3, 9, 12 |
| L4-C-036-BH-005 | When all applicable requirements are satisfied, the system shall establish **Qualification Eligible**; otherwise it shall retain **Qualification Pending**. | L3-C-036 §§3, 9–10 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-036-BH-006 | When Qualification Eligible is established, the system shall publish the canonical `QualificationEligible` outcome from Academic Progression for downstream C05 processing. | L3-C-036 §§3, 9, 14 |

## 6. Failure / Rejection / Continuation

- If academic completion data is incomplete, the system shall not establish eligibility.
- If any mandatory requirement, minimum credit requirement, institution-specific requirement, or applicable exit/NEP requirement remains unsatisfied, the system shall retain Qualification Pending and identify the outstanding condition where supported.
- Qualification completion shall proceed only after the canonical `QualificationEligible` outcome.

## 7. Audit & Evidence Behavior

The system shall preserve the authoritative academic evidence evaluated, completion requirements considered, resulting eligibility position, outstanding requirements when pending, and publication of `QualificationEligible` when eligible.

## 8. Observable Completion Conditions

Eligibility evaluation is complete when all applicable requirements have been evaluated and the Student is established as Qualification Eligible or Qualification Pending. For eligible Students, `QualificationEligible` must be published. C05 qualification completion is not part of completion.

## 9. Traceability & Provenance

**Parent:** L3-C-036 — Qualification Eligibility Determination  
**Primary verification source:** L2-C04 — Student Academic Progression Workflows.  

No local rule or lifecycle ownership is added beyond the approved L3 parent.

## 10. Open Issues / Deferred Design

The exact institution-specific completion checklist is not enumerated in the L2-C04 source available to the parent scenario. Detailed C05 downstream contract and all technical interface, persistence, transport and access-control design remain deferred.

## Canonical Validation Note

This specification is a Draft L4 artifact and is not Architecture Approved or Locked.
