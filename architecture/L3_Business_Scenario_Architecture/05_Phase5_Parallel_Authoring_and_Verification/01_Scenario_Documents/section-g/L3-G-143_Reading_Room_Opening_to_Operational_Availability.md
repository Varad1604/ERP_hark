# Reading Room Opening to Operational Availability

**Scenario ID:** L3-G-143  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, E  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario defines the complete institutional business outcome by which a configured institutional Reading Room reaches its authorised operating start condition, its applicable operational context is established, and the Reading Room becomes operationally available for permitted on-premises study and reference use.

The locked L2 defines Reading Room Management as operational management of institutional reading areas supporting on-premises study. Reading Rooms support reference materials and quiet study. Its authoritative workflow is:

`Reading Room Available → Operational Monitoring → Usage Supervision → Reading Session → Operational Closure`.

Detailed seat reservation is explicitly outside the current architectural scope. fileciteturn199file0

This L3 therefore establishes the **Reading Room Available** business outcome and stops there. It does not expand into reading-session processing, seat reservation, physical security, authentication, or technical access control.

## 3. Business Outcome

A designated institutional Reading Room is established as operationally available for its authorised operating period.

The completed outcome means:

1. The correct Reading Room has been identified.
2. Its configured category and operating context are established.
3. Its authorised operating period is established.
4. Applicable Reading Room operating rules are established.
5. The Reading Room is established as available for permitted institutional use.
6. The availability outcome is preserved in Library Management history.
7. The Reading Room is ready for the subsequent Operational Monitoring stage.

No seat reservation, user identity lifecycle, physical access-control decision, or individual reading session is created by this scenario.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when a configured Reading Room reaches the business condition under which it may commence its authorised operating period.

Creation of the Reading Room as an institutional facility is outside this scenario.

### 4.2 End Boundary

The scenario ends when the Reading Room is established as **Reading Room Available**.

It must not continue into Operational Monitoring, Usage Supervision, Reading Session, or Operational Closure except to establish those as downstream boundaries.

### 4.3 In Scope

- Reading Room identification.
- Configured Reading Room category.
- Authorised operating period.
- Applicable Reading Room rules.
- Business validation of availability.
- Establishment of Reading Room Available.
- Preservation of operational history.
- Preparation for downstream monitoring.
- E participation where authoritative personnel context is required.

### 4.4 Out of Scope

- Detailed seat reservation or allocation.
- Membership registration/change.
- Student lifecycle.
- Employee lifecycle.
- Physical security implementation.
- Authentication.
- technical access control/IAM.
- Device access control.
- Infrastructure/deployment.
- Technical workflow execution.
- UI behaviour.
- Reading-session processing after availability.
- Operational closure.
- Library-wide opening/closure.

## 5. Ownership and Participating Domains

### 5.1 Library Management — G

G is the Primary Owner and owns:

- Reading Room operational management.
- Reading Room availability.
- Reading Room operational rules.
- Monitoring and supervision.
- Reading Room operational history.

The L2 places Reading Room Operations within Library Management. fileciteturn196file0

### 5.2 People & Workforce — E

E participates only where authoritative personnel context is required for the operational responsibility.

E does not own Reading Room availability, its lifecycle, rules, or history.

Participation does not transfer ownership from G.

## 6. Business Actors, Roles and Responsibilities

### 6.1 Library Operational Authority

Establishes Reading Room operational availability under applicable Library Management policy.

### 6.2 Library Operations Personnel

Perform monitoring and supervision after the Reading Room becomes available.

### 6.3 People & Workforce Authority

Provides authoritative personnel context where required.

### 6.4 Actor Boundary

This scenario does not invent security officers, facilities staff, technical administrators, seat-allocation staff, or other unsupported roles.

## 7. Preconditions and Entry Criteria

1. The Reading Room exists as a configured institutional Reading Room.
2. Its applicable category is established.
3. Its operating period is established.
4. Applicable Reading Room rules are available.
5. The Reading Room is not already in an authoritative closed condition.
6. Required Library Management authority is available.
7. Required personnel context is available from E where applicable.

No default operating hours or locally invented rule is introduced.

## 8. Trigger

The Reading Room reaches the authorised business condition to commence operation.

This is a business operating condition, not a technical scheduler, server, or infrastructure event.

## 9. Main Business Flow

### Phase A — Establish Operating Context

1. Identify the Reading Room.
2. Establish its configured category.
3. Establish the applicable operating period.
4. Establish applicable Reading Room rules.
5. Confirm eligibility to become available.

### Phase B — Validate Availability

6. Consider the Reading Room's current operational condition.
7. Consider any authoritative operational restriction.
8. Where required, confirm personnel context through E.
9. Establish that the business conditions for availability are satisfied.

### Phase C — Establish Reading Room Available

10. Establish **Reading Room Available**.
11. Make applicable rules effective for the operating period.
12. Make the Reading Room available for permitted institutional use.
13. Preserve the availability outcome in operational history.

### Phase D — Boundary Completion

14. Establish readiness for the next authoritative workflow stage, Operational Monitoring.
15. End the scenario.

The scenario does not itself perform downstream monitoring or usage supervision.

## 10. Alternate Business Flows

### 10.1 Outside Configured Operating Hours

The Reading Room is not made available outside the authorised operating period.

### 10.2 Temporary Operational Restriction

An authoritative restriction prevents ordinary availability. No new `Temporarily Restricted` state is invented.

### 10.3 Required Operating Context Missing

If operating hours or required rules cannot be established, availability is not falsely asserted.

### 10.4 Personnel Context Required

E supplies authoritative personnel context. G remains the owner of the Reading Room outcome.

### 10.5 Category-Specific Rules

Configured category-specific rules are applied without inventing additional category behaviour.

### 10.6 Already Available

If the Reading Room is already in the authoritative available condition, a duplicate opening outcome is not created.

## 11. Failure, Interruption and Business Recovery

### 11.1 Operating Period Cannot Be Established

No invented operating period is applied. The business outcome remains unresolved.

### 11.2 Applicable Rules Cannot Be Established

The Reading Room does not become available based on an assumed policy.

### 11.3 Operational Restriction Exists

The restriction remains authoritative and availability is not asserted contrary to it.

### 11.4 Personnel Information Unavailable

Where personnel context is mandatory, no substitute person or authority is invented.

### 11.5 Recovery Boundary

Once required business conditions become authoritative, the Reading Room may proceed to Reading Room Available.

No technical retries, queues, rollback, server failures, API recovery, database recovery, or infrastructure mechanics are specified.

## 12. Business Rules and Decision Constraints

The L2 states that Reading Room rules may include:

- Operating Hours.
- Silence Policy.
- Reference Material Usage.
- Food Restrictions.
- Device Usage.
- Entry Restrictions.

Rules remain configuration driven. fileciteturn199file0

Reading Room categories include General, Silent, Reference, Research and Digital Reading Rooms, with additional categories configurable. fileciteturn199file0

Mandatory constraints:

1. Do not invent operating hours.
2. Do not invent category-specific rules.
3. Do not create a seat reservation.
4. Do not create a new Reading Room lifecycle state.
5. Do not transfer ownership to E.
6. Do not describe technical access control.
7. Do not cross the Reading Room Available boundary.

## 13. State and Lifecycle Impact

The authoritative Reading Room workflow is:

`Reading Room Available → Operational Monitoring → Usage Supervision → Reading Session → Operational Closure`. fileciteturn199file0

This scenario establishes **Reading Room Available**.

It does not invent `Opening`, `Opening Pending`, `Ready`, `Available Pending`, `Restricted`, or `Session Active` states.

The next business boundary is Operational Monitoring.

## 14. Business Events and Cross-Domain Collaboration

The retrieved locked L2 establishes the workflow but does not expose a dedicated canonical event name for Reading Room opening/availability.

Therefore this scenario does not invent `ReadingRoomOpened`, `ReadingRoomAvailable`, or `ReadingRoomOpeningCompleted`.

If a canonical event is mandatory, its name and publisher must come from the authoritative event register.

E provides personnel context where applicable. G remains the canonical owner of the Reading Room outcome.

## 15. Business Authorization, Approval and Audit Obligations

Opening/availability follows configured Library Management operational authority.

The following evidence must remain auditable:

- Reading Room identity.
- Operating period.
- Category.
- Applicable operating rules.
- Availability decision.
- Relevant personnel context where applicable.
- Operational history.

The L2 requires immutable audit history and states that lifecycle transitions shall be audited. fileciteturn196file0turn197file0

No technical access control/IAM or authentication implementation is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Correct Reading Room identified.
2. Category established.
3. Operating period established.
4. Applicable rules established.
5. No authoritative restriction prevents availability.
6. Reading Room Available established.
7. Availability history preserved.
8. Reading Room ready for Operational Monitoring.

### Postconditions

- Reading Room is operationally available.
- Applicable rules govern permitted use.
- The room can proceed to Operational Monitoring.
- No seat reservation exists.
- Membership is unchanged.
- No library-wide opening decision is created.
- No technical implementation state is introduced.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. No dedicated canonical Reading Room opening/availability event is exposed in the retrieved L2.
2. The L2 does not define a detailed business readiness checklist beyond the workflow and configurable rules.
3. No separate lifecycle state exists in the L2 for opening readiness.

### Assumptions

1. Operating hours are configuration driven.
2. Category and rules are authoritative upstream configuration.
3. G remains canonical owner.
4. E participation is limited to personnel context.

### Deferred Questions

Confirm whether the final event register defines a canonical Reading Room availability/opening event and whether any additional upstream readiness obligations are mandatory.

No local resolution is invented.
