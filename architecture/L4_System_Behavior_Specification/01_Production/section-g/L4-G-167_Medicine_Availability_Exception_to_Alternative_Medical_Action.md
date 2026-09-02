# Medicine Availability Exception to Alternative Medical Action

## 1. Specification Metadata

**Specification ID:** L4-G-167  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-167 — Medicine Availability Exception to Alternative Medical Action  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-167; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall identify when a prescribed medicine cannot be dispensed as intended, prevent a false dispensing outcome, and preserve the authorised alternative medical action or explicit unresolved healthcare outcome. Clinical substitution decisions remain with authorised healthcare professionals.

## 3. Required Business Context

- Prescription and Medicine Dispense are G06-owned.
- The affected medicine and availability condition are authoritative business facts.
- Alternative medical action requires authorised healthcare judgement.
- Patient and Medical Encounter continuity must be preserved.
- The Prescription lifecycle remains `Created → Issued → Dispensed → Completed`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-167-BH-001 | When a prescribed medicine cannot be dispensed as intended, the system shall record the availability exception against the relevant Prescription and Medical Encounter. | L3-G-167 §§3, 8, 9 |
| L4-G-167-BH-002 | The system shall prevent the unavailable medicine from being represented as dispensed without a valid dispensing outcome. | L3-G-167 §§9, 12, 16 |
| L4-G-167-BH-003 | When an authorised medical professional establishes an alternative medical action, the system shall record that action in the relevant healthcare context. | L3-G-167 §§9, 15, 16 |
| L4-G-167-BH-004 | When no acceptable alternative is established, the system shall preserve the explicit unresolved or next-care outcome rather than inventing a substitute treatment. | L3-G-167 §§10, 11 |
| L4-G-167-BH-005 | When the alternative requires a new or revised prescription, follow-up or referral, the system shall preserve the hand-off to the corresponding authoritative healthcare workflow. | L3-G-167 §§9, 10 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-167-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Institutional Health Services (G06) ownership. | L3-G-167 §14 |

## 6. Failure / Rejection / Continuation

- If the availability exception cannot be established, the system shall not initiate an alternative action on an unsupported shortage.
- If authorised medical authority is unavailable, the system shall preserve the exception until appropriate authority is available.
- If no clinically acceptable alternative exists, the system shall preserve the unresolved healthcare outcome.
- If patient or encounter context is unresolved, the system shall not attach the alternative action to an invented context.

## 7. Audit & Evidence Behavior

The system shall preserve the original Prescription, affected medicine, availability condition, responsible medical professional, alternative action or unresolved outcome, patient/Medical Encounter relationship and relevant lifecycle evidence.

## 8. Observable Completion Conditions

Completion is observable when the availability exception is recorded and either an authorised alternative medical action or an explicit unresolved healthcare outcome is established. The unavailable medicine is not falsely recorded as dispensed.

## 9. Traceability & Provenance

**Parent:** L3-G-167 — Medicine Availability Exception to Alternative Medical Action  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
