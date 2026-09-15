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
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5-A-07-001; L5-A-07-005; L5-A-08-001; L5-B-02-001; L5-D-03-001; L4-D-020; L4-I-083; L5 governance/template/validation standards |
| Referenced By | Timetabling, examination scheduling and resource-allocation workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose & Boundary

Provides reusable technical computation for feasibility checking, conflict detection, candidate schedule generation and objective evaluation using authoritative constraints supplied by the owning business workflow. The engine does not define institutional scheduling policy, decide which constraints are mandatory, approve a timetable, allocate a resource canonically or publish a schedule.

## 3. Architectural Invariants

1. Hard/soft classification, constraint meaning, weights and objectives are authoritative inputs from the owning workflow/rule package.
2. A candidate violating a supplied hard constraint cannot be represented as feasible.
3. The engine may return no feasible candidate; it must not silently relax or delete hard constraints.
4. Candidate generation and business selection/approval remain distinct responsibilities.
5. Every returned candidate/conflict result must be traceable to the exact input, constraint-set and objective versions used.
6. Search/resource exhaustion produces an explicit bounded technical outcome, not an invented business conclusion.
7. Solver/search technology is not canonical until governed downstream.

## 4. Technical Realization

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `OptimizationAdmissionBoundary` | Validate problem identity, owning-workflow/correlation reference, authoritative resource/input snapshot, constraint-set version and objective definition. | Authorized optimization request | Admitted optimization problem | Rejects incomplete or ambiguous problem definition. |
| `ConstraintBinder` | Bind exact owning-domain hard/soft constraints and associated semantics/weights supplied for the attempt. | Constraint references | Bound constraint set | Cannot reclassify or omit supplied constraints. |
| `ObjectiveBinder` | Bind authoritative objective definition and weighting/priority information where supplied. | Objective reference | Bound objective model | Missing required objective context remains unresolved. |
| `CandidateSearchBoundary` | Explore candidate solutions using an implementation selected downstream, subject to governed resource bounds. | Bound problem | Candidate set / bounded search outcome | Resource exhaustion terminates safely without claiming infeasibility unless proven by the chosen method. |
| `ConstraintEvaluator` | Evaluate candidate compliance against the supplied hard/soft constraints. | Candidate + constraint set | Compliance evidence | Hard-constraint violation prevents feasible classification. |
| `ObjectiveEvaluator` | Score feasible candidates according to supplied objective definition; no local institutional priority is invented. | Feasible candidates + objective | Ranked/scored technical candidates | Cannot create new optimization preference. |
| `OptimizationEvidenceAssembler` | Preserve input/constraint/objective versions, outcome, conflicts and technical evidence. | Optimization attempt | Evidence package | Evidence failure prevents successful completion where required. |
| `CandidateResultHandoff` | Return candidates/conflict/infeasibility evidence to owning workflow for decision/approval. | Validated result | Domain-consumable optimization result | Cannot approve/publish canonical schedule. |

## 5. Conceptual Contract

A request conceptually identifies tenant/institution, owning workflow, problem/correlation identity, authoritative resource/input snapshot, exact constraint-set version, hard/soft classifications, objective definition/version and predecessor/reoptimization reference where applicable.

A result conceptually returns technical attempt reference, feasible candidate references and/or conflict/infeasibility evidence, score/objective evidence where applicable, exact input/constraint/objective references, bounded-search outcome classification, and lineage/evidence reference.

Exact payload fields remain downstream design.

## 6. Constraint & Objective Semantics

- Hard/soft classification is authoritative and cannot be changed by the engine.
- Soft-constraint tradeoffs and objective weights are owner-supplied; the engine does not invent institutional priorities.
- If required constraints/objectives are missing or ambiguous, admission fails rather than proceeding with generic defaults.
- Conflict explanations are technical evidence about the supplied problem; they do not authorize policy exceptions.
- If multiple feasible candidates are returned, final selection remains the owning workflow's business decision.

## 7. Search Outcome Semantics

The architecture distinguishes:

- **feasible candidate established** — candidate satisfies all supplied hard constraints;
- **proven infeasible under supplied problem** — only when the downstream selected method can legitimately establish infeasibility;
- **no candidate established within governed bounds** — bounded technical search outcome and not equivalent to proven infeasibility;
- **invalid/incomplete problem definition** — admission/authority failure;
- **transient execution failure** — infrastructure/operational failure eligible for A08-governed recovery where appropriate.

This distinction prevents resource/time limits from being misrepresented as business infeasibility.

## 8. Reoptimization & Lineage

A reoptimization request with changed resources, constraints, weights or objectives is a new technical attempt. Where evidence/history is required, the predecessor attempt is linked rather than overwritten. A previously approved/published schedule remains business-owned and is not altered merely because a new candidate is generated.

## 9. Ownership

### Owns

Technical optimization/search execution, constraint evaluation mechanics, objective scoring mechanics, candidate/conflict evidence, lineage and result handoff.

### Does Not Own

Timetable policy, examination policy, campus-resource policy, constraint semantics, hard/soft classification, institutional preference weighting, approval/publication or business schedule lifecycle.

## 10. Failure & Recovery

| Failure | Treatment |
|---|---|
| Missing/ambiguous constraint/objective/input reference | Reject admission. |
| Hard-constraint violation | Candidate cannot be marked feasible. |
| Infeasible problem | Return evidence only when legitimately established. |
| Search/resource bound reached | Return bounded outcome; do not silently relax constraints or claim proven infeasibility. |
| Deterministic scoring/evaluation error | Return technical failure; no blind retry. |
| Evidence failure | Do not report successful completion where evidence is required. |
| Transient infrastructure failure | Delegate retry/work recovery to A08. |

## 11. Security, Trust & Evidence

- Problem inputs may expose timetable, staff, room, examination or resource data and remain tenant/institution scoped.
- Constraint/rule packages are validated through A07-001/A07-005 trust boundaries.
- The engine cannot access canonical persistence directly merely to discover missing resources or constraints.
- Evidence must correlate returned candidates with exact authoritative input/constraint/objective versions.

## 12. Observability & Operability

Technical evidence should expose problem/correlation reference, constraint/objective versions, search outcome class, candidate/conflict references, predecessor/reoptimization relation, failure class and evidence handoff status. Exact search-duration, candidate-count and capacity targets remain downstream governed decisions.

## 13. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-060 | Optimization consumes authoritative constraints/objectives and returns candidates; business selection remains upstream. | Repository-derived boundary |
| AD-A07-061 | Hard constraints are never silently relaxed by the engine. | Defensible technical derivation |
| AD-A07-062 | Bounded search exhaustion is distinct from proven infeasibility. | Technical correctness derivation |
| OD-A07-060 | Solver family, algorithm, search heuristic, parallelization, resource limits and candidate-count targets remain unresolved pending governed downstream design. | Open decision |

## 14. Conformance Requirements

1. A07-006 shall not hardcode institutional scheduling constraints or accreditation rules without authoritative source.
2. No solver/algorithm family shall be normative at L5 without approved decision authority.
3. Hard constraints supplied by the owner shall not be silently relaxed.
4. Returned candidates shall include traceability to the exact input, constraint and objective versions used.
5. Candidate results shall not be represented as approved/published schedules by this engine alone.
6. Search/resource exhaustion shall not be represented as proven infeasibility unless the chosen governed method can establish that conclusion.
7. Reoptimization shall preserve predecessor lineage where required.

## 15. Architecture Acceptance Evidence

A conforming design must demonstrate that hard constraints cannot be silently downgraded, missing constraints/objectives fail admission, bounded search exhaustion is distinguishable from infeasibility, candidate approval remains upstream, and every candidate/conflict result is traceable to its authoritative problem definition.

## 16. Traceability Summary

Referenced L4-D/L4-I behaviors remain authoritative for timetable/examination scheduling semantics. A07-006 provides reusable computation only and does not transfer business scheduling authority into the optimization engine.