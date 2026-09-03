# Duplicate Examination Registration to Single Valid Registration

## 1. Specification Metadata

**Specification ID:** L4-I-032  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-032 — Duplicate Examination Registration to Single Valid Registration  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-032; L2-I04 Examination Registration; applicable authoritative candidate, academic, and financial context  
**Referenced By:** Downstream examination workflows consuming authoritative registration

## 2. Behavioral Objective & Boundary

Govern duplicate Examination Registrations so one valid registration remains authoritative for the affected candidate and examination scope while duplicate history remains traceable. Duplicate resolution does not create a new registration lifecycle or event taxonomy.

## 3. Required Business Context

- Candidate and Examination Session context.
- All potentially duplicate Examination Registrations and item/category scope.
- Authoritative academic and financial facts where relevant.
- Existing I04 lifecycle positions and permitted dispositions.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-032-BH-001 | Identify registrations that may represent the same candidate and examination participation scope. | L3-I-032 §§3,7–9 |
| L4-I-032-BH-002 | Distinguish legitimate distinct registration items/categories from true duplicate participation. | L3-I-032 §§9–12 |
| L4-I-032-BH-003 | Establish the valid authoritative registration from business evidence; never select an arbitrary survivor. | L3-I-032 §§9,11–12 |
| L4-I-032-BH-004 | Apply only an existing I04 authorised disposition to the duplicate registration. | L3-I-032 §§9,12–13 |
| L4-I-032-BH-005 | Preserve the valid registration, duplicate history, decision basis, and downstream single-authority position. | L3-I-032 §§3,9,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-032-BH-006 | Examination Registration shall publish only the canonical I04 event corresponding to the actual governed disposition or milestone, such as `Registration Amended`, `Registration Withdrawn`, `Registration Confirmed`, or other applicable I04 lifecycle event. No `DuplicateRegistrationResolved` event is created. | L3-I-032 §§14,16; L2-I04 |
| L4-I-032-BH-007 | Downstream examination workflows shall consume only the single authoritative registration position; source domains retain ownership of their candidate, academic, and financial facts. | L3-I-032 §§5,14,16 |

## 6. Failure / Rejection / Continuation

- If the valid registration cannot be determined, the duplicate condition remains unresolved pending authorised review.
- Conflicting upstream facts shall be resolved by their canonical owners before Examination invents a registration resolution.
- Post-confirmation or post-lock duplicate disposition shall follow the existing authorised I04 path.
- Historical duplicates shall not be silently deleted or converted into independent valid participation.

## 7. Audit & Evidence Behavior

Preserve all registrations considered, duplicate-detection basis, source facts, decision authority, selected authoritative registration, disposition applied to duplicates, and historical evidence.

## 8. Observable Completion Conditions

Completion occurs when exactly one registration is authoritative for the affected participation scope, any duplicate has an existing authorised lifecycle disposition, applicable I04 lifecycle events have been published, and downstream workflows cannot treat a duplicate as separate valid authority.

## 9. Traceability & Provenance

**Parent:** L3-I-032 — Duplicate Examination Registration to Single Valid Registration  
**Primary verification source:** L2-I04 — Examination Registration Workflows.

## 10. Open Issues / Deferred Design

I04 defines no duplicate-specific lifecycle state or event. Resolution therefore remains expressed through existing registration states, dispositions, and events. Technical duplicate-detection and persistence mechanisms are outside L4.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to the governed L4 System Behavior Specification template and is not self-locked.
