# HR Compliance Case to Compliance Resolution

**Scenario ID:** L3-E-042  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E04 — HR Workflows; L2-E01 — Workforce & Engagement Workflows; applicable HR policy and compliance governance  
**Referenced By:** None yet

## 1. Scenario Metadata

The canonical identity and frozen Stage 3 semantics are preserved. This scenario applies the locked upstream architecture and does not redefine ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose

Manage an HR compliance requirement through evidence collection and verification so that the institution reaches a governed compliance resolution.

## 3. Business Outcome

The HR compliance case reaches a verified compliance resolution, or a defined failed verification is taken through remediation and re-verification until the business outcome is established.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A compliance requirement is identified and a compliance case is created.

### 4.2 End Boundary

Compliance verification is completed and the case reaches its governed compliance resolution.

### 4.3 In Scope

Compliance requirement; case creation; evidence collection; verification; remediation where verification fails; compliance completion.

### 4.4 Out of Scope

HR does not own the resulting employment lifecycle fact, employee master/profile data, organisational structure, position assignment, or other specialised domain state.

### 4.5 Related Scenarios

None specified in the authoritative sources reviewed for this scenario.

## 5. Ownership and Participating Domains

| Domain | Business responsibility |
|---|---|
| HR / L2-E04 | Owns the HR employment-change/separation/compliance case, workflow execution, approvals, decision record and case outcome. |
| Workforce & Engagement / L2-E01 | Owns authoritative employment lifecycle state and publishes authoritative employment events after successful workforce state transitions. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

- HR Administrator / compliance case owner — manages the HR compliance workflow.
- Employee / workforce member — provides or is subject to required compliance evidence where applicable.
- Verification authority/team — performs business verification where applicable.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- A compliance requirement exists.
- The affected workforce context can be identified where required.
- Evidence requirements and verification expectations are available from applicable policy/governance.

## 8. Trigger

A compliance requirement is identified.

## 9. Main Business Flow

| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Create the HR compliance case from the identified requirement. | HR | Compliance case is established. |
| 2 | Collect the required business evidence. | HR / affected participant | Evidence is available for verification. |
| 3 | Verify the evidence against the applicable compliance requirement. | HR / verification authority | Verification result is established. |
| 4 | If verification succeeds, complete the compliance case. | HR | Compliance resolution is established. |
| 5 | If verification fails, establish the remediation outcome and return the case to verification. | HR / affected participant | The compliance requirement is re-evaluated through the governed business process. |

The flow stops when the singular institutional outcome and end boundary are reached.

## 10. Alternate Business Flows

- A failed verification moves the case to remediation and then back to verification, as defined by the locked HR compliance workflow.
- Where remediation does not resolve the requirement, the compliance case remains governed by its business outcome rather than being treated as technically failed.

## 11. Failure, Interruption and Business Recovery

| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Required evidence is incomplete | Compliance cannot yet be verified. | Obtain the required business evidence and continue the workflow. | HR |
| Verification fails | Requirement remains unresolved. | Record remediation and return to verification. | HR |
| Remediation does not resolve the requirement | Compliance resolution is not established. | Preserve the failed business result and follow applicable institutional compliance governance. | HR |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints

- Compliance Case Aggregate is owned by E04 HR Workflows.
- Compliance workflows cover mandatory documents, policy acknowledgements and verification requirements.
- Compliance execution follows the applicable institutional rules and policies; L3 does not invent local compliance criteria.
- Every workflow has a defined lifecycle and completed workflows publish relevant events.
- Workflow history is immutable business history.
- Manual intervention is supported for exceptional business cases.

These rules remain owned by authoritative upstream architecture.

## 13. State and Lifecycle Impact

The locked compliance workflow is: RequirementCreated → EvidenceCollection → Verification → Completed, with Verification → Failed → Remediation → Verification. These are the authoritative business workflow states described by L2-E04.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

**ComplianceCompleted** is the canonical primary event identified for the Compliance workflow. No additional compliance event is invented.

### 14.2 Synchronous Business Contracts / Hand-offs

Evidence collection and verification are business hand-offs among HR and the relevant verification participant. No technical integration contract is defined at L3.

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Compliance evidence, verification result and remediation outcome must remain traceable within the HR compliance case. Where approval/authorization is applicable, the decision must record actor, timestamp and decision. Technical permission enforcement is outside L3.

## 16. Completion Criteria and Postconditions

- Required evidence has been collected to the extent required by the governing compliance rule.
- Verification result is recorded.
- Successful verification reaches compliance completion and the ComplianceCompleted event is published.
- If verification fails, remediation and re-verification are handled according to the authoritative workflow.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified that materially affect ownership, outcome, lifecycle, event publisher, authority or scenario boundary.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.

**Authoring Classification:** PASS — source-grounded Draft.
