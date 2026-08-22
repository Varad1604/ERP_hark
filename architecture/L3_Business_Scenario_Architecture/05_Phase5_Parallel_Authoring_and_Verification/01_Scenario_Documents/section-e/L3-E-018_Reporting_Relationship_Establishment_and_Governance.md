# Reporting Relationship Establishment and Governance

**Scenario ID:** L3-E-018  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E02 — Position & Assignment Workflows; Organisation/Position Definition architecture; Workforce Engagement architecture  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
This Draft preserves the frozen Stage 3 scenario identity, Primary Owner, Participating Domains, Scenario Type and Priority.

## 2. Scenario Purpose
Establish a governed organisational Reporting Relationship between active Position Assignments while preserving the distinction between reporting accountability and operational authority.

## 3. Business Outcome
The Reporting Relationship is established as the authoritative organisational accountability relationship, with its type, effective applicability and hierarchy constraints satisfied.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A business need exists to establish a reporting relationship between active Position Assignments.

### 4.2 End Boundary
The Reporting Relationship reaches its authoritative established/active lifecycle position and becomes part of the governed organisational reporting structure.

### 4.3 In Scope
- Validate the participating active Position Assignments.
- Establish the reporting relationship and its permitted relationship type.
- Validate organisational consistency and reporting hierarchy constraints.
- Establish the relationship for its applicable effective period.
- Publish the canonical reporting-relationship business fact.

### 4.4 Out of Scope
Implementation mechanics, technical access control, infrastructure behaviour, and unrelated lifecycle outcomes remain outside this scenario.

### 4.5 Related Scenarios
No additional related scenario is asserted where a canonical relationship is not required to establish this boundary.

## 5. Ownership and Participating Domains
| Domain / Section | Responsibility |
|---|---|
| Position & Assignment | Canonically owns Reporting Relationships and their lifecycle. |
| Organisation / Position context | Supplies organisational Position context. |
| Workforce & Engagement | Supplies the Workforce Engagement context behind active Position Assignments. |

## 6. Business Actors, Roles and Responsibilities
| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| HR Administrator | Business administration of Position & Assignment records, where applicable. | Performs governed organisational assignment administration |
| Registrar Office | Institutional Position & Assignment administration, where applicable. | Performs governed organisational administration |

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- Reporting Position Assignment is active.
- Supervising Position Assignment is active.
- Both belong to the applicable institutional organisational context.
- The reporting relationship type is permitted by institutional policy.
- The resulting hierarchy does not violate reporting constraints.

## 8. Trigger
An authorised business administration action establishes a Reporting Relationship between active Position Assignments.

## 9. Main Business Flow
| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the reporting and supervising Position Assignments. | Position & Assignment | Both participating assignments are established as candidates. |
| 2 | Validate the permitted reporting relationship type and organisational context. | Position & Assignment | Relationship definition is valid. |
| 3 | Validate reporting hierarchy constraints, including the prohibition on circular reporting. | Position & Assignment | Organisational accountability remains consistent. |
| 4 | Establish the Reporting Relationship for its applicable effective period. | Position & Assignment | Reporting accountability is established. |
| 5 | Publish `ReportingRelationshipCreated`. | Position & Assignment | The reporting relationship becomes an authoritative business fact. |

The flow stops when the defined Business Outcome and End Boundary are achieved.

## 10. Alternate Business Flows
No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery
| Situation | Business Consequence | Business Resolution | Responsible Domain |
|---|---|---|---|
| One participating Position Assignment is inactive | Reporting relationship cannot be established as an active operational relationship | Reject until the assignment condition is satisfied | Position & Assignment |
| Relationship would create a circular hierarchy | Organisational reporting would become invalid | Reject the relationship | Position & Assignment |
| A Primary Reporting Relationship would conflict with an existing primary | Reporting accountability would become ambiguous | Resolve under the canonical reporting policy before establishment | Position & Assignment |

Only business-level resolution is described.

## 12. Business Rules and Decision Constraints
| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Reporting Relationship connects exactly one Reporting Position Assignment to one Supervising Position Assignment. | Position & Assignment | Establishment requires a defined pair of assignments. | L2-E02 §9.6 BR-051 |
| Reporting Relationships reference active Position Assignments only. | Position & Assignment | Both assignments must be operationally active. | L2-E02 §9.6 BR-052 |
| Reporting hierarchies must form directed acyclic graphs; circular reporting is prohibited. | Position & Assignment | Establishment must preserve hierarchy integrity. | L2-E02 §9.6 BR-055 |
| Every active Position Assignment has zero or one Primary Reporting Relationship. | Position & Assignment | Primary reporting cannot become ambiguous. | L2-E02 §9.6 BR-056 |
| Reporting Relationships are effective-dated. | Position & Assignment | Future relationships do not become operational prematurely. | L2-E02 §9.6 BR-063 |
| Reporting is independent of operational authority. | Position & Assignment | Establishment does not itself create approval authority. | L2-E02 §9.6 BR-066 |

The scenario applies upstream rules; it does not create or re-own them.

## 13. State and Lifecycle Impact
| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Reporting Relationship | Not established / Draft as applicable | Establish reporting accountability | Created/Activated according to effective lifecycle | Position & Assignment | L2-E02 §10.13.3 |
| Position Assignment | Active | Participates as reporting/supervising assignment | Remains Active | Position & Assignment | L2-E02 §9.6 BR-052 |

No new lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ReportingRelationshipCreated` | Reporting Relationship | Leave, Payroll and other interested domains | Records establishment of reporting accountability. | L2-E02 §8.2.13; §10.13.3 |
| `ReportingRelationshipActivated` | Reporting Relationship | Interested domains | Records effective activation. | L2-E02 §10.13.3 |

### 14.2 Business Contracts and Cross-Domain Hand-offs
Organisation and Workforce provide authoritative Position/Workforce context. Reporting Relationship ownership remains entirely within Position & Assignment. The relationship may be consumed by Leave and Payroll as interested domains without transferring ownership.

## 15. Business Authorization, Approval and Audit Obligations
The locked L2 identifies HR Administrator and Registrar Office as primary Position & Assignment workflow actors. It does not provide a reporting-specific approval role. No technical permission model is introduced. Reporting establishment remains subject to the upstream institutional policy and hierarchy constraints.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- A valid Reporting Relationship exists between the defined active Position Assignments.
- Relationship type and effective applicability are governed.
- Reporting hierarchy remains non-circular.
- Primary reporting remains deterministic.
- `ReportingRelationshipCreated` is published.

### 16.2 Postconditions
The completed business outcome remains authoritative for subsequent institutional behaviour according to upstream architecture.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
**Open issue:** The locked L2 material clearly establishes Reporting Relationships as a canonical concept, ownership, invariants, rules, lifecycle events and `ReportingRelationshipCreated`, but the repository material available for this authoring pass does not expose a dedicated detailed Reporting Relationship workflow section comparable to W-011. Detailed establishment sequencing and any reporting-specific approving role should therefore be verified at upstream/L4 review before Architecture Approval.

### 17.2 Assumptions
Where the L2 source does not define a reporting-specific actor beyond the domain's documented primary actors, no additional role is invented.

### 17.3 Deferred Questions
No scenario-specific deferred questions.

**Authoring classification:** PASS WITH ISSUES — source-grounded Draft.
