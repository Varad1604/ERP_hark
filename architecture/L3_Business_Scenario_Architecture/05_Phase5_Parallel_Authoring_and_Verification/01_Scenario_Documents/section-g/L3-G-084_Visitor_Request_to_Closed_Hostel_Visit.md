# Visitor Request to Closed Hostel Visit

**Scenario ID:** L3-G-084  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-084 |
| Canonical Title | Visitor Request to Closed Hostel Visit |
| Primary Owner | Hostel Management |
| Participating Domains | G |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern a hostel visitor from request and registration through authorised entry, visit completion, and closure while preserving Hostel Management ownership of visitor records and resident relationships.

## 3. Business Outcome

The visitor request is governed, the authorised visitor is admitted for the permitted visit, the visitor exits the hostel, and the visit is closed with a complete historical record.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a visitor request is submitted for an active hostel resident.

### 4.2 End Boundary

The scenario ends when the authorised visitor has exited and the Visitor Entry lifecycle is closed.

### 4.3 In Scope

- Visitor registration/request.
- Association to an active resident.
- Visitor approval under configured hostel visitor policy.
- Visitor entry/check-in.
- Visit monitoring as required by hostel policy.
- Visitor exit/check-out.
- Closure and historical retention of the visit.

### 4.4 Out of Scope

- Resident admission or occupancy lifecycle.
- Security-domain ownership of security operations.
- Disciplinary processing resulting from an unauthorised visit.
- Technical identity verification, notification delivery, APIs, databases, or infrastructure.

### 4.5 Related Scenarios

- Hostel residency and occupancy scenarios remain separate.
- A later discipline scenario is separate if visitor conduct creates a disciplinary incident.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Hostel Management (G) | Owns Visitor Entry, visitor registration, approval, entry, exit, and visit history. |
| Security / other consumers | May consume visitor information, but do not own Visitor Entry. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Resident / Student | Requests or sponsors the visitor and participates in the visit process. |
| Hostel Administrator | Performs operational visitor processing and review. |
| Hostel Warden | Provides hostel governance/approval where configured by institutional visitor policy. |
| Visitor | Presents for the authorised visit and completes entry/exit requirements. |

No additional authority is invented beyond the G03 operational model and configurable hostel policy.

## 7. Preconditions and Entry Criteria

- The sponsoring resident has an active hostel residency.
- The visitor request identifies the resident and visitor.
- The request falls within the configured hostel visitor policy.
- The visitor process is available for hostel operational review.

## 8. Trigger

A visitor request is submitted for an active hostel resident.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Result |
|---|---|---|---|
| 1 | The visitor is registered against the active resident and the intended visit. | Resident / Hostel Management | Visitor Entry enters `Registered`. |
| 2 | Hostel Management reviews the request against configured visitor rules and approval requirements. | Hostel Administrator / Hostel Warden | The request reaches an approval decision. |
| 3 | The visit is authorised where the configured hostel policy permits it. | Hostel Management | Visitor Entry reaches `Approved`. |
| 4 | The visitor is admitted and the entry is recorded. | Hostel Management | Visitor Entry reaches `Checked-In`. |
| 5 | The visit occurs within the authorised hostel conditions. | Visitor / Resident / Hostel Management | The authorised visit remains operationally governed. |
| 6 | The visitor exits and the exit is recorded. | Hostel Management / Visitor | Visitor Entry reaches `Checked-Out`. |
| 7 | Hostel Management closes the visit record and retains the visit history. | Hostel Management | Visitor Entry reaches `Closed`. |

**Scenario End:** The flow stops at closed visit. Any later complaint, security, or disciplinary outcome is a separate business scenario.

## 10. Alternate Business Flows

| Condition | Business Path | Outcome |
|---|---|---|
| Visitor request does not satisfy configured visitor policy. | Do not authorise the visit. | No authorised visit is created. |
| Required hostel approval is not granted. | Keep the request from entering authorised entry. | Visitor does not proceed through the authorised visit lifecycle. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution | Owner |
|---|---|---|---|
| Visitor information is incomplete or inconsistent. | Registration cannot be concluded authoritatively. | Correct the visitor request before approval. | Hostel Management |
| Approval is unavailable or not granted. | Visitor cannot be admitted as an authorised visitor. | Resolve through the configured visitor approval path. | Hostel Management |
| Visitor does not complete the expected exit process. | Visit cannot be normally closed. | Record the applicable business outcome under hostel visitor policy; do not falsely mark the visit closed. | Hostel Management |

No technical retry, queue, rollback, or implementation recovery is described.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect | Source |
|---|---|---|---|
| Every visitor shall reference an active resident. | Hostel Management | Visitor request requires a valid resident relationship. | L2-G03 Business Invariants |
| Visitor rules are configuration driven. | Hostel Management | Institutional visitor policy controls approval and operational conditions. | L2-G03 Design Principles / Configuration |
| Visitor Entry owns registration, approval, entry, exit, and visit history. | Hostel Management | No other domain owns visitor lifecycle. | L2-G03 Aggregate Design |
| Significant hostel transitions are auditable and event driven. | Hostel Management | Visit history remains traceable. | L2-G03 Design Principles |

## 13. State and Lifecycle Impact

| Aggregate | Entry State | Transition | Result | Owner |
|---|---|---|---|---|
| Visitor Entry | Registered | Visitor approval | Approved | G03 |
| Visitor Entry | Approved | Visitor entry | Checked-In | G03 |
| Visitor Entry | Checked-In | Visitor exit | Checked-Out | G03 |
| Visitor Entry | Checked-Out | Visit closure | Closed | G03 |

These states are the canonical Visitor lifecycle defined by L2-G03. No additional state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

L2-G03 establishes immutable domain events for significant hostel operational transitions. The supplied authoritative extract does not expose the exact canonical event names for each Visitor Entry transition, so no event name is invented here.

| Business Fact | Canonical Publisher | Interested Consumers |
|---|---|---|
| Visitor approval / entry / exit / closure outcome | G03 Hostel Management | Approved downstream consumers, including Security and Notifications where applicable |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Contract / Hand-off | Purpose |
|---|---|---|---|
| Hostel Management | Security | Visitor information / hostel visitor outcome | Support security consumption without transferring Visitor Entry ownership. |
| Hostel Management | Notifications | Authoritative visitor outcome/event | Support governed communications where configured. |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

| Decision | Authority | Constraint |
|---|---|---|
| Submit visitor request | Resident / configured visitor source | Must relate to an active resident. |
| Approve visitor | Hostel Administrator / Hostel Warden as configured | Must satisfy institutional visitor policy. |

### 15.2 Separation of Duties / Overrides

No dedicated visitor-specific separation-of-duty rule is defined in the supplied L2 source. No additional rule is invented.

### 15.3 Audit Evidence

The complete visitor registration, approval, entry, exit, and closure history must remain auditable within Hostel Management.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Visitor request is associated with an active resident.
- Approval is completed where required.
- Visitor entry is recorded.
- Visitor exit is recorded.
- Visitor Entry reaches `Closed`.
- Visit history remains retained.

### 16.2 Postconditions

- The authorised hostel visit is closed.
- No active visitor entry remains for the completed visit.
- Resident occupancy ownership is unchanged.
- Any subsequent security or disciplinary outcome is handled separately.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| Exact canonical event names for Visitor Entry transitions are not exposed in the authoritative material available for this draft. | Naming a specific event would risk inventing architecture. | Confirm canonical event names before Architecture Approval/Lock. |

### 17.2 Assumptions

Visitor approval authority is governed by the institution's configured hostel visitor policy.

### 17.3 Deferred Questions

Technical visitor verification and notification-delivery mechanics remain outside L3.
