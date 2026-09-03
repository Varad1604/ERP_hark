# Faculty Academic Assignment Establishment

## 1. Specification Metadata

**Specification ID:** L4-D-017
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Faculty Academic Assignment
**Realizes Scenario:** L3-D-017
**Primary Business Owner:** Faculty Academic Assignment
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-017_Faculty_Academic_Assignment_Establishment.md`; `L2-D02_Faculty_Academic_Assignment_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Establish and activate a governed Faculty Academic Assignment using authoritative academic requirements and Section E faculty/employment facts.

**Start boundary:** An approved academic operational requirement requires assignment of an eligible faculty reference.

**End boundary:** The Faculty Academic Assignment reaches Active with immutable identity, approval history and exact lifecycle facts.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Authoritative academic requirement/context
- Section E-owned faculty identity/employment facts
- Valid assignment period
- Applicable assignment approval governance

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-017-BH-001 | The system shall verify the academic requirement, faculty reference, assignment period and operational context before creation. | L3-D-017 §§3–13 |
| L4-D-017-BH-002 | The system shall use Section E faculty/employment facts without modifying or re-owning them. | L3-D-017 §§3–13 |
| L4-D-017-BH-003 | The system shall establish a Faculty Academic Assignment with immutable identity in Draft when validation succeeds. | L3-D-017 §§3–13 |
| L4-D-017-BH-004 | The system shall permit Draft → Submitted → Under Review only through the governed assignment path. | L3-D-017 §§3–13 |
| L4-D-017-BH-005 | The system shall recognize the authoritative approval/rejection decision without becoming the approver. | L3-D-017 §§3–13 |
| L4-D-017-BH-006 | The system shall transition an Approved Faculty Academic Assignment to Active only through the governed D02 lifecycle. | L3-D-017 §§3–13 |
| L4-D-017-BH-007 | The system shall preserve rejection reason/history when a submission is rejected. | L3-D-017 §§3–13 |
| L4-D-017-BH-008 | The system shall not create or modify faculty identity, employment state, Course Offering or Timetable as part of assignment establishment. | L3-D-017 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-017-BH-009 | The system shall publish `FacultyAssignmentCreated` when Draft establishment completes. | L3-D-017 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |
| L4-D-017-BH-010 | The system shall publish `FacultyAssignmentSubmitted` when submission completes. | L3-D-017 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |
| L4-D-017-BH-011 | The system shall publish `FacultyAssignmentReviewed` when the governed review completes where applicable. | L3-D-017 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |
| L4-D-017-BH-012 | The system shall publish `FacultyAssignmentApproved` or `FacultyAssignmentRejected` according to the authoritative decision. | L3-D-017 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |
| L4-D-017-BH-013 | The system shall publish `FacultyAssignmentActivated` when the approved assignment becomes Active. | L3-D-017 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume Section E faculty/employment facts as authoritative references without ownership transfer.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Invalid academic/faculty/period reference prevents establishment/progression.
- Rejected/unauthorized approval prevents Active and retains the governed prior position.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve assignment identity, academic/faculty references, lifecycle transitions, review/approval evidence and exact lifecycle events.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- the assignment is Active
- immutable identity and source references are retained
- approval history is preserved
- `FacultyAssignmentActivated` is published

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-017` — Faculty Academic Assignment Establishment
**Primary verification source:** `L2-D02_Faculty_Academic_Assignment_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

