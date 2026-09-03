# Exhausted Examination Attempts to Conclusive Eligibility Determination

## 1. Specification Metadata

**Specification ID:** L4-I-142  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-142 — Exhausted Examination Attempts to Conclusive Eligibility Determination  
**Primary Business Owner:** Supplementary, Backlog & Improvement Examination  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-142; L2-I12; L2-I01; authoritative Student Lifecycle context  
**Referenced By:** Applicable downstream Examination and Student Lifecycle workflows

## 2. Behavioral Objective & Boundary

Determine one authoritative subsequent-examination eligibility position when the candidate's valid attempt history reaches the governing attempt-limit boundary. I12 applies the policy; I01 owns the examination policy; Student Lifecycle retains ownership of student academic state.

## 3. Required Business Context

- Authoritative candidate and examination scope.
- Complete, traceable valid-attempt history.
- Applicable I01 attempt/eligibility policy.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-142-BH-001 | Establish the candidate, examination scope and authoritative valid-attempt sequence. | L3-I-142 §§3,7–9 |
| L4-I-142-BH-002 | Apply the authoritative attempt policy without inventing a local numeric limit. | L3-I-142 §§9,12 |
| L4-I-142-BH-003 | Determine whether another permitted attempt remains or eligibility is conclusively exhausted. | L3-I-142 §§3,9,16 |
| L4-I-142-BH-004 | Preserve attempt evidence, policy basis and the resulting eligibility determination. | L3-I-142 §§9,15–16 |
| L4-I-142-BH-005 | Make any student-status consequence available to Student Lifecycle rather than changing student state locally. | L3-I-142 §§5,12,14 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-142-BH-006 | I12 shall make the authoritative eligibility outcome available through its existing eligibility/attempt lifecycle. The parent exposes no dedicated `AttemptsExhausted` event; none is invented. Student Lifecycle may consume the resulting examination fact under its own ownership. | L3-I-142 §14 |

## 6. Failure / Rejection / Continuation

- Incomplete attempt history prevents a conclusive exhaustion decision.
- Unknown policy remains unresolved; no default limit is assumed.
- Conflicting attempts/registrations must be resolved by their canonical owners first.
- Any exceptional attempt route requires upstream policy authority.

## 7. Audit & Evidence Behavior

Preserve counted attempts, excluded/non-counted attempts where relevant, policy version/basis, determination authority and resulting eligibility position.

## 8. Observable Completion Conditions

Completion occurs when one authoritative eligibility position exists: another permitted attempt remains, or the applicable attempt opportunity is conclusively exhausted under authoritative policy.

## 9. Traceability & Provenance

**Parent:** L3-I-142  
**Primary verification sources:** L2-I12 and L2-I01.

## 10. Open Issues / Deferred Design

The parent does not define a dedicated exhausted-attempt state or event. Numerical attempt limits and exceptional routes remain policy-owned.

## Canonical Validation Note

This specification applies policy without redefining it and does not modify Student Lifecycle state.
