# Guardian Relationship Termination

**Scenario ID:** L3-C-061  
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

Conclude an existing Guardian Relationship when an institutionally governed termination decision requires the relationship to cease permanently as an active institutional relationship while preserving its historical record.

## 3. Business Outcome

The Guardian Relationship is authoritatively terminated, no longer carries active Guardian responsibilities, and its complete historical relationship record remains preserved for institutional continuity and audit.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An existing Guardian Relationship is eligible for permanent termination and an authorized termination decision is initiated.

### 4.2 End Boundary
The relationship reaches its canonical terminated/inactive business outcome and its historical record remains preserved.

### 4.3 In Scope
- Identify the Guardian Relationship.
- Validate the termination condition and applicable policy.
- Validate required authority.
- Record the termination.
- Preserve the complete relationship history.
- Publish the canonical termination event.
- Allow participating domains to determine their own consequences from the authoritative terminated state.

### 4.4 Out of Scope
- Temporary suspension.
- Reinstatement of a suspended relationship.
- Person identity deletion/termination.
- Student lifecycle termination.
- Deleting historical Guardian relationship evidence.
- Finance, Transport or Communication operational closure mechanics.

### 4.5 Related Scenarios
C059 — Guardian Relationship Suspension; C060 — Guardian Relationship Reinstatement.

## 5. Ownership and Participating Domains

Guardian is the canonical owner of the business outcome described by this scenario. Participating domains contribute or consume business information only within their own authoritative boundaries; participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

- Guardian Workflows owns Guardian Relationship termination.
- Person & Identity owns Person identity.
- Student Lifecycle owns Student lifecycle.
- Downstream Finance, Communication, Transport, Health and other domains consume the Guardian relationship outcome and own their own operational/business consequences.

Only source-supported business roles and authority are used. No technical access control/IAM model is introduced.

## 7. Preconditions and Entry Criteria

- Guardian Relationship exists.
- Termination is permitted by applicable Guardian policy.
- Required authority and evidence exist.
- The termination is distinguishable from temporary suspension.

## 8. Trigger

An authorized institutional decision permanently concludes the Guardian Relationship.

## 9. Main Business Flow

1. Identify the authoritative Guardian Relationship.
2. Validate the termination condition against applicable policy.
3. Validate required authority and evidence.
4. Record the relationship's terminal/inactive outcome.
5. Preserve all prior Guardian relationship history.
6. Publish `GuardianRelationshipTerminated`.
7. Downstream domains consume the termination fact and apply their own domain-specific consequences.

The flow ends when the singular institutional outcome in Section 3 is achieved.

## 10. Alternate Business Flows

- If termination authority or conditions are not satisfied, the relationship remains in its current state.
- Termination does not delete historical relationship information.
- A suspended relationship may be terminated through this distinct scenario when separately authorized.
- Downstream consequences do not change the canonical Guardian lifecycle owner.

## 11. Failure, Interruption and Business Recovery

An invalid termination is not applied. Existing relationship state and history remain authoritative. Domain-specific financial, communication, transport or operational consequences remain with those domains.

Recovery is expressed only at business level. Technical retries, queues, rollback, infrastructure failures and implementation mechanics are outside L3.

## 12. Business Rules and Decision Constraints

- Guardian Relationship lifecycle is exclusively Guardian-owned.
- Historical Guardian Relationships remain immutable.
- Termination is distinct from suspension and reinstatement.
- Termination decisions require appropriate authority and auditability.
- Downstream consumers must not redefine the Guardian lifecycle.

No local replacement rule is introduced where upstream policy is institution-specific or configurable.

## 13. State and Lifecycle Impact

Guardian Relationship transitions from its current active/suspended governed state to its canonical terminated/inactive state. The exact terminal-state terminology is not expanded beyond the locked upstream lifecycle vocabulary.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

`GuardianRelationshipTerminated` — Guardian Relationship Aggregate. Downstream domains consume the completed business fact.

Events remain completed business facts published by their canonical upstream publisher.

## 15. Business Authorization, Approval and Audit Obligations

Termination requires appropriate institutional authority, evidence and audit. Exact authority/separation-of-duty rules remain upstream and are not invented.

## 16. Completion Criteria and Postconditions

- Valid termination decision established.
- Relationship reaches its canonical terminal/inactive outcome.
- Active Guardian responsibilities cease.
- Historical relationship preserved.
- `GuardianRelationshipTerminated` published.
- Downstream domains can apply their own consequences.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C061-01:** The accessible locked L2 material establishes termination as a Guardian lifecycle capability but the exact terminal-state wording/transition detail is not visible in the fetched excerpt. L3 therefore does not invent a state name.

**Classification:** PASS WITH ISSUES — material lifecycle ownership is clear; exact upstream terminal-state vocabulary should be verified before locking.

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
