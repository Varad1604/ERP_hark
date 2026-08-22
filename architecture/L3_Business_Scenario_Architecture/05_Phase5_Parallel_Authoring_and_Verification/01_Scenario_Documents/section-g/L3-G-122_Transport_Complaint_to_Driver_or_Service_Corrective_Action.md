# Transport Complaint to Driver or Service Corrective Action

**Scenario ID:** L3-G-122  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, E, H  
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
| Final L3 ID | L3-G-122 |
| Canonical Title | Transport Complaint to Driver or Service Corrective Action |
| Primary Owner | Transport Management |
| Participating Domains | G, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Transport complaints can identify conduct or service problems that require corrective action rather than maintenance or simple complaint closure.

This scenario establishes the complete transport-owned corrective outcome while preserving the distinction between:
- G04 transport operational responsibility;
- E personnel ownership;
- H communication ownership.

The scenario can result in driver counselling, service correction, route/service adjustment, administrative transport action, or another configured G04 corrective outcome.

It must not create or modify personnel lifecycle states owned by Section E.

## 3. Business Outcome

A transport complaint concerning driver behaviour or transport service results in an authorised and recorded **driver or service corrective action**, with the complaint reaching the appropriate G04 resolution boundary.

The corrective action addresses the transport operational problem. Where the matter requires a personnel-governed consequence beyond G04 authority, the relevant business context is handed to E and the G04 scenario does not claim completion of the E-owned outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins with a registered transport complaint that, after classification/review, concerns driver behaviour or transport service and requires corrective action.

### 4.2 End Boundary

The scenario ends when the G04 transport corrective action has been established and recorded, and the complaint reaches its G04-owned resolution/closure outcome.

A personnel action that continues beyond G04 is outside the scenario.

### 4.3 In Scope

- Complaint review and classification.
- Driver/service context identification.
- Operational assessment.
- Determination of corrective action.
- Driver counselling at transport operational level.
- Service correction.
- Route/stop adjustment where appropriate.
- Administrative transport action.
- Passenger communication where applicable.
- Personnel hand-off where the matter exceeds G04 authority.
- Complaint resolution/closure.
- Immutable history.

### 4.4 Out of Scope

- Employee hiring, employment, disciplinary lifecycle or separation.
- Personnel sanctions not owned by G04.
- Vehicle maintenance execution.
- Student lifecycle changes.
- Financial consequences.
- Technical communication delivery.
- technical access control/IAM, APIs, databases, queues, UI, infrastructure, deployment or retry mechanics.

## 5. Ownership and Participating Domains

### G — Transport Management

G04 owns:
- transport complaint;
- transport operational review;
- transport service corrective action;
- transport-level driver counselling;
- route/service correction;
- complaint resolution.

### E — People & Workforce

E owns:
- employee/personnel identity;
- driver/personnel lifecycle;
- personnel-governed consequences.

G04 does not create a personnel state.

### H — Institutional Services / Communication

H may consume the resulting transport business fact for communication. H does not own corrective action.

The participating-domain set is therefore material and does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

### Complaint Reporter

May be a student, parent, staff member, driver, conductor, or transport supervisor.

### Transport Coordinator

Coordinates complaint registration and corrective-action processing.

### Transport Supervisor

Reviews the complaint and coordinates service corrective action.

### Transport Manager / Authorised Transport Authority

Approves or directs transport corrective action where configured policy requires that authority.

### Driver / Conductor

Provides operational context and may be subject to transport-level counselling.

### Section E Authority

Provides authoritative personnel context and handles personnel-owned consequences.

### Section H Communication Consumer

Consumes the established business outcome when communication is required.

## 7. Preconditions and Entry Criteria

1. A transport complaint is registered.
2. The complaint has been classified as driver behaviour or transport service.
3. Relevant route/trip/driver/service context is identifiable.
4. G04 can perform the operational assessment.
5. E remains authoritative for driver/personnel facts.
6. Required corrective-action authority is determinable under configured policy.

## 8. Trigger

A reviewed transport complaint identifies driver behaviour or a transport-service condition requiring corrective action.

## 9. Main Business Flow

### Phase A — Establish the Issue

1. Review the registered complaint.
2. Establish the affected transport service, route and/or trip.
3. Identify the driver or operational assignment where relevant.
4. Obtain authoritative personnel context from E when required.
5. Assess the complaint against the transport service context.
6. Determine severity and operational impact according to configured policy.

### Phase B — Determine Corrective Action

7. Determine whether the issue is appropriately resolved through:
   - driver counselling;
   - service correction;
   - route/stop adjustment;
   - administrative transport action;
   - passenger communication;
   - escalation to personnel governance; or
   - another configured G04 operational resolution.
8. Where a corrective action requires configured business approval, obtain it before establishing the action as authoritative.

### Phase C — Establish the Outcome

9. Establish the authorised transport corrective action.
10. If the action is driver counselling, record it as a transport operational outcome without altering personnel lifecycle.
11. If the action is service correction, apply the applicable G04 transport workflow.
12. If the issue requires E-owned personnel action, make the relevant business hand-off and retain the G04 complaint context.
13. If passenger communication is required, make the established transport business fact available to H.

### Phase D — Resolve

14. Record the corrective action and supporting complaint evidence.
15. Resolve the complaint.
16. Publish `ComplaintResolved` from G04.
17. Close the complaint according to its authoritative lifecycle.
18. Preserve the immutable complaint history.

## 10. Alternate Business Flows

### 10.1 Driver Counselling Is Sufficient

The transport-level counselling outcome is established. No personnel lifecycle state is created.

### 10.2 Service Correction Is Required

The complaint results in a service-level adjustment, such as a route or operational correction. The appropriate G04 workflow establishes the service outcome.

### 10.3 Personnel Governance Is Required

If the matter exceeds transport operational authority, E receives the relevant personnel context. G04 resolves its transport-side complaint obligation without claiming the E-owned personnel outcome.

### 10.4 Passenger Communication Is Required

H consumes the resulting transport fact and manages the communication outcome. G04 does not prescribe a channel or technical delivery mechanism.

### 10.5 No Corrective Action Is Warranted

The review decision and rationale are recorded and the complaint may be resolved and closed according to policy.

## 11. Failure, Interruption and Business Recovery

### 11.1 Insufficient Evidence

The complaint remains under business review until sufficient evidence/context is available. No technical retry is described.

### 11.2 Driver Context Cannot Be Confirmed

G04 relies on E for authoritative personnel context and does not create a duplicate driver identity.

### 11.3 Corrective Action Is Rejected

The proposed action returns to G04 review and an alternative authorised resolution is considered.

### 11.4 Corrective Action Does Not Resolve the Service Issue

The complaint may be escalated or re-reviewed under the configured G04 process. No new complaint lifecycle state is invented.

### 11.5 Personnel Action Is Required

The personnel concern is handed to E. The G04 scenario remains limited to the transport operational outcome.

### 11.6 Route/Stop Change Is Required

The complaint supplies context for the relevant G04 route/stop workflow. The corrective-action scenario does not absorb the entire route lifecycle.

## 12. Business Rules and Decision Constraints

1. Transport complaints are G04-owned.
2. Complaint sources include students, parents, staff, drivers, conductors and transport supervisors.
3. Driver behaviour and operational service are supported complaint categories.
4. Complaint resolution may include administrative action, driver counselling, route modification, passenger communication, maintenance recommendation and closure.
5. Driver/personnel ownership remains within E.
6. G04 may coordinate operational driver assignment and transport-level corrective action.
7. G04 does not create personnel lifecycle states.
8. Complaint history is immutable.
9. Notification delivery remains outside G04.
10. Maintenance execution remains G01-owned.
11. Exact corrective-action policy remains configuration driven.
12. Manual operational decisions remain supported.

No disciplinary penalty, employment consequence, performance rating, suspension period, or personnel state is invented.

## 13. State and Lifecycle Impact

The Transport Complaint lifecycle is:

`Registered → Reviewed → Resolved → Closed`

The corrective action is part of the `Reviewed → Resolved` business outcome.

It does not create:
- `CorrectiveActionPending`;
- `DriverCounselled`;
- `PersonnelActionPending`;
- `ServiceCorrected`.

Driver assignment lifecycle remains governed by G04, while personnel lifecycle remains governed by E.

Where driver assignment itself changes as an operational consequence, G04's authoritative driver-assignment lifecycle/events apply; this scenario does not invent a corrective-action-specific driver state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The locked G04 event catalogue defines:
- `TransportComplaintRegistered`;
- `OperationalIssueEscalated`;
- `ComplaintResolved`;
- `DriverAssigned`;
- `DriverReassigned`;
- `DriverAssignmentCompleted`.

No canonical `DriverCounselled` or `CorrectiveActionCompleted` event is defined in the supplied G04 event catalogue.

Therefore:
- `ComplaintResolved` is used for the complaint resolution;
- `OperationalIssueEscalated` may be used where the issue is formally escalated;
- driver-assignment events are used only if an actual authoritative driver assignment transition occurs.

No new event is invented.

### 14.2 Synchronous Business Collaboration / Hand-offs

The scenario may require:
- E for authoritative driver/personnel context and personnel-owned consequences;
- H for communication consumption.

G04 remains owner of the transport complaint and operational corrective action.

The distinction between business hand-off and event publication is preserved: the hand-off supplies or transfers an actionable business context; an event records a business fact that has already occurred.

No API or technical contract is defined.

## 15. Business Authorization, Approval and Audit Obligations

Corrective action follows configured G04 operational authority.

Where a supervisory/managerial decision is required, that business decision must be established before the corrective action is authoritative.

The complaint history preserves:
- reporter;
- route;
- trip;
- category;
- severity;
- resolution;
- supporting evidence;
- audit history.

Where a driver is involved, G04 preserves the transport complaint context but E remains authoritative for personnel information and personnel consequences.

No unsupported separation-of-duty rule is invented.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Complaint reviewed.
- Driver/service context established.
- Operational impact assessed.
- Corrective action determined.
- Required approval completed where applicable.
- G04 transport corrective action established.
- Personnel hand-off completed where required, without claiming E's downstream outcome.
- Communication fact made available where required.
- `ComplaintResolved` published by G04.
- Complaint closed.
- History preserved.

### Postconditions

1. The complaint has an authoritative G04 resolution.
2. The driver/service issue has an established transport corrective outcome where warranted.
3. Personnel lifecycle remains E-owned.
4. Communication delivery remains H-owned.
5. Complaint history remains immutable.
6. Any subsequent personnel or route lifecycle remains governed by its canonical domain/workflow.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership or final-outcome conflict was identified.

The supplied G04 event catalogue does not define a dedicated corrective-action event. The scenario therefore uses the exact existing complaint and escalation events and does not invent a new event.

### Assumptions

- Exact corrective-action criteria are configuration/policy driven.
- Driver counselling is a transport operational outcome, not a personnel lifecycle state.
- Any personnel-governed action beyond G04 authority is handled by E.
- A service correction may invoke a separate G04 workflow without expanding this scenario's end boundary.

### Deferred Questions

None material to the frozen G120–G122 outcomes.
