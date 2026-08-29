# Student Emergency Contact Assignment

**Scenario ID:** L3-C-056  
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

Maintain the authoritative Guardian-based emergency contact assignment and priority for a Student, ensuring that emergency contact responsibilities remain valid and auditable.

## 3. Business Outcome

The Student has an authoritative emergency-contact position based on eligible active Guardian Relationships, including the required institutional priority, and the completed change is published as a Guardian business fact.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An authorized emergency-contact assignment or change is requested for an existing Student/Guardian relationship.

### 4.2 End Boundary
The emergency-contact hierarchy/assignment is recorded in the Guardian Relationship Aggregate and `GuardianEmergencyContactUpdated` is published.

### 4.3 In Scope
- Identify the Student and relevant Guardian Relationship.
- Validate that the Guardian Relationship is Active.
- Validate emergency-contact eligibility and priority.
- Record or update the emergency-contact designation.
- Preserve historical emergency-contact information.
- Publish `GuardianEmergencyContactUpdated`.
- Make the current emergency-contact fact available to authorized institutional consumers.

### 4.4 Out of Scope
- Creating a Student.
- Creating a Person identity.
- Medical decision-making outside Guardian relationship governance.
- Emergency response operations themselves.
- Communication delivery.
- Transport or hostel operational execution.

### 4.5 Related Scenarios
C054 — Guardian Consent Authorization and Withdrawal; C055 — Guardian Communication Authorization Governance; C058 — Student Pickup Authorization Management.

## 5. Ownership and Participating Domains

The **Guardian Workflows Domain / Guardian Relationship Aggregate** is the sole canonical owner of Guardian Relationship state, consent, communication authorization and emergency-contact relationship decisions covered by this scenario.

Person & Identity remains authoritative for Person identity. Student Lifecycle remains authoritative for Student existence and lifecycle. Participating domains do not acquire Guardian ownership through collaboration or consumption.

## 6. Business Actors, Roles and Responsibilities

- Guardian Workflows owns the institutional emergency-contact designation and priority attached to Guardian relationships.
- Student Lifecycle owns Student identity/lifecycle.
- Person & Identity owns the underlying person identity.
- Health Centre, Hostel, Transport and other consumers use the published Guardian fact for their own operational purposes.

Only source-supported business authority is used. No technical IAM/RBAC role is introduced.

## 7. Preconditions and Entry Criteria

- Student exists.
- Relevant Guardian Relationship exists.
- Guardian Relationship is Active where required by L2-C06.
- Emergency-contact assignment is permitted by institutional policy.
- Required business authority exists.

## 8. Trigger

An authorized request assigns or changes a Guardian's emergency-contact status for a Student.

## 9. Main Business Flow

1. Identify the authoritative Student and Guardian Relationship.
2. Confirm the Guardian Relationship is Active.
3. Validate the requested emergency-contact designation.
4. Validate the unique institutional priority requirement.
5. Apply the designation/change within the Guardian Relationship Aggregate.
6. Preserve the prior emergency-contact history.
7. Publish `GuardianEmergencyContactUpdated`.
8. Make the resulting emergency-contact fact available to authorized downstream consumers.

The main flow ends when the singular institutional outcome in Section 3 is achieved.

## 10. Alternate Business Flows

- A Guardian that is not Active cannot function as an Emergency Contact.
- If the requested priority conflicts with the institution's unique priority requirement, the conflicting designation is not silently accepted.
- A change to one emergency contact preserves the historical record of the previous designation.
- The emergency-contact relationship does not transfer ownership of Student lifecycle or medical/operational processes.

## 11. Failure, Interruption and Business Recovery

An invalid or conflicting emergency-contact change is not applied. Existing authoritative emergency-contact state remains intact. Resolution follows the institution's governed business process; no technical retry or rollback behavior is specified.

Business recovery preserves the Guardian Relationship Aggregate's authoritative state and historical evidence. Technical retry, queue, rollback and infrastructure mechanics are intentionally outside L3.

## 12. Business Rules and Decision Constraints

- BR-018: Emergency Contacts possess a unique institutional priority ranking.
- BR-019: only Active Guardian Relationships may function as Emergency Contacts.
- BR-022: lifecycle transitions remain within the Guardian Relationship consistency boundary.
- BR-025: historical Guardian Relationships remain immutable.
- BR-026/027: appropriate institutional authority and possible separation of duties.
- BR-032/033: complete auditability and immutable business history.

These constraints are derived from locked L2-C06; no local replacement rule is created.

## 13. State and Lifecycle Impact

Emergency-contact management is a Guardian Relationship modification and follows the L2-C06 **Active → Modified** business transition. No separate Emergency Contact lifecycle is invented.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

`GuardianEmergencyContactUpdated` — Guardian Relationship Aggregate. Downstream Health Centre, Hostel, Transport and other domains consume the business fact but do not publish Guardian relationship state.

The Guardian Relationship Aggregate remains the canonical publisher for Guardian domain events. Events are completed business facts, not commands or technical messages.

## 15. Business Authorization, Approval and Audit Obligations

Emergency-contact designation requires appropriate institutional authority and must satisfy the unique priority and Active Guardian constraints. The resulting decision is auditable.

## 16. Completion Criteria and Postconditions

- Student/Guardian Relationship identified.
- Active Guardian eligibility confirmed.
- Priority validated.
- Emergency-contact designation recorded.
- Historical state preserved.
- `GuardianEmergencyContactUpdated` published.
- Downstream consumers can rely on the current authoritative emergency-contact fact.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C056-01:** L2-C06 defines emergency priority as a unique institutional ranking but does not prescribe a universal numeric/order scheme. L3 does not invent one.

**OI-C056-02:** The scenario does not define emergency response execution; downstream operational domains retain their own responsibilities.

**Classification:** PASS WITH ISSUES — source-grounded with institution-specific priority policy remaining upstream.

## Production Verification Record

- [x] Frozen Section C identity preserved.
- [x] Canonical Guardian Relationship ownership preserved.
- [x] Upstream Person/Student ownership preserved.
- [x] Canonical Guardian workflow/event vocabulary used.
- [x] No implementation-level architecture introduced.
- [x] All 17 mandatory template sections present.
- [x] Material ambiguity/gaps recorded in Section 17.
- [x] Status is intentionally **Draft**, as required for new Phase 5 authored L3 scenarios; the content is prepared to production-document quality but is not represented as Architecture Approved or Locked.
