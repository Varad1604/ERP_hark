# Guardian Relationship Reinstatement

**Scenario ID:** L3-C-060  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Guardian  
**Participating Domains:** A, C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Guardian authoritative L2 workflow architecture and materially applicable upstream L1/L2 sources  
**Referenced By:** Related canonical L3 scenarios as identified below

## 1. Scenario Metadata

This document preserves the frozen Section C scenario identity, Primary Owner, participating domains, Scenario Type and Priority. It applies the authoritative upstream L2 architecture and does not redefine ownership, rules, lifecycle, events, publishers or authority.

## 2. Scenario Purpose

Restore a previously suspended Guardian Relationship to an active governed position when the institution determines that the conditions requiring suspension no longer apply.

## 3. Business Outcome

The suspended Guardian Relationship is authoritatively reinstated to its active business position, its prior history remains preserved, and the reinstatement is available to participating domains as a canonical Guardian business fact.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An existing Guardian Relationship is in Suspended status and an authorized reinstatement decision is initiated.

### 4.2 End Boundary
The Guardian Relationship is recorded as Active again under applicable policy, with the reinstatement decision and prior suspension history preserved.

### 4.3 In Scope
- Identify the suspended Guardian Relationship.
- Establish that the reinstatement condition has been satisfied.
- Validate required institutional authority.
- Reinstate the relationship to its active governed state.
- Preserve the suspension and reinstatement history.
- Publish the canonical reinstatement event.
- Allow downstream domains to consume the authoritative active relationship state.

### 4.4 Out of Scope
- Creating a new Guardian Relationship.
- Permanently terminating a relationship.
- Person identity reinstatement.
- Authentication credential reinstatement.
- Student lifecycle changes.
- Downstream operational consequences.

### 4.5 Related Scenarios
C059 — Guardian Relationship Suspension; C061 — Guardian Relationship Termination.

## 5. Ownership and Participating Domains

Guardian is the canonical owner of the business outcome described by this scenario. Participating domains contribute or consume business information only within their own authoritative boundaries; participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

- Guardian Workflows owns the Guardian Relationship lifecycle.
- Person & Identity remains owner of Person identity.
- Student Lifecycle remains owner of Student lifecycle.
- Authentication & Authorization remains owner of access credentials.
- Downstream domains consume the resulting Guardian relationship state and determine their own business consequences.

Only source-supported business roles and authority are used. No technical access control/IAM model is introduced.

## 7. Preconditions and Entry Criteria

- Guardian Relationship exists.
- Current state is Suspended.
- Reinstatement is permitted under applicable Guardian policy.
- Required authority/evidence is available.

## 8. Trigger

An authorized decision determines that a suspended Guardian Relationship may be reinstated.

## 9. Main Business Flow

1. Identify the suspended Guardian Relationship.
2. Validate the reinstatement condition and applicable policy.
3. Validate required authority.
4. Transition the relationship from Suspended to Active.
5. Preserve the suspension and reinstatement history.
6. Publish `GuardianRelationshipReinstated`.
7. Downstream consumers use the resulting Active relationship state within their own business processes.

The flow ends when the singular institutional outcome in Section 3 is achieved.

## 10. Alternate Business Flows

- If the reinstatement condition is not satisfied, the relationship remains Suspended.
- Reinstatement does not erase the earlier suspension.
- A relationship requiring permanent termination is not silently reinstated.
- Downstream domains independently determine consequences of the restored Guardian status.

## 11. Failure, Interruption and Business Recovery

An invalid reinstatement leaves the relationship Suspended and preserves all prior history. No technical retry or rollback mechanism is specified. Permanent termination remains a distinct business outcome.

Recovery is expressed only at business level. Technical retries, queues, rollback, infrastructure failures and implementation mechanics are outside L3.

## 12. Business Rules and Decision Constraints

- Guardian Relationship lifecycle remains exclusively Guardian-owned.
- Suspension and reinstatement are distinct lifecycle decisions.
- Historical Guardian relationship information is preserved.
- A reinstated relationship must satisfy applicable institutional Guardian policy.
- Decisions are auditable.

No local replacement rule is introduced where upstream policy is institution-specific or configurable.

## 13. State and Lifecycle Impact

Guardian Relationship: Suspended → Active. No new state is introduced.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

`GuardianRelationshipReinstated` — Guardian Relationship Aggregate. Downstream domains consume the business fact; they do not publish Guardian lifecycle state.

Events remain completed business facts published by their canonical upstream publisher.

## 15. Business Authorization, Approval and Audit Obligations

Reinstatement requires the institutionally defined authority and evidence. Where separation of duties applies, it must be honored. Exact approval roles are not invented.

## 16. Completion Criteria and Postconditions

- Reinstatement eligibility established.
- Required authority satisfied.
- Relationship becomes Active.
- Prior suspension history retained.
- Canonical reinstatement event published.
- Downstream consumers can rely on the restored Guardian status.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C060-01:** Institution-specific reinstatement conditions and approval authority are not universally defined in the accessible L2 excerpt; L3 does not invent them.

**Classification:** PASS WITH ISSUES — lifecycle ownership is clear; institution-specific reinstatement policy remains upstream.

## Production Verification Record

- [x] Frozen scenario identity preserved.
- [x] Primary Owner / participating domains / Type / Priority preserved.
- [x] Authoritative L2 ownership preserved.
- [x] Dependencies kept within canonical business boundaries.
- [x] No invented actors, rules, states, events or publishers.
- [x] No implementation-level architecture.
- [x] All 17 mandatory sections present.
- [x] Material source gaps recorded in Section 17.
- [x] Status intentionally remains Draft under Phase 5 authoring governance.
