# Temporary Stop Change to Affected Passenger Reassignment

**Scenario ID:** L3-G-118  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Cross-domain  
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
| Final L3 ID | L3-G-118 |
| Canonical Title | Temporary Stop Change to Affected Passenger Reassignment |
| Primary Owner | Transport Management |
| Participating Domains | G, C, E, H |
| Scenario Type | Cross-domain |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs a temporary change to a transport stop that affects passengers using the existing transport arrangement.

The institutional outcome is not merely the creation or suspension of a stop. It is the coordinated reassignment of affected passengers to an appropriate operational pickup/drop arrangement while preserving the canonical ownership of the route, stop, passenger identity and transport allocation domains.

## 3. Business Outcome

Affected passengers are reassigned to the applicable temporary transport stop arrangement following an authorised temporary stop change, and the resulting allocation/route context is made operationally effective for the affected transport service.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when a temporary stop change is required and the change is known to affect one or more passengers allocated to the transport service.

### 4.2 End Boundary

The scenario ends when the affected passengers have an authoritative effective reassignment to the applicable temporary stop arrangement.

### 4.3 In Scope

- Identify affected route and stop.
- Identify affected passenger allocations.
- Assess the temporary stop change.
- Determine an appropriate temporary pickup/drop arrangement.
- Apply route/stop operational change according to G04 policy.
- Reassign affected passenger transport allocations.
- Preserve allocation history.
- Communicate the resulting business fact to relevant participants/consumers.
- Maintain traceability between the affected stop and the reassigned passenger context.

### 4.4 Out of Scope

- Permanent route redesign.
- Student identity changes.
- Employee identity changes.
- Vehicle ownership or maintenance.
- Financial accounting.
- Technical notification delivery.
- Technical APIs, databases, UI, queues or infrastructure.

## 5. Ownership and Participating Domains

G04 owns the transport stop, route and transport allocation outcomes.

Section C remains the canonical owner of student identity. Section E remains the canonical owner of employee/driver identity. Section H participates as a communication/notification consumer where applicable.

A temporary stop change does not transfer passenger identity ownership to Transport Management; G04 only owns the transport allocation and operational service relationship.

## 6. Business Actors, Roles and Responsibilities

### Transport Administrator / Transport Manager

Reviews and authorises transport route/stop changes according to configured operational policy.

### Transport Supervisor / Coordinator

Coordinates the operational adjustment and affected passenger reassignment.

### Affected Student / Staff Passenger

Is the holder of the transport allocation affected by the temporary stop change.

### Driver / Conductor

Acts on the effective temporary transport arrangement.

### Section C

Provides authoritative student identity/context where the affected passenger is a student.

### Section E

Provides authoritative personnel context where the affected passenger is an employee.

### Section H

May consume the resulting transport business fact for governed communication.

## 7. Preconditions and Entry Criteria

1. A transport route and affected stop are identifiable.
2. A temporary stop change is required.
3. One or more active transport allocations are affected.
4. The affected passenger context can be identified from its canonical owner.
5. The temporary arrangement can be assessed under G04 operational policy.
6. The transport allocation remains G04-owned.
7. Required operational approval can be determined.

## 8. Trigger

A temporary operational condition requires a pickup/drop stop to be changed and the change affects passengers currently associated with that stop.

## 9. Main Business Flow

1. Identify the route and affected stop.
2. Establish why the stop must be temporarily changed.
3. Identify passengers whose active transport allocation is affected.
4. Assess the operational effect of the stop change.
5. Determine an appropriate temporary stop arrangement.
6. Review route, stop and passenger implications.
7. Obtain configured operational approval where required.
8. Apply the temporary stop arrangement.
9. Reassign each affected passenger to the applicable temporary stop/transport arrangement.
10. Preserve the previous allocation and stop history.
11. Make the effective reassignment available to affected operational participants and communication consumers.
12. Conclude when the affected passenger allocation is authoritative under the temporary arrangement.

## 10. Alternate Business Flows

### 10.1 No Passenger Is Affected

If assessment establishes that the temporary stop change does not affect any active allocation, the stop change can proceed under the applicable stop workflow; this scenario's passenger-reassignment outcome is not entered.

### 10.2 Alternative Temporary Stop Selected

If the first proposed temporary stop is unsuitable, another stop arrangement may be evaluated and selected according to operational policy.

### 10.3 Only a Subset of Passengers Is Affected

Where only some passengers are affected, only those allocations are reassigned. Unaffected allocations remain unchanged.

### 10.4 Temporary Stop Change Requires Route Adjustment

Where the temporary stop arrangement requires a route operational adjustment, the route-change decision is governed by G04. The passenger-reassignment outcome remains the endpoint of this scenario.

## 11. Failure, Interruption and Business Recovery

### 11.1 Proposed Stop Cannot Be Used

If the proposed temporary stop is not operationally suitable, the reassignment is not completed against that stop. A different business arrangement must be selected.

### 11.2 Required Approval Is Not Granted

The temporary stop arrangement does not become effective and affected allocations remain governed by the existing authorised arrangement until another business decision is made.

### 11.3 Passenger Context Cannot Be Confirmed

The reassignment is not finalised against unverified passenger identity/context. The authoritative owning domain must resolve the business identity context before G04 completes the allocation change.

### 11.4 Temporary Arrangement Cannot Be Established

If no acceptable temporary stop can be established, Transport Management must make the applicable operational decision for the affected service. This scenario does not invent a new state or emergency rule.

## 12. Business Rules and Decision Constraints


- Transport Management (G04) is the canonical owner of transport routes, stops, route sequencing, allocation, trip planning/execution, route diversions and operational transport services.
- Vehicle ownership remains outside G04; maintenance execution remains within G01.
- Student identity remains owned by Section C; employee/driver ownership remains within Section E.
- Financial accounting remains owned by Finance; G04 provides operational transport information rather than executing accounting.
- Transport routes, stop policies, allocation policies, approval workflows and operational transport policies are configuration driven.
- Institutional transport operations support manual operational decisions; route diversion and trip adjustments remain manually overridable.
- Historical transport operations shall not be deleted.

Scenario-specific constraints:

- G04 defines Transport Stop as a designated pickup or drop location along a route.
- Stop management includes stop creation, assignment and suspension.
- Route change validation may consider active trips, route capacity, vehicle availability, driver assignment, student allocation and administrative approval.
- Transport Allocation owns student, staff and guest allocation status and history.
- A temporary stop change must not rewrite student or employee identity.
- Historical allocation information must remain available.
- The scenario is distinct from permanent route revision.

## 13. State and Lifecycle Impact

The G04 Stop lifecycle is represented operationally through:

`Created → Assigned → Active → Suspended`

The scenario must not invent a `Temporary` stop state.

A temporary stop change may require the affected stop to be operationally adjusted, suspended, or otherwise managed according to the authoritative stop workflow. The resulting passenger reassignment is a Transport Allocation outcome, whose lifecycle is:

`Applied → Approved → Allocated → Active → Withdrawn`

The scenario does not invent a `Reassigned` allocation state. The reassignment is an effective change to the allocation's route/stop context while preserving allocation history.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The locked G04 event catalogue explicitly defines stop events:

- `StopCreated`
- `StopAssigned`
- `StopSuspended`

It also defines allocation events:

- `TransportAllocated`
- `TransportAllocationUpdated`
- `AllocationWithdrawn`

There is no supplied canonical `TemporaryStopChanged` or `PassengerReassigned` event.

Therefore, this scenario does not invent either event.

Where the stop lifecycle or allocation outcome actually undergoes an authoritative event-bearing transition, the exact upstream event name and G04 publisher must be retained.

### 14.2 Synchronous Business Collaboration / Hand-offs

G04 requires authoritative passenger context from the canonical owning domain. For students, Section C remains the owner; for employees, Section E remains the owner.

The resulting transport allocation change is G04-owned. Section H may consume the business fact for communication. None of these collaborations transfers ownership.

## 15. Business Authorization, Approval and Audit Obligations

G04 route-change workflow includes operational review, impact assessment and approval where configured.

The business record should establish:

- affected route and stop;
- reason for temporary change;
- affected passenger allocation;
- operational assessment;
- approved temporary arrangement;
- effective passenger reassignment;
- historical allocation context.

Audit infrastructure remains a Platform concern; G04 retains the operational business history that must be auditable.

No technical permission model is defined.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Affected stop and route identified.
- Affected passenger allocations identified.
- Temporary stop arrangement assessed and authorised.
- Required route/stop business change completed.
- Affected passenger allocations updated.
- Previous allocation history preserved.
- Relevant operational participants can use the effective arrangement.
- No permanent route revision is implied.

### 16.2 Postconditions

- Affected passengers have an authoritative effective transport allocation under the temporary stop arrangement.
- G04 remains owner of the transport allocation and stop.
- C/E remain owners of passenger identity/context.
- Historical allocation and stop information remains traceable.
- A later permanent change requires its own scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material ownership conflict identified.

The supplied G04 event catalogue does not define a dedicated event named for temporary stop change or passenger reassignment. This is recorded rather than filled with an invented event.

### 17.2 Assumptions

- Temporary stop changes are governed by configurable G04 transport policy.
- The exact affected-passenger communication mechanism is outside G04.
- The scenario does not create a new stop lifecycle state.
- Allocation history remains preserved.

### 17.3 Deferred Questions

If the architecture later requires a dedicated temporary-stop or passenger-reassignment event, the event name, semantics and canonical publisher must be defined upstream before inclusion in L3.
