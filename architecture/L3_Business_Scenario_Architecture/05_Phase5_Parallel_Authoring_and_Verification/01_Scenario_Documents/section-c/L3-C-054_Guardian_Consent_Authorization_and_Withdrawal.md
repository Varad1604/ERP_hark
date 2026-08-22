# Guardian Consent Authorization and Withdrawal

**Scenario ID:** L3-C-054  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Guardian  
**Participating Domains:** A, C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-C06 — Guardian Workflows; L2-A02 — Person & Identity; L2-C01 — Student Lifecycle where materially applicable  
**Referenced By:** Related canonical Section C scenarios as applicable

## 1. Scenario Metadata

This scenario preserves the frozen Section C identity, ownership, participating domains, scenario type and priority. The Guardian Relationship Aggregate remains the canonical owner of guardian relationship state and decisions. No upstream ownership, rule, lifecycle, event publisher or authority is redefined.

## 2. Scenario Purpose

Govern institutionally recognized guardian consent for activities or decisions within the Guardian Relationship, including granting and withdrawing consent while preserving complete consent history.

## 3. Business Outcome

The applicable guardian consent decision is authoritatively recorded, the current consent position is available for institutional use, and prior consent decisions remain permanently auditable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An authorized guardian consent decision is requested for an existing Guardian Relationship.

### 4.2 End Boundary
The current consent position is recorded and the applicable downstream business consumers can rely on the authoritative Guardian consent state.

### 4.3 In Scope
- Identify the authoritative Guardian Relationship.
- Validate the requested consent against applicable institutional policy.
- Record consent granted or withdrawn.
- Preserve the prior consent decision as immutable history.
- Publish the corresponding canonical consent event.
- Make the resulting consent state available to authorized institutional consumers.

### 4.4 Out of Scope
- Person identity creation or correction.
- Student lifecycle decisions.
- Communication delivery itself.
- Financial transaction processing.
- Authentication.
- Deleting historical consent evidence.

### 4.5 Related Scenarios
C055 — Guardian Communication Authorization Governance; C056 — Student Emergency Contact Assignment.

## 5. Ownership and Participating Domains

The **Guardian Workflows Domain / Guardian Relationship Aggregate** is the sole canonical owner of Guardian Relationship state, consent, communication authorization and emergency-contact relationship decisions covered by this scenario.

Person & Identity remains authoritative for Person identity. Student Lifecycle remains authoritative for Student existence and lifecycle. Participating domains do not acquire Guardian ownership through collaboration or consumption.

## 6. Business Actors, Roles and Responsibilities

- Guardian Workflows owns consent state and consent decisions.
- The guardian supplies/initiates the business consent decision where applicable.
- Institutionally authorized authorities govern consent where policy requires institutional approval.
- Downstream domains consume the resulting consent fact without modifying it.

Only source-supported business authority is used. No technical IAM/RBAC role is introduced.

## 7. Preconditions and Entry Criteria

- Guardian Relationship exists.
- Consent activity is within Guardian Workflows scope.
- Applicable policy and authority are known.
- Required evidence is available where policy requires it.

## 8. Trigger

An authorized consent grant or withdrawal is initiated for an existing Guardian Relationship.

## 9. Main Business Flow

1. Identify the Guardian Relationship.
2. Validate the requested consent against applicable institutional policy.
3. Validate required business authority.
4. Record the consent decision.
5. Preserve prior consent history.
6. Publish `GuardianConsentGranted` when consent is granted or `GuardianConsentWithdrawn` when consent is withdrawn.
7. Expose the resulting authoritative consent position to participating institutional domains.

The main flow ends when the singular institutional outcome in Section 3 is achieved.

## 10. Alternate Business Flows

- Consent may be withdrawn in accordance with institutional policy.
- A withdrawal does not delete the earlier consent record.
- If required evidence or authority is absent, the consent decision remains unresolved rather than being invented or assumed.
- Mandatory institutional/legal notifications are not silently treated as optional consent-controlled communications.

## 11. Failure, Interruption and Business Recovery

A consent decision that cannot satisfy the applicable policy is not recorded as an approved consent outcome. Existing consent history remains intact. Corrections are represented by subsequent governed business decisions rather than rewriting historical consent.

Business recovery preserves the Guardian Relationship Aggregate's authoritative state and historical evidence. Technical retry, queue, rollback and infrastructure mechanics are intentionally outside L3.

## 12. Business Rules and Decision Constraints

- BR-013: Guardian consent is managed exclusively within Guardian Workflows.
- BR-014: consent decisions require traceability including decision, timestamp, initiating authority, applicable policy and audit reference.
- BR-015: consent may be withdrawn, but historical consent records are retained.
- BR-016/017: communication authorization and preferences do not override mandatory institutional notifications.
- BR-032/033: complete auditability and immutable business history.

These constraints are derived from locked L2-C06; no local replacement rule is created.

## 13. State and Lifecycle Impact

Consent is a governed aspect of the Guardian Relationship Aggregate. The L2 workflow specifies consent changes as **Active → Modified** and returns the relationship to its governed operational position. Consent grant/withdrawal does not create a new Guardian lifecycle state.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

- `GuardianConsentGranted` — Guardian Relationship Aggregate.
- `GuardianConsentWithdrawn` — Guardian Relationship Aggregate.
- These are distinct canonical business facts and must not be collapsed into a generic consent event.

The Guardian Relationship Aggregate remains the canonical publisher for Guardian domain events. Events are completed business facts, not commands or technical messages.

## 15. Business Authorization, Approval and Audit Obligations

Consent decisions require the appropriate institutional authority under applicable policy. The exact institutional authority is not invented at L3. Every consent decision is auditable, and withdrawal preserves the prior decision history.

## 16. Completion Criteria and Postconditions

- Guardian Relationship identified.
- Policy/authority validated.
- Consent granted or withdrawn as applicable.
- Historical consent preserved.
- Canonical consent event published.
- Current consent position is authoritative for downstream business use.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C054-01:** L2-C06 does not define a universal taxonomy of consentable activities; L3 does not invent one.

**OI-C054-02:** Exact institutional approval authority is policy-configurable.

**Classification:** PASS WITH ISSUES — no ownership conflict identified; institution-specific consent policy remains upstream/configurable.

## Production Verification Record

- [x] Frozen Section C identity preserved.
- [x] Canonical Guardian Relationship ownership preserved.
- [x] Upstream Person/Student ownership preserved.
- [x] Canonical Guardian workflow/event vocabulary used.
- [x] No implementation-level architecture introduced.
- [x] All 17 mandatory template sections present.
- [x] Material ambiguity/gaps recorded in Section 17.
- [x] Status is intentionally **Draft**, as required for new Phase 5 authored L3 scenarios; the content is prepared to production-document quality but is not represented as Architecture Approved or Locked.
