# Emergency Case to External Referral

## 1. Specification Metadata

**Specification ID:** L4-G-163  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-163 — Emergency Case to External Referral  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-163; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

the system shall govern the institutional healthcare behavior that establishes an external referral when an Emergency Case requires care beyond institutional capability. The behavior begins from an assessed emergency requiring external care and ends when the institutional referral outcome is truthfully established. External hospital care remains outside G06 ownership.

## 3. Required Business Context

- Emergency Case and patient context are authoritative G06 facts.
- External referral is a G06-owned institutional outcome.
- Referral reason and external healthcare provider are material referral facts.
- Referral lifecycle is `Recommended → Accepted → Completed`.
- C may provide student identity context; E may provide professional context; H may support configured notifications.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-163-BH-001 | When an assessed Emergency Case requires care beyond institutional scope, the system shall establish the referral requirement and preserve the associated patient and Emergency Case context. | L3-G-163 §§3, 8, 9 |
| L4-G-163-BH-002 | When an external referral is required, the system shall record the referral reason and identified external healthcare provider without transferring ownership of external care to G06. | L3-G-163 §§4, 5, 12 |
| L4-G-163-BH-003 | When the referral is established, the system shall position the G06 Referral at `Recommended`. | L3-G-163 §§9, 13 |
| L4-G-163-BH-004 | When referral acceptance is established, the system shall preserve the acceptance outcome and advance the Referral only according to its authoritative lifecycle. | L3-G-163 §§9, 13 |
| L4-G-163-BH-005 | When the institutional referral completion condition is satisfied, the system shall record completion without representing external hospital treatment as a G06 outcome. | L3-G-163 §§3, 13, 16 |
| L4-G-163-BH-006 | The system shall preserve traceability between Emergency Case, patient, Referral, referral reason, provider and resulting institutional outcome. | L3-G-163 §§3, 15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-163-BH-007 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Institutional Health Services (G06) ownership. | L3-G-163 §14 |

## 6. Failure / Rejection / Continuation

- If provider information is insufficient, the system shall prevent completion against an unsupported provider fact.
- If a provider cannot accept the referral, the system shall retain the unresolved referral outcome and permit an authorised alternative provider path.
- If patient identity cannot be authoritatively established, the system shall not attach the referral to an invented patient.
- If referral handoff is not confirmed, the system shall not represent the institutional referral as completed.
- If reassessment establishes that referral is unnecessary, the system shall preserve the applicable emergency outcome without creating an unsupported referral completion.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of Emergency Case identity, patient context, emergency assessment/treatment outcome, referral decision, referral reason, external provider, Referral lifecycle position, responsible healthcare professional and institutional closure outcome.

## 8. Observable Completion Conditions

Completion is observable when the required external referral is established in its authoritative G06 lifecycle and the institutional referral responsibility is complete, with traceability preserved. External hospital treatment shall not be represented as completed by G06.

## 9. Traceability & Provenance

**Parent:** L3-G-163 — Emergency Case to External Referral  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
