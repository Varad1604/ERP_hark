# Conflicting Registration Scope to Reconciled Registration Position

## 1. Specification Metadata

**Specification ID:** L4-I-033  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-033 — Conflicting Registration Scope to Reconciled Registration Position  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-033; L2-I04 Examination Registration; authoritative Student Lifecycle, Academic Delivery, and Finance context  
**Referenced By:** Downstream examination workflows consuming reconciled registration

## 2. Behavioral Objective & Boundary

Govern conflicting registration scope so one policy-valid Examination Registration position becomes authoritative while every participating domain retains ownership of its own source facts.

## 3. Required Business Context

- Existing Examination Registration and affected candidate/session context.
- Each conflicting registration-scope representation.
- Authoritative candidate, academic/curriculum, examination-policy, and Finance facts as applicable.
- Existing I04 lifecycle and approval constraints.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-033-BH-001 | Identify each conflicting registration-scope position and its canonical source. | L3-I-033 §§3,7–9 |
| L4-I-033-BH-002 | Distinguish legitimate multi-item/category scope from a genuine inconsistency. | L3-I-033 §§9–12 |
| L4-I-033-BH-003 | Resolve authoritative upstream facts through their owning domains and determine the resulting registration consequence. | L3-I-033 §§5,9,11–12 |
| L4-I-033-BH-004 | Apply only existing I04 amendment, withdrawal, confirmation, or other authorised registration transitions. | L3-I-033 §§9,12–13 |
| L4-I-033-BH-005 | Preserve conflicting inputs, decision authority, prior position, reconciliation rationale, and resulting registration. | L3-I-033 §§9,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-033-BH-006 | Examination Registration shall publish the canonical I04 event corresponding to the actual reconciliation transition, including `Registration Amended`, `Registration Withdrawn`, or `Registration Confirmed` where that milestone actually occurs. No `RegistrationScopeConflictResolved` event is introduced. | L3-I-033 §§14,16; L2-I04 |
| L4-I-033-BH-007 | Student Lifecycle, Academic Delivery/Curriculum, and Finance shall remain authoritative for their own facts; Examination owns only the registration consequence. | L3-I-033 §§5,12,14–15 |

## 6. Failure / Rejection / Continuation

- If authoritative source facts do not establish one valid scope, the registration shall remain unresolved rather than adopting a compromise.
- A locked registration shall be changed only through an authorised exceptional path.
- Hall Ticket or other downstream consequences already created remain governed by their owning workflows.

## 7. Audit & Evidence Behavior

Preserve all conflicting source positions, source ownership, the applicable business rule, decision authority, lifecycle transition, prior registration position, and resulting authoritative scope.

## 8. Observable Completion Conditions

Completion occurs when a legitimate variation is retained or a genuine conflict is reconciled into one authoritative registration using existing I04 transitions, applicable canonical I04 events are published, and downstream workflows have one current registration position.

## 9. Traceability & Provenance

**Parent:** L3-I-033 — Conflicting Registration Scope to Reconciled Registration Position  
**Primary verification source:** L2-I04 — Examination Registration Workflows.

## 10. Open Issues / Deferred Design

No `Conflict` lifecycle state or conflict-resolution event exists in the locked parent architecture. Technical reconciliation mechanisms are outside this L4 specification.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to the governed L4 System Behavior Specification template and is not self-locked.
