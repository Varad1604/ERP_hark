# Failed Notification Delivery to Recovery or Terminal Failure

# 1. Scenario Metadata

**Scenario ID:** L3-H-015  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Notification  
**Participating Domains:** A, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H02 — Notification Workflows; applicable Section A authorization, event, and audit architecture  
**Referenced By:** Notification recovery, delivery governance, provider receipt processing, operational review, and audit activities

# 2. Scenario Purpose

Govern a failed notification delivery to a conclusive H02 recovery or terminal-failure outcome while preserving the original Notification identity, immutable delivery-attempt history, and originating business outcome.

# 3. Business Outcome

Retry, fallback, dead-letter recovery, or permanent failure completes under policy without duplicate notification identity

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a channel or provider delivery attempt fails.

## 4.2 End Boundary

The scenario ends when H02 reaches a governed recovery completion or terminal failure for the existing Notification identity, while all prior delivery evidence remains traceable and the originating business transaction remains unchanged.

## 4.3 In Scope

- Classifying the failed H02 delivery outcome.
- Determining whether governed re-execution, alternate-provider/channel fallback, controlled terminal recovery, or permanent failure applies.
- Preserving Notification identity and correlation through recovery.
- Creating additional immutable delivery-attempt evidence only as permitted by H02.
- Recording the conclusive delivery outcome and canonical notification lifecycle event.

## 4.4 Out of Scope

- Repeating or rolling back the originating business transaction.
- Changing the source business event or decision because communication failed.
- Defining retry counts, timing algorithms, queue implementation, or provider technology.
- Creating a new Notification identity merely because delivery recovery is required.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains canonical authorization, event, and audit infrastructure ownership. |
| Notification / Section H | Owns Notification identity/lifecycle, Delivery, Delivery Attempt history, retry/fallback governance, controlled terminal recovery, and final notification outcome. |

# 6. Business Actors, Roles and Responsibilities

No universal human actor is required for the policy-driven recovery branches. Locked H02 states that operational personnel may reprocess eligible terminal-recovery entries but does not define a canonical business role for that action. No role is invented.

# 7. Preconditions and Entry Criteria

- A canonical H02 Notification already exists.
- A governed Delivery Attempt has failed.
- Notification identity, correlation context, failure classification, and prior attempt evidence are available.
- The originating business transaction remains owned and complete or otherwise governed by its source domain independently of notification delivery.

# 8. Trigger

A channel or provider delivery attempt fails

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Associate the failed delivery result with the existing Notification and immutable Delivery Attempt history. | Notification / Section H | Recovery remains tied to the original Notification identity. |
| 2 | Classify the failure under H02's canonical failure categories and determine whether governed recovery remains permitted. | Notification / Section H | A source-defined recovery or terminal path is selected. |
| 3 | Where policy permits, perform controlled re-execution or provider/channel fallback without recreating the Notification. | Notification / Section H | A new delivery outcome is attempted while prior evidence is preserved. |
| 4 | Where ordinary recovery is exhausted or not permitted, route the notification into H02's governed terminal-recovery handling or permanent failure position. | Notification / Section H | The unresolved failure reaches a controlled governed position. |
| 5 | Record the conclusive delivery and Notification lifecycle outcome and preserve immutable recovery history. | Notification / Section H | Recovery or terminal failure is traceable. |
| 6 | Publish the applicable exact H02 lifecycle event. | Notification / Section H | Consumers receive the communication outcome without any repeat of the source business transaction. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Temporary failure | H02 classifies the failure as recoverable. | Apply governed re-execution under the same Notification identity. | Recovery continues with additional immutable attempt evidence. | Notification |
| Provider-specific failure | An alternative provider/channel is eligible under policy. | Apply governed fallback while preserving Notification identity. | Delivery may complete through the alternative path. | Notification |
| Recovery exhausted | No permitted ordinary recovery remains. | Move to controlled terminal-recovery handling described upstream. | Notification remains traceable for governed follow-up. | Notification |
| Permanent failure | Failure is non-recoverable. | Record permanent failure. | Notification reaches its terminal Failed position. | Notification |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| A repeated recovery attempt would create duplicate notification identity | Notification history would become inconsistent. | Preserve the existing Notification identity; only delivery-attempt history may increase under H02 rules. | Notification |
| A failure is permanent | Delivery cannot be completed through permitted recovery. | End recovery and record the terminal H02 failure outcome. | Notification |
| Recovery requires human operational follow-up | Policy-driven recovery cannot conclude automatically. | Preserve evidence and route to governed operational follow-up; exact business role is unresolved upstream. | Notification |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Delivery failures shall never alter business state. | Notification | Communication failure cannot roll back or change the originating transaction. | L2-H02 §25.4 |
| Delivery retries preserve Notification identity and correlation. | Notification | Recovery continues under the same canonical Notification. | L2-H02 §§12.3, 27.3 |
| Each recovery execution creates a distinct immutable Delivery Attempt rather than rewriting history. | Notification | Prior failure evidence remains intact. | L2-H02 §26.3 |
| Permanent failures terminate retry execution. | Notification | Non-recoverable failure becomes conclusive. | L2-H02 §27.3 |
| Provider failover preserves Notification identity and delivery traceability. | Notification | Fallback cannot create a competing notification identity. | L2-H02 §30.2 |
| Recovery of exhausted failures must preserve immutable historical delivery attempts. | Notification | Controlled terminal recovery cannot overwrite prior evidence. | L2-H02 §28.3 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Notification | `Delivering`, `PartiallyDelivered`, or a failure position subject to recovery | Existing Notification continues through governed recovery. | `Delivered`, `PartiallyDelivered`, or terminal `Failed` according to H02 lifecycle. | Notification | L2-H02 §12 |
| Delivery | Failed or otherwise incomplete delivery | Governed recovery/fallback may be applied. | `Delivered`, `Failed`, `Cancelled`, or `Expired` as permitted by H02. | Notification | L2-H02 §25.3 |
| Delivery Attempt | Completed failed attempt | Remains immutable; any further attempt is a separate record. | Historical attempt unchanged. | Notification | L2-H02 §26 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Applicable H02 lifecycle events include `DeliveryFailed`, `NotificationRetryStarted`, `NotificationRetryCompleted`, `DeliveryCompleted`, and `NotificationDelivered` as dictated by the actual outcome. H02 defines no distinct canonical event for provider fallback or terminal-recovery entry; none is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The originating domain receives notification outcomes without H02 changing the originating business transaction. Provider receipt/result information is normalized within H02's delivery governance; provider-specific implementation details remain outside L3.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Recovery eligibility, fallback, and permanent-failure treatment follow H02 policy. Where governed operational reprocessing is required, the exact canonical business role is not defined upstream.

## 15.2 Separation of Duties, Overrides and Exceptions

No new override or separation-of-duty rule is introduced. Notification recovery never authorizes repetition of the originating business transaction.

## 15.3 Business Audit and Evidence Obligations

Evidence must preserve Notification ID, correlation/causation context, each immutable Delivery Attempt, failure classification, provider/channel outcome, recovery path, final lifecycle position, and any governed operational follow-up. Platform Audit retains canonical audit persistence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The failed attempt is associated with the existing Notification identity.
- H02 failure classification and policy determine the permitted path.
- Any re-execution/fallback preserves Notification identity and prior attempt history.
- Recovery completes or a terminal failure/controlled recovery position is recorded.
- The originating business transaction is not repeated or altered.
- Applicable H02 lifecycle events and audit evidence are preserved.

## 16.2 Postconditions

- H02 holds the authoritative notification recovery/failure outcome.
- Prior delivery attempts remain immutable.
- No duplicate Notification identity is created.
- The originating business outcome remains owned by its source domain and unaffected by delivery failure.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome uses the term `dead-letter recovery`, while L3 validation prohibits technical queue mechanics. | L3 must preserve the frozen semantic option without describing queue implementation. | Keep the term only as the governed terminal-recovery concept; defer technical realization to L5. |
| H02 states that operational personnel may reprocess eligible terminal-recovery entries but does not define a canonical business role. | L3 cannot assign operational recovery authority to an invented actor. | Define the H02 operational recovery role upstream if human authority is required. |
| H02 standard events do not define a dedicated provider-fallback or terminal-recovery event. | L3 must use existing lifecycle events and cannot invent a more specific publisher/event pair. | Add canonical events upstream only if required. |

**Architecture assessment:** `PASS WITH ISSUES`

The failure classifications, Notification identity invariants, immutable attempt history, recovery/fallback rules, permanent failure, and non-impact on source business state are explicit. The actor/event gaps and implementation-leaning frozen term are non-blocking when L3 remains business-level.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
