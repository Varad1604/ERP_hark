# L5-A-07-004 — Payroll Computation Engine Architecture

## 1. Specification Metadata

**Specification ID:** L5-A-07-004  
**Architecture Level:** L5 — Solution Architecture  
**Technical Architecture Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-07 — Computation Engines  
**Version:** 1.0.0  
**Status:** Draft  
**Author:** Varad  
**Canonical Responsibility:** Execute approved and versioned Payroll calculations and produce technically validated Payroll calculation results while preserving Payroll business authority for approval/finalization and Finance authority for downstream payment/accounting.

### Upstream Applicability

This specification is derived from Payroll computation behaviors including:

- L4-E-053 — Eligible Payroll Period to Finalized and Published Payroll Outcome
- L4-E-055 — Retrospective Change to Retroactive Pay Result
- L4-E-056 — Employee Separation to Final Payroll Settlement Result

### Primary L5 Dependency

- L5-A-07-001 — Secure Computational Engine Architecture Baseline

### Related L5 Specifications

- L5-A-07-005 — General Rules & Formula Engine Architecture
- L5-D — Identity, Security & Trust Architecture
- L5-D-04 — Audit & Evidence Architecture
- L5-E — Reliability, Deployment & Operations Architecture

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the technical architecture required to execute Payroll calculations within the People & Workforce domain.

The Payroll Computation Engine provides a controlled calculation boundary:

`authoritative workforce/payroll context + approved payroll rules`
→ `controlled payroll computation`
→ `technically validated payroll result`
→ `Payroll validation / approval / finalization`

The engine performs technical calculation. It does not become the business authority for Payroll lifecycle decisions or downstream Finance settlement.

### 2.2 In Scope

This specification covers:

- controlled admission of Payroll calculation executions;
- Payroll Run calculation context binding;
- consumption of authoritative Workforce, Attendance, Leave, Compensation, Compliance and Calendar facts;
- execution of approved/versioned Payroll calculation definitions;
- gross, deduction, recovery, adjustment and net-pay calculation where required by the parent behavior;
- retroactive/arrear difference calculation;
- final-settlement calculation;
- dependent calculation evaluation;
- monetary integrity and calculation precision;
- recalculation and correction execution;
- calculation validation and reconciliation;
- calculation-result identity, lineage and supersession;
- controlled failure and continuation;
- technical calculation evidence;
- handoff of validated Payroll calculation results to the owning Payroll workflow.

### 2.3 Explicit Non-Goals

This specification does not:

- define Payroll business rules or compensation policy;
- own Workforce, Attendance, Leave, Compensation, Compliance or Calendar facts;
- determine employee eligibility independently of authoritative Payroll context;
- create or approve a Payroll Run as a business authority;
- approve Payroll;
- finalize Payroll;
- publish canonical Payroll business events;
- execute payment;
- post accounting entries;
- own Finance settlement;
- directly edit finalized historical Payroll;
- define APIs, payloads or transport protocols;
- define database schemas or persistence mechanisms;
- define UI/UX;
- define infrastructure or deployment topology;
- select technology products or vendors.

### 2.4 Ownership Boundary

Payroll remains the canonical business owner of Payroll Run, Payroll calculation outcomes and Payroll lifecycle authority.

Upstream domains remain authoritative for the facts they own.

Finance remains authoritative for downstream payment and accounting.

L5-A-07-004 owns only the technical execution architecture required to calculate Payroll results.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 L4 Applicability

#### L4-E-053

**L4-E-053-BH-003** requires Payroll calculation using applicable governed rules and progression from `Calculating` to `Calculated`.

**L4-E-053-BH-004** requires validation of the calculated result before progression toward approval.

**L4-E-053-BH-008** requires preservation of calculation references and applicable rule versions for reproducibility and auditability.

The computation engine directly realizes the technical calculation portion of these behaviors.

#### L4-E-055

**L4-E-055-BH-001** identifies affected previously processed payroll periods.

**L4-E-055-BH-002** calculates the difference between the prior processed outcome and the outcome applicable under the retrospective compensation change.

**L4-E-055-BH-003** establishes the resulting difference through the governed arrear/retro-pay process rather than editing historical finalized Payroll.

**L4-E-055-BH-004** requires validation of the retroactive calculation before approval.

The computation engine performs the technical calculation and validation contribution while the Payroll workflow retains business authority.

#### L4-E-056

**L4-E-056-BH-002** determines applicable pending salary, deductions, recoveries and settlements from authoritative facts and employment dates.

**L4-E-056-BH-003** requires calculation and validation of the final-settlement Payroll result before approval.

The computation engine executes the required calculation using the authoritative context supplied by Payroll.

### 3.2 Constraints

1. Payroll calculations shall consume authoritative source facts without acquiring their ownership.
2. Applicable Payroll rules and effective versions shall be supplied by authoritative Payroll context.
3. Every material calculation execution shall be identifiable by Payroll scope and calculation-definition/version.
4. Historical finalized Payroll shall not be silently modified by recalculation.
5. Retroactive changes shall be represented through the governed Payroll adjustment boundary.
6. Technical calculation completion shall not equal Payroll approval or finalization.
7. Downstream payment and accounting remain Finance responsibilities.
8. The engine shall not invent Payroll policy when required rule context is absent.

### 3.3 Assumptions

- **AS-001:** Payroll can supply a complete calculation context before execution.
- **AS-002:** Applicable Payroll rule/calculation definitions have stable version identities.
- **AS-003:** Authoritative upstream facts can be referenced or supplied without ownership transfer.
- **AS-004:** A calculated Payroll result can be distinguished from an approved/finalized Payroll outcome.
- **AS-005:** Material calculation evidence can reference source facts, rule versions and calculation outcomes without becoming their canonical owner.

These assumptions do not create new business rules.

---

## 4. Technical Architecture

### 4.1 Architectural Model

The Payroll Computation Engine uses a controlled execution pipeline:

1. **Calculation Admission**
2. **Payroll Scope Binding**
3. **Authoritative Input Resolution**
4. **Rule/Definition Version Binding**
5. **Calculation Dependency Preparation**
6. **Payroll Calculation**
7. **Technical Validation**
8. **Result Finalization for Handoff**
9. **Lineage/Evidence Capture**
10. **Controlled Handoff to Payroll**

Business lifecycle transitions remain outside the computation engine.

### 4.2 Calculation Admission

A calculation shall be admitted only when the required Payroll context is present.

Depending on the parent behavior, this includes:

- Payroll Run or supported calculation scope;
- payroll period/type;
- employee scope;
- authoritative Workforce and employment context;
- Attendance/Leave context where applicable;
- Compensation context;
- Compliance context where applicable;
- Calendar context where applicable;
- applicable Payroll rules and effective versions;
- correction/retroactive/final-settlement context where applicable.

Admission checks technical completeness and compatibility. It does not independently establish business eligibility.

### 4.3 Payroll Scope Binding

Each execution shall be bound to an explicit Payroll scope.

The scope shall prevent accidental mixing of:

- employees;
- payroll periods;
- payroll types;
- employment-effective dates;
- incompatible compensation versions;
- unrelated adjustment contexts.

For batch Payroll calculations, the execution architecture shall preserve individual employee/result scope within the overall Payroll Run so that one employee's calculation cannot silently contaminate another employee's result.

### 4.4 Authoritative Input Resolution

The engine consumes, as applicable:

- employment and Workforce facts;
- effective employment dates;
- compensation facts;
- attendance facts;
- leave facts;
- compliance facts;
- calendar facts;
- Payroll configuration/rule context;
- authorized retrospective changes;
- authorized separation/final-settlement context.

The engine shall preserve references or identities for material inputs used in calculation.

It shall not modify authoritative source facts.

### 4.5 Rule and Version Binding

Every material Payroll calculation shall identify the applicable Payroll rule/calculation definition and effective version.

The binding supports:

- reproducibility;
- historical explanation;
- controlled recalculation;
- distinction between rule-version changes;
- evidence of the calculation context.

The engine consumes the authoritative rule/version context and does not decide whether a Payroll rule is institutionally approved.

### 4.6 Payroll Calculation Stages

Where required by the parent-defined Payroll calculation, execution may include:

1. eligible employee/context verification supplied by the parent;
2. earnings and compensation calculation;
3. attendance/leave-related calculation inputs;
4. deduction calculation;
5. recovery calculation;
6. applicable adjustment calculation;
7. gross-pay derivation;
8. net-pay derivation;
9. retroactive difference calculation;
10. final-settlement calculation;
11. dependent-value reconciliation;
12. technical validation.

The exact business formula remains governed by approved Payroll rules.

### 4.7 Monetary Integrity

Payroll calculations shall preserve explicit monetary semantics.

The execution context shall identify, where applicable:

- currency;
- precision;
- scale;
- rounding context;
- monetary input/output role;
- calculation stage.

The engine shall not invent exchange rates, statutory rates or business rounding policy.

Where a calculation depends on externally established statutory or institutional values, those values must be supplied through the authoritative Payroll context.

### 4.8 Dependency Evaluation

Payroll calculations may contain dependent values.

The engine shall establish calculation dependency order so that:

- required predecessor values are available before dependent evaluation;
- invalid predecessors cannot produce authoritative dependent values;
- recalculation affects all required downstream values;
- dependent values retain sufficient lineage.

A dependency that cannot be resolved shall prevent completion of the affected calculation.

### 4.9 Retroactive Calculation

For an authoritative retrospective compensation change, the engine shall support calculation of the difference between:

- the prior processed outcome; and
- the recalculated outcome applicable under the authorized retrospective change.

The engine shall preserve:

- affected period references;
- original calculation reference;
- retrospective change reference;
- applicable rule/version;
- recalculated outcome;
- resulting difference;
- validation evidence.

The engine shall not edit the original finalized historical Payroll record.

### 4.10 Final-Settlement Calculation

For an authoritative employee separation context, the engine shall support calculation of applicable:

- pending salary;
- deductions;
- recoveries;
- leave-related settlement values where supplied by the parent context;
- other supported settlement components.

The engine returns the calculated and technically validated final-settlement result to Payroll.

The engine does not perform downstream Finance payment or accounting.

### 4.11 Recalculation and Correction

A recalculation shall be represented as a distinguishable execution.

The engine shall preserve:

- prior computation identity;
- recalculation reason;
- changed authoritative input context;
- applicable rule/version;
- new result;
- validation outcome.

The prior calculation remains historically distinguishable.

The Payroll business workflow determines when the new calculation becomes the current business outcome.

### 4.12 Duplicate and Competing Calculations

Where the parent workflow requires controlled uniqueness, the architecture shall detect or contain duplicate executions for the same governed scope and calculation context.

The engine shall distinguish:

- repeated execution of the same context;
- legitimate recalculation following an authorized change;
- competing results requiring Payroll business resolution.

The engine shall not invent a business rule to choose between competing Payroll outcomes.

### 4.13 Result Validation

Before handoff, the engine shall validate, as applicable:

- calculation completeness;
- employee/payroll scope;
- rule/version consistency;
- monetary integrity;
- dependency completeness;
- earnings/deduction/recovery consistency;
- gross/net consistency;
- retroactive difference consistency;
- final-settlement component consistency;
- expected calculation-result structure.

Technical validation does not replace Payroll approval.

### 4.14 Result Integrity

A calculated Payroll result shall carry sufficient technical identity to distinguish:

- Payroll Run/scope;
- employee scope;
- calculation execution;
- calculation-definition/version;
- principal input references;
- result state;
- prior/replaced calculation where applicable;
- validation outcome.

A failed or incomplete calculation shall not be exposed as a valid Payroll result.

### 4.15 Controlled Handoff

A valid calculation result shall be handed to Payroll with:

- calculation identity;
- Payroll scope;
- result;
- rule/calculation-definition version;
- input lineage;
- validation outcome;
- recalculation/supersession relationship where applicable;
- technical execution evidence.

The Payroll workflow retains authority for approval, finalization and canonical Payroll events.

### 4.16 Common Computational Baseline

L5-A-07-001 supplies the common secure computation controls.

L5-A-07-004 specializes them for Payroll calculation and shall not redefine the common computational baseline.

---

## 5. Dependencies / Integration

### 5.1 L5-A-07-001 — Secure Computational Engine Architecture Baseline

Provides common:

- secure execution;
- admission control;
- resource containment;
- rule-package isolation;
- deterministic/reproducible execution where required;
- computation evidence;
- technical/business authority separation.

### 5.2 L5-A-07-005 — General Rules & Formula Engine Architecture

May provide reusable technical formula/rule evaluation capability.

It does not own Payroll policy or Payroll calculation authority.

### 5.3 Payroll Business Workflow

Payroll supplies authoritative calculation context and owns the Payroll Run lifecycle, validation/approval/finalization decisions and canonical Payroll events.

### 5.4 Workforce / Attendance / Leave / Compensation / Compliance / Calendar

These domains supply authoritative facts required by Payroll.

The computation engine consumes those facts without transferring ownership.

### 5.5 Finance

Finance receives finalized Payroll financial outcomes for downstream payment/accounting responsibilities.

L5-A-07-004 does not execute those downstream Finance responsibilities.

### 5.6 L5-D Security / Trust

Provides enterprise technical security and trust mechanisms for protected Payroll context and execution.

### 5.7 L5-D-04 Audit & Evidence

Receives or organizes technical calculation evidence within the enterprise evidence architecture.

### 5.8 L5-E Reliability / Operations

Provides execution availability, operational resilience, resource controls and platform recovery mechanisms.

---

## 6. Technical Behavior / Failure Concerns

### 6.1 Required Technical Behaviors

| Behavior | Requirement |
|---|---|
| Admission | Reject execution when required Payroll calculation context is incomplete. |
| Scope integrity | Prevent cross-employee, cross-period or incompatible Payroll context mixing. |
| Source integrity | Consume authoritative upstream facts without modifying them. |
| Rule binding | Bind material calculations to applicable Payroll rule/calculation-definition versions. |
| Calculation | Execute approved Payroll calculations in governed dependency order. |
| Monetary integrity | Preserve required currency, precision, scale and rounding semantics. |
| Retroactive calculation | Calculate authorized retrospective differences without editing finalized historical Payroll. |
| Final settlement | Calculate supported final-settlement components from authoritative context. |
| Recalculation | Preserve prior and new calculation identities and lineage. |
| Duplicate control | Detect/contain duplicate or competing executions where required by the parent workflow. |
| Validation | Prevent technically invalid Payroll calculations from being handed off as valid. |
| Authority separation | Technical calculation completion never equals Payroll approval/finalization or Finance settlement. |
| Evidence | Preserve sufficient calculation lineage for reproducibility and auditability. |

### 6.2 Failure Categories

**Input failure:** required authoritative Workforce, Compensation, Attendance, Leave, Compliance, Calendar or Payroll context is absent or invalid.

**Rule failure:** applicable Payroll calculation definition/version is missing or inconsistent.

**Scope failure:** employee, period, Payroll Run or calculation contexts are incompatible.

**Dependency failure:** a required intermediate Payroll value is unavailable or invalid.

**Calculation failure:** execution cannot produce a technically valid result.

**Monetary failure:** currency, precision, rounding or amount relationships are invalid.

**Validation failure:** required technical reconciliation does not pass.

**Conflict failure:** competing Payroll calculations require business-level resolution.

### 6.3 Failure Containment

A failed calculation shall not be presented as a valid Payroll result.

Failure evidence shall identify, where available:

- calculation scope;
- failed stage;
- relevant input references;
- rule/version;
- validation finding;
- parent continuation requirement.

The engine shall not synthesize a new Payroll lifecycle state or business event to represent technical failure.

### 6.4 Continuation and Re-execution

A continuation or re-execution shall use a parent-supported Payroll context.

A corrected execution shall remain distinguishable from the failed execution.

Historical finalized Payroll shall not be silently rewritten through re-execution.

---

## 7. Security / Trust / Evidence

### 7.1 Trust Boundary

The Payroll Computation Engine is a controlled technical execution boundary between authoritative Payroll context and calculated Payroll outcomes.

The engine accepts business authority established by Payroll and upstream owners but does not create business authority.

### 7.2 Protected Financial and Workforce Context

The engine shall preserve integrity of:

- calculation inputs;
- rule/calculation-definition versions;
- employee/payroll scope;
- calculated Payroll results;
- calculation lineage;
- validation evidence.

Specific enterprise security mechanisms remain under L5-D.

### 7.3 Calculation Evidence

Material Payroll calculation evidence should identify:

- Payroll Run/scope;
- employee scope;
- calculation execution identity;
- principal authoritative input references;
- rule/calculation-definition version;
- calculation stage/result references;
- validation outcome;
- prior calculation reference for recalculation;
- retrospective change reference where applicable;
- final-settlement context where applicable;
- execution completion/failure state.

Evidence references authoritative facts without taking ownership of them.

---

## 8. Conformance

A conforming implementation shall demonstrate that:

1. Payroll calculations are admitted only with required authoritative context.
2. Payroll scope is explicitly controlled.
3. Source-domain facts are consumed without mutation.
4. Material calculations identify the applicable rule/calculation-definition version.
5. Calculation dependencies are evaluated in controlled order.
6. Monetary semantics are preserved.
7. Authorized retrospective calculations preserve historical finalized Payroll.
8. Final-settlement calculations use the authoritative separation and employment context.
9. Recalculations remain distinguishable from prior calculations.
10. Technically invalid calculations cannot be handed off as valid.
11. Calculation evidence supports reproducibility and auditability.
12. Technical calculation completion cannot autonomously approve or finalize Payroll.
13. Payment execution and accounting remain Finance responsibilities.
14. Payroll remains the canonical business owner of Payroll outcomes and lifecycle authority.

---

## 9. Traceability

### 9.1 Atomic Traceability Method

The atomic relation is:

`(L4 Business Behavior ID, L5 Specification ID)`

### 9.2 Direct Mappings

| L4 Business Behavior | Applicability | L5 Architectural Contribution |
|---|---|---|
| L4-E-053-BH-003 | **Direct** | Executes governed Payroll calculation from `Calculating` toward a technically completed `Calculated` result. |
| L4-E-053-BH-004 | **Direct** | Performs technical validation of the calculated Payroll result before business approval progression. |
| L4-E-053-BH-008 | **Direct** | Preserves calculation references and rule versions required for reproducibility and auditability. |
| L4-E-055-BH-001 | **Supporting** | Provides technical calculation context across identified affected Payroll periods. |
| L4-E-055-BH-002 | **Direct** | Calculates the difference between prior processed Payroll and the outcome applicable under the authorized retrospective change. |
| L4-E-055-BH-003 | **Direct** | Produces the retro-pay calculation result without editing historical finalized Payroll. |
| L4-E-055-BH-004 | **Direct** | Technically validates the resulting retroactive Payroll calculation before Payroll approval. |
| L4-E-055-BH-007 | **Direct** | Preserves impacted-period, input, rule/version, calculation and resulting Payroll history. |
| L4-E-056-BH-002 | **Supporting** | Consumes authoritative settlement inputs and effective employment dates for calculation. |
| L4-E-056-BH-003 | **Direct** | Calculates and technically validates the final-settlement Payroll result before approval. |

### 9.3 Explicit Non-Ownership Mapping

The following behaviors remain outside computation-engine authority even where the engine technically contributes to their realization:

- L4-E-053-BH-001: Payroll Run establishment remains a Payroll business workflow responsibility.
- L4-E-053-BH-005: Payroll approval remains an authorized Payroll business decision.
- L4-E-053-BH-006: Payroll approval/finalization remains business-owned.
- L4-E-053-BH-009 through BH-014: canonical Payroll event publication remains Payroll-owned.
- L4-E-053-BH-015: downstream Finance payment/accounting remains Finance-owned.
- L4-E-055-BH-005/BH-006: Payroll approval/finalization remains business-owned.
- L4-E-055-BH-008/BH-009: canonical Payroll event publication remains Payroll-owned.
- L4-E-055-BH-010: ownership remains with source Compensation and downstream Finance; computation does not acquire either authority.
- L4-E-056-BH-001: Final Settlement Payroll Run establishment remains Payroll workflow responsibility.
- L4-E-056-BH-004/BH-005: Payroll approval/finalization remains business-owned.
- L4-E-056-BH-006: Finance receives the finalized financial outcome without transferring Payroll-result ownership.
- L4-E-056-BH-007: canonical Payroll event publication remains Payroll-owned.
- L4-E-056-BH-008: upstream Workforce and downstream Finance ownership remains unchanged.

### 9.4 Upstream Traceability Chain

Primary:

`L1 Section E — People & Workforce`

→ `L2-E06 — Payroll Workflows`

→ `L3-E-053 / L3-E-055 / L3-E-056`

→ `L4-E-053 / L4-E-055 / L4-E-056`

→ `L5-A-07-004 — Payroll Computation Engine Architecture`

Common technical foundation:

`L5-A-07-001 — Secure Computational Engine Architecture Baseline`

Reusable calculation capability where applicable:

`L5-A-07-005 — General Rules & Formula Engine Architecture`

---

## 10. Open Issues / Deferred Design

### 10.1 Open Technical Decisions

The following remain implementation-level decisions:

- exact computation-isolation mechanism;
- exact representation of Payroll calculation definitions and versions;
- exact monetary precision implementation;
- exact persistence/evidence mechanism;
- exact interface/transport mechanism;
- exact concurrency-control mechanism;
- exact retry/recovery mechanism.

### 10.2 Boundary Clarification

Future Payroll calculation behaviors shall be mapped to this specification at exact L4 BH level before being considered covered.

A calculation behavior that belongs to Finance, Examination, Scheduling or another canonical domain shall not be absorbed into the Payroll computation architecture.

### 10.3 Technology Selection

Technology/product selection is intentionally outside the current authoring scope.

---

## Canonical Architecture Boundary Note

L5-A-07-004 is the **technical execution architecture for Payroll calculation**.

The governing relationship is:

`authoritative Workforce/Payroll facts + approved Payroll rules`
→ `L5-A-07-004 Payroll computation`
→ `technically validated Payroll result`
→ `Payroll business validation / approval / finalization`
→ `Finance downstream payment/accounting`

The computation engine does not own Payroll policy, source facts, approval authority, finalized Payroll lifecycle, canonical Payroll events, payment execution or accounting posting.

**Status:** Draft — not approved or locked.
