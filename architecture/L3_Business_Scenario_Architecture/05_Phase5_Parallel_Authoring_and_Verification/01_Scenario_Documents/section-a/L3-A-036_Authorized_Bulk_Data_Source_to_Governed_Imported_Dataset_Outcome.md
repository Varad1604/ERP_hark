# Authorized Bulk Data Source to Governed Imported Dataset Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-A-036  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Master Data  
**Participating Domains:** A, VAR:Data-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** undefined  
**Referenced By:** Bulk onboarding, migration, data quality and governance review  

---

# 2. Scenario Purpose

Accept an authorized bulk source only after structural, semantic, duplicate and business validation, preserving validation and audit evidence.

---

# 3. Business Outcome

Imported content is structurally and semantically validated, duplicate-checked, accepted into the governed dataset lifecycle only when valid, and retained with validation and audit evidence

Source assessment: `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized source submits Master Data or Reference Data for controlled enterprise import.

## 4.2 End Boundary

Valid imported content is recorded in the governed Dataset lifecycle with retained validation report and audit history, or the import is rejected without committing governed changes.

## 4.3 In Scope

- Import-source authorization.
- Structure, schema, duplicate and business validation.
- Preview and controlled acceptance.
- Dataset lifecycle entry.
- Validation report, event and audit evidence.

## 4.4 Out of Scope

- Transactional business processing and ownership of business semantics.
- Direct modification of immutable published or historical versions.
- Technical APIs, file formats, databases, UI, storage topology or deployment.
- Creating authentication, authorization or audit-storage mechanisms.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|

| Upstream | L3-A-033 | Requires a registered governed Dataset where applicable. |
| Followed by | L3-A-034 | Provides review, approval and publication for imported Draft content. |

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
| Authorized Import Operator | Submits and reviews the governed import source. | May import only within approved scope and format. |

---

# 7. Preconditions and Entry Criteria

- The source and operator are authorized.
- The target Dataset and owning domain are resolved.
- The import format and schema are supported.
- Business validation and duplicate rules are available.
- Tenant isolation can be enforced.

---

# 8. Trigger

An authorized source submits Master Data or Reference Data for controlled enterprise import

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|

| 1 | Receive the source and validate operator, Tenant and target scope. | Authorized Import Operator; Authorization & Policy | An eligible import begins. |
| 2 | Validate file/content structure and supported format. | Import and Export Service | The source is structurally readable. |
| 3 | Validate the target schema and canonical metadata mapping. | Validation Service | Structural semantics align to the Dataset. |
| 4 | Detect duplicates against the governed Dataset. | Master Data Registry Service | Duplicate risk is identified. |
| 5 | Execute business and data-quality validation. | Validation Service; VAR:Data-Owning Domain | Semantic validity is determined. |
| 6 | Generate a preview and validation report. | Import and Export Service | The operator can assess the controlled outcome. |
| 7 | Reject invalid content without committing governed changes, or accept valid records into the Dataset lifecycle. | Import and Export Service; Master Data Registry Service | Only valid content enters governance. |
| 8 | Publish `DatasetImported` after successful acceptance. | Import and Export Service | The completed import fact is available. |
| 9 | Retain the validation report and audit input. | Master Data; Audit Infrastructure | The outcome is traceable. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|

| All records valid | Every submitted record satisfies validation. | Accept the governed import. | Imported content enters the lifecycle. |
| Validation failure | The batch fails required validation. | Reject without committed governed changes. | Failure evidence is retained. |
| Duplicate findings | Potential duplicates are detected. | Resolve under owning-domain rules before acceptance. | No unsupported merge is performed. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|

| Unauthorized source/operator | Import boundary is violated. | Reject before processing. | Authorization & Policy |
| Unsupported format | Structural validation cannot proceed. | Reject and record reason. | Import and Export Service |
| Schema/business validation fails | Content is not governable. | Reject; commit no governed changes. | Validation Service |
| Duplicate resolution unavailable | Canonical integrity is uncertain. | Block acceptance pending governance. | VAR:Data-Owning Domain |

No technical retry, rollback, storage or transport design is defined.

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|

| Only authorized sources may import. | Authorization precedes content acceptance. | L2-A09 MD-13 |
| Invalid imports commit no governed data changes. | Failure preserves the prior Dataset. | L2-A09 MD-13 |
| Duplicate detection precedes acceptance. | Canonical integrity is protected. | L2-A09 §§35, MD-13 |
| Imported content remains subject to the Dataset lifecycle. | Import does not bypass review, approval or publication. | L2-A09 §§7, 25 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|

| Target Dataset | Registered/governed | Receives valid imported content under its lifecycle. | Remains governed. |
| Imported content | External/proposed | Validate and accept or reject. | Draft/governed content or no change. |
| Validation Report | Absent | Create from structural/semantic checks. | Retained evidence. |
| Published Version | Existing or none | Not overwritten by import. | Unchanged until separate publication. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Condition |
|---|---|---|

| `DatasetImported` | Import and Export Service | Valid content is accepted into the governed platform. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|

| Authorized source and scope | Authorization & Policy / source domain | Import and Export Service | Import authority and Tenant boundary. |
| Schema and business rules | Dataset / VAR:Data-Owning Domain | Validation Service | Structural and semantic constraints. |
| Validation outcome | Validation Service | Import and Export Service | Accept/reject evidence. |
| Governed records | Import and Export Service | Master Data Registry Service | Validated content. |
| Audit input | Import and Export Service | Audit Infrastructure | Source, actor, validations and outcome. |

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

- Source, operator, Tenant and target were authorized.
- Structure, schema, duplicates and business rules were validated.
- A validation preview/report was produced.
- Only valid content was accepted into the governed lifecycle.
- `DatasetImported` was published after success.
- Validation and audit evidence were retained.

Postconditions:

- Invalid content produces no governed data change.
- Imported content is not automatically Published unless the publication lifecycle completes.
- Business meaning remains with the owning domain.

---

# 17. Open Issues, Assumptions and Deferred Questions


No open issue prevents this scenario from proceeding to Architecture Approval. No unsupported actor, state, transition, event, publisher or implementation mechanism is assumed.


