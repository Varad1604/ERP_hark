# Route Suspension to Operational Transport Restriction

**Scenario ID:** L3-G-095  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
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

## 2. Scenario Purpose

Govern the temporary or emergency suspension of an operational transport route due to road closures, vehicle unavailability, extreme weather, or administrative restructuring, restricting operational transport while notifying all assigned passengers and downstream operations.

---

## 3. Business Outcome

The target transport route transitions to the **Suspended** state. Scheduled trip generation is halted, passenger allocations are marked restricted or re-routed, and authoritative suspension notifications are published to all affected institutional stakeholders.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario begins upon receipt of an authorized suspension request or operational safety hazard trigger affecting a specific transport route.

### 4.2 End Boundary
The scenario concludes when the route is placed in the **Suspended** state, scheduled trips are cancelled/re-routed, and formal communications are delivered.

### 4.3 In Scope
- Verification of operational suspension justification (safety, maintenance, civic restrictions).
- Authoritative transition of Route entity to **Suspended**.
- Suspension of automated dispatch for linked trips.
- Identification and notification of assigned student and staff commuters.

### 4.4 Out of Scope
- Permanent route decommissioning (governed by separate lifecycle workflow).
- Physical road repair or municipal coordination.
- Passenger transport fee refund adjustments (handled by Finance).

---

## 5. Ownership and Participating Domains

- **Primary Owner:** Transport Management (Section G) owns route status, dispatch controls, and passenger transit schedules.
- **Participating Domains:**
  - *Student Lifecycle (Section C):* Consumes transport route suspension facts for affected student commuters.
  - *People & Workforce (Section E):* Consumes suspension notifications for faculty and staff commuters.
  - *Institutional Services (Section H):* Broadcasts multi-channel operational alerts.

---

## 6. Business Actors, Roles and Responsibilities

- **Transport Manager / Officer:** Assesses operational disruption, verifies justification, and executes route suspension.
- **Fleet Dispatcher:** Halts vehicle assignments and adjusts trip rosters.
- **Affected Commuters:** Receive timely suspension notices and alternative transit options.

---

## 7. Preconditions and Entry Criteria

1. Route currently exists in the **Active** operational state.
2. Verified justification (e.g., civic advisory, hazard report, infrastructure blockage) is submitted to Transport Management.

---

## 8. Trigger

Transport Management receives an emergency road closure alert or administrative directive requiring cessation of route operations.

---

## 9. Main Business Flow

1. Transport Officer logs and reviews the route suspension trigger.
2. Transport Officer verifies affected route stops, active vehicle rosters, and assigned commuters.
3. Transport Officer executes the route suspension command with specified effective start and expected resumption timestamps.
4. Route state transitions from **Active** to **Suspended**.
5. Fleet Dispatcher cancels all pending trips linked to the route during the suspension period.
6. Transport Management publishes the `TransportRouteSuspended` business event.
7. Institutional Services broadcasts emergency transit notifications to affected passengers.

---

## 10. Alternate Business Flows

- **Partial Route Diversion:** If only specific stops are blocked, the route may transition to **Diverted** rather than full suspension.
- **Immediate Restoration:** If the hazard clears early, an authorized restoration workflow is initiated.

---

## 11. Failure, Interruption and Business Recovery

- **Unreachable Commuters:** If notifications fail for certain channels, secondary SMS/App broadcast mechanisms are triggered.
- **Stranded Commuter Recovery:** Transport Management coordinates contingency shuttle support for passengers already in transit.

---

## 12. Business Rules and Decision Constraints

- **BR-G04-01 (Passenger Safety Priority):** Routes with verified safety hazards must be suspended immediately without administrative delay.
- **BR-G04-02 (Advance Notification Obligation):** Non-emergency planned suspensions require a minimum 24-hour advance broadcast to commuters.
- **BR-G04-03 (Audit Trail):** Every suspension order must record authorizer identity, reason code, and evidence reference.

---

## 13. State and Lifecycle Impact

- **Transport Route Lifecycle:** `Active` → `Suspended`.
- **Linked Trip Schedules:** `Scheduled` → `Cancelled`.

---

## 14. Business Events and Cross-Domain Collaboration

- **Emits:** `TransportRouteSuspended` (Publisher: Transport Management).
  - *Payload/Data:* Route ID, Route Code, Effective Date, Reason, Affected Stop IDs.
  - *Consumers:* Student Lifecycle (Section C), People & Workforce (Section E), Institutional Communications (Section H).

---

## 15. Business Authorization, Approval and Audit Obligations

- **Authorization Role:** Transport Manager / Head of Campus Operations.
- **Audit Obligation:** Immutable audit log of suspension authority, timestamp, and linked dispatch cancellations.

---

## 16. Completion Criteria and Postconditions

- Route state is officially recorded as **Suspended**.
- No new trips can be dispatched against the route while suspended.
- `TransportRouteSuspended` business event is emitted.

---

## 17. Open Issues, Assumptions and Deferred Questions

- **Assumptions:** Alternative public transit information is accessible to affected commuters.
- **Open Issues:** None.
