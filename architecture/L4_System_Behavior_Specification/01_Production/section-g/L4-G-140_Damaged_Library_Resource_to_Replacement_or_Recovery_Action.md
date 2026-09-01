# Damaged Library Resource to Replacement or Recovery Action

## 1. Specification Metadata
**Specification ID:** L4-G-140  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-140 — Damaged Library Resource to Replacement or Recovery Action  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-140 — Damaged Library Resource to Replacement or Recovery Action

## 2. Behavioral Objective & Boundary
SutraERP shall record and assess a damaged library resource and establish the applicable operational recovery or replacement outcome without inventing damage thresholds or financial values.

**Start:** Resource is determined damaged.  
**End:** Damage treatment and applicable recovery/replacement obligation are established.

## 3. Required Business Context
- Library Copy/resource is identifiable.
- Physical condition and damage observations can be established.
- Applicable damage/recovery policy is authoritative.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-140-BH-001 | Record Physical Condition and Damage Observations for the affected resource. | L3-G-140 §§2,9,12 |
| L4-G-140-BH-002 | Determine whether the resource remains suitable for ordinary circulation under applicable policy. | L3-G-140 §§3,9 |
| L4-G-140-BH-003 | Prevent ordinary availability when the authoritative damage assessment requires recovery or restriction. | L3-G-140 §§3,4 |
| L4-G-140-BH-004 | Establish a Damaged Resource or Replacement Charge obligation where applicable policy requires it. | L3-G-140 §§2,9,12 |
| L4-G-140-BH-005 | Preserve condition, recovery and resulting operational history. | L3-G-140 §§13,15 |

## 5. Cross-Domain & Event Behavior
No dedicated damaged-resource event is asserted by the authoritative source. Finance handles financial consequences and H handles communication; G retains ownership.

## 6. Failure / Rejection / Continuation
- If condition cannot be established, ordinary availability shall not be asserted.
- If recovery value cannot be established, no invented amount shall be created.
- If administrative review is required, the operational outcome remains subject to the authoritative decision.
- Payment and physical repair remain outside this scenario.

## 7. Audit & Evidence Behavior
Preserve condition, damage observations, assessment basis, operational treatment, recovery/fine decision, authority and financial reference where applicable.

## 8. Observable Completion Conditions
Completion occurs when the damage outcome and applicable operational/recovery treatment are authoritative and traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-140 — Damaged Library Resource to Replacement or Recovery Action  
**Primary verification source:** L3-G-140; locked L2 Library Copy/Fine workflows

## 10. Open Issues / Deferred Design
Damage thresholds, replacement values and repair implementation remain policy/downstream concerns. Technical persistence and interfaces remain outside L4.