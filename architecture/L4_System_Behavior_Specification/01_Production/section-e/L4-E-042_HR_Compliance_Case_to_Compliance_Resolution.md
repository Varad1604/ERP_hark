# L4-E-042 — HR Compliance Case to Compliance Resolution

**Scenario ID:** L4-E-042  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-042 — HR Compliance Case to Compliance Resolution  
**Primary Owner:** HR  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-042. HR owns the compliance case and governed compliance resolution.

## 2. System Behavior Purpose
Specify observable behavior for establishing, verifying, remediating, and completing an HR compliance case.

## 3. System Behavior Outcome
The compliance requirement reaches a governed resolution through evidence collection, verification, and remediation where required.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
A compliance requirement is identified and a compliance case is established.
### 4.2 End Boundary
Compliance verification is complete and the case reaches its governed resolution.
### 4.3 In Scope
Requirement intake, evidence collection, verification, remediation, re-verification, case completion, and audit evidence.
### 4.4 Out of Scope
Employment lifecycle ownership, employee profile ownership, position assignment, technical implementation, and unrelated compliance policy definition.

## 5. Ownership and Participating Domains
HR owns the compliance case, workflow, verification outcome, and case completion. Workforce & Engagement remains the owner of authoritative employment lifecycle facts.

## 6. Business Actors, Roles and Responsibilities
HR case owner manages the workflow; employee/workforce member supplies evidence where applicable; verification authority establishes the business verification result.

## 7. Preconditions and Entry Criteria
- Compliance requirement exists.
- Affected workforce context is identifiable where required.
- Evidence and verification expectations are available.

## 8. Trigger
A compliance requirement is identified.

## 9. Main System Behavior
1. Establish the compliance case.
2. Collect required business evidence.
3. Verify evidence against the applicable requirement.
4. Complete the case when verification succeeds.
5. When verification fails, record remediation and return to verification.
6. Preserve the final compliance resolution and case history.

## 10. Alternate System Behaviors
- Missing evidence prevents successful verification.
- Failed verification enters governed remediation before re-verification.
- Unresolved remediation remains governed by the institutional compliance outcome.

## 11. Failure, Interruption and Business Recovery
Incomplete evidence leaves the case pending evidence collection. Failed verification leaves the requirement unresolved until remediation and re-verification occur. No technical retry or infrastructure behavior is defined.

## 12. Business Rules and Decision Constraints
- Compliance Case Aggregate is owned by HR.
- Compliance workflow history is immutable business history.
- Required evidence must be traceable to the compliance decision.
- Manual intervention is permitted for exceptional business cases under institutional governance.

## 13. State and Lifecycle Impact
RequirementCreated → EvidenceCollection → Verification → Completed, with Verification → Failed → Remediation → Verification. No new state is introduced.

## 14. Business Events and Cross-Domain Collaboration
`ComplianceCompleted` is the canonical compliance completion event. No additional event is invented. Evidence and verification are business handoffs within the governed HR workflow.

## 15. Business Authorization, Approval and Audit Obligations
Evidence, verification outcome, remediation decision, and completion decision remain attributable and auditable.

## 16. Completion Criteria and Postconditions
- Required evidence collected.
- Verification result recorded.
- Remediation completed where necessary.
- Compliance resolution recorded.
- `ComplianceCompleted` published on successful completion.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary, lifecycle, or outcome was identified in the locked L3 parent.

**Authoring Classification:** PASS — source-grounded Draft.
