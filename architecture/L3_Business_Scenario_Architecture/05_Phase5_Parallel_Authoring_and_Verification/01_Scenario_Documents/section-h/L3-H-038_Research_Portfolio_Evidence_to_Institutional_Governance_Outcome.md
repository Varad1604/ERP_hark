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
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; locked Finance architecture

# 2. Scenario Purpose

Provide institutional governance with a Research-owned portfolio/reporting outcome derived from authoritative source evidence without turning the Research Portfolio into a source of record or changing E/F/H transactional facts.

# 3. Business Outcome

Verified project, funding, ethics, output, IP, and collaboration evidence supports the institutional review without changing source records

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A reporting, ranking, audit, or compliance cycle requires authoritative research evidence

## 4.2 End Boundary

H05 supports portfolio updates, KPI snapshots, and Research reports from a defined set of Research events. The frozen outcome requires project, funding, ethics, output, IP, and collaboration evidence; H05 lacks complete portfolio paths for all those categories.

## 4.3 In Scope

Derived Research Portfolio evidence; supported Project, Grant/funding metadata, Publication/output, Patent/IP, and Consultancy facts; KPI generation; Research report generation; preservation of source ownership.

## 4.4 Out of Scope

Inventing Ethics or Collaboration portfolio ingestion; inventing Finance-owned evidence contracts; changing source aggregates to satisfy reporting.

# 5. Ownership and Participating Domains

E03 retains authoritative workforce facts. Finance retains authoritative monetary/accounting facts. Research owns the derived Research Portfolio, Research KPI snapshots, and Research report outcomes defined by H05.

# 6. Business Actors, Roles and Responsibilities

Research Office generates/requests supported Research reports. Management receives supported reports for institutional governance. No additional evidence approver is invented.

# 7. Preconditions and Entry Criteria

Applicable source facts exist in governed lifecycle positions; required portfolio/read-model information is available; report/KPI definitions are configured; source ownership is unchanged.

# 8. Trigger

A reporting, ranking, audit, or compliance cycle requires authoritative research evidence

# 9. Main Business Flow

1. Establish the reporting/ranking/audit/compliance evidence requirement.
2. Assemble the H05 Research Portfolio from its canonical supported Research event sources.
3. Include source-supported project, grant/funding metadata, output, Patent/IP, and Consultancy evidence by reference.
4. Generate Research KPIs where required.
5. Generate the authorized Research report for institutional review.
6. Preserve provenance to source Research facts and E/F ownership.
7. Stop before claiming complete frozen ethics, collaboration, and Finance-authoritative funding evidence because those portfolio hand-offs are not fully defined.

# 10. Alternate Business Flows

Department-level KPI/report variants may be generated where configured. Missing evidence categories remain omissions and are not replaced by inferred metrics.

# 11. Failure, Interruption and Business Recovery

Unavailable authoritative evidence or an undefined portfolio source path prevents the affected evidence category from being claimed as verified. Existing source records remain unchanged.

# 12. Business Rules and Decision Constraints

Research Portfolio is derived and never the system of record. KPI snapshots are immutable after generation. Reporting is read-only to transactional Research aggregates. Finance and E03 retain their source ownership.

# 13. State and Lifecycle Impact

No source aggregate lifecycle changes. H05 may update the derived Research Portfolio and create immutable KPI/report outcomes from supported source events.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H05 publishes `ResearchPortfolioUpdated`, `ResearchKPIsGenerated`, and `ResearchReportGenerated`. Portfolio inputs include supported events such as `ResearchOutputPublished`, `PatentFiled`, `PatentStatusUpdated`, `GrantAwarded`, `FundingUtilizationRecorded`, `ConsultancyProjectCompleted`, `ResearchProjectCompleted`, and `ResearchProjectClosed`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H05 does not define Ethics Aggregate events or Collaboration Aggregate events as complete portfolio inputs, and it does not define a Finance-owned financial-evidence hand-off that establishes the frozen funding evidence as Finance-authoritative within the portfolio.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Research Office and Management participation is limited to H05 report generation and governance use. No source-domain authority is transferred.

## 15.2 Separation of Duties, Overrides and Exceptions

A Research report cannot replace a missing source-domain decision or evidence contract.

## 15.3 Business Audit and Evidence Obligations

Preserve report request, scope/parameters, source references, portfolio provenance, KPI snapshots where used, generated report, and any omitted evidence category caused by an upstream gap.

# 16. Completion Criteria and Postconditions

H05-supported evidence can be reported without changing source records. The full frozen outcome cannot be claimed until all required evidence categories have canonical portfolio/source hand-offs.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H05 Portfolio Management does not consume Ethics Aggregate events, so the frozen ethics-evidence component lacks a canonical portfolio path.
- Collaboration appears in reporting/read models, but the defined portfolio event-consumption path does not consume Collaboration Aggregate events.
- Finance owns monetary/accounting facts, but H05 defines no Finance-owned financial-evidence hand-off sufficient to establish the frozen funding-evidence component as Finance-authoritative within the portfolio.

These missing evidence contracts materially prevent the frozen governance outcome from being fully established.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
