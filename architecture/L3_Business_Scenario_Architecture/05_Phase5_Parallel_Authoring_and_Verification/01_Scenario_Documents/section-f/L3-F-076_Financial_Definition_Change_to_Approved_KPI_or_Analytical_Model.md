# Financial Definition Change to Approved KPI or Analytical Model

# 1. Scenario Metadata

**Scenario ID:** L3-F-076  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics Workflows  
**Referenced By:** KPI calculation, financial analytics and dashboard-projection scenarios

# 2. Scenario Purpose

Govern a change to a financial KPI, metric, forecast or Analytical Model definition so a versioned successor can become effective for future calculation without changing historical results or canonical financial facts.

# 3. Business Outcome

A versioned definition becomes effective for future calculation while historical results remain reproducible

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized governance decision changes a financial KPI, metric, forecast, or analytical definition.

## 4.2 End Boundary

The intended scenario end is an approved and effective successor definition whose future calculations use the new version while prior results retain their historical configuration context. Locked F13 does not define the lifecycle, approval authority, activation transition or events required to prove that end state, so completion is blocked.

## 4.3 In Scope

- Preservation of the existing KPI Definition or Analytical Model version.
- Composition and validation of a successor definition.
- Version-controlled configuration and historical reproducibility.
- Identification of missing approval/effective-state semantics.

## 4.4 Out of Scope

- Rewriting historical KPI or analytical results.
- Modifying operational Finance facts.
- Inventing definition lifecycle states, approval roles or definition-change events.
- Dashboard composition or presentation.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Reporting & Analytics / F | Owns KPI Definitions, Analytical Models, reporting configuration and their business meaning. |
| Applicable Finance capabilities / F | Retain ownership of the canonical financial facts referenced by definitions. |

# 6. Business Actors, Roles and Responsibilities

L2-F13 has no canonical Actors section and defines no business actor for KPI/Analytical Model definition authoring, approval or activation. No local actor is introduced.

# 7. Preconditions and Entry Criteria

- The current governed definition/model and configuration context can be identified.
- Historical results retain their existing evaluated configuration context.
- The proposed change remains within the Financial Reporting & Analytics ownership boundary.
- Required canonical Finance inputs are identifiable without transferring ownership.

# 8. Trigger

An authorized governance decision changes a financial KPI, metric, forecast, or analytical definition

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the existing KPI Definition or Analytical Model and the governed change decision. | Financial Reporting & Analytics / F | The subject and intended change are established. |
| 2 | Preserve the predecessor definition/model and its historical calculation context. | Financial Reporting & Analytics / F | Existing results remain reproducible. |
| 3 | Establish a version-controlled successor configuration containing the intended changed definition. | Financial Reporting & Analytics / F | A successor configuration candidate exists under CP-04. |
| 4 | Validate that the proposed definition uses canonical Finance information and preserves reporting ownership/invariants. | Financial Reporting & Analytics / F | The candidate is evaluated without altering source facts. |
| 5 | Stop before asserting approval, effective activation or supersession because F13 defines no authoritative lifecycle, transition, actor or business event for those outcomes. | Financial Reporting & Analytics / F | Upstream resolution is required before the frozen outcome can be completed. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture for KPI Definition or Analytical Model version activation.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Proposed definition would change or re-own canonical financial facts | The analytical definition would violate ownership | Reject the proposed definition change | F13 / applicable Finance owner |
| Historical configuration context would be lost | Prior results would cease to be reproducible | Do not replace the predecessor version | F13 |
| Approval/effective transition is required | The frozen final outcome cannot be completed canonically | Obtain an authoritative L2-F13 lifecycle and authority decision | F13 architecture owner |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| KPI Definitions and Financial Dashboard Information Projection configuration are owned by Financial Reporting | F13 | Definition meaning cannot be transferred to another domain | L2-F13 §44 |
| Configuration changes shall be version controlled | F13 | In-place replacement is not sufficient | L2-F13 CP-04 |
| Historical reports preserve the configuration context under which they were generated | F13 | Prior outcomes remain reproducible | L2-F13 CP-04 |
| Configuration changes shall generate immutable audit evidence | F13 | Change history must remain traceable | L2-F13 CP-05 |
| Configuration shall not modify financial facts or bypass report validation | F13 | Analytical change cannot alter operational truth | L2-F13 §45 |

# 13. State and Lifecycle Impact

F13 names KPI Definition and Analytical Model aggregates/configurations but does **not** define authoritative lifecycle states or transitions for definition versioning, approval, activation, supersession or retirement. No lifecycle transition is authored locally.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

`FinancialKPICalculated` is an authoritative **calculation-completion** event published by KPI Definition. It is not a definition-version approval or activation event. F13 defines no canonical event for KPI Definition or Analytical Model creation/version approval/effective activation/supersession/retirement, so no such event is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Applicable Finance domains provide canonical financial information for later calculations. The definition/model remains owned by F13 and source facts remain owned by their Finance capabilities.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The frozen trigger states that an authorized governance decision initiates the change. F13 does not define the actor or authority that approves the successor definition or makes it effective.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception rule is defined upstream.

## 15.3 Business Audit and Evidence Obligations

F13 requires immutable audit evidence for configuration changes and preservation of historical configuration context.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen scenario cannot be certified complete until authoritative lifecycle states/transitions, approval authority and definition-version business events are defined for KPI Definitions and Analytical Models.

## 16.2 Postconditions

The predecessor definition/model and historical result context remain preserved. No successor is asserted to be approved or effective by this Draft.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| No KPI Definition or Analytical Model lifecycle states/transitions are defined | The required approved/effective successor cannot be represented canonically | Define lifecycle and effective-version semantics in L2-F13 |
| No canonical business actor/authority is defined for definition approval/activation | The frozen authorization/approval outcome is unsupported | Define governing authority in L2-F13 |
| No definition-version approval/activation/supersession events and publishers are defined | Cross-domain traceability of the frozen outcome cannot be stated | Define canonical events/publishers if required |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
