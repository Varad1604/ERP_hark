# Governed Dataset Creation to Published Enterprise Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-034  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Master Data  
**Participating Domains:** A, VAR:Data-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** undefined  
**Referenced By:** Authorized data consumers, synchronization, export and localization  

---

# 2. Scenario Purpose

Create new governed Master Data or Reference Data and progress it through validation, review, approval, version governance and publication.

---

# 3. Business Outcome

The proposed data passes validation, stewardship review, approval, version governance and publication so an immutable approved version becomes discoverable and available to authorized consumers

Source assessment: `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A registered dataset requires new Master Data or Reference Data to become available for enterprise consumption.

## 4.2 End Boundary

An approved immutable Dataset Version is `Published`, discoverable and available to authorized consumers, with review, approval, publication and audit history.

## 4.3 In Scope

- Master or Reference Data draft creation.
- Schema, business-rule, uniqueness and quality validation.
- Submission, review and approval.
- Version and effective-date governance.
- Publication and consumer notification.

## 4.4 Out of Scope

- Transactional business processing and ownership of business semantics.
- Direct modification of immutable published or historical versions.
- Technical APIs, file formats, databases, UI, storage topology or deployment.
- Creating authentication, authorization or audit-storage mechanisms.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|

| Upstream | L3-A-033 | Requires a registered Dataset. |
| Followed by | L3-A-038 | Distributes published changes. |
| Related | L3-A-039 | Publishes localized representations. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|

| Master Data / Section A | Owns dataset identity, metadata, governance lifecycle, versions, publication, synchronization, localization and archival workflows. |
| Platform & Tenant / Section A | Supplies Tenant context and isolation. |
| Authorization & Policy / Section A | Supplies access and policy decisions without owning dataset state. |
| Workflow Engine / Section A | Coordinates review and approval workflows. |
| Event Infrastructure / Section A | Distributes canonical Master Data events. |
| Audit Infrastructure / Section A | Preserves immutable evidence. |
| Module Registry / Section A | Supplies module and ownership discovery references. |
| VAR:Data-Owning Domain | Owns dataset business meaning, constraints and canonical Business Owner. |

The frozen Participating Domains value remains exactly `A, VAR:Data-Owning Domain`.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|

| Business Owner | Defines business semantics, assigns stewards, approves structural changes and retirement. | Belongs to the owning business domain; does not own platform governance mechanics. |
| Data Steward | Maintains quality, metadata, corrections, review coordination and completeness. | Manages governed data but does not own business meaning. |
| Platform Administrator | Maintains governance, publication, validation and synchronization capabilities. | Does not approve business content. |
| Reviewer | Evaluates a submitted dataset and records approve/reject decisions. | Must hold valid review/approval authority. |
| Data Consumer | Uses published datasets and may subscribe or export when authorized. | Cannot modify datasets without explicit authority. |

---

# 7. Preconditions and Entry Criteria

- The Dataset is registered and owned.
- The actor is an authorized Data Steward.
- Applicable schema, metadata and business constraints exist.
- A Reviewer with valid authority can be assigned.
- Publication policy and effective dates can be evaluated.

---

# 8. Trigger

A registered dataset requires new Master Data or Reference Data to become available for enterprise consumption

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|

| 1 | Receive an authorized request to create Master Data or Reference Data. | Data Steward | A governed content lifecycle begins. |
| 2 | Validate Dataset registration, schema/category, code or key uniqueness and authorization. | Master Data Registry Service | The proposed content is eligible. |
| 3 | Apply business-rule, duplicate and quality validation. | Validation Service; Data Steward | Validation evidence is captured. |
| 4 | Create the data/version in `Draft` and publish the applicable creation event. | Master Data Registry Service | Draft governed content exists. |
| 5 | Verify completeness, lock the draft and transition `Draft → Review`. | Data Steward; Workflow Engine | `DatasetSubmittedForReview` is published. |
| 6 | Review metadata, content, validation results and dependencies. | Reviewer | Findings and an approve/reject decision are recorded. |
| 7 | On approval, transition `Review → Approved`, lock the version and prepare publication. | Reviewer; Workflow Engine | `DatasetApproved` is published. |
| 8 | Validate publication policy, effective dates and authorization. | Publication Service | Publication prerequisites succeed. |
| 9 | Activate the approved version and transition `Approved → Published`. | Publication Service | The version becomes immutable and enterprise-available. |
| 10 | Publish `DatasetPublished`, notify subscribers and record audit input. | Publication Service; Event and Audit Infrastructure | Discoverable governed availability is complete. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|

| Master Data record | Content is a core reusable business entity. | Use MD-02 creation before common review/approval/publication. | A published Master Data version results. |
| Reference value | Content is controlled Reference Data. | Use MD-04 creation before common governance. | A published Reference Data version results. |
| Review rejection | Reviewer rejects with a reason. | Return to Draft for correction and publish `DatasetRejected`. | No publication occurs. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|

| Schema/business validation fails | Draft cannot proceed to review. | Reject or retain Draft for correction. | Validation Service |
| Duplicate detected | Canonical integrity would be violated. | Reject proposed content. | Master Data Registry Service |
| Approval denied | Content cannot be published. | Return to Draft or remain under correction. | Reviewer |
| Publication prerequisite fails | Approved content cannot become available. | Remain Approved; publish no success event. | Publication Service |

No technical retry, rollback, storage or transport design is defined.

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|

| A Dataset cannot bypass Review or Approval. | Sequential governance is mandatory. | L2-A09 §§25.1, 26.1 |
| Publication requires validation, approval, policy, effective dates and authorization. | Any failed prerequisite blocks publication. | L2-A09 §26.3 |
| Published versions are immutable. | Further change requires a successor version. | L2-A09 §§25.2, 27.3 |
| Consumers access published data only. | Draft/Review/Approved content is not ordinarily consumable. | L2-A09 §27.2 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|

| Dataset | Registered/Draft | Progress `Draft → Review → Approved → Published`. | Published Dataset. |
| Dataset Version | Created/Draft | Validate → Approve → Publish. | Immutable Published version. |
| Master/Reference record | Proposed | Create, validate and include in governed version. | Available through published Dataset. |
| Publication | Pending | `Pending → Scheduled/Publishing → Published` as applicable. | Published history retained. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Condition |
|---|---|---|

| `MasterDataCreated` | Master Data Registry Service | A Master Data draft record is created. |
| `ReferenceDataCreated` | Master Data Registry Service | A Reference Data draft value is created. |
| `DatasetSubmittedForReview` | Workflow Engine | Draft enters Review. |
| `DatasetReviewed` | Workflow Engine | Review completes. |
| `DatasetApproved` | Workflow Engine | Approval succeeds. |
| `DatasetRejected` | Workflow Engine | Review rejects the content. |
| `DatasetPublished` | Publication Service | Approved version becomes available. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|

| Business content and constraints | VAR:Data-Owning Domain | Master Data | Authoritative semantics. |
| Validation outcome | Validation Service | Workflow Engine | Schema/business/quality result. |
| Approval decision | Reviewer / Workflow Engine | Publication Service | Approved version and evidence. |
| Authorization decision | Authorization & Policy | Master Data services | Permitted action and scope. |
| Published version | Publication Service | Authorized consumers | Immutable governed data. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Every modifying or disclosure action requires an applicable Authorization decision.
- Scope-based authorization and Tenant isolation apply.
- Only published data is available to ordinary consumers.
- Authentication and authorization remain owned by L2-A03 and L2-A04.

## 15.2 Approval and Separation of Duty

- Business content approval is performed by an authorized Reviewer/Business Owner path, not by the Platform Administrator.
- Publication requires successful validation, governance approval, policy evaluation, valid effective dates and authorization approval.
- Actor attribution and approval authority must be recorded.

## 15.3 Audit Obligations

- Record Tenant, actor, dataset/version, operation, prior state, resulting state, policy decision and correlation identifier.
- Preserve validation results, approval decisions, publication/distribution outcomes and historical lineage.
- A09 produces audit inputs; A07 owns immutable audit evidence and retention.

---

# 16. Completion Criteria and Postconditions

Completion requires:

- The appropriate data type was created in Draft.
- Validation and duplicate checks succeeded.
- Review completed with valid approval.
- The approved version was locked.
- Publication prerequisites and effective dates succeeded.
- The version became Published and immutable.
- Canonical events and audit evidence are available.

Postconditions:

- Authorized consumers can discover and use the Published version.
- No historical version is overwritten.
- The owning domain retains business meaning.

---

# 17. Open Issues, Assumptions and Deferred Questions


No open issue prevents this scenario from proceeding to Architecture Approval. No unsupported actor, state, transition, event, publisher or implementation mechanism is assumed.


