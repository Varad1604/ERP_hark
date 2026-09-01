# L2-G10 — Infrastructure Blocker Remediation — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-G10 — Infrastructure Management Workflows and locked 2026-08-26 amendments |
| Amendment Version | 1.3.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 |

## 1. Zone establishment

For the G10 Zone aggregate:

- canonical establishment event: `ZoneCreated`;
- canonical initial lifecycle position: `Proposed`;
- Zone lifecycle: `Proposed → Operational → Archived` unless another already-locked Zone-specific transition applies;
- generic `Draft` registration wording does not define the Zone lifecycle;
- `ZoneRegistered` describing the same establishment fact is legacy vocabulary and shall not be published as a duplicate event.

`AssignedToZone` remains canonical for Zone Membership establishment.

## 2. Space classification

Initial Space Type is established during Space registration because a canonical Space requires a primary Space Type.

The separate Space Classification workflow governs **reclassification of an existing Space**. Reclassification:

- changes the primary Space Type only;
- does not change Space identity, hierarchy or physical lifecycle;
- publishes `SpaceClassificationChanged` from G10.

`InfrastructureClassificationChanged` shall not be used for this Space-specific fact.

## 3. Infrastructure Inspection versus Facilities Inspection

G10 owns **Infrastructure Inspection** when the subject is the physical Infrastructure condition, structural/physical compliance, criticality or G10-owned Infrastructure governance record. G10 owns its Inspection Record, scheduling, findings, approval and closure.

G01 owns **Facilities Inspection** when the subject is facilities operational service/execution under the Facilities bounded context. G01 retains its separate Inspection aggregate and `FacilityInspection*` events.

For G10 Infrastructure Inspection:

- Infrastructure Administrator establishes/schedules the Inspection Record;
- an authorized Infrastructure Inspector conducts the inspection;
- Infrastructure Manager is the canonical approval authority for closure;
- G10 retains `InfrastructureInspectionScheduled`, `FindingRecorded`, and `InfrastructureInspectionCompleted` for its Inspection Record.

A finding requiring corrective maintenance is handed to G01 without moving Infrastructure ownership. A material risk-only finding is referred to G00; G00 alone creates the Operational Risk.

## 4. Preventive maintenance boundary

G10 owns the physical-Infrastructure **preventive-maintenance requirement**. It does not own the G01 Preventive Maintenance Plan, Maintenance Schedule, Work Order or execution lifecycle.

When G10 records a governed preventive-maintenance requirement it publishes:

- `InfrastructurePreventiveMaintenanceRequired`

G01 alone uses `PreventiveMaintenanceScheduled` for creation/scheduling of its operational maintenance plan/schedule.

## 5. Emergency maintenance boundary

G10 owns the physical-Infrastructure emergency condition and requirement. When urgent maintenance is required G10 publishes:

- `InfrastructureEmergencyMaintenanceRequired`

G01 owns the operational emergency-maintenance response. Existing G01 `EmergencyMaintenanceDeclared` represents initiation/declaration of the Facilities-owned emergency response and shall not be published by G10.

## 6. Disaster/major-disruption recovery

No separate SutraERP bounded context named `Disaster Management` is created by G10 prose.

For physical Infrastructure affected by disaster or major disruption:

- G10 owns Infrastructure identity, condition, availability, lifecycle and recovery metadata;
- recovery uses only existing permitted G10 lifecycle/availability transitions, including temporary closure/restoration where applicable;
- unrecoverable Infrastructure follows the separately governed decommissioning path;
- G00 owns cross-campus recovery governance when the disruption meets the configured governance threshold;
- operational response work remains owned by the applicable operational domain.

G10 records recovery status as governed Infrastructure metadata. No generic `Recovered` lifecycle state is introduced.

## 7. Archival

The canonical generic physical-Infrastructure archival entry path is:

`Decommissioned → Archived`

The phrase “otherwise eligible” means that all archival eligibility conditions must be satisfied; it does not authorize another lifecycle entry state.

Infrastructure Administrator initiates archival evaluation. **Infrastructure Manager** is the canonical approval authority. `InfrastructureArchived` remains the canonical G10 archival event.

## 8. Downstream effect

This amendment resolves the source blockers recorded by L3-G-285, L3-G-286, L3-G-303, L3-G-304, L3-G-306, L3-G-313 and L3-G-316.
