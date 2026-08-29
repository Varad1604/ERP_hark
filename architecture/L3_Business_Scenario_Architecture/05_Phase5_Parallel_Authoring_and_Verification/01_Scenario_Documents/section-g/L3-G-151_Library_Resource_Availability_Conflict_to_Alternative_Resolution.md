# Library Resource Availability Conflict to Alternative Resolution

**Scenario ID:** L3-G-151  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
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
| Final L3 ID | L3-G-151 |
| Canonical Title | Library Resource Availability Conflict to Alternative Resolution |
| Primary Owner | Library Management |
| Participating Domains | G, C, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional outcome when a requested library resource cannot be supplied in the expected availability condition and Library Management establishes an authorised alternative resolution.

The locked L2 defines reservation management for unavailable resources and a reservation workflow of `Reservation Requested → Waiting List → Copy Available → Member Notified → Collection → Reservation Closed`. It also states that resource availability is a configurable validation consideration and that reservation policies remain configurable.

Because the frozen scenario title is broader than a single reservation state, this L3 treats the conflict as the business condition in which the expected resource availability cannot satisfy the request and the institution must establish the applicable alternative outcome, without inventing a new lifecycle state.

The scenario applies the locked L2-G05 ownership and workflow boundaries. Library Management remains the canonical operational owner of library services, while external domains retain ownership of their own information and downstream outcomes.

## 3. Business Outcome

The requesting party receives an authoritative library outcome for the availability conflict: either an applicable alternative resource/resolution is established, or the request is placed into the authoritative reservation/waiting-list path.

The scenario does not invent a universal substitute-resource rule, priority rule, or compensation policy.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario starts when the expected library resource cannot satisfy the active request because the applicable availability condition is not met.

### 4.2 End Boundary
The scenario ends when the availability conflict has an authoritative alternative resolution, or the request has been placed into the existing reservation/waiting-list workflow as the applicable resolution boundary.

### 4.3 In Scope
- Resource availability evaluation.
- Identification of the conflict.
- Review of reservation/waiting-list eligibility.
- Alternative resolution where supported by institutional policy.
- Reservation creation/waiting-list placement where applicable.
- Member notification where governed.
- Preservation of the operational outcome.

### 4.4 Out of Scope
- Technical APIs, databases, Business Data Structures, queues, retries, rollback, infrastructure and deployment.
- UI behaviour.
- technical access control/IAM.
- Any lifecycle, rule, event or ownership not defined by authoritative upstream architecture.
- Unrelated downstream institutional outcomes.

## 5. Ownership and Participating Domains

### Library Management — G
G is the canonical owner of the library business outcome and lifecycle described by this scenario.

### Participating Domains
G, C, H

Participation by another domain supplies/consumes information within its own authority; it does not transfer ownership from G.

## 6. Business Actors, Roles and Responsibilities

- Library operational personnel establish the resource availability outcome.
- Student/member context is consumed from C where required.
- Communication H handles governed notification delivery.
- G remains owner of reservation/resource availability decisions.

Only source-supported business responsibilities are used. No technical role is introduced.

## 7. Preconditions and Entry Criteria

1. A valid library resource request exists.
2. The requested resource can be identified.
3. The authoritative library copy/resource availability context is available.
4. Applicable membership and borrowing/reservation context can be evaluated.
5. The request has not already reached a final collection/closure outcome.

## 8. Trigger

The requested resource is unavailable or otherwise cannot satisfy the expected availability condition.

## 9. Main Business Flow

### Phase A — Establish the Conflict
1. Identify the requested resource and active request.
2. Establish the authoritative availability condition.
3. Determine why the expected availability is not currently satisfied.
4. Preserve the availability conflict in the business context.

### Phase B — Evaluate Existing Resolution Paths
5. Check applicable membership/borrowing context.
6. Check resource availability.
7. Check existing reservation context.
8. Check reservation limits and duplicate-reservation constraints where applicable.
9. Apply configurable reservation policies.

### Phase C — Establish Resolution
10. If an authorised alternative resource/resolution exists, establish it.
11. Otherwise place the request into the existing reservation/waiting-list path where eligible.
12. Preserve queue position, reservation date, priority and notification status where the authoritative reservation workflow requires them.

### Phase D — Communication
13. Where the outcome requires notification, provide the business outcome to H.
14. H performs notification delivery.
15. G retains ownership of the library business outcome.

### Phase E — Complete
16. End when the conflict has an authoritative alternative resolution or the request has entered the applicable existing reservation/waiting-list boundary.

## 10. Alternate Business Flows

### 10.1 Resource Becomes Available
The request follows the existing reservation workflow toward `Copy Available`, notification, collection and closure where applicable.

### 10.2 Waiting-List Placement
Where the resource remains unavailable and the request is eligible, the request progresses to the authoritative waiting-list path.

### 10.3 Alternative Resource
If institutional policy supports an alternative resource, G establishes that alternative. No automatic equivalence rule is invented.

### 10.4 Duplicate Reservation
The L2 identifies duplicate reservations as a validation consideration. The request follows the configured policy rather than an invented local outcome.

### 10.5 Notification
H handles delivery of the governed member notification.

## 11. Failure, Interruption and Business Recovery

### 11.1 Resource Identity Cannot Be Established
No alternative is assigned to an invented resource.

### 11.2 Availability Cannot Be Determined
The scenario does not falsely assert availability.

### 11.3 Reservation Eligibility Cannot Be Established
No waiting-list placement is asserted until the applicable configurable validation is satisfied.

### 11.4 No Alternative Is Authorised
The request remains within the applicable existing reservation/waiting-list boundary.

### 11.5 Downstream Notification Pending
H remains responsible for notification delivery; the library outcome itself is not redefined.

### 11.6 Recovery
Once authoritative availability/eligibility information is established, the business flow continues from the unresolved boundary.

No technical retry, queue, server failure, rollback, database recovery, API or infrastructure mechanics are specified.

## 12. Business Rules and Decision Constraints

The L2 states that reservation validation may consider Active Membership, Existing Borrowings, Reservation Limits, Resource Availability and Duplicate Reservations, with configurable policies.

The L2 reservation workflow is `Reservation Requested → Waiting List → Copy Available → Member Notified → Collection → Reservation Closed`.

The scenario must not invent a universal substitute-resource policy or force every availability conflict into a reservation.

Rules remain upstream-owned and configurable where L2 states that configuration is permitted. No local rule is invented.

## 13. State and Lifecycle Impact

The authoritative reservation statuses are Requested, Waiting, Ready for Collection, Collected, Expired and Cancelled; values remain configurable.

This scenario does not create a new `Availability Conflict` lifecycle state. The conflict is a business condition resolved through an existing authoritative path.

No new lifecycle states or transitions are created.

## 14. Business Events and Cross-Domain Collaboration

The L2 defines reservation events including `LibraryReservationCreated`, `ReservationAvailable`, `ReservationCollected` and `LibraryReservationExpired`.

Where the alternative resolution enters the reservation lifecycle, only these canonical events are used. No `AlternativeResourceAssigned` or `AvailabilityConflictResolved` event is invented.

H handles notification delivery; G publishes/owns the business outcome.

Only canonical events actually defined by L2 are named. Cross-domain collaboration does not transfer the canonical publisher/owner.

## 15. Business Authorization, Approval and Audit Obligations

The resolution must be established under applicable Library Management policy. Relevant audit evidence includes resource identity, request context, availability determination, reservation eligibility, selected resolution, queue/reservation information where applicable, and notification outcome.

No technical authorization model is introduced.

The scenario preserves auditable business history. Technical authorization mechanisms are outside L3.

## 16. Completion Criteria and Postconditions

1. Availability conflict identified.
2. Applicable eligibility/policy evaluated.
3. Alternative resolution or existing reservation/waiting-list path established.
4. Canonical reservation lifecycle preserved where used.
5. Required notification handed to H.
6. Business history retained.
7. G ownership preserved.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues
1. The L2 does not define a dedicated availability-conflict lifecycle or event.
2. It does not define a universal alternative-resource selection policy.

### Assumptions
- Existing reservation policy governs unavailable resources.
- Alternative resolution is used only where an authoritative policy supports it.

### Deferred Questions
Confirm any institution-wide alternative-resource policy before Architecture Approval/Lock.

Unresolved upstream matters are recorded rather than fabricated.
