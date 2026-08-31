# Preventive Maintenance Governance to Infrastructure Maintenance Requirement

## 1. Specification Metadata

**Specification ID:** L4-G-304  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-304  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-304_Preventive_Maintenance_Governance_to_Infrastructure_Maintenance_Requirement.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall record a source-supported G10 preventive-maintenance governance requirement against canonical Infrastructure while preserving G01 ownership of Preventive Maintenance Plan, Maintenance Schedule, Work Orders and operational maintenance execution. The behavior stops before resolving the conflicting schedule ownership and `PreventiveMaintenanceScheduled` publisher semantics.

## 3. Required Business Context

- Canonical physical Infrastructure exists under G10 ownership.
- Infrastructure condition or applicable policy establishes a preventive-maintenance need.
- The requirement is distinguishable from corrective and emergency maintenance.
- G01 retains ownership of operational preventive-maintenance planning/scheduling aggregates and execution.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-304-BH-001 | The system shall validate the canonical Infrastructure and source-supported preventive-maintenance need before recording the G10 governance requirement. | L3-G-304 §§7,9,12 |
| L4-G-304-BH-002 | The system shall distinguish the preventive requirement from source-defined corrective or emergency maintenance conditions before continuing on this path. | L3-G-304 §§9–10 |
| L4-G-304-BH-003 | The system shall record the G10 Infrastructure-side governance requirement without creating or re-owning a G01 Preventive Maintenance Plan, Maintenance Schedule or Work Order. | L3-G-304 §§2–5,9,12–13 |
| L4-G-304-BH-004 | The system shall make the source-supported Infrastructure maintenance requirement available for Facilities-owned operational planning and execution without transferring physical-Infrastructure ownership. | L3-G-304 §§4–5,9,14,16 |

## 5. Cross-Domain & Event Behavior

Both Locked G10 and G01 define `PreventiveMaintenanceScheduled` with different business meanings. The system shall not publish or reinterpret that event through this L4 specification because doing so would silently choose a canonical publisher/semantic. No replacement event name or exact hand-off mechanism is invented.

## 6. Failure / Rejection / Continuation

A requirement that is corrective or emergency shall not continue as preventive maintenance governance. Where the G10/G01 schedule boundary or event publisher is material to further progression, the system shall stop at the recorded G10 governance requirement and preserve Facilities execution ownership rather than create a competing schedule or event.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-304-BH-005 | The system shall preserve the canonical Infrastructure reference, maintenance category, G10 governance requirement and cross-domain hand-off evidence while keeping G10 governance history distinct from G01 execution history. | L3-G-304 §§15–16 |

## 8. Observable Completion Conditions

The source-supported portion is complete when the preventive-maintenance requirement is recorded against canonical Infrastructure and is available to Facilities Management without creation of G01-owned execution aggregates by G10. Full canonical scheduling/event completion remains unresolved until upstream architecture defines the G10/G01 schedule boundary and one-publisher event semantics.

## 9. Traceability & Provenance

**Parent:** L3-G-304 — Preventive Maintenance Governance to Infrastructure Maintenance Requirement  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows  
**Supporting verification source:** L2-G01 — Facilities Management Workflows  
**Later amendment check:** `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` does not resolve preventive-maintenance plan/schedule ownership or the duplicated event semantics.

## 10. Open Issues / Deferred Design

G10 preventive-maintenance governance and G01 Preventive Maintenance Plan/Maintenance Schedule ownership remain insufficiently distinguished, and `PreventiveMaintenanceScheduled` has conflicting Locked semantics/publishers. No exact cross-domain contract name is defined. L4 preserves these gaps; implementation interface, persistence, event transport and UI design remain deferred.
