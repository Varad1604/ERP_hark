# Committee Membership Assignment

**Scenario ID:** L3-E-016  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E02 — Position & Assignment Workflows; Organisation/Position Definition architecture; Workforce Engagement architecture  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
This Draft preserves the frozen Stage 3 scenario identity, Primary Owner, Participating Domains, Scenario Type and Priority.

## 2. Scenario Purpose
Establish a workforce member's formal participation in an institutional committee while keeping committee membership distinct from organisational Position occupancy.

## 3. Business Outcome
The eligible individual is formally appointed as a Committee Member with the applicable membership basis, role, effective period and governance participation rights.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A committee appointment is initiated for an eligible individual.

### 4.2 End Boundary
The Committee Membership is established as the authoritative governance participation outcome.

### 4.3 In Scope
- Validate the Committee and its active status.
- Validate the individual's eligibility and active Position Assignment.
- Determine the membership basis.
- Establish the Committee Membership with its role and defined participation rights.
- Publish the canonical committee-membership business fact.

### 4.4 Out of Scope
Implementation mechanics, technical access control, infrastructure behaviour, and unrelated lifecycle outcomes remain outside this scenario.

### 4.5 Related Scenarios
No additional related scenario is asserted where a canonical relationship is not required to establish this boundary.

## 5. Ownership and Participating Domains
| Domain / Section | Responsibility |
|---|---|
| Position & Assignment | Owns Committee Membership as a governance participation construct. |
| Organisation / Position context | Provides organisational/Position context used for eligibility. |
| Workforce & Engagement | Provides the active workforce context for the eligible individual. |

## 6. Business Actors, Roles and Responsibilities
| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authorized governance authority | Appoints an eligible individual to a committee. | Appoints / governs |
| Eligible individual | Participates in the appointed committee membership. | Receives / performs membership role |

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- Committee exists and is active.
- Position Assignment is active.
- Eligibility is satisfied.
- Organisational approval is complete.
- Policy validation is successful.
- Authorization is successful.

## 8. Trigger
An authorized governance authority appoints an eligible individual to a committee.

## 9. Main Business Flow
| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate the committee and its active status. | Position & Assignment | Committee is eligible for appointment. |
| 2 | Validate the individual's active Position Assignment and eligibility. | Position & Assignment | Individual is eligible for membership. |
| 3 | Determine the membership basis and committee role. | Authorized governance authority / Position & Assignment | Governance participation terms are defined. |
| 4 | Establish the Committee Membership, including effective period and applicable voting/quorum participation. | Position & Assignment | Formal committee participation exists. |
| 5 | Publish `CommitteeMemberAppointed`. | Position & Assignment | Appointment is an authoritative business fact. |

The flow stops when the defined Business Outcome and End Boundary are achieved.

## 10. Alternate Business Flows
No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery
| Situation | Business Consequence | Business Resolution | Responsible Domain |
|---|---|---|---|
| Committee is inactive or does not exist | Appointment cannot be established | Reject until the committee condition is satisfied | Position & Assignment |
| Eligibility is not satisfied | Individual cannot be appointed | Reject the appointment | Position & Assignment |
| Required governance/approval is incomplete | Membership cannot become valid | Follow the applicable governance path before appointment | Position & Assignment |

Only business-level resolution is described.

## 12. Business Rules and Decision Constraints
| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Committee Membership shall not create Position Assignments. | Position & Assignment | Membership remains governance participation rather than occupancy. | L2-E02 W-011 §8.12.9 |
| Committee Membership shall preserve committee history. | Position & Assignment | Membership decisions remain historically reconstructable. | L2-E02 W-011 §8.12.9 |
| Committee authority remains committee-scoped. | Position & Assignment | Membership does not establish general organisational authority. | L2-E02 W-011 §8.12.9 |
| Committee lifecycle is independent of Position lifecycle. | Position & Assignment | Membership lifecycle does not alter Position Assignment lifecycle. | L2-E02 W-011 §8.12.9 |

The scenario applies upstream rules; it does not create or re-own them.

## 13. State and Lifecycle Impact
| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Committee Membership | Eligible for appointment | Membership is created and established for its approved period. | Active/established membership according to effective dating | Position & Assignment | L2-E02 W-011 §§8.12.5–8.12.8 |

No new lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `CommitteeMemberAppointed` | Committee Membership | Governance / Examination interested domains | Records committee appointment. | L2-E02 §8.12.8; §10.19.3 |
| `CommitteeQuorumChanged` | Committee Membership | Governance | Records a quorum consequence where the membership change affects quorum. | L2-E02 §8.12.8 |
| `CommitteeVotingRightsAssigned` | Committee Membership | Governance | Records voting-right assignment where applicable. | L2-E02 §8.12.8 |

### 14.2 Business Contracts and Cross-Domain Hand-offs
Position & Assignment consumes authoritative Position/Workforce context for eligibility. Committee Membership ownership remains within Position & Assignment; participating domains do not acquire ownership through the dependency.

## 15. Business Authorization, Approval and Audit Obligations
Appointment requires organisational approval, policy validation and authorization as defined by the locked L2 workflow. Audit/evidence obligations are governed upstream; no technical permission model is introduced.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Committee Membership is established for the eligible individual.
- Membership role/basis and applicable participation rights are defined.
- Committee history is preserved.
- `CommitteeMemberAppointed` is published.

### 16.2 Postconditions
The completed business outcome remains authoritative for subsequent institutional behaviour according to upstream architecture.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No open issues.

### 17.2 Assumptions
No assumptions.

### 17.3 Deferred Questions
No scenario-specific deferred questions.

**Authoring classification:** PASS — source-grounded Draft.
