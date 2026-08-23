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
**Last Updated:** 2026-08-23  
**Depends On:** L2-H02 — Notification Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern a recipient's permitted H02 preference or subscription change so future eligible Notifications use the effective delivery policy while mandatory communications and historical Notification outcomes remain unchanged.

# 3. Business Outcome

The effective policy governs future eligible notifications while mandatory communications and historical outcomes remain unaffected

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A recipient changes an optional preference or subscription

## 4.2 End Boundary

H02 records the permitted effective preference or subscription position for future eligible Notifications while mandatory communications and historical Notification outcomes remain unchanged.

## 4.3 In Scope

- Recipient context for the preference/subscription change.
- Enterprise/tenant policy precedence and deterministic preference inheritance.
- Subscription eligibility and protection of mandatory Notifications.
- Auditable change history.

## 4.4 Out of Scope

- Changing the originating business transaction.
- Suppressing mandatory Notifications through recipient preference.
- Retroactively changing historical Notification outcomes.

# 5. Ownership and Participating Domains

Section A retains canonical identity, authorization, and audit ownership. Notification / Section H owns Notification Preferences, Subscriptions, their delivery-policy consequence, and notification-specific change history.

# 6. Business Actors, Roles and Responsibilities

The frozen trigger identifies the **recipient** as initiator. H02 defines recipient/user preferences and subscriptions but has no canonical Actors section; no narrower administrative role is invented.

# 7. Preconditions and Entry Criteria

- Recipient context is valid.
- Requested category/channel is eligible for recipient control.
- Applicable enterprise/tenant/institutional policy is available.
- Mandatory communications remain distinguishable from optional subscriptions/preferences.

# 8. Trigger

A recipient changes an optional preference or subscription

# 9. Main Business Flow

1. Resolve the recipient and requested preference/subscription scope.
2. Determine whether the requested scope is eligible for recipient control.
3. Apply enterprise-policy precedence and deterministic preference inheritance.
4. Apply H02 subscription restrictions, including protection of mandatory Notifications.
5. Record the permitted effective preference/subscription position for future eligible Notifications.
6. Preserve prior Notification outcomes and the change history.

# 10. Alternate Business Flows

- **Permitted preference:** becomes effective under higher-precedence policy.
- **Permitted subscription change:** becomes effective immediately.
- **Mandatory category:** unsubscribe request does not suppress the Notification obligation.
- **Emergency Notification:** governed emergency policy may bypass preferences/subscriptions.

# 11. Failure, Interruption and Business Recovery

If the requested change conflicts with enterprise/tenant policy or targets a mandatory category, H02 preserves the higher-authority policy position. If recipient context cannot be resolved, the current effective preference/subscription remains unchanged until authoritative context is available.

# 12. Business Rules and Decision Constraints

- Enterprise policies take precedence over user preferences.
- Preference evaluation occurs before delivery and inheritance is deterministic.
- Mandatory notifications cannot be unsubscribed.
- Subscription changes are effective immediately and history remains auditable.
- Emergency notifications may bypass preferences and subscription settings under governed policy.
- Preference changes do not rewrite historical Notification outcomes.

# 13. State and Lifecycle Impact

H02 does not define a canonical Preference or Subscription state machine. The scenario changes the current effective H02 preference/subscription position only; historical Notifications retain their completed lifecycle states.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H02 defines no canonical preference-change or subscription-change business event. No event or publisher is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A provides canonical recipient identity/authorization context where required. H02 applies only the notification preference/subscription consequence; identity ownership does not transfer.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The recipient initiates the optional change; H02 policy determines whether the requested change is permitted.

## 15.2 Separation of Duties, Overrides and Exceptions

Enterprise/tenant policy may override recipient preference, and emergency policy may bypass preferences/subscriptions where H02 explicitly permits. No additional separation-of-duty rule is defined.

## 15.3 Business Audit and Evidence Obligations

Preserve the governed preference/subscription change history and resulting effective position. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Recipient/scope are resolved.
- Policy precedence and subscription eligibility are evaluated.
- The permitted position becomes effective for future eligible Notifications.
- Mandatory communications and historical outcomes remain unchanged.

## 16.2 Postconditions

- H02 holds the current effective Notification preference/subscription position.
- Future eligible Notifications evaluate that position under higher-precedence policy.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H02 has no canonical Actors section beyond the trigger-level recipient concept.
- H02 defines no canonical preference-change or subscription-change event.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Define finer-grained preference/subscription actors/events only if required upstream.
