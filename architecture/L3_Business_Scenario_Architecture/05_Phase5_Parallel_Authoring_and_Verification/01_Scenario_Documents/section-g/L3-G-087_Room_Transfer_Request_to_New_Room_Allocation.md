# Room Transfer Request to New Room Allocation

**Scenario ID:** L3-G-087  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, C, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-087 |
| Canonical Title | Room Transfer Request to New Room Allocation |
| Primary Owner | Hostel Management |
| Participating Domains | G, C, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern a resident's approved request to move from the current hostel room to a new room, completing the new room allocation while preserving Hostel Management ownership of accommodation operations and the canonical student identity boundary.

## 3. Business Outcome

The resident's room-transfer request is governed and, when approved, the resident receives a new valid room allocation with the transfer completed and historically traceable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a room-transfer request is submitted for an active hostel resident.

### 4.2 End Boundary

The scenario ends when the approved transfer is completed and the new room allocation is established.

### 4.3 In Scope

- Room-transfer request.
- Review and approval under hostel policy.
- Validation of the proposed destination room.
- Completion of the transfer.
- Establishment of the new room allocation.
- Preservation of allocation and transfer history.
- Relevant cross-domain consumption of the authoritative outcome.

### 4.4 Out of Scope

- Creation or ownership of hostel rooms, which remains with G10.
- Student identity or student lifecycle ownership.
- Physical maintenance of rooms, which remains with G01.
- Technical allocation mechanisms, APIs, databases, UI, or infrastructure.

### 4.5 Related Scenarios

Hostel admission, occupancy activation, and later room exit are separate institutional scenarios.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Hostel Management (G03) | Owns Room Transfer and Room Allocation operations. |
| Student Lifecycle (C02) | Provides canonical student identity/context; does not own room allocation. |
| Notifications (H02) | Supports governed communication of the authoritative outcome where applicable. |
| Infrastructure Management (G10) | Owns the canonical physical hostel room; participation is through the infrastructure reference and availability context, not room ownership transfer. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Resident / Student | Initiates the room-transfer request and participates in the transfer process. |
| Hostel Administrator | Reviews and processes the transfer request. |
| Hostel Warden | Provides hostel governance/approval where configured by institutional policy. |

No additional actor or authority is introduced.

## 7. Preconditions and Entry Criteria

- The resident has an active hostel residency.
- The current room allocation is valid.
- A transfer request is permitted by the applicable hostel policy.
- A destination room can be considered using the authoritative infrastructure context.

## 8. Trigger

A room-transfer request is submitted for an active hostel resident.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | The resident submits a request to transfer from the current room. | Resident / G03 | A Room Transfer request exists. |
| 2 | Hostel Management reviews the request against the applicable transfer policy and the resident's current accommodation context. | Hostel Administrator / Warden | The request reaches an approval decision. |
| 3 | Hostel Management evaluates the proposed destination against the available hostel-room context owned by G10. | G03 with G10 context | A valid destination is identified where permitted. |
| 4 | The authorised hostel decision approves the transfer. | G03 | The transfer is permitted to proceed. |
| 5 | Hostel Management completes the transfer and establishes the new room allocation. | G03 | The resident's accommodation reflects the new room. |
| 6 | Hostel Management preserves the transfer and allocation history and makes the authoritative outcome available to applicable consumers. | G03 | The new room allocation is the current accommodation outcome. |

**Scenario End:** The flow stops when the new room allocation is established.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| 1 | The resident's request is approved but the proposed destination is not suitable under the applicable room-allocation policy. | Consider another permitted destination through the same transfer process. | Transfer may proceed only once a valid destination is established. | G03 |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Transfer request is not approved. | Resident remains in the existing accommodation position. | Close/reject the transfer according to the configured hostel process. | G03 |
| No valid destination room is available. | Transfer cannot complete. | Keep the resident's existing allocation unchanged and resolve through the applicable hostel allocation process. | G03 / G10 |
| Required business information about the resident is not valid. | Transfer cannot be authoritatively completed. | Resolve the student-context reference before proceeding. | C02 / G03 |

No technical retries, queues, rollback, or infrastructure failure handling are described.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Room Allocation owns room assignment, allocation date/status, transfer reference, and allocation history. | G03 | The transfer changes accommodation through the Hostel Management-owned allocation lifecycle. | L2-G03 §8 Aggregate Design |
| Room ownership remains with G10. | G10 | G03 cannot create or redefine the physical room. | L2-G03 §4 / L2-G10 |
| Hostel room/allocation policies are configuration driven. | G03 | Transfer approval and operational constraints follow institutional policy. | L2-G03 §6 |
| Student ownership remains in Section C. | C02 | Transfer does not change student identity ownership. | L2-G03 §4; L2-C02 |

## 13. State and Lifecycle Impact

| Aggregate | Lifecycle Impact | Owner |
|---|---|---|
| Room Transfer | Request is reviewed and, when authorised, completed. | G03 |
| Room Allocation | Current accommodation changes to the approved destination and transfer history is retained. | G03 |
| Physical Room | The referenced destination room remains a G10-owned infrastructure entity. | G10 |

The supplied authoritative extracts do not expose the exact canonical Room Transfer state names or transition identifiers. No new state names are invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G03 establishes immutable domain events for significant operational transitions. The authoritative material retrieved for this authoring pass does not expose the exact canonical event name for Room Transfer completion.

| Business Fact | Canonical Publisher | Consumers |
|---|---|---|
| Completed room-transfer / new allocation outcome | G03 Hostel Management | Applicable Student, Notification, reporting, and other authorised consumers |

No event name is fabricated.

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C02 | G03 | Canonical student identity/context | Identify the resident without transferring student ownership. |
| G10 | G03 | Canonical room/infrastructure context | Validate the physical destination without transferring infrastructure ownership. |
| G03 | H02 | Authoritative transfer outcome | Support governed notification where configured. |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

The room-transfer decision is made by the authorised Hostel Management authority under configured hostel policy.

### 15.2 Separation of Duty / Override

No Room Transfer-specific separation-of-duty rule is exposed in the authoritative material retrieved. No additional rule is invented.

### 15.3 Audit / Evidence

The transfer request, approval decision, completed allocation, and historical transfer/allocation record must remain auditable within Hostel Management.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- A valid active resident is identified.
- The transfer request is governed under hostel policy.
- A valid destination room is established where transfer is approved.
- The new Room Allocation is completed.
- Transfer and allocation history is retained.

### 16.2 Postconditions

- The resident has the approved new room allocation.
- The previous allocation remains historically traceable.
- Physical room ownership remains with G10.
- Student ownership remains with C02.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| Exact canonical Room Transfer event name and dedicated state names were not exposed in the authoritative material retrieved. | Exact event/state assertions cannot safely be made. | Confirm against the full G03 event/state catalogue before Architecture Approval/Lock. |

### 17.2 Assumptions

Institutional room-transfer policy determines the applicable approval authority and permitted transfer conditions.

### 17.3 Deferred Questions

Implementation mechanics for room allocation and notifications remain outside L3.
