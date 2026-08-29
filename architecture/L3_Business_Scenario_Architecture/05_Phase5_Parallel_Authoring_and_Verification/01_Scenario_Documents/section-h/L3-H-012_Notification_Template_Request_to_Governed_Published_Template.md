# Notification Template Request to Governed Published Template

# 1. Scenario Metadata

**Scenario ID:** L3-H-012  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Notification  
**Participating Domains:** H  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** L2-H02 — Notification Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern a new or revised H02 Notification Template through validation, review, approval, publication, revision, deprecation, and retirement while preserving immutable published versions and keeping business rules outside template content.

# 3. Business Outcome

A validated template version is approved and published, or returned for revision, with deprecation and retirement preserving prior use

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

New or revised communication content requires governance

## 4.2 End Boundary

The frozen outcome requires a validated Template version to be approved and published or returned for revision, with later deprecation/retirement preserving prior use. The approval branch cannot be completed authoritatively because the approval actor/authority is undefined upstream.

## 4.3 In Scope

- H02 Template identity, versioning and validation.
- The locked Template review, approval, publication, deprecation and retirement lifecycle.
- Immutability of published versions and creation of new versions for revisions.

## 4.4 Out of Scope

- Ownership of source-domain business rules or business decisions.
- Inventing a Template approver, approval event, or separation-of-duty rule.
- Individual Notification delivery outcomes.

# 5. Ownership and Participating Domains

Notification / Section H owns Notification Template identity, lifecycle, versioning, validation, publication, localization, deprecation, retirement, and template governance. Source business domains retain business rules and business meaning.

# 6. Business Actors, Roles and Responsibilities

Locked H02 explicitly has no canonical Actors section. It defines a mandatory review/approval lifecycle but names no business actor who submits, reviews, approves, returns, deprecates, or retires a Template. No placeholder approver is introduced.

# 7. Preconditions and Entry Criteria

- New or revised communication content exists.
- Required Template metadata, channel/localization context, and variable definitions are available for validation.
- Business content remains source-owned and does not transfer business-rule ownership to H02.

# 8. Trigger

New or revised communication content requires governance

# 9. Main Business Flow

1. H02 creates a new Template version in `Draft`.
2. H02 validates metadata, variables, localization/channel context, and the prohibition on executable business logic.
3. The version moves to `Under Review`.
4. The source-defined lifecycle permits return to `Draft` for revision or movement to `Approved`, but the business authority for that decision is undefined upstream.
5. If valid approval were authoritatively established, `Approved → Published` would make the immutable version available for Notification use.
6. Later governance may move `Published → Deprecated → Retired`, preserving prior use and history.

# 10. Alternate Business Flows

- **Revision required:** `Under Review → Draft`.
- **Reconsideration before publication:** `Approved → Under Review`.
- **Change to published content:** create a new version; do not modify the published version.
- **End of future use:** `Published → Deprecated → Retired`.

# 11. Failure, Interruption and Business Recovery

Invalid metadata/variables keep the version from publication. A published version needing correction is superseded by a new version. Because the approval actor/authority is missing, L3 must stop before claiming a valid approval decision and record the upstream blocker.

# 12. Business Rules and Decision Constraints

- Template Management owns creation, validation, approval, publication, localization, versioning, retirement, and operational use.
- Published Template versions are immutable.
- Revisions create new Template versions.
- Variables must be validated before rendering.
- Templates shall not contain executable business logic.
- Templates follow governed approval processes.

# 13. State and Lifecycle Impact

Canonical Template lifecycle: `Draft → Under Review → Approved → Published → Deprecated → Retired`, with `Under Review → Draft` and `Approved → Under Review`. L3 uses only these states and transitions.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H02's Standard Events catalogue does not define canonical Template approval, publication, deprecation, or retirement events. No Template event or publisher is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The Template lifecycle is H02-owned. Source business content may inform Template content, but business rules and decisions remain with their canonical source owners.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The locked Template lifecycle requires review and approval before `Published`, but H02 defines no canonical approving actor or authority.

## 15.2 Separation of Duties, Overrides and Exceptions

No source-supported separation-of-duty rule is defined for Template submission, review, approval, deprecation, or retirement.

## 15.3 Business Audit and Evidence Obligations

Preserve Template version/lifecycle and historical-use evidence required by H02. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The validated version reaches a source-defined review outcome.
- A valid approval authority exists for any `Approved → Published` decision.
- Published versions remain immutable and later deprecation/retirement preserves prior use.

## 16.2 Postconditions

- The returned-for-revision path is valid now.
- The approved-and-published path remains unresolved until approval authority is defined upstream.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- The mandatory Template approval decision has no canonical business actor or approval authority in locked H02.
- H02 defines no canonical Template lifecycle events/publishers.
- No source-supported separation-of-duty rule is defined for Template review/approval.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream architecture must define the Template approval authority and any required approval event/evidence contract.
