# Retired Dataset to Governed Archival

# 1. Scenario Metadata

**Scenario ID:** L3-A-041  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Master Data  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** undefined  
**Referenced By:** Governance, audit, compliance, historical reconstruction and catalog discovery  

---

# 2. Scenario Purpose

Move an eligible Retired Dataset into governed archival while preserving all metadata, version, audit and historical-reference integrity.
---

# 3. Business Outcome

The retired dataset is moved into governed archival status while preserving metadata, version history, audit history, historical references and searchability required for governance and historical use

Source assessment: `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A retired dataset becomes eligible for long-term archival.

## 4.2 End Boundary

The Dataset and its content are in governed archival status/catalog, remain searchable for authorized governance and audit use, and retain complete immutable history.

## 4.3 In Scope

- Archive-eligibility confirmation.
- Metadata, version, audit and reference preservation.
- Long-term archival transition.
- Archive catalog update and searchability.
- Archival event and audit evidence.

## 4.4 Out of Scope

- Transactional-domain processing and business-semantic ownership.
- Modification of immutable Published or historical versions.
- Technical broker, retry scheduler, storage, API, database, UI or deployment design.
- Creation of authentication, authorization or audit-storage mechanisms.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|

| Upstream | L3-A-040 | Supplies the Retired Dataset. |
| Related | L3-A-023 | Uses audit evidence for investigations, not Dataset ownership. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|

| Master Data / Section A | Owns Dataset identity, versions, publication, synchronization, localization, retirement and archival governance. |
| Platform & Tenant / Section A | Supplies Tenant context and isolation. |
| Authorization & Policy / Section A | Supplies access and policy decisions. |
| Workflow Engine / Section A | Coordinates review and approval. |
| Event Infrastructure / Section A | Distributes canonical events. |
| Audit Infrastructure / Section A | Owns immutable audit evidence. |
| Module Registry / Section A | Supplies module and ownership discovery references. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|

| Business Owner | Owns business semantics, structural approval and retirement approval. | Belongs to the data-owning domain. |
| Data Steward | Maintains quality, corrections, metadata and governance coordination. | Does not own business meaning. |
| Platform Administrator | Operates governance, publication and synchronization capabilities. | Does not approve business content. |
| Reviewer | Records authorized review and approval decisions. | Requires valid approval authority. |
| Data Consumer | Consumes or subscribes to Published data. | Cannot modify the governed source. |

---

# 7. Preconditions and Entry Criteria

- The Dataset is `Retired`.
- Archive eligibility is confirmed.
- The Platform Administrator is authorized.
- Metadata, versions, historical references and audit history are complete.
- Applicable retention and preservation requirements can be satisfied.

---

# 8. Trigger

A retired dataset becomes eligible for long-term archival
---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|

| 1 | Receive the archival eligibility outcome for the Retired Dataset. | Platform Administrator | A governed archival request begins. |
| 2 | Resolve the Dataset, all versions, metadata, references and audit-history links. | Master Data | The complete archival scope is established. |
| 3 | Validate Retired status, authorization and archive eligibility. | Publication Service; Authorization & Policy | Only eligible content may proceed. |
| 4 | Verify preservation obligations and historical search requirements. | Platform Administrator; Audit Infrastructure | Archival controls are confirmed. |
| 5 | Preserve canonical metadata and stewardship/ownership history. | Master Data | Governance meaning remains available. |
| 6 | Preserve every Dataset Version and predecessor/successor relationship. | Version Management Service | Version history remains reconstructible. |
| 7 | Preserve audit-history and historical-reference links. | Master Data; Audit Infrastructure | Evidence relationships remain intact. |
| 8 | Transition the Dataset/content into archival status. | Publication Service | Operational Retired content becomes archived. |
| 9 | Update the archive catalog and authorized searchability. | Dataset Catalog Service | Governance can locate the archived Dataset. |
| 10 | Publish `DatasetArchived` and record audit input. | Publication Service; Event and Audit Infrastructure | The archival outcome is complete and traceable. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|

| Archival deferred | Eligibility or preservation evidence is incomplete. | Keep the Dataset Retired until requirements are satisfied. | No archive event is published. |
| Restricted historical access | Policy limits access after archival. | Apply authorization at retrieval while retaining search metadata as permitted. | Archived content remains unchanged. |
| Multiple versions | The Dataset has many historical versions. | Archive the governed scope while preserving complete lineage. | No version is overwritten. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|

| Dataset not Retired | Archive precondition fails. | Reject archival. | Publication Service |
| Eligibility unconfirmed | Long-term transition is unauthorized. | Remain Retired. | Platform Administrator |
| Preservation validation fails | History could be lost. | Block archival until corrected. | Master Data |
| Catalog update fails | Searchability requirement is incomplete. | Do not assert completed archival. | Dataset Catalog Service |

No technical retry, rollback, storage or transport mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|

| Only Retired Datasets are eligible for MD-18 archival. | Lifecycle order is preserved. | L2-A09 MD-18 |
| Archived history remains searchable for governance and audit. | Archival is not deletion. | L2-A09 MD-18 |
| Published/historical versions remain immutable. | Archival changes storage/governance position, not content. | L2-A09 §§25.2, 39.2 |
| Audit and historical references remain intact. | Traceability survives archival. | L2-A09 MD-18; §37 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|

| Dataset | Retired/read-only | Move to governed archival status. | Archived. |
| Dataset Versions | Historical/immutable | Preserve and archive without mutation. | Archived/traceable. |
| Archive Catalog | No entry | Add governed Dataset and discovery metadata. | Searchable entry. |
| Audit History | Existing | Preserve link/reference; no evidence mutation. | Intact. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Publisher | Condition |
|---|---|---|

| `DatasetRetired` | Publication Service | Consumed as an upstream prerequisite. |
| `DatasetArchived` | Publication Service | Archival and catalog update complete. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|

| Retired Dataset scope | Master Data Registry Service | Publication Service | Dataset, versions, metadata and references. |
| Archive eligibility | Governance/retention policy | Publication Service | Permission to archive. |
| Audit-history references | Audit Infrastructure | Master Data | Preservation linkage. |
| Archive catalog entry | Publication Service | Dataset Catalog Service | Searchable archived metadata. |
| Authorization decision | Authorization & Policy | Archive services | Actor and scope permission. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Actions require an applicable Authorization decision and resolved Tenant scope.
- Ordinary consumers access Published data only.
- Authentication and Authorization remain owned by L2-A03 and L2-A04.
- No data lifecycle state independently grants business authority.

## 15.2 Approval and Separation of Duty

- Archival requires an authorized Platform Administrator and confirmed eligibility.
- Historical access remains subject to Authorization policy.
- No deletion or content-modification authority is created.

## 15.3 Audit Obligations

- Record Tenant, actor/service, Dataset and Version, prior/resulting status, policy and correlation identifier.
- Retain distribution, localization, retirement or archival evidence and all historical lineage.
- A09 provides audit input; A07 owns immutable evidence and retention.

---

# 16. Completion Criteria and Postconditions

Completion requires:

- The Dataset was verified as Retired.
- Authorization, eligibility and preservation obligations succeeded.
- Metadata, all versions, references and audit-history links were preserved.
- The governed archival transition completed.
- The archive catalog is updated and authorized searchability remains.
- `DatasetArchived` and audit evidence were recorded.

Postconditions:

- The Dataset remains unavailable for operational use.
- Archived content and history remain immutable.
- Authorized governance and historical use remain possible.

---

# 17. Open Issues, Assumptions and Deferred Questions


No open issue prevents this scenario from proceeding to Architecture Approval. No unsupported actor, state, transition, event, publisher or implementation mechanism is assumed.

