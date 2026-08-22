# Accreditation Framework Change to Active Governed Framework

# 1. Scenario Metadata

**Scenario ID:** L3-H-061  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** Accreditation-cycle planning, readiness, evidence, audit, assessment, compliance, and SSR scenarios

# 2. Scenario Purpose

Govern the creation or revision of an H09 Accreditation Framework so that criteria, metrics, evidence expectations, scoring methodology, applicability, and version lineage are institutionally reviewed, approved, published, and activated without altering source-domain business facts.

# 3. Business Outcome

A versioned framework is approved and activated with criteria, evidence requirements, scoring, and applicability preserved

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A regulator or institution introduces or revises an accreditation framework.

## 4.2 End Boundary

The scenario ends when the governed framework version reaches the source-defined `Active` lifecycle position after institutional review, approval, and publication, with its accreditation structure and version lineage preserved.

## 4.3 In Scope

- Accreditation Framework version creation or governed revision.
- Definition of criteria, metrics, evidence requirements, scoring methodology, review calendar, and applicability.
- IQAC and steering review under the H09 governance chain.
- Institutional framework approval, publication, and activation.
- Preservation of prior active/historical framework versions when a successor version is introduced.

## 4.4 Out of Scope

- Accreditation-cycle execution.
- Collection or approval of individual evidence items.
- External assessment execution or statutory submission.
- Modification of source-domain operational records.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance / H | Owns Accreditation Framework definition, version, criteria, metrics, evidence expectations, scoring rules, review calendar, approval/publication/activation lifecycle, and framework history. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Reviews framework alignment, evidence expectations, and institutional accreditation readiness implications. | Reviews |
| Accreditation Cell | Coordinates framework definition and accreditation structure. | Performs / coordinates |
| Compliance Administrator | Maintains governed framework configuration within H09. | Performs |
| Steering Committee | Reviews the framework as part of institutional accreditation governance. | Reviews |
| Institutional Leadership | Provides final institutional approval for strategic accreditation positions under the H09 governance structure. | Approves |

# 7. Preconditions and Entry Criteria

- The applicable Accreditation Authority is defined as an external reference.
- The institution has a governed basis to introduce or revise the accreditation model.
- For a revision of an already active framework, a new version can be established without modifying the historical active version.

# 8. Trigger

A regulator or institution introduces or revises an accreditation framework

# 9. Main Business Flow

1. H09 identifies the applicable Accreditation Authority and the new or revised accreditation requirement.
2. H09 creates a new Accreditation Framework or a successor framework version where the existing active version must remain immutable.
3. The Accreditation Cell and Compliance Administrator define or revise the criteria, metrics, evidence expectations, scoring methodology, weightages, review calendar, and applicability supported by the framework.
4. IQAC reviews the framework for accreditation alignment and evidence/readiness implications.
5. The Steering Committee reviews the framework within the H09 governance chain.
6. Institutional Leadership provides the source-supported institutional approval for the framework position.
7. H09 publishes the approved framework version.
8. H09 activates the published framework version and preserves its effective version lineage.
9. The scenario ends when the framework is the governed active version for its applicable accreditation context.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Active framework requires change | A currently active framework must be revised | Create a new version; do not modify the activated historical version | Successor version follows review and approval | Accreditation & Compliance |
| Framework no longer applies | H09 governance determines that the framework should leave active use | Preserve history and move the framework through the source-defined archival path | Historical framework retained | Accreditation & Compliance |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Framework definition is incomplete | Approval/activation cannot proceed | Complete the missing criteria, metric, evidence, scoring, or applicability definition and repeat governance review | Accreditation & Compliance |
| Governance review does not support approval | Framework cannot become active | Retain the version outside the active position until revised and approved | Accreditation & Compliance |
| Proposed change would rewrite an active historical version | H09 immutability rule would be violated | Create a successor version instead | Accreditation & Compliance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only one active framework version may exist for a given accreditation authority and assessment cycle | H09 | Activation must not create competing active versions | L2-H09 BR-H09-001 |
| Every Accreditation Cycle uses exactly one approved framework | H09 | Framework approval establishes the governed basis for later cycles | L2-H09 BR-H09-002 |
| Activated framework modifications require a new version | H09 | Historical active versions are not edited | L2-H09 BR-H09-003 |
| Historical frameworks remain immutable | H09 | Prior versions are preserved | L2-H09 BR-H09-004 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Accreditation Framework | Draft | Govern definition and institutional review | Review position → Approved | H09 | L2-H09 H09-A01; §7 |
| Accreditation Framework | Approved | Publish the approved framework | Published | H09 | L2-H09 H09-A01; H09-01 |
| Accreditation Framework | Published | Activate the governed version | Active | H09 | L2-H09 H09-A01; H09-01 |

H09 names the intermediate review state `Under Review` in H09-A01 and `Review` in §7. This Draft does not normalize that naming difference.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Accreditation Framework Created | Accreditation Framework aggregate / H09 | Accreditation governance consumers | Records framework creation | L2-H09 §11 |
| Accreditation Framework Updated | Accreditation Framework aggregate / H09 | Accreditation governance consumers | Records governed framework revision | L2-H09 §11 |
| Accreditation Framework Submitted for Review | Accreditation Framework aggregate / H09 | H09 governance participants | Records review entry | L2-H09 §11 |
| Accreditation Framework Approved | Accreditation Framework aggregate / H09 | Accreditation Cycle, readiness and governance consumers | Records institutional approval | L2-H09 §11 |
| Accreditation Framework Published | Accreditation Framework aggregate / H09 | Accreditation governance consumers | Records publication | L2-H09 §11 |
| Accreditation Framework Activated | Accreditation Framework aggregate / H09 | Accreditation Cycle, readiness and governance consumers | Records active governed availability | L2-H09 §11 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No cross-domain ownership transfer occurs. The framework defines accreditation expectations that later scenarios use when referencing source-domain evidence, while source-domain business records remain externally owned.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Framework approval sits with institutional governance after IQAC and steering review. Institutional Leadership provides the source-supported final approval for strategic accreditation positions; H09 then publishes and activates the approved framework.

## 15.2 Separation of Duties, Overrides and Exceptions

No activated framework may be changed in place. A changed framework follows versioned governance and approval rather than bypassing review.

## 15.3 Business Audit and Evidence Obligations

Preserve framework source/reference, version, criteria, metric and evidence definitions, scoring methodology, review history, approval evidence, publication/activation evidence, and prior-version lineage.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The framework version is institutionally approved.
- Criteria, evidence requirements, scoring, applicability, and review context are preserved in the governed version.
- The approved version is published and activated.
- Historical versions remain immutable.

## 16.2 Postconditions

The active H09 Accreditation Framework is available as the authoritative governance basis for later Accreditation Cycles and assessments.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

H09-A01 names the review state `Under Review`, while §7 names the equivalent lifecycle position `Review`. The discrepancy does not alter the frozen approved/published/active outcome, but upstream lifecycle terminology should be normalized before locking.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Normalize the H09 Accreditation Framework review-state name upstream.
