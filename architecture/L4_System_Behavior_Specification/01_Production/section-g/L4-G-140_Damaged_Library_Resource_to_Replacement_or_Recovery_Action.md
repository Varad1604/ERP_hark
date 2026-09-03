# Damaged Library Resource to Replacement or Recovery Action

## 1. Specification Metadata

**Specification ID:** L4-G-140  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-140 — Damaged Library Resource to Replacement or Recovery Action  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-140; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall record and assess damage to an identified Library Copy/resource and establish the applicable operational recovery or replacement/fine outcome without inventing damage thresholds, repair procedures, financial values or procurement behavior.

## 3. Required Business Context

- The affected Library Copy/resource is identifiable.
- Physical condition and damage observations can be established.
- Associated member/borrowing context is available where relevant.
- Applicable damage/recovery policy is authoritative.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-140-BH-001 | Record the authoritative Physical Condition and Damage Observations for the affected Library Copy/resource. | L3-G-140 §§2,9,12 |
| L4-G-140-BH-002 | Determine whether the resource remains suitable for ordinary circulation under the applicable configurable policy. | L3-G-140 §§3,9–10 |
| L4-G-140-BH-003 | Where damage prevents ordinary use, prevent the resource from being falsely represented as ordinarily available until the authoritative recovery/assessment outcome permits it. | L3-G-140 §§3–4,9–11 |
| L4-G-140-BH-004 | Where policy requires, establish a Damaged Resource fine or Replacement Charge obligation without inventing an amount or threshold. | L3-G-140 §§9,12 |
| L4-G-140-BH-005 | Preserve the canonical Library Copy lifecycle `Added → Available → Issued → Returned → Withdrawn`; damage is an observation/operational condition, not a new `Damaged` lifecycle state. | L3-G-140 §13 |
| L4-G-140-BH-006 | Preserve condition, damage observations, operational treatment, recovery/fine decision and downstream financial/notification references. | L3-G-140 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-140-BH-007 | The authoritative source does not expose a dedicated canonical damaged-resource or replacement-charge event. This specification shall not invent `ResourceDamaged` or `ReplacementChargeCreated`. G owns damage assessment and the library operational/fine outcome; F owns accounting/payment and H owns notification delivery. | L3-G-140 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- If condition cannot be established, ordinary availability shall not be asserted.
- If recovery value cannot be established, no invented amount shall be created.
- If administrative review is required, the operational outcome remains unresolved until the source-supported authority decides.
- Payment, physical repair and procurement remain outside this scenario.

## 7. Audit & Evidence Behavior

Preserve resource identity, relevant borrowing/member context, physical condition, damage observations, assessment basis, operational treatment, recovery/fine decision, authority and financial/notification references where applicable.

## 8. Observable Completion Conditions

Completion occurs when the damage assessment and applicable operational/recovery treatment are authoritative and traceable, without inventing a new damage lifecycle state or downstream payment/procurement result.

## 9. Traceability & Provenance

**Parent:** L3-G-140 — Damaged Library Resource to Replacement or Recovery Action  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The upstream source does not expose a dedicated damaged-resource event or fixed recovery decision matrix. Damage severity thresholds, repair/replacement values and detailed technical mechanisms are not invented. APIs, persistence, UI, infrastructure and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
