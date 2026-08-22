# Reading Room Operational Usage to Session Closure

**Scenario ID:** L3-G-145  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, E, H  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete institutional business outcome in which permitted Reading Room usage proceeds under configured Reading Room rules, operational supervision is maintained, the individual reading session reaches its closure condition, and the session closure outcome is recorded.

The locked L2 defines:

`Reading Room Available → Operational Monitoring → Usage Supervision → Reading Session → Operational Closure`.

Reading Rooms support reference materials and quiet study. Detailed seat reservation is explicitly outside the current architectural scope. fileciteturn199file0

A critical boundary is preserved: **closing one individual reading session does not automatically put the Reading Room into Operational Closure**. The room may remain available for other permitted use.

## 3. Business Outcome

A permitted Reading Room usage session has reached its authoritative business closure condition and the session closure outcome has been recorded.

The completed outcome means:

1. The correct Reading Room is identified.
2. The Reading Room is operationally available.
3. Permitted usage occurs under applicable rules.
4. Operational monitoring/supervision is established.
5. The session reaches its business closure condition.
6. Session closure is recorded.
7. Relevant operational history is preserved.
8. H participates in governed communication where required.
9. The broader Reading Room lifecycle is not incorrectly advanced solely because the individual session ended.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A permitted Reading Room usage session is underway within an operationally available Reading Room.

### 4.2 End Boundary

The individual reading session reaches its business closure outcome.

### 4.3 In Scope

- Reading Room operational context.
- Permitted usage context.
- Applicable Reading Room rules.
- Operational monitoring.
- Usage supervision.
- Session closure.
- Operational history.
- E personnel-context participation where required.
- H communication participation where required.

### 4.4 Out of Scope

- Detailed seat reservation.
- Seat allocation.
- Membership changes.
- Student lifecycle.
- Employee lifecycle.
- Physical security.
- Authentication.
- technical access control/IAM.
- Device-control implementation.
- Infrastructure/deployment.
- Library-wide closure.
- New Reading Room lifecycle states.

## 5. Ownership and Participating Domains

### 5.1 Library Management — G

G is the Primary Owner and owns:

- Reading Room operations.
- Reading Room rules.
- Monitoring and supervision.
- Reading Room usage outcome.
- Operational history.

### 5.2 People & Workforce — E

E provides authoritative personnel context where required.

E does not own the Reading Room or session outcome.

### 5.3 Communication — H

H may participate where a governed operational notification is required.

H owns communication delivery, not the Reading Room/session lifecycle.

## 6. Business Actors, Roles and Responsibilities

### 6.1 Reading Room User

Uses the Reading Room for permitted on-premises study/reference activity.

The L2 establishes Reading Rooms as facilities supporting reference materials and quiet study. fileciteturn199file0

### 6.2 Library Operations Personnel

Monitor and supervise usage according to configured Library Management rules.

### 6.3 People & Workforce Authority

Provides authoritative personnel context where required.

### 6.4 Communication

Provides communication delivery where the applicable business outcome requires notification.

### 6.5 Actor Boundary

No seat-reservation actor, security officer, technical administrator, disciplinary officer, facilities engineer, or other unsupported actor is invented.

## 7. Preconditions and Entry Criteria

1. Reading Room is operationally available.
2. Usage is permitted under applicable Reading Room rules.
3. Operating period is established.
4. Monitoring/usage supervision is available.
5. Required E personnel context is available where applicable.
6. The session has a valid business basis under the Reading Room operating rules.

## 8. Trigger

A permitted Reading Room usage session reaches the business condition requiring closure.

The scenario does not invent a technical timeout or infrastructure trigger.

## 9. Main Business Flow

### Phase A — Establish Operational Context

1. Identify the Reading Room.
2. Confirm operational availability.
3. Establish the permitted usage context.
4. Establish the applicable Reading Room category and rules.
5. Establish operational monitoring and usage supervision.

### Phase B — Conduct Permitted Usage

6. The Reading Room user conducts permitted on-premises study/reference activity.
7. Library Operations Personnel supervise usage according to applicable rules.
8. Applicable Reading Room rules remain in effect.
9. Any operational issue is handled according to configured Library Management procedure.
10. The Reading Room remains under its broader operational lifecycle while the session is active.

### Phase C — Establish Session Closure

11. The applicable business closure condition is reached.
12. Library Operations Personnel establish that the individual session has ended.
13. The session is treated as closed for this scenario.
14. Relevant operational history is preserved.

### Phase D — Communication Where Required

15. Where a governed operational notification is required, the relevant business outcome is made available to H.
16. H performs communication delivery under its own responsibility.
17. Communication participation does not transfer ownership from G.

### Phase E — Complete

18. The individual reading session is closed.
19. The Reading Room remains in its applicable broader operational condition.
20. End the scenario.

## 10. Alternate Business Flows

### 10.1 Normal Session Completion

The user completes permitted study/reference use and the session closes normally.

### 10.2 Session Ends While Room Remains Available

The individual session closes while the Reading Room remains available. This is the expected distinction between session outcome and room lifecycle.

### 10.3 Room Reaches Operational Closure

If the broader Reading Room reaches Operational Closure while a session remains active, the room-level closure workflow governs the room outcome.

This scenario does not invent a separate session lifecycle transition.

### 10.4 Usage Rule Violation

Library Operations Personnel apply the applicable configured rule.

No unsupported fine, disciplinary action, membership suspension, or penalty is invented.

### 10.5 Operational Issue Requires Supervision

The session remains under operational supervision until resolved or closed according to the business procedure.

### 10.6 Communication Required

H receives the governed communication outcome and performs delivery under its responsibility.

### 10.7 Personnel Context Required

E provides authoritative personnel information. G remains owner of the session outcome.

## 11. Failure, Interruption and Business Recovery

### 11.1 Session Context Cannot Be Established

No unsupported session outcome is recorded.

### 11.2 Reading Room Availability Cannot Be Established

Usage is not asserted to have occurred within an available Reading Room without authoritative confirmation.

### 11.3 Rules Cannot Be Established

No invented usage rule is substituted.

### 11.4 Closure Condition Cannot Be Established

The session remains under applicable operational supervision until its closure condition is authoritative.

### 11.5 Operational Issue Remains Unresolved

The session remains under the configured operational procedure.

### 11.6 Notification Pending

H remains responsible for communication delivery. G's session outcome does not become Communication-owned.

### 11.7 Recovery Boundary

Once the relevant business condition becomes authoritative, the session can proceed to closure.

No technical retry, queue, rollback, server failure, API recovery, database transaction, infrastructure, or deployment behaviour is described.

## 12. Business Rules and Decision Constraints

Reading Room rules may include:

- Operating Hours.
- Silence Policy.
- Reference Material Usage.
- Food Restrictions.
- Device Usage.
- Entry Restrictions.

These remain configuration driven. fileciteturn199file0

Reading Room categories include General, Silent, Reference, Research and Digital Reading Rooms, with additional categories configurable. fileciteturn199file0

Constraints:

1. Usage is governed by applicable Reading Room rules.
2. Detailed seat reservation is outside scope.
3. No new Reading Session lifecycle is invented.
4. Session closure does not equal Reading Room Operational Closure.
5. No unsupported disciplinary or financial consequence is introduced.
6. H owns communication delivery where applicable.
7. E provides personnel context where applicable but does not own the session.

## 13. State and Lifecycle Impact

The authoritative Reading Room workflow is:

`Reading Room Available → Operational Monitoring → Usage Supervision → Reading Session → Operational Closure`. fileciteturn199file0

This scenario operates within the **Reading Session** stage.

The L2 does not define a separate lifecycle for an individual reading session. Therefore this scenario does not invent:

- `Session Started`.
- `Session Active`.
- `Session Closed`.
- `Session Completed`.
- `User Departed`.

The session closure is expressed as a business outcome while preserving the authoritative Reading Room lifecycle.

### Critical Boundary

**Ending one reading session does not automatically transition the Reading Room to Operational Closure.**

The Reading Room may remain operationally available for another permitted session.

## 14. Business Events and Cross-Domain Collaboration

The retrieved locked L2 does not expose dedicated canonical event names for Reading Session start or closure.

Therefore this draft does not invent:

- `ReadingSessionStarted`.
- `ReadingSessionClosed`.
- `ReadingUsageCompleted`.

If such events are mandatory, their names and publishers must come from the authoritative event register.

### G — Library Management

Owns the session/usage business outcome.

### E — People & Workforce

Provides authoritative personnel context where applicable.

### H — Communication

Performs governed communication delivery where required.

Cross-domain participation does not transfer ownership away from G.

## 15. Business Authorization, Approval and Audit Obligations

Reading Room usage is governed by configured Library Management rules.

Relevant evidence should remain auditable:

- Reading Room identity.
- Operating period.
- Category.
- Applicable rules.
- Operational supervision outcome where required.
- Session closure outcome.
- Relevant personnel context where applicable.
- Communication outcome where required.

The L2 requires immutable audit history and states that lifecycle transitions shall be audited. fileciteturn196file0turn197file0

No technical authentication, RBAC/IAM, physical security, device-control, or audit-storage implementation is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Reading Room identified.
2. Operational availability confirmed.
3. Usage context established.
4. Applicable rules established.
5. Monitoring/supervision established.
6. Session reaches authoritative closure condition.
7. Session closure recorded.
8. Operational history preserved.
9. Required H communication participation handed off where applicable.
10. Reading Room broader lifecycle remains correctly represented.

### Postconditions

- Individual reading session is closed.
- Session outcome is preserved.
- Reading Room may remain Available.
- Reading Room Operational Closure is not implied solely by session closure.
- No seat reservation is created.
- No membership lifecycle change occurs.
- No employee lifecycle change occurs.
- H owns communication delivery where applicable.
- G remains canonical owner.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The retrieved L2 does not define a dedicated Reading Session aggregate lifecycle.
2. It does not expose dedicated canonical Reading Session start/closure events.
3. It does not provide a detailed session closure decision matrix.

### Assumptions

1. Reading Session is a workflow stage within Reading Room Management rather than a separately defined lifecycle aggregate.
2. Session closure is an operational business outcome.
3. G remains canonical owner.
4. E participation is limited to authoritative personnel context.
5. H participation is limited to communication delivery.

### Deferred Questions

Before Architecture Approval/Lock, confirm whether the final event register defines canonical Reading Session events and whether any upstream rule establishes a mandatory session closure condition.

No local event names, lifecycle states, or rules are invented to close these gaps.
