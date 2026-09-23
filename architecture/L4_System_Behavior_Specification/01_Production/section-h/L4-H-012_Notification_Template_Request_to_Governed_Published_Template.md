# Notification Template Request to Governed Published Template

## 1. Specification Metadata
**Specification ID:** L4-H-012  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Notification  
**Realizes Scenario:** L3-H-012  
**Primary Business Owner:** Notification  
**Version:** 1.1.1  
**Status:** Architecture Approved  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** L3-H-012; L2-H02 Notification Template Approval Amendment

## 2. Behavioral Objective & Boundary
Make Template review/approval/publication observable while preserving immutable published versions.

## 3. Required Business Context
H02 owns Template lifecycle; Notification Template Approval Authority owns review/approval/publication eligibility.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior |
|---|---|
| L4-H-012-BH-001 | Create/revise a Template as a distinct version in `Draft`. |
| L4-H-012-BH-002 | Validate metadata, variables and content constraints before `Under Review`. |
| L4-H-012-BH-003 | Permit the Approval Authority to return `Under Review → Draft` or approve `Under Review → Approved`. |
| L4-H-012-BH-004 | Permit `Approved → Published` only after the authorized approval decision. |
| L4-H-012-BH-005 | Preserve Published versions immutably; content changes create new versions. |
| L4-H-012-BH-006 | Support later `Published → Deprecated → Retired` under H02 governance. |

## 5. Cross-Domain & Event Behavior
No new Template event is required; source business rules remain owned by their originating domains.

## 6. Failure / Rejection / Continuation
Invalid content/metadata or absent approval prevents publication.

## 7. Audit & Evidence Behavior
Preserve Template version, validation, authority, review/approval/publication and retirement evidence.

## 8. Observable Completion Conditions
Template is Published or validly returned for revision; historical published use remains reconstructable.

## 9. Traceability & Provenance
Parent: L3-H-012; governing H02 amendment dated 2026-09-01.

## 10. Open Issues / Deferred Design
None at L4.


## Canonical Validation Note

This specification passed the scoped R2B hard-fail validation against the Locked L4 Governance and Architecture Standard and L4 Validation Standard and is `Architecture Approved` under `L4_R2B_Notification_Authority_Architecture_Approval_Decision_2026-09-23.md`. Existing non-blocking upstream gaps remain bounded exactly as recorded; this approval does not invent universal actors, lifecycle states, events, publishers, provider mechanics, technical retry behavior, or implementation design.
