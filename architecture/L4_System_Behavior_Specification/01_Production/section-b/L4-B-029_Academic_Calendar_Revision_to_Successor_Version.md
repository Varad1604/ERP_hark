# Academic Calendar Revision to Successor Version

## 1. Specification Metadata

**Specification ID:** L4-B-029  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institution & Structure — Academic Calendar  
**Realizes Scenario:** L3-B-029  
**Primary Business Owner:** Academic Calendar  
**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-029_Academic_Calendar_Revision_to_Successor_Version.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md`  
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall realize a material Academic Calendar revision through an immutable successor Calendar Version while preserving Calendar identity, predecessor history, and assignment independence.

The successor is a new Calendar Version, not a new Calendar identity. Existing assignments are not automatically migrated and predecessor retirement belongs to B033.

---

## 3. Required Business Context

- Existing Academic Calendar identity and Published/Active predecessor Calendar Version.
- Proposed successor Calendar Version and authoritative revision/effective context.
- Existing Calendar Assignments remain separate aggregates and are not auto-migrated.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-029-BH-001 | The system shall prevent material revision from editing a Published Calendar Version in place. | L3-B-029 §§9,12-13 |
| L4-B-029-BH-002 | The system shall create a distinct successor Calendar Version under the stable Academic Calendar identity and preserve predecessor linkage. | L3-B-029 §§9,12-13 |
| L4-B-029-BH-003 | The system shall validate and publish the successor only after source-defined revision/approval conditions are satisfied. | L3-B-029 §§7,9,11-12 |
| L4-B-029-BH-004 | The system shall keep the predecessor authoritative for its valid context until the successor reaches its permitted effective/activation point. | L3-B-029 §§9-12,16 |
| L4-B-029-BH-005 | The system shall prevent automatic migration of existing Calendar Assignments to the successor. | L3-B-029 §§4,12,14 |
| L4-B-029-BH-006 | The system shall prevent successor activation from creating an unintended Calendar-resolution gap. | L3-B-029 §§9,11-12 |
| L4-B-029-BH-007 | The system shall preserve historical resolution using the version and assignment effective for the relevant historical scope/date. | L3-B-029 §§11-13,15-16 |
| L4-B-029-BH-008 | The system shall prevent automatic predecessor retirement as part of revision/activation. | L3-B-029 §§4,12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-B-029-BH-009 | The system shall publish `CalendarVersionCreated` when the successor Calendar Version is created; the canonical publisher remains Academic Calendar. | L3-B-029 §14; `L2-B05_Academic_Calendar_Workflows.md` event semantics |
| L4-B-029-BH-010 | The system shall publish `CalendarVersionPublished` when the successor Calendar Version becomes Published; the canonical publisher remains Academic Calendar. | L3-B-029 §14; `L2-B05_Academic_Calendar_Workflows.md` event semantics |
| L4-B-029-BH-011 | The system shall publish `CalendarVersionActivated` when the successor Calendar Version becomes Active; the canonical publisher remains Academic Calendar. | L3-B-029 §14; `L2-B05_Academic_Calendar_Workflows.md` event semantics |

---

## 6. Failure / Rejection / Continuation

- Invalid/stale successor, failed publication/activation conditions, or unresolved resolution gap blocks the relevant progression and preserves the current valid governing context.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Calendar identity; predecessor/successor Version identities; revision basis; publication/activation decisions; effective context; assignment impact; lineage; canonical version events.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- The successor Calendar Version is established and, where the scenario reaches activation, Active without modifying predecessor history or auto-migrating assignments.
- Predecessor retirement is not performed.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-029 — Academic Calendar Revision to Successor Version  
**Primary verification source:** L2-B05 — Academic Calendar Workflows (`L2-B05_Academic_Calendar_Workflows.md`)  

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No automatic assignment migration or predecessor retirement is introduced.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
