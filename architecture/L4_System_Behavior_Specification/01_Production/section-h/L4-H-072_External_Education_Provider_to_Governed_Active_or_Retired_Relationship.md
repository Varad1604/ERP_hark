# External Education Provider to Governed Active or Retired Relationship

## 1. Specification Metadata
**Specification ID:** L4-H-072  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** External Education Integration  
**Realizes Scenario:** L3-H-072  
**Primary Business Owner:** External Education Integration  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-072; L2-H10 — External Education Integration Workflows; `L2-H10_Provider_Reconciliation_and_Synchronization_Normalization_Amendment_2026-09-01.md`; applicable Locked A/B architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern the H10 External Provider relationship through verification, certification, activation, restriction/suspension, rejection or terminal retirement while keeping External Provider and External Education Provider Trust as separate governed concepts.

## 3. Required Business Context
Institutional and organization references remain authoritative in Section B; identity/authentication facts remain owned by Section A; H10 provider due-diligence and certification evidence is available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-072-BH-001 | External Provider progression shall preserve the H10 lifecycle `Pending Verification → Verified → Certified → Active` with governed `Restricted`, `Suspended` and terminal `Retired` outcomes. | L3-H-072 §§9–13; L2-H10 normalization amendment §1 |
| L4-H-072-BH-002 | The system shall not merge External Provider lifecycle positions with the separate Provider Trust lifecycle `Requested → Under Review → Approved → Active → Revoked` and its governed branches. | L2-H10 normalization amendment §1 |
| L4-H-072-BH-003 | Provider activation shall occur only after parent-defined verification, certification, authentication and governance prerequisites are satisfied. | L3-H-072 §§7–12 |
| L4-H-072-BH-004 | A retired External Provider relationship shall not reactivate in place; a future relationship requires new governed onboarding. | L3-H-072 §§10–13,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-072-BH-005 | H10 shall publish `ProviderRegistered`, `ProviderActivated`, `ProviderSuspended` and `ProviderRetired` for the corresponding External Provider facts. | L3-H-072 §14 |
| L4-H-072-BH-006 | When provider onboarding is rejected, the External Provider aggregate shall publish `ProviderOnboardingRejected`; that fact shall not create or mutate a Provider Trust record. | L2-H10 normalization amendment §2 |
| L4-H-072-BH-007 | Section A identity/authentication and Section B institutional facts shall remain under their canonical ownership throughout H10 provider governance. | L3-H-072 §§5,14–16 |

## 6. Failure / Rejection / Continuation
Failed verification/certification or governance rejection shall prevent operational activation. Suspended providers may return to Active only through the parent-supported governed restoration path; Retired remains terminal.

## 7. Audit & Evidence Behavior
The system shall preserve provider registration, validation, certification, security/governance evidence, rejection/activation/suspension/retirement decisions and referenced A/B authority evidence without ownership transfer.

## 8. Observable Completion Conditions
Completion requires the provider relationship to reach its governed approved/Active, rejected, restricted/suspended or Retired outcome with External Provider and Provider Trust lifecycles remaining separate and auditable.

## 9. Traceability & Provenance
**Parent:** L3-H-072 — External Education Provider to Governed Active or Retired Relationship  
**Primary verification sources:** L2-H10 — External Education Integration Workflows; L2-H10 Provider, Reconciliation and Synchronization Normalization Amendment — 2026-09-01.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Connector, authentication and transport implementation remain deferred.
