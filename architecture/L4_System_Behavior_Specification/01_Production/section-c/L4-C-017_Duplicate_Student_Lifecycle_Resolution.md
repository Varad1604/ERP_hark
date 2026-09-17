# Duplicate Student Lifecycle Resolution

## 1. Specification Metadata

**Specification ID:** L4-C-017  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-017 — Duplicate Student Lifecycle Resolution  
**Primary Business Owner:** Student Lifecycle  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-17  
**Depends On:** L3-C-017; applicable locked C architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall govern resolution of duplicate Student lifecycle records by selecting one canonical survivor, reconciling permitted Student information, preserving lineage, migrating approved Student references, and archiving the duplicate operational record. The behavior ends when one canonical Student remains authoritative and `StudentMerged` is published. Canonical Person duplicate resolution remains A02-owned.

## 3. Required Business Context

- Duplicate Student condition is established.
- Canonical survivor can be selected under policy.
- Required merge approval is satisfied.
- Student Lifecycle owns Student merge/duplicate resolution.
- Person & Identity owns Person identity and its own duplicate-resolution lifecycle.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-017-BH-001 | The system shall validate the identified Student records as eligible duplicate candidates under the authoritative Student duplicate-resolution workflow. | L3-C-017 §§7, 9, 11; L2-C02 WF-C02-16 |
| L4-C-017-BH-002 | When the governed merge decision identifies a survivor, the system shall establish that Student as the single canonical surviving Student. | L3-C-017 §§3, 9, 12, 13; L2-C02 |
| L4-C-017-BH-003 | The system shall reconcile only policy-permitted Student business information and preserve enrollment, lifecycle and audit lineage. | L3-C-017 §§3, 9, 12; L2-C02 |
| L4-C-017-BH-004 | The system shall require the authoritative merge approval before completing the governed merge. | L3-C-017 §§6, 7, 9, 15; L2-C02 |
| L4-C-017-BH-005 | The system shall migrate approved Student business references to the canonical survivor before ending operational use of the duplicate Student. | L3-C-017 §§3, 9, 12, 16; L2-C02 |
| L4-C-017-BH-006 | The system shall archive the duplicate Student record from operational use without creating a replacement Person or Student identity. | L3-C-017 §§3, 9, 12, 13; L2-C02/L2-A02 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-017-BH-007 | On successful Student duplicate resolution, the system shall publish the canonical `StudentMerged` business event under Student Lifecycle ownership. | L3-C-017 §§3, 9, 14; L2-C02 WF-C02-16 |
| L4-C-017-BH-008 | Where Person-side duplicate resolution is required, the system shall preserve A02 ownership and shall not perform or republish a Person merge as an implicit Student merge operation. | L3-C-017 §§4.6, 5, 10.2, 12, 14; L2-A02 |

Any A02-owned `PersonMerged` outcome remains distinct from `StudentMerged`; this specification does not define sequencing not established upstream.

## 6. Failure / Rejection / Continuation

- Do not merge when duplicate evidence is insufficient.
- Do not complete without required approval.
- Do not select multiple operational survivors.
- Do not destroy required historical lineage.
- Do not absorb Person merge into Student merge.

## 7. Audit & Evidence Behavior

Preserve duplicate evidence, survivor decision, approval, reconciliation/lineage evidence, reference-migration evidence, archival outcome and completed merge audit evidence.

## 8. Observable Completion Conditions

One canonical Student survivor is authoritative, permitted information is reconciled, approved Student references resolve to that survivor, duplicate operational use is ended, lineage is preserved, audit evidence exists and `StudentMerged` has been published by Student Lifecycle.

## 9. Traceability & Provenance

**Parent:** L3-C-017 — Duplicate Student Lifecycle Resolution  
**Primary verification source:** Authoritative Student Lifecycle L2 workflow architecture.

## 10. Open Issues / Deferred Design

The exact sequencing between Student merge and any separately required A02 Person merge remains upstream-governed where not established by authoritative architecture. Institution-specific policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
