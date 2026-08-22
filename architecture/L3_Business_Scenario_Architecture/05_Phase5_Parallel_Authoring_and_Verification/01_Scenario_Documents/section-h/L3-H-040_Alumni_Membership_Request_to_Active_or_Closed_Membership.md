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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A02 — Person & Identity Workflows; L2-H06 — Alumni Workflows  
**Referenced By:** Alumni participation, chapter, networking, mentorship, event, recognition, verification, and contribution scenarios

# 2. Scenario Purpose

Govern Alumni engagement membership while retaining Section A ownership of the canonical Person and Identity. Locked H06 supports activation, suspension, reactivation, and archival, but not the complete frozen membership-request outcome set.

# 3. Business Outcome

Membership is approved, rejected, renewed, suspended, expired, or closed with preserved history

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An eligible alumnus requests or renews membership.

## 4.2 End Boundary

The frozen end boundary requires all membership outcomes in Section 3. H06 currently defines only activation, suspension, reactivation, inactivity, opt-out, and archival positions; material gaps are recorded in Section 17.

## 4.3 In Scope

- H06-supported Alumni membership activation and participation status changes.
- Preservation of membership history through suspension/reactivation and archival.
- Use of canonical Person and Identity references without H06 modifying Section A records.

## 4.4 Out of Scope

- Missing request, rejection, renewal, expiry, and closed membership states or transitions.
- Changes to Person or Identity lifecycles by H06.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Owns canonical Person, Identity, User Account, and their lifecycle facts referenced by Alumni. |
| Alumni / Section H | Owns Alumni engagement membership and its H06-defined lifecycle positions and history. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Alumni | Accepts invitation and participates in source-supported membership actions. | Initiates / receives outcome |
| Alumni Administrator | Administers activation, suspension, reactivation, and archival where defined by H06. | Performs / decides source-supported actions |

# 7. Preconditions and Entry Criteria

- An H06 Alumni Profile exists from the canonical Alumni transition path.
- Canonical Person and Identity references remain valid under Section A ownership.
- The membership is in an H06-defined state eligible for the requested supported action.

# 8. Trigger

An eligible alumnus requests or renews membership

# 9. Main Business Flow

1. Resolve the existing Alumni Profile and canonical identity references.
2. Apply only H06-defined membership eligibility and one-active-membership rules.
3. For the supported activation path, move membership through `Eligible` to `Registered` and `Active` and publish `AlumniMembershipActivated`.
4. Where a source-supported suspension applies, move `Active` to `Suspended` and publish `AlumniMembershipSuspended` while preserving history.
5. Where source-supported reactivation applies, restore `Suspended` or `Inactive` membership to `Active` and publish `AlumniMembershipReactivated`.
6. Where H06 archival applies, archive the Alumni relationship and publish `AlumniArchived`.
7. Stop before representing unsupported frozen request, rejection, renewal, expiry, or closed outcomes.

# 10. Alternate Business Flows

H06 permits administrator activation or automatic activation policy. Where membership payment is institutionally required, Finance completes its own payment outcome before H06 activation; Finance is not added as a frozen participating domain in this scenario.

# 11. Failure, Interruption and Business Recovery

An already active membership produces no new lifecycle transition. Invalid membership state or identity/reference conditions prevent the requested supported action. Unsupported frozen outcomes require upstream architecture definition.

# 12. Business Rules and Decision Constraints

- An alumnus shall have only one active membership at any point in time.
- Only Active memberships may participate in Alumni services unless institutional policy specifies otherwise.
- Suspended memberships retain historical engagement records.
- Reactivation restores participation rights without recreating the Alumni aggregate.
- Section A retains ownership of Person and Identity; H06 references but does not modify them.

# 13. State and Lifecycle Impact

H06 defines `Eligible → Registered → Active`, `Active → Suspended`, `Suspended → Active`, `Inactive → Active`, and `Active → Archived`. No additional membership state is introduced here.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H06 defines `AlumniMembershipActivated`, `AlumniMembershipSuspended`, `AlumniMembershipReactivated`, and `AlumniArchived` for the supported membership path.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H06 uses canonical identity references owned by A02. Identity participation does not transfer Person, Identity, or User Account ownership to Alumni.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Only the source-supported Alumni and Alumni Administrator responsibilities in H06 are used. H06 does not define the missing approval/rejection/renewal/expiry/closure authorities required by the frozen outcome.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 override may convert H06 `Archived`, `Inactive`, or `Opted Out` into an invented `Closed` or `Expired` state, or manufacture rejection/renewal authority.

## 15.3 Business Audit and Evidence Obligations

Preserve membership identity, prior/current state, activation/suspension/reactivation/archive authority where applicable, canonical Person/Identity references, and exact H06 membership events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

A supported H06 membership transition may complete to its defined state. The full frozen outcome cannot be claimed until all missing Section 17 lifecycle branches are resolved upstream.

## 16.2 Postconditions

Supported membership history remains preserved; Section A identity ownership remains unchanged; unsupported frozen outcomes are not approximated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H06 defines no membership-request lifecycle state or approval workflow corresponding to the frozen request boundary.
- H06 defines no canonical membership **rejection** state, transition, authority, or event.
- H06 defines no canonical **renewal** or **expiry** state, transition, authority, or event despite the frozen trigger/outcome requiring them.
- H06 uses `Archived`, `Inactive`, and `Opted Out` positions but does not define the frozen terminal **Closed membership** state or event.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions beyond the open issues above.
