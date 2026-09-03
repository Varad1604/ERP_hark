# Finalize Examination Registration Population for Downstream Examination Processing

## 1. Specification Metadata

**Specification ID:** L4-I-036  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-036 — Finalize Examination Registration Population for Downstream Examination Processing  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-036; L2-I04 Examination Registration; L2-I02 Examination Session; L2-I03 Examination Planning & Scheduling  
**Referenced By:** Hall Ticket, examination conduct, evaluation, and result workflows

## 2. Behavioral Objective & Boundary

Govern establishment of the authoritative population of valid Examination Registrations for downstream examination processing. Population finalization is a business outcome of existing I04 lifecycle governance, not a new registration state or event.

## 3. Required Business Context

- Applicable Examination Session and Published Examination Schedule.
- All registrations relevant to the examination population.
- Current I04 lifecycle position and eligibility/approval status for each registration.
- Applicable locking, amendment, withdrawal, and exception rules.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-036-BH-001 | Identify all registrations relevant to the applicable examination population. | L3-I-036 §§3,7–9 |
| L4-I-036-BH-002 | Evaluate each registration using its current authoritative I04 lifecycle position and eligibility/approval outcome. | L3-I-036 §§9,11–13 |
| L4-I-036-BH-003 | Exclude rejected, withdrawn, unresolved, or otherwise non-valid registrations from downstream participation. | L3-I-036 §§3,9,11–12 |
| L4-I-036-BH-004 | Resolve permitted amendments and approvals before finalization and apply registration locking where required. | L3-I-036 §§9–13,15 |
| L4-I-036-BH-005 | Establish and preserve the resulting authoritative population and its inclusion/exclusion basis. | L3-I-036 §§3,9,15–16 |
| L4-I-036-BH-006 | Make the final valid registration population available to downstream Examination workflows without transferring registration ownership. | L3-I-036 §§3,5,14,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-036-BH-007 | Where registration locking occurs, Examination Registration shall publish the canonical `Registration Locked` event. Other I04 events, including `Registration Confirmed`, `Registration Withdrawn`, and `Registration Amended`, remain tied only to their actual lifecycle transitions. No population-finalization event is invented. | L3-I-036 §§14,16; L2-I04 |
| L4-I-036-BH-008 | The final population is an authoritative business hand-off to downstream examination workflows; those consumers shall not redefine registration validity or ownership. | L3-I-036 §§5,12,14,16 |

## 6. Failure / Rejection / Continuation

- A registration whose current validity cannot be established shall not be silently included.
- Duplicate/conflicting registrations shall be resolved before inclusion.
- An authoritative Student Lifecycle conflict remains with its canonical owner and prevents unsupported final inclusion.
- Post-lock changes shall use the existing authorised exceptional path.
- Downstream consumer readiness shall not alter the registration lifecycle or finalization authority.

## 7. Audit & Evidence Behavior

Preserve the population boundary, registrations evaluated, lifecycle positions, inclusion/exclusion basis, locking and exceptional decisions, responsible authority, and final authoritative population.

## 8. Observable Completion Conditions

Completion occurs when all relevant registrations have been evaluated, only valid registration authority remains in the final population, required locking has occurred with `Registration Locked` published where applicable, audit evidence is complete, and downstream workflows can consume the authoritative population.

## 9. Traceability & Provenance

**Parent:** L3-I-036 — Finalize Examination Registration Population for Downstream Examination Processing  
**Primary verification source:** L2-I04 — Examination Registration Workflows.

## 10. Open Issues / Deferred Design

The exact institutional locking/finalization point remains policy-governed. No `Finalized Population` registration state or dedicated population event is introduced. Technical batch, persistence, transport, or orchestration mechanisms remain outside L4.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to the governed L4 System Behavior Specification template and is not self-locked.
