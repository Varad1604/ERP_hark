# Emergency Case to External Referral

## 1. Specification Metadata

**Specification ID:** L4-G-163  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-163 — Emergency Case to External Referral  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-163; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish and govern the institutional G06 Referral outcome when an Emergency Case requires care beyond institutional capability, while keeping external hospital treatment outside G06 ownership.

## 3. Required Business Context

- An Emergency Case exists and requires external healthcare after assessment.
- Patient context is sufficiently established.
- Referral reason and intended external healthcare provider/destination can be established.
- Referral lifecycle is `Recommended → Accepted → Completed`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-163-BH-001 | Confirm the Emergency Case, patient, emergency assessment and need for external healthcare. | L3-G-163 §§3,7–9 |
| L4-G-163-BH-002 | Establish referral reason and external provider/destination without claiming ownership of external care. | L3-G-163 §§4–5,9,12 |
| L4-G-163-BH-003 | Create/position the G06 Referral at `Recommended` and preserve its association with the originating Emergency Case/Medical Encounter. | L3-G-163 §§9,13 |
| L4-G-163-BH-004 | Advance the Referral to `Accepted` only when acceptance is authoritatively established. | L3-G-163 §§9–11,13 |
| L4-G-163-BH-005 | Record `Completed` only when the institutional referral handoff/completion condition is satisfied; do not equate it with completion of external hospital treatment. | L3-G-163 §§3,9,13,16 |
| L4-G-163-BH-006 | Preserve patient stabilization, referral decision, provider, status and institutional case-closure evidence. | L3-G-163 §§9,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-163-BH-007 | The exact canonical external-referral event name is not exposed in the Locked parent. No new referral event shall be invented. G06 remains the canonical owner/publisher of Emergency Case and Referral facts; H may only deliver configured notifications. | L3-G-163 §§14,17 |

## 6. Failure / Rejection / Continuation

If provider acceptance, patient context or handoff evidence is unavailable, retain the referral at the appropriate unresolved lifecycle position. If reassessment removes the need for referral, preserve the emergency outcome without fabricating referral completion.

## 7. Audit & Evidence Behavior

Preserve Emergency Case, patient, assessment, immediate treatment, referral decision/reason, external provider, referral lifecycle, case closure and responsible professional.

## 8. Observable Completion Conditions

Completion occurs when the institutional referral responsibility reaches its authoritative completion boundary with traceability preserved; external hospital treatment remains outside G06.

## 9. Traceability & Provenance

**Parent:** L3-G-163 — Emergency Case to External Referral  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Canonical referral-event name, provider-selection criteria and mandatory acceptance evidence remain unresolved upstream and are not invented here. External hospital workflows and technical communication are deferred/outside scope.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
