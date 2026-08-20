# Failed Event Delivery to Controlled Recovery Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-A-020  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Event Infrastructure  
**Participating Domains:** A, VAR:Event-Consuming Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-019 — Committed Business Fact to Reliable Consumer Delivery  
**Referenced By:** L3-A-021 — Historical Event Replay Request to Controlled Redelivery Completion and operational recovery governance

---

# 2. Scenario Purpose

Establish the governed business-visible outcome for one failed consumer delivery while preserving the immutable published fact, Tenant isolation, consumer independence and complete traceability.

---

# 3. Business Outcome

Delivery is recovered through governed retry or isolated as a dead-lettered failure with complete traceability and without affecting successful delivery to other consumers.

The outcome is frozen, but its required retry, dead-letter and transport-failure mechanics are expressly prohibited at L3 by the locked template and validation standards. L2-A06 also lacks canonical recovery commands and lifecycle event names. The scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when delivery of a published event to an authorized consumer fails.

## 4.2 End Boundary

The intended end boundary is reached when the affected delivery is either successfully acknowledged after governed recovery or isolated as `Dead-Lettered`, while the original published event and every other consumer delivery remain unchanged and the outcome is traceable.

## 4.3 In Scope

- Identification and classification of one failed consumer-delivery outcome.
- Preservation of Event identity, version, Tenant, correlation, causation and consumer scope.
- Independent application of the governed recovery policy.
- Successful recovered delivery or governed dead-letter isolation.
- Operational investigation and authorized later replay eligibility.
- Complete failure and recovery traceability.

## 4.4 Out of Scope

- Defining retry schedules, counts, algorithms, transport behaviour, queue implementation or infrastructure recovery.
- Modifying the original published event or its metadata.
- Recreating the business fact.
- Changing another consumer's delivery lifecycle.
- Executing historical replay; that belongs to A021.
- Performing consumer-owned business correction.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Upstream | L3-A-019 — Committed Business Fact to Reliable Consumer Delivery | Supplies the published event and independent failed consumer delivery. |
| Followed by | L3-A-021 — Historical Event Replay Request to Controlled Redelivery Completion | May redeliver an authorized historical or dead-lettered event without mutation. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Event Infrastructure / Section A | Primary owner. Owns Event Delivery, Retry Policy, Dead-Letter Queue, recovery coordination, isolation and operational history. |
| VAR:Event-Consuming Domain | Owns consumer processing behaviour and supplies the authoritative acknowledgement or failure outcome. |
| Platform & Tenant / Section A | Supplies Tenant context and isolation. |
| Person & Identity / Section A | Supplies authorized operational actor references. |
| Authentication / Section A | Establishes authenticated context for privileged recovery actions. |
| Authorization & Policy / Section A | Supplies authorization decisions for recovery and dead-letter access. |
| Audit Infrastructure / Section A | Owns immutable evidence generated from failure and recovery activity. |

The frozen Participating Domains value remains exactly `A, VAR:Event-Consuming Domain`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Consumer Owner | Maintains consumer processing behaviour and provides the consumer outcome. | Cannot modify the event or another consumer's delivery. |
| Operations Engineer | Monitors failures, investigates and coordinates recovery. | Recovery actions remain authorized, policy governed and auditable. |
| Platform Administrator | Operates governed Event Infrastructure controls. | Does not own the business fact or consumer decision. |
| Security Administrator | Oversees privileged access and security policy. | Does not select a business processing outcome. |

L2-A06 does not map these roles to canonical recovery commands or approval transitions.

---

# 7. Preconditions and Entry Criteria

- One immutable canonical event was published under A019.
- An authorized consumer had an independent eligible delivery.
- That delivery did not receive a successful consumer acknowledgement.
- The Event identity, version, Tenant, correlation, causation, consumer identity and failure outcome are preserved.
- Applicable recovery and access policies can be resolved.
- Other consumer deliveries remain independently governed.

---

# 8. Trigger

Delivery of a published event to an authorized consumer fails.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Recognize the unsuccessful consumer-delivery outcome without changing the published event. | Event Infrastructure; VAR:Event-Consuming Domain | One failed delivery is identified. |
| 2 | Preserve Event identity, version, Tenant, correlation, causation, consumer and failure context. | Event Infrastructure | Recovery traceability and isolation are established. |
| 3 | Confirm that the failure affects only the relevant consumer delivery. | Event Delivery Service | Successful and continuing deliveries to other consumers remain unaffected. |
| 4 | Determine whether the condition is eligible for governed recovery under the applicable policy. | Event Infrastructure; Authorization & Policy where privileged action applies | A recovery or isolation outcome is selected without altering business facts. |
| 5 | Where governed recovery is permitted, coordinate a new delivery attempt without defining its technical mechanism at L3. | Event Infrastructure; VAR:Event-Consuming Domain | The consumer may provide a new authoritative acknowledgement or failure outcome. |
| 6 | If the consumer acknowledges success, complete the affected delivery and preserve its recovery history. | Event Infrastructure; VAR:Event-Consuming Domain | Delivery is recovered and completed. |
| 7 | If recovery is exhausted or prohibited, isolate the affected delivery as `Dead-Lettered` without modifying the original event. | Event Infrastructure | The permanent delivery failure is contained and available for governed investigation/replay. |
| 8 | Preserve the failure classification, policy decision, actors, attempts, consumer outcomes and final recovery/isolation outcome. | Event Infrastructure; Audit Infrastructure | The outcome remains completely traceable. |

Steps 4–7 cannot be completed as exact L3 contracts because the frozen scenario depends on prohibited technical recovery concepts and L2-A06 defines no canonical recovery commands or events.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Recovery succeeds | A governed subsequent delivery obtains consumer acknowledgement. | Complete only the affected delivery and record the recovery outcome. | Delivery reaches its successful completion outcome. | Event Infrastructure; VAR:Event-Consuming Domain |
| Failure becomes permanent | Recovery policy does not permit further progression. | Isolate the affected delivery as `Dead-Lettered`. | The original event remains immutable and the failure becomes eligible for governed investigation/replay. | Event Infrastructure |
| Consumer version is unsupported | Consumer rejects the Event Version under its contract responsibilities. | Treat the delivery as failed and apply only the governed recovery/isolation policy. | No incompatible payload change is introduced. | VAR:Event-Consuming Domain; Event Infrastructure |
| Other consumers succeed | One or more independent deliveries are already complete. | Preserve those outcomes and operate only on the failed consumer delivery. | Consumer isolation is maintained. | Event Infrastructure |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Failure context is incomplete or crosses Tenant boundaries. | Safe governed recovery cannot be established. | Do not progress recovery; preserve the unresolved delivery for authorized investigation. | Event Infrastructure; Platform & Tenant |
| Recovery action is unauthorized. | The delivery cannot be re-attempted or managed by that actor. | Deny the action and retain the existing governed state. | Authorization & Policy; Event Infrastructure |
| Consumer outcome remains ambiguous. | Successful completion cannot be asserted. | Retain the failed outcome; do not fabricate acknowledgement. | VAR:Event-Consuming Domain; Event Infrastructure |
| Governed recovery does not succeed. | Delivery remains incomplete. | Apply the authoritative isolation outcome when policy conditions are met. | Event Infrastructure |
| Dead-letter access is unauthorized. | The isolated delivery cannot be inspected or managed by that requester. | Deny access without altering the record. | Authorization & Policy; Event Infrastructure |
| Recovery history cannot be preserved. | Complete traceability required by the frozen outcome is absent. | Do not declare successful recovery or closure. | Event Infrastructure; Audit Infrastructure |

No retry count, schedule, transport handling, queue operation or infrastructure mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Each consumer has an independent delivery lifecycle. | Event Infrastructure | One failed consumer cannot affect another. | L2-A06 DC-01; DS-02 |
| Delivery completes only after consumer acknowledgement. | Event Infrastructure; VAR:Event-Consuming Domain | Failure cannot be reported as delivered. | L2-A06 DS-03; CR-03 |
| Published events and metadata remain immutable. | Event Infrastructure; Event-Owning Domain | Recovery never edits the business fact. | L2-A06 EP-03; EM-02 |
| Recovery is controlled by platform policy and cannot be unbounded. | Event Infrastructure | Only governed progression is permitted. | L2-A06 RM-02 |
| Exhausted failures transition to dead-letter isolation. | Event Infrastructure | Permanent failure is contained rather than repeatedly processed. | L2-A06 RM-05; DLQ principles |
| Dead-letter access is controlled. | Event Infrastructure; Authorization & Policy | Only authorized actors/services may inspect or manage the isolated delivery. | L2-A06 DLQ-03 |
| Recovery and isolation history remain traceable. | Event Infrastructure; Audit Infrastructure | Every outcome must be auditable. | L2-A06 RM-03; DLQ objectives |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Published Event Instance | `Published` and immutable | Referenced by the failed delivery; no payload or identity change. | Remains unchanged. | Event Infrastructure preserves; Event-Owning Domain owns fact |
| Consumer delivery | `Delivering` without successful acknowledgement | Enter governed failure handling. | L2-A06 processing lifecycle uses `Retry Pending` / `Retrying`. | Event Delivery Service |
| Consumer delivery | Governed recovery succeeds | Record consumer acknowledgement. | `Acknowledged → Completed` / Event Instance delivery `Delivered` | Event Delivery Service |
| Consumer delivery | Recovery is exhausted | Isolate permanent delivery failure. | `Dead-Lettered` | Event Delivery Service |
| Other consumer deliveries | Independent states | No transition caused by this failure. | Unchanged. | Event Delivery Service |

The retry states are recorded only because they are upstream-defined; their mechanics are outside L3.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-A06 defines failed-delivery, retry and dead-letter lifecycle states but no exact canonical business-event names or publishers for those transitions. No `DeliveryFailed`, `RetrySucceeded` or `DeadLettered` event is invented.

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Contract / Hand-off | Business Information / Outcome |
|---|---|---|---|
| VAR:Event-Consuming Domain | Event Infrastructure | Consumer processing outcome | Acknowledgement or governed failure result for the immutable event identity. |
| Event Infrastructure | Authorization & Policy | Privileged recovery/dead-letter action context | Tenant, consumer, actor and requested lifecycle action. |
| Authorization & Policy | Event Infrastructure | Authorization decision | Allow or deny result. |
| Event Infrastructure | VAR:Event-Consuming Domain | Recovery or isolation outcome | Successful completion or permanent failed-delivery status. |
| Event Infrastructure | Audit Infrastructure | Failure and recovery history | Correlated evidence without changing the business fact. |

L2-A06 does not define canonical commands for initiating, authorizing, completing or exhausting recovery.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Privileged recovery and dead-letter access require authentication and authorization.
- Consumer acknowledgement remains owned by the Consumer Domain.
- Event Infrastructure owns delivery state; Authorization & Policy owns access decisions.

## 15.2 Approval and Separation of Duty

- L2-A06 requires policy-governed recovery and operational approval for later replay but defines no universal recovery approver or scenario-specific SoD rule.
- No override or additional approval role is invented.

## 15.3 Audit Obligations

Event and delivery identity, Tenant, consumer, failure context, applicable policy, authorized actor, recovery progression, consumer outcomes, final successful completion or dead-letter isolation, correlation and timestamps must remain traceable.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are:

- one failed consumer delivery was isolated from all others;
- the immutable Event Instance, version, Tenant and traceability context were preserved;
- the applicable recovery policy and authorization were evaluated;
- governed recovery either obtained successful consumer acknowledgement or reached dead-letter isolation;
- other consumer deliveries were unaffected; and
- complete failure and recovery history was preserved.

These criteria cannot be validated as a complete L3 outcome until the conflict between the frozen technical outcome and the locked L3 boundary is resolved and canonical recovery contracts are defined.

Postconditions:

- Successful recovery completes only the affected consumer delivery.
- Dead-letter isolation preserves the original event for authorized investigation/replay.
- No business fact, Event Version or other consumer outcome is modified.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A020-BLOCKER-01:** The frozen outcome requires “governed retry” and a “dead-lettered failure,” while the locked L3 template prohibits technical retries, queues, transport failures and infrastructure recovery. | The frozen semantics must remain verbatim, but their defining behaviour cannot be expanded or validated at L3. | Reframe A020 as a governed delivery-resolution business outcome or formally authorize an L3 operational abstraction while deferring mechanisms to L4/L5. |
| **A020-BLOCKER-02:** L2-A06 defines retry and dead-letter states/policies but no canonical recovery commands, inputs, transition authority or exact recovery event names/publishers. | L4 cannot derive the authoritative operations and event contracts required to execute the outcome. | Define canonical delivery-recovery commands and, where required, lifecycle events with one publisher. |
| **A020-ISSUE-01:** L2-A06 recognizes logical Operations Engineer and Platform Administrator roles but does not map them to recovery initiation, approval, exhaustion or dead-letter-resolution authority. | The L3 responsibility boundary is incomplete even though generic operational responsibilities exist. | Add an explicit role-to-transition authority matrix. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
