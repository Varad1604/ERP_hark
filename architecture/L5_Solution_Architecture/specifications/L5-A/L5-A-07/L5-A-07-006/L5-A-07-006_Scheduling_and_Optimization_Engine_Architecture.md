# L5-A-07-006 — Scheduling & Optimization Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-006 |
| Specification Title | Scheduling & Optimization Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Technical Architecture Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Version | 1.0.0 |
| Status | Draft — Authoring Authorized; Not Locked |
| Author | Varad |
| Canonical Responsibility | Provide reusable technical scheduling, conflict-analysis and optimization computation without owning timetable, examination or resource business decisions. |

### Primary L5 Dependency

- **L5-A-07-001 — Secure Computational Engine Architecture Baseline**

### Related L5 Specifications

- **L5-A-07-005 — General Rules & Formula Engine Architecture**
- **L5-A-08-001 — Background Processing & Job Execution Architecture**
- **L5-A-08-002 — Bulk, Scheduled & Retryable Work Execution Architecture**
- **L5-D — Identity, Security & Trust Architecture**
- **L5-D-04 — Audit & Evidence Architecture**
- **L5-E — Reliability, Deployment & Operations Architecture**

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable L5 technical architecture for scheduling, conflict analysis and constrained optimization computations used by SutraERP business workflows.

The engine provides a controlled computational capability that can transform an explicitly supplied scheduling problem, authoritative facts and governed constraints into one or more technically evaluated scheduling candidates, conflict findings, feasibility findings or optimization outcomes.

The engine is a **computation capability, not a business authority**. It does not own the Timetable aggregate, Examination Schedule, physical resources, Faculty Assignment, Course Offering, Academic Calendar or any other business entity. It does not approve, publish, activate, retire or otherwise govern a business schedule.

### 2.2 Scope

This specification covers:

- controlled admission of a scheduling or optimization computation;
- explicit definition of scheduling scope;
- binding authoritative input facts supplied by consuming domains;
- representation and enforcement of hard and soft constraints supplied by the consuming business architecture;
- feasibility analysis;
- conflict detection and classification at the computational level;
- candidate schedule generation;
- optimization and ranking of candidate schedules against supplied objective criteria;
- deterministic or reproducible execution where required;
- bounded computational search;
- comparison of candidate solutions;
- controlled handling of infeasible, incomplete or conflicting inputs;
- technical evidence and lineage for computation;
- controlled handoff of computational findings to the owning business workflow.

### 2.3 Non-Goals

This specification does not define:

- ownership of the canonical Timetable aggregate;
- ownership of Examination Scheduling or Examination Schedule business decisions;
- ownership of physical infrastructure or resource masters;
- ownership of Faculty, Student, Course, Course Offering or Academic Calendar entities;
- institutional scheduling policy or business rules;
- approval or rejection authority;
- publication or activation authority;
- business lifecycle definitions;
- canonical business event definitions or publishers;
- arbitrary code execution;
- arbitrary SQL execution;
- unrestricted filesystem access;
- unrestricted network execution;
- API payloads, protocols or transport mechanisms;
- database schemas or persistence implementation;
- UI/UX behavior;
- deployment topology;
- technology or product selection.

### 2.4 Ownership Boundary

The consuming business domain remains authoritative for:

1. the business scheduling problem being solved;
2. the entities and facts participating in that problem;
3. the applicable business constraints and policy;
4. the meaning and authority of the resulting business schedule;
5. approval, publication and activation decisions;
6. canonical lifecycle transitions and business events.

L5-A-07-006 owns only the **technical computation mechanism** used to evaluate feasible schedules, identify conflicts and optimize candidate solutions.

A technically optimal candidate is not automatically the business-approved or authoritative schedule.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Repository Applicability

The approved L5 corpus assigns L5-A-07-006 the responsibility of **reusable scheduling/conflict/optimization computation without owning timetable/examination/resource business decisions**.

The decomposition is intentionally separate from the business ownership of scheduling domains. The technical engine may be reused by the canonical Timetable domain and by other authorized scheduling workflows, including examination planning where the upstream architecture requires such computation, without acquiring their business authority.

### 3.2 Upstream Constraints

The architecture shall preserve the following constraints:

1. The scheduling problem shall be supplied by an owning business workflow.
2. The computation shall use authoritative facts supplied by their canonical owners.
3. Business constraints shall be supplied by the authoritative business context; the engine shall not invent them.
4. Hard constraints shall not be silently converted into soft constraints to obtain a candidate.
5. Optimization objectives shall be explicitly supplied or otherwise established by the owning architecture.
6. A technically feasible or optimal result shall remain a candidate computational outcome until accepted by the owning business workflow.
7. The engine shall not change source-domain facts while computing a solution.
8. The engine shall not create business lifecycle transitions or canonical business events.
9. Resource availability used by computation shall be treated as authoritative input, not as a resource master owned by the engine.
10. The computation boundary shall inherit the common secure execution controls established by L5-A-07-001.

### 3.3 Assumptions

- **AS-001:** A consuming workflow can define a bounded scheduling scope before computation begins.
- **AS-002:** Required academic, faculty, resource, calendar and other source facts can be supplied or referenced from authoritative owners.
- **AS-003:** Applicable hard constraints and permissible soft constraints can be represented in a form suitable for controlled evaluation.
- **AS-004:** The consuming workflow can identify which objective or comparison criteria apply to a scheduling computation.
- **AS-005:** The consuming workflow remains responsible for interpreting computational findings and obtaining required business decisions.

These assumptions do not establish new institutional policy or business rules.

### 3.4 Architectural Interpretation of Optimization

Optimization is treated as a **technical search and evaluation problem**.

The engine may determine that one candidate scores better than another according to supplied objective criteria. It shall not decide that those objective criteria are institutionally authoritative, nor may it override a mandatory business constraint because doing so produces a better optimization score.

---

## 4. Technical Architecture

### 4.1 Architectural Model

The engine follows a controlled computational pipeline:

```text
Scheduling Problem Admission
            ↓
Scope & Context Binding
            ↓
Authoritative Input Resolution
            ↓
Constraint Normalization
            ↓
Feasibility / Conflict Analysis
            ↓
Candidate Generation
            ↓
Candidate Evaluation
            ↓
Optimization / Ranking
            ↓
Solution Validation
            ↓
Evidence & Lineage Capture
            ↓
Controlled Handoff
```

No stage grants business authority to the engine.

### 4.2 Scheduling Problem Admission

A scheduling computation shall be admitted only when the technical engine can establish an explicit computation scope and sufficient input context.

The admitted problem shall identify, as applicable:

- scheduling scope;
- entities or activities to be scheduled;
- eligible resources or resource references;
- relevant time or calendar context;
- authoritative input references;
- hard constraints;
- soft constraints where applicable;
- optimization objectives where applicable;
- required output characteristics;
- execution purpose or reason where required by the consuming workflow.

An incomplete or internally inconsistent problem shall not be represented as a successful scheduling computation.

### 4.3 Scope Binding

Every computation shall be bound to an explicit scheduling scope.

The scope shall prevent accidental mixing of unrelated:

- institutions;
- campuses;
- academic periods;
- programmes;
- sections;
- faculty assignments;
- resources;
- examination contexts;
- other business scheduling subjects.

The engine enforces the scope supplied by the caller. It does not define the enterprise hierarchy.

### 4.4 Authoritative Input Resolution

The engine shall consume source-owned facts without becoming their owner.

For an academic timetable computation, examples may include authoritative:

- course and course-offering facts;
- faculty-assignment facts;
- academic-calendar facts;
- resource-availability facts;
- capacity facts;
- applicable policy/context references.

The engine shall preserve the identity of material source inputs used by the computation.

It shall not silently substitute a locally stored stale or altered fact when the consuming architecture requires an authoritative source.

### 4.5 Constraint Model

The technical architecture shall distinguish at least:

- **hard constraints** — conditions that a candidate must satisfy to be considered feasible;
- **soft constraints** — conditions used to compare or improve otherwise feasible candidates;
- **objective criteria** — supplied criteria used to evaluate relative candidate quality.

The engine shall preserve the distinction supplied by the consuming business architecture.

It shall not autonomously redefine a hard constraint as a preference or create a new mandatory constraint.

### 4.6 Constraint Evaluation

The engine shall evaluate candidate assignments against the admitted constraint set.

Constraint evaluation shall support, as applicable:

- temporal compatibility;
- resource exclusivity;
- capacity compatibility;
- faculty availability;
- activity/resource eligibility;
- scope compatibility;
- dependency relationships;
- supplied sequencing restrictions;
- supplied institutional constraints.

The exact business meaning of each constraint remains upstream-owned.

### 4.7 Conflict Detection

The engine shall identify computational conflicts that prevent or degrade candidate feasibility according to the admitted constraints.

Conflict analysis shall distinguish, where supported by the supplied constraint model:

- conflicting activities;
- conflicting resource assignments;
- capacity violations;
- temporal overlaps;
- unavailable resources;
- incompatible assignment combinations;
- unresolved prerequisite relationships;
- other supplied constraint violations.

A conflict finding is a **technical finding**. The engine does not decide the institutional disposition of the conflict.

### 4.8 Feasibility Analysis

Before optimization, the engine shall determine whether a candidate satisfies all required hard constraints.

A candidate that violates a mandatory constraint shall not be represented as a feasible solution merely because its optimization score is high.

If no feasible solution exists within the admitted problem and search boundary, the engine shall return an explicit infeasibility outcome rather than silently relaxing mandatory constraints.

### 4.9 Candidate Generation

Where the scheduling problem requires search, the engine may generate multiple candidate assignments.

Candidate generation shall operate only within the admitted scope and constraint model.

Each material candidate shall be distinguishable so that later evaluation and evidence can establish:

- the problem against which it was generated;
- the applicable input context;
- the constraint context;
- the candidate identity;
- the candidate status.

Candidate generation does not create a business timetable version unless the owning business workflow performs that action.

### 4.10 Optimization and Ranking

The engine shall support optimization where the consuming architecture supplies objective criteria.

The technical process may:

1. evaluate candidate feasibility;
2. discard candidates that violate mandatory constraints;
3. calculate objective measures for feasible candidates;
4. compare candidates using the supplied criteria;
5. produce a ranked or selected computational candidate set.

The engine shall not invent the relative importance of business objectives.

Where two candidates cannot be distinguished under the supplied objective criteria, the engine shall not manufacture a business preference solely to force a result.

### 4.11 Multi-Objective Optimization

Where multiple optimization objectives are supplied, the engine shall preserve their declared ordering, weighting or comparison semantics rather than silently replacing them.

The technical architecture shall be capable of representing:

- objective identity;
- objective direction where supplied;
- objective priority or weight where supplied;
- candidate score/evaluation evidence;
- comparison outcome.

The exact business policy establishing objective priority remains outside this specification.

### 4.12 Deterministic and Reproducible Execution

Where reproducibility is required, equivalent:

- scheduling problem;
- authoritative input context;
- constraint set/version;
- objective criteria;
- computation context

shall produce a verifiably equivalent computational outcome, subject to the permitted architecture and execution conditions.

Any nondeterministic behavior that could materially alter a result shall be controlled or explicitly evidenced where reproducibility is required.

### 4.13 Search Boundary and Resource Containment

Scheduling and optimization can require substantial computational search. The engine shall therefore operate within explicit execution boundaries.

The technical architecture shall support bounded controls over, as applicable:

- execution duration;
- search breadth;
- candidate count;
- solution depth;
- intermediate state size;
- computation budget.

Exceeding a computation boundary shall result in an explicit incomplete, constrained or failed technical outcome rather than an unqualified claim of optimality.

### 4.14 Optimality Claim Boundary

The engine shall distinguish among:

- feasible solution;
- best solution found within the permitted search boundary;
- proven optimum, where the computational method actually establishes it;
- incomplete/limited search outcome.

It shall not label a candidate as globally optimal when the computation only establishes that it is the best candidate found within a bounded search.

This distinction is important because the owning business workflow may require a valid schedule without requiring mathematical proof of global optimality.

### 4.15 Candidate Validation

Before handoff, a selected computational candidate shall be re-evaluated against the admitted hard constraints and relevant input context.

The final technical candidate shall be rejected from successful handoff when:

- a hard constraint is violated;
- a required input has become invalid;
- the candidate references an unavailable resource;
- candidate integrity cannot be established;
- the computation context no longer matches the candidate.

This validation is technical validation; it does not replace business approval.

### 4.16 Stale Input Detection

Where source facts can change during or between computations, the architecture shall be capable of detecting that the computation was based on an outdated or invalidated context when such detection is required by the consuming workflow.

A stale-input condition shall prevent an outdated candidate from being represented as a current authoritative business schedule.

The engine does not decide which source fact is correct; the canonical owner remains authoritative.

### 4.17 Recalculation

A scheduling computation may be repeated when:

- authoritative inputs change;
- constraints change;
- the objective context changes;
- the consuming workflow requests a new candidate;
- a prior candidate becomes infeasible.

Each material recalculation shall remain distinguishable from the preceding computation.

The prior computation shall remain traceable and shall not be silently rewritten.

### 4.18 Parallel Candidate Evaluation

The technical architecture may evaluate independent candidate branches in parallel where supported by the execution model.

Parallel execution shall not change:

- the admitted constraint semantics;
- candidate identity;
- scope boundaries;
- source fact ownership;
- evidence requirements.

Technical parallelism is an execution mechanism, not a business decision.

### 4.19 Conflict Explanation

Where computationally feasible, conflict results shall preserve enough information to explain:

- which candidate element was affected;
- which constraint was violated;
- which relevant input references were involved;
- whether the conflict was hard or soft;
- whether the conflict prevented feasibility or merely affected optimization quality.

The engine shall not invent a business remediation decision from the conflict explanation.

### 4.20 Controlled Handoff

The output handed to the consuming workflow shall identify, as applicable:

- computation identity;
- scheduling scope;
- candidate identity;
- feasibility status;
- conflict findings;
- objective evaluation;
- ranking/selection evidence;
- input and constraint context;
- execution status;
- optimality/quality qualification where applicable.

The consuming business workflow remains responsible for acceptance, approval, publication, activation or rejection.

### 4.21 Business Authority Boundary

The engine shall never independently:

- approve a timetable;
- publish a timetable;
- activate a timetable;
- approve an examination schedule;
- reserve a physical resource as a business decision;
- modify a Faculty Assignment;
- modify a Course Offering;
- change an Academic Calendar;
- resolve a business conflict by altering source facts;
- create a canonical business lifecycle transition;
- publish a canonical business event.

It may compute recommendations, candidates, feasibility findings and conflict information for the owning workflow.

### 4.22 Relationship to Timetable Domain

The canonical Timetable business domain remains the owner of the Timetable aggregate and its business lifecycle.

For the L4-D-020 timetable scenario, the engine may support technical generation, conflict analysis and optimization of a Draft timetable under supplied constraints. The L4 behavior explicitly keeps approval, publication and activation as governed business actions; therefore the engine cannot treat a computed candidate as Approved, Published or Active.

### 4.23 Relationship to Examination Planning

The repository separately identifies Examination Planning & Scheduling as a business responsibility. L5-A-07-006 may provide reusable scheduling computation to that workflow where required.

Such reuse does not transfer examination scheduling ownership or authority to the engine.

The engine therefore supplies computation only; the examination business workflow remains responsible for its authoritative schedule and decisions.

### 4.24 Relationship to Resource Architecture

Physical resource availability and resource master information remain owned by their canonical source domain.

The engine may consume resource facts, detect conflicts involving those resources and produce candidate assignments. It shall not become the master of rooms, laboratories, equipment or other physical resources.

### 4.25 Relationship to L5-A-07-005

L5-A-07-005 provides constrained declarative rule/formula evaluation. L5-A-07-006 may consume that capability for reusable calculations when a scheduling objective or constraint requires a governed formula evaluation.

Use of the formula engine does not transfer ownership of the business rule or objective to L5-A-07-005 or L5-A-07-006.

### 4.26 Relationship to L5-A-07-001

L5-A-07-001 provides the common secure computational execution baseline. L5-A-07-006 specializes that baseline for scheduling search, conflict analysis and optimization.

Shared controls include controlled admission, context integrity, isolation, resource containment, reproducibility where required, output integrity and evidence.

---

## 5. Dependencies / Integration

### 5.1 L5-A-07-001 — Secure Computational Engine Architecture Baseline

Primary technical dependency for secure and bounded computation.

### 5.2 L5-A-07-005 — General Rules & Formula Engine Architecture

Optional reusable dependency for constrained formula/rule evaluation required by a scheduling objective or constraint.

### 5.3 L5-A-08 — Background Processing & Job Execution

Large scheduling computations may require controlled background execution. L5-A-07-006 defines the computation semantics and boundaries; L5-A-08 owns generic background/job execution mechanics.

### 5.4 L5-D — Identity, Security & Trust

Provides applicable trust, identity, authorization and protected-data controls. L5-A-07-006 does not create an independent identity or authorization model.

### 5.5 L5-D-04 — Audit & Evidence

Receives or governs technical computation evidence. L5-A-07-006 does not redefine enterprise audit semantics.

### 5.6 L5-E — Reliability, Deployment & Operations

Provides technical availability, capacity, isolation and recovery architecture. The scheduling engine defines computation-level failure outcomes but not deployment technology.

### 5.7 Business-Domain Inputs

The engine may consume authoritative facts from:

- Section B academic/calendar domains;
- D01 Course & Course Offering;
- D02 Faculty Academic Assignment;
- Section G resource domains;
- Examination Planning & Scheduling where the business workflow invokes the reusable engine.

The engine shall consume these facts without ownership transfer.

### 5.8 Integration Boundary

Cross-domain interaction shall occur through the approved L4/L5 contracts and event architecture. This specification does not define API payloads, event schemas, message transport, queues or brokers.

---

## 6. Technical Behavior / Failure Concerns

### 6.1 Required Technical Behaviors

| Behavior | Required Outcome |
|---|---|
| Problem admission | Only a sufficiently defined scheduling problem enters computation. |
| Scope binding | Every computation is bound to an explicit scheduling scope. |
| Input integrity | Material source inputs remain identifiable and authoritative. |
| Constraint integrity | Supplied hard/soft constraint distinction is preserved. |
| Conflict analysis | Constraint violations are detected and represented as technical findings. |
| Feasibility | Hard-constraint violations prevent successful feasible status. |
| Candidate generation | Candidates remain distinguishable and traceable. |
| Optimization | Candidates are compared only using supplied objective criteria. |
| Optimality qualification | The result distinguishes feasible, best-found and proven-optimal outcomes where relevant. |
| Resource containment | Search cannot exceed its permitted execution boundary without explicit incomplete/failure status. |
| Output validation | Handoff occurs only for an internally valid computational result. |
| Recalculation | New executions remain distinguishable from prior executions. |
| Stale input handling | Invalidated input context cannot silently produce a current authoritative result. |
| Evidence | Computation context and findings remain reconstructable. |
| Authority separation | Computation never becomes business approval/publication/activation. |

### 6.2 Failure Categories

**Input failure** — required authoritative input is missing, invalid or inconsistent.

**Scope failure** — the computation context is incomplete or crosses an unauthorized scheduling scope.

**Constraint failure** — a supplied constraint is malformed, unresolved or internally inconsistent.

**Infeasibility** — no candidate satisfies all mandatory constraints within the admitted problem.

**Search-bound failure** — the computation reaches its permitted search boundary before establishing the requested result qualification.

**Execution failure** — the computation cannot complete safely.

**Stale-context failure** — required source facts have become invalid for the computation.

**Candidate-integrity failure** — the candidate cannot be shown to correspond to the admitted problem and execution context.

**Output-validation failure** — the produced result does not satisfy the technical output requirements.

### 6.3 Failure Handling

A failed or incomplete computation shall not be presented as a successful authoritative business schedule.

The engine shall preserve sufficient technical evidence to distinguish:

- no feasible solution;
- feasible solution found;
- optimization completed;
- bounded search completed without proof of optimum;
- computation failed;
- computation interrupted;
- input context invalidated.

Business continuation or recovery remains governed by the consuming workflow.

### 6.4 No Silent Constraint Relaxation

The engine shall not silently relax, remove, reorder or reinterpret a mandatory constraint to obtain a feasible candidate.

If a mandatory constraint prevents feasibility, the computation shall expose that condition to the consuming workflow.

Any business decision to alter a constraint belongs to the authoritative business owner and must return as a new governed computation context.

### 6.5 No Silent Input Substitution

The engine shall not replace an unavailable authoritative fact with an inferred, stale or locally invented value merely to continue optimization.

Where the consuming architecture explicitly supplies a derived input as valid context, that derivation remains attributable to its owning architecture.

### 6.6 Concurrent Computation Isolation

Independent scheduling computations shall remain isolated from one another unless shared inputs are explicitly supplied and governed.

One computation shall not mutate another computation's candidate set, constraint context or result evidence.

---

## 7. Security / Trust / Evidence

### 7.1 Restricted Computational Trust Boundary

The scheduling and optimization engine is a restricted computational boundary derived from L5-A-07-001.

It shall operate only on explicitly admitted inputs, constraints and objective criteria.

### 7.2 Prohibited Execution Capabilities

Scheduling definitions, constraints and optimization expressions shall not obtain unrestricted ability to:

- execute arbitrary application code;
- execute arbitrary SQL;
- read or write arbitrary filesystem resources;
- make unrestricted network calls;
- access undeclared secrets;
- inspect unrelated execution state.

Any capability expansion requires explicit architecture governance.

### 7.3 Protected Context

Where scheduling inputs contain sensitive information, applicable L5-D security and L5-B data-protection controls shall govern their protection.

The scheduling engine shall not create a parallel privacy policy.

### 7.4 Evidence Requirements

For material scheduling computations, technical evidence shall be capable of identifying:

- computation identity;
- scheduling scope;
- source-input references;
- constraint-set identity/version where applicable;
- objective-set identity/version where applicable;
- candidate identity;
- feasibility result;
- conflict findings;
- optimization/ranking outcome;
- execution status;
- search-bound qualification where relevant;
- predecessor/superseding computation relationship.

### 7.5 Evidence Does Not Transfer Ownership

Evidence produced by this engine describes technical computation. It does not establish ownership of the business schedule, resource master, examination schedule, faculty assignment or other source entity.

### 7.6 Audit Integration

The technical evidence shall integrate with the enterprise audit/evidence architecture without redefining canonical business audit events or their publishers.

---

## 8. Conformance

An implementation conforms to this specification when it demonstrates that the scheduling and optimization capability:

1. admits only explicitly defined scheduling problems;
2. binds every computation to an explicit scope;
3. preserves the identity of authoritative source inputs;
4. preserves the supplied distinction between hard and soft constraints;
5. detects violations of mandatory constraints;
6. does not silently relax mandatory constraints;
7. produces distinguishable candidates where candidate search is used;
8. evaluates and ranks candidates only against supplied objective criteria;
9. distinguishes feasible, best-found and proven-optimal outcomes where relevant;
10. enforces bounded computational execution;
11. identifies infeasibility explicitly when no feasible solution is established;
12. detects or handles invalidated input context where required;
13. validates the selected computational candidate before handoff;
14. preserves computation lineage and technical evidence;
15. keeps independent computations isolated;
16. does not mutate authoritative source-domain facts;
17. does not approve, publish or activate a business schedule autonomously;
18. does not become the owner of timetable, examination or resource business decisions;
19. does not provide unrestricted code, SQL, filesystem or network execution through scheduling definitions;
20. remains compatible with the common secure computational baseline.

Conformance is an L5 technical property. It does not constitute approval of any business workflow or business outcome.

---

## 9. Traceability

### 9.1 Atomic Traceability Method

The canonical atomic relationship is:

`(L4 Business Behavior ID, L5 Specification ID)`

L5-A-07-006 is reusable and therefore may support multiple L4 behaviors. The mapping below identifies the L4 behaviors for which the repository currently establishes scheduling/conflict/optimization computation as relevant technical realization.

### 9.2 Direct Timetable Traceability

| L4 Behavior | L5-A-07-006 Technical Contribution |
|---|---|
| L4-D-020-BH-003 | Provides controlled timetable candidate generation under the supplied business constraints. |
| L4-D-020-BH-004 | Provides computational conflict/constraint analysis supporting governed validation. |
| L4-D-020-BH-005 | Provides technical detection of critical constraint/resource conflicts so the consuming workflow can prevent publication. |
| L4-D-020-BH-010 | Provides reusable computation support for governed timetable planning execution; event publication remains D03-owned. |
| L4-D-020-BH-012 | Provides technical validation computation supporting timetable validation; canonical business event remains D03-owned. |

### 9.3 Timetable Revision Traceability

| L4 Behavior | L5-A-07-006 Technical Contribution |
|---|---|
| L4-D-021-BH-002 | Provides technical impact-analysis/computation support using authoritative academic, faculty and resource facts. |
| L4-D-021-BH-004 | Provides constraint/conflict/reservation analysis for the revised candidate. |
| L4-D-021-BH-005 | Provides technical validation supporting prevention of republication when required constraints fail. |
| L4-D-021-BH-007 | Provides candidate/validation computation while publication and version authority remain D03-owned. |

### 9.4 Examination Scheduling Boundary

| L4 Behavior | Relationship |
|---|---|
| L4-I-007-BH-002 | Reusable scheduling computation may support preparation of an Examination Schedule from authoritative planning context. |
| L4-I-007-BH-004 | Reusable conflict-analysis computation may support evaluation of parent-defined conflict-validation conditions. |
| L4-I-007-BH-005 | Reusable conflict/feasibility computation may identify unresolved scheduling conflicts. |

The above is a technical reuse relationship only. Examination Planning & Scheduling remains the business owner; the engine does not approve, publish or otherwise own the Examination Schedule.

### 9.5 Upstream Traceability

The direct upstream chain is:

```text
L5-A-07-006
    ↓
Exact L4 Business Behavior
    ↓
Locked L3 Scenario
    ↓
L2 Workflow
    ↓
Locked L1 Canonical Ownership
```

For the principal timetable path:

- **L4-D-020** realizes **L3-D-020 — Academic Scheduling Requirement to Governed Operational Timetable**.
- **L4-D-021** realizes **L3-D-021 — Operational Timetable to Governed Revised Timetable**.
- **L2-D03** is the canonical Timetable workflow source.
- D03 remains the canonical owner of the Timetable aggregate and timetable business lifecycle.

For examination planning:

- **L4-I-007** realizes **L3-I-007 — Plan and Approve an Examination Schedule**.
- The engine provides reusable computation only.
- Examination Planning & Scheduling remains the business authority.

### 9.6 Ownership Traceability

| Concern | Canonical Owner | L5-A-07-006 Role |
|---|---|---|
| Timetable aggregate | D03 Timetable | Technical candidate/conflict/optimization computation. |
| Examination Schedule | Examination Planning & Scheduling | Reusable scheduling computation only. |
| Faculty Assignment | D02 / canonical People & Workforce ownership as applicable | Consumed authoritative fact only. |
| Course Offering | D01 | Consumed authoritative fact only. |
| Academic Calendar | Section B | Consumed authoritative fact only. |
| Physical resource master | Section G | Consumed authoritative fact/availability only. |
| Scheduling business policy | Owning business domain | Constraint/objective input only. |
| Approval | Authoritative business role/workflow | No approval authority. |
| Publication | Canonical owning workflow | No publication authority. |
| Activation | Canonical owning workflow | No activation authority. |
| Canonical business events | Owning business domain | No independent publisher authority. |

### 9.7 Cross-L5 Traceability

| Peer L5 Specification | Relationship |
|---|---|
| L5-A-07-001 | Common secure computational baseline. |
| L5-A-07-005 | Optional constrained formula/rule evaluation dependency. |
| L5-A-08-001 | Generic background/job execution for long-running scheduling computation. |
| L5-A-08-002 | Generic scheduled/bulk/retryable execution where separately required; business retry policy remains upstream-owned. |
| L5-D | Security/trust controls. |
| L5-D-04 | Audit/evidence architecture. |
| L5-E | Reliability, capacity and operations architecture. |

### 9.8 Traceability Limitation

This specification does not treat a generic statement such as “supports scheduling” as complete traceability. Each production mapping shall use the exact L4 Business Behavior ID and the exact L5 specification ID.

Where future L4 behaviors consume this reusable engine, their exact BH-to-L5 relationship shall be added through controlled traceability rather than silently expanding this document's business ownership.

---

## 10. Open Issues / Deferred Design

The following matters remain intentionally deferred to later architecture or controlled implementation decisions:

1. Exact algorithm families and search strategies for particular scheduling problem classes.
2. Exact technical representation of constraint sets and objective functions.
3. Exact persistence representation for candidate sets and computation lineage.
4. Exact API/interface contracts.
5. Exact event transport and delivery mechanics.
6. Exact background execution mechanism for long-running optimization.
7. Exact concurrency and distributed-search implementation.
8. Exact deployment and infrastructure mechanisms.
9. Technology/product selection.

These deferred matters shall not alter the established responsibility boundary of L5-A-07-006.

### 10.1 Architectural Decisions Requiring Controlled Governance

Any future decision that would:

- allow arbitrary executable logic inside scheduling definitions;
- relax mandatory business constraints automatically;
- make the engine authoritative for a timetable or examination schedule;
- allow direct mutation of source-domain business facts;
- change canonical business-event ownership;
- expand the engine into resource-master ownership

requires explicit architecture governance before implementation.

### 10.2 Current Status

This specification is a **Draft** produced under the authorized L5 authoring phase. It is not L5-locked and does not constitute final corpus approval.
