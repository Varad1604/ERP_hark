# L3-C-053 — Guardian Legal Custody Designation and Change

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** L3-C-053  
**Canonical Title:** Guardian Legal Custody Designation and Change  
**Primary Owner:** Guardian  
**Participating Domains:** A, C  
**Scenario Type:** Critical  
**Priority:** P1  
**Derivation:** KEEP  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  

## 1. Scenario Metadata
Frozen Section C identity is preserved exactly. The Guardian Relationship Aggregate remains the canonical business owner. No ownership, title, participating-domain, type, priority, or upstream rule is reclassified.

## 2. Scenario Purpose
Maintain legally recognized custody information within an institutional Guardian Relationship while preserving the distinction between Guardian relationship ownership and authoritative Person/Student identity ownership.

## 3. Business Outcome
The applicable legal custody designation or change is authoritatively recorded, conflicting active custody assignments are prevented, historical custody information is preserved, and the Guardian Relationship remains internally consistent.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An authorized custody designation or change is presented for an existing Guardian Relationship.

### 4.2 End Boundary
The legally governed custody information is updated and the completed change is recorded through the canonical Guardian event.

### 4.3 In Scope
- Identify the affected Guardian Relationship.
- Validate the authoritative custody information and applicable institutional policy.
- Determine whether the proposed custody arrangement conflicts with an existing active custody assignment.
- Record the legal custody designation/change.
- Preserve historical custody records.
- Publish `LegalCustodyUpdated`.

### 4.4 Out of Scope
- Establishing Person identity.
- Deciding Student lifecycle status.
- Providing legal advice or independently determining legal facts outside institutional policy/evidence.
- Authentication.
- Financial transaction processing.
- Communication delivery.

### 4.5 Related Scenarios
C052 — Guardian Relationship Modification; C054 — Guardian Consent Authorization and Withdrawal; C059 — Guardian Relationship Suspension; C061 — Guardian Relationship Termination.

## 5. Ownership and Participating Domains
The **Guardian Workflows Domain / Guardian Relationship Aggregate** is the sole canonical owner of guardian relationship state and decisions. Person Identity remains authoritative for person identity; Student Lifecycle remains authoritative for student existence/lifecycle; participating downstream domains consume Guardian business facts without acquiring ownership.

## 6. Business Actors, Roles and Responsibilities
- Guardian Workflows owns institutional custody relationship state.
- Person & Identity owns the underlying Person identity.
- Student Lifecycle owns the Student lifecycle.
- The authoritative evidence/source for legal custody remains upstream where applicable.

No technical IAM/RBAC roles are introduced.

## 7. Preconditions and Entry Criteria
- Guardian Relationship exists.
- The proposed custody designation is within institutional Guardian policy.
- Required evidence/authorization is available.
- No conflicting active custody assignment remains unresolved.

## 8. Trigger
An authorized legal-custody designation or change is requested for an existing Guardian Relationship.

## 9. Main Business Flow
1. Identify the affected Guardian Relationship.
2. Resolve the authoritative Person and Student references.
3. Validate the proposed custody information against applicable institutional policy.
4. Check for conflicting active custody assignments.
5. Record the custody designation/change when valid.
6. Preserve prior custody history.
7. Transition the relationship through the governed modification path.
8. Publish `LegalCustodyUpdated`.

The flow stops when the complete guardian business outcome is reached.

## 10. Alternate Business Flows
- Proposed custody conflicts with an active conflicting custody record: do not apply the conflicting designation until the authoritative conflict is resolved.
- Required evidence is incomplete: retain the request unresolved.
- Custody change occurs while the relationship is otherwise active: apply it through Active → Modified while preserving relationship continuity.

## 11. Failure, Interruption and Business Recovery
- Conflicting active custody records are not silently overwritten.
- Historical custody information is never modified.
- Guardian Workflows records the institutional relationship/custody outcome but does not assume ownership of Person identity or Student lifecycle.

Recovery is business-level only. No APIs, queues, technical retries, rollback, infrastructure or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- BR-012: every Guardian Relationship has at least one Guardian Role.
- Aggregate invariant: legal custody assignments shall never overlap with conflicting active custody records.
- BR-022: lifecycle transitions remain within Guardian Relationship consistency boundary.
- BR-025: historical Guardian Relationships remain immutable.
- BR-026/027: appropriate institutional authorization and possible separation of duties apply.
- BR-032/033: complete auditability and immutable business history.

These are applied from locked L2-C06; no local substitute rule is created.

## 13. State and Lifecycle Impact
- **Guardian Relationship:** Active → Modified → Active for custody changes.
- The relationship remains governed by the Guardian Relationship Aggregate.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration
`LegalCustodyUpdated` — published by the Guardian Relationship Aggregate after successful completion of the custody change.

Canonical Guardian Domain Event publishers remain the Guardian Relationship Aggregate. Downstream consumers treat these events as read-only business facts.

## 15. Business Authorization, Approval and Audit Obligations
Legal custody designation/change requires appropriate institutional authorization and auditable evidence. Exact approval roles are institution-configurable; no universal legal authority is invented.

## 16. Completion Criteria and Postconditions
- Correct Guardian Relationship identified.
- Custody evidence/policy validated.
- Conflicting active custody avoided/resolved.
- Custody designation recorded.
- Historical custody preserved.
- `LegalCustodyUpdated` published.

## 17. Open Issues, Assumptions and Deferred Questions
**OI-C053-01:** L2-C06 defines the institutional custody business rule but does not specify a universal legal-document/evidence taxonomy. None is invented.

**OI-C053-02:** The canonical upstream owner of external legal facts is not defined in the retrieved L2; Guardian Workflows owns the institutional relationship/custody state, not an external legal fact.

**Classification:** PASS WITH ISSUES — source-grounded with institution-specific custody evidence/authority remaining configurable.

## Production Verification Record
- [x] Frozen ID/title/owner/domains/type/priority preserved.
- [x] Guardian Relationship Aggregate ownership preserved.
- [x] Person Identity and Student Lifecycle ownership preserved.
- [x] Canonical GW workflow and GDE events applied.
- [x] No implementation leakage.
- [x] All 17 mandatory sections present.
- [x] Material gaps recorded in Section 17.
- [x] Status remains Draft per Phase 5 governance.
