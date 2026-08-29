# Published Dataset Change, Quality Deficiency, or Historical Restoration to Governed Successor Version Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Master Data  
**Participating Domains:** A, VAR:Data-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** undefined  
**Referenced By:** Dataset consumers, quality governance, historical restoration and synchronization  

---

# 2. Scenario Purpose

Apply an approved change, quality correction or historical restoration by creating and publishing a governed successor rather than modifying an immutable Published version.

---

# 3. Business Outcome

A governed successor version is created from the current or selected historical version, validated against applicable quality, restoration and governance policy, reviewed, approved, effective-dated where applicable and published, while every previous version and historical reference remains immutable and traceable

Source assessment: `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A published dataset requires an approved business, structural, or data-quality correction, or an authorized governance need selects a historical version as the basis for restored use.

## 4.2 End Boundary

A successor Dataset Version is Published with complete lineage to its source, while every preceding and historical version remains immutable and traceable.

## 4.3 In Scope

- Business, structural and quality-correction successor creation.
- Historical-version selection and restoration policy.
- Lineage, effective dating and version numbering.
- Validation, review, approval and publication.
- Immutable history and successor events.

## 4.4 Out of Scope

- Transactional business processing and ownership of business semantics.
- Direct modification of immutable published or historical versions.
- Technical APIs, file formats, databases, UI, storage topology or deployment.
- Creating authentication, authorization or audit-storage mechanisms.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|

| Upstream | L3-A-034 | Supplies a Published version. |
| Followed by | L3-A-038 | Synchronizes the published successor. |
| Related | L3-A-040 | Retires datasets rather than restoring versions. |

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

- A current or historical Dataset Version exists.
- The actor is an authorized Steward or Platform Administrator for restoration.
- The requested change or restoration basis is documented.
- Version and restoration policies permit a successor.
- Effective periods can be validated without overlap.

---

# 8. Trigger

A published dataset requires an approved business, structural, or data-quality correction, or an authorized governance need selects a historical version as the basis for restored use

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|

| 1 | Classify the request as current-version change, quality correction or historical restoration. | Data Steward; Business Owner | The applicable source version and policy path are known. |
| 2 | Select the current or historical source version without modifying it. | Version Management Service | The immutable baseline is fixed. |
| 3 | Validate authorization, restoration policy where applicable, and business rationale. | Authorization & Policy; Version Management Service | Successor creation is permitted. |
| 4 | Create a uniquely numbered successor Draft and record predecessor/restoration lineage. | Version Management Service | `DatasetVersionCreated` becomes publishable. |
| 5 | Apply approved modifications or restore the selected historical content as the successor basis. | Data Steward | The proposed successor content is assembled. |
| 6 | Execute schema, quality, duplicate, consistency and effective-period validation. | Validation Service | Validation evidence is complete. |
| 7 | Submit, review and approve the successor through the common governance lifecycle. | Workflow Engine; Reviewer | The successor reaches Approved. |
| 8 | Validate publication policies and activate the successor. | Publication Service | The successor reaches Published and becomes immutable. |
| 9 | Publish the appropriate version/restoration and publication events. | Version Management Service; Publication Service | Consumers receive authoritative completed facts. |
| 10 | Preserve all source versions, references, lineage and audit history. | Master Data; Audit Infrastructure | Historical integrity remains intact. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|

| Current change | The current Published version is the source. | Create the successor from the current version. | Normal successor lineage is recorded. |
| Quality correction | A deficiency requires corrected content. | Record quality findings and apply correction only in the successor. | Original evidence remains intact. |
| Historical restoration | An authorized historical version is selected. | Validate restoration policy and create a successor from it. | `HistoricalVersionRestored` is published after governed reuse. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|

| Source version missing | Successor lineage cannot be established. | Reject request. | Version Management Service |
| Restoration policy denies reuse | Historical content cannot return to governed use. | Preserve history and deny restoration. | Version Management Service |
| Validation or approval fails | Successor cannot publish. | Retain Draft/Review outcome for correction. | Workflow Engine |
| Effective periods overlap | Version governance is invalid. | Block publication until dates are corrected. | Publication Service |

No technical retry, rollback, storage or transport design is defined.

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|

| Published versions are immutable. | All change/restoration creates a successor. | L2-A09 §§25.2, 26.2 |
| Version identifiers are unique and lineage permanent. | Successor history is reconstructible. | L2-A09 §27.3 |
| Active effective periods cannot overlap. | Publication requires valid dates. | L2-A09 §§9, 27.3 |
| Historical restoration does not reactivate or modify the historical version. | A new successor represents restored use. | L2-A09 MD-17 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|

| Source Version | Published or historical | Referenced only; no mutation. | Remains immutable. |
| Successor Version | Absent | Created → Draft → Validated → Approved → Published. | Immutable Published successor. |
| Dataset | Published | Its active/published version reference advances. | Remains Published. |
| Historical lineage | Existing | Add predecessor/restoration relationship. | Permanently traceable. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Condition |
|---|---|---|

| `MasterDataUpdated` | Master Data Registry Service | A governed update produces a pending successor. |
| `DatasetVersionCreated` | Version Management Service | A successor Draft is created. |
| `HistoricalVersionRestored` | Version Management Service | Approved historical reuse produces a successor. |
| `DatasetSubmittedForReview` | Workflow Engine | Successor enters Review. |
| `DatasetApproved` | Workflow Engine | Successor approval succeeds. |
| `DatasetPublished` | Publication Service | Successor becomes enterprise-available. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|

| Change/restoration rationale | VAR:Data-Owning Domain | Version Management Service | Business need and source selection. |
| Validation result | Validation Service | Workflow Engine | Quality/schema/effective-date evidence. |
| Approval decision | Reviewer | Publication Service | Approved successor. |
| Published successor | Publication Service | Consumers | New authoritative version. |
| Lineage/audit input | Master Data | Audit Infrastructure | Source, successor and governance history. |

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

- A valid immutable source version was selected.
- A uniquely identified successor Draft with lineage was created.
- Applicable restoration, quality, version and effective-date rules succeeded.
- Review and approval completed.
- The successor became Published and immutable.
- Version/restoration/publication events and audit evidence are available.
- All earlier versions and references remain unchanged.

Postconditions:

- Consumers use the new Published version according to effective dating.
- Historical versions remain read-only and searchable.
- No source business meaning is transferred to the platform.

---

# 17. Open Issues, Assumptions and Deferred Questions


No open issue prevents this scenario from proceeding to Architecture Approval. No unsupported actor, state, transition, event, publisher or implementation mechanism is assumed.


