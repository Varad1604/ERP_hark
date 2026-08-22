# Bulk Position Assignment Operation to Governed Batch Outcome

**Scenario ID:** L3-E-073  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment — People & Workforce  
**Participating Domains:** B — Institution & Structure; E — People & Workforce  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-E02 — Position & Assignment Workflows; L2-B02 — Organisation Workflows; L2-E01 — Workforce & Engagement; L3 Phase 5 Template v2.0  
**Referenced By:** None yet  

## 2. Scenario Purpose

Provide a governed institutional mechanism for executing a large-scale set of Position Assignment operations while preserving the canonical ownership and lifecycle of each assignment.

The scenario covers high-volume organisational workforce changes such as onboarding, restructuring, mass transfers, promotion cycles, retirement processing, semester workforce operations, and regulatory corrections without creating a second ownership model for Position Assignment.

## 3. Business Outcome

The submitted set of Position Assignment operations reaches a governed batch outcome in which every affected assignment has an authoritative business result, successful assignments have entered their applicable governed state, unsuccessful assignments have an explicit business disposition, and the complete batch outcome is traceable.

The scenario does not imply that every record must succeed. The authoritative L2 workflow explicitly supports partial success unless an atomic processing mode is selected.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an authorized administrator submits a validated batch request for one defined Position Assignment operation type.

### 4.2 End Boundary

The scenario ends when the batch has completed according to its selected processing mode, each included assignment has a governed result, the batch outcome has been recorded, and the relevant Position Assignment business events have been published.

### 4.3 In Scope

- Bulk creation, activation, suspension, transfer, additional-charge, committee-membership, multi-campus, administrative-correction, freeze, or archival operations where the authoritative E02 workflow supports the operation.
- Batch validation.
- Authorization and organisational-scope validation.
- Duplicate detection.
- Independent validation of each assignment.
- Execution according to the selected processing mode.
- Successful and failed assignment outcomes.
- Batch completion or partial completion.
- Business reporting of the batch result.
- Canonical Position Assignment events for successful business outcomes.

### 4.4 Out of Scope

- Defining new Position Assignment operations.
- Changing Position Definition ownership.
- Changing Workforce Engagement ownership.
- Creating a new batch lifecycle outside the locked E02 workflow.
- Technical batch processing, queues, parallel execution, database transactions, retries, rollback mechanics, or infrastructure recovery.

## 5. Ownership and Participating Domains

| Domain | Canonical Responsibility |
|---|---|
| Position & Assignment — E02 | Owns Position Assignment, assignment lifecycle, validation, effective dating, lineage, organisational authority and the governed bulk-operation outcome. |
| Organisation — B02 | Owns organisational structure and Position Definition context consumed when assignment validation requires it. |
| Workforce & Engagement — E01 | Owns Workforce Engagement facts referenced by Position Assignments. |

Participation does not transfer ownership. In particular, B02 does not become owner of Position Assignment and E01 does not become owner of assignment lifecycle.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Administrator / authorised organisational authority | Initiates the bulk Position Assignment operation and provides the governed business request. |
| Payroll / Leave / Academic / other consuming domains | Consume resulting Position Assignment business facts where their workflows require them; they do not own the assignment operation. |

The E02 workflow identifies an Administrator / Workflow as the business initiation context. No technical role is introduced.

## 7. Preconditions and Entry Criteria

- A supported Position Assignment operation has been selected.
- The batch request has been validated as a business request.
- Required organisational context exists.
- Required Workforce Engagement context exists for operations that create or modify assignments.
- The initiating authority is authorised for the relevant organisational scope.
- Duplicate detection and required institutional policy checks can be performed.
- The selected processing mode is explicit: Atomic, Best Effort, Validation Only, or Dry Run.

## 8. Trigger

An authorized administrator submits a validated batch request for a supported Position Assignment operation.

## 9. Main Business Flow

| Step | Business Behaviour | Result |
|---|---|---|
| 1 | Receive the batch request and identify its operation type and processing mode. | A defined bulk business operation exists. |
| 2 | Validate the submitted assignment dataset and organisational scope. | The batch is eligible for detailed processing or is rejected as a whole. |
| 3 | Verify the initiating authority and applicable institutional policy. | Business authority is established. |
| 4 | Detect duplicate or conflicting assignment entries. | Each affected entry has a duplicate/conflict determination. |
| 5 | Validate every assignment independently against Position Assignment rules and applicable Organisation / Workforce context. | Each assignment receives a validation outcome. |
| 6 | Execute the eligible assignment operations according to the selected processing mode. | Successful and unsuccessful assignment outcomes are established. |
| 7 | Preserve each successful assignment's canonical lifecycle, effective dating and historical lineage. | Successful assignment state is authoritative and traceable. |
| 8 | Collect the individual results into the governed batch outcome. | The batch is Completed, PartiallyCompleted, Failed, or otherwise governed by its selected processing mode. |
| 9 | Publish the applicable canonical Position Assignment business events for successful assignment outcomes. | Downstream domains can consume the authoritative assignment facts. |
| 10 | Complete the batch result and provide the business outcome to the initiating authority. | The bulk Position Assignment operation is closed with a traceable outcome. |

### 9.1 Processing Mode Behaviour

| Mode | Business Meaning |
|---|---|
| Atomic | The batch is governed as a whole; the batch succeeds or is rolled back according to the E02 atomic policy. |
| Best Effort | Successful assignments may complete while failed assignments remain explicitly reported. |
| Validation Only | Business validation is performed without committing assignment changes. |
| Dry Run | The institution receives an execution preview without committing assignment changes. |

## 10. Alternate Business Flows

### A1 — Validation Only

The submitted assignments are validated and their business outcomes are reported, but no assignment state is changed. The scenario ends with the validation result.

### A2 — Dry Run

The institution receives a preview of the expected assignment outcomes without committing the proposed changes. The preview itself is not treated as an authoritative Position Assignment outcome.

### A3 — Best-Effort Partial Completion

Some assignments satisfy all applicable rules while others fail validation or conflict checks. Successful assignments complete and failed assignments remain individually reported. The batch therefore reaches a governed partial-completion outcome rather than being treated as an undifferentiated failure.

### A4 — Atomic Processing

The selected policy requires the batch to be treated as a whole. If the business conditions for atomic completion are not satisfied, the batch does not produce a partial authoritative outcome.

### A5 — Duplicate Assignment Entry

A duplicate entry is rejected while other eligible records continue according to the selected processing mode.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Governed Resolution |
|---|---|
| Invalid assignment record | Record receives a failed validation outcome; remaining records continue unless Atomic mode governs the batch as a whole. |
| Duplicate detected | Duplicate entry is rejected and recorded in the batch result. |
| Authorisation failure | The affected operation is rejected; no unauthorised assignment outcome is accepted. |
| Position Definition invalid/unavailable | The affected assignment cannot proceed until the Organisation-owned context is valid. |
| Workforce Engagement invalid/unavailable | The affected assignment cannot proceed until the Workforce-owned context satisfies the assignment precondition. |
| Business policy violation | The affected assignment is rejected and the reason is retained in the batch result. |
| No valid assignment outcome remains | The batch reaches a failed business outcome without creating invalid organisational state. |

Technical failures, queues, retries, database rollback and infrastructure recovery are outside L3.

## 12. Business Rules and Decision Constraints

The scenario applies the authoritative E02 bulk-operation rules:

- Every assignment is validated independently.
- Bulk execution supports explicit processing modes.
- Every record produces an execution result.
- Bulk processing is fully auditable.
- Partial success is supported unless Atomic mode is selected.
- Each successful assignment publishes its own applicable canonical Position Assignment business events.
- Position Assignment remains the canonical owner of assignment lifecycle and organisational occupancy.
- Every Position Assignment references exactly one Position Definition and exactly one Workforce Engagement.
- Historical assignment versions remain immutable.
- Organisational authority is derived from active Position Assignments.

The scenario does not create additional local rules.

## 13. State and Lifecycle Impact

### 13.1 Batch Lifecycle

The locked E02 bulk workflow defines these business states:

`Queued → Validating → Processing → Completed`

with additional outcomes:

- `Processing → PartiallyCompleted`
- `Processing → Failed`

The scenario does not introduce additional batch states.

### 13.2 Position Assignment Lifecycle

Each individual assignment retains the lifecycle appropriate to its specific operation. The bulk operation does not create a new Position Assignment lifecycle.

Examples:

- create/activate → applicable assignment creation/activation lifecycle;
- transfer → successor assignment version and supersession behaviour;
- suspend → `Active → Suspended`;
- freeze → governed freeze behaviour;
- archive → eligible completed assignment to historical state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The bulk workflow explicitly defines:

- `BulkOperationStarted`
- `BulkOperationCompleted`
- `BulkOperationCancelled`
- `BulkAssignmentsProcessed`

For each successful individual assignment, the canonical Position Assignment event for the underlying operation is published by the Position & Assignment domain. E02 states that successful assignments publish their own domain events.

### 14.2 Cross-Domain Hand-offs

**Organisation → Position & Assignment:** authoritative Position Definition and organisational context.

**Workforce & Engagement → Position & Assignment:** authoritative Workforce Engagement context.

**Position & Assignment → consuming domains:** completed Position Assignment facts and applicable domain events.

The batch event and individual assignment events are business facts; they are distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Bulk execution requires:

- authorization for the relevant organisational scope;
- institutional approval where required by policy;
- designated governance authority for large-scale operations where dual authorization is required by institutional policy.

The business record of the batch must retain, at minimum, the batch identifier, operation type, processing mode, total records, successful records, failed records, execution outcome, and relevant event identifiers as defined by E02.

Section A retains ownership of technical Authorization and Audit infrastructure. This scenario defines only the business authority and evidence obligations.

## 16. Completion Criteria and Postconditions

The scenario is complete when:

- the selected processing mode has been honoured;
- every submitted assignment has an explicit business result;
- all successful assignments are authoritative within Position & Assignment;
- failed or rejected assignments are explicitly identified;
- the batch has a governed terminal outcome;
- applicable Position Assignment business events have been published;
- the batch result is traceable;
- no participating domain has acquired ownership of Position Assignment.

For Best Effort processing, `PartiallyCompleted` is a valid successful batch outcome when at least one eligible assignment has completed and the remaining outcomes are explicitly reported.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material upstream ownership, lifecycle, boundary, or final-outcome conflict was identified for this scenario.

E02 defines the bulk event vocabulary and processing modes directly. No additional bulk event or batch state is introduced.

### 17.2 Assumptions

None that override locked architecture.

### 17.3 Deferred Questions

None material to the L3 business outcome. Detailed execution mechanics remain intentionally deferred to later architecture levels.
