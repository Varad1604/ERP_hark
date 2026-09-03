# Administrative Course Offering Cancellation

## 1. Specification Metadata

**Specification ID:** L4-D-004
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Course & Course Offering
**Realizes Scenario:** L3-D-004
**Primary Business Owner:** Course & Course Offering
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-004_Administrative_Course_Offering_Cancellation.md`; `L2-D01_Course_and_Course_Offering_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Terminate an eligible unpublished Course Offering through an administrative cancellation decision without changing Section C-owned registrations.

**Start boundary:** An authorized cancellation requirement exists for an unpublished Course Offering in Draft or Approved.

**End boundary:** The offering reaches Cancelled and the cancellation fact is available to Section C.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Unpublished Draft or Approved Course Offering
- Applicable cancellation policy
- Governed cancellation reason/impact assessment

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-004-BH-001 | The system shall verify that the Course Offering is unpublished and in Draft or Approved. | L3-D-004 §§3–13 |
| L4-D-004-BH-002 | The system shall prevent the administrative cancellation path for Published, Active, Completed or Archived offerings. | L3-D-004 §§3–13 |
| L4-D-004-BH-003 | The system shall evaluate the request against the applicable cancellation policy. | L3-D-004 §§3–13 |
| L4-D-004-BH-004 | The system shall recognize the authoritative cancellation decision without assuming decision authority. | L3-D-004 §§3–13 |
| L4-D-004-BH-005 | The system shall transition Draft/Approved → Cancelled when the governed cancellation is approved. | L3-D-004 §§3–13 |
| L4-D-004-BH-006 | The system shall prevent a Cancelled offering from activation or enrollment availability. | L3-D-004 §§3–13 |
| L4-D-004-BH-007 | The system shall preserve offering identity and cancellation history. | L3-D-004 §§3–13 |
| L4-D-004-BH-008 | The system shall not cancel or correct Section C-owned registration records. | L3-D-004 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-004-BH-009 | The system shall publish `CourseOfferingCancelled` when cancellation completes. | L3-D-004 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Make the completed cancellation fact available to Section C for independently owned registration consequences.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Unsupported lifecycle state preserves the current state.
- Failed cancellation policy/authority validation does not alter the offering.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve prior state, cancellation eligibility, reason, decision/impact evidence, resulting Cancelled state and event evidence.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- the eligible offering is Cancelled
- future activation/enrollment is prevented for the cancelled offering
- `CourseOfferingCancelled` is published
- Section C ownership remains unchanged

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-004` — Administrative Course Offering Cancellation
**Primary verification source:** `L2-D01_Course_and_Course_Offering_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

