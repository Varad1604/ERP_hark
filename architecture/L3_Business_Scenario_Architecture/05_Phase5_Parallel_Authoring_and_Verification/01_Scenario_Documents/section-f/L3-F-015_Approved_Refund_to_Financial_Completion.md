# Approved Refund to Financial Completion

**Scenario ID:** L3-F-015  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Refund Management  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-F00 Finance Domain Model & Foundation; L2-F01 Student Charge & Fee Management; L2-F02 Payment Management Architecture; L2-F03 Refund Management; applicable Accounting architecture  
**Referenced By:** Accounting & General Ledger; Financial Reporting; Notification; Bank Reconciliation; Payment execution capability

## 2. Scenario Purpose

Complete the institutional business lifecycle of an already approved Refund from approved status through financial execution and completion, while preserving Refund ownership, financial integrity, approval evidence, and the separation between Refund Management and the capability that performs outbound payment execution.

## 3. Business Outcome

An approved Refund is financially executed and reaches the canonical Completed business position, with the resulting financial milestone recorded, immutable lineage preserved, and downstream financial consumers able to rely on the completed Refund fact.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins with a Refund in the canonical Approved lifecycle/state position.

### 4.2 End Boundary

The scenario ends when the Refund has reached the canonical Completed lifecycle/state position following successful outbound monetary execution.

### 4.3 In Scope

- Approved Refund context.
- Publication readiness.
- Refund publication.
- Execution initiation.
- External payment execution outcome.
- Recording of successful execution.
- Refund completion.
- Downstream accounting-ready and enterprise-consumption consequences.
- Preservation of references and audit evidence.

### 4.4 Out of Scope

- Refund request creation.
- Refund approval/rejection decision.
- Payment reception.
- Payment allocation.
- Accounting posting mechanics.
- Bank reconciliation mechanics.
- Technical payment-execution implementation.
- Technical retries, queues, transport handling, or infrastructure recovery.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| C — Student Lifecycle | Provides student context where required for the Refund's financial lineage. | Student context remains C-owned. |
| F — Refund Management | Owns Refund lifecycle, state, publication, references, events and completion record. | Refund remains F03-owned. |
| External payment execution capability | Performs the outbound monetary settlement represented by the Refund execution milestone. | Execution capability does not own the Refund aggregate. |
| F — Accounting & General Ledger | Consumes accounting-ready Refund information and owns accounting artifacts. | Accounting remains F05-owned. |

The frozen Stage 3 participating domains remain `C, F`.

## 6. Business Actors, Roles and Responsibilities

L2-F03 does not define actors. The execution capability is described architecturally as the capability performing outbound monetary settlement; no human or technical role is invented.

Refund Management remains responsible for the Refund business record and for recording the execution/completion business milestones.

## 7. Preconditions and Entry Criteria

- Refund is in the canonical `Approved` lifecycle/state position.
- Refund validation has completed.
- Refund possesses its canonical institutional reference.
- Gross Refund Amount, Deductions and Net Refund Amount satisfy the authoritative financial integrity constraints.
- The Refund is eligible for publication and execution.

## 8. Trigger

An Approved Refund becomes eligible for financial publication and outbound execution.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Refund Management confirms the Refund remains in the Approved position and retains its authoritative financial context. | Refund Management | Approved Refund is execution-ready. |
| 2 | Refund Management publishes the Refund for enterprise consumption. | Refund Management | Refund reaches the canonical Published position. |
| 3 | The approved Refund is made available to the appropriate payment execution capability. | Refund Management / payment execution capability | Execution responsibility is transferred operationally without transferring Refund ownership. |
| 4 | The payment execution capability completes the outbound monetary settlement. | Payment execution capability | The approved monetary return has been executed. |
| 5 | Refund Management records the canonical execution milestone and resulting business references. | Refund Management | Refund reaches the Executed position. |
| 6 | Refund Management completes the Refund's business responsibilities and records the Completed milestone. | Refund Management | Refund reaches financial completion. |
| 7 | Refund Management publishes the canonical completion fact for downstream consumers. | Refund Management | Accounting and other consumers receive an authoritative completed Refund fact. |

## 10. Alternate Business Flows

### A1 — Approved Refund Remains Pending Execution

Where execution has not yet completed, the Refund remains in the applicable canonical pending-execution state. The scenario does not treat approval as financial completion.

### A2 — Execution Produces an Authoritative Failure

Where execution fails, the Refund does not enter `Completed`. It follows the canonical Refund exception/state architecture and remains subject to the separate failure-resolution scenario.

### A3 — Cheque-Based Execution Becomes Stale

Where the canonical Refund state architecture records a `Stale Cheque` outcome, the Refund remains outside `Completed` until the applicable governed outcome is established.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Execution has not completed | Approved Refund is not yet financially completed. | Preserve the Refund in the appropriate pending execution position. | Refund Management |
| Execution fails | Outbound monetary settlement is not completed. | Use the canonical `Execution Failed` state and applicable governed recovery path; do not claim completion. | Refund Management / execution capability |
| Cheque execution becomes stale | Monetary return remains unresolved. | Use the canonical `Stale Cheque` state and governed business resolution. | Refund Management |
| Execution evidence is insufficient to establish completion | Refund cannot safely be marked completed. | Preserve the Refund and await authoritative execution evidence. | Refund Management / execution capability |
| Downstream accounting consequence is not yet established | Refund completion must not be confused with accounting ownership. | Preserve completed Refund fact; Accounting governs its own accounting outcome. | Accounting & General Ledger |

No technical retry, queue, rollback, or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only validated and approved Refunds may be published. | Refund Management | Execution starts only from an approved Refund. | L2-F03 |
| Refund lifecycle progresses forward and completed milestones are immutable. | Refund Management | Completion cannot rewrite prior history. | L2-F03 |
| Refund has exactly one active operational state. | Refund Management | Execution has one authoritative current state. | L2-F03 |
| Net Refund Amount = Gross Refund Amount − Total Deductions. | Refund Management | The amount executed is financially valid. | L2-F03 |
| Total Deductions ≤ Gross Refund Amount. | Refund Management | Invalid financial values cannot become completed Refunds. | L2-F03 |
| Refund Management does not create accounting entries. | Refund Management | Accounting remains independently owned. | L2-F03 |
| Refund execution is externally performed and the Refund Domain records the resulting milestone. | Refund Management / execution capability | Execution does not transfer Refund ownership. | L2-F03 |

## 13. State and Lifecycle Impact

### Refund Lifecycle

The authoritative lifecycle is:

`Refund Initiated → Validated → Approved → Published → Executed → Completed → Archived`

This scenario covers:

`Approved → Published → Executed → Completed`

### Refund Operational State

Relevant canonical states are:

`Approved → Published → Pending Execution → Executed → Completed`

Exception positions include `Execution Failed` and `Stale Cheque`. Exceptional states such as `Suspended` and `Cancelled` remain available where permitted by enterprise governance.

### Accounting

Accounting participation occurs downstream from the completed Refund fact. Accounting ownership is not transferred to Refund Management.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Refund Events

The materially relevant L2-F03 events are:

- `RefundApproved`
- `RefundPublished`
- `RefundExecutionStarted`
- `RefundExecuted`
- `RefundCompleted`

The Refund Domain is the canonical publisher of Refund events.

### 14.2 Synchronous Business Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Refund Management | Payment execution capability | Approved and published Refund execution context | Enables outbound monetary execution. |
| Payment execution capability | Refund Management | Authoritative execution outcome | Allows Refund Management to record Executed/Completed business milestones. |
| Refund Management | Accounting & General Ledger | Accounting-ready completed Refund fact | Enables Accounting to process its own accounting outcome. |
| Refund Management | Notification / Reporting / Analytics | Completed Refund fact | Enables downstream enterprise consumption without transferring ownership. |

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Approval has already occurred before this scenario begins. Enterprise Governance owns approval workflow and the approval outcome remains immutable.

### 15.2 Separation of Duties

No execution-specific human separation-of-duty rule is defined in L2-F03. No technical or human authority is invented.

### 15.3 Audit and Evidence

The scenario preserves:

- Refund institutional reference;
- Original Payment Reference;
- Student Charge Reference where applicable;
- Gross Refund Amount;
- Deductions;
- Net Refund Amount;
- approval outcome;
- publication milestone;
- execution outcome and reference;
- completion milestone;
- downstream accounting correlation where available.

L2-F03 requires complete Refund lineage and immutable historical records.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Refund began this scenario in `Approved`.
- Refund was published only after validation and approval.
- Outbound monetary execution completed successfully.
- Refund reached `Executed`.
- Refund reached `Completed`.
- Completion fact is published using the canonical Refund event vocabulary.
- Refund ownership remains with Refund Management.
- Accounting remains independently owned.

### 16.2 Postconditions

- Refund lifecycle position is `Completed`.
- Refund operational state is `Completed`.
- Execution and completion history are immutable and auditable.
- Downstream domains may consume the completed Refund fact.
- Refund is eligible for later archival under the canonical Refund lifecycle.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Execution Ownership Boundary

L2-F03 states that outbound monetary execution is performed by the appropriate payment execution capability, while the Refund Domain records the business milestone. The accessed architecture does not identify a dedicated canonical L2 owner for that payment-execution capability.

**Impact:** L3 can preserve the execution boundary and Refund ownership, but must not invent a separate execution domain or assign ownership to Payment Management without authoritative evidence.

**Required governance:** Confirm the canonical business owner/capability responsible for outbound Refund execution and its authoritative execution outcome contract/event.

### 17.2 Completion Semantics

L2-F03 clearly defines `Executed` and `Completed`, but the exact business condition distinguishing the two is not further elaborated in the accessed architecture.

**Impact:** The L3 preserves the sequence and stops at the canonical `Completed` milestone without inventing additional completion criteria beyond successful execution and completion of Refund business responsibilities.

### 17.3 Assumptions

No assumption overrides locked architecture.

### 17.4 Deferred Questions

The execution-capability ownership and precise Executed-to-Completed business condition should be confirmed during architecture review.
