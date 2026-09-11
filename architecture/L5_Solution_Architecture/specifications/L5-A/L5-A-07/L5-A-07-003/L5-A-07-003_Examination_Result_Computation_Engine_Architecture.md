# L5-A-07-003 — Examination Result Computation Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-003 |
| Specification Title | Examination Result Computation Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Computation Engines |
| Technical Architecture Owner | Examination Result Computation Engine Architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft — Authoring Authorized; Not Locked |

**Canonical Responsibility:** Executes approved/versioned examination-result calculations. It cannot autonomously approve, finalise or publish official academic results.

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose
Define the technical architecture required to execute examination-result calculations within the Result Processing & Computation boundary using authoritative examination inputs and approved academic calculation context.

### 2.2 Scope
Calculation admission; candidate/examination scope binding; authoritative input resolution; rule/calculation-definition version binding; aggregation and derived-result computation; controlled application of parent-authorized grace, benefit, moderation, exception and arrangement consequences; recalculation and result lineage; competing-result handling; technical validation; failure containment; and calculation evidence.

### 2.3 Non-Goals
This specification does not define academic regulations or grading policy; source-mark ownership; business eligibility; I09 lifecycle semantics; approval/finalisation authority; official publication; canonical business-event meaning or publisher; APIs; payloads; database schemas; software classes; infrastructure; UI/UX; or technology/product selection.

### 2.4 Ownership Boundary
I09 Result Processing & Computation remains the canonical business owner of the computed academic outcome. Marks Collection & Evaluation and other source domains retain authority over their facts. This specification owns only the technical execution architecture used to calculate an I09-owned result.

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 L4 Applicability
**L4-I-083-BH-001:** admit computation only when parent-authorized Final Approved Marks and required candidate, examination and academic context are valid.

**L4-I-083-BH-002:** execute parent-governed aggregation, academic-rule, grace/benefit, subject-result, grade, credit, SGPA/CGPA, programme-result and academic-standing behavior without altering approved marks.

**L4-I-083-BH-008 / BH-009:** prevent unsupported progression on invalid/incomplete computation context and preserve attributable computation evidence.

The architecture also directly supports the computation portions of L4-I-084, I085, I086, I087, I088, I089, I090, I091, I093, I094, I095, I096, I097, I099 and I100, including re-conduct, blocking-condition continuation, pre-approval recomputation, corrected-mark recomputation, moderation, grace/benefit, arrangement, competing computation, inconsistent outcome, malpractice, exception, approval-blocking, reopening and pre-publication replacement behaviors.

L4-I-101 is a boundary reference: I09 remains owner of academic computation while I10 owns official publication.

### 3.2 Constraints
1. Only parent-authorized Final Approved Marks may be treated as authoritative marks input where the parent requires them.
2. Source facts remain owned by their canonical domains.
3. Academic rules and policy are consumed from approved authoritative context; the engine does not invent rules.
4. Approved source marks are never modified by computation.
5. Technical success does not equal academic approval, finalisation or publication.
6. Recalculation preserves distinction between previous and newly computed outcomes.
7. Blocking conditions are not converted into invented lifecycle states or events.
8. Grace, benefit, moderation, exception and arrangement consequences must originate from authoritative parent-supported context.

### 3.3 Assumptions
- The I09 workflow supplies a complete candidate/examination/result calculation context.
- Approved academic calculation definitions and rule versions have stable identities.
- The parent establishes authoritative source and scope before computation.
- The consuming workflow distinguishes a computed result from an institutionally approved result.
- Calculation evidence can reference source facts and rule versions without taking ownership of them.

## 4. Technical Architecture

### 4.1 Execution Model
**Admission → Scope Binding → Authoritative Input Resolution → Rule/Definition Binding → Dependency Preparation → Computation → Technical Validation → Result Versioning/Supersession → Evidence → Handoff.** A mandatory stage that cannot be satisfied prevents successful completion.

### 4.2 Computation Admission
A request shall identify candidate/result/examination scope, required authoritative inputs, applicable approved calculation/rule context and the expected result class. Admission validates technical completeness without independently deciding business eligibility.

### 4.3 Scope Binding
Each execution binds to one candidate/examination/result scope. The engine shall prevent mixing marks, academic context, rules or result versions from incompatible scopes.

### 4.4 Authoritative Input Resolution
The engine consumes Final Approved Marks, candidate identity, examination context, organisation/institution context, academic context and approved policy/rule context from their canonical owners. It may also consume authoritative moderation, grace, benefit, exception, arrangement or correction outcomes where the parent behavior requires them. It shall not modify source facts.

### 4.5 Rule and Calculation Definition Binding
Each material computation identifies the approved academic calculation definition/rule context and version used. The version is retained in execution lineage so later rule changes cannot silently change a completed computation. The engine consumes approval/version state; it does not confer business approval.

### 4.6 Computation Stages
Where required by the parent calculation definition, execution may include subject-level result calculation, grade determination, credit determination, aggregate/result calculation, SGPA, CGPA, programme-result and academic-standing calculation, followed by parent-required consistency validation. Exact business formulas remain governed by approved academic rules.

### 4.7 Dependent Calculation Integrity
Downstream values shall not be finalized when a required predecessor is missing, invalid, outside the bound scope or unsupported by the approved calculation definition. Principal inputs and material dependent stages shall remain traceable.

### 4.8 Grace, Benefit, Moderation, Exception and Arrangement Consequences
The engine shall apply only consequences already established by authoritative I09 context. It shall bind the consequence to its authoritative source, preserve original inputs, calculate affected values and identify dependent values requiring recomputation. It shall not create new academic policy.

### 4.9 Blocking Conditions
An unresolved blocking condition is a result-processing control input. The engine shall not invent a lifecycle state or business event to represent the blockage. After authoritative resolution, the parent workflow may request supported continuation/recomputation from the preserved I09 position.

### 4.10 Recalculation and Supersession
Recalculation is a distinguishable execution, not a silent mutation of the earlier computation. A recalculation shall identify affected scope, authoritative reason, changed/new inputs, rule version, prior computation reference, new outcome and validation result. The consuming I09 workflow determines when the new result becomes current or approved.

### 4.11 Competing and Inconsistent Computations
Multiple computations for the same scope retain distinct identities and lineage. The engine supports parent-defined selection criteria and technical consistency checks but does not invent business selection rules. Non-selected or invalid computations remain historically traceable and do not silently compete with the selected current computation.

### 4.12 Result Validation
Validation shall include, as applicable, input completeness, scope consistency, rule-version consistency, numeric validity, dependency completeness, grade/credit consistency, aggregate consistency, SGPA/CGPA consistency, programme-result consistency, academic-standing consistency and recalculation/supersession consistency. Technical validation does not replace I09 business approval.

### 4.13 Result Integrity
A computed result shall carry sufficient identity to distinguish candidate/examination/result scope, execution, rule/calculation version, principal input versions, computation status, validation outcome and predecessor/supersession relationship where applicable.

### 4.14 Handoff Boundary
The engine returns result identity, calculation/rule identity, input lineage, computation outcome, validation evidence and prior-result relationship where applicable. I09 retains authority for validation, approval, finalisation and publication-readiness. Official publication remains outside this specification.

### 4.15 Common Computational Baseline
L5-A-07-001 supplies common secure execution, admission, isolation, determinism/reproducibility, resource-containment and computation-evidence controls. L5-A-07-003 specializes those controls for examination-result computation and does not redefine the common baseline.

## 5. Dependencies / Integration

### 5.1 L5-A-07-001 — Secure Computational Engine Architecture Baseline
Provides the common computational execution controls inherited by this specialized engine.

### 5.2 L5-A-07-005 — General Rules & Formula Engine Architecture
May provide reusable formula-evaluation mechanisms where applicable. It does not own I09 academic rules or result authority.

### 5.3 I09 Result Processing & Computation
Supplies business context, authoritative result-processing decisions and lifecycle authority. The computation engine returns technical results to I09.

### 5.4 Marks Collection & Evaluation
Remains authoritative for approved marks. The computation engine consumes them without mutation.

### 5.5 Cross-Domain Source Owners
Identity, organisation, academic and policy facts remain authoritative at their canonical owners. Consumption does not transfer ownership.

### 5.6 Security, Audit and Reliability
L5-D supplies security/trust controls, L5-D-04 supplies enterprise audit/evidence architecture, and L5-E supplies reliability/operations controls. This specification defines only the result-computation boundary.

## 6. Technical Behavior / Failure Concerns

| Behavior | Required outcome |
|---|---|
| Admission | Incomplete or incompatible result-computation context is rejected. |
| Scope integrity | Cross-candidate, cross-examination or incompatible-result input mixing is prevented. |
| Source integrity | Approved marks and canonical source facts are consumed without mutation. |
| Rule binding | Each material computation identifies its approved rule/calculation version. |
| Dependency integrity | Required predecessor calculations must be valid before dependent finalization. |
| Result calculation | Approved result calculation stages execute in governed dependency order. |
| Consequence application | Only parent-authorized academic consequences are applied. |
| Recalculation | New executions remain distinguishable and linked to prior results. |
| Conflict handling | Competing/inconsistent computations are contained and exposed for governed resolution. |
| Validation | Technically invalid computation cannot be handed off as valid. |
| Authority separation | Technical computation cannot approve, finalise or publish an official result. |
| Evidence | Calculation basis and execution lineage remain traceable. |

### 6.1 Failure Categories
**Input failure:** authoritative marks or required context are missing or invalid.

**Rule failure:** approved calculation/rule context cannot be resolved or is inconsistent.

**Dependency failure:** a required intermediate result is unavailable or invalid.

**Scope failure:** inputs or computations belong to incompatible scopes.

**Calculation failure:** execution cannot produce a valid computed outcome.

**Consistency failure:** required grade, credit, aggregate, SGPA/CGPA or other relationships fail technical validation.

**Conflict failure:** competing results require parent-defined selection or resolution.

**Authority failure:** execution attempts to proceed beyond the authority supplied by I09.

### 6.2 Failure Containment and Continuation
A failed execution shall not produce a valid authoritative result. Failure evidence shall identify execution scope, failed stage, relevant input/rule references and validation outcome. Continuation or re-execution begins only from a parent-supported I09 position. The engine does not invent retry policy or business recovery semantics.

## 7. Security / Trust / Evidence

### 7.1 Trust Boundary
The engine is a controlled technical boundary between authoritative examination context and calculated academic outcomes. It accepts authority established by upstream owners but does not create business authority.

### 7.2 Protected Inputs and Outputs
Integrity of approved-mark references, calculation/rule versions, computation scope, computed outcomes and evidence shall be protected according to L5-D architecture. Exact security mechanisms are outside this specification.

### 7.3 Evidence Requirements
Material calculation evidence should identify candidate/result/examination scope, computation execution identity, authoritative input references, calculation/rule definition and version, applicable consequence references, validation outcome, prior computation reference where recalculated, supersession/replacement relationship and completion/failure status. Evidence references source facts; it does not become their owner.

## 8. Conformance

A conforming implementation shall demonstrate that it:

1. admits computation only through a controlled boundary;
2. binds each execution to a defined candidate/examination/result scope;
3. consumes authoritative marks without mutation;
4. binds an approved calculation/rule definition and version;
5. controls calculation dependencies;
6. applies only parent-authorized academic consequences;
7. preserves distinguishable recalculation and supersession lineage;
8. detects or contains competing/inconsistent computations as required;
9. rejects technically invalid computation results;
10. preserves calculation evidence;
11. cannot autonomously approve, finalise or publish an official academic result;
12. preserves I09 ownership of the computed academic outcome.

## 9. Traceability

### 9.1 Atomic Relation
The canonical atomic relation is **(L4 Business Behavior ID, L5 Specification ID)**. A validated relation record additionally binds the applicable **L4 Version** and **L5 Version**.

### 9.2 Direct / Supporting Mapping

| L4 Behavior | Applicability | L5 Contribution |
|---|---|---|
| L4-I-083-BH-001 | Direct | Controlled admission using parent-authorized Final Approved Marks and valid candidate/examination/academic context. |
| L4-I-083-BH-002 | Direct | Executes parent-governed result aggregation and derived academic calculations without modifying approved marks. |
| L4-I-083-BH-008 | Direct | Prevents unsupported progression when computation inputs/rules/validation prerequisites are invalid or incomplete. |
| L4-I-083-BH-009 | Direct | Produces attributable computation lineage and evidence. |
| L4-I-084-BH-003 | Direct | Computes replacement academic outcome from authoritative re-conduct outcome and approved evaluation input. |
| L4-I-084-BH-004 | Supporting | Preserves earlier computation as historical/non-current when a replacement exists. |
| L4-I-085-BH-001 | Direct | Evaluates mandatory result conditions at the computation boundary. |
| L4-I-085-BH-002 | Direct | Prevents unsupported progression when mandatory result conditions remain unsatisfied. |
| L4-I-086-BH-003 | Direct | Supports continuation from the preserved valid I09 position after authoritative blocking-condition resolution. |
| L4-I-086-BH-004 | Direct | Re-evaluates affected computations after blocking-condition resolution. |
| L4-I-087-BH-001 | Direct | Controls admission of pre-approval recomputation. |
| L4-I-087-BH-002 | Direct | Recomputes from authoritative approved marks and applicable rules. |
| L4-I-088-BH-002 | Direct | Recomputes dependent values from corrected authoritative marks. |
| L4-I-088-BH-003 | Direct | Preserves corrected marks as immutable source input. |
| L4-I-089-BH-002 | Direct | Applies established moderation consequence. |
| L4-I-089-BH-003 | Direct | Recomputes affected academic values. |
| L4-I-090-BH-001 | Direct | Consumes authoritative policy applicability/eligibility context. |
| L4-I-090-BH-002 | Direct | Calculates the supported grace/conditional consequence. |
| L4-I-090-BH-004 | Direct | Recomputes dependent values after the supported consequence. |
| L4-I-091-BH-002 | Direct | Applies only the supported consequence of an approved arrangement. |
| L4-I-091-BH-003 | Direct | Recomputes affected result values. |
| L4-I-093-BH-002 | Direct | Supports parent-defined selection criteria over competing computations. |
| L4-I-093-BH-003 | Direct | Supports establishment of the selected valid computation as current candidate for parent processing. |
| L4-I-093-BH-004 | Direct | Preserves non-selected computations as historical/non-authoritative. |
| L4-I-094-BH-001 | Direct | Detects mutually inconsistent computed outcomes. |
| L4-I-094-BH-002 | Direct | Re-evaluates from authoritative inputs and rules. |
| L4-I-094-BH-004 | Direct | Keeps conflicting invalid computations outside approval/publication readiness while retaining traceability. |
| L4-I-095-BH-002 | Direct | Applies authoritative malpractice consequence. |
| L4-I-095-BH-003 | Direct | Recomputes or constrains affected results. |
| L4-I-096-BH-002 | Direct | Applies the result-processing consequence of authoritative exception resolution. |
| L4-I-096-BH-003 | Direct | Recomputes dependent academic values. |
| L4-I-097-BH-001 | Direct | Recognizes approval-blocking condition at result-processing boundary. |
| L4-I-097-BH-002 | Direct | Prevents unsupported approval/finalisation/publication readiness. |
| L4-I-097-BH-003 | Direct | Preserves valid I09 state without inventing a synthetic `Withheld` lifecycle outcome. |
| L4-I-097-BH-004 | Direct | Associates blocking reason/source references with computation context. |
| L4-I-099-BH-002 | Direct | Preserves approved history while admitting corrected computation after reopening authority. |
| L4-I-099-BH-003 | Direct | Recomputes corrected inputs/rules/dependent values. |
| L4-I-100-BH-002 | Direct | Binds computation to parent-authorized replacement basis. |
| L4-I-100-BH-003 | Direct | Recomputes, validates and prepares replacement computation for parent approval. |
| L4-I-100-BH-004 | Supporting | Preserves prior replaced result as historical/non-current. |

## 10. Open Issues / Deferred Design

- Exact L4 applicability/version/status remains subject to repository atomic traceability validation.
- Concrete implementation mechanisms remain subject to downstream design and governed ADRs.
- No technology or product selection is made by this specification.
