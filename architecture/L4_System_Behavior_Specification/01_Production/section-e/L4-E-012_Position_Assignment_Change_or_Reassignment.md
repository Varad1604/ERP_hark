# L4-E-012 — Position Assignment Change or Reassignment

**Scenario ID:** L4-E-012  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-012 — Position Assignment Change or Reassignment  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-012. Position Assignment remains exclusively E02-owned.

## 2. System Behavior Purpose
Specify observable behavior for governing an authorized change or reassignment while preserving assignment identity, lineage, effective dating, and organizational authority.

## 3. System Behavior Outcome
An approved successor Position Assignment outcome becomes authoritative at its effective boundary while the prior position remains immutable history.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An operational Position Assignment is subject to an authorized organizational change or reassignment.
### 4.2 End Boundary
The approved successor outcome is authoritative at its effective boundary.
### 4.3 In Scope
Validate existing assignment, resolve target Position/organization, apply E02 change workflow, establish successor version, preserve history, publish completed business facts.
### 4.4 Out of Scope
Workforce Engagement lifecycle, Position Definition lifecycle, payroll, leave, recruitment, technical implementation.

## 5. Ownership and Participating Domains
Position & Assignment owns Position Assignment. Workforce & Engagement remains authoritative for the engagement; Organisation Management remains authoritative for Position and organizational context.

## 6. Business Actors, Roles and Responsibilities
Authorized institutional authority governs reassignment; Position & Assignment executes it; participating domains supply authoritative context.

## 7. Preconditions and Entry Criteria
- Existing Position Assignment is valid and operational.
- Workforce Engagement remains valid.
- Target Position/organization is authoritative.
- Change policy and required approval are satisfied.

## 8. Trigger
An authorized organizational change to an existing Position Assignment is established.

## 9. Main System Behavior
1. Validate the existing assignment.
2. Resolve target Position and organizational context.
3. Validate eligibility and effective dates.
4. Apply the canonical E02 transfer/change workflow.
5. Establish the successor Assignment Version.
6. Preserve prior assignment lineage as immutable history.
7. Establish the successor outcome at its effective boundary.
8. Publish the completed assignment-change facts.

## 10. Alternate System Behaviors
- Future-effective changes remain non-operational until effective.
- Invalid target context rejects the change.
- Incomplete approval prevents the successor outcome from becoming authoritative.

## 11. Failure, Interruption and Business Recovery
If validation or governance fails, the existing authoritative assignment remains unchanged. No technical rollback or retry mechanism is specified.

## 12. Business Rules and Decision Constraints
- Position Assignment ownership remains E02.
- Prior assignment history remains immutable.
- Effective dates remain valid.
- Organizational authority changes only through approved lifecycle behavior.

## 13. State and Lifecycle Impact
The existing Position Assignment follows the E02 change/reassignment lifecycle; no new state is introduced.

## 14. Business Events and Cross-Domain Collaboration
Completed assignment-change facts are published by Position & Assignment for downstream business consumption. Exact event vocabulary remains governed by the authoritative upstream event contract.

## 15. Business Authorization, Approval and Audit Obligations
Authorized reassignment follows E02 governance and remains auditable. Approval requirements are not technically implemented here.

## 16. Completion Criteria and Postconditions
- Successor Position Assignment outcome is authoritative when effective.
- Prior lineage remains preserved.
- Organizational authority reflects the approved successor outcome.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting the scenario boundary or ownership was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
