# Open Assessment Opportunity to Governed Student Assessment Submission

## 1. Specification Metadata

**Specification ID:** L4-D-053
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Internal Assessment
**Realizes Scenario:** L3-D-053
**Primary Business Owner:** Internal Assessment
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-053_Open_Assessment_Opportunity_to_Governed_Student_Assessment_Submission.md`; `L2-D07_Internal_Assessment_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Capture one governed Assessment Submission and attributable pre-moderation internal marks for an eligible student during an Open Assessment Window.

**Start boundary:** An Assessment Window is Open and an authoritative eligible student participates in the assessment opportunity.

**End boundary:** A unique Assessment Submission and applicable awarded internal marks are recorded and ready for moderation where required.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Open Assessment Window
- Published applicable blueprint component
- Authoritative Section C student registration/eligibility facts
- Applicable D07 assessment authority/policy

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-053-BH-001 | The system shall validate the Open window, blueprint component, authoritative student eligibility and duplicate-submission constraints before recording. | L3-D-053 §§3–13 |
| L4-D-053-BH-002 | The system shall establish one submission identity/evidence for the eligible assessment opportunity. | L3-D-053 §§3–13 |
| L4-D-053-BH-003 | The system shall record awarded internal marks only under the applicable D07 assessment policy/authority. | L3-D-053 §§3–13 |
| L4-D-053-BH-004 | The system shall prevent more than one governed submission for the same student and assessment opportunity where D07 policy requires uniqueness. | L3-D-053 §§3–13 |
| L4-D-053-BH-005 | The system shall where policy permits a pre-moderation mark update, record the update as new governed history with reason rather than destructive overwrite. | L3-D-053 §§3–13 |
| L4-D-053-BH-006 | The system shall make the completed submission available to D07 moderation where required. | L3-D-053 §§3–13 |
| L4-D-053-BH-007 | The system shall not change Section C student/registration facts or compute official Examination grades. | L3-D-053 §§3–13 |
| L4-D-053-BH-008 | The system shall require authoritative D07 business decisions for mark recording or pre-moderation updates where business authority is required. | L3-D-053 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-053-BH-009 | The system shall publish `AssessmentSubmissionRecorded` when the governed submission is recorded. | L3-D-053 §14; L2-D07_Internal_Assessment_Workflows.md exact event semantics |
| L4-D-053-BH-010 | The system shall publish `AssessmentSubmissionUpdated` when a permitted pre-moderation update is recorded. | L3-D-053 §14; L2-D07_Internal_Assessment_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume Section C student/registration eligibility facts without ownership transfer.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Invalid window/eligibility/blueprint blocks submission.
- Missing required assessment authority blocks governed mark acceptance/update.
- Interrupted capture does not assert an authoritative submission until completion.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve student/assessment/source references, submission evidence, awarded marks, permitted updates/reasons, business authority and exact submission events.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- one governed submission exists for the eligible opportunity
- marks and any permitted update are attributable/auditable
- `AssessmentSubmissionRecorded` is published
- Section C ownership remains unchanged

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-053` — Open Assessment Opportunity to Governed Student Assessment Submission
**Primary verification source:** `L2-D07_Internal_Assessment_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

