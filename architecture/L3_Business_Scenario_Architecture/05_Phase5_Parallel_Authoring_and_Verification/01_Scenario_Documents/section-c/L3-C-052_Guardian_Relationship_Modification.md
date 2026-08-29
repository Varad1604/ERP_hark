# L3-C-052 — Guardian Relationship Modification

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** L3-C-052  
**Canonical Title:** Guardian Relationship Modification  
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
Maintain an existing institutional Guardian Relationship by changing governed relationship information while preserving aggregate ownership, authorization, auditability, and historical continuity.

## 3. Business Outcome
The authorized modification is applied to the active Guardian Relationship, the resulting relationship remains internally consistent, and the completed business change is recorded as immutable history.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An existing Guardian Relationship is in an eligible lifecycle state and an authorized business change is requested.

### 4.2 End Boundary
The requested governed relationship modification is completed and the relationship returns to its applicable operational state with historical evidence preserved.

### 4.3 In Scope
- Validate the target Guardian Relationship.
- Validate the requested relationship change against Guardian policy.
- Apply changes to governed relationship responsibilities or attributes, such as Guardian Role, communication permissions, financial responsibility, emergency designation, or contact priority.
- Preserve the prior business history.
- Publish the applicable Guardian modification event.

### 4.4 Out of Scope
- Person identity correction.
- Student lifecycle change.
- Financial transaction.
- Communication delivery.
- Authentication/authorization.
- Direct modification by a downstream domain.

### 4.5 Related Scenarios
C051 — Guardian Relationship Establishment and Verification; C053 — Guardian Legal Custody Designation and Change; C054 — Guardian Consent Authorization and Withdrawal; C055 — Guardian Communication Authorization Governance; C056 — Student Emergency Contact Assignment; C057 — Guardian Financial Responsibility Assignment; C058 — Student Pickup Authorization Management; C059 — Guardian Relationship Suspension.

## 5. Ownership and Participating Domains
The **Guardian Workflows Domain / Guardian Relationship Aggregate** is the sole canonical owner of guardian relationship state and decisions. Person Identity remains authoritative for person identity; Student Lifecycle remains authoritative for student existence/lifecycle; participating downstream domains consume Guardian business facts without acquiring ownership.

## 6. Business Actors, Roles and Responsibilities
- Guardian Workflows owns relationship modification.
- Person & Identity remains authoritative for person identity.
- Student Lifecycle remains authoritative for Student existence/lifecycle.
- Downstream domains consume the resulting Guardian facts.

No technical IAM/RBAC roles are introduced.

## 7. Preconditions and Entry Criteria
- Guardian Relationship exists.
- Requested change is within Guardian domain scope.
- Required institutional authorization exists.
- The relationship is not permanently Historical.

## 8. Trigger
An authorized request to modify an existing Guardian Relationship is received.

## 9. Main Business Flow
1. Identify the authoritative Guardian Relationship.
2. Validate the requested modification against Guardian policy.
3. Validate applicable authorization.
4. Apply the governed relationship change.
5. Maintain aggregate invariants.
6. Preserve the prior relationship history.
7. Return the lifecycle to Active where the modification occurs through Active → Modified → Active.
8. Publish `GuardianRelationshipUpdated` and any more specific canonical event required by the change.

The flow stops when the complete guardian business outcome is reached.

## 10. Alternate Business Flows
- Role change publishes `GuardianRoleChanged`.
- Legal custody change follows GW-05 and publishes `LegalCustodyUpdated`.
- Consent change follows GW-06 and publishes the appropriate consent event.
- Communication authorization change follows GW-07.
- Emergency contact change follows GW-08.
- Financial responsibility follows GW-09.
- Pickup authorization follows GW-10.
- A suspended relationship may not receive responsibilities that require Active state.

## 11. Failure, Interruption and Business Recovery
- Do not alter historical relationship records.
- Do not permit conflicting active custody assignments.
- Do not permit communication authorization to bypass consent.
- Do not assign pickup authorization to a non-active relationship.
- Do not manage financial transactions; only Guardian financial responsibility assignment belongs here.

Recovery is business-level only. No APIs, queues, technical retries, rollback, infrastructure or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- BR-013/014/015: consent is Guardian-owned, traceable and withdrawable without deleting history.
- BR-016/017: communication authorization and preferences govern permitted communication without overriding mandatory notices.
- BR-018/019: emergency priority and active-contact requirements.
- BR-020/021: financial responsibility assignment does not transfer Finance transaction ownership.
- BR-022: lifecycle transition remains within Guardian Relationship consistency boundary.
- BR-032/033: complete auditability and immutable business history.
- BR-034/035: institution-configurable policy and Guardian Roles.

These are applied from locked L2-C06; no local substitute rule is created.

## 13. State and Lifecycle Impact
- **Guardian Relationship:** Active → Modified → Active for ordinary relationship changes.
- A specific governed sub-workflow may produce its own canonical lifecycle transition where applicable.
- Historical records remain immutable.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration
- `GuardianRelationshipUpdated` — general relationship modification.
- `GuardianRoleChanged` — role modification.
- More specific canonical events are used for custody, consent, communication, emergency, financial responsibility and pickup changes.

Canonical Guardian Domain Event publishers remain the Guardian Relationship Aggregate. Downstream consumers treat these events as read-only business facts.

## 15. Business Authorization, Approval and Audit Obligations
Guardian state changes require appropriate institutional authorization. Separation of duties may apply to verification, approval, suspension and termination according to institutional policy. All operations are auditable.

## 16. Completion Criteria and Postconditions
- Requested change validated.
- Applicable authorization satisfied.
- Aggregate invariants preserved.
- New relationship state is authoritative.
- Prior history retained.
- Appropriate canonical event(s) published.

## 17. Open Issues, Assumptions and Deferred Questions
**OI-C052-01:** The exact set of institution-specific change authorities is configurable.

**OI-C052-02:** This scenario is the umbrella relationship-maintenance outcome; specialized changes retain their canonical GW workflow semantics and events rather than being collapsed into a generic event.

**Classification:** PASS — no unresolved ownership conflict identified in locked L2-C06.

## Production Verification Record
- [x] Frozen ID/title/owner/domains/type/priority preserved.
- [x] Guardian Relationship Aggregate ownership preserved.
- [x] Person Identity and Student Lifecycle ownership preserved.
- [x] Canonical GW workflow and GDE events applied.
- [x] No implementation leakage.
- [x] All 17 mandatory sections present.
- [x] Material gaps recorded in Section 17.
- [x] Status remains Draft per Phase 5 governance.
