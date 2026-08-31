# Erroneous Person Consolidation to Restored Identity Separation

## 1. Specification Metadata

**Specification ID:** L4-A-006  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Person & Identity  
**Realizes Scenario:** L3-A-006  
**Primary Business Owner:** Person & Identity  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-A-006_Erroneous_Person_Consolidation_to_Restored_Identity_Separation.md`; `L2-A02_Person_and_Identity_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A07_Audit_Infrastructure_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern correction of an approved erroneous Person consolidation so separate canonical Persons are restored, references are redistributed only through governed ownership-preserving behavior, original merge history remains intact, and `PersonSplit` records the completed separation. The system shall not invent unresolved Person/Identity states, approving roles, or downstream reference-transfer contracts.

## 3. Required Business Context

- One resolved `Active` Tenant containing the original consolidation.
- Preserved original merge history and affected Person references.
- Approved administrative determination identifying an allowed separation cause.
- Split eligibility, dependency and legal-compliance results.
- Required reference-redistribution plan and administrative approval.
- Canonical `PersonSplit` event semantics.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-006-BH-001 | The system shall prevent A006 separation unless an approved administrative determination identifies an incorrect merge, identity fraud, or data-migration error as the governed cause. | L3-A-006 §§7–9, 11–12 |
| L4-A-006-BH-002 | Before separation, the system shall require validated original merge history, a complete audit trail, a reference-redistribution plan, and successful dependency and legal-compliance validation. | L3-A-006 §§9, 11–12 |
| L4-A-006-BH-003 | The system shall prevent Person separation until the required administrative approval outcome is valid. | L3-A-006 §§9, 12, 15 |
| L4-A-006-BH-004 | When the source-defined split outcome proceeds, the system shall preserve the immutable identifiers of existing Persons and establish the additional canonical Person required by the split workflow without assigning an unsupported lifecycle state. | L3-A-006 §§9, 12–13 |
| L4-A-006-BH-005 | The system shall preserve the original merge history while recording the corrective separation as a later governed outcome rather than erasing the prior consolidation. | L3-A-006 §§9, 12, 15–16 |
| L4-A-006-BH-006 | The system shall permit reference redistribution between restored canonical Persons only when the redistribution basis is source-governed, and shall not transfer ownership of downstream business records. | L3-A-006 §§4–5, 9, 12, 14 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-006-BH-007 | When the governed Person separation completes, the system shall publish `PersonSplit` on behalf of the Person aggregate. | L3-A-006 §§9, 14, 16 |

No `IdentitySplit` event or downstream-reference redistribution contract is asserted because neither is canonically defined by the parent sources.

## 6. Failure / Rejection / Continuation

If merge history, audit evidence, redistribution planning, dependency/legal validation or approval is missing or invalid, the system shall prevent separation. Where the required Person or Identity lifecycle effect is undefined, or downstream references cannot be redistributed through an approved contract, the system shall expose the unresolved condition and shall not claim completed separation.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-006-BH-008 | The system shall preserve attributable evidence of the original merge, approved separation determination, split validations, redistribution basis, approval, `PersonSplit`, and the final restored-separation relationship. | L3-A-006 §15.3–16 |

## 8. Observable Completion Conditions

Completion requires the approved separation determination and all mandatory validations, required approval, restored separate canonical Persons, source-governed reference redistribution, preserved merge and split history, and publication of `PersonSplit`. Undefined Identity lifecycle effects or reference-transfer contracts shall not be silently treated as completed.

## 9. Traceability & Provenance

**Parent:** L3-A-006 — Erroneous Person Consolidation to Restored Identity Separation  
**Primary verification source:** L2-A02 — Person & Identity Workflows  
**Additional verification:** L2-A04 — Authorization & Policy; L2-A07 — Audit Infrastructure; Locked L1 Person-scope amendment 2026-08-26

## 10. Open Issues / Deferred Design

The current Locked L1 amendment confirms Tenant-scoped canonical Person identity. Upstream still does not define the specific split-approving role, the exact Person/Identity lifecycle effects of split-created or reassociated identities, or the cross-domain contract for reference redistribution. These remain explicit deferred architecture issues.
