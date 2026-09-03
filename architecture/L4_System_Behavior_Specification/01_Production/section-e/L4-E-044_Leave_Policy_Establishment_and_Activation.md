# Leave Policy Establishment and Activation

## 1. Specification Metadata

**Specification ID:** L4-E-044  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-044 — Leave Policy Establishment and Activation  
**Primary Business Owner:** Leave Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-044; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish, validate, approve and activate a Leave Management-owned policy so that an effective policy version governs applicable leave decisions without transferring policy ownership to participating governance actors.

## 3. Required Business Context

- A leave policy draft exists.
- Policy rules are available for validation.
- Required governance and approval authority is available.
- Policy versioning and effective dating remain governed by the authoritative Leave Policy lifecycle.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-044-BH-001 | The system shall establish the Leave Policy Aggregate in its governed draft position and make it available for rule validation. | L3-E-044 §§4, 5, 9, 12 |
| L4-E-044-BH-002 | The system shall validate the policy rules and record the validation outcome. | L3-E-044 §§3, 9, 11, 15 |
| L4-E-044-BH-003 | The system shall identify policy conflicts and, where a conflict exists, preserve the pre-activation position until governed conflict resolution is completed and the policy returns to validation. | L3-E-044 §§9, 10, 11, 13 |
| L4-E-044-BH-004 | The system shall obtain and record the required policy approval with attributable actor, timestamp and decision evidence. | L3-E-044 §§6, 9, 15 |
| L4-E-044-BH-005 | The system shall activate only an approved policy and establish the effective policy version according to its governed effective dating. | L3-E-044 §§3, 9, 12, 13, 16 |
| L4-E-044-BH-006 | The system shall preserve validation findings, conflict resolution, approval, policy version and activation evidence. | L3-E-044 §§12, 15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-044-BH-007 | After successful activation, the system shall publish the policy-activation business event required by the authoritative L2 workflow, but L4 shall not invent a canonical event identifier because the Locked parent does not expose one. | L3-E-044 §§9, 14.1, 17.1 |
| L4-E-044-BH-008 | The activated policy outcome shall remain owned by Leave Management and may be consumed as governed business context by downstream leave decision workflows without ownership transfer. | L3-E-044 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- If policy validation identifies a conflict, the system shall prevent approval/activation until the conflict is resolved and the policy is re-validated.
- If required approval is not obtained, the policy shall not become Active and shall remain in its governed pre-activation lifecycle position.
- If activation is not yet appropriate, no effective policy shall be established from this scenario.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of validation, identified conflicts, governance resolution, approval, policy version history, effective dating and activation.

## 8. Observable Completion Conditions

Completion requires policy rules to be validated, identified conflicts to be governed/resolved, required approval to be recorded, the policy to reach Active, the effective version to be established, and the required policy-activation business event to be published without inventing an unsupported identifier.

## 9. Traceability & Provenance

**Parent:** L3-E-044 — Leave Policy Establishment and Activation  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

The Locked parent confirms that L2-E05 requires a policy-activation event but does not expose its canonical identifier/name. That identifier remains an upstream architecture question before approval/lock. Technical APIs, persistence schemas, UI, transport and retry mechanics are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
