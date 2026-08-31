# Failed Notification Delivery to Recovery or Terminal Failure

## 1. Specification Metadata
**Specification ID:** L4-H-015  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Notification  
**Realizes Scenario:** L3-H-015  
**Primary Business Owner:** Notification  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-015; L2-H02 — Notification Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall recover an H02 delivery failure under the same Notification identity or record a controlled terminal failure without repeating or changing the originating business transaction.

## 3. Required Business Context
Canonical Notification; failed Delivery Attempt; immutable attempt history; failure classification; causation/correlation; H02 recovery/fallback policy.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-015-BH-001 | When a Delivery Attempt fails, the system shall associate the failure with the existing Notification and immutable attempt history and classify it under H02 policy. | L3-H-015 §§7–9 |
| L4-H-015-BH-002 | Where permitted, controlled re-execution or provider/channel fallback shall occur under the same Notification identity, with each later Delivery Attempt preserved as distinct immutable evidence. | L3-H-015 §§9,10,12 |
| L4-H-015-BH-003 | When ordinary recovery is exhausted or not permitted, the Notification shall remain in the source-defined controlled recovery position or terminal `Failed` outcome; no invented recovery state is introduced. | L3-H-015 §§9–13 |
| L4-H-015-BH-004 | Recovery shall never repeat, reverse or change the originating business transaction and shall not create a new Notification identity solely because recovery is required. | L3-H-015 §§11–12,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-015-BH-005 | H02 shall publish the applicable `DeliveryFailed`, `NotificationRetryStarted`, `NotificationRetryCompleted`, `DeliveryCompleted` or `NotificationDelivered` event according to the actual recovery path; no provider-fallback or terminal-recovery event is invented. | L3-H-015 §14 |

## 6. Failure / Rejection / Continuation
If human operational follow-up is required, the unresolved condition and evidence remain governed without inventing a business actor or lifecycle state.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-015-BH-006 | The system shall preserve Notification identity, every Delivery Attempt, failure classification, recovery/fallback lineage, correlation and final H02 outcome. | L3-H-015 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-015-BH-007 | Completion shall require policy-selected recovery or terminal failure, immutable prior attempts, no duplicate Notification identity and no repetition of the source business transaction. | L3-H-015 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-015 — Failed Notification Delivery to Recovery or Terminal Failure  
**Primary verification source:** L2-H02 — Notification Workflows.

## 10. Open Issues / Deferred Design
Exceptional operational reprocessing actor and dedicated fallback/terminal-recovery events are not defined upstream; technical retry mechanics remain downstream.