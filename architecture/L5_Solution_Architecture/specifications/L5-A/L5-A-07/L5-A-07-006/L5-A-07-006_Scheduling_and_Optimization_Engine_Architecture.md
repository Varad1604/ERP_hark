# L5-A-07-006 — Scheduling & Optimization Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-006 |
| Specification Title | Scheduling & Optimization Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Technical Architecture Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Computation Engines |
| Version | 1.0.1 |
| Status | Draft — Authoring Authorized; Not Locked |
| Author | Varad |
| Canonical Responsibility | Provide reusable technical scheduling, conflict-analysis and optimization computation without owning timetable, examination or resource business decisions. |

**Primary L5 Dependency:** L5-A-07-001 — Secure Computational Engine Architecture Baseline

**Related L5 Specifications:** L5-A-07-005, L5-A-08-001, L5-A-08-002, L5-D, L5-D-04, L5-E.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable L5 technical architecture for scheduling, conflict analysis and constrained optimization computations used by SutraERP business workflows.

The engine transforms an explicitly supplied scheduling problem, authoritative facts and governed constraints into technically evaluated candidates, conflict findings, feasibility findings or optimization outcomes.

It is a computation capability, not a business authority.

### 2.2 Scope

- controlled scheduling/optimization admission;
- explicit scope binding;
- authoritative input context binding;
- hard/soft constraint evaluation;
- feasibility and conflict analysis;
- candidate generation;
- optimization/ranking against supplied objectives;
- deterministic/reproducible execution where required;
- bounded computational search;
- candidate comparison and validation;
- technical evidence and lineage;
- controlled handoff to owning business workflows.

### 2.3 Non-Goals

This specification does not own the Timetable aggregate, Examination Schedule, physical resource masters, Faculty Assignment, Course Offering, Academic Calendar, institutional scheduling policy, approval/rejection authority, publication/activation authority, business lifecycle definitions, canonical business events, APIs, database schemas, UI/UX, deployment topology, or technology/product selection. It does not permit arbitrary code, SQL, filesystem or unrestricted network execution.

### 2.4 Ownership Boundary

The consuming business domain remains authoritative for the business scheduling problem, participating entities/facts, applicable business constraints/policy, meaning of the resulting schedule, approval/publication/activation decisions and canonical lifecycle/events.

L5-A-07-006 owns only the technical computation mechanism used to evaluate feasible schedules, identify conflicts and optimize candidate solutions.

A technically optimal candidate is not automatically a business-approved or authoritative schedule.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Repository Applicability

The L5 corpus assigns L5-A-07-006 reusable scheduling/conflict/optimization computation without ownership of timetable, examination or resource business decisions.

The engine may support the canonical Timetable workflow and other authorized scheduling workflows, including examination planning where upstream architecture requires the computation, without acquiring business authority.

### 3.2 Upstream Constraints

1. The scheduling problem is supplied by an owning business workflow.
2. Computation uses authoritative facts supplied by canonical owners.
3. Business constraints are supplied by authoritative context; the engine does not invent them.
4. Hard constraints are never silently converted into soft constraints.
5. Optimization objectives are explicitly supplied or established by the owning architecture.
6. A feasible/optimal computational result remains a candidate until accepted by the owning workflow.
7. Source-domain facts are not modified by computation.
8. Business lifecycle transitions and canonical events are not created by the engine.
9. Resource availability is authoritative input, not a resource master owned by the engine.
10. Common secure computational controls derive from L5-A-07-001.

### 3.3 Assumptions

- **AS-001:** A consuming workflow can define a bounded scheduling scope.
- **AS-002:** Required academic, faculty, resource, calendar and other source facts can be supplied or referenced from authoritative owners.
- **AS-003:** Applicable hard/soft constraints can be represented for controlled evaluation.
- **AS-004:** The consuming workflow can identify applicable objective/comparison criteria.
- **AS-005:** The consuming workflow remains responsible for interpretation and business decisions.

These assumptions do not establish new institutional policy or business rules.

### 3.4 Architectural Interpretation of Optimization

Optimization is a technical search and evaluation problem. The engine may compare candidates according to supplied objectives but shall not decide that those objectives are institutionally authoritative or override mandatory constraints.

---

## 4. Technical Architecture

### 4.1 Architectural Model

`Problem Admission → Scope/Context Binding → Authoritative Input Resolution → Constraint Normalization → Feasibility/Conflict Analysis → Candidate Generation → Candidate Evaluation → Optimization/Ranking → Solution Validation → Evidence/Lineage → Controlled Handoff`

No stage grants business authority.

### 4.2 Problem Admission

A computation is admitted only when explicit scope and sufficient input context exist. The admitted problem identifies, as applicable, scheduling scope, activities/entities, eligible resources, time/calendar context, authoritative input references, hard/soft constraints, objectives and required output characteristics.

### 4.3 Scope Binding

Every computation is bound to explicit scheduling scope. The scope prevents accidental mixing of unrelated institutions, campuses, academic periods, programmes, sections, faculty assignments, resources or examination contexts. The engine enforces supplied scope; it does not define enterprise hierarchy.

### 4.4 Authoritative Input Resolution

The engine consumes source-owned facts without becoming their owner. Timetable computation may consume course/course-offering, faculty-assignment, academic-calendar, resource-availability, capacity and applicable policy/context facts. Material source inputs remain identifiable. Stale or invented substitution is prohibited where authoritative context is required.

### 4.5 Constraint Model

The engine distinguishes:
- **hard constraints** — mandatory feasibility conditions;
- **soft constraints** — preferences used among feasible candidates;
- **objective criteria** — supplied measures of candidate quality.

The engine shall preserve upstream semantics and shall not invent or redefine business constraints.

### 4.6 Constraint and Conflict Evaluation

The engine evaluates temporal compatibility, resource exclusivity, capacity, faculty availability, activity/resource eligibility, scope compatibility, dependencies and other supplied constraints as applicable.

Conflict findings remain technical findings. Institutional disposition remains upstream-owned.

### 4.7 Feasibility Analysis

Candidates violating mandatory constraints are not feasible regardless of optimization score. If no feasible solution is established within the admitted search boundary, the engine returns explicit infeasibility rather than silently relaxing constraints.

### 4.8 Candidate Generation

The engine may generate multiple candidates. Each material candidate remains distinguishable and traceable to its problem, input context, constraint context, identity and status. Candidate generation does not create a business timetable version.

### 4.9 Optimization and Ranking

For supplied objectives, the engine may evaluate feasibility, discard infeasible candidates, calculate objective measures, compare candidates and produce a ranked/selected computational candidate set. It shall not invent objective priority or manufacture a business preference where supplied criteria cannot distinguish candidates.

### 4.10 Multi-Objective Optimization

Where multiple objectives are supplied, their declared identity, ordering, weighting or comparison semantics shall be preserved. Business policy establishing those semantics remains outside this specification.

### 4.11 Determinism and Reproducibility

Where required, equivalent scheduling problem, authoritative inputs, constraint set/version, objective criteria and computation context shall produce verifiably equivalent computational outcomes subject to permitted execution conditions. Material nondeterminism shall be controlled or evidenced where reproducibility is required.

### 4.12 Search Boundary and Resource Containment

Computation shall support bounded controls over duration, search breadth, candidate count, solution depth, intermediate state size and computation budget as applicable. Exceeding a boundary produces an explicit incomplete/constrained/failed outcome rather than an unqualified optimality claim.

### 4.13 Optimality Claim Boundary

The engine distinguishes:
- feasible solution;
- best solution found within the permitted search boundary;
- proven optimum where actually established;
- incomplete/limited search outcome.

A best-found candidate shall not be labelled globally optimal without supporting proof.

### 4.14 Candidate Validation

Before handoff, a selected candidate is re-evaluated against admitted hard constraints and relevant context. Handoff fails where hard constraints are violated, required inputs are invalid, resources are unavailable, candidate integrity cannot be established or computation context no longer matches.

### 4.15 Stale Input Detection

Where required by the consuming workflow, the architecture detects invalidated/outdated source context and prevents an outdated candidate from being represented as current business authority.

### 4.16 Recalculation

Recalculation may occur after authoritative input, constraint or objective changes, a new workflow request, or candidate infeasibility. Each material recalculation remains distinguishable and prior computation evidence is not silently rewritten.

### 4.17 Parallel Candidate Evaluation

Independent candidate branches may be evaluated in parallel where supported, without changing constraint semantics, candidate identity, scope boundaries, source ownership or evidence requirements.

### 4.18 Conflict Explanation

Where feasible, conflict results identify the affected candidate element, violated constraint, relevant input references, hard/soft classification and feasibility/quality impact. The engine does not invent business remediation.

### 4.19 Controlled Handoff

The consuming workflow receives computation identity, scheduling scope, candidate identity, feasibility status, conflict findings, objective evaluation, ranking evidence, input/constraint context, execution status and optimality/quality qualification as applicable.

The consuming workflow remains responsible for acceptance, approval, publication, activation or rejection.

### 4.20 Business Authority Boundary

The engine shall never independently approve, publish or activate a timetable; approve an examination schedule; reserve a physical resource as a business decision; modify Faculty Assignment, Course Offering or Academic Calendar; resolve a business conflict by changing source facts; create a canonical lifecycle transition; or publish a canonical business event.

### 4.21 Timetable Domain Relationship

The canonical Timetable business domain remains owner of the Timetable aggregate and lifecycle. For L4-D-020, the engine may support technical generation, conflict analysis and optimization of a Draft timetable under supplied constraints; approval, publication and activation remain governed business actions.

### 4.22 Examination Planning Relationship

Examination Planning & Scheduling remains the business owner. L5-A-07-006 may provide reusable scheduling computation without acquiring examination schedule authority.

### 4.23 Resource Architecture Relationship

Resource availability and resource masters remain owned by canonical source domains. The engine may consume resource facts, detect conflicts and produce candidate assignments, but does not become the resource master.

### 4.24 L5-A-07-005 Relationship

L5-A-07-005 provides constrained declarative rule/formula evaluation. L5-A-07-006 may consume it for reusable calculations required by scheduling objectives or constraints. Use does not transfer rule or objective ownership.

### 4.25 L5-A-07-001 Relationship

L5-A-07-001 provides common secure computational execution controls. L5-A-07-006 specializes those controls for scheduling search, conflict analysis and optimization.

---

## 5. Dependencies / Integration

### 5.1 L5-A-07-001
Primary technical dependency for secure and bounded computation.

### 5.2 L5-A-07-005
Optional reusable dependency for constrained formula/rule evaluation required by scheduling objectives or constraints.

### 5.3 L5-A-08
Large scheduling computations may require background/job execution. L5-A-07-006 defines computation semantics; L5-A-08 owns generic background/job execution mechanics.

### 5.4 L5-D / L5-D-04 / L5-E
Provide applicable identity/security/trust, evidence, availability, capacity, isolation and recovery architecture. This specification does not redefine those responsibilities.

### 5.5 Business-Domain Inputs

The engine may consume authoritative facts from Section B academic/calendar domains, D01 Course & Course Offering, D02 Faculty Academic Assignment, Section G resource domains and Examination Planning & Scheduling where invoked.

### 5.6 Integration Boundary

Cross-domain interaction uses approved L4/L5 contracts and event architecture. This specification does not define API payloads, event schemas, transport, queues or brokers.

---

## 6. Technical Behavior / Failure Concerns

| Behavior | Required Outcome |
|---|---|
| Problem admission | Only sufficiently defined scheduling problems enter computation. |
| Scope binding | Every computation is explicitly scoped. |
| Input integrity | Material source inputs remain identifiable and authoritative. |
| Constraint integrity | Hard/soft distinction is preserved. |
| Conflict analysis | Constraint violations are detected as technical findings. |
| Feasibility | Hard-constraint violations prevent feasible status. |
| Candidate generation | Candidates remain distinguishable and traceable. |
| Optimization | Candidates are compared only against supplied objectives. |
| Optimality qualification | Feasible, best-found and proven-optimal outcomes are distinguished where relevant. |
| Resource containment | Search remains within explicit bounds. |
| Candidate validation | Selected candidates are revalidated before handoff. |
| Stale input handling | Invalidated context prevents stale candidates from being treated as current authority. |
| Authority separation | Technical results cannot independently approve/publish/activate schedules. |
| Evidence | Material computation lineage is preserved. |

### 6.1 Failure Categories

Admission, input, constraint, feasibility, resource, execution, staleness, validation and context failures are technical outcomes. Business continuation, remediation and approval remain governed by the consuming workflow.

### 6.2 No Silent Constraint Relaxation

The engine shall not silently relax, remove, reorder or reinterpret a mandatory constraint to obtain feasibility.

### 6.3 No Silent Input Substitution

The engine shall not replace unavailable authoritative facts with inferred, stale or locally invented values merely to continue optimization.

### 6.4 Concurrent Computation Isolation

Independent computations remain isolated unless shared inputs are explicitly supplied and governed.

---

## 7. Security / Trust / Evidence

### 7.1 Restricted Computational Trust Boundary

The engine operates only on explicitly admitted inputs, constraints and objective criteria under L5-A-07-001.

### 7.2 Prohibited Execution Capabilities

Scheduling definitions, constraints and optimization expressions shall not obtain unrestricted ability to execute arbitrary code/SQL, read/write arbitrary filesystem resources, make unrestricted network calls, access undeclared secrets or inspect unrelated execution state.

### 7.3 Protected Context

Applicable L5-D security and L5-B data-protection controls govern sensitive scheduling inputs. The scheduling engine does not create a parallel privacy policy.

### 7.4 Evidence Requirements

Material computations shall be capable of identifying computation identity, scheduling scope, source-input references, constraint-set identity/version, objective-set identity/version, candidate identity, feasibility result, conflict findings, optimization/ranking outcome, execution status, search-bound qualification and predecessor/superseding computation relationship as applicable.

### 7.5 Evidence Does Not Transfer Ownership

Technical evidence does not establish ownership of the business schedule, resource master, examination schedule, faculty assignment or other source entity.

### 7.6 Audit Integration

Technical evidence integrates with enterprise audit/evidence architecture without redefining canonical business audit events or publishers.

---

## 8. Conformance

An implementation conforms when it demonstrates that the capability:

1. admits only explicitly defined scheduling problems;
2. binds every computation to explicit scope;
3. preserves authoritative source-input identity;
4. preserves hard/soft constraint distinction;
5. detects mandatory constraint violations;
6. does not silently relax mandatory constraints;
7. produces distinguishable candidates where search is used;
8. evaluates/ranks candidates only against supplied objective criteria;
9. distinguishes feasible, best-found and proven-optimal outcomes where relevant;
10. enforces bounded computation;
11. identifies infeasibility explicitly when no feasible solution is established;
12. handles invalidated input context where required;
13. validates selected candidates before handoff;
14. preserves computation lineage and evidence;
15. keeps independent computations isolated;
16. does not mutate authoritative source-domain facts;
17. does not approve, publish or activate a business schedule autonomously;
18. does not become owner of timetable, examination or resource business decisions;
19. does not provide unrestricted code, SQL, filesystem or network execution;
20. remains compatible with L5-A-07-001.

Conformance is an L5 technical property and does not constitute approval of any business workflow or outcome.

---

## 9. Traceability

### 9.1 Atomic Traceability Method

The canonical atomic relationship is:

`(L4 Business Behavior ID, L5 Specification ID)`

L5-A-07-006 is reusable and may support multiple L4 behaviors. The mappings below identify L4 behaviors for which scheduling/conflict/optimization computation is a relevant technical contribution.

### 9.2 Direct Timetable Traceability

| L4 Behavior | L5-A-07-006 Technical Contribution |
|---|---|
| L4-D-020-BH-003 | Controlled timetable candidate generation under supplied business constraints. |
| L4-D-020-BH-004 | Computational conflict/constraint analysis supporting governed validation. |
| L4-D-020-BH-005 | Technical detection of critical constraint/resource conflicts supporting prevention of publication. |
| L4-D-020-BH-010 | Reusable computation support for governed timetable planning execution; event publication remains D03-owned. |
| L4-D-020-BH-012 | Technical validation computation supporting timetable validation; canonical event remains D03-owned. |

### 9.3 Timetable Revision Traceability

| L4 Behavior | L5-A-07-006 Technical Contribution |
|---|---|
| L4-D-021-BH-002 | Technical impact-analysis/computation support using authoritative academic, faculty and resource facts. |
| L4-D-021-BH-004 | Constraint/conflict/reservation analysis for the revised candidate. |
| L4-D-021-BH-005 | Technical validation supporting prevention of republication when required constraints fail. |
| L4-D-021-BH-007 | Candidate/validation computation while publication and version authority remain D03-owned. |

### 9.4 Examination Scheduling Boundary

| L4 Behavior | Technical Relationship |
|---|---|
| L4-I-007-BH-002 | Reusable scheduling computation may support preparation of an Examination Schedule from authoritative planning context. |
| L4-I-007-BH-004 | Reusable conflict-analysis computation may support parent-defined conflict-validation conditions. |
| L4-I-007-BH-005 | Reusable conflict/feasibility computation may identify unresolved scheduling conflicts. |

These are technical reuse relationships only. Examination Planning & Scheduling remains the business owner; the engine does not approve, publish or otherwise own the Examination Schedule.

### 9.5 Upstream Traceability

`L5-A-07-006 → Exact L4 Business Behavior → Locked L3 Scenario → L2 Workflow → L1 Domain Architecture`

---

## 10. Open Issues / Deferred Design

- exact optimization/search mechanism;
- exact constraint representation;
- exact candidate representation;
- exact resource bounds;
- exact reproducibility mechanism;
- exact evidence storage mechanism;
- exact interface/transport mechanism;
- exact concurrency mechanism.

Objective priority, constraint meaning, approval authority and business remediation remain upstream-owned.

Technology/product selection is intentionally outside the current authoring scope.

---

## Canonical Architecture Boundary Note

L5-A-07-006 is the reusable scheduling/conflict/optimization computation architecture.

`authoritative scheduling context + supplied constraints/objectives → L5-A-07-006 technical computation → candidate / feasibility / conflict / optimization result → owning business workflow`

The engine does not own the Timetable, Examination Schedule, resource master, business lifecycle, business approval, canonical business events or domain scheduling decisions.

**Status:** Draft — not approved or locked.
