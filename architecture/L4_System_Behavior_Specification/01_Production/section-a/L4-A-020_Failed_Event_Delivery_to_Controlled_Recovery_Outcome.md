# Failed Event Delivery to Controlled Recovery Outcome

## 1. Specification Metadata

**Specification ID:** L4-A-020  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Event Infrastructure  
**Realizes Scenario:** L3-A-020  
**Primary Business Owner:** Event Infrastructure  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-A-020_Failed_Event_Delivery_to_Controlled_Recovery_Outcome.md`; `L2-A06_Event_Infrastructure_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A07_Audit_Infrastructure_Workflows.md`; `L3-A-019`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern the business-visible resolution of one failed consumer delivery so the immutable published event, Tenant boundary, correlation/causation and other consumer deliveries remain unchanged, while the affected delivery reaches either successful acknowledged completion or source-defined `Dead-Lettered` isolation. Technical retry schedules, queue/broker behavior and transport mechanics are outside L4.

## 3. Required Business Context

- One immutable event published under the A019 boundary.
- One authorized consumer with an independent failed delivery.
- Event identity/version, Tenant, correlation, causation and consumer identity.
- Authoritative consumer acknowledgement or failure outcome.
- Applicable recovery and access policies.
- Source-defined delivery recovery lifecycle, acknowledged completion condition and `Dead-Lettered` isolation outcome.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-020-BH-001 | When one consumer delivery fails, the system shall preserve the original published event and identify the failure as belonging only to that consumer delivery. | L3-A-020 §§8–12 |
| L4-A-020-BH-002 | The system shall preserve Event identity, version, Tenant, correlation, causation, consumer and failure context throughout controlled recovery. | L3-A-020 §§7, 9, 12–13 |
| L4-A-020-BH-003 | The system shall not alter the lifecycle outcome of any other consumer delivery because one consumer delivery failed. | L3-A-020 §§3–5, 9–13 |
| L4-A-020-BH-004 | Before further recovery progression, the system shall evaluate the source-defined recovery policy and any required authorization for privileged recovery action. | L3-A-020 §§9, 11–12, 15 |
| L4-A-020-BH-005 | When governed redelivery is permitted, the system shall keep the affected delivery within the source-defined recovery lifecycle and shall require a new authoritative consumer acknowledgement or failure result without specifying the delivery mechanism. | L3-A-020 §§9–13 |
| L4-A-020-BH-006 | The system shall declare the affected delivery successfully complete only after the consumer supplies the source-required acknowledgement. | L3-A-020 §§9, 12–13 |
| L4-A-020-BH-007 | When the source-defined recovery policy reaches its exhaustion/prohibition condition, the system shall isolate only the affected delivery as `Dead-Lettered` without modifying the original event. | L3-A-020 §§3, 9–13 |
| L4-A-020-BH-008 | The system shall prevent unauthorized inspection or management of a `Dead-Lettered` delivery. | L3-A-020 §§11–12, 15 |
| L4-A-020-BH-009 | A `Dead-Lettered` outcome shall remain eligible only for separately governed investigation or replay and shall not itself execute A021 replay. | L3-A-020 §§4, 10, 16 |

## 5. Cross-Domain & Event Behavior

The consuming domain retains ownership of its processing outcome; Event Infrastructure owns delivery state and isolation. L2-A06 defines no canonical recovery lifecycle event names or sole publishers for failed delivery, retry success or dead-letter isolation. No `DeliveryFailed`, `RetrySucceeded` or `DeadLettered` event is invented.

## 6. Failure / Rejection / Continuation

Incomplete or cross-Tenant failure context, unauthorized recovery, ambiguous consumer outcome, unsuccessful governed recovery, unauthorized dead-letter access or inability to preserve recovery history shall prevent successful closure. The system shall retain the last source-defined governed delivery position and shall not fabricate acknowledgement.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-020-BH-010 | The system shall preserve attributable evidence of Event/delivery identity, Tenant, consumer, failure classification, applicable policy, authorized actor, governed recovery progression, consumer outcomes, final acknowledged completion or `Dead-Lettered` isolation, correlation and causation. | L3-A-020 §§15–16 |

## 8. Observable Completion Conditions

Completion requires the affected delivery to have either obtained authoritative acknowledgement and reached successful completion, or reached source-defined `Dead-Lettered` isolation after governed recovery is no longer permitted, with the original event and all other consumer deliveries unchanged and complete traceability preserved.

## 9. Traceability & Provenance

**Parent:** L3-A-020 — Failed Event Delivery to Controlled Recovery Outcome  
**Primary verification source:** L2-A06 — Event Infrastructure Workflows  
**Additional verification:** L2-A04 — Authorization & Policy; L2-A07 — Audit Infrastructure; L3-A-019 publication/delivery boundary

## 10. Open Issues / Deferred Design

The parent’s historical blocker arose because its frozen wording uses retry and dead-letter concepts while L3 prohibits technical recovery detail. At L4, this Draft specifies only observable governed delivery states/outcomes and defers retry schedules, counts, algorithms, queue/broker behavior and transport recovery to later design. Upstream still leaves canonical recovery commands, transition authority, exact recovery event names/publishers and role-to-recovery authority mapping undefined; none is invented here.
