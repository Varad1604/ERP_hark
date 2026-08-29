# Refund Request to Approval or Rejection Decision

**Scenario ID:** L3-F-014  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Refund Management  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-F00 Finance Domain Model & Foundation; L2-F01 Student Charge & Fee Management; L2-F02 Payment Management Architecture; L2-F03 Refund Management  
**Referenced By:** Payment Management; Student Charge & Fee Management; Accounting & General Ledger; Financial Reporting

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-014 |
| Canonical Title | Refund Request to Approval or Rejection Decision |
| Primary Owner | Refund Management |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern a refund request from its establishment through a governed approval or rejection decision while preserving Refund ownership, financial lineage, validation integrity, and the separation between Refund Management and enterprise approval ownership.

The scenario ends at the decision outcome. Approved Refund execution is a separate canonical scenario.

## 3. Business Outcome

A valid Refund request reaches an authoritative approval decision: the Refund is approved and becomes eligible for the next lifecycle stage, or the request is rejected without being represented as an approved Refund.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a Refund request is established in the Refund Domain.

### 4.2 End Boundary

The scenario ends when the enterprise approval capability produces either a positive approval outcome or a rejection outcome for the Refund request.

### 4.3 In Scope

- Refund establishment.
- Refund source and reason context.
- Refund validation.
- Financial integrity validation.
- Enterprise approval participation.
- Approval or rejection decision.
- Preservation of decision evidence and Refund traceability.
- Resulting Refund lifecycle/state position.

### 4.4 Out of Scope

- Payment execution of an approved Refund.
- Accounting posting.
- Bank reconciliation.
- Refund cancellation/withdrawal after approval.
- Refund failure during execution.
- Technical approval workflow implementation.
- technical access control or authorization enforcement.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| C — Student Lifecycle | Provides authoritative student context where the Refund relates to student lifecycle circumstances. | Student context remains C-owned. |
| F — Refund Management | Establishes, validates, maintains and records the Refund aggregate and consumes the approval outcome. | Refund remains F03-owned. |
| Enterprise Governance capability | Owns approval workflow and approval routing. | Approval capability remains externally owned. |

The frozen Stage 3 participation value is `C, F` and is preserved. Approval participation does not transfer Refund ownership.

## 6. Business Actors, Roles and Responsibilities

L2-F03 explicitly states that Actors are not defined in the document and that Enterprise Governance owns approval workflows, routing, rules and hierarchy.

Therefore no student, finance officer, refund officer, approver, committee, or role-specific authority is invented.

The business responsibility is:

- Refund Management owns the Refund request and its financial integrity.
- Enterprise Governance owns the approval decision mechanism.
- Student Lifecycle provides authoritative student context where applicable.

## 7. Preconditions and Entry Criteria

- A Refund aggregate can be established from an authoritative financial context.
- The Refund has one canonical Refund Source and Refund Reason.
- The Refund has a canonical institutional reference.
- Refund financial validation can be performed.
- Enterprise approval participation is available for the decision.

## 8. Trigger

A Refund request is established and submitted for governed approval after required Refund validation.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Refund from its originating financial context and preserve the Refund Source and Refund Reason. | Refund Management | A canonical Refund exists. |
| 2 | Perform the authoritative Refund validation, including structural, financial, source, reason and reference validation. | Refund Management | The Refund is validated for approval participation. |
| 3 | Submit the validated Refund for enterprise approval participation. | Refund Management / Enterprise Governance | The Refund enters the governed approval decision. |
| 4 | Enterprise Governance evaluates the Refund according to its externally owned approval policy and produces an approval or rejection outcome. | Enterprise Governance | An authoritative decision exists. |
| 5 | Refund Management consumes the approval outcome without acquiring ownership of the approval workflow. | Refund Management | Refund state/lifecycle reflects the governed decision. |
| 6 | Where approved, the Refund becomes eligible for publication and subsequent execution under the separate approved-refund scenario. | Refund Management | Approved Refund is ready for downstream progression. |
| 7 | Where rejected, the Refund is not represented as an approved Refund and the decision remains traceable. | Refund Management | Rejected outcome is preserved. |

## 10. Alternate Business Flows

### A1 — Refund Requires Correction Before Approval

If the authoritative approval process returns a request for correction rather than producing a final approval/rejection decision, the Refund remains governed by the enterprise approval capability. No correction state is invented in this scenario.

### A2 — Refund Is Rejected

The Refund does not progress to the approved lifecycle position. The rejection outcome remains traceable.

No separate approval workflow is authored because L2-F03 explicitly places approval routing and rules outside the Refund Domain.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Refund fails financial validation | Refund cannot safely enter approval as a validated financial request. | Resolve the business validation deficiency before approval participation continues. | Refund Management |
| Approval authority is unavailable or undefined | No authoritative approval decision can be claimed. | Preserve the Refund and await governed approval resolution; do not invent an approver. | Enterprise Governance / Refund Management |
| Approval outcome is rejected | Refund cannot progress as an approved Refund. | Preserve the rejection decision and its financial lineage. | Refund Management |
| Approval context is disputed | Refund decision cannot safely be treated as authoritative. | Preserve the Refund and resolve the governing approval context through Enterprise Governance. | Enterprise Governance |

No technical retry, queue, timeout, or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Refund is represented by one canonical Refund aggregate. | Refund Management | Request remains Refund-owned. | L2-F03 |
| Every Refund has one Refund Source and at least one Refund Reason. | Refund Management | Request cannot bypass canonical financial context. | L2-F03 |
| Every Refund undergoes validation before publication. | Refund Management | Approval participation follows validated Refund context. | L2-F03 |
| Net Refund Amount = Gross Refund Amount − Total Deductions. | Refund Management | Financial integrity must hold before the Refund proceeds. | L2-F03 |
| Total Deductions ≤ Gross Refund Amount. | Refund Management | Refund cannot represent an invalid net amount. | L2-F03 |
| Approval workflow, routing, rules and hierarchy are Enterprise Governance-owned. | Enterprise Governance | Refund Management consumes the decision rather than defining approval. | L2-F03 |
| Approval outcome is immutable and auditable. | Enterprise Governance / Refund Management | Decision evidence remains traceable. | L2-F03 |

## 13. State and Lifecycle Impact

### Refund Lifecycle

The authoritative Refund lifecycle is:

`Refund Initiated → Validated → Approved → Published → Executed → Completed → Archived`

For this scenario, the relevant progression is:

`Refund Initiated → Validated → Approved`

or, where the decision is negative, the Refund does not enter the Approved lifecycle milestone.

### Refund Operational State

Relevant canonical states include:

`Draft → Pending Validation → Validated → Pending Approval → Approved`

The L2 state architecture also defines `Published`, `Pending Execution`, `Executed`, `Execution Failed`, `Stale Cheque`, `Completed`, `Suspended`, `Cancelled`, and `Archived`.

This scenario stops at the approval decision and does not execute the Refund.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Refund Events

L2-F03 defines canonical lifecycle events including:

- `RefundInitiated`
- `RefundValidated`
- `RefundApproved`
- `RefundPublished`
- `RefundExecutionStarted`
- `RefundExecutionFailed`
- `RefundExecuted`
- `RefundChequeDishonourRecorded`
- `RefundCompleted`
- `RefundSuspended`
- `RefundCancelled`
- `RefundArchived`

For this scenario, the materially relevant events are `RefundInitiated`, `RefundValidated`, and `RefundApproved`, published by the Refund Domain.

### 14.2 Synchronous Business Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Student Lifecycle | Refund Management | Authoritative student context | Supports the financial context for a student-related Refund. |
| Refund Management | Enterprise Governance | Validated Refund approval context | Requests the externally governed approval decision. |
| Enterprise Governance | Refund Management | Approval/rejection outcome | Establishes the authoritative decision consumed by Refund Management. |

Events remain distinct from synchronous approval participation.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Enterprise Governance owns approval workflow, routing, rules and hierarchy. Refund Management does not redefine these responsibilities.

### 15.2 Separation of Duties

No Refund-specific maker/checker or approver role is defined in L2-F03. No role-specific separation rule is invented.

### 15.3 Audit and Evidence

The scenario preserves:

- Refund Source;
- Refund Reason;
- Refund Reference;
- Gross Refund Amount;
- Refund Deductions;
- Net Refund Amount;
- validation outcome;
- approval participation context;
- approval/rejection decision;
- resulting Refund state/lifecycle position.

L2-F03 requires approval outcomes and refund progression to remain traceable and auditable.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Refund is canonically established.
- Refund validation is complete.
- Enterprise approval has produced an authoritative decision.
- Approved outcome is represented as `Approved`, or rejection is preserved without claiming approval.
- Refund ownership remains with Refund Management.
- Approval ownership remains with Enterprise Governance.

### 16.2 Postconditions

For approval:
- Refund lifecycle position is `Approved`.
- Refund operational state is `Approved`.
- Refund is eligible for publication and subsequent financial execution.

For rejection:
- Refund does not progress as an approved Refund.
- Rejection remains historically traceable.
- No outbound Refund execution is implied.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Upstream Definition Gap — Rejection State

L2-F03 defines approval/rejection as an enterprise approval outcome but does not include `Rejected` in the canonical Refund operational states or lifecycle milestones. The state list includes `Cancelled` and `Suspended`, while the lifecycle is Initiated → Validated → Approved → Published → Executed → Completed → Archived.

**Impact:** The L3 can correctly model the frozen scenario outcome as an approval or rejection decision, but it must not invent a `Rejected` Refund state or lifecycle stage.

**Required governance:** Confirm how a negative approval decision is represented in the Refund lifecycle/state model.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

The rejection representation requires upstream clarification before the scenario can be treated as fully resolved for Architecture Approved/Locked status.
