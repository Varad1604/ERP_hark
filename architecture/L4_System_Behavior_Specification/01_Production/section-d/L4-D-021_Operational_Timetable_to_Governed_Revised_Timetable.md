# Operational Timetable to Governed Revised Timetable

## 1. Specification Metadata

**Specification ID:** L4-D-021
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Timetable
**Realizes Scenario:** L3-D-021
**Primary Business Owner:** Timetable
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-021_Operational_Timetable_to_Governed_Revised_Timetable.md`; `L2-D03_Timetable_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Govern material revision of an operational timetable through impact analysis, distinct version creation, validation, approval and republication without editing historical published versions.

**Start boundary:** An authorized revision request exists for an operational timetable with an immutable published version.

**End boundary:** A distinct revised timetable version reaches Published and becomes the governed operational timetable while preceding versions remain immutable.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Operational/published timetable
- Revision reason and affected scope
- Authoritative faculty/resource/academic impact facts
- Revision validation/approval/publication authority

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-021-BH-001 | The system shall register the revision request and affected scope against the current authoritative timetable. | L3-D-021 §§3–13 |
| L4-D-021-BH-002 | The system shall use authoritative academic/faculty/resource facts for impact analysis without re-owning them. | L3-D-021 §§3–13 |
| L4-D-021-BH-003 | The system shall create a distinct revised timetable version and leave the prior published version unchanged. | L3-D-021 §§3–13 |
| L4-D-021-BH-004 | The system shall present the revised timetable and applicable constraint, conflict and reservation evidence for governed validation and prevent approval until an authoritative validation outcome permits progression. | L3-D-021 §§3–13 |
| L4-D-021-BH-005 | The system shall prevent republication when validation fails or revision approval is rejected. | L3-D-021 §§3–13 |
| L4-D-021-BH-006 | The system shall recognize Academic Administrator approval and Registrar republication authority without transferring that authority to the system. | L3-D-021 §§3–13 |
| L4-D-021-BH-007 | The system shall publish the approved revised version while preserving complete version lineage. | L3-D-021 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-021-BH-008 | The system shall publish `TimetableRevisionCreated` when the distinct revised timetable version is established. | L3-D-021 §14; L2-D03_Timetable_Workflows.md exact event semantics |
| L4-D-021-BH-009 | The system shall publish `TimetableRepublished` when the approved revised version is published. | L3-D-021 §14; L2-D03_Timetable_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume D02/Section E Faculty Assignment/faculty facts and Section G resource-availability facts for impact analysis without ownership transfer.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Validation/reference failure leaves the prior published timetable authoritative.
- Rejected/invalid approval leaves the revision unpublished.
- Interrupted revision preserves the last valid business position and does not imply republication.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve prior/revised versions, revision rationale/scope, impact evidence, validation/conflicts, approval/republication authority and exact revision facts.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- a new approved revised version is Published
- all preceding published versions remain immutable/traceable
- `TimetableRepublished` is published
- activation remains subject to the separate authorized operating window stated by the parent

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-021` — Operational Timetable to Governed Revised Timetable
**Primary verification source:** `L2-D03_Timetable_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

