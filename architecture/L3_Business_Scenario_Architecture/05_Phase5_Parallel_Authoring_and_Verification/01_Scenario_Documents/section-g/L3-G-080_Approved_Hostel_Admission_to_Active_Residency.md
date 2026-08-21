# Approved Hostel Admission to Active Residency

**Scenario ID:** L3-G-080  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** C, G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-G03 — Hostel Management Workflows; locked L1 Business Domain Architecture; Section C Student Lifecycle context; DEPENDENCY_MAP.md  
**Referenced By:** Downstream hostel residency scenarios including leave, room transfer, and exit processing as applicable  

## 2. Scenario Purpose

Convert an approved hostel admission into an active hostel residency by completing the required operational room allocation and check-in activities while keeping student identity and infrastructure ownership in their canonical domains.

## 3. Business Outcome

The approved student is checked into the assigned hostel room and the Hostel Occupancy lifecycle reaches **Active**, establishing a governed operational residency.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a Hostel Admission has reached the authoritative `Approved` state.

### 4.2 End Boundary

The scenario ends when the student has completed the governed hostel check-in and the Hostel Occupancy lifecycle is active.

### 4.3 In Scope

- Validation that the approved hostel admission can proceed to residency.
- Room search and allocation within the Hostel Management business boundary.
- Operational arrival verification and required check-in verification.
- Hostel acceptance and activation of occupancy.
- Publication of the canonical occupancy-activation event.
- Preservation of audit history.

### 4.4 Out of Scope

- Hostel admission decision itself; it is completed before this scenario begins.
- Creation or modification of hostel buildings, floors, rooms, capacity, or infrastructure lifecycle; those are owned by G10 Infrastructure Management.
- Modification of Student Lifecycle state or academic status.
- Leave, room transfer, visitor, discipline, mess, complaint, or exit processes that occur after active residency.
- Financial accounting or billing.
- Technical authentication, API, database, deployment, or infrastructure mechanics.

### 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-G-079 — Hostel Admission Application to Admission Decision | An approved Hostel Admission is the required business starting condition. |

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Hostel Management (G) | Owns Room Allocation, Check-In, Hostel Occupancy, and activation of active residency. |
| Student Lifecycle (C) | Provides canonical Student Identity / student context used to validate the resident; it does not own Hostel Occupancy. |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student | Presents for hostel arrival and completes the resident-facing check-in requirements applicable to the institution. | Participates in residency commencement. |
| Hostel Administrator | Governs room allocation and hostel check-in processing. | Operational authority supported by L2-G03. |
| Hostel Warden | Participates in hostel operational governance and may be involved in applicable approval/acceptance decisions. | Business authority is configurable. |

No additional business actor is introduced beyond roles supported by L2-G03.

## 7. Preconditions and Entry Criteria

- The Hostel Admission is in the authoritative `Approved` state.
- A canonical Student Identity is available from Section C.
- An assignable hostel room is available under the applicable operational allocation policy.
- The student's hostel admission remains eligible for activation under the applicable business rules.

## 8. Trigger

An approved Hostel Admission is released for room allocation and operational check-in.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Hostel Management confirms that the approved admission is eligible to proceed to residency activation. | Hostel Management | Approved residency eligibility is available for room allocation. |
| 2 | An eligible hostel room is identified and allocated using the applicable hostel allocation policy. | Hostel Administrator / Hostel Management | A Room Allocation is established for the resident. |
| 3 | The arriving student is verified against the approved hostel admission and assigned room. | Hostel Management with Student Lifecycle identity context | Arrival is validated for the intended residency. |
| 4 | Required operational check-in verification and applicable document/identity checks are completed. | Hostel Management | The resident satisfies the governed check-in prerequisites. |
| 5 | Hostel Management accepts the check-in and activates the Hostel Occupancy. | Hostel Management | Hostel Occupancy transitions to `Active`. |
| 6 | The occupancy activation is published as the canonical hostel business event. | Hostel Management | Interested downstream consumers receive the authoritative residency-activation fact. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | No assignable room is available at activation time. | Do not activate residency; use an applicable alternative allocation or governed waiting/administrative resolution path. | The student does not reach Active Occupancy. | Hostel Management |
| A2 | Check-in verification does not satisfy the applicable policy. | Do not activate occupancy and resolve the business deficiency before reattempting check-in. | Hostel Occupancy remains inactive. | Hostel Management |
| A3 | The approved admission is no longer valid for activation under the applicable business rules. | Prevent occupancy activation and return the admission/residency matter to the appropriate hostel business resolution path. | No active residency is created. | Hostel Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Room availability changes before allocation is completed. | The intended room cannot be allocated. | Identify another eligible room under the configured allocation rules, or apply the applicable alternative resolution path. | Hostel Management |
| Identity or required check-in evidence cannot be validated. | Check-in cannot be completed authoritatively. | Resolve the student-information or evidence deficiency through the owning business source before occupancy activation. | Hostel Management / Student Lifecycle |
| A required business acceptance/approval is not available. | Occupancy cannot become Active. | Hold activation until the applicable business approval or acceptance requirement is completed. | Hostel Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only approved hostel admissions may create occupancy. | Hostel Management | Active residency cannot be created from an application that has not reached `Approved`. | L2-G03 §13, §16–§18 |
| Room allocation may consider Hostel Category, Gender, Department, Programme, Academic Year, Medical Requirements, Accessibility, and Administrative Preferences. | Hostel Management | The selected room must satisfy applicable allocation policy. | L2-G03 §17 |
| Hostel Management consumes room availability from G10; room ownership remains in G10. | Infrastructure Management | Hostel Management can allocate operational occupancy without owning the room or infrastructure. | L2-G03 §17, Appendix B |
| Check-In confirms operational occupancy and may validate Approved Admission, Valid Student Status, Room Allocation, Identity Verification, Required Documents, and Administrative Approval. | Hostel Management / Student Lifecycle | Occupancy activation requires these applicable business checks. | L2-G03 §19 |
| Every occupancy references exactly one active room allocation. | Hostel Management | Active residency must correspond to a single active room allocation. | L2-G03 §13 |
| Finance consumes occupancy information but does not modify residency state. | Finance / Hostel Management | Financial participation does not transfer ownership of occupancy. | L2-G03 §13, Appendix B |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Hostel Admission | Approved | Provides the entry eligibility for room allocation and check-in. | Approved remains the completed admission state. | G03 Hostel Management | L2-G03 §16, §30 |
| Room Allocation | Created / Allocated | Assigns the approved resident to an eligible hostel room. | Active | G03 Hostel Management | L2-G03 §17, §30 |
| Hostel Occupancy | Allocated / Checked-In | Check-in activates operational residency. | Active | G03 Hostel Management | L2-G03 §18, §19, §30 |

Student Context has no lifecycle transition in this scenario; Section C remains canonical owner of Student Lifecycle.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| RoomAllocated | G03 Hostel Management | Interested downstream consumers | Records the room allocation supporting residency activation. | L2-G03 §28 |
| OccupancyActivated | G03 Hostel Management | Student Operations, Finance, Security, Institutional Reporting, Parent Communication, Notifications as applicable | Communicates that the resident's hostel occupancy is active. | L2-G03 §28, §9 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Student Lifecycle (C) | Hostel Management (G) | Student Identity / student status context | Validate the canonical resident identity and applicable student information. | L2-G03 §9 |
| Infrastructure Management (G10) | Hostel Management (G) | Hostel room availability and infrastructure context | Supply the infrastructure-owned room context needed for operational allocation. | L2-G03 §9, §17 |
| Hostel Management (G) | Finance / Security / Institutional Reporting / Notifications | Occupancy business event | Make the authoritative active-residency fact available without transferring occupancy ownership. | L2-G03 §9, §27–§28 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Allocate room | Hostel Administrator | Room allocation is a Hostel Management responsibility governed by configurable allocation rules. | Hostel Management |
| Complete check-in / accept residency commencement | Hostel Administrator / applicable hostel authority | Check-in acceptance is governed by applicable hostel policy. | Hostel Management |

### 15.2 Separation of Duties, Overrides and Exceptions

L2-G03 supports configurable approval and delegation but does not define a specific separation-of-duty rule for room allocation and check-in. No additional SoD rule is invented.

Administrative override may be used where institutional policy permits it; the authoritative source does not define a dedicated override authority for this scenario.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Residency commencement must be auditable. | Immutable audit history for room allocation, check-in, occupancy activation, and associated business action/state change. | Hostel Management, with Section A audit infrastructure |

L2-G03 identifies Room Allocation, Check-In, and significant residency activity as auditable and preserves historical residency information.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The approved hostel admission is associated with an eligible room allocation.
- Required check-in verification is complete.
- Hostel Occupancy is in the `Active` state.
- The occupancy activation is recorded and the canonical `OccupancyActivated` event is published.
- The active occupancy references exactly one active room allocation.

### 16.2 Postconditions

- The student is an active hostel resident under the Hostel Management domain.
- The assigned room is occupied through the canonical Hostel Occupancy relationship.
- Subsequent resident operations, such as leave, visitor, room transfer, complaint, discipline, mess, and exit processing, may now operate within the active-residency boundary.
- Student Lifecycle remains the canonical owner of the student's identity and academic lifecycle.
- Infrastructure Management remains the canonical owner of the hostel room and physical infrastructure.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material open issue prevents the scenario from expressing the canonical business outcome.

### 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| The configured hostel allocation/approval policy identifies the applicable authority for room allocation and check-in acceptance. | L2-G03 defines configurable allocation and authorization and identifies Hostel Administrator / Warden roles. | Exact institutional authority sequencing would need the applicable configuration. |

### 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| How active occupancy is technically persisted or how downstream event delivery is implemented is deferred. | L5 | Implementation architecture is outside L3. |

