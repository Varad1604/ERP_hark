# Dashboard Sharing Request to Controlled Access Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-051  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Dashboard  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A04 — Authorization & Policy Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Dashboard collaboration, workspace access, dashboard sharing expiry, and audit scenarios

# 2. Scenario Purpose

Govern sharing of H07 dashboard metadata and access descriptors while Section A remains authoritative for authorization and while no operational business data or source-domain ownership is transferred.

# 3. Business Outcome

Access is granted, denied, changed, expired, or revoked under authorization and visibility policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized owner requests dashboard sharing.

## 4.2 End Boundary

The scenario ends when the sharing request or existing share reaches a governed access outcome of granted, denied, changed, expired, or revoked under the source-defined H07 sharing controls and Section A authorization constraints.

## 4.3 In Scope

- H07 Dashboard Sharing Metadata.
- Share offer, acceptance, permission change, expiry, and revocation.
- Rejection of invalid or unauthorized sharing requests as the frozen denied outcome.
- Section A authorization constraints.
- Metadata-only sharing.

## 4.4 Out of Scope

- Transfer or duplication of operational business data.
- Transfer of Dashboard or source-domain ownership.
- Inventing a `Denied` lifecycle state or event.
- Technical invitation delivery, token design, UI, or implementation mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Owns authorization policy, roles, decisions, and permission boundaries constraining access. |
| Enterprise Dashboard / H | Owns Dashboard Sharing Metadata, share permission metadata, share lifecycle, and dashboard presentation availability. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Dashboard Owner | Initiates a share for a shareable dashboard where authorized. | Initiates |
| Administrator | Performs H07 sharing administration where authorized. | Performs |
| Recipient User | Accepts an offered share where the recipient identity and authorization satisfy policy. | Decides / receives access |
| Recipient Administrator | May accept a share for the recipient context where H07 defines that participation. | Decides / performs |

# 7. Preconditions and Entry Criteria

- The source dashboard exists and is shareable.
- The target principal exists.
- The Dashboard Owner or Administrator is authorized to initiate sharing.
- The recipient can receive only access permitted by Section A authorization and H07 visibility policy.
- Sharing is limited to metadata and access descriptors.

# 8. Trigger

An authorized owner requests dashboard sharing

# 9. Main Business Flow

1. The authorized owner requests sharing of an H07 dashboard with a valid target principal or workspace.
2. H07 validates shareability, target identity, expiry policy, classification, scope compatibility, and applicable Section A authorization.
3. H07 creates the governed share metadata and offers access under the defined terms.
4. The recipient accepts the valid share where recipient authorization permits access.
5. H07 makes the shared dashboard available only within the permitted metadata scope.
6. Any later governed permission change modifies only H07 sharing permissions and does not transfer ownership.
7. When the time or policy boundary is reached, H07 expires the share and removes access; where revocation is required, H07 revokes the share through its defined sharing action and lifecycle.
8. H07 preserves sharing history and the source dashboard remains independently owned.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Denied | Dashboard is restricted/unpublished, target is invalid, or authorization/scope policy fails | H07 rejects the sharing request without creating valid access | Access denied without inventing a `Denied` state | Enterprise Dashboard / Platform Foundation |
| Time-limited share | A governed expiry is defined | Share is accepted and remains active only until the boundary is reached | Access later expires | Enterprise Dashboard |
| Permission change | Existing sharing permission scope is changed through the H07-defined sharing-permission action | H07 updates sharing metadata within authorization limits | Access scope changed | Enterprise Dashboard |
| Revocation | An active share must be withdrawn | H07 revokes the share under governed authority | Access revoked | Enterprise Dashboard |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Invalid target, restricted dashboard, or incompatible scope | Share cannot be granted | Reject the request; no access is created | Enterprise Dashboard |
| Recipient lacks sufficient permission | Shared dashboard cannot become accessible | Deny/withhold access under Section A authorization | Platform Foundation / A |
| Share becomes expired or revoked | Recipient no longer has access through that share | A new governed share is required for future access | Enterprise Dashboard |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Sharing transfers configuration metadata only | Enterprise Dashboard | No business data is duplicated | L2-H07 BR-SHR-001, BR-SHR-002 |
| Recipients require sufficient permissions before access | Platform Foundation / Enterprise Dashboard boundary | Sharing cannot expand authorization | L2-H07 BR-SHR-003; L2-A04 |
| Accepted shared dashboards remain independent unless synchronization is explicitly enabled | Enterprise Dashboard | Owner changes do not silently transfer into the recipient copy/access context | L2-H07 BR-SHR-004 |
| Workspace/dashboard visibility never grants additional business permissions | Platform Foundation | Section A authorization remains authoritative | L2-A04 AP-07, AP-09 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Dashboard Share | Draft / Offered | Governed share offer and recipient acceptance | Accepted / Active | Enterprise Dashboard | L2-H07 §52.8 |
| Dashboard Share | Active | Governed expiry or revocation | Expired or Revoked, followed by archival where applicable | Enterprise Dashboard | L2-H07 §§51.23, 52.8 |
| Dashboard Sharing permissions | Existing share metadata | Governed permission modification | Changed metadata within the same H07 ownership boundary | Enterprise Dashboard | L2-H07 §21.8 |

A denied request is represented by the explicit H07 rejection/failure path, not by an invented lifecycle state.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| DashboardShared | Enterprise Dashboard | Target sharing context | Records governed share creation | L2-H07 §23.7; §51.21 |
| DashboardShareAccepted | Enterprise Dashboard | H07 sharing context | Records recipient acceptance | L2-H07 §23.7; §51.22 |
| DashboardShareExpired | Enterprise Dashboard | H07 sharing context | Records governed expiry | L2-H07 §23.7; §51.23 |
| DashboardShareRevoked | Enterprise Dashboard | H07 sharing context | Records governed revocation | L2-H07 §23.7; §52.8 |

H07 defines a governed sharing-permission modification action but no distinct canonical event for a sharing-permission change.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A supplies authorization decisions and policy constraints. H07 owns the share metadata and changes only dashboard presentation access. No operational business fact is transferred.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate share | Dashboard Owner / Administrator | Must have source ownership/share authority and satisfy Section A authorization | Enterprise Dashboard |
| Accept share | Recipient User / Recipient Administrator | Recipient identity and authorization must match the governed target | Enterprise Dashboard with Section A authorization |
| Change/revoke/expire access | H07 sharing governance authority as defined by the applicable action | Cannot exceed Section A policy or tenant/scope constraints | Enterprise Dashboard |

## 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is defined. Sharing never overrides tenant, role, classification, or Section A authorization restrictions.

## 15.3 Business Audit and Evidence Obligations

Preserve source and target principal references, permission scope, expiry where defined, acceptance, permission-change history, revocation/expiry history, and the authorization outcome required by H07.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The sharing request or existing share has reached a source-supported granted, denied, changed, expired, or revoked access outcome.
- No operational business data or source-domain ownership has been transferred.
- Section A authorization remains authoritative.

## 16.2 Postconditions

The effective dashboard-sharing availability reflects the governed outcome, and the H07 sharing history remains auditable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H07 supports governed sharing-permission modification, but no distinct canonical business event is defined for the frozen `changed` access branch. The branch remains source-supported through the H07 sharing workflow material and audited permission-change obligation, so this is non-blocking for the Draft.
- H07 §52.8 presents the textual allowed-transition chain in a way that could be read as `Expired → Revoked`, while its accompanying state diagram and explicit workflows treat expiry and revocation as alternative closures from active sharing. This Draft relies only on the explicit expiry/revocation workflows and does not assert the disputed sequence.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
