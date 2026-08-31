# Duplicate Person Identification to Canonical Identity Reconciliation

## 1. Specification Metadata

**Specification ID:** L4-A-005  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Person & Identity  
**Realizes Scenario:** L3-A-005  
**Primary Business Owner:** Person & Identity  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-A-005_Duplicate_Person_Identification_to_Canonical_Identity_Reconciliation.md`; `L2-A02_Person_and_Identity_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A07_Audit_Infrastructure_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern reconciliation of confirmed duplicate Person records inside one Tenant so one canonical Person survives, permitted duplicate records become historical merge sources, applicable references remain traceable, and the merge is auditable. The behavior shall not invent Identity lifecycle effects, downstream record-write mechanisms, or approval authority that the Locked parent leaves unresolved.

## 3. Required Business Context

- One resolved `Active` Tenant.
- Two or more Person records confirmed to represent the same real individual within that Tenant.
- Proposed canonical survivor and merge-policy context.
- Dependency and legal-restriction validation results.
- Required administrative approval outcome.
- Canonical Person lifecycle and `PersonMerged` event semantics.
- Tenant-scoped Person identity established by the 2026-08-26 Locked L1 amendment.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-005-BH-001 | The system shall prevent duplicate reconciliation when the proposed Person records do not resolve to the same Tenant. | L3-A-005 §§7, 9, 11–12 |
| L4-A-005-BH-002 | Before merge progression, the system shall require a selected canonical survivor and successful source-defined merge-policy, dependency and legal-restriction validation. | L3-A-005 §§9, 11–12 |
| L4-A-005-BH-003 | The system shall prevent merge execution until the required administrative approval outcome exists. | L3-A-005 §§9, 12, 15 |
| L4-A-005-BH-004 | When the governed merge is permitted, the system shall retain the selected Person as the canonical survivor without changing its immutable Person identifier. | L3-A-005 §§9, 12–13 |
| L4-A-005-BH-005 | The system shall preserve each non-surviving Person as a historical merge source and shall apply `Archived` only where the source-defined Person lifecycle permits that transition. | L3-A-005 §§9, 11, 13 |
| L4-A-005-BH-006 | The system shall preserve historical audit references and business history while making the governed canonical-survivor relationship observable. | L3-A-005 §§9, 12, 15–16 |
| L4-A-005-BH-007 | The system shall not directly modify, re-own, or redefine downstream business records as part of Person reconciliation. | L3-A-005 §§4–5, 9, 12, 14 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-005-BH-008 | When the Person merge has reached its source-defined completed outcome, the system shall publish `PersonMerged` on behalf of the Person aggregate. | L3-A-005 §§9, 14, 16 |

Admissions, Finance and other consumers retain ownership of their records. No synchronous downstream-reference transfer contract is invented because the parent records that contract as unresolved.

## 6. Failure / Rejection / Continuation

The system shall leave Person records outside a completed merge when Tenant isolation, survivor selection, policy, dependency, legal restriction, or required approval fails. If a non-surviving Person is in a state without an authoritative transition to `Archived`, or an associated Identity effect is required but undefined, the system shall expose the unresolved condition and shall not invent a lifecycle transition.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-005-BH-009 | The system shall preserve attributable evidence of duplicate confirmation, survivor selection, validation results, administrative approval, resulting Person relationships, `PersonMerged`, and the preserved historical merge chain. | L3-A-005 §§15–16 |

## 8. Observable Completion Conditions

Completion requires one canonical surviving Person, all mandatory validations and approval satisfied, source-permitted historical handling of duplicate Persons, preserved history and references, and publication of `PersonMerged`. No completion assertion is made for undefined associated-Identity effects or an undefined downstream-reference transfer contract.

## 9. Traceability & Provenance

**Parent:** L3-A-005 — Duplicate Person Identification to Canonical Identity Reconciliation  
**Primary verification source:** L2-A02 — Person & Identity Workflows  
**Additional verification:** L2-A04 — Authorization & Policy; L2-A07 — Audit Infrastructure; Locked L1 Person-scope amendment 2026-08-26

## 10. Open Issues / Deferred Design

The parent’s former Person-scope conflict is resolved by the Locked 2026-08-26 L1 amendment establishing Tenant-scoped canonical Person identity. Still unresolved upstream are the specific merge-approving business role, split/merge effects on associated Identity aggregates, and the governed cross-domain contract for reconciling downstream-owned references. This Draft preserves those gaps and does not repair them locally.
