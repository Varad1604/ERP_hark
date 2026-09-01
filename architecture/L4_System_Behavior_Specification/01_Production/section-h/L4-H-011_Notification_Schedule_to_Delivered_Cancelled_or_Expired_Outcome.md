# Notification Schedule to Delivered, Cancelled, or Expired Outcome

## 1. Specification Metadata
**Specification ID:** L4-H-011  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Notification  
**Realizes Scenario:** L3-H-011  
**Primary Business Owner:** Notification  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-011; L2-H02 — Notification Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern scheduling of an already authorized H02 Notification so each instance executes only within its permitted window or ends in a traceable cancelled or expired outcome without changing Notification identity.

## 3. Required Business Context
Authorized Notification; valid recipient/template/channel context; scheduling mode; delivery window/time context; expiry; recurrence rule where applicable.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-011-BH-001 | When an authorized Notification is scheduled, delayed or made recurring, the system shall validate recipient/template/channel context, scheduling mode, delivery window and expiry before execution. | L3-H-011 §§7–9 |
| L4-H-011-BH-002 | H02 scheduling shall follow `Created → Scheduled → Waiting → Executing → Completed` with source-defined cancellation/expiry branches, while the Notification may move `Scheduled → Queued`, `Scheduled → Cancelled` or `Scheduled → Expired` as applicable. | L3-H-011 §§9–13 |
| L4-H-011-BH-003 | A recurring schedule shall create independent Notification instances under the recurrence rule; scheduling or schedule modification shall not change the identity of an existing Notification instance. | L3-H-011 §§9,10,12 |
| L4-H-011-BH-004 | A permitted cancellation before execution shall result in `Cancelled`; expiry before execution shall result in `Expired` and the expired Notification shall not execute. | L3-H-011 §§9–13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-011-BH-005 | H02 shall publish `NotificationScheduled`, `NotificationCancelled`, `NotificationExpired`, `DeliveryStarted`, `DeliveryCompleted` and `NotificationDelivered` only when the corresponding locked transitions occur. | L3-H-011 §14 |

## 6. Failure / Rejection / Continuation
If scheduling conditions become invalid, H02 shall preserve Notification identity and apply only the defined cancellation, expiry or defer behavior. Delivery failure after execution begins belongs to the separate recovery scenario.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-011-BH-006 | The system shall preserve schedule definition/change history, Notification identity, recurrence-instance identity, execution window, cancellation/expiry facts and resulting lifecycle evidence. | L3-H-011 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-011-BH-007 | Completion shall require every affected Notification instance to execute through its governed delivery path or reach `Cancelled`/`Expired`, with scheduling history preserved. | L3-H-011 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-011 — Notification Schedule to Delivered, Cancelled, or Expired Outcome  
**Primary verification source:** L2-H02 — Notification Workflows.

## 10. Open Issues / Deferred Design
H02 does not define a canonical business actor for schedule creation/change/cancellation or separate events for every scheduling transition; none is invented.