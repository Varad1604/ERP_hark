# Financial Definition Change to Approved KPI or Analytical Model

# 1. Scenario Metadata

**Scenario ID:** L3-F-076  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics; L2-A04 — Authorization & Policy; L2-A07 — Audit Infrastructure  
**Referenced By:** KPI calculation, financial analytics, forecasting and dashboard-projection scenarios

---

# 2. Scenario Purpose

Govern a changed financial KPI, metric, forecast or Analytical Model definition as an immutable successor that becomes effective for future calculation while preserving historical reproducibility.

---

# 3. Business Outcome

A versioned definition becomes effective for future calculation while historical results remain reproducible

L2-F13 establishes KPI Definition and Analytical Model ownership, versioned configuration and historical reproducibility principles, but defines no definition lifecycle, workflow, commands, approval actors or definition-version events. This scenario is `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized governance decision changes a financial KPI, metric, forecast, or analytical definition.

## 4.2 End Boundary

The intended successor definition is approved and effective for future calculations, while prior definition versions and their calculated results remain reproducible. Exact canonical completion is undefined upstream.

## 4.3 In Scope

- Change rationale and ownership.
- Successor definition/version composition.
- Formula, input, scope and compatibility validation.
- Approval and future-effective activation.
- Historical definition/result preservation and audit.

## 4.4 Out of Scope

- Recalculating or rewriting historical KPI results.
- Modifying canonical financial facts.
- Dashboard composition and presentation.
- Formula engine, BI tool, API, database or UI implementation.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Followed by | L3-F-077 — Financial Facts to KPI and Dashboard Projection | Uses the effective definition; it does not approve definitions. |
| Related | L3-F-079 — Financial Evidence Reconstruction | Reproduces results under historical versions. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Financial Reporting & Analytics / F | Owns KPI Definition, Analytical Model and reporting configuration. |
| Operational Finance domains / F | Own input facts referenced by definitions. |
| Authorization & Policy / A | Supplies privileged authorization. |
| Audit Infrastructure / A | Preserves immutable version and approval evidence. |

---

# 6. Business Actors, Roles and Responsibilities

L2-F13 defines no canonical actors for definition authoring, validation, approval or activation. A Domain Owner, Finance Analyst or Approver is not invented.

---

# 7. Preconditions and Entry Criteria

- Tenant, institution and definition ownership are resolved.
- Current definition/version and historical results are discoverable.
- Changed formula, inputs, scope, effective date and rationale are available.
- Canonical financial input contracts exist.
- Approval and version-governance policy can be evaluated, although its actors/contracts are absent.

---

# 8. Trigger

An authorized governance decision changes a financial KPI, metric, forecast, or analytical definition

---

# 9. Main Business Flow

| Step | Intended Business Behaviour | Responsible Domain | Intended Result |
|---|---|---|---|
| 1 | Receive and authorize the definition-change decision. | F13 | Governed change begins. |
| 2 | Resolve the current definition, consumers and historical calculation references. | F13 | Impact scope is established. |
| 3 | Create an immutable successor definition/version. | KPI Definition / Analytical Model | Prior version remains unchanged; command/state undefined. |
| 4 | Define formula, canonical inputs, scope, parameters, effective date and compatibility. | F13 | Intended successor semantics are complete. |
| 5 | Validate determinism, source ownership, Tenant isolation and historical reproducibility. | F13 | Invalid definition cannot progress. |
| 6 | Complete required governance approval. | F13 | Intended approval is recorded; actor/transition undefined. |
| 7 | Make the successor effective for future calculations. | F13 | Intended active/effective version exists; state/command/event undefined. |
| 8 | Preserve previous versions and bind every calculation to its evaluated definition version. | F13; A07 | Historical outputs remain reproducible. |

---

# 10. Alternate Business Flows

| Case | Condition | Intended Path | Outcome |
|---|---|---|---|
| Future effective date | Approval precedes applicability. | Keep current version effective until the governed date. | No premature calculation change. |
| Validation rejected | Formula or inputs are invalid. | Return successor for correction. | Current version remains effective. |
| Breaking semantic change | Comparability is materially affected. | Record classification and consumer impact. | Historical results remain under prior meaning. |
| Definition retired | Metric is no longer calculated. | Govern terminal availability without deleting history. | Exact lifecycle is undefined. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Input source is non-canonical. | KPI would not be authoritative. | Reject definition. | F13 |
| Formula is non-deterministic. | Result cannot be reproduced. | Reject activation. | F13 |
| Approval is absent. | Successor cannot become effective. | Retain current version. | F13 |
| Activation is interrupted. | Effective definition is uncertain. | Do not use successor; exact recovery is undefined. | F13 |
| Historical reference would change. | Reproducibility is lost. | Reject mutation. | F13; A07 |

---

# 12. Business Rules and Decision Constraints

| Rule | Effect | Source |
|---|---|---|
| F13 owns KPI Definitions and Analytical Models. | Other domains cannot version them. | L2-F13 §5/17 |
| Configuration changes are version controlled. | In-place change is prohibited for historical meaning. | L2-F13 CP-04 |
| Historical reports retain evaluated configuration context. | Prior results remain reproducible. | L2-F13 CP-04 |
| Reporting never modifies financial facts. | Definition change is analytical only. | L2-F13 FRP-02 |
| Configuration changes are auditable. | Complete governance history is required. | L2-F13 CP-05 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Intended Impact | Result |
|---|---|---|---|
| Current KPI Definition / Analytical Model | Effective | Preserve as historical. | Immutable predecessor. |
| Successor Definition | Absent | Compose, validate, approve and activate. | Effective future version; canonical states undefined. |
| Historical KPI result | Existing | Retain evaluated definition reference. | Unchanged and reproducible. |
| Future calculation | Not yet run | Use successor after effective date. | Version-bound result. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Event Determination

F13 defines `FinancialKPICalculated` for calculation completion, but no KPI/Analytical Model definition created, versioned, approved, activated, superseded or retired events and publishers. No event is invented.

## 14.2 Contracts and Hand-offs

| Contract | Provider | Consumer | Outcome |
|---|---|---|---|
| Canonical input contract | Finance domains | Definition validation/calculation | Authoritative inputs. |
| Effective definition | F13 | KPI/analytics calculation | Formula, scope and version. |
| Historical definition | F13 | Audit/reconstruction | Prior evaluated meaning. |
| Audit input | F13 | A07 | Change and approval evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Definition changes require privileged Tenant-scoped authorization and never authorize financial transactions.

## 15.2 Approval and Separation of Duty

Approval Policy is configurable, but no actor, approval transition or SoD matrix is defined for KPI/Analytical Model changes.

## 15.3 Audit Obligations

Record predecessor/successor, formula, inputs, scope, effective date, validation, approval, activation, consumers, actor, timestamps and correlation.

---

# 16. Completion Criteria and Postconditions

The intended completion requires a versioned, validated, approved and effective successor plus immutable predecessor/result references. Missing lifecycle, commands, events and actors prevent exact canonical validation. The scenario remains `BLOCKED`.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue | Required Resolution |
|---|---|---|---|
| F076-BLOCKER-01 | Blocking | L2-F13 names KPI Definition and Analytical Model aggregates but defines no lifecycle states or transition matrices. | Define canonical definition-version lifecycles. |
| F076-BLOCKER-02 | Blocking | No create/version/validate/approve/activate/supersede/retire commands or workflow exists. | Define authoritative commands and workflow. |
| F076-BLOCKER-03 | Blocking | No definition-version events, publishers or approval actors exist. | Define exact events and governance authorities. |

No missing state, command, event, actor or formula implementation is supplied by inference.
