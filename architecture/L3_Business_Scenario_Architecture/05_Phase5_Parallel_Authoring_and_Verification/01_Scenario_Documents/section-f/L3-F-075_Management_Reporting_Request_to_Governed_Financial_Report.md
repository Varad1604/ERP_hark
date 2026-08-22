# Management Reporting Request to Governed Financial Report

# 1. Scenario Metadata

**Scenario ID:** L3-F-075  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics; L2-H07 — Enterprise Dashboard; L2-A04 — Authorization & Policy; L2-A07 — Audit Infrastructure  
**Referenced By:** Authorized institutional leadership and governed dashboard consumers

---

# 2. Scenario Purpose

Produce an authorized, reproducible management financial report from canonical Finance facts for leadership consumption without allowing reporting or dashboards to become a transactional source of truth.

---

# 3. Business Outcome

A reproducible governed financial report is produced from authoritative Finance data

Source assessment: `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Authorized leadership requires budget, receivable, payable, cash, aid, asset, or performance information.

## 4.2 End Boundary

A validated and, where policy requires, Published management report is available to the authorized requester with definition, period, parameter, input and version traceability.

## 4.3 In Scope

- Request authorization and scope.
- Report definition, parameters and period resolution.
- Read-only canonical Finance input collection.
- Generation, validation and authorized release.
- Reproducibility and audit evidence.

## 4.4 Out of Scope

- Financial transaction creation or correction.
- Dashboard composition, widgets, layout or presentation.
- Statutory submission.
- BI tool, query engine, API or storage design.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Related | L3-F-077 — KPI and Dashboard Projection | Publishes reusable metrics/projections rather than one governed report request. |
| Related | H07 Enterprise Dashboard | Composes authorized presentation; it does not own financial report meaning. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Financial Reporting & Analytics / F | Owns report definition, generation, validation, publication and reproducibility. |
| Operational Finance domains / F | Own canonical budget, AR, AP, treasury, aid, asset and Ledger facts. |
| Enterprise Dashboard / H | Presents authorized information without owning financial meaning. |
| Authorization & Policy / A | Supplies access decisions. |
| Audit Infrastructure / A | Preserves immutable evidence. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|
| Authorized Leadership Requester | Specifies required information and scope. | Cannot bypass authorization or alter source data. |
| Finance Reviewer | Validates report correctness where required. | Reviews report only. |
| Reporting Service | Generates and versions the report. | Does not own financial transactions. |

---

# 7. Preconditions and Entry Criteria

- Tenant, institution, requester and authorization scope are resolved.
- The requested information maps to an active Report Definition.
- Reporting period, parameters and applicable configuration are known.
- Canonical Finance read models are available.
- Sensitive financial distribution rules can be enforced.

---

# 8. Trigger

Authorized leadership requires budget, receivable, payable, cash, aid, asset, or performance information

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Receive the management-report request and intended use. | Authorized Requester; F13 | Governed request begins. |
| 2 | Validate requester authorization, Tenant, institution and financial scope. | A04; F13 | Least-privilege boundary is established. |
| 3 | Resolve the Report Definition, template, period, parameters and configuration versions. | F13 | Reproducible report context is fixed. |
| 4 | Schedule or accept the on-demand execution. | Financial Report Aggregate | `ReportScheduled` is published where applicable. |
| 5 | Read authoritative Finance facts without changing them. | Operational Finance → F13 | Canonical input set is assembled. |
| 6 | Calculate and generate the management report. | F13 | `FinancialReportGenerated` is published. |
| 7 | Validate completeness, calculations, authorization scope and traceability. | Finance Reviewer; F13 | `FinancialReportValidated` is published on success. |
| 8 | Publish or deliver the validated report according to distribution policy. | F13 | `FinancialReportPublished` is published when official release applies. |
| 9 | Preserve definition, input, parameter, output, validation and access evidence. | F13; A07 | The report is reproducible and auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|
| On-demand internal report | Policy does not require official publication. | Generate, validate and return through authorized query/distribution. | Governed report is available without changing facts. |
| Scheduled report | Recurring schedule applies. | Use Reporting Schedule lifecycle. | Same definition and policy produce periodic versions. |
| Narrower authorization | Requester lacks part of the requested scope. | Deny or constrain the request according to policy. | Unauthorized financial information is excluded. |
| Validation failure | Calculations or inputs are incomplete. | Stop release and correct through owning domains/definition. | No authoritative report is delivered. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Authorization fails. | Requester cannot receive report. | Deny and audit. | A04; F13 |
| Definition or parameters are invalid. | Reproducibility is not possible. | Return request for correction. | F13 |
| Source data is unavailable. | Report is incomplete. | Stop generation; do not publish. | Source Finance domain; F13 |
| Generation fails. | No report outcome exists. | Preserve failure evidence and retry through governed execution. | F13 |

---

# 12. Business Rules and Decision Constraints

| Rule | Effect | Source |
|---|---|---|
| Reports consume canonical Finance facts read-only. | Management reporting cannot alter financial truth. | L2-F13 FRP-01/02 |
| Report definition and period are mandatory. | Output is reproducible. | L2-F13 FRI-02/03 |
| Validation precedes publication. | Unvalidated reports are not official. | L2-F13 FRI-05 |
| Financial values remain traceable. | Audit can reconstruct the report. | L2-F13 FRP-05 |
| H07 owns dashboard presentation only. | Financial semantics remain with F13. | L2-F13 explicit non-ownership |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Financial Report | Defined | `Scheduled → Generating → Generated → Validated`; optionally `Published`. | Governed report version. |
| Operational financial facts | Existing | Read only. | Unchanged. |
| Dashboard | Existing or absent | May later consume projection/report reference. | No composition change in this scenario. |
| Authorization Decision | Request received | Evaluate requested scope. | Allow, constrained outcome or Deny. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

`ReportScheduled`, `FinancialReportGenerated`, `FinancialReportValidated`, and—where official release occurs—`FinancialReportPublished` are published by the Financial Report Aggregate only after successful workflow completion.

## 14.2 Contracts and Hand-offs

| Contract | Provider | Consumer | Outcome |
|---|---|---|---|
| Authorization decision | A04 | F13 | Permitted requester and scope. |
| Canonical financial reads | Finance domains | F13 | Authoritative report inputs. |
| Governed report | F13 | Leadership / H07 | Reproducible authorized information product. |
| Audit input | F13 | A07 | Request-to-output evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Every request and distribution is evaluated for Tenant, institution, role, data classification and financial scope.

## 15.2 Approval and Separation of Duty

Validation/approval follows configurable Approval Policy. No named approval actor or additional SoD rule is introduced.

## 15.3 Audit Obligations

Retain requester, scope, definition/template versions, parameters, period, input references, calculations, validation, delivery, actor, timestamp and correlation.

---

# 16. Completion Criteria and Postconditions

- Requester and scope were authorized.
- Definition, period, parameters and configuration were fixed.
- Only canonical Finance facts were consumed.
- Report was generated, validated and delivered/published according to policy.
- Complete traceability makes the output reproducible.
- Operational financial facts and H07 ownership remain unchanged.

---

# 17. Open Issues, Assumptions and Deferred Questions

No open issue prevents this scenario from proceeding to Architecture Approval. Actor titles remain logical business responsibilities because L2-F13 does not define a canonical actor catalogue.
