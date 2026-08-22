# Recipient Preference or Subscription Change to Effective Delivery Policy

# 1. Scenario Metadata

**Scenario ID:** L3-H-013  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Notification  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H02 — Notification Workflows; applicable Section A identity, authorization, and audit architecture  
**Referenced By:** Governed notification delivery, preference evaluation, subscription enforcement, and notification audit activities

# 2. Scenario Purpose

Govern a recipient's permitted notification preference or subscription change so that future eligible notifications follow the effective H02 delivery policy without altering mandatory communication obligations or historical notification outcomes.

# 3. Business Outcome

The effective policy governs future eligible notifications while mandatory communications and historical outcomes remain unaffected

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a recipient changes an optional preference or subscription.

## 4.2 End Boundary

The scenario ends when H02 records the effective permitted preference or subscription position and applies it to future eligible notifications while mandatory communications and historical outcomes remain unchanged.

## 4.3 In Scope

- Resolving the recipient context for the permitted preference or subscription change.
- Applying H02 preference inheritance and enterprise-policy precedence.
- Applying subscription rules only where subscription is permitted.
- Making the resulting preference or subscription position effective for future eligible notifications.
- Preserving historical notification outcomes and auditable change history.

## 4.4 Out of Scope

- Changing the originating business transaction or business outcome.
- Allowing a recipient to suppress mandatory notifications.
- Changing historical notifications because a future preference changes.
- Defining technical access-control, delivery-provider, or persistence mechanisms.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains canonical identity, authorization, and audit ownership supporting an authorized recipient-context change. |
| Notification / Section H | Owns Notification Preference, Notification Subscription, their effective delivery-policy consequence, and notification-specific change history. |

# 6. Business Actors, Roles and Responsibilities

The frozen trigger identifies the **recipient** as the party changing an optional preference or subscription. Locked H02 defines recipient-specific and user-level preferences and subscription participation, but its canonical Actors section is absent. No narrower recipient role is invented.

# 7. Preconditions and Entry Criteria

- The recipient context is valid and belongs to the governed tenant/institutional boundary.
- The requested preference or subscription category is eligible for recipient control.
- Enterprise and institutional policy applicable to the change is available.
- Mandatory notification obligations remain distinguishable from optional preferences or subscriptions.

# 8. Trigger

A recipient changes an optional preference or subscription

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the recipient and the preference or subscription scope being changed. | Notification / Section H using Section A context | The change is associated with the correct governed recipient context. |
| 2 | Determine whether the requested category or channel is eligible for recipient control. | Notification / Section H | Mandatory communication obligations remain protected. |
| 3 | Apply enterprise-policy precedence, deterministic preference inheritance, and applicable subscription constraints. | Notification / Section H | The permitted effective delivery policy is determined. |
| 4 | Record the permitted preference or subscription change as effective for future eligible notifications. | Notification / Section H | Future eligible delivery uses the current governed position. |
| 5 | Preserve prior notification outcomes and auditable preference/subscription history. | Notification / Section H with Platform Audit | Historical communication evidence remains unchanged. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Optional preference accepted | The change is permitted by enterprise and institutional policy. | Make the permitted preference effective. | Future eligible notifications use the new preference. | Notification |
| Subscription changed | The notification category permits subscription control. | Apply the subscription change immediately. | Future eligible notifications reflect the new subscription position. | Notification |
| Mandatory communication | The recipient attempts to unsubscribe from a mandatory notification. | Preserve the mandatory delivery obligation. | Mandatory communication remains unaffected. | Notification |
| Emergency communication | A future notification is governed by emergency policy. | Emergency policy may override configured preferences or subscriptions. | Emergency communication remains governed by emergency policy. | Notification |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Preference change conflicts with enterprise policy | The requested recipient choice cannot govern delivery. | Keep the authoritative higher-precedence policy effective and reject the conflicting change. | Notification |
| Subscription change targets a mandatory category | Recipient control is not permitted for that category. | Preserve mandatory notification eligibility and record the unsuccessful change outcome. | Notification |
| Recipient context cannot be authoritatively resolved | The change cannot safely be applied. | Do not change the current effective preference/subscription position until the canonical recipient context is resolved. | Platform Foundation / Notification |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Enterprise policies take precedence over user preferences. | Notification | Recipient choice cannot override higher-order policy. | L2-H02 §20.4 |
| Preference evaluation occurs before delivery execution and inheritance is deterministic. | Notification | Future eligible notifications use the effective governed position. | L2-H02 §20.4 |
| Preference changes do not modify historical notifications. | Notification | Historical outcomes remain unchanged. | L2-H02 §20.4 |
| Mandatory notifications cannot be unsubscribed. | Notification | Subscription control is limited to permitted categories. | L2-H02 §22.3 |
| Subscription changes are effective immediately and subscription history is auditable. | Notification | Future eligibility changes without rewriting history. | L2-H02 §22.3 |
| Emergency notifications may bypass preferences and ignore subscription settings. | Notification | Recipient settings do not suppress governed emergency communication. | L2-H02 §§20.4, 22.3 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Notification Preference | Existing recipient-specific governed preference position | Permitted preference is changed under policy precedence. | Updated effective preference for future eligible notifications. | Notification | L2-H02 §20 |
| Notification Subscription | Existing category participation position | Permitted subscription is changed. | Updated effective subscription position for future eligible notifications. | Notification | L2-H02 §22 |
| Historical Notification | Any completed historical lifecycle state | No lifecycle change. | Unchanged. | Notification | L2-H02 §§12, 20.4 |

H02 does not define a canonical state machine for Preference or Subscription. L3 therefore does not invent preference/subscription lifecycle states.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Locked H02's standard event catalogue covers Notification lifecycle events but does not define a canonical preference-change or subscription-change event. This scenario therefore does not invent one.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A supplies canonical recipient identity/authorization context where required. H02 applies the resulting effective preference/subscription position only within Notification delivery governance; no identity or originating-business ownership transfers to H02.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The frozen trigger establishes that the recipient initiates the optional preference or subscription change. H02 policy determines whether that requested change is eligible to become effective. No additional approver is defined upstream.

## 15.2 Separation of Duties, Overrides and Exceptions

Enterprise and institutional policy may override recipient preference, and emergency policy may override preferences/subscriptions where H02 explicitly permits. No additional separation-of-duty rule is defined.

## 15.3 Business Audit and Evidence Obligations

The effective change, prior position where available, recipient context, scope/category, effective time, and resulting governed position must remain auditable. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The recipient and requested optional scope are resolved.
- Policy precedence and subscription eligibility are evaluated.
- The permitted preference or subscription position becomes effective for future eligible notifications.
- Mandatory communication obligations remain unaffected.
- Historical notification outcomes remain unchanged.
- The change remains auditable.

## 16.2 Postconditions

- H02 holds the current effective notification preference/subscription position for its owned delivery governance.
- Future eligible notifications evaluate that position under enterprise policy.
- Mandatory and emergency notification governance remains intact.
- Historical notifications remain immutable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-H02's canonical crosswalk records the required Actors section as absent. | The frozen trigger supports a recipient-initiated change, but H02 does not provide a canonical actor catalogue or narrower role model for preference/subscription administration. | Define H02 business actors upstream if finer-grained authority is required. |
| H02 defines preference and subscription rules but no canonical business event for a completed preference or subscription change. | L3 cannot publish or assign an invented change event. | Add an event only if upstream architecture requires cross-domain consumption of these changes. |

**Architecture assessment:** `PASS WITH ISSUES`

The ownership, rules, effective-policy consequence, historical non-retroactivity, and mandatory-notification protection are explicit. The actor-catalogue and event gaps do not block the frozen business outcome.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
