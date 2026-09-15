# L5-A-07-006 — Scheduling & Optimization Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-006 |
| Specification Title | Scheduling & Optimization Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Reusable scheduling/conflict/optimization computation without owning timetable/examination/resource business decisions. |
| Technical Architecture Owner | Scheduling & Optimization Engine Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-A-07-001; L5-A-07-005; L5-A-08-001; L5-B-02-001; L5-D-03-001; L5-E-04-001; L4-D-020; L4-I-083 |
| Referenced By | Academic Timetabling; Examination Scheduling; Campus Resource Scheduling workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution architecture for the Scheduling & Optimization Engine within SutraERP. The engine solves complex multi-dimensional constraint satisfaction and combinatorial optimization problems—such as academic course timetabling, examination seating/invigilation schedules, and shared campus resource allocations—while strictly ensuring that it does not own the Timetable aggregate, make room allocation approvals, or publish official academic schedules.

### In Scope

- Execution of constrained optimization algorithms (Constraint Satisfaction Problems - CSP, Integer Linear Programming - ILP, and meta-heuristic solvers).
- Evaluation of hard constraints (no faculty double-booking, room capacity limits, mandatory course prerequisites) vs soft constraints (faculty time preferences, student travel minimization, spread of exam dates).
- Detection and explanation of conflict bottlenecks and unsatisfiable constraint states.
- Multi-candidate solution generation and penalty-weighted scoring.
- Incremental rescheduling and local perturbation solving following localized disruptions.
- Emission of immutable optimization evidence and candidate schedule metrics.

### Out of Scope / Non-Goals

- Approving or publishing official academic timetables or exam rosters (owned by Timetable Committee / Dean of Academics in L4-D-020).
- Allocating campus physical facilities or room maintenance schedules directly.
- Modifying faculty teaching contracts or student course enrollment records.
- UI/UX interactive timetable grid displays or drag-and-drop schedule editing.
- Storing master campus calendar entities directly in engine persistence.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-D-020` | Approved / Locked | `L4-D-020-BH-003` | Controlled generation of timetable candidates under supplied constraints | Combinatorial timetable candidate solver |
| L4 | `L4-D-020` | Approved / Locked | `L4-D-020-BH-004` | Computational conflict analysis supporting governed validation | Hard/soft constraint conflict analysis |
| L4 | `L4-D-020` | Approved / Locked | `L4-D-020-BH-005` | Detection of critical constraint/resource conflicts preventing publication | Infeasibility barrier enforcement |
| L4 | `L4-D-020` | Approved / Locked | `L4-D-020-BH-010` | Reusable computation support for timetable planning execution | Batch schedule generation pipeline |
| L4 | `L4-D-020` | Approved / Locked | `L4-D-020-BH-012` | Technical validation computation supporting timetable validation | Post-candidate feasibility verification |
| Decision | Corpus Register | Locked | `L5-A-07-006` | Reusable scheduling computation without owning timetable decisions | Canonical technical responsibility |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | The engine shall never autonomously publish or activate a timetable. | L5 Candidate Register §3.1 | Generated candidates carry status `PROPOSED_CANDIDATE` and require domain workflow approval. |
| C-002 | Hard constraints must be satisfied with zero violations (100% compliance). | Academic Accreditation Standards | Any candidate with hard constraint violations is marked `INFEASIBLE` and cannot be promoted. |
| C-003 | The solver search space must be bounded by explicit execution budgets (time, iterations). | ADR-L5-0014 | Prevents exponential combinatorial explosions from stalling worker nodes. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Calling scheduling workflows supply valid, certified resource inventories (rooms, timeslots, faculty). | Engine does not query facility management tables directly. | Missing room capacity causes admission rejection. | Resource constraints validated at admission gate. |
| AS-002 | Optimization goals are quantified as numeric penalty weights across soft constraints. | Solvers require objective functions to score candidate quality. | Unweighted preferences lead to arbitrary candidate selection. | Objective penalty weights defined in calling problem context. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Constraint satisfaction and optimization algorithms (branch-and-bound, tabu search, genetic heuristics).
- Hard constraint conflict detection and root-cause bottleneck explanation.
- Soft constraint penalty evaluation and candidate ranking.
- Feasibility boundary checking and unsatisfiable core extraction.
- Incremental rescheduling algorithms (minimizing perturbation to existing assignments).

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-A-07-001` | Secure Computational Engine Baseline | Execution framework | Inherits admission gate, sandbox execution, and lineage hashing. |
| `L5-A-07-005` | General Rules & Formula Engine | Formula evaluation | Evaluates complex conditional scheduling rules and eligibility predicates. |
| `L5-A-08-001` | Background Processing & Job Execution | Asynchronous work-unit | Orchestrates heavy combinatorial optimization search jobs (ADR-L5-0013). |
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Persistence handoff | Stores candidate schedule solutions and conflict reports immutably (ADR-L5-0005). |
| `L5-D-03-001` | Canonical Trust Boundary & Segmentation | Security boundary | Enforces runtime isolation for scheduling worker processes. |
| `L5-E-04-001` | Runtime Packaging & Deployment Baseline | Worker execution | Packages scheduling solver workers as isolated, compute-optimized runtimes. |

### 4.3 Must Not Own

- Approving, locking, or publishing academic course or examination timetables.
- Managing room maintenance, building closures, or physical access security.
- Hiring faculty, modifying teaching load policies, or altering student course registrations.
- Determining institutional scheduling priorities or granting administrative policy waivers.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `ScheduleAdmissionGate` | Control Point | Validates problem formulation, resource domain completeness, and constraint feasibility. | `SchedulingProblemRequest` | Stateless | Admitted scheduling envelope | `L5-A-07-001` | Rejects missing timeslot grids or empty room sets. | L4-D-020-BH-003 |
| `ConstraintModelBinder` | Component | Binds hard rules (room capacity, no overlap) and soft penalty functions (spread, preferences). | Problem ID & Rule refs | In-memory constraint graph | Bound Constraint Graph | `L5-A-07-005` | Fails on malformed constraint definitions. | C-002 |
| `FeasibilityAnalyzer` | Component | Performs arc consistency / constraint propagation; identifies early unsatisfiability bottlenecks. | Bound Constraint Graph | Ephemeral graph structures | Feasibility report | Mathematical solver | Flags impossible problems (e.g. 50 classes for 10 rooms). | L4-D-020-BH-005 |
| `OptimizationSolver` | Runtime Component | Executes search algorithm within timeout bounds; generates top-N feasible schedule candidates. | Feasible constraint graph & budget | Ephemeral state tree | Raw schedule candidates | `L5-E-04-001` | Terminates search upon reaching time/iteration budget. | C-003, ADR-L5-0014 |
| `CandidateScorer` | Control Point | Scores candidates by calculating total penalty points across soft constraints; ranks solutions. | Raw candidates & soft penalties | Stateless | `RankedScheduleCandidates` | Objective functions | Rejects candidates with any hard constraint breaches. | C-002, L4-D-020-BH-004 |

### 5.2 Primary Technical Flow

```text
Academic Timetable Workflow (L4-D-020)
         ↓
ScheduleAdmissionGate (verify Course Offerings, Room Capacities, Faculty Availability)
         ↓
ConstraintModelBinder (bind Hard Constraints & Soft Penalty Weights)
         ↓
FeasibilityAnalyzer (perform domain propagation; check for immediate capacity shortfalls)
         ↓
OptimizationSolver (execute combinatorial search within CPU time budget, e.g. 120s)
         ↓
CandidateScorer (verify Zero Hard Conflicts; compute Soft Penalty Score)
         ↓
LineageEvidenceEmitter (emit SHA-256 evidence record via L5-A-07-001)
         ↓
Return RankedScheduleCandidates to Timetable Committee (Status: PROPOSED_CANDIDATES)
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Unsatisfiable Problem State | `FeasibilityAnalyzer` | Immediate termination (`ERR_PROBLEM_INFEASIBLE`); emit conflict bottleneck explanation. | Endless search looping or outputting candidates with hard collisions. | Academic Dean must add rooms, extend timeslots, or split classes. | L4-D-020-BH-005 |
| Search Budget Exhaustion | `OptimizationSolver` | Return best feasible candidate found so far with warning flag (`WARN_BUDGET_REACHED`). | Crashing or terminating without returning partial feasible solutions. | Caller can accept candidate or re-run with relaxed soft constraints. | C-003, ADR-L5-0014 |
| Hard Constraint Collision | `CandidateScorer` | Candidate disqualified (`ERR_HARD_COLLISION_DETECTED`). | Emitting a schedule with double-booked faculty or room over-capacity. | Solver algorithm must prune invalid search branch. | C-002, L4-D-020-BH-004 |
| Stale Input Data | `ScheduleAdmissionGate` | Invalidate calculation (`ERR_STALE_RESOURCE_SNAPSHOT`). | Optimizing against superseded room or faculty availability records. | Calling workflow re-submits with current resource snapshot. | L4-D-020-BH-012 |

### 5.4 Conceptual Contracts

#### `SchedulingProblemRequest`
```text
SchedulingProblemRequest {
  TenantId: UUID
  InstitutionId: UUID
  ProblemCorrelationId: UUID
  DomainType: Enum [ACADEMIC_TIMETABLE, EXAM_SEATING, FACILITY_BOOKING]
  TermId: String
  EventsToSchedule: List<SchedulableEvent> (e.g. lecture, lab, exam)
  ResourcesAvailable: {
    Rooms: List<RoomResource>
    Timeslots: List<TimeslotResource>
    Instructors: List<InstructorResource>
  }
  HardConstraints: List<HardConstraintDefinition>
  SoftConstraints: List<SoftConstraintPenalty>
  MaxSearchDurationSeconds: Integer (Default: 300)
  MaxCandidateCount: Integer (Default: 3)
}
```

#### `SchedulingSolutionCandidate`
```text
SchedulingSolutionCandidate {
  ProblemCorrelationId: UUID
  EvidenceRecordId: UUID
  CandidateRank: Integer
  FeasibilityStatus: Enum [FEASIBLE_OPTIMAL, FEASIBLE_SUBOPTIMAL, INFEASIBLE]
  TotalSoftPenaltyScore: Decimal(12, 2)
  HardConstraintViolationsCount: Integer (Must be 0 for FEASIBLE)
  AssignmentsList: List<ScheduleAssignment>
  ConflictBottleneckExplanations: List<ConflictExplanation>
  SearchMetrics: {
    IterationsExplored: Long
    DurationSeconds: Decimal(8, 2)
    ConvergenceAchieved: Boolean
  }
}
```

### 5.5 State, Persistence & Consistency Boundaries

The Scheduling Engine is strictly **stateless**. It does not maintain active timetable state. Problem formulations and candidate outputs are transient in worker memory during solving. Candidate schedule solutions and conflict diagnostic reports are durably persisted in `L5-B-02` via `L5-A-07-001`. Recalculation cycles append new solution attempts without deleting prior optimization run evidence.

### 5.6 Asynchronous & Job Realization

Combinatorial scheduling optimization is computationally intensive and operates exclusively as asynchronous long-running background jobs orchestrated by `L5-A-08-001` under `ADR-L5-0013`. Progress updates (e.g. % iterations, current best penalty score) are periodically checkpointed to the job context.

### 5.7 Security, Trust & Isolation

- Solvers operate inside resource-restricted containers (`L5-E-04-001`).
- Solvers are prohibited from making external network or database calls.
- Personal faculty preferences and medical accommodation notes are processed using pseudonymized instructor IDs (`L5-B-05-002`).

### 5.8 Runtime & Deployment Implications

Deploys in a dedicated high-compute worker pool with multi-core CPU affinity, decoupled from general application server clusters.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-A07-060` | Zero-Hard-Conflict Invariant | Feasible schedule candidate status requires exactly zero hard constraint violations. | C-002; Academic integrity | Guarantees that no published candidate contains physical double-booking or room over-capacity. | None (Local L5 Architecture Decision) |
| `AD-A07-061` | Asynchronous Job Optimization Execution | All optimization runs execute asynchronously via `L5-A-08-001` work-units with execution time ceilings. | ADR-L5-0013; C-003 | Prevents long-running combinatorial searches from blocking API gateway threads. | Governed by ADR-L5-0013 |
| `AD-A07-062` | Explainable Bottleneck Extraction | Infeasible problems automatically trigger conflict graph analysis to identify minimal unsatisfiable cores. | L4-D-020-BH-004 | Gives academic planners actionable reasons for infeasibility rather than opaque failure codes. | None (Local L5 Architecture Decision) |
| `AD-A07-063` | Separation of Optimization from Activation | Engine produces `PROPOSED_CANDIDATE` records; official activation requires explicit Timetable Board approval. | Corpus Register L5-A-07-006; C-001 | Preserves institutional authority and governance separation. | Governed by ADR-L5-0005 |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Resource Protection | Strict wall-clock timeouts and CPU quotas prevent combinatorial explosion denial-of-service. | High availability & resilience | `L5-E-01-001`, `L5-E-02-001` |
| Privacy Protection | Faculty constraints and medical exemptions are processed using anonymized resource IDs. | Employee privacy baseline | `L5-B-05-002` |
| Solution Reproducibility | Random seeds and solver configuration parameters are captured in lineage evidence records. | Mathematical reproducibility | `L5-A-07-001` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | The implementation shall admit scheduling problems only after validating resource sets and constraint parameters. | §5.1 | `L4-D-020-BH-003` |
| `CR-002` | Any schedule candidate returned as feasible shall have exactly zero hard constraint violations. | §5.1, `AD-A07-060` | C-002 |
| `CR-003` | If a problem is mathematically infeasible, the engine shall terminate and return a structured bottleneck explanation. | §5.1, `AD-A07-062` | `L4-D-020-BH-004`, `L4-D-020-BH-005` |
| `CR-004` | Solver execution duration shall be strictly bounded by the `MaxSearchDurationSeconds` parameter. | §5.1, `AD-A07-061` | C-003, ADR-L5-0014 |
| `CR-005` | All combinatorial optimization executions shall be orchestrated as asynchronous work-units via `L5-A-08-001`. | §5.6, `AD-A07-061` | ADR-L5-0013 |
| `CR-006` | The engine shall emit an immutable evidence record binding problem formulation and candidate metrics via `L5-A-07-001`. | §5.1, §5.4 | `L5-A-07-001` |
| `CR-007` | The engine shall carry status `PROPOSED_CANDIDATE` and shall not autonomously publish or activate schedules. | §4.3, `AD-A07-063` | C-001 |
| `CR-008` | Incremental rescheduling shall minimize assignment perturbations relative to the existing active schedule baseline. | §4.1 | `L4-D-020-BH-010` |
| `CR-009` | The engine shall enforce zero network and disk I/O during the computational solving lifecycle. | §5.7 | `L5-A-07-001`, ADR-L5-0001 |
| `CR-010` | Soft constraint evaluations shall be scored deterministically using penalty weights supplied in the problem context. | §5.1 | `L4-D-020-BH-004` |
| `CR-011` | In recalculation cycles, historical candidate schedule records shall remain intact and linked via prior attempt hashes. | §5.5 | `L5-A-07-001` |
| `CR-012` | When search budgets are reached before convergence, the engine shall return the best feasible candidate found with warning metadata. | §5.3 | C-003 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Solver Core Algorithm Selection | ADR-Required | Choco-solver vs OptaPlanner / Timefold vs OR-Tools | Lead Architect / L5-A-07 | No | Evaluate JVM vs native C++ optimization engine footprint in Stage 6 consolidation. |
| `OD-002` | Distributed Parallel Solving Topology | Architecture / Scaling | Island model genetic search across multiple worker nodes | Performance Architecture / L5-E-02 | No | Evaluated during high-scale performance testing. |
| `OD-003` | Standard Constraint Definition Grammar | Specification | Declarative JSON/YAML schema for scheduling rules | L5-A-07 Family Owner | No | Standardized during L6 detailed component contract design. |

---

## 10. Traceability Summary

- `L4-D-020-BH-003` — Realized by `ScheduleAdmissionGate` and `OptimizationSolver` generating timetable candidates (§5.1, `CR-001`, `CR-002`).
- `L4-D-020-BH-004` — Realized by `CandidateScorer` and `FeasibilityAnalyzer` evaluating conflicts and soft penalties (§5.1, `CR-010`).
- `L4-D-020-BH-005` — Realized by hard collision rejection and bottleneck explanation preventing invalid publication (§5.1, `CR-002`, `CR-003`).
- `L4-D-020-BH-010` — Realized by batch schedule generation and incremental perturbation minimization (§5.1, `CR-005`, `CR-008`).
- `L4-D-020-BH-012` — Realized by technical validation computation supporting timetable verification (§5.1, `CR-002`).
