# Person Establishment to Active Canonical Identity

## 1. Specification Metadata

**Specification ID:** L4-A-004  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Person & Identity  
**Realizes Scenario:** L3-A-004  
**Primary Business Owner:** Person & Identity  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** `L3-A-004_Person_Establishment_to_Active_Canonical_Identity.md`; `L2-A02_Person_and_Identity_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Amendment_2026-08-26.md`; `DEPENDENCY_MAP.md`  ; `L2-A02_User_Account_and_Person_Reconciliation_Authority_Amendment_2026-09-23.md`; L3-A-004 v1.1
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish one Tenant-scoped canonical Person and associated Identity for a real individual, prevent uncontrolled duplicate creation, progress both aggregates through their source-defined verification and activation lifecycles, and expose immutable canonical references only after the governed outcome is reached. User Account creation, authentication, authorization and business-role lifecycles remain outside this specification.

## 3. Required Business Context

- One resolved `Active` Tenant.
- Tenant-scoped canonical Person uniqueness under the locked 2026-08-26 L1 amendment.
- Applicable duplicate-detection and verification policies.
- Canonical Person states `Draft`, `Created`, `Verified`, `Active`.
- Canonical Identity states `Pending`, `Verified`, `Active`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-004-BH-001 | The system shall prevent Person establishment outside exactly one resolved `Active` Tenant. | L3-A-004 §§4, 7, 9, 12 |
| L4-A-004-BH-002 | Before creating a Person, the system shall evaluate the required Person information and applicable duplicate-detection policy. | L3-A-004 §§7, 9, 12 |
| L4-A-004-BH-003 | When creation is eligible, the system shall create the canonical Person through `Draft → Created` and publish `PersonCreated` on behalf of the Person aggregate. | L3-A-004 §§9, 13–14 |
| L4-A-004-BH-004 | After canonical Person creation, the system shall create exactly one associated Identity in `Pending` and publish `IdentityCreated` on behalf of the Identity aggregate. | L3-A-004 §§9, 12–14 |
| L4-A-004-BH-005 | The system shall prevent verification until the applicable Tenant verification requirements and any source-required approval are satisfied. | L3-A-004 §§9, 11–12, 15 |
| L4-A-004-BH-006 | When Person verification succeeds, the system shall perform `Created → Verified` and publish `PersonVerified`. | L3-A-004 §§9, 13–14 |
| L4-A-004-BH-007 | When Identity verification succeeds, the system shall perform `Pending → Verified` and publish `IdentityVerified`. | L3-A-004 §§9, 13–14 |
| L4-A-004-BH-008 | The system shall prevent the establishment outcome from completing unless both Person and associated Identity have satisfied their source-defined activation prerequisites. | L3-A-004 §§9, 12, 16 |
| L4-A-004-BH-009 | When activation is valid, the system shall transition the Person `Verified → Active` and publish `PersonActivated`. | L3-A-004 §§9, 13–14 |
| L4-A-004-BH-010 | When activation is valid, the system shall transition the Identity `Verified → Active` and publish `IdentityActivated`. | L3-A-004 §§9, 13–14 |
| L4-A-004-BH-011 | After both aggregates are `Active`, the system shall make their immutable canonical references available for governed downstream use without transferring Person or Identity ownership. | L3-A-004 §§3–5, 9, 14, 16 |

## 5. Cross-Domain & Event Behavior

The system shall recognize the exact canonical events `PersonCreated`, `IdentityCreated`, `PersonVerified`, `IdentityVerified`, `PersonActivated`, and `IdentityActivated` with the Person or Identity aggregate retaining sole semantic publisher ownership as defined by the parent. Downstream consumption does not transfer ownership.

## 6. Failure / Rejection / Continuation

If Tenant eligibility fails, duplicate policy blocks creation, verification fails, or required approval is absent, the system shall prevent unsupported progression and preserve the last valid lifecycle position. A confirmed duplicate transfers to the separate reconciliation scenario; L4-A-004 shall not merge records.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-004-BH-012 | The system shall preserve attributable evidence of Person/Identity creation, verification and activation and of any policy-permitted duplicate-warning override. | L3-A-004 §§12, 15 |

## 8. Observable Completion Conditions

Completion requires one canonical Tenant-scoped Person and one associated Identity to be `Active`, all six canonical lifecycle events to have been published at their governed transitions, and immutable Person/Identity references to be available for downstream reference. No User Account, credential, authentication session, Role Assignment or business persona is created by this specification.

## 9. Traceability & Provenance

**Parent:** L3-A-004 — Person Establishment to Active Canonical Identity  
**Primary verification source:** L2-A02 — Person & Identity Workflows  
**Higher-level verification:** Locked L1 Person-scope amendment 2026-08-26

The L1 amendment supersedes the older Institute-scoped Person wording and establishes Tenant-scoped canonical Person identity.

## R3B Canonical Behavioral Addendum

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-004-BH-013 | Completion of Person/Identity activation shall not automatically create a User Account. | L3-A-004 v1.1; A02 R3B amendment §1 |
| L4-A-004-BH-014 | Where login capability is required after activation, the system shall require both authoritative login-capability basis and User Account Provisioning Authority before invoking the separate A02 User Account lifecycle. | L3-A-004 v1.1; A02 R3B amendment §1 |
| L4-A-004-BH-015 | Absence of login-capability authority shall leave the Active Person/Identity valid without fabricating an account. | L3-A-004 v1.1; A02 R3B amendment §1 |

## 10. Open Issues / Deferred Design

No blocking L4 issue remains after the Locked R3B upstream amendment(s) and synchronized L3-A-004 v1.1.

Implementation-specific APIs, persistence, cryptographic mechanisms, provider/product choices and UI remain downstream concerns.


## Canonical Validation Note

This specification passed the scoped R3B hard-fail validation and is `Architecture Approved` under `L4_R3B_Core_IAM_Semantics_Architecture_Approval_Decision_2026-09-23.md`. This approval does not Lock the specification or automatically validate any L4→L5 relation.
