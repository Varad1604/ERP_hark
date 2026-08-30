# Governed Dataset Creation to Published Enterprise Availability

## 1. Specification Metadata
**Specification ID:** L4-A-034  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Master Data  
**Realizes Scenario:** L3-A-034  
**Primary Business Owner:** Master Data  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-A-034_Governed_Dataset_Creation_to_Published_Enterprise_Availability.md`; L2-A09 — Master Data & Reference Data Workflows; L3-A-033  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall progress governed Master Data or Reference Data through Draft, validation, review, approval and publication so only an approved immutable Dataset Version becomes discoverable and available to authorized consumers.

## 3. Required Business Context
Registered Dataset; authorized Data Steward; applicable schema/business constraints; Reviewer authority; publication policy and effective dates.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-034-BH-001 | The system shall validate Dataset registration, schema/category, uniqueness, authorization, business rules, duplicates and quality before review. | L3-A-034 §§7,9,11–12 |
| L4-A-034-BH-002 | Valid proposed content shall enter `Draft`; invalid or duplicate content shall not progress to review. | L3-A-034 §§9–13 |
| L4-A-034-BH-003 | The system shall permit `Draft → Review` only after completeness succeeds and shall publish `DatasetSubmittedForReview`. | L3-A-034 §§9,12–14 |
| L4-A-034-BH-004 | The system shall record the authorized Reviewer outcome and prevent approval where review fails. | L3-A-034 §§9–12 |
| L4-A-034-BH-005 | On approval, the system shall transition `Review → Approved`, preserve the approved version and publish `DatasetApproved`. | L3-A-034 §§9,13–14 |
| L4-A-034-BH-006 | The system shall prevent publication unless validation, approval, policy, effective-date and authorization prerequisites all succeed. | L3-A-034 §§9,11–12 |
| L4-A-034-BH-007 | Successful publication shall transition the approved version to `Published`, make it immutable and publish `DatasetPublished`. | L3-A-034 §§3,9,12–14,16 |
| L4-A-034-BH-008 | Ordinary consumers shall not receive Draft, Review or merely Approved content. | L3-A-034 §§4,12,16 |

## 5. Cross-Domain & Event Behavior
Source-defined events include `MasterDataCreated` or `ReferenceDataCreated`, `DatasetSubmittedForReview`, `DatasetReviewed`, `DatasetApproved`, `DatasetRejected` and `DatasetPublished`, with publishers preserved exactly as the parent specifies.

## 6. Failure / Rejection / Continuation
Validation/duplicate failure prevents review; review denial prevents approval; failed publication prerequisites retain the version outside `Published` and prohibit success-event publication.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-034-BH-009 | The system shall preserve validation results, review/approval decisions, lifecycle transitions, publication outcome, actor/Tenant context and historical lineage. | L3-A-034 §15.3 |

## 8. Observable Completion Conditions
One approved immutable Dataset Version is `Published`, discoverable, authorized for consumption, and fully traceable.

## 9. Traceability & Provenance
**Parent:** L3-A-034 — Governed Dataset Creation to Published Enterprise Availability  
**Primary verification source:** L2-A09 — Master Data & Reference Data Workflows

## 10. Open Issues / Deferred Design
Parent metadata contains `Depends On: undefined`; no replacement dependency set is invented here. Upstream metadata should be corrected before L4 lock. Technical publication mechanisms remain deferred.