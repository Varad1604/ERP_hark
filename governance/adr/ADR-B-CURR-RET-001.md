# ADR-B-CURR-RET-001 — Canonical Curriculum Retirement Event

| Attribute | Value |
|---|---|
| ADR ID | ADR-B-CURR-RET-001 |
| Date | 2026-08-24 |
| Status | Approved |
| Architecture Level | L2 — Domain Workflow Architecture |
| Domain | B04 — Curriculum Framework |
| Branch | dhruv |
| Approval Source | Explicit repository-change authorization recorded in the architecture working session: “Go ahead I authorize you.” |

## 1. Context

Locked `L2-B04_Curriculum_Framework_Workflows.md` contains an internal naming inconsistency for the completed Curriculum retirement fact. The retirement workflow and publication semantics use `CurriculumRetired`, while another event reference uses `CurriculumVersionRetired` for downstream consumption.

This ambiguity prevents L3 from using one exact canonical event name without inventing or arbitrarily selecting terminology.

## 2. Decision

The canonical public business event for the existing B04 Curriculum retirement workflow is:

`CurriculumRetired`

Canonical semantics:

- **Canonical Publisher:** B04 — Curriculum Framework.
- **Publication condition:** only after the authoritative B04 retirement transition to `Retired` succeeds.
- **Business meaning:** the governed Curriculum retirement outcome is authoritative; future governed use is prevented while historical interpretation and existing historical references remain preserved.
- **Known direct consumer:** D01 — Academic Planning / Course & Course Offering context where the existing architecture requires retirement awareness.
- **Additional consumers:** only where already established by another authoritative repository source; this ADR creates no new dependency.

`CurriculumVersionRetired` is treated as a non-canonical conflicting/legacy reference for this same B04 retirement fact and shall not be introduced into new L3 architecture.

No separate `AcademicStructureRetired` event is created by this ADR. Where a non-Curriculum B04 academic definition is retired and no exact public event is established upstream, the scenario may use the authoritative lifecycle fact or governed business hand-off without inventing an event.

## 3. Preserved Architecture

This decision does **not** change:

- B04 canonical ownership.
- Curriculum or Programme Version ownership.
- The existing retirement lifecycle or `Retired` state.
- Historical-preservation requirements.
- D01 ownership of downstream academic-planning or course-offering facts.
- Any L1 domain boundary.
- Any repository dependency direction.

## 4. Alternatives Considered

### Option A — `CurriculumRetired`

Selected because it is already the event explicitly published by the B04 retirement workflow and is already tied to successful retirement transition semantics.

### Option B — `CurriculumVersionRetired`

Rejected as the canonical name because adopting it would override the event already published by the authoritative workflow and would require a broader rename without stronger upstream evidence.

### Option C — Introduce a new generic retirement event

Rejected because it would create new terminology and unnecessary downstream impact when an existing B04 event already expresses the completed business fact.

## 5. Repository Effect

A locked B04 amendment shall make this decision authoritative for the conflicting retirement-event references only. Downstream L3 scenarios shall consume `CurriculumRetired` exactly where the Curriculum-retirement path applies and shall not invent an event for other academic-definition retirement paths unless separately established upstream.

## 6. Traceability

Authoritative sources reviewed:

- `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B04_Curriculum_Framework_Workflows.md`
- `DEPENDENCY_MAP.md`
- `governance/constitution/Volume_5_Change_Management.md`
- `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-025_Academic_Structure_or_Curriculum_Retirement.md`
