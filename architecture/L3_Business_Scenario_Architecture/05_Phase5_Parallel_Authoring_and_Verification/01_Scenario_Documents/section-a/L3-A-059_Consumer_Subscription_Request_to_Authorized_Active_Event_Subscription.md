# Consumer Subscription Request to Authorized Active Event Subscription

# 1. Scenario Metadata

**Scenario ID:** L3-A-059  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Event Infrastructure  
**Participating Domains:** A  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Platform and business consumers of canonical events

---

# 2. Scenario Purpose

Govern an identified consumer's subscription to active canonical Event Definitions through Tenant, authorization, version-compatibility and delivery-policy review until activation or an evidenced non-activation outcome.

---

# 3. Business Outcome

Authorization, version compatibility, tenant scope and delivery-policy review complete and the subscription becomes Active, or an evidenced denial or return prevents delivery

Source assessment: `PASS WITH ISSUES`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An identified platform or business consumer requires governed access to one or more active canonical event definitions.

## 4.2 End Boundary

The Event Subscription is `Active` and eligible for routing/delivery under its approved versions, Tenant scope and policies, or the request is denied/returned with evidence and no delivery begins.

## 4.3 In Scope

- Consumer and Tenant identification.
- Active Event Definition discovery.
- Supported-version and compatibility declaration.
- Authorization and subscription-governance review.
- Delivery preferences and Retry Policy reference.
- Approval and activation or evidenced rejection/return.
- Subscription history and audit evidence.

## 4.4 Out of Scope

- Creating or changing the Event Definition.
- Publishing an event.
- Delivery execution, retry, DLQ or replay.
- Consumer business processing.
- Broker, topic, API, database or UI implementation.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Upstream | L3-A-018 — Event Definition Proposal to Active Canonical Event Contract | Supplies the active definition. |
| Followed by | L3-A-019 — Committed Business Fact to Reliable Consumer Delivery | Owns delivery after a valid subscription. |
| Related | L3-A-060 — Active Event Contract Change | Governs later version transition and consumer migration. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Event Infrastructure / Section A | Owns Event Subscription lifecycle, registry discovery, compatibility validation and routing eligibility. |
| Authorization & Policy / Section A | Supplies subscription authorization and approval decisions. |
| Authentication / Section A | Supplies authenticated actor/consumer context. |
| Platform & Tenant / Section A | Supplies Tenant isolation and configuration. |
| Audit Infrastructure / Section A | Preserves immutable subscription evidence. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Consumer Owner | Requests and maintains the subscription and supported versions. | Cannot activate an unauthorized subscription. |
| Platform Administrator | Operates approved subscription controls. | Does not own consumer business processing. |
| Security Administrator | Oversees subscription security policy. | Authorization decisions remain under A04. |
| Domain Owner | Owns event semantics. | Publisher approval is not required unless governance policy says so. |

---

# 7. Preconditions and Entry Criteria

- The consumer identity and one Tenant scope are known.
- Requested Event Definitions are registered and `Active`.
- Requested versions and compatibility capabilities are declared.
- Authorization, subscription, delivery and Retry Policy rules are available.
- Duplicate active subscription rules can be evaluated.

---

# 8. Trigger

An identified platform or business consumer requires governed access to one or more active canonical event definitions

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the consumer's subscription request. | Consumer Owner; Event Infrastructure | Subscription enters `Requested`. |
| 2 | Resolve authenticated consumer identity and Tenant scope. | Authentication; Platform & Tenant | Request boundary is established. |
| 3 | Discover the requested active Event Definitions in the registry. | Event Registry Service | Canonical contracts are identified. |
| 4 | Record supported versions, delivery preferences, Retry Policy and security requirements. | Consumer Owner; Subscription Management Service | Complete subscription metadata exists. |
| 5 | Transition to `Under Review` and validate Tenant isolation and duplicate-subscription rules. | Event Infrastructure | Structurally valid request progresses. |
| 6 | Validate version compatibility with each requested Event Definition/Version. | Event Infrastructure | Consumer compatibility is established. |
| 7 | Request and apply the subscription authorization decision. | Authorization & Policy | Unauthorized access is denied. |
| 8 | Complete applicable subscription-governance approval. | Event Infrastructure; Authorization & Policy | Subscription reaches `Approved`, or is returned/denied. |
| 9 | Activate the approved subscription. | Subscription Management Service | Subscription reaches `Active`. |
| 10 | Make the Active subscription available to deterministic routing. | Event Routing Service | Future matching publications may create deliveries. |
| 11 | Preserve request, review, approval/denial, activation and correlation history. | Event Infrastructure; Audit Infrastructure | Outcome is auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome |
|---|---|---|---|
| Request returned | Metadata or supported versions are incomplete. | Return for correction without activation. | No delivery occurs. |
| Authorization denied | Consumer lacks permission. | Record evidenced denial. | Subscription does not become Active. |
| Duplicate active subscription | Policy prohibits another equivalent subscription. | Reject or reference the existing subscription. | Duplicate delivery is prevented. |
| Multiple versions | Consumer supports an approved range. | Record every supported version explicitly. | Routing respects compatibility. |
| Later suspension | Temporary governance condition applies. | Transition `Active → Suspended`. | Delivery eligibility pauses without erasing history. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Event Definition is not Active. | New subscription cannot be valid. | Reject or wait for active contract availability. | Event Infrastructure |
| Tenant scope is invalid. | Isolation would be violated. | Deny request. | Platform & Tenant; Event Infrastructure |
| Version compatibility fails. | Consumer cannot process the contract safely. | Return for compatible version declaration or deny. | Event Infrastructure |
| Authorization is unavailable. | Subscription cannot be approved. | Do not activate. | Authorization & Policy |
| Activation is interrupted. | Routing eligibility is uncertain. | Retain non-Active state and revalidate; exact command recovery is undefined. | Event Infrastructure |

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|
| Consumers subscribe to Event Definitions, not publishers. | Publisher and consumer remain decoupled. | L2-A06 §34.4 |
| Authorization is required before activation. | Unauthorized consumers receive no events. | L2-A06 §34.4; §52 |
| Supported versions are explicit. | Routing can enforce compatibility. | L2-A06 §34.3–34.4 |
| Tenant boundaries apply to subscription and delivery. | Cross-Tenant delivery is prohibited by default. | L2-A06 EIP-07, SMG-05 |
| Subscription lifecycle transitions are auditable. | Complete request and decision history is retained. | L2-A06 SMG-04 |
| Publisher approval is unnecessary unless policy requires it. | Independent registration is preserved. | L2-A06 SMG-01 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Event Subscription | Absent | `Requested → Under Review → Approved → Active`. | Active routing eligibility. |
| Denied/returned request | Requested/Under Review | Stop before Active and preserve evidence. | No delivery. |
| Event Definition/Version | Active | Referenced without modification. | Unchanged. |
| Consumer | Identified | Registered as subscription owner. | No consumer-domain state transfer. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Event Determination

L2-A06 defines the Event Subscription lifecycle and service responsibilities but no canonical SubscriptionRequested, SubscriptionApproved, SubscriptionActivated, SubscriptionDenied or SubscriptionReturned events and publishers. No event is invented.

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|
| Event registry contract | Event Registry Service | Consumer Owner | Active definitions, versions and schemas. |
| Authorization decision | Authorization & Policy | Subscription Management | Approved or denied consumer access. |
| Active subscription | Subscription Management | Event Routing Service | Consumer, definition, versions, Tenant and policy. |
| Audit input | Event Infrastructure | Audit Infrastructure | Request-to-outcome evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Subscription creation is privileged and requires authentication, authorization, least privilege and Tenant isolation.

## 15.2 Approval and Separation of Duty

Subscription approval is policy-controlled. L2-A06 names operational roles but does not allocate the exact approval transition or SoD rule.

## 15.3 Audit Obligations

Record consumer, Tenant, definitions, versions, compatibility, delivery preferences, Retry Policy, authorization, approval/denial/return, lifecycle, actor, timestamp and correlation.

---

# 16. Completion Criteria and Postconditions

- Consumer and Tenant were resolved.
- Requested definitions were Active.
- Supported versions and delivery policy were valid.
- Authorization and governance review completed.
- The subscription became Active or an evidenced denial/return prevented delivery.
- Only Active subscriptions participate in routing.
- Subscription history is auditable.

Postconditions: publishers remain unaware of consumers; no event publication or delivery is performed by this scenario.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|
| A059-ISSUE-01 | Non-blocking | L2-A06 defines lifecycle states and service responsibilities but no subscription commands, transition matrix or canonical lifecycle events/publishers. | Use the explicit state order and synchronous contracts; define commands and events before L4. |
| A059-ISSUE-02 | Non-blocking | `Denied` and `Returned` are frozen terminal outcomes but not named Subscription lifecycle states. | Treat them as evidenced request-review outcomes before activation, not invented aggregate states. |

No subscription command, event, state or delivery implementation is invented.
