# Infrastructure Lifecycle Initiation to Active Infrastructure State

## 1. Specification Metadata

**Specification ID:** L4-G-314  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-314  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-314_Infrastructure_Lifecycle_Initiation_to_Active_Infrastructure_State.md`; L2-G10 — Infrastructure Management Workflows; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall govern progression of registered and validated physical Infrastructure only through an authoritative G10 lifecycle toward the source-defined operational position. For newly established Building, Floor and Space aggregates, the later Locked G10 amendment establishes `Proposed` as the canonical initial lifecycle position; that amendment does not resolve every other G10 lifecycle or completion-event ambiguity.

## 3. Required Business Context

- The physical Infrastructure aggregate exists under G10 ownership and its exact aggregate type is known.
- For newly established Building, Floor or Space, the canonical initial externally authoritative lifecycle position is `Proposed` under the 2026-08-26 Locked amendment.
- Required parent hierarchy, metadata and operational validations are available.
- The lifecycle action is authorized.
- The exact requested transition must belong to the authoritative lifecycle applicable to the aggregate.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-314-BH-001 | The system shall resolve the aggregate type and authoritative current lifecycle position before progression; a newly established Building, Floor or Space shall begin at `Proposed` under the later Locked G10 amendment. | L3-G-314 §§7,9,12–13; Locked G10 amendment §2 |
| L4-G-314-BH-002 | The system shall validate parent hierarchy, required metadata, operational prerequisites and authorization before allowing a lifecycle transition. | L3-G-314 §§7,9,11–12 |
| L4-G-314-BH-003 | The system shall prevent any transition that skips or invents a lifecycle position and shall apply only a transition supported for the exact aggregate by authoritative G10 architecture. | L3-G-314 §§9,11–13 |
| L4-G-314-BH-004 | When an authoritative lifecycle path validly reaches `Operational`, the system shall expose `Operational` as the current lifecycle position and shall not treat the resource as operational before that position is reached. | L3-G-314 §§2–3,7,9,12–13,16 |

## 5. Cross-Domain & Event Behavior

G10 uses `InfrastructureActivated`, `InfrastructureLifecycleChanged` and `InfrastructureOperational` for related lifecycle/activation facts without one unambiguous completion-event rule for this generic scenario. The system shall not select or rename a completion event merely to close the specification. Any event produced must correspond to an exact source-defined transition whose semantics are unambiguous for the affected aggregate.

## 6. Failure / Rejection / Continuation

Failed validation or authorization shall preserve the last valid lifecycle position. If the aggregate-specific lifecycle path remains contradictory or undefined, the system shall stop progression rather than apply a neighboring aggregate's lifecycle or use legacy `Draft`/`Commissioned` vocabulary by assumption. The later `Proposed` clarification shall be applied only to Building, Floor and Space establishment, not generalized beyond its Locked scope.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-314-BH-005 | The system shall preserve the aggregate type, prior and resulting lifecycle positions, validation evidence, authorization basis and the governing amendment/source for each accepted transition. | L3-G-314 §§15–16 |

## 8. Observable Completion Conditions

For an aggregate with an unambiguous authoritative lifecycle, behavior is complete when every required validation passes, each transition is source-supported and the aggregate reaches `Operational` without skipped or invented positions. The Building/Floor/Space initial-state ambiguity is resolved to `Proposed`; remaining generic lifecycle and completion-event inconsistencies still prevent one universal progression rule for every G10 aggregate.

## 9. Traceability & Provenance

**Parent:** L3-G-314 — Infrastructure Lifecycle Initiation to Active Infrastructure State  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows  
**Superseding verification source:** `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` §2  
**Supporting verification sources:** L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows

## 10. Open Issues / Deferred Design

The later Locked amendment resolves the obsolete registration-state ambiguity only for newly established Building, Floor and Space by establishing `Proposed`. Other G10 lifecycle variants and the relationship among `InfrastructureActivated`, `InfrastructureLifecycleChanged` and `InfrastructureOperational` remain insufficiently reconciled for one generic completion rule. L4 does not normalize them. API/interface, persistence, event transport, technical authorization and UI design remain deferred.
