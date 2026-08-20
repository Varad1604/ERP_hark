# Published Dataset to Governed Retirement and Historical Preservation

# 1. Scenario Metadata

**Scenario ID:** L3-A-040  
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
**Referenced By:** Data consumers, successor planning, historical query and archival governance  

---

# 2. Scenario Purpose

Withdraw a Published Dataset through deprecation and retirement while preserving metadata, versions, references and historical integrity.
---

# 3. Business Outcome

Consumers are informed through deprecation governance, retirement eligibility is completed, operational use ends, and metadata, version history, references and historical integrity remain preserved

Source assessment: `PASS WITH ISSUES`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A published dataset is approved for controlled withdrawal from operational enterprise use.

## 4.2 End Boundary

The Dataset is `Retired`, read-only and unavailable for operational use, while historical metadata, versions, references and audit history remain preserved; archival remains a separate A041 outcome.

## 4.3 In Scope

- Retirement approval and replacement information.
- `Published → Deprecated → Retired`.
- Consumer notification and deprecation history.
- Retirement-eligibility verification.
- Operational withdrawal and historical preservation.

## 4.4 Out of Scope

- Transactional-domain processing and business-semantic ownership.
- Modification of immutable Published or historical versions.
- Technical broker, retry scheduler, storage, API, database, UI or deployment design.
- Creation of authentication, authorization or audit-storage mechanisms.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|

| Upstream | L3-A-034 or L3-A-035 | Supplies a Published Dataset. |
| Followed by | L3-A-041 — Retired Dataset to Governed Archival | Owns the separate archival outcome. |

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
| VAR:Data-Owning Domain | Owns business retirement rationale, successor information and approval. |

The frozen Participating Domains value remains exactly `A, VAR:Data-Owning Domain`.

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

- The Dataset is Published.
- The Business Owner approves controlled withdrawal.
- Replacement information is identified where applicable.
- A deprecation period and retirement eligibility criteria exist.
- Subscribed consumers can be notified.

---

# 8. Trigger

A published dataset is approved for controlled withdrawal from operational enterprise use
---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|

| 1 | Receive the approved withdrawal request and business rationale. | Business Owner; Data Steward | Retirement governance begins. |
| 2 | Resolve the Published Dataset, active version, consumers and historical references. | Master Data | The withdrawal scope is established. |
| 3 | Validate authorization, replacement information and deprecation request. | Publication Service | The Dataset is eligible to deprecate. |
| 4 | Record replacement/migration guidance and deprecation period. | Data Steward; Business Owner | Consumer transition information is traceable. |
| 5 | Transition `Published → Deprecated`. | Publication Service | New adoption is discouraged while current support may continue. |
| 6 | Notify subscribed consumers and publish `DatasetDeprecated`. | Publication and Synchronization Services | Consumers receive the governed warning. |
| 7 | At period completion, verify retirement approval and eligibility. | Business Owner; Platform Administrator | Terminal withdrawal is authorized. |
| 8 | Disable operational usage and transition `Deprecated → Retired`. | Publication Service | The Dataset becomes read-only and non-operational. |
| 9 | Preserve metadata, versions, references, audit history and historical query availability. | Master Data; Audit Infrastructure | Historical integrity is maintained. |
| 10 | Publish `DatasetRetired`. | Publication Service | The retirement fact becomes available; archival has not yet occurred. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|

| Replacement exists | Consumers must migrate to another Dataset/version. | Record replacement and guidance before retirement. | Lineage remains discoverable. |
| No replacement | The Dataset is permanently unnecessary. | Retire under approved rationale. | Historical preservation still applies. |
| Retirement not eligible | Dependencies or approval remain incomplete. | Extend deprecation and keep operational constraints. | No Retired transition occurs. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|

| Withdrawal not approved | Lifecycle change is unauthorized. | Reject request. | Business Owner; Authorization & Policy |
| Consumer notification cannot complete | Governance communication is incomplete. | Retain Deprecated and record issue. | Publication Service |
| Eligibility or period incomplete | Retirement prerequisites fail. | Do not retire. | Publication Service |
| Historical preservation incomplete | Terminal integrity is at risk. | Block retirement until preservation is assured. | Master Data |

No technical retry, rollback, storage or transport mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|

| Published data deprecates before retirement. | Direct retirement bypass is prohibited. | L2-A09 §§7, 25.1 |
| Retired data is read-only and not operational. | It remains available for history and audit. | L2-A09 §7.6 |
| Business Owner approves retirement. | Platform Administrator does not approve business content. | L2-A09 §§6.1, 6.3 |
| Historical versions remain preserved after retirement. | No version or reference is erased. | L2-A09 §39.2 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|

| Dataset | Published | `Published → Deprecated → Retired`. | Retired/read-only. |
| Dataset Versions | Published/historical | Preserve unchanged. | Historically accessible. |
| Consumer subscriptions | Active | Notify and end operational reliance according to plan. | No new operational consumption. |
| Archive status | Not archived | No A040 transition. | Eligible for separate A041. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Publisher | Condition |
|---|---|---|

| `DatasetDeprecated` | Publication Service | Dataset enters controlled retirement. |
| `DatasetRetired` | Publication Service | Operational use ends. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|

| Retirement rationale/approval | VAR:Data-Owning Domain | Publication Service | Business authority and successor strategy. |
| Consumer/subscription view | Synchronization Service | Publication Service | Affected consumers. |
| Deprecation notice | Publication Service | Consumers | Replacement and timing guidance. |
| Historical view | Master Data | Governance/Audit consumers | Preserved read-only history. |
| Audit input | Publication Service | Audit Infrastructure | Approval, transitions and notifications. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Actions require an applicable Authorization decision and resolved Tenant scope.
- Ordinary consumers access Published data only.
- Authentication and Authorization remain owned by L2-A03 and L2-A04.
- No data lifecycle state independently grants business authority.

## 15.2 Approval and Separation of Duty

- Withdrawal and retirement actions require authorization.
- Business Owner approves retirement; Data Steward coordinates.
- No archival approval is asserted in A040.

## 15.3 Audit Obligations

- Record Tenant, actor/service, Dataset and Version, prior/resulting status, policy and correlation identifier.
- Retain distribution, localization, retirement or archival evidence and all historical lineage.
- A09 provides audit input; A07 owns immutable evidence and retention.

---

# 16. Completion Criteria and Postconditions

Completion requires:

- Withdrawal and replacement strategy were approved.
- Consumers were informed through deprecation governance.
- The Dataset reached Deprecated.
- Retirement eligibility and approval completed.
- Operational use ended and the Dataset reached Retired.
- Versions, metadata, references and history remain preserved.
- `DatasetDeprecated` and `DatasetRetired` were published.

Postconditions:

- The Retired Dataset cannot return to Published.
- Historical queries remain possible.
- Archival has not yet occurred and belongs to A041.

---

# 17. Open Issues, Assumptions and Deferred Questions


| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|

| A040-ISSUE-01 | Non-blocking | L2-A09 MD-12 `Retire Dataset` includes a final step `Archive dataset`, while MD-18 and frozen A041 define archival as a separate scenario after retirement. | A040 stops at Retired and historical preservation; perform archival only through A041. Align L2-A09 workflow fragmentation before locking. |

No missing contract is supplied by inference.

