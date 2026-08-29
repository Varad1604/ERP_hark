# Historical Event Replay Request to Controlled Redelivery Completion

# 1. Scenario Metadata

**Scenario ID:** L3-A-021  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Event Infrastructure  
**Participating Domains:** A, VAR:Replay-Requesting Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-019 — Committed Business Fact to Reliable Consumer Delivery  
**Referenced By:** Operational recovery, reconciliation, migration and governed correction scenarios requiring historical redelivery

---

# 2. Scenario Purpose

Govern the authorized redelivery of previously published immutable Event Instances for an approved operational need while preserving original identity, ordering, Tenant isolation, domain ownership and complete replay history.

---

# 3. Business Outcome

The approved historical event scope is replayed without modifying the original business facts, with ordering, tenant isolation, authorization and replay history preserved.

The locked L2 architecture defines replay scope, lifecycle, authorization, preservation and audit requirements sufficiently. This Draft is classified `PASS WITH ISSUES` because replay commands, lifecycle events and exact request/approval role mapping are not defined.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized operational recovery, reconciliation, migration or correction need requires previously published events to be redelivered.

## 4.2 End Boundary

The scenario ends when the approved replay scope reaches `Completed`, `Cancelled` or `Failed`; original Event Instances remain immutable; required ordering and Tenant isolation are preserved; the requesting domain receives the replay outcome; and full replay history is auditable.

## 4.3 In Scope

- Replay request and approved operational purpose.
- Explicit selection of historical Event scope.
- Authentication, authorization and governance approval.
- Scope preparation and compatibility/eligibility validation.
- Controlled redelivery using original immutable Event identity and context.
- Preservation of ordering where required.
- Isolation from normal delivery and other consumers.
- Completion, cancellation or failed replay outcome and audit history.

## 4.4 Out of Scope

- Modifying or republishing the original business fact as a new event.
- Correcting consumer-owned business state directly.
- Selecting technical transport, storage, batching, scheduling or performance mechanisms.
- Bypassing consumer duplicate-tolerance responsibilities.
- Changing Event Definition, Version, Schema or Contract.
- Granting authorization inside Event Infrastructure.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Upstream | L3-A-019 — Committed Business Fact to Reliable Consumer Delivery | Provides the original immutable published Event Instances. |
| Related | L3-A-020 — Failed Event Delivery to Controlled Recovery Outcome | A dead-lettered delivery may motivate an approved replay request. |
| Related | L3-A-022 — Canonical Business Event to Immutable Audit Evidence Availability | Replay must not duplicate or modify canonical audit evidence for the original fact. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Event Infrastructure / Section A | Primary owner. Owns Historical Event Repository, Replay Session, replay scope preparation, controlled redelivery, lifecycle and replay history. |
| VAR:Replay-Requesting Domain | Owns the operational need, requested scope and consumption of the replay outcome; it does not own Event Infrastructure lifecycle. |
| Platform & Tenant / Section A | Supplies Tenant context and isolation. |
| Person & Identity / Section A | Supplies canonical requestor and operational actor references. |
| Authentication / Section A | Establishes authenticated context. |
| Authorization & Policy / Section A | Owns replay authorization and applicable policy decisions. |
| Audit Infrastructure / Section A | Owns immutable audit evidence derived from replay governance and lifecycle activity. |

The frozen Participating Domains value remains exactly `A, VAR:Replay-Requesting Domain`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Operations Engineer | Investigates operational conditions and coordinates approved recovery activity. | Cannot approve its own authority unless an upstream policy explicitly permits it. |
| Platform Administrator | Performs governed Event Infrastructure operational actions. | Cannot modify historical events or bypass authorization. |
| Security Administrator | Oversees privileged access and applicable security controls. | Does not own the replay need or business facts. |
| Consumer Owner | Maintains affected consumer behaviour where the replay targets that consumer. | Must treat replayed events as immutable and duplicate tolerant. |

L2-A06 requires a recorded requestor and approval but does not name a universal Replay Requestor or approving business role. The VAR:Replay-Requesting Domain remains a participant, not an invented actor.

---

# 7. Preconditions and Entry Criteria

- A legitimate operational recovery, reconciliation, migration or correction need exists.
- Previously published immutable Event Instances remain available in the Historical Event Repository.
- The requested scope identifies a single event, definition, version, time range, consumer, correlation or Tenant boundary, or an explicitly governed combination.
- The requestor is authenticated and the request belongs to an identified Tenant scope.
- Applicable replay, authorization, ordering, retention and consumer-compatibility policies can be evaluated.
- The request does not seek to modify historical business facts.

---

# 8. Trigger

An authorized operational recovery, reconciliation, migration or correction need requires previously published events to be redelivered.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Submit the replay need, business/operational purpose, requested historical scope and intended consumer context. | VAR:Replay-Requesting Domain; authorized actor not universally named | A Replay Session enters `Requested`. |
| 2 | Validate requestor identity, Tenant boundary, purpose, requested scope, historical availability and absence of a prohibited fact modification. | Event Infrastructure; Platform & Tenant; Person & Identity | The request is eligible for authorization review. |
| 3 | Obtain the replay authorization and governance approval through Authorization & Policy. | Authorization & Policy; approving role undefined | The Replay Session reaches `Authorized` or is denied. |
| 4 | Resolve the exact approved historical events, versions, consumer destinations, correlation boundaries and required ordering. | Replay Management Service | The Replay Session reaches `Prepared`. |
| 5 | Confirm that original identities, publication timestamps, correlation, causation, Tenant context and Event Versions will be preserved. | Event Infrastructure | Historical integrity is established before redelivery. |
| 6 | Begin controlled replay for only the approved scope and record its start. | Platform Administrator / Operations Engineer under authorization; Replay Management Service | The Replay Session reaches `Executing`. |
| 7 | Redeliver each selected immutable event to only its approved destination while preserving required ordering and isolation. | Event Infrastructure; applicable consumer | The approved historical scope progresses without creating new business facts. |
| 8 | Track processed and failed replay items and preserve the original events unchanged. | Replay Management Service | Replay progress remains governed and traceable. |
| 9 | When the approved scope completes successfully, transition the Replay Session to `Completed`. | Replay Management Service | Controlled redelivery completion is established. |
| 10 | Return the replay scope, processing outcome and final status to the Replay-Requesting Domain. | Event Infrastructure | The requesting domain receives the governed outcome. |
| 11 | Preserve requestor, approval, scope, start/completion, processed/failed counts, ordering, actors and final outcome as immutable evidence. | Event Infrastructure; Audit Infrastructure | Complete replay history is available. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Single-event replay | One historical Event Instance is approved. | Prepare and redeliver only that immutable identity. | The smallest approved replay scope completes. | Replay Management Service |
| Correlated-process replay | The approved scope uses one Correlation Identifier. | Select the complete governed correlation set and preserve required ordering. | The approved process history is redelivered without modification. | Replay Management Service |
| Consumer-specific replay | The need concerns one consumer. | Limit redelivery to that consumer and preserve all other consumer histories. | Consumer isolation is maintained. | Event Infrastructure; Consumer Owner |
| Replay cancellation | An authorized cancellation condition occurs before completion. | Stop further redelivery, preserve completed replay outcomes and transition to `Cancelled`. | The partial replay history is retained; the scenario closes as cancelled. | Replay Management Service; authorized actor |
| Replay execution fails | The approved scope cannot complete. | Stop progression, preserve processed/failed outcomes and transition to `Failed`. | The failed replay remains auditable and may require a new governed request. | Replay Management Service |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Requestor or Tenant scope cannot be resolved. | Replay isolation and authority cannot be established. | Reject the request without redelivery. | Event Infrastructure; Platform & Tenant; Person & Identity |
| Requested scope is ambiguous or exceeds the approved purpose. | Historical events cannot be selected safely. | Return the request for correction; remain outside `Prepared`. | Replay Management Service; VAR:Replay-Requesting Domain |
| Historical events are outside governed availability. | The approved scope cannot be assembled. | Reject or narrow the request through a new authorized decision. | Event Infrastructure; Authorization & Policy |
| Authorization or governance approval is denied. | Replay cannot proceed. | Retain the denied outcome; do not prepare or execute redelivery. | Authorization & Policy; Event Infrastructure |
| Proposed replay would modify original facts or identity. | Historical integrity would be violated. | Reject the request. | Event Infrastructure |
| Ordering or Tenant isolation cannot be preserved. | Redelivery could create an invalid or unauthorized outcome. | Do not execute; return the governed failure. | Replay Management Service; Platform & Tenant |
| Consumer is incompatible or ineligible. | Approved delivery cannot be completed safely. | Exclude or correct the scope only through renewed governance; do not change the event. | Event Infrastructure; Consumer Owner; Authorization & Policy |
| Replay is interrupted. | The approved scope is incomplete. | Preserve progress and finish as `Cancelled` or `Failed` according to the authorized outcome. | Replay Management Service |

No transport, batching, scheduling or infrastructure-recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Replay uses historical events; it does not generate new business facts. | Event Infrastructure; originating domain retains fact ownership | Original event identity and semantics remain authoritative. | L2-A06 §44; RP-01–RP-02 |
| Historical events cannot be modified. | Event Infrastructure | Payload, original version and publication context remain immutable. | L2-A06 RP-02; EIP-01 |
| Replay requires authorization through Authorization & Policy. | Authorization & Policy | Unapproved redelivery is prohibited. | L2-A06 RP-03; SEC-02 |
| Replay scope must be explicit before execution. | Event Infrastructure | Redelivery cannot exceed the approved event/definition/version/time/consumer/correlation/Tenant scope. | L2-A06 §44.3 |
| Required original ordering is preserved. | Event Infrastructure | Sequential business meaning cannot be reordered. | L2-A06 EO-04; RP-01 |
| Replay remains Tenant isolated. | Event Infrastructure; Platform & Tenant | Cross-Tenant redelivery is prohibited. | L2-A06 MT-04; SEC-04 |
| Replay must not disrupt normal delivery or unrelated consumers. | Event Infrastructure | Recovery remains isolated. | L2-A06 RP-05 |
| Consumers remain duplicate tolerant and preserve traceability. | Consumer Owner | Replayed identity cannot create inconsistent duplicate outcomes. | L2-A06 CR-02; CR-05; ID-03 |
| Every replay operation is fully auditable. | Event Infrastructure; Audit Infrastructure | Requestor, approval, scope and result are recorded. | L2-A06 RP-04 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Historical Event Instance | Previously `Published` and immutable | Selected by reference and redelivered. | Original record remains unchanged; operational lifecycle recognizes `Replayed`. | Event Infrastructure preserves; originating domain owns fact |
| Replay Session | Does not exist | Create from the governed request. | `Requested` | Replay Management Service |
| Replay Session | `Requested` | Apply authorization and governance approval. | `Authorized` | Replay Management Service; Authorization & Policy decides access |
| Replay Session | `Authorized` | Resolve exact approved scope. | `Prepared` | Replay Management Service |
| Replay Session | `Prepared` | Begin controlled redelivery. | `Executing` | Replay Management Service |
| Replay Session | `Executing` | Finish, cancel or record unsuccessful execution. | `Completed`, `Cancelled` or `Failed` | Replay Management Service |
| Consumer-owned aggregate | Domain-defined state | May change only through consumer-owned processing of the replayed immutable event. | Domain-defined outcome. | Applicable consuming domain |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Replay does not publish a new business event for the original fact. It redelivers the original canonical Event Instance while preserving its identity, version, timestamp, correlation, causation and Tenant context.

L2-A06 defines replay lifecycle states but no exact canonical replay lifecycle-event names. No `ReplayRequested`, `ReplayAuthorized` or `ReplayCompleted` event is invented.

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Contract / Hand-off | Business Information / Outcome |
|---|---|---|---|
| VAR:Replay-Requesting Domain | Replay Management Service | Replay request | Purpose, requested scope, Tenant, consumer/correlation context and requestor. |
| Authorization & Policy | Replay Management Service | Replay authorization/governance decision | Approved or denied scope and protected-action outcome. |
| Historical Event Repository | Replay Management Service | Immutable historical Event references | Original identity, definition/version, timestamp, correlation, causation and Tenant. |
| Replay Management Service | Approved consumer | Original canonical Event Contract and replay context | Controlled redelivery without business-fact mutation. |
| Replay Management Service | VAR:Replay-Requesting Domain | Replay status and final outcome | Scope, processed/failed result and `Completed`, `Cancelled` or `Failed` status. |
| Event Infrastructure | Audit Infrastructure | Replay governance and lifecycle evidence | Immutable request, approval, execution and outcome history. |

L2-A06 defines no canonical replay commands; these contracts remain business hand-offs pending L4 clarification.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Replay initiation, execution, cancellation and historical/dead-letter access are privileged operations.
- Requestors and operational actors must be authenticated.
- Authorization & Policy supplies the decision; Replay Management owns lifecycle execution.
- Approval is limited to the explicit Tenant and replay scope.

## 15.2 Approval and Separation of Duty

- Governance approval is mandatory before the Replay Session becomes `Authorized`.
- L2-A06 does not identify the universal approving role or define scenario-specific SoD/override rules.
- The Operations Engineer, Platform Administrator and Security Administrator responsibilities cannot be converted into an invented approval hierarchy.

## 15.3 Audit Obligations

Requestor, approval, purpose, Tenant, exact scope, selected Event identities/versions, consumer, ordering requirements, start and completion time, events processed/failed, cancellation/failure reason where defined, actors and final status must be auditable. Original event and audit evidence remain immutable.

---

# 16. Completion Criteria and Postconditions

The scenario is complete when:

- an eligible historical replay need and explicit scope were received;
- identity, Tenant, purpose and historical availability were validated;
- authorization and governance approval succeeded;
- the scope was prepared without changing original facts;
- original identity, version, timestamp, correlation, causation and Tenant context were preserved;
- required ordering and consumer isolation were maintained;
- the Replay Session reached `Completed`, `Cancelled` or `Failed`;
- the requesting domain received the governed outcome; and
- complete replay history was preserved.

Postconditions:

- No new business fact was created by replay.
- Original Event Instances and metadata remain unchanged.
- Replayed consumer processing remains owned by the applicable consuming domain.
- Normal delivery and unrelated consumers remain unaffected.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A021-ISSUE-01:** L2-A06 requires replay requestor and approval evidence but does not name a universal Replay Requestor or governance approving role. | The policy decision and lifecycle are sufficient for the scenario, but the exact human authority cannot be mapped for L4/RBAC. | Define the role-to-replay-request/approval authority mapping without changing domain participation. |
| **A021-ISSUE-02:** L2-A06 defines replay lifecycle states and business hand-offs but no canonical replay commands or lifecycle events. | The governed outcome is clear, but exact L4 operation contracts and event-driven notifications are unavailable. | Define replay commands and, where required, event names with one canonical publisher. |
| **A021-ISSUE-03:** The frozen trigger describes an “authorized ... need,” while the L2 replay lifecycle begins `Requested → Authorized`. | Authorization may describe need legitimacy or the lifecycle decision; the sequencing terminology is ambiguous. | Clarify whether trigger-time authorization is preliminary eligibility or the formal Replay Session authorization transition. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
