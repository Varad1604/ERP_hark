# Alumni Membership Request to Active or Closed Membership

# 1. Scenario Metadata

**Scenario ID:** L3-H-040  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Alumni  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H06 — Alumni Workflows; applicable locked Section A identity/authorization architecture

# 2. Scenario Purpose

Govern an Alumni membership request or renewal while retaining H06 ownership of membership state and Section A ownership of identity and authorization facts.

# 3. Business Outcome

Membership is approved, rejected, renewed, suspended, expired, or closed with preserved history

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an eligible alumnus requests or renews membership.

## 4.2 End Boundary

The frozen end boundary requires one of the membership outcomes: approved, rejected, renewed, suspended, expired, or closed.

## 4.3 In Scope

- Validate the existing Alumni Profile and relevant identity verification.
- Govern H06 membership activation, suspension, reactivation, and archival where defined.
- Preserve membership history.

## 4.4 Out of Scope

- Person or User Account ownership.
- Authentication or authorization lifecycle changes.
- Student-to-Alumni transition.
- Inventing membership rejection, renewal, expiry, or closure states not defined by H06.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Section A | Owns canonical identity/authentication/authorization facts consumed by H06. |
| Alumni / H06 | Owns Alumni Membership and its defined lifecycle. |

# 6. Business Actors, Roles and Responsibilities

H06 supports the Alumni as requester and the Alumni Administrator / Institutional Administrator for governed membership administration. L3 introduces no new approval role.

# 7. Preconditions and Entry Criteria

- An Alumni Profile exists.
- Membership is in an H06-defined state that permits the requested action.
- Identity verification is complete where H06 requires it.
- The actor is authorized for the H06 action.

# 8. Trigger

An eligible alumnus requests or renews membership

# 9. Main Business Flow

1. H06 receives the membership intent against an existing Alumni Profile.
2. H06 validates membership state and required identity facts.
3. For activation, H06 follows `Eligible → Registered → Active` and publishes `AlumniMembershipActivated`.
4. For suspension, H06 follows `Active → Suspended` and publishes `AlumniMembershipSuspended`.
5. For reactivation, H06 follows `Suspended → Active` and publishes `AlumniMembershipReactivated`.
6. For archival, H06 may move an Active Alumni relationship to `Archived` under its defined lifecycle and publish `AlumniArchived`.
7. H06 preserves membership history.

The flow cannot be extended to the frozen rejection, renewal, expiry, and closed outcomes because H06 does not define those membership transitions.

# 10. Alternate Business Flows

- Suspended membership may be reactivated through the H06-defined reactivation path.
- Inactive membership may be reactivated where the H06 transition matrix permits.
- Archival follows the H06-defined terminal archival path; it is not renamed to `Closed`.

# 11. Failure, Interruption and Business Recovery

A request that does not correspond to an H06-defined transition remains unresolved at the last valid H06 state. L3 does not map undefined renewal, expiry, rejection, or closure semantics onto existing states.

# 12. Business Rules and Decision Constraints

- H06 owns Alumni Membership.
- Identity remains owned by Section A.
- Membership activation requires an existing Alumni Profile and H06 eligibility.
- H06 defines membership states `Eligible`, `Invited`, `Registered`, `Active`, `Suspended`, and `Inactive`, and separately defines `Archived` in its transition matrix.
- Only H06-defined transitions may be applied.

# 13. State and Lifecycle Impact

Authoritative H06 transitions include `Eligible → Registered → Active`, `Active → Suspended`, `Suspended → Active`, `Active → Archived`, and `Inactive → Active`. No authoritative H06 transitions define `Rejected`, `Renewed`, `Expired`, or `Closed` membership outcomes.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H06 defines `AlumniMembershipActivated`, `AlumniMembershipSuspended`, `AlumniMembershipReactivated`, and `AlumniArchived`. No canonical H06 membership rejection, renewal, expiry, or closed event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H06 consumes identity facts from Section A. That participation does not transfer identity ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Alumni may initiate supported membership actions. H06 administrative roles govern membership administration as defined upstream.

## 15.2 Separation of Duties, Overrides and Exceptions

No local override may manufacture a missing membership state or transition.

## 15.3 Business Audit and Evidence Obligations

Preserve requester/administrator attribution, prior and resulting H06 membership state, applicable reason, and membership history for supported transitions.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen scenario is complete only if the requested membership lifecycle can reach all permitted frozen outcomes under authoritative H06 rules.

## 16.2 Postconditions

Supported H06 activation/suspension/reactivation/archival outcomes remain valid, but the complete frozen outcome set cannot be represented.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

H06 does not define membership `Rejected`, `Renewed`, `Expired`, or `Closed` states/transitions/events even though all are frozen outcomes for H040. `Archived` cannot be silently renamed to `Closed`.

**Architecture impact:** the frozen lifecycle and end boundary are materially incomplete upstream.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream architecture must define the missing membership outcomes or formally amend the frozen scenario semantics.
