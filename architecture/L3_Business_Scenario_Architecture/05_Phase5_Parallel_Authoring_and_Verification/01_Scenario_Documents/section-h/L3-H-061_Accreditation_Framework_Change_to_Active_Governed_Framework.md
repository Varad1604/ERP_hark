# Accreditation Framework Change to Active Governed Framework

## 1. Scenario Metadata

**Scenario ID:** L3-H-061  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Govern a new or revised accreditation framework from definition through institutional approval, publication, and activation while preserving historical framework versions.

## 3. Business Outcome

A versioned framework is approved and activated with criteria, evidence requirements, scoring, and applicability preserved

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A regulator or institution introduces or revises an accreditation framework.

### 4.2 End Boundary

A versioned framework is approved and activated with criteria, evidence requirements, scoring, and applicability preserved.

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
| IQAC | Reviews framework alignment, evidence expectations, and readiness implications. | Reviews |
| Accreditation Cell | Prepares and maintains framework content and accreditation structure. | Performs |
| Compliance Administrator | Coordinates governed framework definition and lifecycle progression. | Performs |
| Institutional Leadership | Provides institutional approval for the governed framework position. | Approves |

## 7. Preconditions and Entry Criteria

- The accreditation authority is defined as a reference.
- Institutional governance approval arrangements exist.

## 8. Trigger

A regulator or institution introduces or revises an accreditation framework

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the new or revised accreditation framework requirement. | Accreditation Cell | Framework change is established for governance. |
| 2 | Define the versioned framework, criteria, metrics, evidence expectations, scoring methodology, applicability and review calendar. | Accreditation & Compliance | A complete draft framework version exists. |
| 3 | Submit the framework for governance review. | Accreditation & Compliance | The framework enters the governed review position. |
| 4 | Review and approve the framework through the H09 governance chain. | IQAC / Institutional Leadership | The framework is approved. |
| 5 | Publish and activate the approved framework. | Accreditation & Compliance | The approved version becomes the active governed framework. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Framework requires revision during review | Review identifies required corrections | Revise the draft/current version and resubmit through the governed review path. | Review continues without activating an unapproved version. | Accreditation & Compliance |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| An active framework requires modification | In-place modification would violate the locked framework rule. | Create a new framework version and govern it through review and approval. | Accreditation & Compliance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-H09-001 — only one active framework version may exist for a given authority and assessment cycle. | Accreditation & Compliance | Prevents simultaneous active versions. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-003 — framework modifications after activation require a new version. | Accreditation & Compliance | Forces successor-version governance. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-004 — historical frameworks remain immutable. | Accreditation & Compliance | Preserves historical framework evidence. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Accreditation Framework | Draft | Governed review, approval, publication and activation. | Active | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A01 / §7 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Accreditation Framework Created | Accreditation Framework aggregate | Accreditation Cycle; Readiness Assessment; Compliance Review | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Accreditation Framework Updated | Accreditation Framework aggregate | Accreditation Cycle; Readiness Assessment; Compliance Review | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Accreditation Framework Submitted for Review | Accreditation Framework aggregate | Accreditation Cycle; Readiness Assessment; Compliance Review | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Accreditation Framework Approved | Accreditation Framework aggregate | Accreditation Cycle; Readiness Assessment; Compliance Review | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Accreditation Framework Published | Accreditation Framework aggregate | Accreditation Cycle; Readiness Assessment; Compliance Review | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Accreditation Framework Activated | Accreditation Framework aggregate | Accreditation Cycle; Readiness Assessment; Compliance Review | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous contract or governed cross-domain hand-off materially participates beyond H09-owned workflow governance.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Framework review | IQAC | Reviews framework alignment and evidence sufficiency before approval. | Accreditation & Compliance |
| Framework approval | Institutional Leadership | H09 governance places final institutional approval with institutional governance after IQAC and steering review. | Accreditation & Compliance |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule is defined beyond the governed review and approval chain in H09.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Framework approval and activation | Approved framework version and governance decision | Accreditation & Compliance |
| Historical framework preservation | Immutable prior framework version | Accreditation & Compliance |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The successor framework version is approved.
- The approved framework is published and Active.
- Criteria, evidence requirements, scoring and applicability remain associated with the activated version.

### 16.2 Postconditions

- Exactly one applicable framework version is Active for the authority/assessment-cycle scope.
- Historical framework versions remain immutable.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H09-A01 names the review state `Under Review`, while H09 §7 names the same lifecycle position `Review`. | The intermediate framework lifecycle label is internally inconsistent, although both paths converge on Approved → Published → Active. | Upstream H09 lifecycle vocabulary should be reconciled before locking the L3 scenario. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
