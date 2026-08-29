# Guardian Communication Authorization Governance

**Scenario ID:** L3-C-055  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Guardian  
**Participating Domains:** A, C  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-C06 — Guardian Workflows; L2-A02 — Person & Identity; L2-C01 — Student Lifecycle where materially applicable  
**Referenced By:** Related canonical Section C scenarios as applicable

## 1. Scenario Metadata

This scenario preserves the frozen Section C identity, ownership, participating domains, scenario type and priority. The Guardian Relationship Aggregate remains the canonical owner of guardian relationship state and decisions. No upstream ownership, rule, lifecycle, event publisher or authority is redefined.

## 2. Scenario Purpose

Govern which institutional communications a Guardian is authorized to receive, while preserving the distinction between Guardian authorization and the Communication Services responsibility for actual delivery.

## 3. Business Outcome

The Guardian Relationship contains an authoritative communication-authorization position, and downstream communication consumers can determine whether the Guardian is authorized for the relevant institutional communication.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An authorized request changes or establishes communication authorization for an existing Guardian Relationship.

### 4.2 End Boundary
The communication authorization decision is recorded and the canonical `CommunicationAuthorizationUpdated` business fact is published.

### 4.3 In Scope
- Identify the Guardian Relationship.
- Validate communication authorization against consent and institutional policy.
- Record the authorization decision.
- Preserve prior authorization history.
- Publish `CommunicationAuthorizationUpdated`.
- Allow Communication Services and other consumers to use the authoritative authorization fact.

### 4.4 Out of Scope
- Sending or delivering communications.
- Managing communication infrastructure.
- Person identity management.
- Student lifecycle management.
- Authentication/authorization technology.
- Changing consent history outside the Guardian consent workflow.

### 4.5 Related Scenarios
C054 — Guardian Consent Authorization and Withdrawal; C056 — Student Emergency Contact Assignment.

## 5. Ownership and Participating Domains

The **Guardian Workflows Domain / Guardian Relationship Aggregate** is the sole canonical owner of Guardian Relationship state, consent, communication authorization and emergency-contact relationship decisions covered by this scenario.

Person & Identity remains authoritative for Person identity. Student Lifecycle remains authoritative for Student existence and lifecycle. Participating domains do not acquire Guardian ownership through collaboration or consumption.

## 6. Business Actors, Roles and Responsibilities

- Guardian Workflows owns the institutional communication authorization attached to the Guardian Relationship.
- Communication Services owns message delivery.
- Person & Identity owns person identity.
- Downstream consumers use Guardian authorization as a business fact and do not modify Guardian state.

Only source-supported business authority is used. No technical IAM/RBAC role is introduced.

## 7. Preconditions and Entry Criteria

- Guardian Relationship exists.
- Communication authorization is within Guardian scope.
- Applicable consent and institutional policy can be evaluated.
- Required authority is available.

## 8. Trigger

An authorized communication-authorization change is requested for a Guardian Relationship.

## 9. Main Business Flow

1. Identify the authoritative Guardian Relationship.
2. Determine the current consent position.
3. Evaluate requested communication authorization against applicable policy.
4. Record the authorization decision.
5. Preserve prior authorization history.
6. Publish `CommunicationAuthorizationUpdated`.
7. Provide the resulting authorization fact to Communication Services and other authorized consumers.

The main flow ends when the singular institutional outcome in Section 3 is achieved.

## 10. Alternate Business Flows

- Communication preferences may change without changing the underlying Guardian relationship.
- A Guardian may be unauthorized for a particular communication while remaining an active Guardian.
- Mandatory institutional notifications required by law or policy are not silently disabled by an ordinary communication preference.
- If consent required for the requested authorization is absent, the requested authorization is not approved.

## 11. Failure, Interruption and Business Recovery

Communication delivery remains outside this scenario. If authorization cannot be established, the authorization remains unchanged and historical state is preserved. A subsequent governed authorization decision may establish a new current position.

Business recovery preserves the Guardian Relationship Aggregate's authoritative state and historical evidence. Technical retry, queue, rollback and infrastructure mechanics are intentionally outside L3.

## 12. Business Rules and Decision Constraints

- BR-016: institutional communications are only delivered to Guardians authorized to receive them.
- BR-017: communication preferences influence delivery but cannot override mandatory institutional notifications.
- BR-013/015: consent is Guardian-owned and may be withdrawn without deleting historical consent.
- BR-032/033: complete auditability and immutable history.
- BR-031: notification delivery remains exclusively owned by Communication Services; Guardian Workflows governs authorization only.

These constraints are derived from locked L2-C06; no local replacement rule is created.

## 13. State and Lifecycle Impact

Communication authorization changes occur through the Guardian Relationship's governed modification path, identified in L2-C06 as **Active → Modified** for communication-authorization management. No new lifecycle state is introduced.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

`CommunicationAuthorizationUpdated` — Guardian Relationship Aggregate. Communication Services is a consumer/operational downstream domain, not the publisher of Guardian authorization.

The Guardian Relationship Aggregate remains the canonical publisher for Guardian domain events. Events are completed business facts, not commands or technical messages.

## 15. Business Authorization, Approval and Audit Obligations

Authorization requires appropriate institutional authority and applicable consent/policy evaluation. All authorization changes are auditable. No technical IAM/RBAC model is introduced.

## 16. Completion Criteria and Postconditions

- Current Guardian Relationship identified.
- Consent/policy constraints satisfied.
- Communication authorization recorded.
- Historical authorization preserved.
- `CommunicationAuthorizationUpdated` published.
- Communication Services can consume the authoritative business authorization.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C055-01:** L2-C06 does not define a universal taxonomy of communication categories; L3 therefore does not invent categories.

**OI-C055-02:** Communication Services owns delivery, while Guardian owns authorization. This boundary is explicit and must not be collapsed.

**Classification:** PASS — no unresolved ownership conflict identified in locked L2-C06.

## Production Verification Record

- [x] Frozen Section C identity preserved.
- [x] Canonical Guardian Relationship ownership preserved.
- [x] Upstream Person/Student ownership preserved.
- [x] Canonical Guardian workflow/event vocabulary used.
- [x] No implementation-level architecture introduced.
- [x] All 17 mandatory template sections present.
- [x] Material ambiguity/gaps recorded in Section 17.
- [x] Status is intentionally **Draft**, as required for new Phase 5 authored L3 scenarios; the content is prepared to production-document quality but is not represented as Architecture Approved or Locked.
