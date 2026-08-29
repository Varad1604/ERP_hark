# L3-C-051 — Guardian Relationship Establishment and Verification

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** L3-C-051  
**Canonical Title:** Guardian Relationship Establishment and Verification  
**Primary Owner:** Guardian  
**Participating Domains:** A, C  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Derivation:** KEEP  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  

## 1. Scenario Metadata
Frozen Section C identity is preserved exactly. The Guardian Relationship Aggregate remains the canonical business owner. No ownership, title, participating-domain, type, priority, or upstream rule is reclassified.

## 2. Scenario Purpose
Establish and verify an institutional relationship between an existing Student and an authoritative Person acting as Guardian, so that the relationship can proceed into the governed Guardian lifecycle.

## 3. Business Outcome
A valid Guardian Relationship is established and, where institutional policy requires verification, progresses through verification to the authoritative verified relationship state.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An institutionally authorized request identifies a Student and an authoritative Person for guardian relationship establishment.

### 4.2 End Boundary
The Guardian Relationship exists in the appropriate verified lifecycle position, with the establishment and verification business facts recorded.

### 4.3 In Scope
- Resolve the authoritative Student and Person identities.
- Validate institutional Guardian policy.
- Establish the Guardian Relationship in Proposed.
- Determine whether verification is required.
- Perform Guardian Verification where required.
- Record the verification decision.
- Preserve the complete relationship history.

### 4.4 Out of Scope
- Creating or modifying Person identity.
- Creating or modifying Student identity/lifecycle.
- Authentication or authorization service operation.
- Financial transactions.
- Communication delivery.
- Any downstream modification of Guardian state.

### 4.5 Related Scenarios
C052 — Guardian Relationship Modification; C053 — Guardian Legal Custody Designation and Change.

## 5. Ownership and Participating Domains
The **Guardian Workflows Domain / Guardian Relationship Aggregate** is the sole canonical owner of guardian relationship state and decisions. Person Identity remains authoritative for person identity; Student Lifecycle remains authoritative for student existence/lifecycle; participating downstream domains consume Guardian business facts without acquiring ownership.

## 6. Business Actors, Roles and Responsibilities
- Student Lifecycle supplies authoritative Student existence/lifecycle.
- Person & Identity supplies authoritative Person identity.
- Guardian Workflows owns the relationship and verification decision.
- An authorized institutional authority approves/rejects verification where required.

No technical IAM/RBAC roles are introduced.

## 7. Preconditions and Entry Criteria
- Student exists.
- Guardian Person exists.
- Institution is active.
- Guardian policy permits establishment.
- Required authority is present.

## 8. Trigger
An institution requests establishment of a guardian relationship.

## 9. Main Business Flow
1. Validate the Student reference against Student Lifecycle.
2. Validate the Guardian Person reference against Person & Identity.
3. Validate applicable institutional Guardian policy.
4. Establish the Guardian Relationship in Proposed.
5. Move to Pending Verification when verification is required.
6. Perform Guardian Verification against the applicable evidence and policy.
7. Record the verification decision.
8. When verification succeeds, move to Verified.
9. Preserve the relationship history and publish the applicable canonical Guardian events.

The flow stops when the complete guardian business outcome is reached.

## 10. Alternate Business Flows
- Verification may be deferred by institutional policy.
- A Guardian may already have an independent relationship with another Student; each relationship remains separately governed.
- Verification rejection leaves the relationship outside the verified/active responsibility state until a governed subsequent verification outcome exists.
- An existing duplicate active relationship representing the same Guardian Role may be rejected unless institution policy explicitly permits it.

## 11. Failure, Interruption and Business Recovery
- Do not establish a relationship for a non-existent Student or unresolved Person.
- Do not activate guardian responsibilities before required verification.
- A rejected verification remains auditable; it is not silently overwritten.
- Historical verification decisions are preserved as separate business history.

Recovery is business-level only. No APIs, queues, technical retries, rollback, infrastructure or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- BR-001: Guardian Relationship only for an existing Student.
- BR-002: exactly one authoritative Person identity per Guardian Relationship.
- BR-004: duplicate active Guardian Role relationships are prohibited unless policy permits.
- BR-005: newly established relationships begin Proposed.
- BR-006: verification precedes Active where mandated.
- BR-007: completed verification decisions remain auditable.
- BR-008: only authorized institutional authorities approve/reject verification.
- BR-009: only Verified relationships receive institutional responsibilities.
- BR-010/011: multiple guardians and multiple Student relationships are supported.

These are applied from locked L2-C06; no local substitute rule is created.

## 13. State and Lifecycle Impact
- **Guardian Relationship:** Proposed → Pending Verification where verification is required.
- **Guardian Relationship:** Pending Verification → Verified on successful verification.
- Active is a later GW-03 assignment outcome and is not silently collapsed into this scenario.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration
- `GuardianRelationshipCreated` — Guardian Relationship Aggregate.
- `GuardianVerificationCompleted` — Guardian Relationship Aggregate.
- These are distinct business facts; relationship creation does not imply successful verification.

Canonical Guardian Domain Event publishers remain the Guardian Relationship Aggregate. Downstream consumers treat these events as read-only business facts.

## 15. Business Authorization, Approval and Audit Obligations
Verification and approval require appropriate institutional authority. The exact institutional authority is policy-configurable; no universal role is invented. Every verification decision is permanently auditable.

## 16. Completion Criteria and Postconditions
- Student and Person references validated.
- Guardian Relationship established.
- Required verification completed.
- Verified state reached where applicable.
- Establishment and verification events published.
- Historical relationship/verification evidence preserved.

## 17. Open Issues, Assumptions and Deferred Questions
**OI-C051-01:** L2-C06 permits verification to be deferred by institutional policy; the scenario does not invent a universal mandatory verification sequence.

**OI-C051-02:** Exact institutional verification authority is configurable.

**Classification:** PASS WITH ISSUES — source-grounded, with institution-specific verification policy remaining upstream.

## Production Verification Record
- [x] Frozen ID/title/owner/domains/type/priority preserved.
- [x] Guardian Relationship Aggregate ownership preserved.
- [x] Person Identity and Student Lifecycle ownership preserved.
- [x] Canonical GW workflow and GDE events applied.
- [x] No implementation leakage.
- [x] All 17 mandatory sections present.
- [x] Material gaps recorded in Section 17.
- [x] Status remains Draft per Phase 5 governance.
