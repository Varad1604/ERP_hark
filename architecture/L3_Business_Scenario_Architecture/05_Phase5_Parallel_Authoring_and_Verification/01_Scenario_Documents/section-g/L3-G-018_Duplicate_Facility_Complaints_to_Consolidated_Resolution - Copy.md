# Duplicate Facility Complaints to Consolidated Resolution

## 1. Scenario Metadata

**Scenario ID:** L3-G-018  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.3  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-G01_Facilities_Management_Workflows.md; L3_Business_Scenario_Template_v2.0.md  
**Referenced By:** None yet  
**Architecture Approval:** ACR-G-2026-08-21-002  
**Approved By:** Arjun Mehta, Chief Enterprise Architect  
**Approval Date:** 21 August 2026  
**Approval Decision:** Approved — No Conditions  

## 2. Scenario Purpose

Consolidate confirmed duplicate Facility Complaints into one governed resolution context without deleting or overwriting their historical references.

## 3. Business Outcome

Duplicate Complaints are linked or merged under a governed relationship, resolved through the consolidated path, and preserved with complete lineage.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Facilities validation identifies two or more open Complaints as potential duplicates.

### 4.2 End Boundary

The duplicate relationship is established, the consolidated Complaint is resolved and closed, and every historical reference remains intact.

### 4.3 In Scope

- duplicate validation
- complaint linking or merging
- parent/child relationship preservation
- consolidated investigation, resolution and closure

### 4.4 Out of Scope

- deletion or replacement of duplicate Complaint history
- unrelated Complaint splitting into multiple Work Orders

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns duplicate validation, merge/link decision, consolidated resolution and Complaint history. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Supervisor | Reviews potential duplicates and applies configured merge/link policy. | Reviews and performs. |
| Facilities Manager | Governs the consolidation decision and resolution under policy. | Decides. |

## 7. Preconditions and Entry Criteria

- Each candidate Complaint exists and retains a valid infrastructure location.
- Applicable duplicate-detection, merge and linking policies are effective.

## 8. Trigger

Potential duplicate Facility Complaints are identified for governed consolidation.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Compare the Complaints against configured duplicate criteria. | Maintenance Supervisor | Duplicate relationship is confirmed or rejected. |
| 2 | Select the governed linking or merge treatment. | Facilities Manager | Parent/child or merged relationship is established. |
| 3 | Publish the canonical merge event when an approved merge occurs. | Facilities Management | Consolidation is visible and auditable. |
| 4 | Continue investigation and resolution through the consolidated Complaint context. | Maintenance Supervisor | One governed resolution path is applied. |
| 5 | Verify and close the resolved Complaint while retaining all linked histories. | Facilities Management | Consolidated resolution is complete. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Linked, not merged | Policy requires related Complaints to remain separately open but linked. | Preserve separate lifecycles and coordinate their resolution. | Linked governed resolution. | Facilities Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Duplicate not confirmed | Complaints must not be consolidated. | Retain independent processing and record the validation outcome. | Facilities Management |
| Merge would lose history | Consolidation violates the history invariant. | Reject the merge and use supported linking. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Historical complaint references remain intact. | Facilities Management | Prevents destructive consolidation. | L2-G01 §17 |
| Merge policies remain configuration-driven. | Facilities Management | Only governed consolidation is valid. | L2-G01 §§17, 36 |
| Verification precedes closure. | Facilities Management | Consolidated resolution must be verified. | L2-G01 §29 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Complaint | Open / Assigned / In Progress | Linking or merging changes relationship and resolution context without inventing a state. | Existing Complaint lifecycle continues to Verified → Closed | Facilities Management | L2-G01 §§17, 33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| ComplaintMerged | Facilities Management | Facilities operational participants | Records an approved merge while preserving lineage. | L2-G01 §31 |
| ComplaintClosed | Facilities Management | Facilities operational participants | Publishes verified final closure. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates in this scenario.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Validate duplicate relationship | Maintenance Supervisor | Must apply effective duplicate policy. | Facilities Management |
| Approve merge/link treatment | Facilities Manager | Must preserve historical references. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Duplicate determination | Candidate Complaint references, comparison outcome, actor and reason. | Facilities Management |
| Merge/link and closure | Parent/child lineage, resolution evidence, prior/new state and closure outcome. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Duplicate status was authoritatively determined.
- The governed merge or link preserved every historical reference.
- The consolidated resolution was verified and closed.

### 16.2 Postconditions

- Complaint lineage and consolidated resolution remain searchable.
- No Complaint history was deleted or overwritten.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
