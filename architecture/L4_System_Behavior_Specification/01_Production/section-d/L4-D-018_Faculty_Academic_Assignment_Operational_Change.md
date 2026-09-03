# Faculty Academic Assignment Operational Change

## 1. Specification Metadata

**Specification ID:** L4-D-018
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Faculty Academic Assignment
**Realizes Scenario:** L3-D-018
**Primary Business Owner:** Faculty Academic Assignment
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-018_Faculty_Academic_Assignment_Operational_Change.md`; `L2-D02_Faculty_Academic_Assignment_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Apply a controlled operational change to an existing Faculty Academic Assignment while preserving prior assignment history and Section E ownership.

**Start boundary:** An authorized justified modification requirement exists for an operational Faculty Academic Assignment.

**End boundary:** The Operational Change reaches Closed after approval/application, with historical assignment accuracy preserved.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Existing assignment permitting controlled change
- Justification and impact information
- Authoritative Section E faculty/employment facts
- Applicable policy/approval authority

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-018-BH-001 | The system shall establish a justified Operational Change request against the existing assignment. | L3-D-018 §§3–13 |
| L4-D-018-BH-002 | The system shall validate assignment lifecycle, applicable policy, operational impact and Section E faculty facts. | L3-D-018 §§3–13 |
| L4-D-018-BH-003 | The system shall prevent progression when the request or authoritative references are invalid. | L3-D-018 §§3–13 |
| L4-D-018-BH-004 | The system shall recognize the authoritative approval/rejection decision without becoming the approver. | L3-D-018 §§3–13 |
| L4-D-018-BH-005 | The system shall apply an approved change without rewriting prior assignment history. | L3-D-018 §§3–13 |
| L4-D-018-BH-006 | The system shall close the Operational Change only after the applied business change is confirmed. | L3-D-018 §§3–13 |
| L4-D-018-BH-007 | The system shall not execute Timetable revision or modify Section E faculty master as part of this scenario. | L3-D-018 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-018-BH-008 | The system shall publish `OperationalChangeRequested` when the change request is established. | L3-D-018 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |
| L4-D-018-BH-009 | The system shall publish `OperationalChangeValidated` when validation completes. | L3-D-018 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |
| L4-D-018-BH-010 | The system shall publish `OperationalChangeApproved` when the authoritative approval occurs. | L3-D-018 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |
| L4-D-018-BH-011 | The system shall publish `OperationalChangeApplied` when the approved change becomes effective. | L3-D-018 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |
| L4-D-018-BH-012 | The system shall publish `OperationalChangeClosed` when the governed change closes. | L3-D-018 §14; L2-D02_Faculty_Academic_Assignment_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume Section E faculty/employment facts without ownership transfer; any timetable consequence remains outside this scenario.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Validation/reference failure prevents approval/application.
- Rejected/unauthorized change preserves the prior assignment position.
- Interrupted business progression preserves the last valid Operational Change position.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve request reason, prior assignment position, Section E references, impact/validation evidence, decision, applied change and closure evidence.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- the approved change is applied
- the Operational Change is Closed
- prior assignment history remains traceable
- `OperationalChangeClosed` is published

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-018` — Faculty Academic Assignment Operational Change
**Primary verification source:** `L2-D02_Faculty_Academic_Assignment_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

