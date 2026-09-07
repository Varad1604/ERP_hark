# Academic Scheduling Requirement to Governed Operational Timetable

## 1. Specification Metadata

**Specification ID:** L4-D-020
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Timetable
**Realizes Scenario:** L3-D-020
**Primary Business Owner:** Timetable
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-020_Academic_Scheduling_Requirement_to_Governed_Operational_Timetable.md`; `L2-D03_Timetable_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Transform authoritative academic, faculty and resource planning inputs into one validated, approved, published and Active Timetable for the governed scope.

**Start boundary:** Approved academic planning prerequisites and an authorized scheduling requirement are available.

**End boundary:** The Timetable reaches Active and is the canonical operational scheduling truth for the scope.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Approved Academic Calendar and Course Offerings
- Complete Faculty Assignments
- Authoritative Section G resource availability
- Scheduling constraints/policy and conflict/reservation context

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-020-BH-001 | The system shall verify authoritative academic, faculty and resource prerequisites before timetable planning. | L3-D-020 §§3–13 |
| L4-D-020-BH-002 | The system shall establish the timetable planning/draft position without changing source-domain masters. | L3-D-020 §§3–13 |
| L4-D-020-BH-003 | The system shall generate/record a Draft timetable under the approved business constraints without granting the system governance authority. | L3-D-020 §§3–13 |
| L4-D-020-BH-004 | The system shall make the applicable faculty, resource, calendar, curriculum, capacity, policy and conflict evidence available for governed validation and prevent approval until the authoritative validation outcome confirms that publication blockers are resolved. | L3-D-020 §§3–13 |
| L4-D-020-BH-005 | The system shall prevent publication while critical conflicts/reservations remain unresolved. | L3-D-020 §§3–13 |
| L4-D-020-BH-006 | The system shall recognize the Academic Administrator approval/rejection decision without becoming the approver. | L3-D-020 §§3–13 |
| L4-D-020-BH-007 | The system shall publish only an approved immutable timetable version. | L3-D-020 §§3–13 |
| L4-D-020-BH-008 | The system shall recognize the Registrar publication/activation decision and transition Published → Active only at the authorized operating window. | L3-D-020 §§3–13 |
| L4-D-020-BH-009 | The system shall maintain only one Active timetable for the governed scope. | L3-D-020 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-020-BH-010 | The system shall publish `TimetablePlanningStarted` / `TimetablePlanningCompleted` as their governed planning facts occur. | L3-D-020 §14; L2-D03_Timetable_Workflows.md exact event semantics |
| L4-D-020-BH-011 | The system shall publish `TimetableDraftCreated` when the Draft exists. | L3-D-020 §14; L2-D03_Timetable_Workflows.md exact event semantics |
| L4-D-020-BH-012 | The system shall publish `TimetableValidated` when validation succeeds. | L3-D-020 §14; L2-D03_Timetable_Workflows.md exact event semantics |
| L4-D-020-BH-013 | The system shall publish `TimetableApproved` or `TimetableRejected` according to the authoritative decision. | L3-D-020 §14; L2-D03_Timetable_Workflows.md exact event semantics |
| L4-D-020-BH-014 | The system shall publish `TimetablePublished` when publication completes. | L3-D-020 §14; L2-D03_Timetable_Workflows.md exact event semantics |
| L4-D-020-BH-015 | The system shall publish `TimetableActivated` when the timetable becomes Active. | L3-D-020 §14; L2-D03_Timetable_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume Section B academic/calendar, D01 Course Offering, D02/Section E faculty-assignment and Section G resource facts without ownership transfer.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Invalid prerequisites prevent planning/progression.
- Unresolved critical conflicts prevent approval/publication.
- Rejected approval preserves the prior timetable position.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve source references, conflict/reservation evidence, timetable version, validation, approval/publication/activation authority and exact lifecycle events.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- a validated approved immutable timetable version is Published
- the timetable is Active for the governed scope
- only one Active timetable exists for that scope
- `TimetableActivated` is published

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-020` — Academic Scheduling Requirement to Governed Operational Timetable
**Primary verification source:** `L2-D03_Timetable_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

