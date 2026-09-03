# Internal Assessment Requirement to Published Assessment Blueprint

## 1. Specification Metadata

**Specification ID:** L4-D-051
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Internal Assessment
**Realizes Scenario:** L3-D-051
**Primary Business Owner:** Internal Assessment
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-051_Internal_Assessment_Requirement_to_Published_Assessment_Blueprint.md`; `L2-D07_Internal_Assessment_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Transform an internal-assessment requirement into a valid Published D07 blueprint defining component structure, weightage and policy binding.

**Start boundary:** An approved Course Offering or academic-term context requires an internal/continuous assessment structure.

**End boundary:** The Internal Assessment Blueprint reaches Published and becomes the authoritative D07 structure required before an Assessment Window can open.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Authoritative Course Offering/curriculum/calendar references
- Assessment-cycle policy
- Component/weightage definition
- Required institutional publication decision

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-051-BH-001 | The system shall establish a Draft Internal Assessment Blueprint for the governed assessment cycle. | L3-D-051 §§3–13 |
| L4-D-051-BH-002 | The system shall use D01/Section B source references without ownership transfer. | L3-D-051 §§3–13 |
| L4-D-051-BH-003 | The system shall record the governed component structure, weightage and policy binding. | L3-D-051 §§3–13 |
| L4-D-051-BH-004 | The system shall validate source references, completeness, weightage and policy before publication. | L3-D-051 §§3–13 |
| L4-D-051-BH-005 | The system shall require an authoritative institutional publication decision before the Internal Assessment Blueprint becomes Published. | L3-D-051 §§3–13 |
| L4-D-051-BH-006 | The system shall prevent publication when validation or required authority fails. | L3-D-051 §§3–13 |
| L4-D-051-BH-007 | The system shall transition Draft → Published only after the governed publication outcome. | L3-D-051 §§3–13 |
| L4-D-051-BH-008 | The system shall prevent an Assessment Window from opening before the blueprint is Published. | L3-D-051 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-051-BH-009 | The system shall publish `InternalAssessmentBlueprintPublished` when the blueprint reaches Published. | L3-D-051 §14; L2-D07_Internal_Assessment_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume D01/Section B Course Offering, curriculum and calendar context without ownership transfer.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Source/component/weightage validation failure blocks publication.
- Missing/rejected publication authority retains the blueprint unpublished.
- Interruption preserves last valid blueprint position.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve blueprint version, source references, components/weightage, policy binding, validation and authoritative publication decision.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- the blueprint is valid and Published
- publication authority is evidenced without invented role names
- `InternalAssessmentBlueprintPublished` is published
- an Assessment Window may proceed only after publication

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-051` — Internal Assessment Requirement to Published Assessment Blueprint
**Primary verification source:** `L2-D07_Internal_Assessment_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

The parent is Locked under the final corpus decision despite historical actor-mapping text; this L4 specification does not invent D07 role names.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

