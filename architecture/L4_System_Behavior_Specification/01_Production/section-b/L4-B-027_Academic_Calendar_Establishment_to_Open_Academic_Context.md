# Academic Calendar Establishment to Open Academic Context

## 1. Specification Metadata

**Specification ID:** L4-B-027
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Institution & Structure — Academic Calendar
**Realizes Scenario:** L3-B-027
**Primary Business Owner:** Academic Calendar
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-027_Academic_Calendar_Establishment_to_Open_Academic_Context.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall establish, publish, and activate a Calendar Version to provide an open academic temporal context without automatically assigning it to every scope or starting downstream processes.

The Calendar Version must be Published before Active. Activation does not create Calendar Assignments and does not start Admissions, teaching, attendance, exams, payroll, scheduling, or other downstream operations.

---

## 3. Required Business Context

- Canonical Academic Calendar and Calendar Version definition.
- Required validation/approval/publication evidence and effective activation context.
- Assignment is a separate B028 responsibility.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-027-BH-001 | The system shall prevent Calendar Version activation before the version has reached Published. | L3-B-027 §§7,12-13 |
| L4-B-027-BH-002 | The system shall validate mandatory Calendar Version definition, approval, and publication conditions before publication. | L3-B-027 §§7,9,11-12 |
| L4-B-027-BH-003 | The system shall make the valid Calendar Version Published when publication conditions are satisfied. | L3-B-027 §§9,13,16 |
| L4-B-027-BH-004 | The system shall validate source-defined activation conditions before making a Published Calendar Version Active. | L3-B-027 §§7,9,11-12 |
| L4-B-027-BH-005 | The system shall make the Published Calendar Version Active only at its permitted effective point. | L3-B-027 §§3,9,13,16 |
| L4-B-027-BH-006 | The system shall activate the Calendar Version without creating or activating Calendar Assignments as part of this scenario. | L3-B-027 §§4,12,14 |
| L4-B-027-BH-007 | The system shall expose the Active Calendar Version as temporal context only and shall not derive commencement or readiness of downstream institutional processes from activation alone. | L3-B-027 §§4,12,16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-B-027-BH-008 | The system shall publish `CalendarVersionPublished` when the Calendar Version becomes Published; the canonical publisher remains Academic Calendar. | L3-B-027 §14; `L2-B05_Academic_Calendar_Workflows.md` event semantics |
| L4-B-027-BH-009 | The system shall publish `CalendarVersionActivated` when the Published Calendar Version becomes Active; the canonical publisher remains Academic Calendar. | L3-B-027 §14; `L2-B05_Academic_Calendar_Workflows.md` event semantics |

---

## 6. Failure / Rejection / Continuation

- Invalid definition, missing approval/publication evidence, or failed activation conditions block progression and preserve the current valid Calendar Version state.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Calendar and Calendar Version identities; definition/validation/approval basis; publication and activation effective context; prior/resulting state; canonical events.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- The Calendar Version is Active after being Published.
- CalendarVersionPublished and CalendarVersionActivated are produced at their respective milestones.
- No Calendar Assignment or downstream process start is implied.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-027 — Academic Calendar Establishment to Open Academic Context
**Primary verification source:** L2-B05 — Academic Calendar Workflows (`L2-B05_Academic_Calendar_Workflows.md`)

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No automatic assignment/readiness behavior may be absorbed from B028 or downstream domains.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
