# Guardian Relationship Suspension

**Scenario ID:** L3-C-059  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Guardian  
**Participating Domains:** A, C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-C06 — Guardian Workflows; L2-A02 — Person & Identity; L2-C01 — Student Lifecycle; additional participating-domain L2 sources where applicable  
**Referenced By:** Related canonical Section C scenarios as identified below

## 1. Scenario Metadata

The frozen Section C identity is preserved exactly. Guardian Workflows / Guardian Relationship remains the canonical owner of the Guardian business outcome. Participating domains retain ownership of their own facts and operational outcomes.

## 2. Scenario Purpose

Temporarily suspend an existing Guardian Relationship when an institutionally governed condition requires the relationship to cease carrying active Guardian responsibilities without prematurely terminating its historical relationship.

## 3. Business Outcome

The Guardian Relationship is authoritatively placed in Suspended status, applicable Guardian responsibilities are no longer treated as active, the suspension is auditable, and the relationship remains eligible for a governed reinstatement or termination outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An existing Guardian Relationship is in a suspendable active state and an authorized institutional suspension decision is initiated.

### 4.2 End Boundary
The relationship is recorded as Suspended, the applicable active responsibilities cease under Guardian policy, and the canonical suspension business fact is published.

### 4.3 In Scope
- Identify the authoritative Guardian Relationship.
- Validate the suspension trigger and applicable institutional policy.
- Validate required authority/separation of duties.
- Record the suspension.
- Preserve the complete relationship history and prior active state.
- Publish the canonical suspension event.
- Ensure downstream consumers receive the authoritative suspended relationship state.

### 4.4 Out of Scope
- Permanent termination of the Guardian Relationship.
- Reinstatement after suspension.
- Person identity suspension.
- Authentication credential suspension.
- Student lifecycle suspension.
- Financial transaction reversal or cancellation.
- Downstream operational execution.

### 4.5 Related Scenarios
C052 — Guardian Relationship Modification; C060 — Guardian Relationship Reinstatement; C061 — Guardian Relationship Termination.

## 5. Ownership and Participating Domains

The **Guardian Relationship Aggregate** is the authoritative owner of Guardian relationship state and the business decision described by this scenario.

Participating domains provide or consume authoritative information within their own boundaries. Participation does not transfer ownership to Finance, Transport, Person & Identity, Student Lifecycle, or any other domain.

## 6. Business Actors, Roles and Responsibilities

- Guardian Workflows owns Guardian Relationship suspension.
- Person & Identity owns Person identity.
- Authentication & Authorization owns authentication/security access.
- Student Lifecycle owns Student lifecycle.
- Finance, Transport, Communication and other consumers independently determine consequences within their own domains based on the suspended Guardian relationship fact.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- Guardian Relationship exists.
- Relationship is in a suspendable state.
- A valid institutional reason/condition exists.
- Required authority is available.
- The suspension is distinguishable from permanent termination.

## 8. Trigger

An authorized institutional suspension decision is made for an existing Guardian Relationship.

## 9. Main Business Flow

1. Identify the Guardian Relationship.
2. Validate the suspension condition against applicable Guardian policy.
3. Validate required institutional authority.
4. Record the relationship as Suspended.
5. Stop the relationship from carrying responsibilities that require an active Guardian relationship.
6. Preserve prior relationship history and the suspension decision.
7. Publish `GuardianRelationshipSuspended`.
8. Downstream domains consume the suspended relationship state and determine their own domain-specific consequences.

The flow stops when the singular institutional outcome is achieved; downstream operational execution remains outside the scenario unless explicitly owned by Guardian.

## 10. Alternate Business Flows

- Suspension may later be followed by governed reinstatement.
- Suspension may instead be followed by permanent termination where separately authorized.
- If the suspension condition is invalid or authority is absent, the relationship remains in its prior state.
- A downstream domain may cease using a Guardian for its own operational purpose, but that does not change Guardian ownership or lifecycle state.

## 11. Failure, Interruption and Business Recovery

An invalid suspension is rejected without altering the current relationship. A suspension does not erase the historical relationship or automatically terminate it. Domain-specific downstream consequences remain with downstream owners.

Recovery remains business-level. No technical retries, queues, rollback, infrastructure, API or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints

- BR-022: Guardian lifecycle transitions remain within the Guardian Relationship consistency boundary.
- Guardian relationship status is authoritative within Guardian Workflows.
- Historical Guardian relationships remain immutable.
- Suspension is distinct from termination.
- Suspension decisions are fully auditable.
- Downstream domains consume Guardian status without redefining it.

Only source-supported Guardian rules are applied. No local replacement rule is invented.

## 13. State and Lifecycle Impact

**Guardian Relationship:** Active → Suspended. The relationship remains part of the Guardian Relationship Aggregate and retains its historical continuity. No termination state is implied by suspension.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

`GuardianRelationshipSuspended` — Guardian Relationship Aggregate. Downstream domains consume the suspended relationship fact and determine their own operational consequences.

Canonical Guardian events remain published by the Guardian Relationship Aggregate. Events are business facts, not technical commands.

## 15. Business Authorization, Approval and Audit Obligations

Suspension requires institutionally authorized business authority and complete audit evidence. Where separation of duties is required by policy, the decision must follow that policy. No technical IAM/RBAC suspension is defined here.

## 16. Completion Criteria and Postconditions

- Suspendable relationship identified.
- Valid suspension condition established.
- Required authority satisfied.
- Relationship becomes Suspended.
- Active Guardian responsibilities cease as governed.
- Historical state preserved.
- `GuardianRelationshipSuspended` published.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C059-01:** The locked C06 material establishes suspension as a Guardian relationship lifecycle capability, but institution-specific suspension reasons/categories are not universally defined here. None is invented.

**OI-C059-02:** Downstream consequences of suspension are domain-specific. Finance, Communication, Transport and other domains consume the authoritative Guardian status but retain their own business decisions.

**OI-C059-03:** Reinstatement is a separate canonical scenario and is not absorbed into this scenario.

**Classification:** PASS WITH ISSUES — source-grounded; institution-specific suspension reasons and downstream consequence policies remain upstream.

## Production Verification Record

- [x] Frozen ID/title/owner/domains/type/priority preserved.
- [x] Guardian Relationship ownership preserved.
- [x] Cross-domain ownership boundaries preserved.
- [x] Canonical Guardian lifecycle/event vocabulary used.
- [x] Finance/Transport responsibilities are not absorbed into Guardian.
- [x] No implementation leakage.
- [x] All 17 mandatory sections present.
- [x] Material gaps recorded in Section 17.
- [x] Status intentionally remains Draft under Phase 5 governance.
