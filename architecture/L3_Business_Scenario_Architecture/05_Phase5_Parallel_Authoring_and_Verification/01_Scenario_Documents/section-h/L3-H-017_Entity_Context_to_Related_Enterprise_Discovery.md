# Entity Context to Related Enterprise Discovery

# 1. Scenario Metadata

**Scenario ID:** L3-H-017  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Directory & Search  
**Participating Domains:** A, H, VAR:Source-Owning Domain  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H03 — Enterprise Directory & Search Workflows; applicable authoritative source-domain architecture; applicable Section A identity, authorization, and audit architecture  
**Referenced By:** Enterprise discovery, related-entity navigation, source-domain lookup, search governance, and audit activities

# 2. Scenario Purpose

Allow an authorized user to discover enterprise entities related to a known institutional subject through H03 searchable projections and relationship context while preserving each result's canonical source ownership and security boundaries.

# 3. Business Outcome

Cross-domain relationships are discoverable through references while canonical source ownership remains unchanged

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized user requests entities related to a known institutional subject.

## 4.2 End Boundary

The scenario ends when H03 returns the permitted related and multi-domain entities with secure discovery/navigation context, each result remaining a reference to its authoritative source-owned entity.

## 4.3 In Scope

- Establishing authenticated/authorized search context.
- Resolving the known institutional subject as discovery context.
- Determining eligible related entity types.
- Resolving relationship context across source-derived searchable projections.
- Applying authorization, institution, campus, department, and field-level visibility rules.
- Returning related entities and discovery groups as references to canonical source-owned records.

## 4.4 Out of Scope

- Creating or modifying any discovered business entity.
- Transferring canonical source ownership to H03.
- Executing source-domain business workflows or approvals.
- Designing search UI, indexes, ranking technology, or implementation mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains canonical identity, authorization, and audit ownership supporting authenticated and permitted discovery. |
| Enterprise Directory & Search / Section H | Owns searchable projections, discovery metadata, relationship-resolution/search behavior, and the H03 discovery outcome. |
| VAR:Source-Owning Domain | Retains canonical ownership of every business entity represented in the discovery result set. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Authenticated User | Requests discovery of entities related to a known institutional subject. | Initiates the discovery request within an authorized security context. |

H03-W03 also permits a consuming service as primary actor, but the frozen trigger for this scenario is specifically an authorized user; this Draft does not broaden the trigger.

# 7. Preconditions and Entry Criteria

- The user is authenticated and has an authorization context.
- The known institutional subject has a valid searchable reference.
- At least one eligible related entity type exists.
- H03 security metadata and source-derived searchable projections required for the request are available.

# 8. Trigger

An authorized user requests entities related to a known institutional subject

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the discovery request for entities related to the known institutional subject. | Authenticated User / Enterprise Directory & Search | H03 has a governed discovery request. |
| 2 | Validate the user's authorization context and determine eligible entity types. | Enterprise Directory & Search using Section A context | Unauthorized scopes are excluded before result delivery. |
| 3 | Resolve relationship context from H03 searchable projections and canonical source references. | Enterprise Directory & Search | Related multi-domain references are identified without changing source records. |
| 4 | Apply policy, institutional, campus, departmental, field-level, and visibility constraints. | Enterprise Directory & Search | Only permitted related entities remain eligible. |
| 5 | Produce discovery groups and related-entity references with secure navigation context. | Enterprise Directory & Search | Cross-domain relationships are discoverable. |
| 6 | Return the discovery outcome while preserving source-domain ownership of every entity. | Enterprise Directory & Search | The frozen business outcome is reached. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| No eligible related entities | Relationship context contains no entities visible to the user. | Return an empty permitted result without disclosing restricted information. | No unauthorized information is exposed. | Enterprise Directory & Search |
| Some related entities are restricted | Relationship exists but visibility policy denies part of the result set. | Exclude unauthorized entities before returning results. | Only authorized relationships are discoverable. | Enterprise Directory & Search |
| Multiple source domains contribute | Related entities originate from several canonical owners. | Group and return source-derived references without merging ownership. | Cross-domain discovery is provided with ownership intact. | Enterprise Directory & Search |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization context is invalid or denied | Related-entity discovery cannot proceed for the requested scope. | Return no restricted result and preserve the security boundary. | Platform Foundation / Enterprise Directory & Search |
| Known subject reference is invalid or unavailable | Relationship context cannot be established reliably. | Do not fabricate related entities; require a valid canonical/searchable subject reference. | Source-Owning Domain / Enterprise Directory & Search |
| No eligible entity type exists | The request cannot produce a governed related-entity result. | Return the source-defined failed/empty discovery outcome without altering any source record. | Enterprise Directory & Search |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H03 never becomes the system of record for any business entity. | Enterprise Directory & Search | Discovery cannot transfer source ownership. | L2-H03 BR-H03-001 |
| Every searchable projection references exactly one authoritative canonical entity and identifies its source owner. | Enterprise Directory & Search | Returned relationships remain source-referenced. | L2-H03 BR-H03-002/003 |
| Business data modification is prohibited within H03. | Enterprise Directory & Search | Discovery is read/discovery behavior only. | L2-H03 BR-H03-004 |
| Every search request executes within an authenticated security context. | Platform Foundation / H03 | Discovery requires governed caller context. | L2-H03 BR-H03-017 |
| Unauthorized entities never appear and empty results do not disclose restricted information. | Enterprise Directory & Search | Security filtering constrains discovery results. | L2-H03 BR-H03-024/025 |
| H03-W03 must preserve canonical ownership and security boundaries. | Enterprise Directory & Search | Related-entity discovery cannot create a competing source of truth. | L2-H03 H03-W03 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Source business entity | Existing canonical lifecycle state | Referenced only through H03 projection/relationship context. | Unchanged. | VAR:Source-Owning Domain | L2-H03 §§2, 5, 9 |
| Searchable Projection | `Searchable` or other source-permitted searchable position | Used for relationship discovery without source mutation. | Unchanged by ordinary discovery request. | Enterprise Directory & Search | L2-H03 §6.4 |
| Discovery request/outcome | H03-W03 request context | Related entities are resolved and returned. | `Discovery Completed` on success or `Discovery Failed` according to W03. | Enterprise Directory & Search | L2-H03 H03-W03 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H03-W03 identifies `EntityDiscoveryRequested`, `EntityDiscoveryCompleted`, and `EntityDiscoveryFailed` as primary workflow events. However, H03's global published-event catalogue does not enumerate these W03 events. This Draft therefore does not assert them as cross-domain canonical published events or assign a new publisher beyond the H03 workflow context.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H03-W03 is explicitly a synchronous request/response discovery workflow with inputs of Discovery Query, Entity Scope, and Authorization Context and outputs of Discovery Groups and Related Entities. Source domains contribute only authoritative referenced facts/projections; ownership never transfers to H03.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Authenticated User initiates the discovery request. Section A retains authorization-policy ownership; H03 applies the resulting security context before returning results.

## 15.2 Separation of Duties, Overrides and Exceptions

Ranking, relationship expansion, or discovery convenience can never override authorization or visibility restrictions. No scenario-specific approval chain is defined.

## 15.3 Business Audit and Evidence Obligations

H03-W03 requires audit. Evidence should preserve caller context, discovery request/scope, known subject reference, security decision context, returned discovery groups/references, and the discovery outcome without copying source-domain ownership into H03. Platform Audit retains canonical audit persistence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authenticated user's request and known subject are resolved.
- Eligible entity types and relationship context are determined.
- Authorization and visibility rules are applied before result delivery.
- Related entities are returned only as source-derived references.
- Cross-domain ownership remains unchanged.
- The discovery outcome is auditable.

## 16.2 Postconditions

- The user receives the permitted related-entity discovery outcome.
- H03 remains owner only of search/discovery projections and metadata.
- Every business entity remains owned by its canonical source domain.
- No source lifecycle or business record is modified.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H03-W03 lists `EntityDiscoveryRequested`, `EntityDiscoveryCompleted`, and `EntityDiscoveryFailed` as primary events, while H03 §10.2's canonical published-event catalogue does not list them. | L3 cannot safely claim these workflow events as registered cross-domain published events with a canonical publisher. | Reconcile the H03 workflow-event list with the canonical published-event catalogue if downstream event consumption is required. |

**Architecture assessment:** `PASS WITH ISSUES`

The W03 actor, trigger, inputs, outputs, relationship discovery, security constraints, source ownership, and synchronous hand-off are explicit. The event-catalogue inconsistency is non-blocking because the frozen outcome is fully satisfied by the documented request/response contract.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
