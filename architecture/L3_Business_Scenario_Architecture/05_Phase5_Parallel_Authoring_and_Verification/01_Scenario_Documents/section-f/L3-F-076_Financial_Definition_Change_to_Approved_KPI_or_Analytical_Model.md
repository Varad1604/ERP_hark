# Financial Definition Change to Approved KPI or Analytical Model

# 1. Scenario Metadata

**Scenario ID:** L3-F-076  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics Workflows; `L2-F13_KPI_and_Analytical_Model_Governance_Amendment_2026-09-20.md`; ADR-FIN-ANALYTICS-001  
**Referenced By:** KPI calculation, financial analytics and dashboard-projection scenarios

# 2. Scenario Purpose

Govern a change to a financial KPI, metric, forecast or Analytical Model definition so a versioned successor can become effective for future calculation without changing historical results or canonical financial facts.

# 3. Business Outcome

A versioned definition becomes effective for future calculation while historical results remain reproducible

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized governance decision changes a financial KPI, metric, forecast, or analytical definition.

## 4.2 End Boundary

The scenario ends when a distinct successor KPI Definition or Analytical Model version has completed the canonical Finance configuration governance lifecycle through approval, publication and activation, becomes the Active version for new eligible calculations/projections, any replaced predecessor is preserved as Superseded, and historical results retain their original governing version.

## 4.3 In Scope

- Preservation of the existing KPI Definition or Analytical Model version.
- Composition and validation of a successor definition.
- Version-controlled configuration and historical reproducibility.
- Governed approval, publication, activation and supersession of the successor definition/model.

## 4.4 Out of Scope

- Rewriting historical KPI or analytical results.
- Modifying operational Finance facts.
- Inventing any lifecycle state, approval role or event outside the locked F13 amendment.
- Dashboard composition or presentation.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Reporting & Analytics / F | Owns KPI Definitions, Analytical Models, reporting configuration and their business meaning. |
| Applicable Finance capabilities / F | Retain ownership of the canonical financial facts referenced by definitions. |

# 6. Business Actors, Roles and Responsibilities

- Financial Reporting & Analytics owns KPI Definition and Analytical Model governance.
- **Financial Analytics Definition Approval Authority** provides the canonical business approval for new or revised KPI Definition / Analytical Model versions.
- F13-authorized business participants may prepare and validate candidate definitions/models under institutional policy.
- Institutional policy maps the approval authority to the applicable level of the existing Finance approval hierarchy; no universal local job title is invented.

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
| 5 | Progress the validated candidate to `Approval` and obtain the Financial Analytics Definition Approval Authority decision. | Financial Reporting & Analytics / Approval Authority | The candidate is approved or remains outside effective use. |
| 6 | Publish the approved candidate and activate it at its governed effective boundary. | Financial Reporting & Analytics / F | The successor becomes the Active version for new eligible calculations/projections. |
| 7 | Where an Active predecessor exists for the same governed scope, transition the predecessor to `Superseded` without changing historical results. | Financial Reporting & Analytics / F | Current and historical versions remain unambiguous and reconstructable. |
| 8 | Publish the aggregate-specific approval, activation and supersession business facts. | KPI Definition or Analytical Model / F13 | Downstream consumers can trace the governed definition/model version. |

# 10. Alternate Business Flows

| Condition | Alternate Path | Result |
|---|---|---|
| Validation fails | Correct the candidate and repeat validation; do not progress to approval | Existing Active version remains authoritative |
| Approval is rejected or absent | Keep the candidate outside Published/Active | Existing Active version remains authoritative |
| No predecessor exists | Activate the approved first version | First Active definition/model becomes effective |
| Superseded version reaches retention eligibility | Archive it under governed retention | Historical version remains reconstructable |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Proposed definition would change or re-own canonical financial facts | The analytical definition would violate ownership | Reject the proposed definition change | F13 / applicable Finance owner |
| Historical configuration context would be lost | Prior results would cease to be reproducible | Do not replace the predecessor version | F13 |
| Approval is rejected or unavailable | Candidate cannot become effective | Retain candidate outside Active; current Active version remains authoritative | F13 |
| Activation would lose historical reproducibility | Historical integrity would be violated | Reject activation until lineage/version references are preserved | F13 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| KPI Definitions and Financial Dashboard Information Projection configuration are owned by Financial Reporting | F13 | Definition meaning cannot be transferred to another domain | L2-F13 §44 |
| Configuration changes shall be version controlled | F13 | In-place replacement is not sufficient | L2-F13 CP-04 |
| Historical reports preserve the configuration context under which they were generated | F13 | Prior outcomes remain reproducible | L2-F13 CP-04 |
| Configuration changes shall generate immutable audit evidence | F13 | Change history must remain traceable | L2-F13 CP-05 |
| Configuration shall not modify financial facts or bypass report validation | F13 | Analytical change cannot alter operational truth | L2-F13 §45 |

# 13. State and Lifecycle Impact

KPI Definition and Analytical Model consume the locked F13/F00 Finance configuration lifecycle:

`Draft → Validation → Approval → Published → Active → Superseded → Archived`

Only `Active` versions govern new eligible calculations/projections. A successor version does not rewrite the predecessor or historical results.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

The locked F13 amendment establishes:

- KPI Definition Aggregate / F13 publishes `KPIDefinitionApproved`, `KPIDefinitionActivated`, `KPIDefinitionSuperseded`, and `KPIDefinitionArchived`.
- Analytical Model Aggregate / F13 publishes `AnalyticalModelApproved`, `AnalyticalModelActivated`, `AnalyticalModelSuperseded`, and `AnalyticalModelArchived`.
- `FinancialKPICalculated` remains the KPI calculation-completion event.
- `FinancialDashboardProjectionUpdated` remains the analytical projection-completion event.

Governance events and calculation/projection completion events are not interchangeable.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Applicable Finance domains provide canonical financial information for later calculations. The definition/model remains owned by F13 and source facts remain owned by their Finance capabilities.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Financial Analytics Definition Approval Authority is the canonical business authority for approving a new or revised KPI Definition or Analytical Model version. Institutional policy maps this authority to the applicable Finance approval level. Activation follows the approved F13 configuration lifecycle.

## 15.2 Separation of Duties, Overrides and Exceptions

Existing L2-F00 Finance segregation-of-duties controls apply where configured. Preparation/validation does not itself imply approval authority.

## 15.3 Business Audit and Evidence Obligations

F13 requires immutable audit evidence for configuration changes and preservation of historical configuration context.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario completes when the successor version has been validated, approved, published and activated; any replaced Active predecessor is Superseded; applicable governance events are published; and historical results preserve their original governing definition/model version.

## 16.2 Postconditions

The predecessor definition/model and historical result context remain preserved. The approved successor is Active for new eligible calculations/projections. Any predecessor is Superseded and remains historically reconstructable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No material upstream architecture issue remains for this scenario after `L2-F13_KPI_and_Analytical_Model_Governance_Amendment_2026-09-20.md` and ADR-FIN-ANALYTICS-001.

**Architecture assessment:** `READY`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
