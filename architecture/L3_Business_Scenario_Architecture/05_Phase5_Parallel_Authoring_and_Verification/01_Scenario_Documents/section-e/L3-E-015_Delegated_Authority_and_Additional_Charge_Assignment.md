# Delegated Authority and Additional Charge Assignment

**Scenario ID:** L3-E-015  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E02 — Position & Assignment Workflows; Organisation/Position Definition architecture; Workforce Engagement architecture  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
Frozen Final L3 identity, owner, participating domains, type and priority are preserved from the authoritative Phase 4/5 sources.

## 2. Scenario Purpose
Extend operational authority or responsibility for an existing Position Assignment through governed delegation and/or Additional Charge without transferring organisational occupancy.

## 3. Business Outcome
The governed delegated authority and/or Additional Charge becomes effective within its defined scope and period while the underlying Position Assignment and original organisational ownership remain unchanged.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An active Position Assignment and an authorised need for temporary delegated authority or additional operational responsibility exist.

### 4.2 End Boundary
The delegated authority and/or Additional Charge has reached its authoritative active outcome, with scope, period and original ownership preserved.

### 4.3 In Scope
- Validate the source Position Assignment and applicable responsibility.
- Validate the eligible receiving Position Assignment where delegation is used.
- Define and validate the business authority/responsibility scope.
- Apply the canonical delegation or Additional Charge workflow.
- Preserve original Position Assignment ownership and organisational occupancy.
- Publish the applicable completed business facts.

### 4.4 Out of Scope
Only adjacent outcomes owned by another canonical scenario/domain or implementation-level behaviour are excluded.

### 4.5 Related Scenarios
Where materially relevant, related canonical scenarios are identified without changing their ownership.

## 5. Ownership and Participating Domains
| Domain | Responsibility |
|---|---|
| Position & Assignment | Owns Assignment Delegation and Responsibility Assignment. |
| Organisation / Position Definition owner | Supplies organisational Position context. |
| Workforce & Engagement | Supplies active Workforce Engagement context. |

## 6. Business Actors, Roles and Responsibilities
- Source Position Assignment is Active.
- For delegation, delegate Position Assignment is Active and eligible.
- Scope and effective period are valid.
- Applicable institutional policy permits delegation/additional charge.
- Required governance is complete.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
An authorised Position Holder or institutional administrator establishes a temporary delegation or additional operational charge.

## 8. Trigger
| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Validate the source assignment, receiving assignment where applicable, scope and effective period. | Position & Assignment | Authority/responsibility request is eligible. |
| 2 | For delegation, establish the governed delegated authority without transferring Position occupancy. | Position & Assignment | Delegated authority is bounded by the source authority. |
| 3 | For Additional Charge, establish the responsibility on the active Position Assignment without creating a new occupancy. | Position & Assignment | Additional responsibility is operationally assigned. |
| 4 | Resolve the resulting effective authority according to institutional policy. | Position & Assignment | Authority remains deterministic. |
| 5 | Publish the applicable delegation/responsibility business facts. | Position & Assignment | Downstream participants can consume the completed business outcome. |

## 9. Main Business Flow
| Case | Condition | Alternate Path | Result |
|---|---|---|---|
| Future delegation/charge | Effective date is future | Schedule the governed authority/responsibility for its effective boundary | Existing authority remains current until effective |
| Partial delegation | Only selected authority is delegated | Apply the explicitly scoped delegation | Original authority remains outside delegated scope |
| Time-bound Additional Charge | Responsibility has an expiry | Apply the effective-dated responsibility lifecycle | Responsibility ends without ending Position occupancy |

The flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
| Situation | Business Consequence | Business Resolution | Responsible Domain |
|---|---|---|---|
| Invalid delegate or responsibility | Authority/responsibility cannot safely be established | Reject the requested assignment | Position & Assignment |
| Scope conflict | Resulting authority would be ambiguous | Reject or resolve under authoritative policy before activation | Position & Assignment |
| Governance not satisfied | Delegation/charge cannot become effective | Follow the applicable approval/governance path | Position & Assignment |
| Expiry/revocation | Delegated authority or responsibility must cease | Apply the canonical expiry/revocation lifecycle | Position & Assignment |

## 11. Failure, Interruption and Business Recovery
| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Delegation does not transfer Position occupancy. | Position & Assignment | Original organisational ownership remains unchanged. | L2-E02 W-007 / BR-W007-001 |
| Delegation is effective-dated and explicitly scoped. | Position & Assignment | Delegated authority has bounded business applicability. | L2-E02 W-007 / BR-W007-002–003 |
| Delegated authority cannot exceed delegator authority. | Position & Assignment | Receiving authority remains within source authority. | L2-E02 W-007 / BR-W007-006 / §9.4 BR-014 |
| Additional Charge does not create a Position Assignment. | Position & Assignment | Responsibility augmentation does not create occupancy. | L2-E02 W-010 / BR-W010-001–003 |
| Responsibility scope must be explicitly defined. | Position & Assignment | Additional Charge remains bounded. | L2-E02 W-010 / BR-W010-004 |
| Responsibility changes are auditable. | Position & Assignment | Business evidence remains traceable. | L2-E02 W-010 / BR-W010-005 |

Only business-level resolution is described.

## 12. Business Rules and Decision Constraints
| Business Concept | Relevant Lifecycle | Scenario Impact | Result |
|---|---|---|---|
| Assignment Delegation | Draft → Approved → Scheduled → Active → Revoked/Expired → Archived | Establishes temporary authority extension | Active within approved scope/period |
| Responsibility Assignment | Draft → Approved → Active → Expired/Revoked → Archived | Establishes Additional Charge | Active within approved scope/period |
| Position Assignment | Active | Underlying occupancy remains unchanged | Still Active |

Rules remain owned by the authoritative upstream domain.

## 13. State and Lifecycle Impact
| Event | Canonical Publisher | Purpose |
|---|---|---|
| `DelegationCreated` | Responsibility Assignment | Records creation of delegated authority. |
| `AuthorityDelegated` | Responsibility Assignment | Records the effective delegated-authority outcome. |
| `DelegationRevoked` | Responsibility Assignment | Records revocation of delegation. |
| `AuthorityRestored` | Responsibility Assignment | Records restoration of original authority. |
| `ResponsibilityAssignmentCreated` | Responsibility Assignment | Records creation of Additional Charge responsibility. |
| `ResponsibilityAssignmentEnded` | Responsibility Assignment | Records completion of the responsibility. |
| `AdditionalChargeAssigned` | Responsibility Assignment | Records the Additional Charge business outcome. |
| `AdditionalChargeRevoked` | Responsibility Assignment | Records revocation of the Additional Charge. |

No new lifecycle state or transition is created by this scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events
Organisation and Workforce facts are consumed as authoritative context; no contract transfers ownership of Position Assignment or Responsibility Assignment.

### 14.2 Business Contracts and Cross-Domain Hand-offs
Delegation and Additional Charge require the applicable institutional governance and approval. The L2 material establishes delegation/authority validation and responsibility governance but does not justify inventing a universal named approving role for this combined scenario.

## 15. Business Authorization, Approval and Audit Obligations
- Delegated authority, where used, is active only within its approved scope and period.
- Additional Charge, where used, is active on an existing Position Assignment.
- Position occupancy and original ownership remain unchanged.
- Applicable canonical business events are published.
- Authority remains deterministic and bounded.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
No open issues.

### 16.2 Postconditions
The completed business outcome remains authoritative and available for subsequent institutional behaviour according to upstream architecture.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No open issues.

### 17.2 Assumptions
No assumptions.

### 17.3 Deferred Questions
No scenario-specific deferred questions.

**Authoring classification:** PASS — source-grounded Draft.
