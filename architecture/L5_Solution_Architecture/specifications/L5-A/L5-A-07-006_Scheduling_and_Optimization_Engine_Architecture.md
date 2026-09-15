# L5-A-07-006 — Scheduling & Optimization Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-006 |
| Specification Title | Scheduling & Optimization Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Reusable scheduling/conflict/optimization computation without owning timetable, examination or resource business decisions. |
| Technical Architecture Owner | Scheduling & Optimization Engine Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.1 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5-A-07-001; L5-A-07-005; L5-A-08-001; L5-B-02-001; L5-D-03-001; L4-D-020; L4-I-083; L5 governance/template/validation standards |
| Referenced By | Timetabling, examination scheduling and resource-allocation workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose & Boundary

Provides reusable technical computation for feasibility checking, conflict detection, candidate schedule generation and objective evaluation using authoritative constraints supplied by the owning business workflow. The engine does not define institutional scheduling policy, decide which constraints are mandatory, approve a timetable, allocate a resource canonically or publish a schedule.

## 3. Technical Realization

| Element | Responsibility |
|---|---|
| `OptimizationAdmissionBoundary` | Validate problem identity, authoritative resource/input snapshot, constraint-set version and objective definition. |
| `ConstraintBinder` | Bind the exact owning-domain hard/soft constraints supplied for this optimization attempt. |
| `CandidateSearchBoundary` | Explore candidate solutions using an implementation selected downstream, subject to governed resource bounds. |
| `ConstraintEvaluator` | Evaluate candidate compliance against the supplied hard/soft constraints. |
| `ObjectiveEvaluator` | Score feasible candidates according to the supplied objective definition; no local institutional priority is invented. |
| `OptimizationEvidenceAssembler` | Preserve input/constraint/objective versions, technical outcome and candidate evidence. |
| `CandidateResultHandoff` | Return feasible candidates/conflict evidence to the owning workflow for decision/approval. |

## 4. Canonical Constraints

- Hard versus soft classification is an authoritative input from the owning workflow/rule package.
- A candidate violating a supplied hard constraint cannot be represented as feasible.
- The engine may return no feasible candidate; it must not silently relax hard constraints.
- Solver family, search algorithm, heuristic, parallelization strategy and runtime topology are not selected at this L5 specification.
- Maximum search duration, candidate count and other numeric limits are governed configuration/implementation decisions, not canonical defaults here.

## 5. Ownership

Owns technical optimization execution and conflict evidence. Does not own timetable policy, exam policy, campus-resource policy, constraint semantics, institutional preference weighting, approval/publication or business schedule lifecycle.

## 6. Failure & Recovery

- Missing/ambiguous constraints or objective definition: reject admission.
- Infeasible constraint set: return infeasibility/conflict evidence; do not weaken constraints automatically.
- Search/resource limit reached: return bounded technical outcome indicating no accepted solution was established within governed execution bounds.
- Transient infrastructure failure: retry only through A08.

## 7. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-060 | Optimization consumes authoritative constraints/objectives and returns candidates; business selection remains upstream. | Repository-derived boundary |
| AD-A07-061 | Hard constraints are never silently relaxed by the engine. | Defensible technical derivation |
| OD-A07-060 | Solver family, algorithm, search heuristic, parallelization, resource limits and candidate-count targets remain unresolved pending governed downstream design. | Open decision |

## 8. Conformance Requirements

1. A07-006 shall not hardcode institutional scheduling constraints or accreditation rules without authoritative source.
2. No solver/algorithm family shall be normative at L5 without approved decision authority.
3. Hard constraints supplied by the owner shall not be silently relaxed.
4. Returned candidates shall include traceability to the exact constraint/objective versions used.
5. Candidate results shall not be represented as approved/published schedules by this engine alone.