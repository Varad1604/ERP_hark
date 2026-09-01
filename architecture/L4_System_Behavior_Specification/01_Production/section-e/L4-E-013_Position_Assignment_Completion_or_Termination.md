# L4-E-013 — Position Assignment Completion or Termination

**Scenario ID:** L4-E-013  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-013 — Position Assignment Completion or Termination  
**Primary Owner:** Position & Assignment  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-013. Position & Assignment owns Position Assignment completion/termination.

## 2. System Behavior Purpose
Specify observable behavior for bringing an operational Position Assignment to its authoritative completed or terminated business outcome while preserving history.

## 3. System Behavior Outcome
The Position Assignment ceases to be operational through its authoritative completion/closure outcome and its historical lineage remains traceable.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An active Position Assignment is subject to an authorized completion or termination decision.
### 4.2 End Boundary
The Position Assignment reaches the authoritative Closed outcome; subsequent archival is separate.
### 4.3 In Scope
Validate assignment and completion condition, apply canonical close/termination lifecycle, end occupancy and operational authority as governed, preserve immutable history, publish closure fact.
### 4.4 Out of Scope
Adjacent outcomes owned by another canonical scenario/domain and implementation-level behavior.

## 5. Ownership and Participating Domains
Position & Assignment owns Position Assignment completion/termination and lifecycle history.

## 6. Business Actors, Roles and Responsibilities
Authorized institutional authority governs completion; Position & Assignment executes the lifecycle. No unsupported technical role is introduced.

## 7. Preconditions and Entry Criteria
- Position Assignment is operational.
- Completion/termination condition is established.
- Required authorization and evidence are available.

## 8. Trigger
An authorized completion or termination decision is established.

## 9. Main System Behavior
1. Validate the Position Assignment and completion condition.
2. Validate required authority.
3. Apply the canonical completion/termination lifecycle.
4. End organizational occupancy and operational authority where required.
5. Preserve immutable assignment history and lineage.
6. Publish the completed closure business fact.

## 10. Alternate System Behaviors
- If authority is absent, the operational assignment remains authoritative.
- If the condition is disputed, the current authoritative position remains until resolution.
- Archival remains a separate subsequent lifecycle outcome.

## 11. Failure, Interruption and Business Recovery
A completion that cannot satisfy business conditions does not replace the current authoritative assignment. Technical recovery is outside L4.

## 12. Business Rules and Decision Constraints
- Position Assignment ownership remains E02.
- Closed assignment history remains immutable and traceable.
- Completion does not retroactively alter prior assignment facts.

## 13. State and Lifecycle Impact
The assignment follows the E02 completion/termination lifecycle and reaches the authoritative Closed outcome.

## 14. Business Events and Cross-Domain Collaboration
The completed closure fact is published by Position & Assignment using the authoritative upstream event vocabulary.

## 15. Business Authorization, Approval and Audit Obligations
Completion/termination follows E02 governance and remains auditable.

## 16. Completion Criteria and Postconditions
- Assignment is no longer operational.
- Authoritative Closed outcome is established.
- Historical lineage remains preserved.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary, lifecycle or outcome was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
