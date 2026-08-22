# Transport Complaint to Operational Resolution

**Scenario ID:** L3-G-120  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

Transport Management must provide a complete institutional path for complaints concerning transport services. A complaint is not complete merely because it has been received or classified: it must be reviewed, its operational significance determined, an appropriate business resolution established, and the outcome retained as part of the institution's transport history.

This scenario therefore connects complaint intake to the correct operational resolution without redesigning any upstream ownership. It supports complaints involving vehicle condition, driver behaviour, delay, missed stop, route issue, safety concern, cleanliness, overcrowding and other configured transport-service categories.

The scenario also provides a controlled boundary for complaints that reveal a maintenance concern, personnel concern, route issue, or communication requirement. Those concerns remain owned by their canonical domains; G04 owns the transport complaint and its operational resolution.

## 3. Business Outcome

The transport complaint reaches an authoritative G04-owned **Resolved** outcome through the appropriate operational response and subsequently reaches **Closed** in accordance with the Transport Complaint lifecycle.

The resolution must correspond to the reviewed complaint. Depending on the complaint, the business resolution may be administrative action, driver counselling, route modification, passenger communication, maintenance recommendation, or another configured transport operational response.

The scenario does not claim that a downstream maintenance activity, personnel action, or notification delivery has completed. Those are separate domain outcomes.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when a transport complaint is registered concerning an institutional transport service.

The L2 source identifies complaint sources including students, parents, staff, drivers, conductors and transport supervisors. The complaint can concern the operational service itself, a vehicle condition, driver behaviour, delay, missed stop, route issue, safety, cleanliness, overcrowding, or another configured category.

### 4.2 End Boundary

The scenario ends after Transport Management has established the appropriate complaint resolution and the complaint reaches its G04-owned closure outcome.

A complaint requiring a separate domain action does not wait for that entire downstream lifecycle to be represented here. Instead, the G04 scenario records and resolves the transport-side obligation and hands the separately owned business concern to the canonical owner.

### 4.3 In Scope

1. Complaint registration and contextualisation.
2. Identification of reporter and transport context.
3. Complaint classification.
4. Severity/operational assessment where supported by configured policy.
5. Operational review.
6. Selection of the appropriate resolution path.
7. Administrative action where applicable.
8. Driver counselling where applicable.
9. Route/service correction where applicable.
10. Passenger communication requirement where applicable.
11. Maintenance recommendation where applicable.
12. Recording of resolution and supporting evidence.
13. Complaint resolution and closure.
14. Preservation of immutable complaint history.
15. Publication/consumption of the exact applicable G04 business events.

### 4.4 Out of Scope

- Student identity lifecycle.
- Employee/driver personnel lifecycle.
- Vehicle asset ownership.
- Vehicle repair or maintenance execution.
- Financial accounting.
- Technical notification delivery.
- Technical workflow engine design.
- API, database, class, service, Business Data Structure, UI, infrastructure, deployment, queue, retry or rollback design.

## 5. Ownership and Participating Domains

### G — Transport Management

G04 is the canonical owner of the Transport Complaint aggregate and the operational resolution.

G04 owns:
- complaint registration;
- complaint classification;
- operational review;
- transport-side resolution;
- maintenance recommendation when the transport assessment identifies maintenance;
- complaint resolution and closure;
- complaint evidence/history.

### C — Student Lifecycle

C remains authoritative for student identity and student lifecycle facts. G04 may consume the required student context but does not create or modify student identity through this scenario.

### E — People & Workforce

E remains authoritative for employee/driver/personnel facts. G04 may identify the operational driver involved in a complaint and may coordinate transport-level counselling, but it does not create a personnel lifecycle state.

### H — Institutional Services / Communication

H may consume a transport outcome where passenger communication is required. H owns the communication/notification concern, not the complaint or its operational resolution.

Participating-domain collaboration therefore does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

### Complaint Reporter

Supported reporters include:
- Student;
- Parent;
- Staff member;
- Driver;
- Conductor;
- Transport Supervisor.

The reporter provides the complaint and whatever operational context is available.

### Transport Coordinator

Coordinates complaint registration, classification, review and the operational resolution process.

### Transport Supervisor

Provides operational review and coordinates corrective action when the complaint requires supervisory intervention.

### Transport Manager / Authorised Transport Authority

Performs configured transport decision or approval responsibilities where the resolution requires higher operational authority.

### Driver / Conductor

May provide operational context and may receive transport-level counselling where the complaint concerns conduct.

### Section C Authority

Provides authoritative student context where needed.

### Section E Authority

Provides authoritative employee/driver context where needed and owns personnel consequences that exceed G04 transport authority.

### Section H Communication Consumer

Consumes an established transport business fact when a communication outcome is required.

No technical role or permission model is defined.

## 7. Preconditions and Entry Criteria

1. A complaint concerning institutional transport is available for registration.
2. The complaint can be associated with a transport context sufficiently to begin review.
3. The reporter is identified when required by the applicable policy, or the complaint can otherwise be accepted under the institution's complaint process.
4. The transport service, route, trip, vehicle, driver or other relevant context can be established where materially available.
5. G04 is the owner of the transport complaint.
6. Any student/personnel information required for review remains sourced from C/E.
7. The complaint has not already reached an authoritative terminal resolution.

## 8. Trigger

A student, parent, staff member, driver, conductor, or transport supervisor raises an operational complaint concerning an institutional transport service.

## 9. Main Business Flow

### Phase A — Register the Complaint

1. Transport Management receives the complaint.
2. The complaint is registered as a Transport Complaint.
3. The reporter and available operational context are recorded.
4. The complaint is assigned the applicable configured category.
5. The complaint is assigned the applicable severity/priority according to authoritative policy.
6. `TransportComplaintRegistered` is published by G04.

### Phase B — Review and Assess

7. Transport Management reviews the complaint.
8. The reviewer determines what transport service, route, trip, vehicle, driver or stop context is relevant.
9. Where a student is involved, the authoritative student context is obtained from C rather than recreated by G04.
10. Where a driver/personnel fact is involved, the authoritative personnel context is obtained from E.
11. The complaint is assessed to determine whether it represents:
    - a transport operational issue;
    - a driver/service issue;
    - a route/stop issue;
    - a vehicle-condition/maintenance issue;
    - a communication requirement; or
    - another configured transport complaint category.
12. The reviewer determines the appropriate resolution path.

### Phase C — Establish the Operational Resolution

13. If administrative action is appropriate, the configured transport authority establishes that action.
14. If driver counselling is appropriate, the transport-side counselling outcome is established without changing the employee lifecycle.
15. If a route/service correction is required, the applicable G04 route or trip workflow is used.
16. If passenger communication is required, the resulting business fact is made available to H.
17. If a maintenance concern is identified, G04 establishes a maintenance recommendation and preserves the G04-to-G01 boundary.
18. If no corrective action is warranted, the review decision and rationale are recorded.

### Phase D — Resolve and Close

19. The selected G04-owned resolution is recorded.
20. Supporting complaint evidence and operational context are preserved.
21. The complaint reaches `Resolved`.
22. `ComplaintResolved` is published by G04.
23. The complaint reaches `Closed` according to the complaint lifecycle.
24. Historical complaint information remains immutable and available for operational traceability.

The main flow stops here. It does not extend into downstream maintenance execution, personnel action, or communication delivery.

## 10. Alternate Business Flows

### 10.1 Complaint Is a Vehicle-Condition Complaint

The complaint is reviewed against the relevant vehicle context. If a maintenance requirement is established, G04 records the maintenance recommendation. The scenario does not claim that G01 has repaired the vehicle.

### 10.2 Complaint Is a Driver-Behaviour Complaint

The complaint is reviewed with authoritative driver/personnel context. Where transport-level counselling is appropriate, G04 records the operational corrective outcome. Any personnel-governed consequence remains with E.

### 10.3 Complaint Is a Route or Stop Issue

The complaint becomes business context for the relevant route/stop operational workflow. G04 remains owner of both the complaint and route/stop operations.

### 10.4 Complaint Requires Passenger Communication

The G04 resolution fact is made available to H for the governed communication outcome. H owns delivery; G04 remains owner of the complaint resolution.

### 10.5 Complaint Requires No Corrective Action

If review establishes that the complaint does not warrant corrective action, the review outcome and rationale are recorded. The complaint may then proceed to `Resolved` and `Closed`.

### 10.6 Complaint Is Escalated

Where the operational issue cannot be resolved at the current review level, `OperationalIssueEscalated` may be published by G04 and the complaint proceeds through the configured business escalation path.

## 11. Failure, Interruption and Business Recovery

### 11.1 Insufficient Business Context

If the complaint cannot yet be meaningfully classified, it remains under business review until the information necessary for a decision is available. No technical retry behaviour is defined.

### 11.2 Required Cross-Domain Context Is Unavailable

If authoritative student or personnel context is needed, G04 does not create a substitute fact. The relevant owning domain remains authoritative.

### 11.3 Proposed Resolution Is Rejected

If configured operational authority does not approve the proposed corrective action, the complaint returns to the appropriate business review path and an alternative authorised resolution is considered.

### 11.4 Operational Resolution Does Not Address the Complaint

If the reviewed outcome is found insufficient, the complaint may be escalated or re-reviewed under the configured transport process. The scenario does not invent a new complaint lifecycle state.

### 11.5 Maintenance Is Required

G04 records the maintenance recommendation and ends its maintenance-specific responsibility at that business hand-off. G01 owns subsequent maintenance execution.

### 11.6 Personnel Action Is Required

If the complaint requires personnel governance beyond transport operational authority, E receives the relevant business context. G04 does not create a personnel state.

## 12. Business Rules and Decision Constraints

The following rules are directly supported by L2-G04:

1. Transport Management coordinates complaint resolution.
2. Complaint sources include students, parents, staff, drivers, conductors and transport supervisors.
3. Complaint categories include vehicle condition, driver behaviour, delay, missed stop, route issue, safety concern, cleanliness, overcrowding and operational service.
4. Institutions may configure additional complaint categories.
5. Complaint resolution may include administrative action, driver counselling, route modification, passenger communication, maintenance recommendation and complaint closure.
6. Every complaint preserves reporter, route, trip, category, severity, resolution, supporting evidence and audit history.
7. Historical complaint records are immutable.
8. Transport Management remains the owner of transport operations.
9. Student ownership remains with C.
10. Personnel ownership remains with E.
11. Notification delivery remains outside G04.
12. Maintenance execution remains within G01.
13. Transport policy is configuration driven.
14. Manual operational decisions remain supported.

No additional complaint severity, approval threshold, SLA, penalty, compensation rule, disciplinary rule, or communication channel is invented here.

## 13. State and Lifecycle Impact

The authoritative Transport Complaint lifecycle is:

`Registered → Reviewed → Resolved → Closed`

The scenario applies those states exactly.

### Registered → Reviewed

Occurs after the complaint has been received and entered into the operational review process.

### Reviewed → Resolved

Occurs when the appropriate G04-owned business resolution has been established.

### Resolved → Closed

Occurs when the G04 complaint obligation is complete and the complaint reaches its closure outcome.

The following states are deliberately **not** invented:
- `Classified`;
- `MaintenancePending`;
- `Escalated`;
- `AwaitingPassenger`;
- `AwaitingDriver`;
- `AwaitingRepair`.

Escalation is an operational event/outcome, not a newly invented complaint lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The locked G04 event catalogue defines:

| Event | Meaning | Canonical Publisher |
|---|---|---|
| `TransportComplaintRegistered` | Complaint recorded | G04 |
| `OperationalIssueEscalated` | Issue escalated | G04 |
| `ComplaintResolved` | Complaint resolved | G04 |

These exact event names are used. No complaint-specific event names are invented.

Where a downstream outcome has its own canonical event, the downstream owner remains the publisher. G04 does not publish another domain's event.

### 14.2 Synchronous Business Collaboration / Hand-offs

The scenario may require business-level collaboration with:

- **C:** authoritative student context;
- **E:** authoritative personnel/driver context;
- **G01:** maintenance recommendation/service-request boundary;
- **H:** communication/notification consumption.

The collaboration is distinct from event publication. An event communicates an established business fact; a synchronous hand-off is used where an immediate authoritative business response or governed cross-domain action is required.

No API, payload, protocol, service operational execution, or database mechanism is defined.

## 15. Business Authorization, Approval and Audit Obligations

Complaint resolution follows the configured G04 operational authority.

Where a resolution requires supervisory or managerial approval, the applicable business approval must be established before the resolution is treated as authoritative.

Where a complaint involves a driver, the transport decision must remain distinct from personnel authority. G04 may establish transport counselling/corrective action; E remains authoritative for personnel lifecycle consequences.

The complaint record must preserve:
- reporter;
- route;
- trip;
- category;
- severity;
- resolution;
- supporting evidence;
- audit history.

G04's immutable complaint history requirement is applied directly. No unsupported evidence type, retention duration, approval role, or technical audit artefact is added.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Complaint registered.
- Reporter/context recorded.
- Category and severity established according to configured policy.
- Operational review completed.
- Appropriate resolution path selected.
- Required business approval completed where applicable.
- G04-owned resolution established.
- Relevant downstream business hand-off made where required.
- `ComplaintResolved` published by G04.
- Complaint reaches `Closed`.
- Historical complaint evidence remains preserved.

### Postconditions

1. The complaint has an authoritative G04 resolution.
2. The complaint lifecycle is complete through `Closed`.
3. Any maintenance requirement remains a G01-owned downstream outcome.
4. Any personnel consequence remains E-owned.
5. Any student identity consequence remains C-owned.
6. Communication delivery remains H-owned.
7. Complaint history remains immutable.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership, lifecycle, scenario-boundary, final-outcome, or canonical event-publisher conflict was identified for the core complaint-resolution outcome.

### Assumptions

- Exact complaint categories may be institutionally configured in addition to the categories explicitly listed by G04.
- Exact approval thresholds are configuration driven.
- A complaint can provide context to another workflow without absorbing that workflow's outcome.
- The G04 complaint remains closed when the G04-owned resolution obligation is complete; downstream domain execution is not silently included.

### Deferred Questions

None material to the L3 business outcome. Any future change to complaint lifecycle, canonical event names, or ownership must be governed upstream before being incorporated into L3.
