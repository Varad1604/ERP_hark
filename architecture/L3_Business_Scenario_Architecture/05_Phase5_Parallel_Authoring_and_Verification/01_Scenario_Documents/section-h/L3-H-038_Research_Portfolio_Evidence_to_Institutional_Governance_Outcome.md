# Research Portfolio Evidence to Institutional Governance Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-038  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; L2-F00 — Finance Domain Model & Foundation  
**Referenced By:** Institutional research governance, reporting, ranking, audit, and compliance review

# 2. Scenario Purpose

Provide institutional governance with research evidence derived from authoritative Research sources while preserving People & Workforce and Finance ownership boundaries and without changing source records.

# 3. Business Outcome

Verified project, funding, ethics, output, IP, and collaboration evidence supports the institutional review without changing source records

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A reporting, ranking, audit, or compliance cycle requires authoritative research evidence.

## 4.2 End Boundary

The frozen outcome is reached only when all required evidence categories in Section 3 have an authoritative Research portfolio path. L2-H05 currently supports only a subset; the material omissions are recorded in Section 17.

## 4.3 In Scope

- Research portfolio evidence derived from source-owned Research facts defined by L2-H05.
- Research KPI and report generation from the Research portfolio where supported upstream.
- Preservation of People & Workforce and Finance ownership while their authoritative facts participate by reference or governed hand-off.

## 4.4 Out of Scope

- Creation of missing ethics, collaboration, or Finance evidence contracts.
- Modification of Faculty Workforce, Finance, or Research transactional source records for reporting purposes.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| People & Workforce / Section E | Retains authoritative faculty capability, participation, and workforce facts referenced by Research. |
| Finance / Section F | Retains authoritative monetary, accounting, and financial consequences associated with research funding. |
| Research / Section H | Owns Research portfolio, KPI, and report outcomes defined by L2-H05 and the institutional research evidence assembly. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Research Office | Requests or produces supported Research reports from governed Research evidence. | Performs / receives outcome |
| Management | Uses supported Research reports for institutional governance and decision making. | Receives outcome |

# 7. Preconditions and Entry Criteria

- Applicable source Research facts have reached the authoritative lifecycle positions required by L2-H05.
- Research portfolio information required for the supported evidence categories is available.
- Source-domain ownership remains unchanged.

# 8. Trigger

A reporting, ranking, audit, or compliance cycle requires authoritative research evidence

# 9. Main Business Flow

1. Identify the institutional research evidence requirement.
2. Use the L2-H05 Research Portfolio derived from supported Research source events.
3. Include supported project, grant/funding metadata, publication/output, IP, and consultancy evidence without modifying their source records.
4. Generate Research KPIs where required from the portfolio.
5. Generate the governed Research report for institutional review.
6. Stop before claiming the frozen outcome because ethics, collaboration, and the required Finance-owned funding evidence hand-off are not completely defined for the portfolio path.

# 10. Alternate Business Flows

Department-level KPI calculation and institution-configured Research report variants may be used where defined by L2-H05. They do not resolve the missing evidence categories in Section 17.

# 11. Failure, Interruption and Business Recovery

If required authoritative evidence is unavailable or is not represented by an approved portfolio source path, the institutional review remains incomplete for that evidence category and requires upstream architecture resolution rather than local substitution.

# 12. Business Rules and Decision Constraints

- The Research Portfolio is derived and shall never become the source of record.
- KPI calculation operates on Research portfolio information rather than changing source aggregates.
- Research reporting is read-only with respect to transactional Research sources.
- Finance remains the authoritative owner of accounting and monetary consequences.
- Faculty Workforce remains authoritative for faculty capability, capacity, availability, and participation facts.

# 13. State and Lifecycle Impact

This scenario does not change source lifecycle states. L2-H05 may update the derived Research Portfolio and create immutable KPI snapshots and Research report outcomes from supported source facts.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

For the portfolio path, L2-H05 defines `ResearchPortfolioUpdated`, `ResearchKPIsGenerated`, and `ResearchReportGenerated`. The portfolio consumes supported Research events including `ResearchOutputPublished`, `PatentFiled`, `PatentStatusUpdated`, `GrantAwarded`, `FundingUtilizationRecorded`, `ConsultancyProjectCompleted`, `ResearchProjectCompleted`, and `ResearchProjectClosed`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

People & Workforce and Finance retain ownership of their source facts. L2-H05 does not define the complete ethics, collaboration, and Finance-owned funding-evidence hand-offs required by the frozen outcome.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Research Office and Management participation is limited to the report-generation and governance-use authority defined by L2-H05.

## 15.2 Separation of Duties, Overrides and Exceptions

No Research report or portfolio projection may replace a missing source-domain decision or redefine source ownership.

## 15.3 Business Audit and Evidence Obligations

Preserve the report request, applicable parameters, source references, portfolio provenance, KPI history where used, generated Research report outcome, and source-domain ownership boundaries.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported L2-H05 report path may complete for evidence categories represented in the Research Portfolio. The full frozen outcome cannot be claimed while any material Section 17 gap remains unresolved.

## 16.2 Postconditions

Supported Research evidence is available for institutional review without changing source records; People & Workforce and Finance ownership remains unchanged; unsupported evidence categories are not approximated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- L2-H05 Repository & Portfolio Management does not consume Ethics Aggregate events, so the frozen **ethics evidence** component has no defined portfolio evidence path.
- L2-H05 Repository & Portfolio Management does not consume Collaboration Aggregate events, so the frozen **collaboration evidence** component has no defined portfolio evidence path.
- Although Finance is a frozen participating domain and owns monetary/accounting facts, L2-H05 does not define a Finance-owned financial-evidence hand-off into the Research Portfolio sufficient to establish the frozen **funding evidence** component as Finance-authoritative evidence.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions beyond the open issues above.
