# Curriculum Revision to Immutable Successor Programme Version

## 1. Specification Metadata

**Specification ID:** L4-B-024  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institution & Structure — Curriculum Framework  
**Realizes Scenario:** L3-B-024  
**Primary Business Owner:** Curriculum Framework  
**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-024_Curriculum_Revision_to_Immutable_Successor_Programme_Version.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md`  
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall realize a material Curriculum revision through an immutable successor Programme Version while preserving predecessor history and source-defined applicability.

The existing Published version is not edited in place. Existing cohorts do not automatically migrate and Section D cannot redefine B04 static definitions.

---

## 3. Required Business Context

- Published predecessor Programme Version/Curriculum.
- Proposed successor Programme Version linked to predecessor.
- Applicability scope/time/cohort rules, validation/approval evidence, and effective context.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-024-BH-001 | The system shall prevent material revision from modifying an existing Published Programme Version/Curriculum in place. | L3-B-024 §§9,12-13 |
| L4-B-024-BH-002 | The system shall create a distinct governed successor Programme Version linked to its Published predecessor. | L3-B-024 §§9,12-13 |
| L4-B-024-BH-003 | The system shall keep the predecessor authoritative while the successor is proposed, rejected, or not yet effective. | L3-B-024 §§9-11,16 |
| L4-B-024-BH-004 | The system shall make the successor Published/effective only after source-defined validation, approval, and applicability conditions are satisfied. | L3-B-024 §§7,9,12,16 |
| L4-B-024-BH-005 | The system shall preserve the predecessor and successor lineage for historical interpretation. | L3-B-024 §§11-13,15-16 |
| L4-B-024-BH-006 | The system shall prevent automatic migration of existing cohorts to the successor without explicit source-supported applicability rules. | L3-B-024 §§4,12,14 |
| L4-B-024-BH-007 | The system shall prevent a stale successor proposal from being applied against newer authoritative academic/regulatory facts. | L3-B-024 §11 |

---

## 5. Cross-Domain & Event Behavior

No additional exact canonical public event is established by the approved parent for this outcome. The system shall expose only the source-supported authoritative business fact or governed hand-off required by the scenario, without inventing an event name or publisher.

---

## 6. Failure / Rejection / Continuation

- Rejected, invalid, inapplicable, or stale successor proposals leave the predecessor authoritative for its valid scope.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Predecessor/successor version identities; revision basis; applicability scope/time/cohort evidence; approval/publication result; effective context; lineage.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- A distinct successor Programme Version is Published/effective for its governed applicability.
- The predecessor remains historically resolvable and existing cohorts are not automatically migrated.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-024 — Curriculum Revision to Immutable Successor Programme Version  
**Primary verification source:** L2-B04 — Curriculum Framework Workflows (`L2-B04_Curriculum_Framework_Workflows.md`)  

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No exact canonical public event for successor Programme Version revision/publication is established; none is invented.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
