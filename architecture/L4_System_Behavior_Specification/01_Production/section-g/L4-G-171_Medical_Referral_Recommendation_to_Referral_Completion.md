# Medical Referral Recommendation to Referral Completion

## 1. Specification Metadata

**Specification ID:** L4-G-171  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-171 — Medical Referral Recommendation to Referral Completion  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-171; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall govern the institutional Referral lifecycle from an authorised medical recommendation through applicable provider acceptance and institutional completion. Completion shall represent fulfilment of G06 referral responsibility, not completion of external hospital treatment.

## 3. Required Business Context

- Referral is a G06 aggregate.
- Referral owns referral reason, external healthcare provider, status and history.
- Referral lifecycle is `Recommended → Accepted → Completed`.
- E supplies professional context; H may support notifications.
- External hospital treatment remains outside G06 ownership.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-171-BH-001 | When an authorised medical assessment establishes an external referral requirement, the system shall record the referral reason and external healthcare provider context. | L3-G-171 §§3, 8, 9 |
| L4-G-171-BH-002 | When the Referral is created, the system shall position it at `Recommended`. | L3-G-171 §§9, 13 |
| L4-G-171-BH-003 | When the applicable provider acceptance condition is satisfied, the system shall preserve the acceptance outcome and position the Referral at `Accepted`. | L3-G-171 §§9, 13 |
| L4-G-171-BH-004 | When the institutional referral completion condition is satisfied, the system shall position the Referral at `Completed` and preserve the referral history. | L3-G-171 §§9, 13, 16 |
| L4-G-171-BH-005 | The system shall preserve the association between Referral, patient and Medical Encounter throughout the referral lifecycle. | L3-G-171 §§4, 9, 15 |
| L4-G-171-BH-006 | The system shall not represent external hospital treatment as an institutional referral completion fact. | L3-G-171 §§3, 4, 13 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-171-BH-007 | The retrieved upstream event catalogue does not expose exact canonical Referral Recommended, Accepted or Completed event names. No event name shall be invented; G06 remains canonical publisher of Referral business facts. | L3-G-171 §§14,17 |
| L4-G-171-BH-008 | H may consume applicable G06 Referral facts for configured notifications; E supplies healthcare-professional context without acquiring Referral ownership. | L3-G-171 §§5,14 |

## 6. Failure / Rejection / Continuation

- If referral reason or provider cannot be established, the system shall not complete the Referral against unsupported information.
- If the provider does not accept, the system shall retain the current valid referral position and permit an authorised alternative-provider path.
- If referral completion evidence is pending, the system shall not advance to `Completed`.
- If medical reassessment withdraws the referral before acceptance, the system shall preserve the applicable authoritative business outcome without creating a new unsupported state.
- External treatment uncertainty shall not be converted into G06 completion.

## 7. Audit & Evidence Behavior

The system shall preserve referral reason, provider, responsible healthcare decision, patient/Medical Encounter relationship, acceptance outcome, completion outcome and relevant lifecycle transitions.

## 8. Observable Completion Conditions

Completion is observable when Referral reaches `Completed` under the authoritative institutional completion condition, with history and traceability preserved. External hospital treatment remains outside the completion criterion, and no invented Referral event is required.

## 9. Traceability & Provenance

**Parent:** L3-G-171 — Medical Referral Recommendation to Referral Completion  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Exact Referral lifecycle event names and detailed external-provider acceptance/completion evidence are not exposed in the retrieved upstream source. These remain upstream verification items. Technical communication, APIs, persistence, UI and infrastructure remain deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
