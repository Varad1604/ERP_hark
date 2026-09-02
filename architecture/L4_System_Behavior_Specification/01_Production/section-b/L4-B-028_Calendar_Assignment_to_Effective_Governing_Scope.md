# Calendar Assignment to Effective Governing Scope

## 1. Specification Metadata

**Specification ID:** L4-B-028  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institution & Structure — Academic Calendar  
**Realizes Scenario:** L3-B-028  
**Primary Business Owner:** Academic Calendar  
**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-028_Calendar_Assignment_to_Effective_Governing_Scope.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md`  
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall create, validate, approve, and activate a Calendar Assignment so a Published Calendar Version governs an explicit supported scope without ambiguous overlapping authority.

Activation of a Calendar Version alone does not govern a scope. This scenario establishes the Active Calendar Assignment and preserves historical assignment periods.

---

## 3. Required Business Context

- Published Calendar Version.
- Calendar Assignment targeting a source-supported Organisation, Campus, Programme, or Batch scope.
- Assignment validation/approval/effective context and existing overlapping assignments.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-028-BH-001 | The system shall accept a Calendar Assignment scope only when it is one of the source-supported Organisation, Campus, Programme, or Batch scope types. | L3-B-028 §§7,12 |
| L4-B-028-BH-002 | The system shall require the referenced Calendar Version to be Published before it can govern through an assignment. | L3-B-028 §§7,12 |
| L4-B-028-BH-003 | The system shall create and validate the Calendar Assignment against its Calendar Version, scope, effective period, and authoritative relationships. | L3-B-028 §§7,9,12 |
| L4-B-028-BH-004 | The system shall prevent a merely Approved Calendar Assignment from governing before it becomes Active. | L3-B-028 §§9,12-13 |
| L4-B-028-BH-005 | The system shall prevent ambiguous equal-precedence overlapping Active assignments for the same governing context. | L3-B-028 §§9,11-12 |
| L4-B-028-BH-006 | The system shall make the Assignment Active only after validation/approval and at its permitted effective point. | L3-B-028 §§3,9,13,16 |
| L4-B-028-BH-007 | The system shall preserve expired/ended assignments for historical resolution while preventing future governing effect. | L3-B-028 §§11-13,15-16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-B-028-BH-008 | The system shall publish `AssignmentCreated` when the governed Calendar Assignment is created; the canonical publisher remains Academic Calendar. | L3-B-028 §14; `L2-B05_Academic_Calendar_Workflows.md` event semantics |
| L4-B-028-BH-009 | The system shall publish `AssignmentValidated` when assignment validation succeeds; the canonical publisher remains Academic Calendar. | L3-B-028 §14; `L2-B05_Academic_Calendar_Workflows.md` event semantics |
| L4-B-028-BH-010 | The system shall publish `AssignmentApproved` when the assignment receives its governed approval; the canonical publisher remains Academic Calendar. | L3-B-028 §14; `L2-B05_Academic_Calendar_Workflows.md` event semantics |
| L4-B-028-BH-011 | The system shall publish `AssignmentActivated` when the Calendar Assignment becomes Active; the canonical publisher remains Academic Calendar. | L3-B-028 §14; `L2-B05_Academic_Calendar_Workflows.md` event semantics |

---

## 6. Failure / Rejection / Continuation

- Invalid scope/version/relationship, unresolved overlap, missing approval, or premature effective timing prevents the assignment from governing.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Assignment identity; Calendar Version; governing scope; validation/approval basis; effective period; overlap resolution basis; prior/resulting status; canonical assignment events.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- The Calendar Assignment is Active for an explicit supported scope with no unresolved authoritative overlap.
- AssignmentActivated is produced and historical assignment resolution remains possible.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-028 — Calendar Assignment to Effective Governing Scope  
**Primary verification source:** L2-B05 — Academic Calendar Workflows (`L2-B05_Academic_Calendar_Workflows.md`)  

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

The exact universal formula for resolving equal-precedence overlaps is not invented; only source-supported deterministic resolution may be used.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
