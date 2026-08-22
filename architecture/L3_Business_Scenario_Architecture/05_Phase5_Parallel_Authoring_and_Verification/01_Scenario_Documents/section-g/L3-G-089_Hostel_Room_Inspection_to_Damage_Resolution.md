# Hostel Room Inspection to Damage Resolution

**Scenario ID:** L3-G-089  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, F, H  
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
| Final L3 ID | L3-G-089 |
| Canonical Title | Hostel Room Inspection to Damage Resolution |
| Primary Owner | Hostel Management |
| Participating Domains | G, F, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern a hostel room inspection from inspection through an authoritative damage-related resolution while preserving G10 ownership of the physical room, G03 ownership of hostel occupancy/inspection-related hostel operations, and Facilities ownership of maintenance execution where required.

## 3. Business Outcome

A hostel room inspection identifies and resolves a damage condition through the applicable business process, with the resulting room condition and required operational follow-up recorded and communicated to applicable consumers.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a hostel room inspection is initiated against an occupied or otherwise applicable hostel room.

### 4.2 End Boundary

The scenario ends when the damage finding has reached its governed resolution and the required hostel/infrastructure history is recorded.

### 4.3 In Scope

- Inspection initiation and assignment.
- Assessment of room condition.
- Recording damage findings.
- Determination of the applicable damage resolution.
- Maintenance recommendation/hand-off where repair is required.
- Relevant financial consumption where the authoritative architecture requires it.
- Closure and historical evidence.

### 4.4 Out of Scope

- Ownership of the physical room or infrastructure.
- Facilities work-order execution.
- Financial accounting or liability recognition.
- Student lifecycle changes.
- Technical inspection tooling, evidence storage, APIs, databases, or UI.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Hostel Management (G03) | Owns hostel occupancy/room operational context and hostel-side inspection/damage resolution process. |
| Infrastructure Management (G10) | Owns the physical hostel room and infrastructure lifecycle. |
| Facilities Management (G01) | Owns maintenance execution when damage requires repair/service. |
| Finance (F) | Participates where a financial consequence is consumed/processed under Finance ownership. |
| Notifications (H) | Supports governed notification of the authoritative business outcome where configured. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Hostel Administrator | Initiates or manages the room inspection process. |
| Hostel Warden | Provides hostel governance and decision authority where configured. |
| Resident / Student | May be the resident associated with the inspected room and relevant damage context. |
| Facilities operational role | Executes maintenance when a repair path is required. |
| Finance authority | Owns any financial accounting consequence; G03 does not perform accounting. |

No additional damage-assessment or financial authority is invented.

## 7. Preconditions and Entry Criteria

- The room is a valid G10-owned physical space.
- The room is within Hostel Management's operational accommodation scope.
- An inspection requirement exists under the applicable hostel process.
- Where a resident is relevant, the resident context is valid.

## 8. Trigger

A hostel room inspection is initiated.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Hostel Management initiates the inspection against the canonical room and applicable hostel context. | G03 | Inspection enters its governed inspection process. |
| 2 | The room condition is assessed and findings are recorded. | Hostel Management | Any damage finding is established as a business fact. |
| 3 | Hostel Management determines the applicable damage resolution path. | G03 | The damage is classified for operational resolution. |
| 4 | Where repair/service is required, the maintenance requirement is handed to Facilities. | G03 → G01 | Facilities owns the maintenance execution. |
| 5 | Where a financial consequence is applicable, the authoritative damage outcome is made available to Finance without transferring accounting ownership to G03. | G03 → F | Finance determines and records the financial consequence under its own authority. |
| 6 | Hostel Management records the final room-damage resolution and completes the inspection outcome. | G03 with G10 context | Damage resolution is historically traceable. |
| 7 | The authoritative outcome is made available to Notifications where governed communication is required. | G03 → H02 | Applicable stakeholders can receive the business outcome. |

**Scenario End:** The flow stops when the damage resolution and inspection outcome are recorded.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| 1 | Inspection identifies no damage requiring resolution. | Record the inspection finding and complete the inspection without a damage-recovery path. | Inspection closes without a damage action. | G03 |
| 2 | Damage requires maintenance but no financial consequence is established by the applicable business policy. | Complete the maintenance resolution while retaining the damage history. | Room condition is resolved without asserting an unsupported financial outcome. | G03 / G01 |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Inspection cannot establish a reliable room condition. | Damage resolution cannot be concluded. | Complete the inspection through the applicable governed assessment path before closure. | G03 |
| Damage requires repair. | Hostel Management cannot execute maintenance. | Hand the maintenance requirement to Facilities while retaining the hostel damage context. | G03 / G01 |
| A financial consequence is disputed or not yet determined. | Financial resolution cannot be concluded. | Preserve the operational damage outcome and allow Finance to determine its own accounting/financial result. | F / G03 |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Physical rooms are owned by G10. | G10 | Inspection references the room but does not transfer physical ownership. | L2-G03 §4; L2-G10 |
| Facilities owns maintenance execution. | G01 | Damage repair is handed to Facilities rather than executed by Hostel Management. | L2-G03 §4; L2-G01 |
| Hostel Management maintains residency and operational history. | G03 | Damage/inspection context remains traceable within hostel operations. | L2-G03 §5 |
| Finance owns financial accounting. | F | G03 does not determine or record accounting as its own lifecycle. | L2-G03 §4; L1 |
| Every significant operational transition is audited. | G03 / G10 / G01 as applicable | Inspection and resulting resolution remain historically traceable. | L2-G03 / G10 / G01 |

## 13. State and Lifecycle Impact

The authoritative G03 extract available for this authoring pass does not expose a dedicated Hostel Room Inspection state matrix. G10 separately defines an Inspection lifecycle:

`Scheduled → Assigned → Inspection → Findings → Approved → Closed`

| Aggregate / Lifecycle | Impact | Owner |
|---|---|---|
| Hostel-side room inspection context | Inspection is initiated, findings are recorded, and the damage resolution is completed. | G03 |
| Infrastructure inspection record, where the G10 inspection workflow applies | Follows the G10 inspection lifecycle. | G10 |
| Facilities Work Order, if created through the approved maintenance path | Follows the G01 maintenance lifecycle. | G01 |

No new damage-specific state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G03, G10, and G01 establish event-driven publication for significant operational transitions. Exact event names for this combined room-inspection/damage outcome were not exposed in the retrieved authoritative material.

| Business Fact | Publisher | Consumer |
|---|---|---|
| Hostel inspection/damage outcome | G03 | Finance, Notifications, and other approved consumers as applicable |
| Infrastructure inspection outcome, where G10 owns the inspection record | G10 | Applicable operational consumers |
| Maintenance completion, where Facilities executes repair | G01 | Hostel Management and other approved consumers |

No event name is fabricated.

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| G10 | G03 | Canonical physical room context | Identify the physical room without transferring infrastructure ownership. |
| G03 | G01 | Maintenance requirement | Route repair execution to Facilities. |
| G03 | F | Damage outcome / financial reference | Allow Finance to determine its own financial consequence where applicable. |
| G03 | H02 | Authoritative outcome | Support governed notification. |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Hostel Management determines the hostel-side inspection outcome and applicable operational resolution under configured hostel policy.

G10 retains authority over physical infrastructure lifecycle.

G01 retains authority over maintenance execution.

Finance retains authority over financial accounting outcomes.

### 15.2 Separation of Duty / Override

No room-damage-specific separation-of-duty rule is exposed in the retrieved authoritative material. No additional rule is invented.

### 15.3 Audit / Evidence

The inspection finding, damage determination, resolution, maintenance hand-off where applicable, and final closure must remain historically traceable.

Any financial outcome is evidenced within Finance's own authoritative records.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The canonical room is identified.
- Inspection is completed under the applicable inspection process.
- Damage is either not found or is governed through an appropriate resolution path.
- Required maintenance hand-off is completed where applicable.
- Any applicable financial consequence is handed to Finance without ownership transfer.
- The final inspection/damage resolution is recorded and auditable.

### 16.2 Postconditions

- The hostel damage/inspection outcome is historically traceable.
- Physical room ownership remains with G10.
- Maintenance ownership remains with G01.
- Financial accounting ownership remains with F.
- Notification participation does not transfer business ownership.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| Exact G03 room-inspection event names and the precise boundary between G03 hostel inspection and G10 infrastructure inspection were not fully exposed in the retrieved authoritative material. | The scenario can preserve the ownership boundary but cannot safely assert a more specific event/state mapping. | Confirm the applicable G03/G10 inspection contract and event catalogue before Architecture Approval/Lock. |
| Exact financial damage/recovery contract is not exposed in the retrieved G03/G01 extracts. | No specific financial obligation or event is invented. | Confirm the authoritative F-side contract if a financial outcome is mandatory for this scenario. |

### 17.2 Assumptions

Where an inspection is a G03 hostel operational inspection, G03 owns the hostel-side business outcome while G10 remains owner of the physical room.

### 17.3 Deferred Questions

Technical inspection evidence, maintenance execution mechanics, financial accounting mechanics, and notification delivery remain outside L3.
