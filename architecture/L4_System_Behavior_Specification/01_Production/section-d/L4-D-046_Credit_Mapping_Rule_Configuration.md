# Credit Mapping Rule Configuration

## 1. Specification Metadata

**Specification ID:** L4-D-046
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Academic Delivery — Open Electives & MOOCs
**Realizes Scenario:** L3-D-046
**Primary Business Owner:** Open Electives & MOOCs
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-D-046_Credit_Mapping_Rule_Configuration.md`; `L2-D06_Open_Electives_and_MOOCs_Workflows.md`; Locked L4 governance/template/validation baseline
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

Define, validate, approve and publish versioned institutional equivalency rules between flexible/external learning and canonical curriculum.

**Start boundary:** Curriculum, learning-opportunity or equivalency-governance change requires a new mapping version.

**End boundary:** One governed Active mapping exists for the applicable curriculum version/scope and is available to future Credit Recognition cases.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Published canonical curriculum version
- Candidate learning opportunity/provider evidence
- Applicable mapping/credit policy
- Prior mapping version and publication authority

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-046-BH-001 | The system shall establish a distinct mapping version against authoritative curriculum/opportunity references. | L3-D-046 §§3–13 |
| L4-D-046-BH-002 | The system shall use Section B curriculum/Course/Learning Outcome definitions without re-owning them. | L3-D-046 §§3–13 |
| L4-D-046-BH-003 | The system shall record the governed equivalency, credit, scope and effective-period definition. | L3-D-046 §§3–13 |
| L4-D-046-BH-004 | The system shall validate duplicate mapping, curriculum availability, credit limits and policy before approval. | L3-D-046 §§3–13 |
| L4-D-046-BH-005 | The system shall require an authoritative institutional mapping-approval outcome before publication. | L3-D-046 §§3–13 |
| L4-D-046-BH-006 | The system shall prevent publication when validation/approval fails. | L3-D-046 §§3–13 |
| L4-D-046-BH-007 | The system shall publish the governed Active mapping while retaining predecessor history. | L3-D-046 §§3–13 |
| L4-D-046-BH-008 | The system shall not retroactively alter completed recognition decisions or Academic Records facts. | L3-D-046 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-046-BH-009 | The system shall publish `CreditMappingRulesPublished` when the governed mapping becomes Published/Active. | L3-D-046 §14; L2-D06_Open_Electives_and_MOOCs_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Section B retains curriculum definitions; Section C/Academic Records retains permanent student academic records.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Invalid curriculum/policy/duplicate/limit condition blocks publication.
- Missing/invalid publication authority retains the prior Active mapping.
- Interrupted progression preserves last authoritative mapping version.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve mapping version, source curriculum references, equivalency rationale/credit values/scope, validation/approval evidence and version lineage.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- a governed mapping is Published/Active for future cases
- historical recognition decisions remain unchanged
- ownership remains separated
- `CreditMappingRulesPublished` is published

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-046` — Credit Mapping Rule Configuration
**Primary verification source:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

The parent does not establish the exact canonical final mapping-approval role name; this L4 specification requires authoritative approval without inventing that role.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

