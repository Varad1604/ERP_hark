# Multi-Factor Enrollment to Active Verified Authentication Factor

## 1. Specification Metadata

**Specification ID:** L4-A-009  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Authentication  
**Realizes Scenario:** L3-A-009  
**Primary Business Owner:** Authentication  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** `L3-A-009_Multi_Factor_Enrollment_to_Active_Verified_Authentication_Factor.md`; `L2-A03_Authentication_Workflows.md`; `L2-A02_Person_and_Identity_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`  ; `L2-A03_Authentication_Recovery_Session_and_MFA_Clarification_Amendment_2026-09-23.md`; L3-A-009 v1.1
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall enroll an additional authentication factor only from a valid authenticated context, apply the governing MFA Enrollment Policy, verify factor ownership, and make the factor available for later authentication only after the MFA Enrollment reaches `Active`.

## 3. Required Business Context

- Exactly one Tenant and canonical Identity.
- An `Active` Authentication Session.
- Applicable MFA Enrollment Policy and policy-permitted factor/method.
- Canonical MFA Enrollment lifecycle `Pending Verification → Verified → Active` as used by the Locked parent.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-009-BH-001 | The system shall prevent factor enrollment unless an `Active` Authentication Session is associated with the requesting Identity within exactly one Tenant. | L3-A-009 §§7, 9, 11–12 |
| L4-A-009-BH-002 | The system shall evaluate the applicable MFA Enrollment Policy before registering the proposed factor and method. | L3-A-009 §§9, 12 |
| L4-A-009-BH-003 | When enrollment is eligible, the system shall create a distinct MFA Enrollment, register the factor, publish `MFAEnrollmentStarted`, and place the enrollment in `Pending Verification`. | L3-A-009 §§9, 13–14 |
| L4-A-009-BH-004 | The system shall prevent the factor from contributing to authentication while ownership verification remains incomplete. | L3-A-009 §§9–13 |
| L4-A-009-BH-005 | When factor ownership and required method conditions are verified, the system shall transition `Pending Verification → Verified` and publish `MFAEnrollmentVerified`. | L3-A-009 §§9, 13–14 |
| L4-A-009-BH-006 | The system shall prevent activation unless the canonical Identity remains valid and activation satisfies the applicable Authentication Policy. | L3-A-009 §§9, 11–13 |
| L4-A-009-BH-007 | When activation conditions succeed, the system shall transition `Verified → Active` and publish `MFAActivated`. | L3-A-009 §§9, 13–16 |
| L4-A-009-BH-008 | The system shall make only an `Active` verified factor available for later policy-driven MFA challenges without treating enrollment as a completed challenge or an authorization decision. | L3-A-009 §§2–4, 12, 16 |

## 5. Cross-Domain & Event Behavior

The exact canonical events are `MFAEnrollmentStarted`, `MFAEnrollmentVerified`, and `MFAActivated`, each published by the MFA Enrollment aggregate. The system shall not invent `MFAEnrolled`. Authorization & Policy may consume resulting assurance information without gaining MFA lifecycle ownership.

## 6. Failure / Rejection / Continuation

If Tenant/session/Identity eligibility, factor policy, ownership verification or activation eligibility fails, the system shall prevent further progression and shall not publish the corresponding success event or expose the factor as active.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-009-BH-009 | The system shall preserve attributable evidence of enrollment initiation, policy basis, factor/method classification, ownership verification, lifecycle transitions and canonical event publication without exposing factor secrets. | L3-A-009 §15.3 |

## 8. Observable Completion Conditions

Completion requires an MFA Enrollment associated with one Tenant and one Identity to reach `Active` through the parent-defined verification sequence, with all three canonical events published and the factor available for later MFA challenges. Existing factors, Person/Identity lifecycles, credentials and authorization decisions remain unchanged.

## 9. Traceability & Provenance

**Parent:** L3-A-009 — Multi-Factor Enrollment to Active Verified Authentication Factor  
**Primary verification source:** L2-A03 — Authentication Workflows

## R3B Canonical Behavioral Addendum

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-009-BH-010 | `EnrollMFAFactor` shall create the enrollment directly in `Pending Verification` and publish `MFAEnrollmentStarted`; no durable `Enrolled` state is created. | L3-A-009 v1.1; A03 R3B amendment §7 |
| L4-A-009-BH-011 | `VerifyMFAFactor` shall transition `Pending Verification → Verified` and publish `MFAEnrollmentVerified`. | L3-A-009 v1.1; A03 R3B amendment §7 |
| L4-A-009-BH-012 | `ActivateMFAFactor` shall transition `Verified → Active` and publish `MFAActivated`; no `MFAEnrolled` event is canonical. | L3-A-009 v1.1; A03 R3B amendment §7 |

## 10. Open Issues / Deferred Design

No blocking L4 issue remains after the Locked R3B upstream amendment(s) and synchronized L3-A-009 v1.1.

Implementation-specific APIs, persistence, cryptographic mechanisms, provider/product choices and UI remain downstream concerns.


## Canonical Validation Note

This specification passed the scoped R3B hard-fail validation and is `Architecture Approved` under `L4_R3B_Core_IAM_Semantics_Architecture_Approval_Decision_2026-09-23.md`. This approval does not Lock the specification or automatically validate any L4→L5 relation.
