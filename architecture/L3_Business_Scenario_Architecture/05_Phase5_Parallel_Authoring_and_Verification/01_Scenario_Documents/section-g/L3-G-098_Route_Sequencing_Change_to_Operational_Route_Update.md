# Route Sequencing Change to Operational Route Update

**Scenario ID:** L3-G-098  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-098 |
| Canonical Title | Route Sequencing Change to Operational Route Update |
| Primary Owner | Transport Management |
| Participating Domains | G |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs a change to the configured order in which Transport Stops are serviced on an existing Transport Route. G04 explicitly makes Route Sequencing a Transport Route responsibility and requires every route to maintain an operational stop sequence.

## 3. Business Outcome

An authorised revised stop sequence becomes the effective operational sequence for the affected Transport Route, and the updated route remains historically traceable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A business need arises to change the order in which stops are serviced on an existing route.

### 4.2 End Boundary
The revised sequence is effective as the operational route sequence.

### 4.3 In Scope
- Current route/sequence identification.
- Proposed sequence change.
- Operational validation and impact assessment.
- Review/approval.
- Effective route update.
- Historical preservation.

### 4.4 Out of Scope
- Vehicle asset ownership or maintenance execution.
- Student/workforce lifecycle ownership.
- Financial accounting.
- Notification delivery mechanics.
- Technical APIs, databases, services, queues, UI, deployment or infrastructure.
- Separate canonical allocation, trip, diversion or passenger-reassignment outcomes.

## 5. Ownership and Participating Domains

Transport Management (G04) is the canonical owner of the transport operational outcome. The frozen participating-domain set is **G only**. Participation or downstream consumption does not transfer ownership.

G04 does not own vehicle assets, maintenance execution, student lifecycle, employee ownership or financial accounting. Those boundaries remain unchanged.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Transport Administrator / Coordinator | Establishes and manages the proposed route change. |
| Transport Manager | Typical authority for route-change approval. |
| Downstream transport consumers | Consume the updated route without acquiring ownership. |

technical access control/IAM is outside L3.

## 7. Preconditions and Entry Criteria

- An existing Transport Route is authoritative.
- A current stop sequence exists.
- A legitimate business reason for changing the sequence exists.
- The proposed sequence can be evaluated against applicable operational constraints.

## 8. Trigger

A route sequencing change is requested.

## 9. Main Business Flow

1. **Identify the route and current sequence.**
2. **Establish the proposed sequence.**
3. **Validate the proposed sequence.** G04 identifies operational considerations including active trips, route capacity, vehicle availability, driver assignment, student allocation and administrative approval.
4. **Perform impact assessment.**
5. **Conduct operational review.**
6. **Approve the change.**
7. **Update the route.** The revised sequence becomes effective.
8. **Preserve history.** The previous route sequence remains historically traceable.
9. **Publish the authoritative route modification event.**

## 10. Alternate Business Flows

- If validation fails, correct and re-evaluate the proposed sequence.
- If approval is not granted, the current sequence remains effective.
- If the change is actually a temporary diversion, use the Route Diversion outcome.
- If the request creates or removes a stop, the Stop Management outcome remains authoritative; this scenario handles sequencing.

## 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Business Recovery |
|---|---|---|
| Proposed sequence violates configured constraints | New sequence cannot become effective | Correct and revalidate |
| Review incomplete | Current sequence remains effective | Complete business review |
| Change rejected | No operational sequence change | Retain current sequence or submit revised proposal |
| Change is actually diversion | Ordinary sequencing boundary is insufficient | Use the canonical diversion scenario |

Recovery is business-level only. No technical retries, queues, rollback or infrastructure failure handling is introduced.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Scenario Effect |
|---|---|
| Transport Route owns Route Sequencing. | Sequence change is G04-owned. |
| Every route shall maintain an operational stop sequence. | Effective route retains an ordered sequence. |
| Sequence modifications may occur due to traffic conditions, road closures, administrative decisions, temporary diversions or emergency situations. | These are source-supported circumstances. |
| Historical route versions shall remain preserved. | Previous sequence remains traceable. |
| Route-change validation may consider active trips, capacity, vehicle availability, driver assignment, student allocation and administrative approval. | These are source-supported considerations. |
| Validation rules are configuration driven. | Institution-specific policy remains configurable. |

## 13. State and Lifecycle Impact

The Transport Route lifecycle is:

`Draft → Configured → Active → Suspended → Retired`

Sequencing is a responsibility of Transport Route, not a separate lifecycle aggregate.

The route-change workflow is:

`Change Requested → Operational Review → Impact Assessment → Approval → Route Updated → Stakeholders Notified`

The L3 completion boundary is **Route Updated**. Notification delivery is downstream.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G04 explicitly defines `RouteModified` — Route updated.

Because Route Sequencing is owned by Transport Route and no separate sequence-change event is defined, the canonical event is:

**`RouteModified` — publisher: Transport Management (G04).**

No `RouteSequenceChanged` event is invented.

### 14.2 Synchronous Business Contracts / Hand-offs

The frozen participating-domain set is G only. No additional cross-domain synchronous hand-off is required to complete the scenario. Downstream consumers do not acquire ownership or directly manipulate G04 transport state.

## 15. Business Authorization, Approval and Audit Obligations

G04 identifies Transport Manager as the typical authorization for approving route changes. This is business authority only.

Audit evidence includes the affected route, prior sequence, revised sequence, review/impact assessment, approval and effective update.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Existing route identified.
- Proposed sequence established.
- Validation/impact assessment completed.
- Review and approval completed.
- Route sequence becomes effective.
- `RouteModified` is published by G04.
- Previous route history remains preserved.

### 16.2 Postconditions
- The named business outcome is authoritative within G04.
- Relevant transport history remains traceable.
- No unrelated domain lifecycle is implicitly changed.
- Later allocation, trip and notification activity remains governed by its own canonical scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No material unresolved issue was identified for the core sequencing workflow. G04 explicitly provides the route-change workflow and `RouteModified` event.

### 17.2 Assumptions
- Institution-specific operational policies remain configuration driven as stated by G04.
- No actor, rule, state or event absent from authoritative architecture is invented.

### 17.3 Deferred Questions
Any material unresolved upstream naming or semantic issue must be resolved at the authoritative architecture level before Architecture Approval/Lock.
