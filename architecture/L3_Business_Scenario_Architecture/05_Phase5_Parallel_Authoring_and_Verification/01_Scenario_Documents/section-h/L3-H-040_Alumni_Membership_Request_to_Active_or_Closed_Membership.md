# Alumni Membership Request to Active or Closed Membership

# 1. Scenario Metadata

**Scenario ID:** L3-H-040  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Alumni  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H06 — Alumni Workflows; applicable locked Section A identity/authorization architecture; `L2-H06_Alumni_Engagement_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern an Alumni membership request or renewal through H06-owned review, activation, rejection, suspension/reactivation and closure while retaining Section A ownership of identity and authorization facts.

# 3. Business Outcome

Membership is approved, rejected, renewed, suspended, expired, or closed with preserved history

The frozen words `renewed` and `expired` describe business outcomes and do not introduce new H06 lifecycle state names. A renewed membership is represented through a new governed membership request/activation where policy permits; expiry of a current membership is represented by governed closure of that membership record rather than a new `Expired` state.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an eligible alumnus requests or renews membership.

## 4.2 End Boundary

The scenario ends when the membership reaches the applicable H06 outcome: Active, Rejected, Suspended, or Closed, with renewal/expiry meaning represented through the governed membership-record lifecycle and preserved history.

## 4.3 In Scope

- Membership request and review.
- Activation, rejection, suspension/reactivation and closure.
- New governed membership request after a prior Closed record where policy permits.
- Preservation of membership history.

## 4.4 Out of Scope

- Person/User Account ownership.
- Authentication/authorization lifecycle changes.
- Student-to-Alumni transition.
- Inventing `Renewed` or `Expired` lifecycle states.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Section A | Owns canonical identity/authentication/authorization facts consumed by H06. |
| Alumni / H06 | Owns Alumni Membership and its governed lifecycle. |

# 6. Business Actors, Roles and Responsibilities

Alumni initiates a membership request. H06 Alumni / Alumni Administrator roles participate according to the governed membership process. H06 business governance owns membership decisions; Section A supplies identity/authorization capability only.

# 7. Preconditions and Entry Criteria

- An H06 Alumni Profile exists.
- The membership request is permitted for the current engagement context.
- Required identity facts are available by reference.
- The requester and decision participants are authorized for their H06 responsibilities.

# 8. Trigger

An eligible alumnus requests or renews membership

# 9. Main Business Flow

1. Record the Membership request in `Requested`.
2. Validate the Alumni Profile, identity references and applicable membership conditions.
3. Move the request to `Under Review`.
4. If rejected, move to `Rejected` and publish `AlumniMembershipRejected`.
5. If approved for participation, move to `Active` and publish `AlumniMembershipActivated`.
6. Where temporary restriction is required, move `Active → Suspended` and publish `AlumniMembershipSuspended`.
7. Where governed restoration is permitted, move `Suspended → Active`.
8. When the current membership relationship ends, move `Active / Suspended → Closed` and publish `AlumniMembershipClosed`.
9. Where policy permits renewal after a Closed record, create a new governed membership request; preserve the prior Closed record historically.

# 10. Alternate Business Flows

- `Under Review → Rejected`.
- `Active → Suspended → Active`.
- `Active / Suspended → Closed`.
- Renewal after closure creates a new governed membership request rather than rewriting the closed record.

# 11. Failure, Interruption and Business Recovery

Invalid identity/context or failed membership review prevents activation. Suspended membership remains historical and may be restored only through the governed branch. A Closed record is not rewritten by renewal.

# 12. Business Rules and Decision Constraints

- H06 owns Alumni Membership.
- Canonical lifecycle: `Requested → Under Review → Active → Closed`.
- Branches: `Under Review → Rejected`, `Active → Suspended → Active`, and `Active / Suspended → Closed`.
- A Closed membership may be followed by a new governed membership request where policy permits; the prior record remains historical.
- Section A identity/authorization ownership is unchanged.
- `Renewed` and `Expired` are not introduced as additional lifecycle states.

# 13. State and Lifecycle Impact

Canonical Alumni Membership lifecycle:

`Requested → Under Review → Active → Closed`

Branches:

- `Under Review → Rejected`
- `Active → Suspended → Active`
- `Active / Suspended → Closed`

# 14. Business Events and Cross-Domain Collaboration

H06 publishes the applicable membership facts, including:

- `AlumniMembershipActivated`
- `AlumniMembershipRejected`
- `AlumniMembershipSuspended`
- `AlumniMembershipClosed`

Section A identity facts are consumed by reference and are not republished by H06.

# 15. Business Authorization, Approval and Audit Obligations

H06 governs membership review and decision authority. Evidence preserves request, review, decision, suspension/reactivation/closure, prior membership history and any subsequent new membership request. Section A authorization evidence remains source-owned.

# 16. Completion Criteria and Postconditions

Completion requires the membership to reach the applicable authoritative H06 outcome with complete history preserved. Renewal does not overwrite a closed predecessor; expiry is represented through the governed ending/closure of the membership relationship rather than an invented state.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing rejection, suspension and closure lifecycle semantics were resolved by locked `L2-H06_Alumni_Engagement_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26. Frozen renewal/expiry terminology is retained as business outcome language without creating non-canonical lifecycle states.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
