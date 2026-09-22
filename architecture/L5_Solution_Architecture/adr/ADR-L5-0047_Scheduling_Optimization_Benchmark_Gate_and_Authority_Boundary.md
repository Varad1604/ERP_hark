# ADR-L5-0047 — Scheduling Optimization Benchmark Gate & Authority Boundary

**ADR ID:** `ADR-L5-0047`  
**Title:** Scheduling Optimization Benchmark Gate & Authority Boundary  
**Scope:** Specification  
**Decision Owner:** L5-A-07-006 — Scheduling & Optimization Engine Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-A-07-006`; constrains timetable/examination/resource-allocation consumers and F02 verification  
**Related ADRs:** `ADR-L5-0031`; `ADR-L5-0045`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

A07-006 correctly leaves solver family/algorithm unresolved. Selecting CP-SAT, MILP or a heuristic product now would be speculative because representative SutraERP scheduling workloads and constraint mixes have not yet been benchmarked.

## 2. Decision

1. **No solver product locked yet:** CP-SAT, MILP and bounded heuristic/other optimization approaches remain candidates until the benchmark gate is completed.
2. **Mandatory benchmark/PoC gate:** before a solver family/product becomes canonical, A07/F02 shall execute representative SutraERP scheduling PoCs across material scenario classes such as timetabling, examination scheduling and resource allocation where applicable.
3. **Evaluation dimensions:** the PoC evaluates at minimum:
   - hard-constraint correctness/coverage;
   - ability to represent required soft objectives without silently weakening hard rules;
   - feasibility discovery;
   - deterministic/reproducible behavior or explicitly governed stochastic behavior;
   - solution quality/objective score;
   - explainability/diagnostic evidence;
   - runtime and resource consumption;
   - scale behavior with representative data volume;
   - operational complexity/licensing/portability where a product is considered.
4. **Authority boundary:** the optimization engine consumes authoritative constraints/objectives and returns candidate solution(s). The source business domain remains owner of rules, acceptance/selection and resulting business state.
5. **Hard constraints:** hard constraints shall never be silently relaxed to obtain a solution.
6. **No AI authority:** probabilistic AI/ML is not the authoritative scheduling optimizer or rule validator. It may later assist search/recommendation/explanation under separate governance, but deterministic constraint validation remains authoritative.
7. **Selection ADR:** the benchmark outcome may propose a specific solver family/product through a later A07 technology ADR. That future ADR must cite benchmark evidence and migration/exit implications.
8. **Capacity integration:** solver runtime/resource envelopes are governed by E02 measured capacity profiles, not guessed universal limits.

## 3. Benchmark Exit Criteria

A solver recommendation is eligible only when:

- representative scenarios are documented;
- required hard constraints are expressible and verified;
- result quality is assessed against agreed objective criteria;
- failure/infeasibility is explainable enough for governed operational handling;
- resource/runtime evidence exists;
- deterministic/reproducibility characteristics are understood;
- portability/licensing/operational consequences are documented;
- no source-domain authority is transferred to the engine.

## 4. Consequences

SutraERP avoids premature solver lock-in while creating a concrete, testable route to a later implementation choice.

## 5. Approval Record

**Decision selected:** mandatory CP-SAT/MILP/heuristic benchmark gate; evaluate correctness, feasibility, reproducibility, solution quality, explainability, operational cost and runtime; source domain retains scheduling-rule/acceptance authority; AI/ML is non-authoritative; product selection deferred to evidence-backed ADR.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
