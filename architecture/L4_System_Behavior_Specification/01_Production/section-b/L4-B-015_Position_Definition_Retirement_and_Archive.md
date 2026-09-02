# Position Definition Retirement and Archive

## 1. Specification Metadata

**Specification ID:** L4-B-015  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institution & Structure — Organisation  
**Realizes Scenario:** L3-B-015  
**Primary Business Owner:** Organisation  
**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-015_Position_Definition_Retirement_and_Archive.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md`  
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall retire and subsequently archive PositionDefinition while preventing new assignment use and preserving historical relationships.

The source lifecycle is Active to Retired to Archived. Retirement does not automatically terminate or modify existing Section E assignments.

---

## 3. Required Business Context

- Canonical PositionDefinition in Active or Retired as applicable.
- Authoritative retirement/archive decision and impact evidence.
- Existing Section E-owned PositionAssignments where relevant.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-015-BH-001 | The system shall make an eligible Active PositionDefinition Retired only after required retirement evidence and decision are satisfactory. | L3-B-015 §§7,9,13,16 |
| L4-B-015-BH-002 | The system shall prevent new PositionAssignments from using a Retired PositionDefinition while leaving existing PositionAssignments unchanged unless their owning workflow produces a separate governed outcome. | L3-B-015 §§4,12-13,16 |
| L4-B-015-BH-003 | The system shall preserve existing assignment relationships and PositionDefinition history for interpretation. | L3-B-015 §§11-12,15-16 |
| L4-B-015-BH-004 | The system shall make a Retired PositionDefinition Archived only through the source-supported archival step. | L3-B-015 §§9,13,16 |
| L4-B-015-BH-005 | The system shall prevent shortcut reactivation from Retired or Archived through this scenario. | L3-B-015 §§4,12-13 |
| L4-B-015-BH-006 | The system shall preserve the archived PositionDefinition identifier and prevent unsupported reuse. | L3-B-015 §§11-12,16 |

---

## 5. Cross-Domain & Event Behavior

No additional exact canonical public event is established by the approved parent for this outcome. The system shall expose only the source-supported authoritative business fact or governed hand-off required by the scenario, without inventing an event name or publisher.

---

## 6. Failure / Rejection / Continuation

- Unresolved impact, missing authority/evidence, or invalid lifecycle position blocks retirement/archive and preserves the current state.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: PositionDefinition identity; retirement/archive decision; assignment-impact evidence; prior/resulting states; historical assignment relationships.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- Retirement completes at Retired with new assignment use prevented and history preserved.
- Archival completes at Archived without destructive assignment changes.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-015 — Position Definition Retirement and Archive  
**Primary verification source:** L2-B02 — Organisation Workflows (`L2-B02_Organisation_Workflows.md`)  

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No exact canonical PositionDefinition retirement/archive event is established; none is invented.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
