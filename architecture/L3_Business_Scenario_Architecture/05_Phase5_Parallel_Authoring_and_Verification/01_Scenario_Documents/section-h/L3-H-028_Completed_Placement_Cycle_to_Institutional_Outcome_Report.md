# Completed Placement Cycle to Institutional Outcome Report

# 1. Scenario Metadata

**Scenario ID:** L3-H-028  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-C02 — Student Lifecycle & Activation Workflows; L2-E01 — Workforce & Engagement Workflows  
**Referenced By:** Institutional placement reporting and dashboard composition

# 2. Scenario Purpose

Produce institutionally usable Placement reporting from completed Placement-owned outcomes without reopening completed lifecycles or becoming a competing source of Student/workforce facts.

# 3. Business Outcome

Verified placement, offer, joining, diversity, and employer outcomes are published for authorized institutional use

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a placement cycle or reporting period closes.

## 4.2 End Boundary

H04 defines Placement statistics, institutional reports/read models, and completion/closure events. It does not define canonical **diversity** dimensions/source attributes/verification rules, a report publication/approval lifecycle, a report-publication event/authority, or a generic reporting-period lifecycle. The frozen end boundary is therefore unresolved.

## 4.3 In Scope

Use completed Placement Drive, offer, joining, internship and employer outcomes; generate H04-defined statistics/read models; reference permitted C/E facts; preserve traceability and authorization boundaries.

## 4.4 Out of Scope

Invented diversity metrics/classifications, report approval/publication lifecycle, reporting-period aggregate, Student/workforce recomputation, and H07 dashboard composition.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / C | Owns Student facts referenced for authorized reporting. |
| People & Workforce / E | Owns workforce information referenced by Placement. |
| Placement / H | Owns Placement outcome facts, Placement statistics and H04 reporting projections. |

# 6. Business Actors, Roles and Responsibilities

H04 names Placement Officer/Coordinator in operational workflows but its formal Actors section is absent and it defines no canonical business role for approving/publishing the frozen institutional outcome report. No report approver is invented.

# 7. Preconditions and Entry Criteria

- Placement cycle/reporting scope has closed.
- Placement-owned drive/candidate/offer/internship/joining outcomes needed for reporting are in governed completed states.
- Historical Placement outcomes remain traceable.
- Source C/E facts remain source-owned.

# 8. Trigger

A placement cycle or reporting period closes

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the closed Placement scope and completed Placement outcomes. | H | Governed reporting population is established. |
| 2 | Assemble Placement-owned drive, offer, joining, internship and employer outcome evidence. | H | Outcomes remain traceable to canonical Placement lifecycles. |
| 3 | Generate the Placement statistics and institutional reporting information explicitly defined by H04. | H | Supported Placement analytics become available. |
| 4 | Reference permitted C/E information without copying source ownership. | H with C/E | Cross-domain context remains authoritative at source. |
| 5 | Apply authorization to institutional consumption. | A via governed authorization / H | Only eligible use is permitted. |
| 6 | Publish verified diversity-inclusive institutional outcome report. | H | **BLOCKED:** diversity semantics and report-publication lifecycle/authority are undefined upstream. |

# 10. Alternate Business Flows

Incomplete Placement outcomes are excluded/deferred rather than guessed. Unavailable source facts remain omitted. Unsupported metrics/classifications are not manufactured.

# 11. Failure, Interruption and Business Recovery

Statistics that cannot reconcile to completed Placement evidence are not treated as verified. Historical inconsistencies remain visible for governed review. Missing diversity/publication semantics require upstream architecture resolution.

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Closed Placement Drives are immutable and historical records cannot be deleted. | H | Reporting uses preserved outcomes. |
| Placement statistics are generated during drive closure and from Placement completion events. | H | H04 provides a governed reporting basis. |
| Student/workforce facts remain source-owned. | C / E | Reporting cannot become a competing system of record. |
| Unsupported metrics/rules cannot be invented at L3. | Upstream governance | Diversity semantics must come from an authoritative owner. |

# 13. State and Lifecycle Impact

PlacementDrive remains `Closed`; completed JoiningConfirmation and Internship records remain unchanged. H04's Placement Statistics projection becomes available for analytical use. No report-publication or reporting-period lifecycle is defined by H04.

# 14. Business Events and Cross-Domain Collaboration

H04 defines `PlacementStatisticsGenerated`, `PlacementDriveClosed`, `PlacementCompleted`, and `InternshipCompleted` as reporting/statistical source events. It defines no `InstitutionalPlacementReportPublished` event and no diversity-specific reporting event/contract.

# 15. Business Authorization, Approval and Audit Obligations

Section A retains authorization ownership. H04 does not define a report publication approver/authority. Evidence must preserve reporting scope, Placement source events/lifecycle references, statistical generation, source C/E references, authorization context, and any excluded unsupported semantics.

# 16. Completion Criteria and Postconditions

H04-supported statistics/read models can be generated for authorized institutional use. The frozen scenario cannot fully complete until diversity semantics and a governed publication lifecycle/authority are defined. Completed Placement history and C/E ownership remain unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Frozen outcome requires verified **diversity** outcomes, but H04 defines no diversity dimensions, source attributes, verification rules, or canonical contract. | Material reporting semantics/ownership are missing. | Define diversity reporting semantics and source owners upstream. |
| Frozen outcome says outcomes are **published** for authorized use, but H04 defines statistics/read-model availability without report approval/publication lifecycle, event, or authority. | Final publication outcome cannot be asserted. | Define governed report publication/approval semantics upstream. |
| Trigger allows generic reporting-period closure, but H04 defines no reporting-period aggregate/lifecycle. | Period-based boundary outside a drive is undefined. | Define or bind the reporting-period boundary upstream. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
