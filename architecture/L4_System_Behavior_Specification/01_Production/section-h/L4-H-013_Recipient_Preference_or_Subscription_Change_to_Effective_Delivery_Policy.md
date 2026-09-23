# Recipient Preference or Subscription Change to Effective Delivery Policy

## 1. Specification Metadata
**Specification ID:** L4-H-013  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Notification  
**Realizes Scenario:** L3-H-013  
**Primary Business Owner:** Notification  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** L3-H-013; L2-H02 — Notification Workflows; applicable Section A authorization/audit architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall make a permitted recipient preference or subscription change effective for future eligible Notifications while preserving mandatory communication obligations and historical Notification outcomes.

## 3. Required Business Context
Valid recipient; category/channel scope; recipient-control eligibility; enterprise/tenant/institution policy; mandatory/optional classification; current effective preference/subscription history.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-013-BH-001 | On a recipient request to change an optional preference or subscription, the system shall resolve recipient and scope, determine recipient-control eligibility, and apply enterprise-policy precedence and deterministic preference inheritance. | L3-H-013 §§7–9,12 |
| L4-H-013-BH-002 | A permitted change shall become the current effective H02 preference/subscription position for future eligible Notifications while prior Notification outcomes remain unchanged. | L3-H-013 §§9,12–13,16 |
| L4-H-013-BH-003 | Mandatory Notifications shall not be suppressible by unsubscribe or optional preference, and governed emergency policy may bypass preferences/subscriptions only where H02 explicitly permits. | L3-H-013 §§10–12 |
| L4-H-013-BH-004 | If recipient context cannot be resolved or the requested change conflicts with higher-authority policy, the current effective position shall remain unchanged. | L3-H-013 §11 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-013-BH-005 | H02 shall not publish an invented preference-change or subscription-change event because the Locked parent defines none; Section A identity/authorization context remains source-owned. | L3-H-013 §14 |

## 6. Failure / Rejection / Continuation
Policy-conflicting or mandatory-category changes are not applied; the higher-authority policy position remains effective and historical Notification outcomes are unchanged.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-013-BH-006 | The system shall preserve recipient/scope, requested change, policy decision, prior and resulting effective position, and change history. | L3-H-013 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-013-BH-007 | Completion shall require resolved recipient/scope, evaluated policy precedence and eligibility, the permitted effective position for future eligible Notifications, and unchanged mandatory obligations/history. | L3-H-013 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-013 — Recipient Preference or Subscription Change to Effective Delivery Policy  
**Primary verification source:** L2-H02 — Notification Workflows.

## 10. Open Issues / Deferred Design
H02 defines no canonical Preference/Subscription state machine or change event; L4 preserves that boundary.

## Canonical Validation Note

This specification passed the scoped R2B hard-fail validation against the Locked L4 Governance and Architecture Standard and L4 Validation Standard and is `Architecture Approved` under `L4_R2B_Notification_Authority_Architecture_Approval_Decision_2026-09-23.md`. Existing non-blocking upstream gaps remain bounded exactly as recorded; this approval does not invent universal actors, lifecycle states, events, publishers, provider mechanics, technical retry behavior, or implementation design.
