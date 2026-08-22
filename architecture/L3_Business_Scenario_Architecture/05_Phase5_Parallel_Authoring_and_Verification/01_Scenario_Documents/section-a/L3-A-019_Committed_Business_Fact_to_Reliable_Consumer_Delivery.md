# Committed Business Fact to Reliable Consumer Delivery

# 1. Scenario Metadata

**Scenario ID:** L3-A-019  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Event Infrastructure  
**Participating Domains:** A, VAR:Event-Owning Domain, VAR:Event-Consuming Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-018 — Event Definition Proposal to Active Canonical Event Contract  
**Referenced By:** All domains exchanging canonical business events; L3-A-020 — Failed Event Delivery to Controlled Recovery Outcome; L3-A-022 — Canonical Business Event to Immutable Audit Evidence Availability

---

# 2. Scenario Purpose

Make a committed domain-owned business fact available to eligible consumers through its active canonical Event Contract while preserving immutability, Tenant isolation, publisher/consumer independence and traceability.

---

# 3. Business Outcome

The immutable event is validated, published after the committed business fact, routed within the authorized tenant scope, and reliably delivered to eligible subscribed consumers.

The locked L2 architecture defines the publication, routing and delivery outcome sufficiently. This Draft is classified `PASS WITH ISSUES` because infrastructure lifecycle commands/events are unnamed and the L2 operational state model includes technical states that cannot be expanded at L3.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an owning domain completes a business transaction that produces a registered canonical event.

## 4.2 End Boundary

The scenario ends when the immutable Event Instance has passed canonical validation, been published after the committed fact, routed only to eligible Tenant-scoped subscriptions, and independently acknowledged by each intended consumer or transferred to A020 for a failed delivery.

## 4.3 In Scope

- Confirmation that the originating business fact is committed and domain-owned.
- Validation against the active Event Definition, Version, Schema, Contract, metadata, Tenant and publisher authority.
- Immutable canonical publication.
- Deterministic subscription and Tenant-scoped routing.
- Independent delivery to eligible subscribed consumers.
- Consumer acknowledgement and delivery completion.
- Ordering, correlation, causation and traceability where required by the contract.

## 4.4 Out of Scope

- Creating, approving or activating the Event Definition; that belongs to A018.
- Executing consumer-owned business logic or deciding its business outcome.
- Failed-delivery retry or dead-letter recovery; that belongs to A020.
- Historical replay; that belongs to A021.
- Message brokers, queues, transactional mechanisms, protocols, APIs, storage or deployment design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Upstream | L3-A-018 — Event Definition Proposal to Active Canonical Event Contract | Supplies the active definition, version, schema and contract. |
| Exception | L3-A-020 — Failed Event Delivery to Controlled Recovery Outcome | Owns the governed outcome for an individual failed consumer delivery. |
| Related | L3-A-021 — Historical Event Replay Request to Controlled Redelivery Completion | Redelivers previously published immutable events without creating new facts. |
| Downstream | L3-A-022 — Canonical Business Event to Immutable Audit Evidence Availability | Consumes auditable canonical events. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Event Infrastructure / Section A | Primary owner. Owns publication validation, canonical publication coordination, routing, subscription resolution, independent delivery state and delivery traceability. |
| VAR:Event-Owning Domain | Owns the committed business fact, creates its canonical Event Instance and remains its authoritative publisher. |
| VAR:Event-Consuming Domain | Owns its subscription, consumer processing, business decisions and acknowledgement. |
| Platform & Tenant / Section A | Supplies and enforces Tenant context. |
| Person & Identity / Section A | Supplies publisher and consumer identity references. |
| Authentication / Section A | Establishes authenticated publisher, consumer and administrator context. |
| Authorization & Policy / Section A | Supplies publication, subscription and routing authorization decisions. |
| Audit Infrastructure / Section A | Owns immutable audit evidence generated from the canonical event. |

The frozen Participating Domains value remains exactly `A, VAR:Event-Owning Domain, VAR:Event-Consuming Domain`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Domain Owner | Owns the published business fact and its semantics. | Cannot route to consumers or modify the active contract during publication. |
| Consumer Owner | Maintains the governed subscription and consumer processing behaviour. | Cannot modify the published event or assume publisher implementation details. |
| Platform Administrator | Operates governed Event Infrastructure controls where intervention is required. | Does not own the business fact or consumer decision. |

The publishing and consuming domains may use domain-specific actors, but no additional universal actor is introduced.

---

# 7. Preconditions and Entry Criteria

- The originating domain's business transaction has completed successfully.
- The resulting event represents that completed fact and is owned by the originating domain.
- The Event Definition and Event Contract are registered and `Active`.
- The selected Event Version is active and its Schema is valid.
- Mandatory Event Metadata, Tenant, correlation, causation and publisher context are available.
- The publisher is authenticated and authorized.
- Every intended consumer has an eligible active subscription for the event and version within the authorized Tenant scope.

---

# 8. Trigger

An owning domain completes a business transaction that produces a registered canonical event.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create one canonical Event Instance representing the completed business fact. | VAR:Event-Owning Domain | The event exists in `Created` and remains owned by the originating domain. |
| 2 | Submit the event for governed publication with its Event Definition, Version, Tenant, publisher, correlation, causation and required metadata. | VAR:Event-Owning Domain; Event Infrastructure | A canonical publication request is established. |
| 3 | Validate the active definition, approved version, schema, contract, payload, mandatory metadata, Tenant context and publisher authorization. | Event Infrastructure; Authorization & Policy | The event reaches `Validated` or publication is rejected. |
| 4 | Confirm that publication follows the committed fact and that repeated submission does not create a second business fact. | Event Infrastructure; VAR:Event-Owning Domain | Publication preserves transactional meaning and event identity. |
| 5 | Publish the canonical Event Instance and attach immutable publication metadata. | Event Infrastructure; VAR:Event-Owning Domain remains fact publisher | The event reaches `Published` and cannot be modified. |
| 6 | Resolve active compatible subscriptions and apply deterministic routing, Tenant boundaries, authorization and explicit ordering scope. | Event Routing Service; Subscription Management Service | Only eligible consumer destinations are selected. |
| 7 | Establish an independent delivery lifecycle for each eligible consumer. | Event Infrastructure | One consumer's outcome cannot alter another's delivery. |
| 8 | Make the immutable event available to each intended consumer under the active Event Contract. | Event Infrastructure; VAR:Event-Consuming Domain | Consumer processing begins without publisher coupling. |
| 9 | Validate the consumer acknowledgement for each successful delivery. | VAR:Event-Consuming Domain; Event Infrastructure | The applicable delivery reaches `Delivered`/`Completed`. |
| 10 | Preserve publication, routing, delivery, acknowledgement, ordering and correlation history. | Event Infrastructure; Audit Infrastructure | The end-to-end outcome remains traceable. |

A consumer delivery that fails exits A019 at Step 8 and follows A020 without affecting other consumers.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Multiple eligible consumers | More than one active compatible subscription resolves. | Create an independent delivery outcome for every eligible consumer. | Each consumer receives the event without publisher knowledge of the fan-out. | Event Infrastructure |
| Ordered event stream | Business semantics define an explicit ordering scope. | Preserve the immutable logical sequence within that scope. | Consumers receive the governed sequence; unrelated streams remain independent. | Event Infrastructure; VAR:Event-Owning Domain defines business need |
| No eligible subscription | No active authorized consumer resolves at publication time. | Complete publication and record the routing outcome without inventing a destination. | The event remains a valid published fact with no consumer delivery in scope. | Event Infrastructure |
| Consumer already processed the event | The same immutable identity is delivered again. | Consumer applies its governed duplicate-tolerance responsibility and acknowledges the stable identity. | No duplicate business outcome is created. | VAR:Event-Consuming Domain |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Business transaction is not committed. | The event cannot represent a completed fact. | Reject publication; do not expose the event to consumers. | VAR:Event-Owning Domain; Event Infrastructure |
| Definition, version, schema or contract is inactive or invalid. | Canonical publication eligibility is absent. | Reject publication and resolve the governance deficiency under A018 or version governance. | Event Infrastructure |
| Mandatory metadata or Tenant context is invalid. | Identity, isolation or traceability cannot be preserved. | Reject publication without routing or delivery. | Event Infrastructure; Platform & Tenant |
| Publisher authorization is denied. | The domain cannot publish through the governed contract. | Return the authorization outcome; do not publish. | Authorization & Policy; Event Infrastructure |
| Consumer subscription is inactive, unauthorized or incompatible. | That consumer is ineligible for routing. | Exclude it from the routing result without affecting eligible consumers. | Subscription Management Service; Authorization & Policy |
| Consumer does not acknowledge successful processing. | That delivery cannot be completed. | Transfer only the affected delivery to A020; other delivery lifecycles continue independently. | Event Infrastructure; VAR:Event-Consuming Domain |
| Published event requires business correction. | The immutable fact cannot be edited. | The owning domain publishes a new governed correction event where its business architecture permits. | VAR:Event-Owning Domain |

Technical transport recovery is outside A019.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Publication occurs only after successful business completion. | VAR:Event-Owning Domain; Event Infrastructure | Premature events are prohibited. | L2-A06 EP-01 |
| The originating domain owns and publishes the completed fact. | VAR:Event-Owning Domain | Event Infrastructure coordinates publication without re-owning semantics. | L2-A06 EIP-02; CDI-03 |
| Every publication uses an active registered canonical contract. | Event Infrastructure | Ad-hoc event formats are rejected. | L2-A06 EIP-03; ER-02; EC-01 |
| Published Event Instances and metadata are immutable. | Event Infrastructure; VAR:Event-Owning Domain | Corrections require new events. | L2-A06 EIP-01; EP-03; EM-02 |
| Publisher knowledge of consumers is prohibited. | Event Infrastructure | Routing remains infrastructure-owned and loosely coupled. | L2-A06 ERA-01; CDI-01 |
| Routing is deterministic and Tenant isolated. | Event Infrastructure; Platform & Tenant | Unauthorized or cross-Tenant delivery is prohibited. | L2-A06 ERA-03–ERA-05 |
| Each consumer has an independent delivery lifecycle. | Event Infrastructure | One consumer failure cannot delay others. | L2-A06 DC-01; DS-02 |
| Delivery completes only after consumer acknowledgement. | Event Infrastructure; VAR:Event-Consuming Domain | Unacknowledged delivery is not successful. | L2-A06 DS-03; CR-03 |
| Consumers treat events as immutable and tolerate duplicates. | VAR:Event-Consuming Domain | Consumer processing preserves fact integrity. | L2-A06 CR-01; CR-02 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Event Definition / Version / Contract | `Active` | Referenced for validation; no lifecycle transition. | Remains `Active`. | Event Registry Service |
| Event Instance | `Created` | Validate against canonical governance. | `Validated` | Event Infrastructure; fact ownership remains with publisher |
| Event Instance | `Validated` | Publish immutably after the committed fact. | `Published` | Event Infrastructure coordinates; VAR:Event-Owning Domain publishes fact |
| Event Instance | `Published` | Resolve eligible consumer destinations. | `Routed` | Event Routing Service |
| Consumer delivery | Eligible route exists | Coordinate independent delivery and acknowledgement. | `Delivering → Delivered`; processing lifecycle `Acknowledged → Completed` | Event Delivery Service |
| Consumer-owned aggregate | Domain-defined condition | Changes only through consumer-owned business behaviour. | Domain-defined outcome. | VAR:Event-Consuming Domain |

L2-A06 also defines technical `Queued` and delivery-recovery states; they are not expanded in this L3 business flow.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Publication Condition | Business Effect / Consumers |
|---|---|---|---|
| The registered canonical event identified by the active Event Contract | VAR:Event-Owning Domain | The underlying business fact is committed and every publication validation succeeds. | Eligible subscribed VAR:Event-Consuming Domains receive the immutable fact; Event Infrastructure coordinates publication, routing and delivery. |

The exact event name is scenario-variable and comes from A018's active Event Definition. L2-A06 defines no separate canonical infrastructure event names for validation, routing or delivery completion, so none are invented.

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Contract / Hand-off | Business Information / Outcome |
|---|---|---|---|
| Event Registry Service | Publisher and Event Infrastructure | Active Event Contract | Definition, owner, version, schema, metadata, security and compatibility rules. |
| VAR:Event-Owning Domain | Event Infrastructure | Canonical Event Instance and publisher context | Committed fact, immutable identity, Tenant and traceability information. |
| Authorization & Policy | Event Infrastructure | Publication/routing authorization decision | Governed allow or deny outcome. |
| Subscription Management | Event Routing Service | Active compatible subscriptions | Eligible consumer and Tenant scope. |
| Event Infrastructure | VAR:Event-Consuming Domain | Immutable canonical event | Fact and metadata governed by the active contract. |
| VAR:Event-Consuming Domain | Event Infrastructure | Consumer acknowledgement | Authoritative delivery-processing outcome. |
| Event Infrastructure | Audit Infrastructure | Canonical event and traceability context | Input for immutable audit evidence under A022. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Publisher, subscriptions, routing and privileged operations require authentication and authorization.
- An active subscription grants eligibility to receive the contract; it does not transfer fact ownership.
- Tenant scope and least privilege apply to every publisher and consumer.

## 15.2 Approval and Separation of Duty

- The active Event Definition, Version, Schema and Contract were approved upstream under A018.
- Subscription approval is governed independently; publisher approval is not required unless policy explicitly requires it.
- L2-A06 defines no additional A019-specific SoD or override rule.

## 15.3 Audit Obligations

Event identity, definition/version, owning domain, Tenant, publisher, publication time, correlation, causation, routing decision, eligible consumers, delivery and acknowledgement outcomes, and authorization decisions must remain traceable without changing the event payload.

---

# 16. Completion Criteria and Postconditions

The scenario is complete when:

- the originating business fact was committed;
- the publisher retained ownership of the fact;
- the event passed active definition, version, schema, contract, metadata, Tenant and authorization validation;
- one immutable canonical Event Instance was published;
- all eligible subscriptions were resolved deterministically within Tenant scope;
- independent delivery was coordinated for every intended consumer;
- each successful delivery was acknowledged and completed;
- any failed delivery transferred independently to A020; and
- end-to-end traceability was preserved.

Postconditions:

- The published event and metadata are immutable.
- Publisher and consumers remain decoupled.
- Each consumer owns its subsequent business processing.
- One consumer's failure did not change any other delivery outcome.
- Historical replay has not occurred.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A019-ISSUE-01:** L2-A06 defines publication, routing and delivery responsibilities and states but no canonical commands or infrastructure lifecycle events. | The end-to-end business outcome is clear, but exact L4 operation contracts and optional lifecycle notifications cannot be derived. | Define command contracts and lifecycle event names/publishers where enterprise notification is required. |
| **A019-ISSUE-02:** L2-A06's Event Instance lifecycle includes the technical `Queued` state and Transactional Outbox detail, while the L3 template prohibits queues and transaction-handling mechanisms. | This Draft preserves publication-after-commit behaviour but omits the technical state/mechanism from the business flow. | Keep those mechanics at L4/L5 or define a business-readable L3 lifecycle abstraction. |
| **A019-ISSUE-03:** L2-A06 says `Delivered` follows consumer acknowledgement but also says delivery completion does not imply successful business processing unless explicitly acknowledged. | Acknowledgement meaning must be contract-specific; A019 treats it only as the authoritative delivery-processing outcome supplied by the consumer. | Clarify acknowledgement semantics in each Event Contract or consumer contract. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
