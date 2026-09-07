# Course Offering Establishment to Activation

## 1. Specification Metadata

**Specification ID:** L4-D-001
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Course & Course Offering
**Realizes Scenario:** L3-D-001
**Primary Business Owner:** Course & Course Offering
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-001_Course_Offering_Establishment_to_Activation.md`; `L2-D01_Course_and_Course_Offering_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Support governed establishment of a Course Offering from an authorized institutional requirement through creation, applicable approval, publication, enrollment-lifecycle progression, readiness validation and activation.

**Start boundary:** An authorized institutional requirement exists to operationalize an Active Course for a valid Academic Period and institutional context.

**End boundary:** The Course Offering reaches Active after the governed Published → Enrollment Open → Enrollment Closed path and successful authoritative readiness validation.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Active Course and valid Programme/Department relationships
- Academic Period that permits Course Offering creation
- Applicable institutional, campus, calendar, capacity and enrollment configuration
- Applicable approval policy and authoritative readiness results

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-001-BH-001 | The system shall verify the authoritative Course, Programme, Department, Academic Period and institutional context before establishment. | L3-D-001 §§3–13 |
| L4-D-001-BH-002 | The system shall prevent establishment when a required reference is invalid or the Academic Period does not permit offering creation. | L3-D-001 §§3–13 |
| L4-D-001-BH-003 | The system shall validate mandatory offering attributes, applicable configuration, capacity constraints and creation rules before creating the offering. | L3-D-001 §§3–13 |
| L4-D-001-BH-004 | The system shall establish one Course Offering with immutable identity in Draft when establishment validation succeeds. | L3-D-001 §§3–13 |
| L4-D-001-BH-005 | The system shall not treat Draft creation as publication, enrollment availability or activation. | L3-D-001 §§3–13 |
| L4-D-001-BH-006 | The system shall where approval is required, permit only a complete Draft to enter Pending Approval and recognize the authoritative governance decision without becoming the approver. | L3-D-001 §§3–13 |
| L4-D-001-BH-007 | The system shall transition an approved request to Approved and return a rejected request to Draft with rejection evidence preserved. | L3-D-001 §§3–13 |
| L4-D-001-BH-008 | The system shall, where policy does not require approval, permit progression toward publication without recording a fictitious approval decision. | L3-D-001 §§3–13 |
| L4-D-001-BH-009 | The system shall prevent publication until the applicable publication prerequisites are satisfied. | L3-D-001 §§3–13 |
| L4-D-001-BH-010 | The system shall move an eligible approved offering to Published and make the published business fact available to applicable downstream consumers. | L3-D-001 §§3–13 |
| L4-D-001-BH-011 | The system shall govern Published → Enrollment Open → Enrollment Closed without creating or modifying Section C-owned student registration transactions. | L3-D-001 §§3–13 |
| L4-D-001-BH-012 | The system shall at Enrollment Closed, obtain the authoritative readiness results required for activation. | L3-D-001 §§3–13 |
| L4-D-001-BH-013 | The system shall transition Enrollment Closed → Active only when required readiness succeeds. | L3-D-001 §§3–13 |
| L4-D-001-BH-014 | The system shall when readiness fails, prevent activation and retain Enrollment Closed with the failed readiness outcome exposed. | L3-D-001 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-001-BH-015 | The system shall publish `CourseOfferingCreated` when Draft establishment completes. | L3-D-001 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-001-BH-016 | The system shall publish `CourseOfferingSubmittedForApproval` when the approval-required path is submitted. | L3-D-001 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-001-BH-017 | The system shall publish `CourseOfferingApproved` or `CourseOfferingRejected` according to the authoritative approval outcome. | L3-D-001 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-001-BH-018 | The system shall publish `CourseOfferingPublished` when publication completes. | L3-D-001 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-001-BH-019 | The system shall publish `CourseOfferingActivated` when activation succeeds. | L3-D-001 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |
| L4-D-001-BH-020 | The system shall publish `CourseOfferingActivationFailed` when readiness prevents activation. | L3-D-001 §14; L2-D01_Course_and_Course_Offering_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Use Section B-owned Course, Programme, Department and Academic Period facts without re-owning them.
- Use authoritative Faculty/Timetable readiness facts where applicable without transferring their ownership.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Invalid academic/institutional context prevents creation.
- Rejected approval preserves the governed Draft position and approval history.
- Unmet publication prerequisites prevent Published.
- Failed readiness prevents Active and preserves Enrollment Closed.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve offering identity, source references, validation basis, approval decision where applicable, publication outcome, enrollment-lifecycle position, readiness evidence and final activation outcome.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- one governed Course Offering with immutable identity exists
- applicable approval/publication obligations are complete
- the offering has followed the governed enrollment lifecycle
- successful readiness has produced Active and `CourseOfferingActivated`

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-001` — Course Offering Establishment to Activation
**Primary verification source:** `L2-D01_Course_and_Course_Offering_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

Faculty assignment, timetable creation, resource allocation, student-specific registration and assessment execution remain outside this specification.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

