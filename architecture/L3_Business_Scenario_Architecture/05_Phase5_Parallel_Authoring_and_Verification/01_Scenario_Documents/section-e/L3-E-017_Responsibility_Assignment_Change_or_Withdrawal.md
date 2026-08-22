# Responsibility Assignment Change or Withdrawal

**Scenario ID:** L3-E-017  
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
This Draft preserves the frozen Stage 3 scenario identity, Primary Owner, Participating Domains, Scenario Type and Priority.

## 2. Scenario Purpose
Change, suspend or withdraw an existing Responsibility Assignment while preserving the governing Position Assignment and the historical lineage of the responsibility.

## 3. Business Outcome
The Responsibility Assignment reaches the requested authoritative changed, suspended or closed outcome without altering the lifecycle of its governing Position Assignment.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An existing Responsibility Assignment associated with an active Position Assignment requires an authorised business change, suspension or withdrawal.

### 4.2 End Boundary
The Responsibility Assignment has the authoritative revised or withdrawn business state, with its historical responsibility record preserved.

### 4.3 In Scope
- Identify the existing Responsibility Assignment and governing Position Assignment.
- Validate the requested responsibility change or withdrawal.
- Apply the appropriate responsibility lifecycle operation.
- Preserve the governing Position Assignment and responsibility history.
- Publish the applicable responsibility business event.

### 4.4 Out of Scope
Implementation mechanics, technical access control, infrastructure behaviour, and unrelated lifecycle outcomes remain outside this scenario.

### 4.5 Related Scenarios
No additional related scenario is asserted where a canonical relationship is not required to establish this boundary.

## 5. Ownership and Participating Domains
| Domain / Section | Responsibility |
|---|---|
| Position & Assignment | Owns Responsibility Assignment lifecycle, scope and historical lineage. |
| Organisation / Position context | Provides Position Definition context where relevant. |
| Workforce & Engagement | Provides Workforce Engagement context linked to the governing Position Assignment. |

## 6. Business Actors, Roles and Responsibilities
| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authorized administrator | Initiates the responsibility change, suspension or closure. | Initiates / performs governed change |
| Approving authority | Provides required responsibility approval where applicable. | Approves where required |

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- Responsibility Assignment exists.
- Governing Position Assignment is active.
- Responsibility Definition exists.
- Responsibility scope is defined.
- Required approval and policy conditions are satisfied.

## 8. Trigger
An authorized administrator initiates a governed change, suspension or withdrawal of an existing Responsibility Assignment.

## 9. Main Business Flow
| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify and validate the existing Responsibility Assignment and governing Position Assignment. | Position & Assignment | Responsibility is eligible for change. |
| 2 | Determine the applicable business change: update, suspend, close, extend or transfer as governed upstream. | Position & Assignment | Correct lifecycle path is selected. |
| 3 | Apply the approved responsibility change without changing the governing Position Assignment lifecycle. | Position & Assignment | Responsibility reaches the requested business outcome. |
| 4 | Preserve responsibility history and scope evidence. | Position & Assignment | Historical responsibility remains reconstructable. |
| 5 | Publish the applicable canonical responsibility event. | Position & Assignment | Change/withdrawal is available as an authoritative business fact. |

The flow stops when the defined Business Outcome and End Boundary are achieved.

## 10. Alternate Business Flows
| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Extended responsibility | Valid extension is approved | Extend the effective responsibility period | Responsibility remains governed for the revised period | Position & Assignment |
| Temporary suspension | Responsibility must pause without closure | Suspend and later reactivate through the approved lifecycle | Responsibility remains historically intact | Position & Assignment |

## 11. Failure, Interruption and Business Recovery
| Situation | Business Consequence | Business Resolution | Responsible Domain |
|---|---|---|---|
| Responsibility is no longer valid | Existing responsibility cannot continue | Close/revoke the responsibility through the governed lifecycle | Position & Assignment |
| Scope conflict | Responsibility cannot safely operate as requested | Reject or resolve under authoritative policy | Position & Assignment |
| Required approval is absent | Change cannot become valid | Complete the applicable governance path | Position & Assignment |
| Governing Position Assignment becomes non-operational | Dependent responsibility may no longer remain operational | Apply the upstream lifecycle dependency rules | Position & Assignment |

Only business-level resolution is described.

## 12. Business Rules and Decision Constraints
| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Responsibility Assignment belongs to exactly one active Position Assignment. | Position & Assignment | Change remains anchored to the canonical assignment. | L2-E02 §9.5 BR-026 |
| Every Responsibility Assignment references exactly one Responsibility Definition. | Position & Assignment | Change must remain within a canonical responsibility definition. | L2-E02 §9.5 BR-028 |
| Responsibility scope must be explicitly defined. | Position & Assignment | Changed responsibility remains bounded. | L2-E02 §9.5 BR-033 |
| Responsibility changes are lifecycle-governed and effective-dated where applicable. | Position & Assignment | Change/withdrawal uses existing lifecycle semantics. | L2-E02 §9.5 BR-041–042 |
| Revoking a Responsibility Assignment does not automatically terminate its governing Position Assignment. | Position & Assignment | Withdrawal remains distinct from Position closure. | L2-E02 §9.5 BR-043 |
| Responsibility history remains immutable. | Position & Assignment | Historical changes cannot rewrite prior facts. | L2-E02 §9.5 BR-046 |

The scenario applies upstream rules; it does not create or re-own them.

## 13. State and Lifecycle Impact
| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Responsibility Assignment | Draft / Approved / Active | Update, suspend, close, extend or transfer as governed | Existing lifecycle position appropriate to the selected operation; Active may progress to Suspended/Closed | Position & Assignment | L2-E02 §6.6.5; §10.12.3 |
| Position Assignment | Active | Governing assignment remains intact | Active unless separately changed by its own lifecycle | Position & Assignment | L2-E02 §9.5 BR-043 |

No new lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ResponsibilityUpdated` | Responsibility Assignment | Workflow / interested domains | Records a responsibility change. | L2-E02 §6.6.5 |
| `ResponsibilitySuspended` | Responsibility Assignment | Workflow / interested domains | Records temporary withdrawal of operational responsibility. | L2-E02 §10.12.3 |
| `ResponsibilityRevoked` | Responsibility Assignment | Workflow / interested domains | Records removal of responsibility. | L2-E02 §10.12.3 |
| `ResponsibilityClosed` | Responsibility Assignment | Workflow / interested domains | Records responsibility closure. | L2-E02 §6.6.5 |

### 14.2 Business Contracts and Cross-Domain Hand-offs
Organisation and Workforce facts remain supporting context. No participating domain becomes owner of the Responsibility Assignment through a cross-domain dependency.

## 15. Business Authorization, Approval and Audit Obligations
Every Responsibility Assignment identifies its approving authority, and responsibility changes are auditable. The scenario does not introduce a new named approval role or technical permission.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Requested responsibility change/withdrawal is reflected in the authoritative Responsibility Assignment lifecycle.
- Governing Position Assignment remains independently governed.
- Historical responsibility lineage remains intact.
- Applicable responsibility event is published.

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
