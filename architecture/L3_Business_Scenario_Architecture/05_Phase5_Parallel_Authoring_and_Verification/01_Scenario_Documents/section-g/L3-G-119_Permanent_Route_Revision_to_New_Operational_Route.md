# Permanent Route Revision to New Operational Route

**Scenario ID:** L3-G-119  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Cross-domain  
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
| Final L3 ID | L3-G-119 |
| Canonical Title | Permanent Route Revision to New Operational Route |
| Primary Owner | Transport Management |
| Participating Domains | G, C, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs a permanent revision of an institutional transport route when the existing operational route must be replaced by a new enduring route arrangement.

It takes the route-change decision through operational review, impact assessment and approval to an effective new operational route while preserving historical route information and maintaining the ownership boundaries of passengers, personnel, finance and maintenance.

## 3. Business Outcome

A new permanent operational route arrangement is authorised and becomes the effective route for the applicable transport service, with the prior route history retained and affected transport stakeholders able to operate against the revised route.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when a permanent route revision is requested or identified as necessary.

### 4.2 End Boundary

The scenario ends when the revised route has been approved and established as the new operational route arrangement.

### 4.3 In Scope

- Identify the route requiring permanent revision.
- Establish the business reason for change.
- Assess affected trips, stops, capacity, vehicle availability, driver assignment and passenger allocation.
- Determine the revised route arrangement.
- Apply required approval.
- Establish the revised route as effective.
- Preserve historical route information.
- Coordinate affected transport allocations and operational participants where required.
- Make the resulting route-change fact available to relevant consumers.

### 4.4 Out of Scope

- Temporary emergency diversion.
- Vehicle ownership or procurement.
- Vehicle maintenance.
- Student or employee identity lifecycle.
- Financial accounting.
- Notification delivery implementation.
- Technical integration or infrastructure design.
- Any unrelated transport complaint or pass lifecycle.

## 5. Ownership and Participating Domains

Transport Management is the canonical owner of the route revision and resulting operational route.

Section C participates because student allocations may be affected. Section E participates where staff/driver context is affected. Section H may consume the route-change business fact for communication.

Participation does not transfer route ownership to C, E or H.

G04 explicitly defines route changes as an operational capability and provides a route-change workflow:

`Change Requested → Operational Review → Impact Assessment → Approval → Route Updated → Stakeholders Notified`

## 6. Business Actors, Roles and Responsibilities

### Transport Administrator / Transport Manager

Owns the operational review and approval authority assigned by configured transport policy.

### Transport Supervisor / Coordinator

Coordinates assessment and operational transition to the revised route.

### Driver / Conductor

Acts on the effective route after the revision becomes operational.

### Affected Student / Staff Passenger

Is affected when the route revision changes the passenger's transport arrangement.

### Section C Authority

Provides authoritative student identity/context where student transport allocation is affected.

### Section E Authority

Provides authoritative personnel/driver context where staff transport operations are affected.

### Section H Communication Consumer

May consume the resulting route-change business fact for governed communication.

## 7. Preconditions and Entry Criteria

1. An existing operational route is identifiable.
2. A permanent revision is required.
3. The reason for revision is established.
4. The affected operational context can be assessed.
5. Required approval authority is determinable.
6. A revised route can be described at business level.
7. Relevant affected allocations, driver assignments and trip arrangements can be identified.
8. G04 remains the canonical owner of the route.

## 8. Trigger

A permanent business requirement, operational change, sustained route condition, administrative decision or other authorised transport need requires the existing route to be permanently revised.

## 9. Main Business Flow

1. A permanent route revision need is identified.
2. The affected existing route is identified.
3. The reason and intended business outcome of the revision are established.
4. Transport Management performs operational review.
5. Impact assessment considers active trips, route capacity, vehicle availability, driver assignment and affected passenger allocation where applicable.
6. A revised route arrangement is defined at business level.
7. The proposed revision is reviewed against applicable transport policy.
8. Required business approval is obtained.
9. The route is updated to the approved operational arrangement.
10. Affected transport allocations, driver assignments or trip arrangements are adjusted where the approved route change requires it.
11. Historical route information is preserved.
12. The revised route becomes the effective operational route.
13. Relevant stakeholders/consumers receive the resulting business fact through the governed communication arrangement.
14. The scenario concludes at the new operational route state; future trips operate under the revised arrangement.

## 10. Alternate Business Flows

### 10.1 Revision Is Not Approved

The existing route remains authoritative and no permanent route revision is established.

### 10.2 Revision Scope Is Narrowed

If review identifies that only part of the proposed change is justified, an approved revised route may be established with the narrower business scope.

### 10.3 Passenger Allocation Changes Are Required

Where the revised route changes pickup/drop arrangements for allocated passengers, the affected transport allocations are updated through the applicable G04 allocation workflow.

### 10.4 Driver Assignment Changes Are Required

Where the revised route requires different driver assignment, G04 coordinates the operational assignment while Section E remains owner of driver/personnel identity.

### 10.5 Existing Trip Arrangement Is Affected

Where a scheduled trip is affected by the route revision, its operational arrangement is adjusted through the applicable trip workflow. This scenario does not redefine the Trip lifecycle.

## 11. Failure, Interruption and Business Recovery

### 11.1 Impact Assessment Identifies an Unsuitable Revision

The proposed route is not approved in its current form. Transport Management reassesses the business route arrangement.

### 11.2 Required Approval Is Rejected

The existing route remains in force unless another authorised transport decision establishes a different outcome.

### 11.3 Revised Route Cannot Be Established

The route revision remains incomplete and the existing authoritative route remains the basis for transport operations until a valid business decision is reached.

### 11.4 Revision Becomes Temporary Instead

If the requirement is determined to be temporary rather than permanent, the change must be handled through the emergency/temporary diversion or temporary stop-change outcome instead of being forced into this scenario.

### 11.5 Revision Creates Downstream Allocation Impacts

Affected allocations are resolved through G04 operational allocation processes. The scenario does not change student/personnel ownership.

## 12. Business Rules and Decision Constraints


- Transport Management (G04) is the canonical owner of transport routes, stops, route sequencing, allocation, trip planning/execution, route diversions and operational transport services.
- Vehicle ownership remains outside G04; maintenance execution remains within G01.
- Student identity remains owned by Section C; employee/driver ownership remains within Section E.
- Financial accounting remains owned by Finance; G04 provides operational transport information rather than executing accounting.
- Transport routes, stop policies, allocation policies, approval workflows and operational transport policies are configuration driven.
- Institutional transport operations support manual operational decisions; route diversion and trip adjustments remain manually overridable.
- Historical transport operations shall not be deleted.

Additional scenario-specific constraints:

- G04 defines permanent route revision as a supported route-change type.
- G04 route-change workflow includes Change Requested, Operational Review, Impact Assessment, Approval, Route Updated and Stakeholders Notified.
- Impact assessment may consider active trips, route capacity, vehicle availability, driver assignment, student allocation and administrative approval.
- Route change validation rules remain configuration driven.
- Historical route information must remain preserved.
- A permanent revision must remain distinct from an emergency diversion.
- A route revision does not transfer driver ownership to G04 or student identity ownership to G04.
- Any financial consequence remains within Finance and is not authored as a G04 accounting outcome.

## 13. State and Lifecycle Impact

The authoritative Transport Route lifecycle is:

`Draft → Configured → Active → Suspended → Retired`

G04 also defines `RouteModified` as the event describing a route update.

This scenario therefore treats the permanent revision as an authoritative route modification rather than inventing a `Revised` route lifecycle state.

Where the revised route remains operational, the route can continue under the applicable active/configured lifecycle state. The revision changes the route's business definition and operational arrangement; it does not create a new lifecycle category.

If the route must be suspended or retired as a consequence of a separately governed decision, that lifecycle transition must follow its own authoritative business rule rather than being assumed by this scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The locked G04 event catalogue defines:

`RouteModified` — Route updated.

This is the canonical event applicable to the route update outcome and is published by G04.

The scenario does not invent `PermanentRouteRevised`, `RouteRevisionApproved`, or another event.

Where affected transport allocation changes occur, G04's canonical allocation events include:

- `TransportAllocated`
- `TransportAllocationUpdated`
- `AllocationWithdrawn`

The applicable event depends on the actual authoritative allocation transition. No event is asserted merely because an allocation is potentially affected.

### 14.2 Synchronous Business Collaboration / Hand-offs

G04 may require authoritative student context from Section C and personnel context from Section E.

C and E retain ownership of their respective identity/personnel facts.

Where Finance consumes transport allocation or service information, Finance remains owner of financial accounting.

H may consume the route-change business fact for governed communication.

These collaborations support the route revision without transferring route ownership.

## 15. Business Authorization, Approval and Audit Obligations

G04's route-change workflow explicitly includes operational review, impact assessment and approval.

The business decision record should preserve:

- route being revised;
- reason for permanent revision;
- impact assessment;
- affected operational context;
- proposed revised route;
- approval decision;
- effective revised route;
- affected allocation/assignment consequences where applicable;
- historical route context.

Transport Management owns the business route decision and operational history. Platform remains owner of audit infrastructure.

No technical access control, IAM, API or evidence-storage design is introduced.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Existing route identified.
- Permanent business reason established.
- Operational impact assessed.
- Revised route arrangement defined.
- Required approval completed.
- Route updated to the approved operational arrangement.
- Affected transport allocations/assignments adjusted where required.
- Historical route information preserved.
- `RouteModified` published by G04.
- Revised route is authoritative for subsequent applicable transport operations.

### 16.2 Postconditions

- The revised route is the effective operational route.
- G04 remains canonical owner.
- Previous route information remains historically available.
- Affected transport users operate under the revised route arrangement.
- Student and personnel identity ownership remains unchanged.
- Any later temporary diversion remains a separate operational scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material ownership or lifecycle conflict was identified.

The G04 source explicitly defines the route-change workflow and the `RouteModified` event, so no missing event is required for the core route-update outcome.

### 17.2 Assumptions

- “Permanent” is interpreted according to the frozen canonical scenario title and G04 route-change capability.
- Exact approval authority is configuration/policy driven.
- Affected allocation changes are included only when the approved route revision actually changes those allocations.
- Historical route information remains preserved.

### 17.3 Deferred Questions

If future architecture introduces a distinct lifecycle for route versions or a dedicated permanent-revision event, that must be established upstream before being represented in L3.
