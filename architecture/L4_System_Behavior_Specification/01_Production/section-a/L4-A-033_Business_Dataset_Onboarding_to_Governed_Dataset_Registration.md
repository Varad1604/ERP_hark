# Business Dataset Onboarding to Governed Dataset Registration

## 1. Specification Metadata
**Specification ID:** L4-A-033  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Master Data  
**Realizes Scenario:** L3-A-033  
**Primary Business Owner:** Master Data  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-A-033_Business_Dataset_Onboarding_to_Governed_Dataset_Registration.md`; L2-A09 — Master Data & Reference Data Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall register reusable Master Data or Reference Data with stable identity, canonical ownership, stewardship, metadata and `Draft` governance status without transferring business meaning from the originating domain.

## 3. Required Business Context
Authorized Business Owner; resolved Tenant; unique Dataset name/scope; originating domain semantics; at least one eligible Data Steward.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-033-BH-001 | The system shall reject onboarding when Tenant, authorization, originating-domain ownership or Dataset-name uniqueness cannot be established. | L3-A-033 §§7,9,11–12 |
| L4-A-033-BH-002 | The system shall capture canonical business/governance metadata without assuming ownership of business meaning. | L3-A-033 §§2–5,9,12 |
| L4-A-033-BH-003 | The system shall assign one stable Dataset Identifier only after uniqueness succeeds. | L3-A-033 §§9,12–13 |
| L4-A-033-BH-004 | The system shall require exactly one Business Owner and at least one Data Steward before registration completes. | L3-A-033 §§7,9,12,16 |
| L4-A-033-BH-005 | On successful registration, the system shall establish the Dataset in `Draft`. | L3-A-033 §§3,9,13 |
| L4-A-033-BH-006 | The system shall publish `DatasetRegistered` only after registration completes and shall make governed discovery metadata available. | L3-A-033 §§9,14,16 |
| L4-A-033-BH-007 | Registration alone shall not create a Published Dataset Version or make Draft content available to ordinary consumers. | L3-A-033 §§4,13,16 |

## 5. Cross-Domain & Event Behavior
`DatasetRegistered` is published by the Master Data Registry Service. The originating domain retains business semantics; Module Registry may supply discovery context; A04 supplies authorization decisions.

## 6. Failure / Rejection / Continuation
Duplicate name, unresolved owner, absent steward or denied authorization shall produce no Dataset identity and shall preserve a traceable rejection outcome.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-033-BH-008 | The system shall retain Tenant, actor, owner, steward, stable identifier, metadata, registration outcome and `DatasetRegistered` evidence. | L3-A-033 §15.3 |

## 8. Observable Completion Conditions
A unique `Draft` Dataset exists with stable identity, one Business Owner, at least one Data Steward, discoverable metadata and registration history.

## 9. Traceability & Provenance
**Parent:** L3-A-033 — Business Dataset Onboarding to Governed Dataset Registration  
**Primary verification source:** L2-A09 — Master Data & Reference Data Workflows

## 10. Open Issues / Deferred Design
The Locked parent metadata contains `Depends On: undefined`. This draft does not invent a corrected dependency set; L2-A09 is used only as the parent-cited verification source. Upstream metadata should be corrected before L4 lock. Technical interfaces, storage and UI remain deferred.