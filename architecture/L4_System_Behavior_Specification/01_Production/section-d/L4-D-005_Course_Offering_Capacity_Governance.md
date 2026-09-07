# Course Offering Capacity Governance

## 1. Specification Metadata

**Specification ID:** L4-D-005
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Course & Course Offering
**Realizes Scenario:** L3-D-005
**Primary Business Owner:** Course & Course Offering
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-005_Course_Offering_Capacity_Governance.md`; `L2-D01_Course_and_Course_Offering_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Govern an effective-dated Course Offering capacity change while protecting confirmed Section C registrations and applicable capacity constraints.

**Start boundary:** An authorized capacity-change requirement exists for a modifiable Course Offering.

**End boundary:** The authorized capacity position becomes effective and the applicable exact capacity fact is published, or the request is rejected without changing capacity.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Current governed capacity
- Lifecycle position permitting capacity modification
- Reserved-seat/waitlist constraints
- Authoritative Section C confirmed-registration facts
- Applicable min/max/institutional policy

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-005-BH-001 | The system shall verify that the Course Offering lifecycle permits capacity modification. | L3-D-005 §§3–13 |
| L4-D-005-BH-002 | The system shall use authoritative confirmed-registration information from Section C without changing it. | L3-D-005 §§3–13 |
| L4-D-005-BH-003 | The system shall evaluate the proposed value against applicable capacity, reserved-seat, waitlist and institutional policy. | L3-D-005 §§3–13 |
| L4-D-005-BH-004 | The system shall prevent any negative capacity value. | L3-D-005 §§3–13 |
| L4-D-005-BH-005 | The system shall for a reduction, prevent the change when it would invalidate a confirmed registration. | L3-D-005 §§3–13 |
| L4-D-005-BH-006 | The system shall recognize required approval where institutional policy requires it without becoming the approver. | L3-D-005 §§3–13 |
| L4-D-005-BH-007 | The system shall apply an authorized capacity change as an effective-dated D01 fact while preserving prior capacity history. | L3-D-005 §§3–13 |
| L4-D-005-BH-008 | The system shall leave the Course Offering lifecycle state unchanged by the capacity decision. | L3-D-005 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-005-BH-009 | The system shall publish `CourseOfferingCapacityIncreased` for a completed governed increase where applicable. | L3-D-005 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-005-BH-010 | The system shall publish `CourseOfferingCapacityReduced` for a completed governed reduction where applicable. | L3-D-005 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-005-BH-011 | The system shall publish `CourseOfferingCapacityChanged` for a completed general capacity modification where applicable. | L3-D-005 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Make the completed capacity fact available to Section C without transferring registration ownership.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Lifecycle-ineligible requests preserve current capacity.
- Policy-invalid values are rejected/returned without changing capacity.
- A reduction that would invalidate confirmed registrations is prevented.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve previous/new capacity, effective position, applicable policy, Section C registration facts used as evidence, decision authority where required and exact completed capacity fact.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- a governed capacity decision exists
- any effective change complies with policy
- confirmed registrations remain valid
- the applicable exact capacity event is published

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-005` — Course Offering Capacity Governance
**Primary verification source:** `L2-D01_Course_and_Course_Offering_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

