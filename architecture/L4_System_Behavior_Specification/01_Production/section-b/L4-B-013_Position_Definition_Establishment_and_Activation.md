# Position Definition Establishment and Activation

## 1. Specification Metadata

**Specification ID:** L4-B-013  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institution & Structure — Organisation  
**Realizes Scenario:** L3-B-013  
**Primary Business Owner:** Organisation  
**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-013_Position_Definition_Establishment_and_Activation.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md`  
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall establish and activate a canonical PositionDefinition owned by Organisation without creating an occupant or PositionAssignment.

The scenario establishes the structural PositionDefinition only; employee/person assignment belongs to Section E.

---

## 3. Required Business Context

- Eligible Organisation context.
- Proposed PositionDefinition and required structural/governance evidence.
- Source-defined PositionDefinition lifecycle and activation decision.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-013-BH-001 | The system shall establish a new PositionDefinition in Draft without creating a PositionAssignment or employee/person occupant. | L3-B-013 §§5,9,12 |
| L4-B-013-BH-002 | The system shall permit PositionDefinition progression only through the source-defined Draft, Under Review, Approved, and Active lifecycle positions. | L3-B-013 §§9,13 |
| L4-B-013-BH-003 | The system shall validate eligible Organisation context before allowing PositionDefinition to become Active. | L3-B-013 §§7,9,12 |
| L4-B-013-BH-004 | The system shall prevent activation when mandatory definition, structural, approval, or dependency evidence is unsatisfactory. | L3-B-013 §11 |
| L4-B-013-BH-005 | The system shall make an approved eligible PositionDefinition Active only through the canonical lifecycle path. | L3-B-013 §§3,9,13,16 |
| L4-B-013-BH-006 | When the PositionDefinition becomes Active, the system shall not create or imply an employee/person occupant or Section E PositionAssignment. | L3-B-013 §§4-5,12,14 |

---

## 5. Cross-Domain & Event Behavior

No additional exact canonical public event is established by the approved parent for this outcome. The system shall expose only the source-supported authoritative business fact or governed hand-off required by the scenario, without inventing an event name or publisher.

---

## 6. Failure / Rejection / Continuation

- Missing/invalid context, definition evidence, or approval prevents activation and retains the last valid PositionDefinition state.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: PositionDefinition identity; Organisation context; lifecycle progression; validation/approval evidence; prior/resulting state.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- The canonical PositionDefinition is Active and structurally available for governed use.
- No occupant or PositionAssignment is created by this outcome.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-013 — Position Definition Establishment and Activation  
**Primary verification source:** L2-B02 — Organisation Workflows (`L2-B02_Organisation_Workflows.md`)  

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No exact canonical PositionDefinition activation event is established; none is invented.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
