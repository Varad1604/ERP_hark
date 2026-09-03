# Institutional Progression Policy Lifecycle to Effective Progression Governance

## 1. Specification Metadata

**Specification ID:** L4-C-075  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-075 — Institutional Progression Policy Lifecycle to Effective Progression Governance  
**Primary Business Owner:** Academic Progression  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-C-075; applicable locked C architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Specify observable behavior for establishing an authoritative effective progression-policy position before it is used by Academic Progression to evaluate individual progression decisions.

## 3. Required Business Context

- Authoritative Academic Progression context exists.
- Applicable upstream business policy and governance conditions are satisfied.
- Relevant cross-domain facts remain source-owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-075-BH-001 | The system shall govern progression-policy definition or change. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-002 | The system shall validate the policy against applicable institutional progression governance and academic context. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-003 | The system shall establish effective applicability. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-004 | The system shall preserve policy history and version continuity. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-005 | The system shall make the effective policy position available to subsequent Academic Progression evaluations. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-006 | The system shall individual Student progression decisions. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-007 | The system shall conditional promotion, probation, detention, repeat, credit transfer, or programme migration decisions. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-008 | The system shall examination processing. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-009 | The system shall academic registration. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-010 | The system shall curriculum ownership. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-011 | The system shall studentContext mutation. | L3-C-075 §§3, 9, 12 |
| L4-C-075-BH-012 | The system shall technical policy engines, APIs, databases, or access-control implementation. | L3-C-075 §§3, 9, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-075-BH-013 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Academic Progression ownership. | L3-C-075 §14 |

## 6. Failure / Rejection / Continuation

- If required policy authority is not established, the proposed policy cannot become effective and the existing effective policy position remains authoritative. If institutional context is ambiguous, the policy is not made effective until the authoritative context is resolved. Conflicts with institutional academic governance remain subject to upstream resolution.
- No technical retry, queue, rollback, server, or infrastructure recovery behavior is specified.

## 7. Audit & Evidence Behavior

- Progression policy must pass applicable institutional governance before becoming effective. - The exact canonical approving role is not sufficiently defined in the accessible L2-C04 source and is not invented. - Policy applicability and effective position must remain auditable. - Historical policy/progression context must remain reproducible. - Manual overrides require authorised approval and complete audit history. - No technical IAM, permissions, or enforcement mechanism is specified.

## 8. Observable Completion Conditions

- Applicable institutional governance has been completed. - Effective applicability has been established. - The effective progression-policy position is authoritative for Academic Progression. - Policy history is preserved. - The effective policy can be used by subsequent progression decision scenarios. - No Student-specific progression decision is made by this behavior.

## 9. Traceability & Provenance

**Parent:** L3-C-075 — Institutional Progression Policy Lifecycle to Effective Progression Governance  
**Primary verification source:** Authoritative Student Lifecycle L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
