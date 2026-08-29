# Student Pickup Authorization Management

**Scenario ID:** L3-C-058  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Guardian  
**Participating Domains:** C, G  
**Scenario Type:** Cross-domain  
**Priority:** P2  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-C06 — Guardian Workflows; L2-A02 — Person & Identity; L2-C01 — Student Lifecycle; additional participating-domain L2 sources where applicable  
**Referenced By:** Related canonical Section C scenarios as identified below

## 1. Scenario Metadata

The frozen Section C identity is preserved exactly. Guardian Workflows / Guardian Relationship remains the canonical owner of the Guardian business outcome. Participating domains retain ownership of their own facts and operational outcomes.

## 2. Scenario Purpose

Govern which Guardian or authorized representative is institutionally permitted to collect a Student, while preserving the distinction between Guardian authorization and Transport/Student-service execution.

## 3. Business Outcome

The authoritative pickup authorization is recorded against the applicable Guardian Relationship, is auditable, and is available to the operational domain responsible for pickup execution.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An authorized request establishes, changes or withdraws pickup authorization for an existing Student/Guardian relationship.

### 4.2 End Boundary
The current pickup authorization is recorded and the canonical Guardian pickup-authorization business fact is published for operational consumption.

### 4.3 In Scope
- Identify the Student and relevant Guardian Relationship.
- Validate that the relationship is eligible for pickup authorization.
- Validate applicable institutional pickup policy.
- Establish, change or withdraw pickup authorization.
- Preserve historical pickup authorization decisions.
- Publish the canonical pickup-authorization business fact.
- Allow Transport/Student Services to execute pickup operations under its own responsibilities.

### 4.4 Out of Scope
- Transport execution.
- Vehicle/route management.
- Student identity creation or correction.
- Person identity management.
- Guardian authentication.
- Physical handover operations.
- Emergency response operations.

### 4.5 Related Scenarios
C052 — Guardian Relationship Modification; C056 — Student Emergency Contact Assignment; C059 — Guardian Relationship Suspension.

## 5. Ownership and Participating Domains

The **Guardian Relationship Aggregate** is the authoritative owner of Guardian relationship state and the business decision described by this scenario.

Participating domains provide or consume authoritative information within their own boundaries. Participation does not transfer ownership to Finance, Transport, Person & Identity, Student Lifecycle, or any other domain.

## 6. Business Actors, Roles and Responsibilities

- Guardian Workflows owns pickup authorization attached to the Guardian Relationship.
- Student Lifecycle owns Student existence/lifecycle.
- Person & Identity owns person identity.
- Transport or the relevant operational domain owns pickup execution.
- Downstream consumers use the Guardian authorization as a business fact.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- Student exists.
- Guardian Relationship exists.
- The relationship is eligible under Guardian policy.
- Required authorization/evidence is available.
- The Student has not reached a state in which pickup authorization is no longer applicable, where such upstream lifecycle constraint exists.

## 8. Trigger

An authorized pickup-authorization request is received.

## 9. Main Business Flow

1. Identify the Student and Guardian Relationship.
2. Validate authoritative Person and Student references.
3. Validate the relationship's eligibility.
4. Evaluate pickup authorization under applicable institutional policy.
5. Record the authorization, change or withdrawal.
6. Preserve prior pickup authorization history.
7. Publish the canonical pickup-authorization business fact.
8. Transport/Student Services consumes the authorization and performs its own operational pickup responsibilities.

The flow stops when the singular institutional outcome is achieved; downstream operational execution remains outside the scenario unless explicitly owned by Guardian.

## 10. Alternate Business Flows

- A non-eligible Guardian is not authorized.
- Withdrawal removes the current authorization without deleting its historical record.
- Conflicting pickup permissions are resolved through governed institutional rules rather than silently overwritten.
- Transport execution problems do not alter the Guardian authorization decision.

## 11. Failure, Interruption and Business Recovery

An invalid authorization is not applied. Existing authoritative authorization remains unchanged. Operational pickup exceptions are handled by the responsible downstream domain and do not cause Guardian to assume operational ownership.

Recovery remains business-level. No technical retries, queues, rollback, infrastructure, API or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints

- Pickup authorization is a Guardian-owned relationship responsibility.
- Historical Guardian relationship decisions remain immutable.
- Only an eligible Guardian relationship may receive the applicable authorization.
- Guardian authorization does not become Transport ownership.
- All authorization decisions are auditable.

Only source-supported Guardian rules are applied. No local replacement rule is invented.

## 13. State and Lifecycle Impact

Pickup authorization is governed as part of Guardian Relationship modification. No separate technical or operational pickup lifecycle is introduced at L3.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

`GuardianPickupAuthorizationUpdated` — Guardian Relationship Aggregate. Transport/Student Services consumes the business fact and owns pickup execution.

Canonical Guardian events remain published by the Guardian Relationship Aggregate. Events are business facts, not technical commands.

## 15. Business Authorization, Approval and Audit Obligations

Pickup authorization requires appropriate institutional authority and audit evidence. The exact approval authority is not invented. No technical access-control design is introduced.

## 16. Completion Criteria and Postconditions

- Student/Guardian relationship identified.
- Eligibility validated.
- Authorization granted/changed/withdrawn.
- Historical authorization preserved.
- Canonical Guardian event published.
- Downstream operational domain can act on the current authorization.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C058-01:** The frozen scenario includes C and G, while the L2-C06 dependency model identifies Transport as the operational consumer. The scenario therefore keeps pickup execution outside Guardian ownership.

**OI-C058-02:** The locked C06 material does not define a universal taxonomy for pickup authorization evidence or handover exceptions. None is invented.

**Classification:** PASS WITH ISSUES — ownership boundary is clear; institution-specific pickup evidence/authority remains configurable.

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
