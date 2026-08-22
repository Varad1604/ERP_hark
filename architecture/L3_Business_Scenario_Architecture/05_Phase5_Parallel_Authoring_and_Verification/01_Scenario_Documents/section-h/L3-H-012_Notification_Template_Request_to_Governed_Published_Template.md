# Notification Template Request to Governed Published Template

# 1. Scenario Metadata

**Scenario ID:** L3-H-012  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Notification  
**Participating Domains:** H  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** L2-H02 — Notification Workflows  
**Referenced By:** Notification creation, template rendering, localization, template revision, deprecation, retirement, and audit activities

# 2. Scenario Purpose

Govern creation or revision of an H02 Notification Template through validation, review, approval, publication, deprecation, and retirement while preserving immutable published versions and preventing business logic from being embedded in notification content.

# 3. Business Outcome

A validated template version is approved and published, or returned for revision, with deprecation and retirement preserving prior use

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when new or revised communication content requires governance.

## 4.2 End Boundary

The frozen end boundary requires a validated template version to be approved and published or returned for revision, with later deprecation/retirement preserving prior use. Locked H02 defines the Template lifecycle but does not define the business actor or authority that performs the required approval decision, so the approval branch cannot be fully grounded.

## 4.3 In Scope

- H02 Notification Template identity, metadata, variables, localization, and version governance.
- Validation before governed review/approval.
- Source-defined Template lifecycle from Draft through Under Review, Approved, Published, Deprecated, and Retired.
- Return from review to Draft for revision.
- Immutability of published Template versions and creation of new versions for revisions.
- Preservation of audit evidence and prior published use.

## 4.4 Out of Scope

- Source-domain business decisions or business-rule ownership.
- Technical rendering implementation or provider-specific formatting mechanics.
- Inventing a template approver, approval event, or separation-of-duty rule absent from locked H02.
- Individual Notification delivery outcomes.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Notification / Section H | Owns Notification Template lifecycle, versioning, validation, approval state, publication, deprecation, retirement, localization, variables, and template auditability. |

Business domains may own business content supplied for communication, but the frozen scenario itself has H as its only participating domain and H02 owns Template lifecycle governance.

# 6. Business Actors, Roles and Responsibilities

Locked H02 explicitly has no canonical Actors section. Although the Template lifecycle requires review and approval, the canonical business actor who submits, reviews, approves, returns, deprecates, or retires a Template is not defined. No placeholder role such as “Template Approver” or “Reviewer” is introduced.

# 7. Preconditions and Entry Criteria

- New or revised communication content exists for H02 template governance.
- Required Template metadata, channel, language, and variable definitions are available for validation.
- Business content remains owned by its source business context and does not transfer business-rule ownership to H02.

# 8. Trigger

New or revised communication content requires governance

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish a new Template version in the source-defined `Draft` state with its H02-owned Template identity/version context. | Notification | Governed Template version exists. |
| 2 | Validate Template metadata, supported variables, channel/localization context, and the rule that rendering contains no executable business logic. | Notification | Invalid Template content is prevented from progressing. |
| 3 | Move the validated Template version to `Under Review`. | Notification | Template is ready for the required governance decision. |
| 4 | Apply the source-defined review outcome: return `Under Review` to `Draft` for revision or move to `Approved`. | Notification | Lifecycle branch is source-defined, but the business decision authority is missing upstream. |
| 5 | If approved, move the immutable version to `Published` and make it available for H02 Notification use. | Notification | Published Template version becomes effective. |
| 6 | When later governance requires withdrawal from normal future use, move the source-defined published version through `Deprecated` to `Retired` without rewriting historical use. | Notification | Prior Template version and Notification references remain traceable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Review requires revision | Under-review Template is not ready for approval. | Return `Under Review` to `Draft` using the source-defined lifecycle. | Template is revised without publishing the current version. | Notification |
| Approved version needs further review before publication | An approved version requires governance reconsideration. | Use the source-defined `Approved` to `Under Review` transition. | Version is not published until governance is satisfied. | Notification |
| Published content needs change | A published Template must be revised. | Create a new Template version; do not modify the published version. | Historical published version remains immutable. | Notification |
| Template reaches end of future use | Published Template is being phased out. | Move `Published` to `Deprecated` and then `Retired`. | Prior use remains traceable. | Notification |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required variables or metadata are invalid | Template cannot progress safely. | Keep/return the Template in editable governance before publication. | Notification |
| Published Template requires correction | Published version cannot be modified. | Create a new version and preserve the original published version. | Notification |
| Approval authority cannot be established | Required business approval cannot be attributed to an authoritative actor. | Stop short of claiming a valid approval decision and escalate the architecture gap. | Notification |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Published templates are immutable and shall never be modified. | Notification | Corrections require a new Template version. | L2-H02 §14.4 |
| New revisions create new Template versions. | Notification | Version lineage is preserved. | L2-H02 §14.4 |
| Variables must be validated before rendering. | Notification | Invalid variable definitions prevent valid publication/use. | L2-H02 §§14.4, 16.3 |
| Template rendering shall never execute business logic. | Notification / source business domain | H02 Template cannot become a new owner of business rules. | L2-H02 §§6, 14.4 |
| Every published Template remains auditable. | Notification | Publication/deprecation/retirement history must remain traceable. | L2-H02 §14.4 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Notification Template version | `Draft` | Validate and submit for review. | `Under Review` or remains/returns `Draft`. | Notification | L2-H02 §§14–15 |
| Notification Template version | `Under Review` | Governance decision required. | `Approved` or `Draft`. | Notification | L2-H02 §15 |
| Notification Template version | `Approved` | Governed publication or reconsideration. | `Published` or `Under Review`. | Notification | L2-H02 §15 |
| Notification Template version | `Published` | Available for governed use; immutable. | `Deprecated` when future retirement begins. | Notification | L2-H02 §§14.4, 15 |
| Notification Template version | `Deprecated` | Continue historical traceability while ending future use. | `Retired`. | Notification | L2-H02 §15 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Locked H02's Standard Events catalogue defines Notification lifecycle events but does not define canonical Template lifecycle events such as template-approved, template-published, template-deprecated, or template-retired. L3 therefore does not invent Template events or publishers.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The H02 Template lifecycle is internal to the Notification bounded context for this Simple scenario. Published Template versions are referenced by Notifications; business source data used as runtime variables remains source-owned.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Template lifecycle requires a review and approval decision before `Published`, but locked H02 does not define the canonical business actor or approval authority. No local approval role is created.

## 15.2 Separation of Duties, Overrides and Exceptions

No source-supported separation-of-duty or override rule is defined for Template review/approval. L3 cannot determine whether submission, review, approval, deprecation, and retirement require distinct actors.

## 15.3 Business Audit and Evidence Obligations

H02 must preserve Template identifier, immutable version, category, channel, language/localization, variable definitions, approval status, effective/expiry context, review/publication/deprecation/retirement lifecycle history, and historical Notification references to published versions.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The source-defined lifecycle and validation rules are explicit, but the frozen business outcome cannot be considered fully complete until an authoritative business actor/authority is defined for the required approval decision. A returned-for-revision outcome is lifecycle-supported; a valid approved-and-published outcome lacks explicit approval authority.

## 16.2 Postconditions

- Published Template versions, once validly approved, are immutable.
- Revised content requires a new Template version.
- Deprecated/Retired versions preserve prior Notification references and audit history.
- No Template approval actor or event is implied beyond locked H02.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked H02 explicitly has no Actors section, while the Template lifecycle requires review and approval before `Published`. | The actor/authority for a mandatory business approval is absent, which materially affects the frozen approved-and-published outcome. | Define the canonical Template submission/review/approval authority upstream. |
| H02 defines no canonical Template lifecycle events or publishers. | L3 cannot publish or consume template-approved/published/deprecated/retired events without invention. | Add Template lifecycle events upstream only if cross-domain event collaboration is required. |
| H02 defines no explicit separation-of-duty requirement for Template submission, review, approval, deprecation, or retirement. | L3 cannot infer whether the same or different actors may perform these decisions. | Clarify separation-of-duty only if institutional governance requires it. |

**Architecture assessment:** `BLOCKED`

The Template lifecycle, immutability, versioning, validation, publication, deprecation, and retirement rules are explicit. However, the frozen outcome requires approval and locked H02 provides no canonical actor or approval authority. Because approval authority is material, the scenario requires upstream resolution before it can be a complete source-grounded Draft.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
