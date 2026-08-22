# Reading Room Closure to Restricted Library Usage

**Scenario ID:** L3-G-144  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
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
| Final L3 ID | L3-G-144 |
| Canonical Title | Reading Room Closure to Restricted Library Usage |
| Primary Owner | Library Management |
| Participating Domains | G |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario defines the complete institutional outcome by which an operating Reading Room reaches its authorised closure condition, ordinary Reading Room usage is no longer available for the operating period, and the closure outcome is recorded.

The locked L2 defines the Reading Room workflow as:

`Reading Room Available → Operational Monitoring → Usage Supervision → Reading Session → Operational Closure`.

The authoritative endpoint for this scenario is therefore **Operational Closure**. The scenario does not invent a separate `Closed` or `Restricted` lifecycle state.

## 3. Business Outcome

The designated Reading Room has reached its authoritative Operational Closure condition.

The completed outcome means:

1. The correct Reading Room has been identified.
2. The applicable operating period and closure condition are established.
3. Relevant operational monitoring/supervision has been completed to the extent required by the business process.
4. Ordinary Reading Room availability ends for that operating period.
5. Operational Closure is established.
6. Closure history is preserved and auditable.
7. A future authorised operating period remains a separate business occurrence.

This is not a library-wide closure and does not change membership status.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins with a Reading Room in an active operating condition that reaches its authorised closure condition.

### 4.2 End Boundary

The scenario ends when **Operational Closure** is established.

### 4.3 In Scope

- Reading Room identification.
- Operating period.
- Closing condition.
- Applicable Reading Room rules.
- Operational monitoring/supervision relevant to closure.
- Establishment of Operational Closure.
- Restriction of ordinary use.
- Preservation of closure history.

### 4.4 Out of Scope

- Library-wide closure.
- Membership suspension/closure.
- Seat reservation.
- Individual disciplinary processing.
- Physical security implementation.
- Authentication.
- technical access control/IAM.
- Infrastructure/deployment.
- Technical workflow execution.
- Subsequent Reading Room opening.
- New Reading Room lifecycle states.

## 5. Ownership and Participating Domains

### 5.1 Library Management — G

G is the sole Primary Owner and owns:

- Reading Room operations.
- Operating rules.
- Monitoring and supervision.
- Operational Closure.
- Closure history.

No cross-domain ownership transfer occurs.

## 6. Business Actors, Roles and Responsibilities

### 6.1 Library Operational Authority

Establishes closure under configured Library Management rules.

### 6.2 Library Operations Personnel

Complete applicable monitoring and supervision and support the operational closure outcome.

### 6.3 Actor Boundary

The scenario does not invent security officers, facilities staff, technical administrators, seat-allocation personnel, or other unsupported roles.

## 7. Preconditions and Entry Criteria

1. The Reading Room is operationally available or in an active operating period.
2. The applicable operating period is established.
3. The closure condition can be established.
4. Applicable Reading Room rules are available.
5. Required operational supervision is available.
6. Library operational authority is available.

## 8. Trigger

The Reading Room reaches its authorised operational closure condition.

The trigger is a business operating condition, not a technical scheduler or infrastructure event.

## 9. Main Business Flow

### Phase A — Establish Closure Context

1. Identify the Reading Room.
2. Establish the applicable operating period.
3. Establish the configured closing condition.
4. Establish applicable Reading Room rules.
5. Confirm that the room is in the operating condition from which closure is being established.

### Phase B — Complete Relevant Supervision

6. Complete applicable operational monitoring.
7. Complete applicable usage supervision.
8. Consider active Reading Room usage relevant to the closure decision.
9. Establish that the business condition for Operational Closure is satisfied.

### Phase C — Establish Operational Closure

10. Establish **Operational Closure**.
11. End ordinary Reading Room availability for the operating period.
12. Preserve the closure decision.
13. Preserve relevant operational history.

### Phase D — Complete

14. End the scenario.

The next opening of the Reading Room is a separate business occurrence.

## 10. Alternate Business Flows

### 10.1 Normal Scheduled Closure

When the configured operating period ends, the Reading Room progresses to Operational Closure.

### 10.2 Authorised Early Closure

An authoritative operational decision may establish an earlier closure.

The scenario does not invent the authority or reason for early closure.

### 10.3 Active Usage Requires Supervision

Where usage remains active at the closure boundary, applicable operational supervision is completed before closure is treated as complete.

No seat-level workflow is invented.

### 10.4 Temporary Operational Restriction

An authoritative restriction governs the room's business condition. No new `Restricted` lifecycle state is created.

### 10.5 Closure Condition Unclear

The Reading Room remains in its current authoritative condition and is not falsely marked closed.

### 10.6 Already in Operational Closure

A duplicate closure outcome is not created.

## 11. Failure, Interruption and Business Recovery

### 11.1 Closing Condition Unavailable

No default closing condition is applied.

### 11.2 Operating Rules Unavailable

Closure is not asserted contrary to authoritative operating rules.

### 11.3 Usage Condition Unclear

The Reading Room remains under operational supervision until the business condition is authoritative.

### 11.4 Authoritative Decision Required

No substitute authority is invented.

### 11.5 Recovery Boundary

Once the business condition becomes authoritative, the Reading Room may proceed to Operational Closure.

No technical retry, queue, rollback, server failure, API recovery, database transaction, infrastructure or deployment behaviour is described.

## 12. Business Rules and Decision Constraints

The L2 identifies Reading Room rules including:

- Operating Hours.
- Silence Policy.
- Reference Material Usage.
- Food Restrictions.
- Device Usage.
- Entry Restrictions.

Rules remain configuration driven.

The authoritative workflow is:

`Reading Room Available → Operational Monitoring → Usage Supervision → Reading Session → Operational Closure`.

Constraints:

1. Closure follows the authoritative operating condition.
2. No new closure lifecycle state is invented.
3. No library-wide closure is implied.
4. No membership change is implied.
5. No seat reservation is introduced.
6. Physical security and technical access control remain outside scope.

## 13. State and Lifecycle Impact

The authoritative endpoint is **Operational Closure**.

This scenario does not invent:

- `Closing`.
- `Closing Pending`.
- `Closed`.
- `Restricted`.
- `Locked`.

“Restricted library usage” is the business consequence of Operational Closure, not a new lifecycle state.

A later operating period can establish availability through a separate scenario.

## 14. Business Events and Cross-Domain Collaboration

The retrieved locked L2 does not expose a dedicated canonical Reading Room closure event.

Therefore this scenario does not invent:

- `ReadingRoomClosed`.
- `ReadingRoomClosureCompleted`.
- `ReadingRoomRestricted`.

If a canonical event is mandatory, its name and publisher must come from the authoritative event register.

This scenario is G-owned and requires no participating domain to own any part of the Reading Room lifecycle.

## 15. Business Authorization, Approval and Audit Obligations

Closure follows configured Library Management operational authority.

The following evidence must remain auditable:

- Reading Room identity.
- Operating period.
- Closing condition.
- Relevant operational decision.
- Operational Closure outcome.
- Relevant monitoring/supervision history.

The L2 requires immutable audit history and states that lifecycle transitions shall be audited.

No technical authorization model is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Correct Reading Room identified.
2. Operating period established.
3. Closure condition authoritative.
4. Required operational supervision completed.
5. Operational Closure established.
6. Ordinary availability ended for the period.
7. Closure history preserved.

### Postconditions

- Reading Room is in Operational Closure.
- Ordinary use is unavailable for the operating period.
- Closure history is auditable.
- Library-wide services are unaffected.
- Membership status is unchanged.
- A future opening remains a separate business scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. No dedicated canonical Reading Room closure event is exposed in the retrieved L2.
2. The L2 does not define a detailed closure checklist beyond workflow and configurable rules.
3. The L2 does not define a separate lifecycle state for restricted usage.

### Assumptions

1. Operating hours are configuration driven.
2. Operational Closure is the authoritative endpoint.
3. G remains the sole owner.

### Deferred Questions

Confirm whether the final event register defines a canonical Reading Room closure event and whether any additional upstream closure obligations are mandatory.

No local resolution is invented.
