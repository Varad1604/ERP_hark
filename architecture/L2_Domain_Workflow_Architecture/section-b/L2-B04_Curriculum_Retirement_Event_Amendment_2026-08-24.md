# L2-B04 — Curriculum Framework Workflows — Curriculum Retirement Event Amendment

| Attribute | Value |
|---|---|
| Target | L2-B04 — Curriculum Framework Workflows |
| Amendment Version | 1.3.1 |
| Status | Locked |
| Effective Date | 2026-08-24 |
| Approved Through | ADR-B-CURR-RET-001 |
| Authority | Explicit repository-change authorization recorded 2026-08-24 |

This amendment is part of the canonical L2-B04 architecture. It resolves only the internal naming ambiguity for the completed Curriculum retirement event. All unaffected L2-B04 ownership, lifecycles, rules, dependencies, actors, workflows, and event definitions remain unchanged.

---

## 1. Canonical Retirement Event

The canonical public business event for the existing B04 Curriculum retirement workflow is:

`CurriculumRetired`

| Attribute | Canonical Definition |
|---|---|
| Publisher | B04 — Curriculum Framework |
| Publication condition | Published only after the authoritative B04 transition to `Retired` succeeds. |
| Business meaning | The governed Curriculum retirement outcome is authoritative; future governed use is prevented while historical interpretation and historical references remain preserved. |
| Known direct consumer | D01 — Academic Planning / Course & Course Offering context where retirement awareness is already required by authoritative architecture. |
| Additional consumers | Only consumers already established by another authoritative repository source. This amendment creates no new dependency. |

---

## 2. Conflicting Legacy Reference

`CurriculumVersionRetired` is a conflicting/non-canonical legacy reference to the same completed B04 Curriculum retirement fact.

For all new downstream architecture, including L3 scenarios:

- use `CurriculumRetired` exactly for the Curriculum-retirement path;
- do not publish or consume `CurriculumVersionRetired` as a separate business fact;
- do not infer a second lifecycle or publisher from the legacy terminology.

---

## 3. Non-Curriculum Academic Definitions

This amendment does not create a generic `AcademicStructureRetired` event or any other new public event for non-Curriculum B04 academic-definition retirement.

Where a B04-owned academic definition reaches `Retired` and no exact public event is established by an authoritative upstream source, downstream architecture shall use the authoritative lifecycle fact or governed business hand-off and shall record any need for a new public event as an upstream architecture decision rather than inventing one locally.

---

## 4. Preserved Lifecycle and Ownership

The following architecture remains unchanged:

1. B04 remains the canonical owner of Curriculum Framework business truth.
2. The existing B04 retirement state and lifecycle semantics remain authoritative.
3. Retirement prevents future governed use while preserving historical interpretation and existing historical references.
4. D01 retains ownership of downstream academic-planning and Course Offering facts.
5. Event consumption does not transfer ownership.
6. No L1 boundary, aggregate ownership, dependency direction, business rule, or lifecycle transition is changed by this amendment.

---

## 5. Downstream Rule

L3 scenarios may consume this amendment but shall not redefine:

- event name;
- publisher;
- retirement semantics;
- lifecycle ownership;
- historical-preservation obligations; or
- downstream ownership.

For Curriculum retirement, `CurriculumRetired` is the exact canonical event name.

---

## 6. Traceability

This amendment is governed by:

- `governance/adr/ADR-B-CURR-RET-001.md`
- `governance/constitution/Volume_5_Change_Management.md`
- `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md`

The original L2-B04 document and this amendment shall be read together as the canonical L2-B04 architecture for the amended subject.
