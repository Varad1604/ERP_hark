# Administrative Examination Registration on Behalf of a Candidate

## 1. Specification Metadata

**Specification ID:** L4-I-034  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-034 — Administrative Examination Registration on Behalf of a Candidate  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-034; L2-I04 Examination Registration; applicable Student Lifecycle, Academic Delivery, Finance, authorization and audit architecture  
**Referenced By:** Downstream examination workflows

## 2. Behavioral Objective & Boundary

Govern an authorised institutional administrator completing Examination Registration on behalf of a candidate without changing the candidate's participation ownership or creating a separate administrative-registration lifecycle.

## 3. Required Business Context

- Active Examination Session and Published Examination Schedule.
- Authoritative candidate, academic, and applicable Finance context.
- Administrator authority under institutional policy.
- Applicable category, subjects, eligibility, window, review, and approval conditions.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-034-BH-001 | Establish candidate identity and the administrator's business authority to act on the candidate's behalf. | L3-I-034 §§7–9,15 |
| L4-I-034-BH-002 | Apply the same authoritative eligibility, subject, category, registration-window, Finance, review, and approval conditions required by I04. | L3-I-034 §§9,11–12 |
| L4-I-034-BH-003 | Establish the Examination Registration for the candidate using the existing I04 lifecycle. | L3-I-034 §§3,9,13 |
| L4-I-034-BH-004 | Preserve the administrative actor, authority basis, candidate participation, and registration history. | L3-I-034 §§9,12,15–16 |
| L4-I-034-BH-005 | Require candidate confirmation where the governing institutional policy requires it. | L3-I-034 §§10,15 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-034-BH-006 | Examination Registration shall publish the normal canonical I04 lifecycle events at their actual milestones, including `Registration Confirmed` when confirmation is established. No `AdministrativeRegistrationCompleted` event is created. | L3-I-034 §§14,16; L2-I04 |
| L4-I-034-BH-007 | Student Lifecycle, Academic Delivery/Curriculum, Finance, and Platform authorization/audit capabilities retain ownership of their respective facts and capabilities. | L3-I-034 §§5,12,14–15 |

## 6. Failure / Rejection / Continuation

- Unauthorised administrative action shall not establish registration.
- Administrative origin shall not bypass candidate eligibility, Finance requirements, registration windows, or approvals.
- Conflicting candidate/academic facts remain with their canonical owners for resolution.
- A locked registration shall use the existing exceptional correction path.
- Duplicate registration shall use the governed duplicate-resolution path rather than create a second valid participation record.

## 7. Audit & Evidence Behavior

Preserve candidate identity, administrative actor, delegation/authority basis, eligibility evaluation, source facts, review/approval, confirmation or rejection, and registration lifecycle history.

## 8. Observable Completion Conditions

Completion occurs when an authorised administrative action establishes a valid Confirmed Examination Registration for the candidate, normal I04 lifecycle events are published, administrative provenance is traceable, and downstream workflows can consume the registration; or a governed non-confirmed outcome remains.

## 9. Traceability & Provenance

**Parent:** L3-I-034 — Administrative Examination Registration on Behalf of a Candidate  
**Primary verification source:** L2-I04 — Examination Registration Workflows.

## 10. Open Issues / Deferred Design

The exact institutional delegation model remains upstream-policy governed. No new permanent role, lifecycle, state, or administrative-registration event is introduced. Technical access-control and implementation mechanisms remain outside L4.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to the governed L4 System Behavior Specification template and is not self-locked.
