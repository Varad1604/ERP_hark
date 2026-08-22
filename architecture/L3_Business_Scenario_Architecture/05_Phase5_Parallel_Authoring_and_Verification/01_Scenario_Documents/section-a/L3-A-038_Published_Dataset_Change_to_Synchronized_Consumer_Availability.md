# Published Dataset Change to Synchronized Consumer Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-038  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Master Data  
**Participating Domains:** A, VAR:Data-Consuming Domain  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** undefined  
**Referenced By:** Subscribed data consumers, reconciliation and downstream domain availability  

---

# 2. Scenario Purpose

Distribute a Published Dataset change to authorized subscribed consumers with governed delivery status, acknowledgement, failure recovery and audit traceability.
---

# 3. Business Outcome

The published change is distributed to authorized subscribed consumers, delivery outcomes are recorded, failures follow governed recovery, and the authoritative source dataset remains unchanged

Source assessment: `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A published dataset changes and subscribed consumers require the approved update.

## 4.2 End Boundary

Every in-scope consumer has an Acknowledged/Completed delivery or a governed recorded failure/recovery position, without changing the authoritative source Dataset.

## 4.3 In Scope

- Published-change detection.
- Subscription and consumer authorization validation.
- Synchronization-package creation and delivery.
- Acknowledgement and status lifecycle.
- Governed retry/isolation and audit history.

## 4.4 Out of Scope

- Transactional-domain processing and business-semantic ownership.
- Modification of immutable Published or historical versions.
- Technical broker, retry scheduler, storage, API, database, UI or deployment design.
- Creation of authentication, authorization or audit-storage mechanisms.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|

| Upstream | L3-A-034 or L3-A-035 | Supplies a Published new or successor version. |
| Related | L3-A-020 | Governs infrastructure-level failed event delivery; A038 retains synchronization outcome ownership. |

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
| VAR:Data-Consuming Domain | Owns downstream use and acknowledgement, not the source Dataset. |

The frozen Participating Domains value remains exactly `A, VAR:Data-Consuming Domain`.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|

| Business Owner | Owns business semantics, structural approval and retirement approval. | Belongs to the data-owning domain. |
| Data Steward | Maintains quality, corrections, metadata and governance coordination. | Does not own business meaning. |
| Platform Administrator | Operates governance, publication and synchronization capabilities. | Does not approve business content. |
| Reviewer | Records authorized review and approval decisions. | Requires valid approval authority. |
| Data Consumer | Consumes or subscribes to Published data. | Cannot modify the governed source. |
| Synchronization Service | Packages, delivers and records subscribed updates. | Owns synchronization status, not source semantics or consumer business processing. |

---

# 7. Preconditions and Entry Criteria

- A Dataset Version is Published.
- At least one authorized consumer subscription exists.
- The published change and source version are identifiable.
- Consumer scope and Tenant boundaries can be validated.
- Synchronization status can be recorded per consumer.

---

# 8. Trigger

A published dataset changes and subscribed consumers require the approved update
---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|

| 1 | Detect the Published Dataset change. | Synchronization Service | A synchronization run begins. |
| 2 | Resolve the exact source Dataset Version and its publication metadata. | Publication Service; Synchronization Service | The authoritative update is fixed. |
| 3 | Resolve eligible subscriptions and validate Tenant and access scope. | Synchronization Service; Authorization & Policy | Only authorized consumers remain in scope. |
| 4 | Create a synchronization package without changing the source. | Synchronization Service | A governed delivery representation exists. |
| 5 | Set the delivery lifecycle to `Queued → Preparing → Delivering`. | Synchronization Service | Delivery progress is traceable. |
| 6 | Deliver the update to each eligible consumer. | Synchronization Service | Consumers receive the approved content. |
| 7 | Receive and validate consumer acknowledgement. | VAR:Data-Consuming Domain; Synchronization Service | Delivery acceptance is evidenced. |
| 8 | Advance successful delivery to `Delivered → Acknowledged → Completed`. | Synchronization Service | The consumer outcome completes. |
| 9 | Apply governed retry or isolation for failed delivery without affecting successful consumers. | Synchronization Service | Failure remains controlled. |
| 10 | Publish `DatasetSynchronized` for completed synchronization and record audit input. | Synchronization Service; Event and Audit Infrastructure | The final outcome is observable and auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|

| Multiple consumers | Several eligible subscriptions exist. | Track an independent delivery outcome per consumer. | One failure does not alter other successful outcomes. |
| Transient failure | Delivery can be retried. | Retain status and retry under the A09 recovery rule. | Source remains unchanged. |
| Persistent failure | Recovery cannot complete. | Isolate and expose for administrative intervention. | No false success is recorded. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|

| Subscription absent | No eligible consumer relationship exists. | Do not deliver. | Synchronization Service |
| Authorization fails | Consumer may not receive the version/scope. | Exclude consumer and audit denial. | Authorization & Policy |
| Acknowledgement missing | Completion cannot be asserted. | Keep non-terminal status and apply governed recovery. | Synchronization Service |
| Source version changes during run | Package lineage could become ambiguous. | Continue with the fixed source version or start a separate run. | Synchronization Service |

No technical retry, rollback, storage or transport mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|

| Synchronization starts from Published data. | Non-published content cannot be distributed. | L2-A09 MD-15; §27.2 |
| Source Dataset remains unchanged. | Synchronization manages delivery copies/status only. | L2-A09 ownership principles |
| Failures enter retry before administrative intervention. | No successful consumer is rolled back because another fails. | L2-A09 §25.4 |
| Each delivery outcome is recorded and audited. | Consumer availability is traceable. | L2-A09 MD-15; §37 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|

| Source Dataset Version | Published | Read and packaged only. | Unchanged/immutable. |
| Synchronization | Absent | `Queued → Preparing → Delivering → Delivered → Acknowledged → Completed`. | Completed or governed failure position. |
| Consumer copy | Prior version | Receives the approved update. | Updated under consumer ownership. |
| Subscription | Existing/authorized | Referenced for scope and delivery. | Unchanged. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Publisher | Condition |
|---|---|---|

| `DatasetPublished` | Publication Service | Consumed as the source change. |
| `DatasetSynchronized` | Synchronization Service | A subscribed delivery completes. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|

| Published version | Publication Service | Synchronization Service | Exact authoritative change. |
| Subscription and scope | Consumer/subscription governance | Synchronization Service | Eligible destination. |
| Authorization decision | Authorization & Policy | Synchronization Service | Permitted delivery. |
| Acknowledgement | VAR:Data-Consuming Domain | Synchronization Service | Consumer receipt outcome. |
| Audit input | Synchronization Service | Audit Infrastructure | Run and per-consumer outcomes. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Actions require an applicable Authorization decision and resolved Tenant scope.
- Ordinary consumers access Published data only.
- Authentication and Authorization remain owned by L2-A03 and L2-A04.
- No data lifecycle state independently grants business authority.

## 15.2 Approval and Separation of Duty

- Only approved subscriptions and authorized scopes are synchronized.
- No new business-content approval is introduced after publication.
- Recovery does not bypass authorization or Tenant isolation.

## 15.3 Audit Obligations

- Record Tenant, actor/service, Dataset and Version, prior/resulting status, policy and correlation identifier.
- Retain distribution, localization, retirement or archival evidence and all historical lineage.
- A09 provides audit input; A07 owns immutable evidence and retention.

---

# 16. Completion Criteria and Postconditions

Completion requires:

- The exact Published source version was fixed.
- Eligible authorized subscriptions were resolved.
- Packages were delivered independently per consumer.
- Acknowledgements and terminal statuses were recorded for successes.
- Failures entered governed recovery/isolation.
- `DatasetSynchronized` and audit evidence represent completed outcomes.
- The source Dataset remained unchanged.

Postconditions:

- Consumers own their downstream processing.
- Source ownership and Published immutability are preserved.
- Incomplete deliveries remain distinguishable from completed ones.

---

# 17. Open Issues, Assumptions and Deferred Questions


No open issue prevents this scenario from proceeding to Architecture Approval. No unsupported actor, state, transition, event, publisher or implementation mechanism is assumed.

