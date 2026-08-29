# Notification Schedule to Delivered, Cancelled, or Expired Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-011  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Notification  
**Participating Domains:** H  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** L2-H02 — Notification Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern H02 scheduling for an already authorized Notification so delayed, scheduled, and recurring communications execute only when eligible, or end in a source-defined cancelled or expired outcome, without changing Notification identity.

# 3. Business Outcome

Notification instances execute within the governed window or reach a traceable cancelled or expired outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized notification is scheduled, delayed, or made recurring

## 4.2 End Boundary

The affected Notification instance executes through its governed H02 delivery path within the permitted window, or reaches the source-defined `Cancelled` or `Expired` outcome, with scheduling history preserved.

## 4.3 In Scope

- H02 scheduling eligibility and delivery-window governance.
- Cancellation before execution and expiry behavior.
- Recurring generation of independent Notification instances.
- Applicable H02 scheduling and Notification lifecycle facts.

## 4.4 Out of Scope

- Template approval or content governance.
- The originating business transaction or decision.
- Delivery-failure recovery after execution begins.

# 5. Ownership and Participating Domains

Notification / Section H owns Notification scheduling, Notification identity/lifecycle, scheduling eligibility, cancellation/expiry behavior, and H02 scheduling/delivery outcome evidence. No ownership transfer occurs because this is an H-only scenario.

# 6. Business Actors, Roles and Responsibilities

Locked H02 contains no canonical Actors section. The trigger establishes an already authorized Notification, but H02 does not name a canonical business actor for schedule creation, change, or cancellation. No actor is invented.

# 7. Preconditions and Entry Criteria

- A governed H02 Notification exists or is authorized for scheduled creation.
- Required recipient, template, and channel context is valid.
- Scheduling mode, delivery window, time context, and expiry are governed by H02 policy.
- The schedule has not already expired.

# 8. Trigger

An authorized notification is scheduled, delayed, or made recurring

# 9. Main Business Flow

1. H02 establishes the applicable delayed, scheduled, recurring, calendar-based, event-triggered, or policy-driven scheduling context without changing Notification identity.
2. H02 evaluates the governed execution window and expiry conditions.
3. The Notification enters the source-defined `Scheduled` position and `NotificationScheduled` is published where applicable.
4. When eligible, the Notification progresses into its normal delivery lifecycle.
5. A recurring schedule generates independent Notification instances under the source-defined recurrence rule.
6. If cancelled before execution, H02 records `Cancelled` and publishes `NotificationCancelled`; if validity expires first, H02 records `Expired` and publishes `NotificationExpired`.
7. Schedule and Notification history remain auditable.

# 10. Alternate Business Flows

- **Delayed execution:** the Notification remains scheduled until the governed window opens.
- **Recurring execution:** each recurrence produces an independent Notification instance.
- **Cancellation:** permitted before execution begins; prior schedule history is preserved.
- **Expiry:** an expired scheduled Notification does not execute.

# 11. Failure, Interruption and Business Recovery

If scheduling conditions become invalid, H02 preserves the existing Notification identity and applies only the source-defined cancellation, expiry, or defer behavior. A delivery failure after execution begins is not resolved here; it enters the separate H02 delivery-recovery outcome.

# 12. Business Rules and Decision Constraints

- Scheduling shall not modify Notification identity.
- Scheduled notifications remain cancellable until execution begins.
- Recurring schedules generate independent Notification instances.
- Expired scheduled notifications shall not execute.
- Schedule modifications remain auditable.
- Terminal Notification states are immutable.

# 13. State and Lifecycle Impact

H02 defines a scheduling lifecycle `Created → Scheduled → Waiting → Executing → Completed`, with cancellation/expiry branches, and the Notification lifecycle permits `Scheduled → Queued`, `Scheduled → Cancelled`, and `Scheduled → Expired`. L3 does not create a separate aggregate beyond what H02 defines.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Applicable exact H02 events are `NotificationScheduled`, `NotificationCancelled`, `NotificationExpired`, `DeliveryStarted`, `DeliveryCompleted`, and `NotificationDelivered`, only when the corresponding source-defined transition occurs. H02 is their canonical publisher.

## 14.2 Business Contracts and Cross-Domain Hand-offs

This H-owned Simple scenario starts after notification authorization. No new cross-domain business hand-off is required within the scheduling lifecycle.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The frozen trigger establishes an authorized Notification before scheduling. Locked H02 does not define the canonical business actor for schedule creation/change/cancellation.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or override rule is defined for ordinary scheduling.

## 15.3 Business Audit and Evidence Obligations

Preserve source-defined scheduling and Notification lifecycle history. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An authorized Notification is governed by an H02 schedule.
- Scheduling preserves Notification identity.
- Delivery-window, recurrence, cancellation and expiry rules are applied.
- Each affected instance executes or reaches `Cancelled`/`Expired` with traceable H02 evidence.

## 16.2 Postconditions

- H02 retains ownership of the Notification/scheduling lifecycle.
- Executed, cancelled, and expired outcomes remain historically traceable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H02 has no canonical Actors section and does not assign schedule creation/change/cancellation to a named business role.
- H02 does not define separate canonical events for every scheduling-lifecycle transition.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Define H02 scheduling actors/events upstream only if explicit business authority or cross-domain event consumption is required.
