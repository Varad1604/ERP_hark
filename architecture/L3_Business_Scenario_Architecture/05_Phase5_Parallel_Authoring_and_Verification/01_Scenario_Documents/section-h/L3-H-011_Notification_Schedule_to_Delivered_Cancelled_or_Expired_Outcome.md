# Notification Schedule to Delivered, Cancelled, or Expired Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-011  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Notification  
**Participating Domains:** H  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** L2-H02 — Notification Workflows  
**Referenced By:** Scheduled, delayed, recurring, calendar-based, and policy-driven notification activities

# 2. Scenario Purpose

Govern an authorized H02 Notification Schedule from creation through execution, cancellation, or expiry so that scheduled and recurring communications occur only within their permitted delivery window and every resulting Notification instance remains traceable.

# 3. Business Outcome

Notification instances execute within the governed window or reach a traceable cancelled or expired outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized notification is scheduled, delayed, or made recurring.

## 4.2 End Boundary

The scenario ends when the scheduled Notification instance executes to its governed delivery outcome or becomes `Cancelled` or `Expired` before eligible execution, with scheduling and notification evidence preserved.

## 4.3 In Scope

- Establishing an H02-owned Notification Schedule for an already authorized notification.
- Immediate/delayed/scheduled/recurring scheduling behavior relevant to the frozen scenario.
- Delivery-window and expiry governance.
- Cancellation before execution begins where permitted.
- Generation of independent Notification instances for recurring schedules.
- Exact H02 notification lifecycle events for scheduling, cancellation, expiry, and delivery.

## 4.4 Out of Scope

- Authoring or approving notification content/templates.
- Changing source business transactions that caused the notification.
- Defining technical scheduling infrastructure or execution mechanics.
- Delivery-failure recovery after execution begins, which belongs to a separate scenario.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Notification / Section H | Owns Notification Schedule, Notification identity/lifecycle, execution eligibility, cancellation/expiry behavior, and resulting notification events/evidence. |

# 6. Business Actors, Roles and Responsibilities

Locked H02 contains no canonical Actors section. The trigger guarantees an already authorized notification, but the business actor who establishes or changes a schedule is not defined upstream. No placeholder scheduling role is introduced.

# 7. Preconditions and Entry Criteria

- A governed H02 Notification exists or is authorized for scheduled creation.
- Required recipient, template, and channel context is valid for the Notification.
- Scheduling mode and delivery window are governed by H02 configuration/policy.
- The requested schedule has not already expired.

# 8. Trigger

An authorized notification is scheduled, delayed, or made recurring

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Notification Schedule for the already authorized notification without changing Notification identity. | Notification | Schedule is governed within H02. |
| 2 | Determine the permitted scheduling mode, timing context, delivery window, and expiry conditions from H02 policy. | Notification | Schedule becomes eligible for source-defined execution behavior. |
| 3 | Record the scheduled Notification state and preserve schedule traceability. | Notification | `NotificationScheduled` communicates the completed scheduling fact where applicable. |
| 4 | When the governed execution window is reached and the schedule remains valid, allow the Notification instance to progress through the normal H02 delivery lifecycle. | Notification | Notification executes within the governed window. |
| 5 | For a recurring schedule, create each independent Notification instance under the source-defined recurring-schedule rule. | Notification | Each recurrence has its own Notification lifecycle while remaining traceable to the schedule context. |
| 6 | If cancellation or expiry occurs before eligible completion, transition the Notification to the exact source-defined terminal state and preserve evidence. | Notification | `NotificationCancelled` or `NotificationExpired` records the terminal outcome. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Delayed or calendar-based schedule | Notification is valid but not yet eligible for delivery. | Keep the schedule in its source-defined waiting path until the governed window begins. | Notification later executes or reaches cancellation/expiry. | Notification |
| Recurring schedule | Schedule is recurring. | Generate independent Notification instances for each governed recurrence. | Each instance is independently traceable. | Notification |
| Schedule cancelled | Cancellation occurs before execution begins. | Apply the source-defined cancellation path. | Notification/Schedule reaches `Cancelled` as applicable. | Notification |
| Schedule expires | Delivery window or validity expires before execution. | Do not execute the expired Notification. | Notification/Schedule reaches `Expired` as applicable. | Notification |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Scheduled time falls outside the governed delivery window | Notification is not eligible for normal execution. | Preserve schedule identity and apply the source-defined expiry/defer behavior; an expired schedule never executes. | Notification |
| Schedule is cancelled before execution | Communication should not proceed. | Record the cancellation and preserve prior schedule evidence. | Notification |
| Delivery fails after execution begins | Scheduling scenario has handed off to delivery lifecycle. | Preserve the delivery failure; later recovery belongs to the separate notification-recovery scenario. | Notification |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Scheduling shall not modify Notification identity. | Notification | Schedule changes never create a replacement identity for the same Notification. | L2-H02 §19.3 |
| Scheduled notifications remain cancellable until execution begins. | Notification | Cancellation is permitted only before execution begins. | L2-H02 §19.3 |
| Recurring schedules generate independent Notification instances. | Notification | Each recurrence has an independent Notification lifecycle. | L2-H02 §19.3 |
| Expired schedules shall never execute. | Notification | Expiry is terminal for schedule eligibility. | L2-H02 §§12.5, 19.3 |
| Schedule modifications remain auditable. | Notification | Scheduling history must remain traceable. | L2-H02 §19.3 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Notification Schedule | `Created`, `Scheduled`, or `Waiting` | Schedule progresses to execution, cancellation, or expiry. | `Completed`, `Cancelled`, or `Expired` according to source-defined schedule lifecycle. | Notification | L2-H02 §19.4 |
| Notification | `Scheduled` or subsequent delivery states | If eligible, Notification progresses through normal delivery; otherwise it may cancel or expire. | `Delivered`, `Cancelled`, or `Expired` for the frozen outcome; delivery failure is handled separately. | Notification | L2-H02 §§12, 19 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Applicable exact H02 events include `NotificationScheduled`, `NotificationCancelled`, `NotificationExpired`, `DeliveryStarted`, `DeliveryCompleted`, and `NotificationDelivered` according to the actual path. H02 remains the publisher of its notification lifecycle events.

## 14.2 Business Contracts and Cross-Domain Hand-offs

This Simple scenario is H-owned. It begins after notification authorization has already been established and does not require a new cross-domain business hand-off inside the schedule lifecycle.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The trigger establishes that the notification is authorized before scheduling begins. Locked H02 does not define the canonical human actor who schedules, changes, or cancels the schedule, so no actor is invented.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific approval chain or separation-of-duty rule is defined for Notification scheduling.

## 15.3 Business Audit and Evidence Obligations

H02 must preserve Notification identity, Schedule reference, scheduling mode/context, governed delivery window, schedule changes, cancellation or expiry outcome where applicable, Notification instance relationships for recurring schedules, lifecycle timestamps, and resulting Notification events/evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An authorized notification enters an H02 schedule.
- Scheduling does not change Notification identity.
- Delivery-window and expiry governance are applied.
- Recurring schedules create independent Notification instances where applicable.
- Each instance either executes within the governed window or reaches a traceable `Cancelled` or `Expired` outcome.
- Applicable H02 events/evidence are preserved.

## 16.2 Postconditions

- H02 remains canonical owner of the schedule and Notification lifecycle.
- Executed Notification instances retain their normal H02 delivery history.
- Cancelled or expired instances do not execute outside the governed schedule rules.
- Schedule history remains auditable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked H02 has no canonical Actors section and does not assign a business actor to schedule creation/change/cancellation. | L3 cannot state the human scheduling authority, although the frozen trigger begins after notification authorization. | Define H02 scheduling actors upstream if explicit human responsibility is required. |
| H02 publishes `NotificationScheduled`, `NotificationCancelled`, and `NotificationExpired`, but does not define separate canonical events for every Notification Schedule aggregate transition (`Created`, `Waiting`, `Executing`, `Completed`). | L3 must not invent schedule-specific events. | Add schedule-specific event definitions upstream only if cross-domain consumers require them. |

**Architecture assessment:** `PASS WITH ISSUES`

The Schedule lifecycle, Notification lifecycle, cancellation/expiry rules, recurring-instance rule, and applicable H02 events are explicit. Missing actor and schedule-specific event detail do not prevent completion of the H-owned Draft because the frozen trigger already establishes authorization.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
