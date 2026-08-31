# Notification Template Request to Governed Published Template

## 1. Specification Metadata

**Specification ID:** L4-H-012  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Notification  
**Realizes Scenario:** L3-H-012  
**Primary Business Owner:** Notification  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-H-012_Notification_Template_Request_to_Governed_Published_Template.md`; L2-H02 — Notification Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern Notification Template version creation, validation, review, revision, publication eligibility, deprecation and retirement using the Locked H02 Template lifecycle while preserving published-version immutability and source-domain ownership of business rules. The behavior stops before asserting a valid approval decision because H02 defines the mandatory approval lifecycle step but no canonical approving business actor or authority.

## 3. Required Business Context

- New or revised communication content exists within H02 Template governance scope.
- Template metadata, variables and applicable channel/localization context are available for validation.
- Template content does not acquire ownership of source-domain business rules or decisions.
- Published Template versions are immutable; revisions require a new governed version.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-012-BH-001 | The system shall establish a new governed Template version in `Draft` for new or revised communication content. | L3-H-012 §§2,9,13 |
| L4-H-012-BH-002 | The system shall validate Template metadata, declared variables and applicable channel/localization context before progression beyond Draft. | L3-H-012 §§7,9,12 |
| L4-H-012-BH-003 | The system shall reject Template content that embeds executable business logic or attempts to own a source-domain business decision. | L3-H-012 §§4–5,12 |
| L4-H-012-BH-004 | The system shall apply only the canonical Template lifecycle `Draft → Under Review → Approved → Published → Deprecated → Retired` and the source-defined review return paths. | L3-H-012 §§9–10,13 |
| L4-H-012-BH-005 | The system shall preserve a Published Template version as immutable; any later content revision shall be represented by a successor version rather than mutation of the Published version. | L3-H-012 §§2,10–12,16 |
| L4-H-012-BH-006 | The system shall not assert that a Template has validly entered `Approved` or `Published` unless a canonical approval authority has been defined upstream. | L3-H-012 §§6,9,11,15–17 |

## 5. Cross-Domain & Event Behavior

H02 owns the Template lifecycle. Source business domains may supply communication content or business meaning, but those domains retain ownership of the business facts/rules. Locked H02 does not define canonical Template approval, publication, deprecation or retirement events, so L4 shall not invent Template event names or publishers.

## 6. Failure / Rejection / Continuation

Invalid Template metadata or variables shall prevent progression toward publication. A version requiring revision may return to `Draft` through the source-defined review branch. A Published version requiring correction shall remain historically preserved and be superseded by a new version. Missing approval authority shall keep the version from being treated as validly Approved/Published.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-012-BH-007 | The system shall preserve Template identity, version, validation result, review position, revision history, publication/deprecation/retirement lifecycle evidence where validly reached, and the unresolved approval-authority condition. | L3-H-012 §§15–16 |

## 8. Observable Completion Conditions

The revision-return path is complete when the version is validly returned to `Draft` with its review history preserved. The approved-and-published branch is not fully observable as canonical completion until upstream architecture defines the approving business authority and any required event/evidence contract for approval/publication.

## 9. Traceability & Provenance

**Parent:** L3-H-012 — Notification Template Request to Governed Published Template  
**Primary verification source:** L2-H02 — Notification Workflows  
**Later amendment check:** No later Locked H02 amendment defining Template approval authority or Template lifecycle events was found in the live Section H L2 directory.

## 10. Open Issues / Deferred Design

The canonical Template approver/approval authority remains undefined. H02 also defines no canonical Template approval/publication/deprecation/retirement event vocabulary or publisher. No separation-of-duty rule is invented. Provider, rendering engine, persistence and UI implementation remain deferred.
