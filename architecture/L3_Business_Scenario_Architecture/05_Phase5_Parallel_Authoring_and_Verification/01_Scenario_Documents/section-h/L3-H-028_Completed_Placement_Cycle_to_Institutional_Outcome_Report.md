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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; applicable locked Student Lifecycle and People & Workforce architecture  
**Referenced By:** Institutional placement reporting, placement governance, authorized placement analytics, and downstream dashboard composition

# 2. Scenario Purpose

Provide an institutionally governed placement outcome report after a placement cycle or reporting period closes, using verified Placement-owned outcomes while preserving Student and workforce source ownership.

# 3. Business Outcome

Verified placement, offer, joining, diversity, and employer outcomes are published for authorized institutional use

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a placement cycle or reporting period closes.

## 4.2 End Boundary

The frozen end boundary requires verified placement, offer, joining, diversity, and employer outcomes to be published for authorized institutional use. Locked H04 supports placement statistics and institutional analytical read models but does not fully define this publication outcome; the material gaps are recorded in Section 17.

## 4.3 In Scope

- Using completed Placement-owned drive, offer, joining, internship, and employer outcome evidence.
- Generating Placement statistics from completed placement lifecycle events.
- Preserving Student and workforce ownership while referencing permitted source information.
- Providing governed institutional placement analytical information to authorized consumers where H04 defines that availability.
- Preserving traceability from reported placement outcomes back to Placement-owned lifecycle evidence.

## 4.4 Out of Scope

- Reopening or changing completed placement lifecycles.
- Recomputing Student or workforce canonical facts.
- Defining new diversity dimensions, metrics, classifications, or source contracts.
- Defining a new report approval/publication lifecycle or event not present in H04.
- Dashboard composition owned by H07.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Retains canonical Student information referenced for authorized institutional placement reporting. |
| People & Workforce / Section E | Retains canonical workforce and placement-staff information referenced by Placement. |
| Placement / Section H | Owns placement-specific outcome facts, Placement Statistics, Institutional Placement Statistics, placement analytical projections, and the placement reporting context. |

Participation does not transfer Student or workforce ownership to Placement.

# 6. Business Actors, Roles and Responsibilities

H04's canonical crosswalk records its Actors section as absent. Individual placement workflows identify roles such as Placement Officer and Placement Coordinator, but H04 does not define a canonical business actor or approval authority for institutional report publication. No reporting approver is invented here.

# 7. Preconditions and Entry Criteria

- The applicable placement cycle or reporting period has closed.
- Placement-owned drive, candidate, offer, internship, and joining outcomes needed for the report are available in their governed states.
- Historical placement outcomes remain immutable and traceable.
- Any Student or workforce facts used remain owned by their canonical source domains.
- Authorized institutional use is required for the resulting information.

# 8. Trigger

A placement cycle or reporting period closes

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the closed placement cycle or reporting scope and the Placement-owned outcomes within that scope. | Placement | The governed reporting population is established. |
| 2 | Assemble completed drive, offer, joining, internship, and employer outcome evidence from Placement-owned records and projections. | Placement | Placement outcomes are traceable to completed lifecycles. |
| 3 | Generate the Placement statistics and institutional analytical information supported by H04. | Placement | Current placement reporting information is available for governed institutional use. |
| 4 | Reference permitted Student and workforce information without transferring ownership or altering source facts. | Placement / Sections C and E | Cross-domain context remains source-owned. |
| 5 | Validate that only supported, authorized reporting information is made available. | Placement | Unsupported or unauthorized information is excluded. |
| 6 | Stop before claiming the frozen final publication outcome where diversity semantics and a governed report-publication lifecycle remain undefined upstream. | Placement | No unsupported publication, metric, or authority is invented. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Placement outcomes remain incomplete | Required offer, joining, internship, or drive outcomes are not final. | Exclude unresolved outcomes or defer the affected reporting scope until governed lifecycle evidence is complete. | Reported information remains based on completed placement evidence. | Placement |
| Source-owned context is unavailable | Required Student or workforce information cannot be obtained from its owner. | Preserve the Placement-owned outcomes and omit unsupported source-domain detail. | Placement does not create substitute source facts. | Placement / Source Domain |
| Reporting request exceeds defined H04 content | Requested metric or classification is not defined upstream. | Record the gap and do not manufacture the metric or classification. | Unsupported reporting semantics remain unresolved. | Placement |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Placement statistics cannot be reconciled to completed lifecycle evidence | Institutional reporting cannot be treated as verified. | Reconcile the reporting population against Placement-owned completion and closure evidence before further use. | Placement |
| Historical placement evidence is inconsistent | The report cannot be trusted. | Preserve the inconsistency for governed placement review; do not overwrite completed history. | Placement |
| Required reporting semantics are undefined | Frozen final outcome cannot be fully expressed from locked H04. | Escalate for upstream architecture resolution and keep the scenario Draft/BLOCKED. | Placement / Architecture Governance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Closed Placement Drives become immutable and historical records cannot be deleted. | Placement | Reporting must use preserved historical outcomes rather than rewriting them. | L2-H04 §19.16 |
| Placement statistics are generated as part of drive closure. | Placement | Completed placement cycles provide the Placement-owned statistical basis. | L2-H04 §§19.16, 20.2 |
| `PlacementStatisticsGenerated`, `PlacementDriveClosed`, `PlacementCompleted`, and `InternshipCompleted` support Placement statistics projections. | Placement | Analytical outcomes remain traceable to canonical Placement events. | L2-H04 §§21, 22.1 |
| Student and workforce information remain owned by their source domains. | Sections C and E | Placement reporting cannot become a competing source of Student or workforce truth. | L2-H04 §§4, 13, 23.1; `DEPENDENCY_MAP.md` |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| PlacementDrive | `Closed` | Historical drive evidence contributes to institutional reporting. | Remains `Closed` and immutable. | Placement | L2-H04 §§17.2, 19.16 |
| JoiningConfirmation | `Completed` | Verified placement completion contributes to Placement Statistics. | Remains completed and immutable. | Placement | L2-H04 §§17.8, 19.15 |
| Internship | `Completed` / governed terminal position | Completed internship outcomes contribute to Placement analytics. | Unchanged by reporting. | Placement | L2-H04 §§17.7, 19.17 |
| Placement Statistics projection | Current governed reporting position | Aggregates Placement-owned completion and closure outcomes. | Available for authorized institutional analytical use. | Placement | L2-H04 §22.1 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H04 defines the Placement-owned events `PlacementStatisticsGenerated`, `PlacementDriveClosed`, `PlacementCompleted`, and `InternshipCompleted` as inputs to placement reporting and statistics. H04 does not define a separate canonical `InstitutionalPlacementReportPublished` event or equivalent final report-publication event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Placement may reference governed Student and workforce information while retaining only Placement-owned outcomes and reporting projections. H04 defines institutional placement statistics and analytical read models, but it does not define a canonical diversity-reporting contract or formal report-publication hand-off.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The frozen outcome requires authorized institutional use. Section A retains canonical authorization ownership, while H04 does not define a named business authority for approving or publishing this institutional outcome report.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or reporting override rule is defined in H04. Completed placement evidence must not be altered merely to satisfy reporting expectations.

## 15.3 Business Audit and Evidence Obligations

Evidence must preserve the reporting scope, source Placement events, relevant Placement lifecycle identities, statistical generation outcome, source-domain references used, and any exclusions caused by unavailable or undefined reporting semantics.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario cannot claim full completion while the material Section 17 gaps remain unresolved. The unambiguous partial criteria are:

- the placement cycle/reporting scope is closed;
- Placement-owned outcomes are traceable to completed lifecycle evidence;
- H04-supported Placement statistics are available;
- source-domain ownership remains unchanged;
- unsupported reporting dimensions are not invented.

## 16.2 Postconditions

- Placement-owned historical outcomes remain immutable.
- H04-supported statistics and analytics are available for governed institutional consumption.
- The frozen full report-publication outcome remains pending upstream architecture resolution.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome requires verified **diversity** outcomes, but H04 defines no canonical diversity dimensions, metric definitions, source attributes, verification rule, or cross-domain contract for diversity reporting. | L3 cannot state what constitutes a verified diversity outcome without inventing business semantics or source ownership. | Define the canonical diversity reporting semantics, source owner(s), and permitted attributes upstream. |
| The frozen outcome requires outcomes to be **published** for authorized institutional use, but H04 defines analytical/read-model availability and `PlacementStatisticsGenerated` without a report approval/publication lifecycle, publication event, or named publication authority. | L3 cannot complete the frozen end boundary without inventing a publication process or authority. | Define the governed institutional placement report publication/approval contract, lifecycle, event, and authority upstream. |
| The frozen trigger permits a generic “reporting period” closure, while H04 defines Placement Drive closure but no separate reporting-period aggregate/lifecycle. | The reporting boundary is incomplete for periods not identical to a Placement Drive lifecycle. | Define the canonical reporting-period boundary or explicitly bind this scenario to existing institutional reporting governance. |

**Architecture assessment:** `BLOCKED`

The Placement-owned statistics and historical reporting basis are supported, but the frozen diversity and formal publication outcomes cannot be fully authored from locked H04 without inventing material reporting semantics and authority.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions beyond the open issues above.
