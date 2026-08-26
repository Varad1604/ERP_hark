# Approved Accreditation Evidence to Self-Study Report Approval and Handoff

## 1. Scenario Metadata

**Scenario ID:** L3-H-071  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md; L2-H08_NEP_NAAC_AISHE_Reporting_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Govern Self-Study Report preparation, collaboration, internal review, institutional approval and immutable handoff from H09 to H08 without transferring H09 content-approval ownership or H08 submission ownership.

## 3. Business Outcome

H09 assembles, reviews and approves the Self-Study Report package, then hands the immutable approved version to H08 for external submission and acknowledgement

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Approved evidence, narrative inputs, and readiness decisions are complete for an accreditation cycle.

### 4.2 End Boundary

H09 assembles, reviews and approves the Self-Study Report package, then hands the immutable approved version to H08 for external submission and acknowledgement.

### 4.3 In Scope

- H09-owned governance behaviour materially required to reach the frozen business outcome.
- Applicable source-domain references, evidence and governed hand-offs without ownership transfer.
- Only upstream-defined rules, lifecycle positions, events, approvals and evidence obligations relevant to this scenario.

### 4.4 Out of Scope

- Operational business-state ownership belonging to participating source domains.
- Adjacent downstream institutional outcomes after this scenario's End Boundary.
- Later architecture-level implementation concerns.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance (H) | Owns the H09 governance outcome and applicable H09 lifecycles. |

Participation and evidence contribution do not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Owns day-to-day accreditation governance, evidence sufficiency and readiness review. | Reviews |
| Accreditation Cell | Coordinates accreditation preparation and SSR content. | Performs |
| Department Coordinators | Provide departmental evidence/narrative inputs. | Participates |
| Institutional Leadership | Provides institutional governance approval where required by the H09 approval chain. | Approves |

## 7. Preconditions and Entry Criteria

- Approved accreditation evidence, narrative inputs and readiness decisions are complete for the Accreditation Cycle.

## 8. Trigger

Approved evidence, narrative inputs, and readiness decisions are complete for an accreditation cycle

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create the SSR Package for the applicable institution, framework and cycle. | Accreditation & Compliance | The SSR Package is Draft. |
| 2 | Coordinate collaboration across departments and governance offices, mapping approved evidence to the SSR content. | Accreditation & Compliance | The package is Under Collaboration. |
| 3 | Review completeness, evidence mapping and readiness alignment. | IQAC / Accreditation & Compliance | The package is Under Review and suitable for institutional decision. |
| 4 | Approve the SSR Package under H09 governance. | Accreditation & Compliance | The package is Approved. |
| 5 | Prepare the approved immutable, versioned package for handoff. | Accreditation & Compliance | The package is Handoff Ready. |
| 6 | Hand the immutable approved SSR version to H08. | Accreditation & Compliance | `SSR Package Handed Off` establishes the H09→H08 handoff; H08 owns external submission and acknowledgement. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Approved institutional content requires correction after handoff | A content correction is required before/after H08 submission activity | Return the required institutional-content correction to H09 and create a governed successor SSR version before any new handoff. | H08 never alters the approved H09 package in place. | Accreditation & Compliance |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| SSR package is incomplete or evidence mapping is insufficient | Institutional approval/handoff cannot proceed. | Return the package to governed H09 collaboration/review until the content is complete and approved. | Accreditation & Compliance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-H09-038 — H09 owns SSR preparation, collaboration, review and approval before handoff to H08. | Accreditation & Compliance | Preserves H09 content-governance ownership. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-039 — H08 owns SSR submission, external reporting and government exchange. | NEP, NAAC & AISHE Reporting | Preserves H08 submission ownership. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| SSR Package | Draft | Collaboration, review, approval and handoff preparation. | Handoff Ready | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A12 / §7 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| SSR Package Created | SSR Package aggregate | H08 Submission | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| SSR Package Collaborated On | SSR Package aggregate | H08 Submission | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| SSR Package Reviewed | SSR Package aggregate | H08 Submission | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| SSR Package Approved | SSR Package aggregate | H08 Submission | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| SSR Package Handed Off | SSR Package aggregate | H08 Submission | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Accreditation & Compliance | NEP, NAAC & AISHE Reporting (H08) | SSR Handoff Contract | Transfer the immutable, versioned, institutionally approved SSR package for external submission and acknowledgement. | L2-H09_Accreditation_and_Compliance_Workflows.md §23.4; L2-H08_NEP_NAAC_AISHE_Reporting_Workflows.md §2.2 Submission Management |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| SSR internal review | IQAC | Reviews completeness, evidence mapping and readiness alignment. | Accreditation & Compliance |
| SSR institutional approval | Accreditation & Compliance governance / Institutional Leadership | H09 owns institutional approval before handoff. | Accreditation & Compliance |

### 15.2 Separation of Duties, Overrides and Exceptions

H09 approves the institutional SSR content; H08 owns external submission and acknowledgement and may not alter the approved H09 package.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| SSR approval | Approved SSR version, approval evidence and evidence-reference manifest | Accreditation & Compliance |
| SSR handoff | Immutable versioned package and handoff correlation | Accreditation & Compliance |
| External submission and acknowledgement | Submission/acknowledgement history after handoff | NEP, NAAC & AISHE Reporting |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The SSR Package is institutionally approved by H09.
- The approved package is immutable and versioned for handoff.
- The approved package is handed to H08 under the SSR Handoff Contract.

### 16.2 Postconditions

- H08 owns external submission, acknowledgement and submission history after handoff.
- Any correction requiring new institutional content returns to H09 for a governed successor version.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
