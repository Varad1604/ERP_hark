# L2-E05 --- Leave Management Workflows



------------------------------------------------------------------------

# 0. Document Metadata

  -----------------------------------------------------------------------
  Attribute                           Value
  ----------------------------------- -----------------------------------
  Document Name                       L2-E05 --- Leave Management
  Document Owner                      Varad
                                      Workflows

  Document Layer                      Level 2 --- Domain Workflow
                                      Architecture

  Domain Code                         E05

  Domain Name                         Leave Management

  Document Type                       Enterprise Domain Workflow
                                      Specification

  Architecture Style                  Domain-Driven Design (DDD) +
                                      Workflow Architecture

  Primary Ownership                   Leave Management Domain

  Version                             1.0

  Status                              Draft
                                      Phase
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 1. Purpose

The Leave Management domain provides the enterprise capability for
defining, managing, processing, and governing employee leave lifecycle
operations across SutraERP.

The domain establishes a controlled framework for:

-   Leave policy definition and governance
-   Leave eligibility determination
-   Leave entitlement generation
-   Leave accrual processing
-   Leave balance management
-   Leave request lifecycle management
-   Approval workflow execution
-   Leave adjustment processing
-   Carry-forward management
-   Leave encashment eligibility management
-   Leave-related business event publication

Leave Management is the authoritative domain for leave-related business
decisions while consuming workforce, organizational, calendar,
attendance, and authorization context from other enterprise domains.

------------------------------------------------------------------------

# 2. Scope

## 2.1 In Scope

-   Leave Policy Management
-   Leave Eligibility Rules
-   Leave Entitlement Management
-   Leave Accrual Processing
-   Leave Request Lifecycle
-   Approval Workflow
-   Leave Balance Management
-   Leave Ledger Management
-   Leave Adjustments
-   Carry Forward Processing
-   Leave Encashment Eligibility

## 2.2 Out of Scope

  Capability               Owner
  ------------------------ -------------------------
  Employee Master Data     Employee Profile Domain
  Organization Hierarchy   Organization Domain
  Attendance Calculation   Attendance Domain
  Salary Calculation       Payroll Domain
  Notification Delivery    Notification Domain
  Document Storage         Document Platform

------------------------------------------------------------------------

# 3. Upstream Dependencies

``` mermaid
flowchart LR

Employee["Employee Profile"]

Organization["Organization"]

Calendar["Organization Calendar"]

Attendance["Attendance"]

Security["Authorization Platform"]

Employee --> Leave["Leave Management"]

Organization --> Leave

Calendar --> Leave

Attendance --> Leave

Security --> Leave
```

Leave Management consumes:

-   Employee context
-   Organization context
-   Calendar information
-   Attendance events
-   Authorization context

------------------------------------------------------------------------

# 4. Downstream Dependencies

``` mermaid
flowchart LR

Leave["Leave Management"]

Payroll["Payroll"]

Analytics["Analytics"]

Notification["Notification"]

Audit["Audit Platform"]

Leave --> Payroll

Leave --> Analytics

Leave --> Notification

Leave --> Audit
```

Leave Management publishes:

-   Leave approval information
-   Leave balance changes
-   Leave lifecycle events
-   Eligibility information

------------------------------------------------------------------------

# 5. Domain Ownership Boundaries

## Leave Management Owns

-   Leave Policies
-   Leave Eligibility
-   Leave Entitlements
-   Leave Accrual
-   Leave Requests
-   Leave Approvals
-   Leave Balances
-   Leave Ledger
-   Leave Adjustments
-   Carry Forward
-   Leave Encashment Eligibility
-   Leave Domain Events

------------------------------------------------------------------------

## Leave Management Does Not Own

-   Employee identity
-   Organization hierarchy
-   Attendance calculation
-   Salary calculation
-   Document storage
-   Notification delivery
-   Enterprise security engine

------------------------------------------------------------------------

``` mermaid
flowchart TD

E05["Leave Management"]

E02["Employee Profile"]

ORG["Organization"]

ATT["Attendance"]

PAY["Payroll"]

DOC["Document Platform"]

NOTIFY["Notification Domain"]

E02 -->|Employee Context| E05

ORG -->|Hierarchy Context| E05

ATT -->|Attendance Context| E05

E05 -->|Leave Facts| PAY

E05 -->|Events| NOTIFY

DOC -->|References| E05
```

------------------------------------------------------------------------

## Boundary Principle

E05 owns leave decisions but never owns external business facts required
to make those decisions.

This preserves domain ownership clarity and prevents duplication across
SutraERP.



------------------------------------------------------------------------

# 6. Domain Model / Aggregate / Lifecycle

## 6.1 Domain Model Overview

The Leave Management domain is modeled around four primary aggregates:

  -----------------------------------------------------------------------
  Aggregate                           Responsibility
  ----------------------------------- -----------------------------------
  Leave Policy Aggregate              Defines leave rules, eligibility,
                                      inheritance, and policy versions

  Leave Request Aggregate             Controls leave application and
                                      approval lifecycle

  Leave Balance Aggregate             Represents current employee leave
                                      availability projection

  Leave Ledger Aggregate              Maintains immutable leave
                                      financial-style transactions
  -----------------------------------------------------------------------

The design follows the principle:

> Leave balance is not the source of truth. The Leave Ledger is the
> source of truth.

------------------------------------------------------------------------

# 6.2 Aggregate Relationship Model

``` mermaid
flowchart TD

Policy["Leave Policy Aggregate"]

Request["Leave Request Aggregate"]

Ledger["Leave Ledger Aggregate"]

Balance["Leave Balance Aggregate"]

Employee["Employee Context"]

Policy --> Request

Request --> Ledger

Ledger --> Balance

Employee --> Policy

Employee --> Request
```

------------------------------------------------------------------------

# 6.3 Leave Policy Aggregate

## Responsibility

The Leave Policy Aggregate manages the rules that determine:

-   Leave type availability
-   Eligibility conditions
-   Accrual rules
-   Carry-forward rules
-   Expiry rules
-   Employee category applicability
-   Policy version lifecycle

------------------------------------------------------------------------

## Core Entities

``` text
Leave Policy

Leave Type Rule

Eligibility Rule

Accrual Rule

Carry Forward Rule

Policy Version
```

------------------------------------------------------------------------

## Lifecycle

``` mermaid
stateDiagram-v2

[*] --> Draft

Draft --> UnderReview

UnderReview --> Approved

Approved --> Scheduled

Scheduled --> Active

Active --> Superseded

Superseded --> Archived
```

------------------------------------------------------------------------

# 6.4 Leave Request Aggregate

## Responsibility

The Leave Request Aggregate controls:

-   Leave submission
-   Validation state
-   Approval lifecycle
-   Cancellation
-   Withdrawal
-   Completion

------------------------------------------------------------------------

## Core Entities

``` text
Leave Request

Leave Period

Approval Step

Approval Decision

Request Status
```

------------------------------------------------------------------------

## Lifecycle

``` mermaid
stateDiagram-v2

[*] --> Draft

Draft --> Submitted

Submitted --> Validated

Validated --> PendingApproval

PendingApproval --> Approved

PendingApproval --> Rejected

Approved --> Reserved

Reserved --> Consumed

Consumed --> Closed

Approved --> Cancelled

Reserved --> Cancelled

Rejected --> Closed
```

------------------------------------------------------------------------

# 6.5 Leave Balance Aggregate

## Responsibility

The Leave Balance Aggregate provides the current available leave view.

It is a projection generated from ledger transactions.

It does not directly modify balances.

------------------------------------------------------------------------

## Core Entities

``` text
Employee Leave Balance

Leave Type Balance

Available Balance

Reserved Balance

Consumed Balance
```

------------------------------------------------------------------------

## Balance Calculation Model

``` mermaid
flowchart LR

Ledger["Leave Ledger Transactions"]

Projection["Balance Projection"]

Balance["Current Leave Balance"]

Ledger --> Projection

Projection --> Balance
```

------------------------------------------------------------------------

# 6.6 Leave Ledger Aggregate

## Responsibility

The Leave Ledger Aggregate is the authoritative record of leave
transactions.

All balance-affecting operations create ledger entries.

------------------------------------------------------------------------

## Core Transactions

``` text
Opening Entitlement Transaction

Accrual Transaction

Consumption Transaction

Carry Forward Transaction

Expiry Transaction

Adjustment Transaction

Reversal Transaction
```

------------------------------------------------------------------------

## Ledger Principle

``` text
No direct balance mutation.

Every balance change requires:

Business Action

↓

Ledger Transaction

↓

Balance Projection Update
```

------------------------------------------------------------------------

# 6.7 Aggregate Boundary Rules

## Leave Policy Aggregate Owns

-   Policy decisions
-   Rule evaluation
-   Policy version lifecycle

------------------------------------------------------------------------

## Leave Request Aggregate Owns

-   Request lifecycle
-   Approval state
-   Request decisions

------------------------------------------------------------------------

## Leave Ledger Aggregate Owns

-   Transaction history
-   Balance-affecting events

------------------------------------------------------------------------

## Leave Balance Aggregate Owns

-   Current balance representation

------------------------------------------------------------------------

# 6.8 Domain Lifecycle Overview

``` mermaid
flowchart LR

Policy["Policy Activation"]

Eligibility["Eligibility Evaluation"]

Entitlement["Entitlement Creation"]

Request["Leave Request"]

Approval["Approval Workflow"]

Reservation["Balance Reservation"]

Consumption["Leave Consumption"]

Ledger["Ledger Transaction"]

Projection["Balance Projection"]


Policy --> Eligibility

Eligibility --> Entitlement

Entitlement --> Ledger

Request --> Approval

Approval --> Reservation

Reservation --> Consumption

Consumption --> Ledger

Ledger --> Projection
```

------------------------------------------------------------------------

# 6.9 Lifecycle Design Principles

## Immutable History

Historical leave transactions must never be edited.

Corrections happen through:

``` text
Original Transaction

↓

Reversal Transaction

↓

Correct Transaction
```

------------------------------------------------------------------------

## Effective Dating

Policies and rules must support:

-   Start date
-   End date
-   Version history

------------------------------------------------------------------------

## Aggregate Ownership

Each aggregate controls its own invariants.

External domains cannot directly mutate aggregate state.

------------------------------------------------------------------------

# 6.10 Final Aggregate Decision

The final locked aggregate architecture:

``` text
Leave Policy Aggregate

        |

        ↓

Leave Request Aggregate

        |

        ↓

Leave Ledger Aggregate

        |

        ↓

Leave Balance Projection
```

This model ensures:

-   Auditability
-   Policy consistency
-   Reconciliation capability
-   Enterprise scalability




------------------------------------------------------------------------

# 7. Domain Workflows

The Leave Management domain operates through controlled business
workflows.

Each workflow:

-   Has a defined trigger
-   Executes within domain boundaries
-   Maintains aggregate integrity
-   Produces auditable outcomes
-   Publishes domain events when required

------------------------------------------------------------------------

# 7.1 Leave Request Workflow

## Purpose

Handles employee leave submission from request creation until final
completion.

## Actors

-   Employee
-   Manager / Approver
-   Leave Management Domain
-   Policy Engine

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart TD

Start["Employee Creates Leave Request"]

Validate["Validate Request"]

Eligibility["Evaluate Eligibility"]

Balance["Check Leave Balance"]

Approval["Approval Workflow"]

Decision{"Approved?"}

Reserve["Reserve Leave Balance"]

Reject["Reject Request"]

Consume["Leave Consumed"]

Ledger["Create Ledger Transaction"]

Event["Publish Leave Events"]

End["Workflow Completed"]


Start --> Validate

Validate --> Eligibility

Eligibility --> Balance

Balance --> Approval

Approval --> Decision

Decision -->|Yes| Reserve

Decision -->|No| Reject

Reserve --> Consume

Consume --> Ledger

Ledger --> Event

Reject --> Event

Event --> End
```

------------------------------------------------------------------------

# 7.2 Leave Approval Workflow

## Purpose

Controls approval decisions while keeping authorization responsibility
outside the Leave Aggregate.

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart TD

Request["Leave Request Submitted"]

Routing["Determine Approval Route"]

Approver["Assigned Approver"]

Review{"Decision"}

Approve["Approve Leave"]

Reject["Reject Leave"]

Escalate["Escalation"]

Event["Publish Decision Event"]


Request --> Routing

Routing --> Approver

Approver --> Review

Review -->|Approve| Approve

Review -->|Reject| Reject

Approver -->|Timeout| Escalate

Escalate --> Approver

Approve --> Event

Reject --> Event
```

------------------------------------------------------------------------

# 7.3 Leave Accrual Workflow

## Purpose

Generates employee leave entitlement based on configured accrual rules.

------------------------------------------------------------------------

## Trigger

-   Scheduled accrual cycle
-   Policy-defined event
-   Administrative correction

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart LR

Trigger["Accrual Trigger"]

Policy["Load Active Policy"]

Eligibility["Evaluate Employee Eligibility"]

Calculate["Calculate Accrual"]

Ledger["Create Accrual Transaction"]

Balance["Update Balance Projection"]

Event["Publish Accrual Event"]


Trigger --> Policy

Policy --> Eligibility

Eligibility --> Calculate

Calculate --> Ledger

Ledger --> Balance

Balance --> Event
```

------------------------------------------------------------------------

# 7.4 Carry Forward Workflow

## Purpose

Handles transition of eligible unused leave between leave periods.

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart TD

YearEnd["Leave Year Closure"]

Policy["Evaluate Carry Forward Rules"]

Eligible{"Eligible?"}

Carry["Create Carry Forward Transaction"]

Expire["Expire Remaining Balance"]

Ledger["Update Leave Ledger"]

Projection["Update Balance Projection"]


YearEnd --> Policy

Policy --> Eligible

Eligible -->|Yes| Carry

Eligible -->|No| Expire

Carry --> Ledger

Expire --> Ledger

Ledger --> Projection
```

------------------------------------------------------------------------

# 7.5 Leave Adjustment Workflow

## Purpose

Provides controlled correction of leave balances without direct
modification.

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart TD

Request["Adjustment Request"]

Validate["Validate Authority"]

Review["Review Reason"]

Decision{"Approved?"}

Reverse["Create Reversal Transaction"]

Correction["Create Adjustment Transaction"]

Ledger["Update Leave Ledger"]

Projection["Refresh Balance Projection"]


Request --> Validate

Validate --> Review

Review --> Decision

Decision -->|Yes| Reverse

Reverse --> Correction

Correction --> Ledger

Ledger --> Projection

Decision -->|No| End["Rejected"]
```

------------------------------------------------------------------------

# 7.6 Leave Cancellation Workflow

## Purpose

Allows cancellation of previously approved leave while maintaining
ledger consistency.

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart TD

Cancel["Cancellation Request"]

Check["Check Cancellation Rules"]

Decision{"Allowed?"}

Release["Release Reserved Balance"]

Reverse["Create Reversal Transaction"]

Update["Update Request Status"]

Event["Publish Cancellation Event"]


Cancel --> Check

Check --> Decision

Decision -->|Yes| Release

Release --> Reverse

Reverse --> Update

Update --> Event

Decision -->|No| End["Cancellation Rejected"]
```

------------------------------------------------------------------------

# 7.7 Leave Encashment Eligibility Workflow

## Purpose

Determines whether an employee is eligible for leave encashment.

Financial calculation remains owned by Payroll.

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart LR

Request["Encashment Request"]

Policy["Evaluate Encashment Policy"]

Eligibility["Check Eligibility"]

Decision{"Eligible?"}

Provide["Provide Encashment Eligibility"]

Reject["Reject Request"]

Payroll["Payroll Calculation"]


Request --> Policy

Policy --> Eligibility

Eligibility --> Decision

Decision -->|Yes| Provide

Decision -->|No| Reject

Provide --> Payroll
```

------------------------------------------------------------------------

# 7.8 Policy Activation Workflow

## Purpose

Controls introduction of new leave policies.

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart TD

Draft["Policy Draft"]

Validation["Validate Rules"]

Conflict{"Conflict Exists?"}

Governance["Governance Resolution"]

Approval["Policy Approval"]

Activation["Activate Policy"]

Event["Publish Policy Activated Event"]


Draft --> Validation

Validation --> Conflict

Conflict -->|Yes| Governance

Governance --> Validation

Conflict -->|No| Approval

Approval --> Activation

Activation --> Event
```

------------------------------------------------------------------------

# 7.9 Workflow Interaction Model

``` mermaid
flowchart LR

Policy["Leave Policy"]

Request["Leave Request"]

Approval["Approval Workflow"]

Ledger["Leave Ledger"]

Balance["Balance Projection"]

Events["Domain Events"]


Policy --> Request

Request --> Approval

Approval --> Ledger

Ledger --> Balance

Balance --> Events
```

------------------------------------------------------------------------

# 7.10 Workflow Design Principles

## Domain Authority

All leave decisions are controlled by Leave Management.

------------------------------------------------------------------------

## Ledger-Based Changes

All balance changes require transactions.

------------------------------------------------------------------------

## Event Publication

Important business outcomes generate domain events.

------------------------------------------------------------------------

## Controlled Exceptions

Exceptional cases use workflows, not direct data changes.

------------------------------------------------------------------------

## Auditability

Every workflow transition must produce traceable history.




------------------------------------------------------------------------

# 8. Workflow Specifications

This section defines the operational specification of Leave Management
workflows.

Each workflow specification contains:

-   Workflow identity
-   Purpose
-   Trigger
-   Actors
-   Preconditions
-   Execution flow
-   Decision points
-   Alternate scenarios
-   Post conditions
-   Domain events
-   Audit requirements

------------------------------------------------------------------------

# WF-E05-001 --- Leave Request Submission Workflow

## Purpose

Allows employees to submit leave requests while enforcing policy,
eligibility, and validation rules.

## Trigger

Employee initiates a leave request.

## Actors

-   Employee
-   Leave Management Domain
-   Policy Engine

## Preconditions

-   Employee exists
-   Active leave policy exists
-   Leave type is available
-   Employee is eligible

## Main Flow

1.  Employee selects leave type.
2.  Employee provides leave period.
3.  System validates request format.
4.  Policy rules are evaluated.
5.  Leave balance availability is checked.
6.  Leave request aggregate is created.
7.  Approval workflow is initiated.

## Decision Points

  Decision               Outcome
  ---------------------- --------------------------------
  Employee eligible      Continue
  Policy violation       Reject or exception workflow
  Insufficient balance   Reject or controlled exception

## Post Conditions

-   Leave request created
-   Audit entry generated
-   Approval workflow started

## Domain Events

``` text
LeaveRequested
```

------------------------------------------------------------------------

# WF-E05-002 --- Leave Approval Workflow

## Purpose

Controls approval decisions according to organizational authority and
workflow rules.

## Trigger

Validated leave request enters approval stage.

## Actors

-   Approver
-   Workflow Engine
-   Leave Management Domain

## Preconditions

-   Request validated
-   Approval route available
-   Approver authorized

## Main Flow

1.  Determine approval chain.
2.  Notify approver.
3.  Approver reviews request.
4.  Decision is recorded.
5.  Leave request state is updated.

## Decision Points

  Decision   Outcome
  ---------- -----------------
  Approved   Reserve balance
  Rejected   Close request
  Timeout    Escalation

## Post Conditions

Request moves to approved or rejected state.

## Domain Events

``` text
LeaveApproved
LeaveRejected
```

------------------------------------------------------------------------

# WF-E05-003 --- Leave Accrual Processing Workflow

## Purpose

Generates leave entitlement through configured accrual rules.

## Trigger

Scheduled accrual event or policy-defined trigger.

## Actors

-   Leave Management Domain
-   Policy Engine

## Preconditions

-   Active accrual policy exists
-   Employee eligibility confirmed

## Main Flow

1.  Load active policy.
2.  Identify eligible employees.
3.  Calculate accrual amount.
4.  Create ledger transactions.
5.  Update balance projection.

## Post Conditions

-   Accrual transaction recorded
-   Balance projection updated

## Domain Events

``` text
LeaveAccrualCompleted
LeaveBalanceUpdated
```

------------------------------------------------------------------------

# WF-E05-004 --- Leave Consumption Workflow

## Purpose

Records approved leave usage.

## Trigger

Approved leave reaches consumption date.

## Actors

-   Leave Management Domain
-   Attendance Domain

## Preconditions

-   Leave approved
-   Balance reservation exists

## Main Flow

1.  Convert reservation into consumption.
2.  Create ledger transaction.
3.  Update balance projection.
4.  Publish consumption event.

## Domain Events

``` text
LeaveConsumed
LeaveBalanceUpdated
```

------------------------------------------------------------------------

# WF-E05-005 --- Leave Cancellation Workflow

## Purpose

Allows cancellation while preserving ledger integrity.

## Trigger

Employee or authorized user requests cancellation.

## Actors

-   Employee
-   Approver
-   Leave Management Domain

## Preconditions

-   Leave exists
-   Cancellation policy allows action

## Main Flow

1.  Validate cancellation request.
2.  Check policy restrictions.
3.  Reverse reservation or consumption.
4.  Create reversal ledger entry.
5.  Update request status.

## Domain Events

``` text
LeaveCancelled
LeaveBalanceUpdated
```

------------------------------------------------------------------------

# WF-E05-006 --- Leave Adjustment Workflow

## Purpose

Provides controlled correction of leave balances.

## Trigger

Authorized adjustment request.

## Actors

-   HR Authorized User
-   Leave Management Domain

## Preconditions

-   Adjustment authority exists
-   Reason provided
-   Audit context available

## Main Flow

1.  Submit adjustment request.
2.  Validate authority.
3.  Review adjustment reason.
4.  Create correction transactions.
5.  Update balance projection.

## Rules

Direct balance editing is prohibited.

All corrections require ledger transactions.

## Domain Events

``` text
LeaveAdjusted
LeaveBalanceUpdated
```

------------------------------------------------------------------------

# WF-E05-007 --- Carry Forward Processing Workflow

## Purpose

Transfers eligible unused leave into the next leave period.

## Trigger

Leave year closure.

## Actors

-   Leave Management Domain
-   Policy Engine

## Preconditions

-   Leave year closing
-   Carry-forward policy active

## Main Flow

1.  Evaluate carry-forward rules.
2.  Calculate eligible amount.
3.  Create carry-forward transaction.
4.  Update balance projection.

## Decision Points

  Condition        Action
  ---------------- --------------------
  Eligible         Carry forward
  Limit exceeded   Apply policy limit
  Not eligible     Expire balance

## Domain Events

``` text
LeaveCarryForwardCompleted
```

------------------------------------------------------------------------

# WF-E05-008 --- Leave Encashment Eligibility Workflow

## Purpose

Determines whether leave can be considered for encashment.

## Trigger

Encashment request or payroll evaluation.

## Actors

-   Employee
-   HR
-   Leave Management Domain
-   Payroll Domain

## Preconditions

-   Encashment policy exists
-   Employee eligible

## Main Flow

1.  Evaluate policy.
2.  Check available leave.
3.  Determine eligible quantity.
4.  Send eligibility information to Payroll.

## Boundary Rule

Leave Management:

``` text
Determines eligibility
```

Payroll:

``` text
Calculates monetary value
```

## Domain Events

``` text
LeaveEncashmentEligible
```

------------------------------------------------------------------------

# WF-E05-009 --- Leave Policy Activation Workflow

## Purpose

Controls activation of new leave policies.

## Trigger

New policy version submitted.

## Actors

-   HR Governance
-   Leave Management Domain

## Preconditions

-   Policy draft exists
-   Validation completed

## Main Flow

1.  Validate policy rules.
2.  Detect conflicts.
3.  Obtain approval.
4.  Activate policy version.
5.  Publish policy event.

## Decision Points

  Condition         Action
  ----------------- ------------------
  Conflict exists   Block activation
  Approved          Activate
  Rejected          Return to draft

## Domain Events

``` text
LeavePolicyActivated
```

------------------------------------------------------------------------

# WF-E05-010 --- Exception Handling Workflow

## Purpose

Provides controlled handling of exceptional leave scenarios.

## Examples

-   Negative balance approval
-   Emergency HR override
-   Policy exception
-   Manual correction

## Rules

Exceptions require:

-   Authorized role
-   Business reason
-   Approval where required
-   Complete audit trail

## Domain Events

``` text
LeaveExceptionApproved
LeaveExceptionRejected
```

------------------------------------------------------------------------

# Workflow Specification Principles

## Principle 1 --- Domain Ownership

Leave decisions remain inside E05.

## Principle 2 --- Ledger Integrity

All balance changes require transactions.

## Principle 3 --- Controlled Overrides

Exceptional operations never bypass governance.

## Principle 4 --- Event Publication

Important workflow outcomes generate domain events.

## Principle 5 --- Auditability

Every workflow transition must be traceable.



------------------------------------------------------------------------

# 9. Business Rules

Business rules define the decision logic that governs Leave Management
behavior.

The Leave Management domain follows the principle:

> Policies are configurable, but domain invariants are protected.

Business rules are categorized into:

1.  Eligibility Rules
2.  Entitlement Rules
3.  Accrual Rules
4.  Balance Rules
5.  Approval Rules
6.  Carry Forward Rules
7.  Adjustment Rules
8.  Exception Rules
9.  Policy Governance Rules

------------------------------------------------------------------------

# 9.1 Eligibility Rules

## BR-E05-001 --- Employee Eligibility Evaluation

An employee must satisfy applicable leave policy conditions before
requesting a leave type.

Eligibility may depend on:

-   Employment category
-   Joining date
-   Service duration
-   Employee status
-   Institution policy
-   Leave type restrictions

Example:

``` text
Probation Employee

↓

Earned Leave Request

↓

Eligibility Evaluation

↓

Allowed / Rejected / Exception Workflow
```

------------------------------------------------------------------------

## BR-E05-002 --- Policy Hierarchy Evaluation

Leave policies are evaluated according to configured priority.

Recommended hierarchy:

``` text
Enterprise Default Policy

↓

Institution Policy

↓

Campus / Department Policy

↓

Employee Category Policy

↓

Employee Specific Override
```

Conflicting policies must not silently override each other.

------------------------------------------------------------------------

# 9.2 Entitlement Rules

## BR-E05-003 --- Entitlement Generation

Leave entitlement must be generated according to active policy rules.

Inputs:

-   Employee category
-   Effective date
-   Leave type
-   Policy version

Output:

-   Entitlement transaction in Leave Ledger

------------------------------------------------------------------------

## BR-E05-004 --- Effective Dating

Policy changes affect only applicable future periods.

Historical transactions remain unchanged.

Example:

``` text
Policy Version 1

Jan-Jun

↓

Policy Version 2

Jul-Dec
```

------------------------------------------------------------------------

# 9.3 Accrual Rules

## BR-E05-005 --- Accrual Through Ledger Transactions

Accrual must never directly modify balance.

Required flow:

``` text
Accrual Calculation

↓

Accrual Transaction

↓

Ledger Update

↓

Balance Projection
```

------------------------------------------------------------------------

## BR-E05-006 --- Accrual Frequency

Supported accrual models:

-   Monthly
-   Quarterly
-   Yearly
-   Event-driven
-   Custom institutional schedules

------------------------------------------------------------------------

# 9.4 Balance Rules

## BR-E05-007 --- Ledger Is Source of Truth

Leave balance is derived from transactions.

No direct update:

``` text
UPDATE LeaveBalance
```

Allowed:

``` text
Create Ledger Transaction

↓

Recalculate Projection
```

------------------------------------------------------------------------

## BR-E05-008 --- Balance Reservation

Approved leave reserves available balance before consumption.

Lifecycle:

``` text
Available Balance

↓

Reserved Balance

↓

Consumed Balance
```

------------------------------------------------------------------------

## BR-E05-009 --- Negative Balance Handling

Negative balance behavior is policy-controlled.

Possible outcomes:

-   Reject request
-   Allow with approval
-   Route exception workflow

------------------------------------------------------------------------

# 9.5 Approval Rules

## BR-E05-010 --- Approval Authority

Approval authority comes from:

-   Organization hierarchy
-   Workflow configuration
-   Authorization model

Leave Management validates authority but does not own organizational
hierarchy.

------------------------------------------------------------------------

## BR-E05-011 --- Approval Timeout

Approval timeout triggers escalation.

Example:

``` text
Manager

↓

Department Head

↓

HR
```

------------------------------------------------------------------------

# 9.6 Carry Forward Rules

## BR-E05-012 --- Carry Forward Evaluation

Carry forward depends on:

-   Leave type
-   Policy configuration
-   Maximum carry-forward limit
-   Expiry rules

------------------------------------------------------------------------

## BR-E05-013 --- Carry Forward Transaction

Carry forward creates a ledger transaction.

It does not manually increase balance.

------------------------------------------------------------------------

# 9.7 Adjustment Rules

## BR-E05-014 --- Controlled Adjustment

Leave adjustments require:

-   Authorized user
-   Reason
-   Approval where applicable
-   Audit record

------------------------------------------------------------------------

## BR-E05-015 --- Correction Through Reversal

Incorrect transactions are corrected through:

``` text
Original Transaction

↓

Reversal Transaction

↓

Correct Transaction
```

------------------------------------------------------------------------

# 9.8 Exception Rules

## BR-E05-016 --- Controlled Override

Overrides may exist for:

-   Emergency leave
-   Exceptional approval
-   Policy exceptions

Every override requires:

-   Authorization
-   Reason
-   Audit trail

------------------------------------------------------------------------

# 9.9 Policy Governance Rules

## BR-E05-017 --- Policy Conflict Prevention

Conflicting policies must block activation.

Resolution requires governance approval.

------------------------------------------------------------------------

# 10. Domain Events

Domain events represent meaningful business outcomes produced by Leave
Management.

Events communicate facts.

They do not request actions.

------------------------------------------------------------------------

# 10.1 Event Ownership Model

``` mermaid
flowchart LR

E05["Leave Management Domain"]

Events["Leave Domain Events"]

Consumers["External Consumers"]

E05 --> Events

Events --> Consumers
```

------------------------------------------------------------------------

# 10.2 Core Domain Events

## LeaveRequested

### Purpose

Published when a valid leave request is submitted.

Producer:

``` text
Leave Request Aggregate
```

Consumers:

-   Notification Domain
-   Analytics Domain

------------------------------------------------------------------------

## LeaveApproved

### Purpose

Published after successful approval.

Producer:

``` text
Leave Request Aggregate
```

Consumers:

-   Attendance Domain
-   Notification Domain
-   Payroll Domain

------------------------------------------------------------------------

## LeaveRejected

### Purpose

Published when approval is denied.

Consumers:

-   Notification
-   Analytics

------------------------------------------------------------------------

## LeaveCancelled

### Purpose

Published when approved leave is cancelled.

Consumers:

-   Attendance
-   Notification
-   Analytics

------------------------------------------------------------------------

## LeaveBalanceUpdated

### Purpose

Indicates balance projection changed.

Producer:

``` text
Leave Ledger Aggregate
```

Consumers:

-   Reporting
-   Analytics

------------------------------------------------------------------------

## LeaveAccrualCompleted

### Purpose

Published after successful accrual processing.

Consumers:

-   Analytics
-   Reporting

------------------------------------------------------------------------

## LeavePolicyActivated

### Purpose

Published after policy activation.

Consumers:

-   Configuration consumers
-   Analytics

------------------------------------------------------------------------

# 10.3 Event Payload Principles

Events should contain:

-   Business identifiers
-   Required business facts
-   State transition information

Events should not contain:

-   Full employee profile
-   Salary information
-   Unrelated documents

Example:

``` text
LeaveApproved Event

Contains:

LeaveId

EmployeeId

LeaveType

StartDate

EndDate

ApprovalStatus
```

------------------------------------------------------------------------

# 11. Service Contracts

Service contracts define how E05 communicates with users and other
domains.

Communication model:

``` mermaid
flowchart LR

Commands["Commands"]

Queries["Queries"]

Domain["Leave Management Domain"]

Events["Domain Events"]

Commands --> Domain

Domain --> Events

Queries --> Domain
```

------------------------------------------------------------------------

# 11.1 Command Contracts

## SubmitLeaveRequest

Purpose:

Creates a new leave request.

Input:

``` text
EmployeeId

LeaveType

StartDate

EndDate

Reason
```

Output:

``` text
LeaveRequestId

CurrentStatus
```

------------------------------------------------------------------------

## ApproveLeaveRequest

Purpose:

Approves a pending request.

Input:

``` text
LeaveRequestId

ApproverId

DecisionReason
```

------------------------------------------------------------------------

## RejectLeaveRequest

Purpose:

Rejects a request.

------------------------------------------------------------------------

## CancelLeaveRequest

Purpose:

Cancels existing leave.

------------------------------------------------------------------------

## CreateLeaveAdjustment

Purpose:

Creates controlled balance correction.

Required:

-   Reason
-   Authority
-   Audit context

------------------------------------------------------------------------

# 11.2 Query Contracts

## GetLeaveBalance

Purpose:

Returns employee leave availability.

Input:

``` text
EmployeeId

LeaveType
```

------------------------------------------------------------------------

## GetLeaveHistory

Purpose:

Returns historical leave transactions.

------------------------------------------------------------------------

## GetLeaveEligibility

Purpose:

Evaluates employee eligibility.

------------------------------------------------------------------------

## GetLeavePolicy

Purpose:

Returns active applicable policy.

------------------------------------------------------------------------

# 11.3 Integration Contracts

## Attendance Integration

Consumes:

``` text
LeaveApproved

LeaveCancelled
```

Provides:

``` text
Attendance Context
CompOff Eligibility
```

------------------------------------------------------------------------

## Payroll Integration

Consumes:

``` text
LeaveApproved

LeaveEncashmentEligible
```

Provides:

``` text
Payroll Impact Context
```

------------------------------------------------------------------------

# 11.4 Contract Versioning

All external contracts must support versioning.

Example:

``` text
LeaveApproved.v1

LeaveApproved.v2
```

Breaking changes require new versions.

------------------------------------------------------------------------

# 11.5 Service Contract Principles

## Principle 1

External systems use business commands.

Not:

``` text
Direct database updates
```

------------------------------------------------------------------------

## Principle 2

Queries provide read access.

They do not modify state.

------------------------------------------------------------------------

## Principle 3

Events communicate completed business facts.

------------------------------------------------------------------------

## Principle 4

E05 remains the authority for leave decisions.




------------------------------------------------------------------------

# 12. State Transition Matrix

State transitions define the controlled movement of Leave Management
entities through their lifecycle.

The principle:

> No state transition is valid unless it is allowed by domain rules,
> authorization rules, and workflow rules.

------------------------------------------------------------------------

# 12.1 Leave Request State Machine

``` mermaid
stateDiagram-v2

[*] --> Draft

Draft --> Submitted : Submit Request

Submitted --> Validated : Validation Passed

Submitted --> Draft : Correction Required

Validated --> PendingApproval : Approval Initiated

PendingApproval --> Approved : Approve

PendingApproval --> Rejected : Reject

PendingApproval --> PendingApproval : Escalation

Approved --> Reserved : Reserve Balance

Reserved --> Consumed : Leave Period Started

Consumed --> Closed : Completion

Approved --> Cancelled : Cancellation

Reserved --> Cancelled : Cancellation

Rejected --> Closed
```

------------------------------------------------------------------------

# 12.2 Leave Request Transition Matrix

  --------------------------------------------------------------------------
  Current State  Action         Next State     Authority      Validation
  -------------- -------------- -------------- -------------- --------------
  Draft          Submit Request Submitted      Employee       Mandatory
                                                              fields

  Submitted      Validate       Validated      Leave Domain   Policy +
                                                              eligibility

  Submitted      Correction     Draft          Employee       Validation
                 Required                                     failure

  Validated      Start Approval Pending        Workflow       Approval route
                                Approval       Engine         exists

  Pending        Approve        Approved       Authorized     Authority
  Approval                                     Approver       check

  Pending        Reject         Rejected       Authorized     Decision
  Approval                                     Approver       reason

  Approved       Reserve        Reserved       Leave Domain   Balance
                 Balance                                      available

  Reserved       Consume Leave  Consumed       Leave Domain   Leave period
                                                              reached

  Consumed       Close Request  Closed         Leave Domain   Completion
                                                              confirmed

  Approved       Cancel         Cancelled      Authorized     Cancellation
                                               User           policy
  --------------------------------------------------------------------------

------------------------------------------------------------------------

# 12.3 Leave Policy State Machine

``` mermaid
stateDiagram-v2

[*] --> Draft

Draft --> UnderReview

UnderReview --> Approved

UnderReview --> Draft

Approved --> Scheduled

Scheduled --> Active

Active --> Superseded

Superseded --> Archived
```

------------------------------------------------------------------------

# 12.4 Leave Policy Transition Matrix

  Current State   Action            Next State    Authority
  --------------- ----------------- ------------- ----------------------
  Draft           Submit Review     UnderReview   Policy Owner
  UnderReview     Approve           Approved      Governance Authority
  Approved        Schedule          Scheduled     HR Administrator
  Scheduled       Activate          Active        System Scheduler
  Active          Replace Version   Superseded    Governance Authority
  Superseded      Archive           Archived      System Governance

------------------------------------------------------------------------

# 12.5 Leave Balance Lifecycle

Balance is a projection and not independently mutated.

``` mermaid
stateDiagram-v2

[*] --> Available

Available --> Reserved

Reserved --> Consumed

Available --> Adjusted

Consumed --> Reconciled

Adjusted --> Reconciled
```

------------------------------------------------------------------------

# 13. Decision Matrix

Decision ownership prevents domain responsibility conflicts.

------------------------------------------------------------------------

# 13.1 Leave Request Decision Matrix

  -----------------------------------------------------------------------
  Decision                Owner                   Supporting Domain
  ----------------------- ----------------------- -----------------------
  Leave type availability Leave Management        Policy Engine

  Employee eligibility    Leave Management        Employee Profile

  Approval routing        Workflow Engine         Organization Domain

  Final approval decision Authorized Approver     Authorization Platform

  Balance reservation     Leave Management        Leave Ledger

  Salary impact           Payroll                 Leave Management
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 13.2 Policy Decision Matrix

  Scenario                      Decision Owner         Action
  ----------------------------- ---------------------- ------------------
  Policy conflict detected      Policy Governance      Block activation
  Eligibility rule evaluation   Policy Engine          Approve / Reject
  Policy version activation     Governance Authority   Activate
  Policy expiry                 System Scheduler       Supersede

------------------------------------------------------------------------

# 13.3 Balance Decision Matrix

  Scenario                     Decision Owner           Result
  ---------------------------- ------------------------ ------------------------
  Sufficient balance           Leave Domain             Continue
  Insufficient balance         Policy Engine            Reject / Exception
  Manual correction required   Authorized HR            Adjustment Workflow
  Ledger mismatch              Reconciliation Process   Corrective transaction

------------------------------------------------------------------------

# 13.4 Cross-Domain Decision Matrix

  Scenario                    Owner
  --------------------------- ---------------------
  Employee identity           Employee Profile
  Reporting hierarchy         Organization Domain
  Attendance interpretation   Attendance Domain
  Salary calculation          Payroll Domain
  Notification delivery       Notification Domain
  Document storage            Document Platform

------------------------------------------------------------------------

# 14. Failure Matrix

Failure handling follows:

``` mermaid
flowchart LR

Failure["Failure Detection"]

Recovery["Controlled Recovery"]

Audit["Audit Record"]

Reconcile["Reconciliation"]

Failure --> Recovery

Recovery --> Audit

Audit --> Reconcile
```

------------------------------------------------------------------------

# 14.1 Workflow Failure Matrix

  -----------------------------------------------------------------------
  Failure           Detection         Recovery          Owner
  ----------------- ----------------- ----------------- -----------------
  Invalid state     Domain validation Reject transition Leave Domain
  transition                                            

  Approval timeout  Workflow monitor  Escalation        Workflow Engine

  Policy conflict   Rule validation   Block activation  Governance

  Duplicate request Duplicate         Reject request    Leave Domain
                    detection                           

  Unauthorized      Authorization     Deny + Audit      Security Platform
  action            check                               
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 14.2 Integration Failure Matrix

  -----------------------------------------------------------------------
  Failure           Detection         Recovery          Owner
  ----------------- ----------------- ----------------- -----------------
  Payroll event     Event monitoring  Retry +           Integration Layer
  failure                             reconciliation    

  Attendance event  Queue monitoring  Retry + dead      Integration Layer
  failure                             letter            

  Notification      Delivery          Retry delivery    Notification
  failure           monitoring                          Domain

  Document          Reference         Manual            Document Platform
  reference failure validation        reconciliation    
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 14.3 Data Integrity Failure Matrix

  -----------------------------------------------------------------------
  Failure                 Detection               Recovery
  ----------------------- ----------------------- -----------------------
  Incorrect migrated      Reconciliation check    Ledger correction
  balance                                         

  Duplicate ledger entry  Transaction validation  Reverse transaction

  Projection mismatch     Balance reconciliation  Rebuild projection

  Incorrect policy        Policy validation       Governance correction
  configuration                                   
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 15. Validation Rules

Validation ensures that Leave Management decisions remain consistent and
compliant.

------------------------------------------------------------------------

# 15.1 Request Validation Rules

  -----------------------------------------------------------------------
  Rule ID                 Rule                    Description
  ----------------------- ----------------------- -----------------------
  VAL-E05-001             Date Validation         Leave dates must be
                                                  valid

  VAL-E05-002             Duplicate Detection     Duplicate requests must
                                                  be prevented

  VAL-E05-003             Leave Type Validation   Leave type must be
                                                  active

  VAL-E05-004             Mandatory Data          Required request
                                                  information must exist
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 15.2 Eligibility Validation Rules

  Rule ID       Rule
  ------------- ------------------------------------------
  VAL-E05-005   Employee category must match policy
  VAL-E05-006   Service duration rules must pass
  VAL-E05-007   Policy version must be active
  VAL-E05-008   Leave type eligibility must be satisfied

------------------------------------------------------------------------

# 15.3 Balance Validation Rules

  Rule ID       Rule
  ------------- -----------------------------------------
  VAL-E05-009   Balance availability must be checked
  VAL-E05-010   Reservation conflicts must be prevented
  VAL-E05-011   Ledger must remain consistent
  VAL-E05-012   Direct balance mutation is prohibited

------------------------------------------------------------------------

# 15.4 Authorization Validation Rules

  Rule ID       Rule
  ------------- --------------------------------------------
  VAL-E05-013   User role must permit action
  VAL-E05-014   Organization scope must be valid
  VAL-E05-015   Sensitive leave access requires permission
  VAL-E05-016   Approval authority must be verified

------------------------------------------------------------------------

# 15.5 Policy Validation Rules

  Rule ID       Rule
  ------------- ----------------------------------------------
  VAL-E05-017   Policy version must be valid
  VAL-E05-018   Conflicting policies must block activation
  VAL-E05-019   Effective dates must not overlap incorrectly
  VAL-E05-020   Policy changes require audit history

------------------------------------------------------------------------

# 15.6 Validation Architecture Model

``` mermaid
flowchart TD

Request["Incoming Request"]

API["API Validation"]

Workflow["Workflow Validation"]

Domain["Domain Validation"]

Authorization["Authorization Validation"]

Decision["Approved Domain Decision"]


Request --> API

API --> Workflow

Workflow --> Authorization

Authorization --> Domain

Domain --> Decision
```

------------------------------------------------------------------------

# Final Design Principles

## State Integrity

Only valid transitions are allowed.

## Decision Ownership

Each decision belongs to one authoritative domain.

## Failure Recovery

Failures recover through controlled workflows.

## Validation First

Invalid states never enter the domain.

## Audit Everything Important

All significant state changes remain traceable.


# 16. Authorization Model

Authorization in Leave Management ensures that every leave-related
action is performed by an authorized actor within the correct
organizational and security context.

The authorization model follows:

``` text
RBAC + ABAC + Organizational Scope + Business Context
```

The principle:

> Authentication identifies the user. Authorization determines whether
> the user can perform a specific leave operation.

------------------------------------------------------------------------

# 16.1 Authorization Architecture Model

``` mermaid
flowchart TD

User["User"]

Identity["Identity Provider"]

RBAC["Role Based Access Control"]

ABAC["Attribute Based Access Control"]

Scope["Organization Scope"]

Context["Business Context"]

Decision["Authorization Decision"]

Leave["Leave Management Domain"]


User --> Identity

Identity --> RBAC

RBAC --> ABAC

ABAC --> Scope

Scope --> Context

Context --> Decision

Decision --> Leave
```

------------------------------------------------------------------------

# 16.2 Authorization Dimensions

Authorization decisions are evaluated using multiple dimensions.

  -----------------------------------------------------------------------
  Dimension                           Purpose
  ----------------------------------- -----------------------------------
  User Identity                       Identifies the requesting user

  Role                                Defines permitted capabilities

  Organization Scope                  Restricts access by institution,
                                      campus, department

  Employee Relationship               Determines allowed employee
                                      population

  Leave Sensitivity                   Protects confidential leave
                                      categories

  Workflow State                      Controls allowed actions

  Business Context                    Applies policy-specific
                                      restrictions
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 16.3 Role Based Access Control (RBAC)

RBAC defines what actions a role can perform.

------------------------------------------------------------------------

## Employee Role

Permissions:

  Capability             Permission
  ---------------------- -------------------
  Create Leave Request   Allowed
  View Own Balance       Allowed
  View Own History       Allowed
  Cancel Own Request     Policy Controlled
  Approve Requests       Not Allowed

------------------------------------------------------------------------

## Manager / Approver Role

Permissions:

  Capability                 Permission
  -------------------------- -------------
  View Team Leave Requests   Allowed
  Approve Requests           Allowed
  Reject Requests            Allowed
  Modify Policy              Not Allowed
  Adjust Balance             Not Allowed

------------------------------------------------------------------------

## HR Administrator Role

Permissions:

  Capability                  Permission
  --------------------------- ------------
  Configure Leave Policies    Allowed
  Process Adjustments         Allowed
  Manage Exceptions           Allowed
  View Sensitive Leave Data   Controlled

------------------------------------------------------------------------

## System Administrator Role

Permissions:

  Capability                 Permission
  -------------------------- -------------
  Technical Configuration    Allowed
  Business Decisions         Not Allowed
  Direct Data Modification   Prohibited

------------------------------------------------------------------------

# 16.4 Attribute Based Access Control (ABAC)

RBAC alone is insufficient for enterprise ERP environments.

ABAC evaluates:

``` text
Who

+

What

+

Where

+

Why

+

Current State
```

------------------------------------------------------------------------

Examples:

## Example 1 --- Manager Approval

Condition:

``` text
User Role = Manager

AND

Employee belongs to reporting hierarchy

AND

Request Status = Pending Approval
```

Result:

Approved

------------------------------------------------------------------------

## Example 2 --- Sensitive Leave Access

Condition:

``` text
Role = HR

AND

Sensitive Leave Permission = Granted
```

Result:

Access allowed

------------------------------------------------------------------------

# 16.5 Organization Scope Security

SutraERP supports institution-level isolation.

Authorization must validate:

-   Institution
-   Campus
-   Department
-   Reporting unit
-   Workforce category

Example:

``` mermaid
flowchart TD

User["HR User"]

Institution["Institution Scope"]

Campus["Campus Scope"]

Department["Department Scope"]

Leave["Leave Data"]


User --> Institution

Institution --> Campus

Campus --> Department

Department --> Leave
```

------------------------------------------------------------------------

# 16.6 Sensitive Leave Authorization

Certain leave categories may contain sensitive information.

Examples:

-   Medical leave
-   Disability-related leave
-   Special protected leave categories

Controls:

  Control                 Purpose
  ----------------------- -----------------------------
  Restricted Visibility   Limit access
  Purpose-Based Access    Prevent unnecessary viewing
  Audit Logging           Track access
  Document Protection     Protect evidence

------------------------------------------------------------------------

# 16.7 Authorization Rules

  -----------------------------------------------------------------------
  Rule ID                             Rule
  ----------------------------------- -----------------------------------
  AUTH-E05-001                        Users can access only authorized
                                      organizational scope

  AUTH-E05-002                        Employees can modify only permitted
                                      self-service actions

  AUTH-E05-003                        Approvers cannot approve their own
                                      requests unless policy permits

  AUTH-E05-004                        Balance adjustments require
                                      elevated authority

  AUTH-E05-005                        Sensitive leave access requires
                                      explicit permission
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 17. Audit Requirements

Auditability is a core requirement because leave decisions may impact:

-   Employee rights
-   Attendance records
-   Payroll outcomes
-   Compliance reporting

------------------------------------------------------------------------

# 17.1 Audit Architecture

``` mermaid
flowchart LR

Action["Business Action"]

Leave["Leave Management"]

Audit["Audit Record"]

Storage["Audit Repository"]

Action --> Leave

Leave --> Audit

Audit --> Storage
```

------------------------------------------------------------------------

# 17.2 Auditable Events

The following actions require audit records:

  Action                   Audit Required
  ------------------------ ----------------
  Leave Request Creation   Yes
  Approval Decision        Yes
  Rejection Decision       Yes
  Cancellation             Yes
  Balance Adjustment       Yes
  Policy Change            Yes
  Permission Change        Yes
  Sensitive Data Access    Yes

------------------------------------------------------------------------

# 17.3 Audit Record Contents

Every audit record should contain:

``` text
Audit ID

Timestamp

User ID

Action

Entity ID

Previous State

New State

Reason

Organization Context

Correlation ID
```

------------------------------------------------------------------------

# 17.4 Business Audit vs Security Audit

## Business Audit

Tracks:

-   Leave decisions
-   Policy changes
-   Adjustments
-   Exceptions

Owner:

Leave Management Domain

------------------------------------------------------------------------

## Security Audit

Tracks:

-   Login
-   Permission changes
-   Unauthorized access attempts

Owner:

Security Platform

------------------------------------------------------------------------

# 17.5 Adjustment Audit Requirement

Leave adjustment is a high-risk operation.

Required:

``` text
Adjustment Request

↓

Authorization Validation

↓

Reason Capture

↓

Approval

↓

Ledger Transaction

↓

Audit Record
```

Direct modification is prohibited.

------------------------------------------------------------------------

# 18. Security Considerations

------------------------------------------------------------------------

# 18.1 Data Protection

Leave data may contain sensitive employee information.

Protection measures:

-   Encryption at rest
-   Encryption in transit
-   Access restrictions
-   Data classification
-   Secure document references

------------------------------------------------------------------------

# 18.2 Data Minimization

E05 should store only required information.

Example:

Allowed:

``` text
Medical Document Reference
```

Not recommended:

``` text
Full Medical History
```

------------------------------------------------------------------------

# 18.3 Document Security

Supporting documents are owned by the Document Platform.

Leave Management stores:

-   Document reference
-   Verification status
-   Relationship metadata

It does not own document storage.

------------------------------------------------------------------------

# 18.4 Event Security

Domain events must avoid leaking sensitive information.

Example:

Good:

``` text
LeaveApproved

EmployeeId

LeaveType

Dates

Status
```

Avoid:

``` text
Medical Details

Personal Notes

Confidential Attachments
```

------------------------------------------------------------------------

# 18.5 API Security

Service contracts require:

-   Authentication
-   Authorization validation
-   Input validation
-   Rate limiting
-   Request tracing

------------------------------------------------------------------------

# 18.6 Security Threat Model

``` mermaid
flowchart TD

Threat["Security Threat"]

Unauthorized["Unauthorized Access"]

Privilege["Privilege Escalation"]

Data["Data Exposure"]

Mitigation["Security Controls"]

Threat --> Unauthorized

Threat --> Privilege

Threat --> Data

Unauthorized --> Mitigation

Privilege --> Mitigation

Data --> Mitigation
```

------------------------------------------------------------------------

# 18.7 Security Rules

  -----------------------------------------------------------------------
  Rule ID                             Rule
  ----------------------------------- -----------------------------------
  SEC-E05-001                         Sensitive leave data requires
                                      restricted access

  SEC-E05-002                         Authorization must be checked
                                      before every protected action

  SEC-E05-003                         Events must not expose confidential
                                      information

  SEC-E05-004                         Audit records must be immutable

  SEC-E05-005                         Direct database modification is
                                      prohibited
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Final Authorization and Security Principles

## Principle 1

No user bypasses domain authorization.

## Principle 2

Roles provide capability; attributes provide context.

## Principle 3

Sensitive leave information follows least privilege access.

## Principle 4

Every important business action is traceable.

## Principle 5

Security controls protect the domain without changing ownership
boundaries.


# 19. Performance & Scalability

The Leave Management domain must support enterprise-scale operations
across multiple institutions, campuses, and large employee populations.

Performance design follows:

``` text
High Transaction Integrity

+

Efficient Read Performance

+

Asynchronous Processing

+

Horizontal Scalability

+

Tenant Isolation
```

The primary design principle:

> Optimize read-heavy operations without compromising transactional
> correctness.

------------------------------------------------------------------------

# 19.1 Performance Architecture Overview

``` mermaid
flowchart LR

Users["Users"]

API["Application APIs"]

Domain["Leave Management Domain"]

Ledger["Leave Ledger"]

Projection["Read Projections"]

Cache["Configuration Cache"]

Queue["Processing Queue"]

Analytics["Analytics Store"]


Users --> API

API --> Domain

Domain --> Ledger

Ledger --> Projection

Domain --> Queue

Domain --> Cache

Projection --> Analytics
```

------------------------------------------------------------------------

# 19.2 Transaction Processing Strategy

Leave transactions require strong consistency.

Transactional operations include:

-   Leave request creation
-   Approval decision
-   Balance reservation
-   Ledger transaction creation
-   Policy activation

These operations follow:

``` text
Command

↓

Domain Validation

↓

Aggregate Update

↓

Ledger Transaction

↓

Event Publication
```

------------------------------------------------------------------------

# 19.3 Read Optimization Strategy

Leave Management has many read-heavy operations:

Examples:

-   Employee checking balance
-   Manager viewing team leave
-   HR reporting
-   Policy lookup

Therefore read optimization uses:

## Read Models

Examples:

``` text
Employee Leave Balance View

Team Leave Calendar View

Leave History View

Policy Eligibility View
```

------------------------------------------------------------------------

## Projection Model

``` mermaid
flowchart LR

Ledger["Leave Ledger"]

Events["Domain Events"]

Projection["Read Projection"]

Queries["Query APIs"]


Ledger --> Events

Events --> Projection

Projection --> Queries
```

------------------------------------------------------------------------

# 19.4 Caching Strategy

Caching is suitable for stable configuration data.

Recommended cache candidates:

  Data                    Cache Strategy
  ----------------------- ----------------
  Active Leave Policies   Cached
  Leave Types             Cached
  Approval Rules          Cached
  Employee Balance        Limited Cache
  Transaction History     Not Cached

------------------------------------------------------------------------

# 19.5 Asynchronous Processing

The following processes should execute asynchronously:

-   Bulk entitlement generation
-   Monthly accrual processing
-   Leave year closure
-   Carry-forward processing
-   Large report generation
-   Migration reconciliation

------------------------------------------------------------------------

# 19.6 Queue-Based Processing Model

``` mermaid
flowchart LR

Request["Large Processing Request"]

Queue["Processing Queue"]

Worker["Background Worker"]

Ledger["Ledger Processing"]

Result["Completion Status"]


Request --> Queue

Queue --> Worker

Worker --> Ledger

Ledger --> Result
```

------------------------------------------------------------------------

# 19.7 Bulk Processing Strategy

Example:

``` text
Institution

↓

50,000 Employees

↓

Monthly Accrual Processing
```

Processing approach:

1.  Create batch job.
2.  Divide employees into chunks.
3.  Process independently.
4.  Track failures.
5.  Retry failed chunks.
6.  Generate completion report.

------------------------------------------------------------------------

# 19.8 Failure-Aware Scalability

Scaling must not compromise correctness.

Required controls:

-   Idempotent processing
-   Retry mechanisms
-   Dead letter queues
-   Reconciliation jobs
-   Processing checkpoints

------------------------------------------------------------------------

# 19.9 Multi-Tenant Scalability

SutraERP supports institution-based deployment models.

Supported approaches:

``` text
Shared Infrastructure

+

Tenant Isolation

+

Configurable Scaling
```

------------------------------------------------------------------------

# 19.10 Tenant Isolation Model

``` mermaid
flowchart TD

Platform["SutraERP Platform"]

TenantA["Institution A"]

TenantB["Institution B"]

TenantC["Institution C"]

Config["Tenant Configuration"]

Platform --> TenantA

Platform --> TenantB

Platform --> TenantC

TenantA --> Config

TenantB --> Config

TenantC --> Config
```

------------------------------------------------------------------------

# 19.11 Monitoring Requirements

Performance monitoring should track:

  Metric                          Purpose
  ------------------------------- ----------------------
  Leave Request Processing Time   Workflow health
  Approval Processing Time        Bottleneck detection
  Accrual Job Duration            Batch efficiency
  Queue Failure Rate              Reliability
  Projection Lag                  Data freshness
  API Response Time               User experience

------------------------------------------------------------------------

# 19.12 Scalability Principles

## Principle 1

Transactional truth remains in domain aggregates.

## Principle 2

Reporting uses projections.

## Principle 3

Heavy processing uses asynchronous execution.

## Principle 4

Scaling must preserve tenant isolation.

## Principle 5

Performance optimization cannot bypass business rules.

------------------------------------------------------------------------

# 20. Extension Points

Leave Management is designed for future evolution.

The extension philosophy:

``` text
Stable Core

+

Configurable Policies

+

Extension Modules

+

Event-Based Integration
```

------------------------------------------------------------------------

# 20.1 Extension Categories

Future extensions may include:

-   New leave types
-   New workforce categories
-   Regional leave regulations
-   Shift-based absence management
-   Remote work absence
-   Advanced approval models

------------------------------------------------------------------------

# 20.2 Leave Type Extension

New leave types should primarily be configuration-driven.

Example:

``` text
Create Leave Type

↓

Configure Policy

↓

Define Eligibility

↓

Activate
```

Development changes should only be required when new behavior is
introduced.

------------------------------------------------------------------------

# 20.3 Workforce Category Extension

Future categories:

-   Visiting faculty
-   Contract employees
-   Researchers
-   Temporary workforce

Supported through:

``` text
Workforce Category Configuration

+

Policy Rules
```

------------------------------------------------------------------------

# 20.4 Institution Customization Model

Institutions may require:

-   Additional approvals
-   Custom leave rules
-   Different carry-forward limits

Preferred approach:

``` text
Configuration First

↓

Extension Module When Required

↓

Core Modification Only As Last Resort
```

------------------------------------------------------------------------

# 20.5 Workflow Extension

Future workflows should extend through:

-   Workflow configuration
-   New workflow handlers
-   Domain events

Example:

``` mermaid
flowchart LR

Core["Leave Core Workflow"]

Extension["Extension Workflow"]

Event["Domain Event"]


Core --> Event

Event --> Extension
```

------------------------------------------------------------------------

# 20.6 Integration Extension

Future integrations should use contracts.

Examples:

-   External HR systems
-   Government compliance systems
-   Workforce analytics platforms

Approach:

``` text
New Integration

↓

Event Subscription

↓

Independent Adapter
```

------------------------------------------------------------------------

# 20.7 Policy Engine Extension

The policy engine should support:

-   New rules
-   New eligibility conditions
-   New calculation strategies

Without modifying:

-   Leave Request Aggregate
-   Leave Ledger Aggregate

------------------------------------------------------------------------

# 20.8 Extension Boundaries

## Allowed Extension

-   New policies
-   New rules
-   New workflows
-   New integrations

------------------------------------------------------------------------

## Restricted Extension

-   Direct balance modification
-   Changing ledger history
-   Bypassing authorization
-   Altering audit records

------------------------------------------------------------------------

# 20.9 Extension Decision Matrix

  Requirement                  Solution
  ---------------------------- ---------------------
  New leave category           Configuration
  New approval step            Workflow extension
  New country regulation       Policy extension
  New external system          Integration adapter
  New core business behavior   Domain extension

------------------------------------------------------------------------

# Final Performance and Extension Principles

## Performance

-   Optimize reads through projections
-   Keep writes consistent
-   Process heavy operations asynchronously
-   Monitor continuously

## Scalability

-   Support multiple institutions
-   Preserve tenant isolation
-   Scale workers independently

## Extension

-   Configure before coding
-   Extend before modifying core
-   Preserve domain boundaries

The Leave Management domain remains stable while supporting future
institutional and regulatory evolution.


# 21. Architecture Visual Models

This section provides visual representations of the Leave Management
domain architecture.

All models use Mermaid notation to maintain consistency with SutraERP
architecture documentation standards.

------------------------------------------------------------------------

# 21.1 Domain Context Model

``` mermaid
flowchart LR

Employee["Employee Profile Domain"]

Organization["Organization Domain"]

Attendance["Attendance Domain"]

Calendar["Calendar Domain"]

Security["Authorization Platform"]

Leave["E05 Leave Management Domain"]

Payroll["Payroll Domain"]

Notification["Notification Domain"]

Analytics["Analytics Domain"]


Employee --> Leave

Organization --> Leave

Calendar --> Leave

Attendance --> Leave

Security --> Leave

Leave --> Payroll

Leave --> Notification

Leave --> Analytics
```

------------------------------------------------------------------------

# 21.2 Domain Boundary Model

``` mermaid
flowchart TD

subgraph E05["Leave Management Domain"]

Policy["Leave Policy"]

Request["Leave Request"]

Ledger["Leave Ledger"]

Balance["Leave Balance Projection"]

end


Employee["Employee Profile"]

Org["Organization"]

Attendance["Attendance"]

Payroll["Payroll"]


Employee --> E05

Org --> E05

Attendance --> E05

E05 --> Payroll
```

------------------------------------------------------------------------

# 21.3 Aggregate Model

``` mermaid
flowchart TD

Policy["Leave Policy Aggregate"]

Request["Leave Request Aggregate"]

Ledger["Leave Ledger Aggregate"]

Balance["Leave Balance Projection"]


Policy --> Request

Request --> Ledger

Ledger --> Balance
```

------------------------------------------------------------------------

# 21.4 Leave Request Workflow Model

``` mermaid
flowchart TD

Create["Create Request"]

Validate["Validate"]

Approve["Approval"]

Reserve["Reserve Balance"]

Consume["Consume Leave"]

Close["Close Request"]


Create --> Validate

Validate --> Approve

Approve --> Reserve

Reserve --> Consume

Consume --> Close
```

------------------------------------------------------------------------

# 21.5 Event Flow Model

``` mermaid
flowchart LR

Aggregate["Leave Aggregate"]

Events["Domain Events"]

Consumers["External Consumers"]


Aggregate --> Events

Events --> Consumers


Consumers --> Payroll

Consumers --> Attendance

Consumers --> Notification

Consumers --> Analytics
```

------------------------------------------------------------------------

# 21.6 Ledger and Projection Model

``` mermaid
flowchart LR

Action["Business Action"]

Transaction["Ledger Transaction"]

Projection["Balance Projection"]

Query["Query Consumers"]


Action --> Transaction

Transaction --> Projection

Projection --> Query
```

------------------------------------------------------------------------

# 21.7 Authorization Model

``` mermaid
flowchart TD

User["User"]

Role["Role"]

Attributes["Attributes"]

Scope["Organization Scope"]

Decision["Authorization Decision"]

Resource["Leave Resource"]


User --> Role

User --> Attributes

User --> Scope

Role --> Decision

Attributes --> Decision

Scope --> Decision

Decision --> Resource
```

------------------------------------------------------------------------

# 21.8 Scalability Model

``` mermaid
flowchart LR

Request["High Volume Requests"]

Queue["Processing Queue"]

Worker["Worker Nodes"]

Domain["Leave Domain"]

Projection["Read Models"]


Request --> Queue

Queue --> Worker

Worker --> Domain

Domain --> Projection
```

------------------------------------------------------------------------

# 21.9 Complete Architecture View

``` mermaid
flowchart TB

User["Users"]

API["Leave APIs"]

Workflow["Workflow Engine"]

Domain["Leave Management Domain"]

Ledger["Leave Ledger"]

Events["Domain Events"]

Read["Read Models"]

Consumers["Enterprise Consumers"]


User --> API

API --> Workflow

Workflow --> Domain

Domain --> Ledger

Domain --> Events

Ledger --> Read

Events --> Consumers
```

------------------------------------------------------------------------

# 22. Architecture Decision Records (ADR)

------------------------------------------------------------------------

# ADR-E05-001 --- Ledger-Based Leave Balance Model

## Status

Accepted

## Decision

Leave balance is derived from immutable ledger transactions.

## Context

Direct balance modification creates:

-   Audit problems
-   Reconciliation issues
-   Compliance risks

## Consequences

Positive:

-   Complete history
-   Easy reconciliation
-   Reliable corrections

Negative:

-   Additional transaction processing

------------------------------------------------------------------------

# ADR-E05-002 --- Policy Hierarchy Model

## Status

Accepted

## Decision

Leave policies follow hierarchical inheritance.

Model:

``` text
Enterprise Policy

↓

Institution Policy

↓

Campus Policy

↓

Employee Category Policy
```

## Consequences

Positive:

-   Institutional flexibility
-   Central governance

Risk:

-   Policy conflicts

Mitigation:

-   Conflict detection
-   Governance approval

------------------------------------------------------------------------

# ADR-E05-003 --- Event Driven Integration

## Status

Accepted

## Decision

E05 communicates important business outcomes through domain events.

Examples:

``` text
LeaveApproved

LeaveCancelled

LeaveBalanceUpdated

LeaveAccrualCompleted
```

## Consequences

Positive:

-   Loose coupling
-   Scalability
-   Independent consumers

Risk:

-   Event consistency management

Mitigation:

-   Retry
-   Replay
-   Reconciliation

------------------------------------------------------------------------

# ADR-E05-004 --- E05 Domain Ownership Boundary

## Status

Accepted

## Decision

Leave Management owns leave decisions but not external business facts.

Owned:

``` text
Leave Policies

Leave Requests

Leave Ledger

Leave Balances
```

Not Owned:

``` text
Employee Master

Attendance Calculation

Payroll Calculation
```

------------------------------------------------------------------------

# ADR-E05-005 --- RBAC + ABAC Authorization Model

## Status

Accepted

## Decision

Authorization uses:

``` text
Role

+

Attributes

+

Organization Scope

+

Business Context
```

## Consequences

Positive:

-   Enterprise security
-   Fine-grained access

Risk:

-   Authorization complexity

Mitigation:

-   Central authorization policies

------------------------------------------------------------------------

# ADR-E05-006 --- Configuration First Extension Strategy

## Status

Accepted

## Decision

New requirements should first be solved through:

1.  Configuration
2.  Workflow extension
3.  Integration extension
4.  Core modification only when required

------------------------------------------------------------------------

# ADR-E05-007 --- Aggregate Controlled State Transition

## Status

Accepted

## Decision

Workflow engines request transitions, but aggregates enforce business
rules.

Reason:

Prevents invalid states and preserves domain integrity.

------------------------------------------------------------------------

# ADR-E05-008 --- Sensitive Leave Data Protection

## Status

Accepted

## Decision

Sensitive leave information follows least privilege access.

Examples:

-   Medical leave
-   Protected leave categories
-   Supporting documents

Controls:

-   Restricted access
-   Audit logging
-   Data minimization

------------------------------------------------------------------------

# Final Architecture Visual and ADR Summary

The final E05 architecture is based on:

``` text
Domain Ownership

+

Ledger Integrity

+

Event Driven Integration

+

Policy Configuration

+

Enterprise Security

+

Controlled Extensibility
```

# 23. Glossary

This glossary defines the canonical terminology used within the Leave
Management domain.

The objective is to maintain consistent language across:

-   Architecture documents
-   Domain models
-   Workflow definitions
-   APIs
-   Events
-   Business discussions

------------------------------------------------------------------------

# 23.1 Leave Management Domain Terms

## Leave Management Domain

The enterprise domain responsible for managing employee leave lifecycle
decisions including policies, eligibility, entitlement, requests,
approvals, balances, and leave-related events.

The domain owns leave decisions but does not own employee, attendance,
payroll, or organization master data.

------------------------------------------------------------------------

## Leave Policy

A configurable set of rules defining how a leave type behaves.

A leave policy may define:

-   Eligibility
-   Accrual rules
-   Limits
-   Carry-forward rules
-   Expiry rules
-   Approval requirements
-   Employee category applicability

------------------------------------------------------------------------

## Leave Type

A classification of leave available within the organization.

Examples:

-   Annual Leave
-   Sick Leave
-   Casual Leave
-   Research Leave
-   Special Leave

A leave type does not contain business behavior itself. Behavior is
provided through policies.

------------------------------------------------------------------------

## Leave Policy Version

A versioned representation of a leave policy effective during a specific
period.

Purpose:

-   Preserve historical correctness
-   Support future changes
-   Prevent retroactive modification

Example:

``` text
Policy Version 1

January - June


Policy Version 2

July - December
```

------------------------------------------------------------------------

# 23.2 Entitlement and Balance Terms

## Leave Entitlement

The amount of leave an employee becomes eligible to receive according to
policy rules.

Example:

``` text
Annual Leave Entitlement

= 24 Days
```

Entitlement generation creates ledger transactions.

------------------------------------------------------------------------

## Leave Accrual

The process of gradually generating leave entitlement over time.

Examples:

-   Monthly accrual
-   Quarterly accrual
-   Annual allocation

Accrual always produces ledger transactions.

------------------------------------------------------------------------

## Leave Balance

The current available view of employee leave availability.

Important:

Leave Balance is a projection.

It is not the source of truth.

------------------------------------------------------------------------

## Leave Ledger

The authoritative history of all leave-related transactions.

The ledger records:

-   Entitlement creation
-   Accrual
-   Consumption
-   Adjustment
-   Carry-forward
-   Expiry
-   Reversal

------------------------------------------------------------------------

## Balance Projection

A calculated view generated from Leave Ledger transactions.

Purpose:

-   Fast retrieval
-   Reporting efficiency
-   User experience optimization

------------------------------------------------------------------------

# 23.3 Workflow Terms

## Leave Request

A business request initiated by an employee or authorized user to
consume leave.

A leave request contains:

-   Leave type
-   Duration
-   Reason
-   Request status
-   Approval information

------------------------------------------------------------------------

## Approval Workflow

The controlled process used to evaluate and decide leave requests.

Approval workflow determines:

-   Who reviews
-   Approval sequence
-   Escalation
-   Decision capture

It does not own leave business rules.

------------------------------------------------------------------------

## Leave Reservation

The temporary allocation of leave balance after approval but before
actual consumption.

Purpose:

Prevent conflicting usage of the same leave balance.

Lifecycle:

``` text
Available

↓

Reserved

↓

Consumed
```

------------------------------------------------------------------------

## Leave Consumption

The process of converting approved leave into actual leave usage.

Consumption creates ledger transactions.

------------------------------------------------------------------------

## Leave Adjustment

A controlled correction process for modifying leave history.

Adjustments do not directly edit balances.

Required approach:

``` text
Original Transaction

↓

Reversal

↓

Correction Transaction
```

------------------------------------------------------------------------

## Carry Forward

The process of transferring eligible unused leave from one leave period
to another.

Controlled by:

-   Policy rules
-   Limits
-   Expiry conditions

------------------------------------------------------------------------

## Leave Encashment Eligibility

The determination of whether an employee's leave qualifies for monetary
conversion.

Responsibility split:

Leave Management:

-   Eligibility decision

Payroll:

-   Monetary calculation

------------------------------------------------------------------------

# 23.4 Architecture Terms

## Aggregate

A consistency boundary containing related entities controlled by domain
rules.

E05 aggregates:

-   Leave Policy Aggregate
-   Leave Request Aggregate
-   Leave Ledger Aggregate

------------------------------------------------------------------------

## Domain Event

A statement that a meaningful business fact has occurred.

Examples:

``` text
LeaveApproved

LeaveCancelled

LeaveBalanceUpdated
```

Events communicate facts, not commands.

------------------------------------------------------------------------

## Command

A request to perform a business operation.

Examples:

``` text
SubmitLeaveRequest

ApproveLeaveRequest

CreateLeaveAdjustment
```

Commands may succeed or fail.

------------------------------------------------------------------------

## Projection

A read-optimized representation generated from domain data.

Used for:

-   Dashboards
-   Queries
-   Reports

------------------------------------------------------------------------

## Policy Engine

The component responsible for evaluating configurable business rules.

Examples:

-   Eligibility
-   Accrual
-   Carry-forward

------------------------------------------------------------------------

## Exception Workflow

A controlled path for cases outside normal rules.

Examples:

-   Emergency leave
-   Negative balance approval
-   Special authorization

Exceptions require:

-   Authority
-   Reason
-   Audit

------------------------------------------------------------------------

# 23.5 Security and Governance Terms

## RBAC

Role Based Access Control.

Defines permissions based on user roles.

Example:

``` text
Employee

Manager

HR Administrator
```

------------------------------------------------------------------------

## ABAC

Attribute Based Access Control.

Uses contextual attributes:

-   Organization scope
-   Employee relationship
-   Data sensitivity
-   Workflow state

------------------------------------------------------------------------

## Audit Trail

Immutable history of important business and security actions.

Contains:

-   Who
-   What
-   When
-   Previous state
-   New state
-   Reason

------------------------------------------------------------------------

## Sensitive Leave Data

Leave information requiring additional protection.

Examples:

-   Medical leave information
-   Protected leave categories
-   Supporting documents

------------------------------------------------------------------------

# 24. Domain Summary

## 24.1 Domain Purpose

The Leave Management domain provides SutraERP with an enterprise
capability to manage employee leave lifecycle operations in a
controlled, configurable, and auditable manner.

The domain enables institutions to define leave policies, evaluate
eligibility, process requests, maintain leave history, and integrate
leave outcomes with workforce-related systems.

------------------------------------------------------------------------

# 24.2 Core Capabilities

The domain provides:

## Policy Management

-   Leave policy creation
-   Versioning
-   Eligibility rules
-   Governance

------------------------------------------------------------------------

## Leave Lifecycle Management

-   Request creation
-   Approval processing
-   Cancellation
-   Consumption tracking

------------------------------------------------------------------------

## Entitlement Management

-   Initial allocation
-   Accrual
-   Carry-forward
-   Expiry

------------------------------------------------------------------------

## Transaction Integrity

-   Immutable ledger
-   Reconciliation
-   Correction workflows

------------------------------------------------------------------------

## Enterprise Integration

Publishes leave information to:

-   Attendance
-   Payroll
-   Notification
-   Analytics

------------------------------------------------------------------------

# 24.3 Ownership Summary

## E05 Owns

``` text
Leave Policies

Leave Eligibility

Leave Requests

Leave Approvals

Leave Ledger

Leave Balance Projection

Leave Adjustments

Leave Events
```

------------------------------------------------------------------------

## E05 Does Not Own

``` text
Employee Master Data

Organization Hierarchy

Attendance Calculation

Payroll Calculation

Notification Delivery

Document Storage
```

------------------------------------------------------------------------

# 24.4 Final Architecture Principles

## Principle 1 --- Domain Authority

Leave decisions belong to Leave Management.

------------------------------------------------------------------------

## Principle 2 --- Ledger Integrity

Every balance change is transaction-based.

------------------------------------------------------------------------

## Principle 3 --- Configuration First

Institution differences should be solved through configuration before
customization.

------------------------------------------------------------------------

## Principle 4 --- Event Driven Integration

External systems consume leave facts through contracts and events.

------------------------------------------------------------------------

## Principle 5 --- Enterprise Security

Leave information follows least privilege access.

------------------------------------------------------------------------

# 24.5 Final Domain Statement

The Leave Management domain represents a stable enterprise capability
that provides configurable leave governance while maintaining strong
transaction integrity, clear domain ownership, secure access control,
and extensible integration patterns.

It enables SutraERP to support diverse institutional leave requirements
without compromising architectural consistency.


# 25. Document Completion & Governance

This section defines the governance model for maintaining the Leave
Management architecture document.

The purpose is to ensure that:

-   Architecture decisions remain controlled
-   Domain ownership remains clear
-   Future changes are evaluated properly
-   Documentation remains synchronized with implementation

------------------------------------------------------------------------

# 25.1 Document Ownership

## Primary Owner

``` text
Leave Management Domain Owner
```

Responsibilities:

-   Maintain domain accuracy
-   Approve workflow changes
-   Validate business rule changes
-   Coordinate with dependent domains

------------------------------------------------------------------------

## Architecture Owner

``` text
Enterprise Architecture Team
```

Responsibilities:

-   Validate architecture alignment
-   Review cross-domain impacts
-   Approve major architectural changes

------------------------------------------------------------------------

## Supporting Owners

  Area                       Owner
  -------------------------- -------------------------
  Employee context           Employee Profile Domain
  Attendance integration     Attendance Domain
  Payroll integration        Payroll Domain
  Notifications              Notification Domain
  Security model             Security Platform
  Documentation governance   Architecture Governance

------------------------------------------------------------------------

# 25.2 Document Lifecycle

The document follows controlled lifecycle management.

``` mermaid
stateDiagram-v2

Draft --> Review

Review --> Approved

Approved --> Published

Published --> ChangeRequested

ChangeRequested --> Review

Review --> Deprecated
```

------------------------------------------------------------------------

# 25.3 Change Management Process

Any change affecting E05 must be evaluated.

Change categories:

## Minor Change

Examples:

-   Clarifying documentation
-   Non-breaking terminology updates
-   Additional examples

Process:

``` text
Documentation Review

↓

Approval

↓

Update Version
```

------------------------------------------------------------------------

## Major Change

Examples:

-   New aggregate
-   New ownership boundary
-   New domain event
-   New workflow lifecycle

Process:

``` text
Change Proposal

↓

Impact Analysis

↓

Architecture Review

↓

ADR Update

↓

Document Update
```

------------------------------------------------------------------------

# 25.4 Architecture Governance Rules

The following rules are mandatory:

## Rule 1

Domain ownership cannot change without architecture review.

------------------------------------------------------------------------

## Rule 2

New integrations require contract definition.

------------------------------------------------------------------------

## Rule 3

New business rules require validation impact analysis.

------------------------------------------------------------------------

## Rule 4

Existing audit history cannot be invalidated.

------------------------------------------------------------------------

## Rule 5

Breaking changes require versioning.

------------------------------------------------------------------------

# 25.5 Document Versioning Strategy

Version format:

``` text
Major.Minor.Patch
```

Example:

``` text
1.0.0
```

Meaning:

  Version   Meaning
  --------- -------------------------------
  Major     Architecture-impacting change
  Minor     New capability
  Patch     Documentation correction

------------------------------------------------------------------------

# 25.6 Review Frequency

Recommended reviews:

  Review Type           Frequency
  --------------------- ---------------------
  Architecture Review   Major changes
  Security Review       Periodic
  Workflow Review       Business cycle
  Dependency Review     Integration changes

------------------------------------------------------------------------

# 25.7 Final Completion Checklist

  Area                           Status
  ------------------------------ ----------
  Purpose Defined                Complete
  Scope Defined                  Complete
  Dependencies Defined           Complete
  Ownership Boundaries Defined   Complete
  Aggregates Defined             Complete
  Workflows Defined              Complete
  Business Rules Defined         Complete
  Events Defined                 Complete
  Service Contracts Defined      Complete
  State Models Defined           Complete
  Security Defined               Complete
  Scalability Defined            Complete
  Extensions Defined             Complete
  ADRs Defined                   Complete
  Glossary Defined               Complete

------------------------------------------------------------------------

# 26. Appendices

------------------------------------------------------------------------

# Appendix A --- Domain Artifact Summary

## Aggregates

``` text
Leave Policy Aggregate

Leave Request Aggregate

Leave Ledger Aggregate

Leave Balance Projection
```

------------------------------------------------------------------------

## Major Workflows

``` text
Leave Request

Approval

Accrual

Consumption

Cancellation

Adjustment

Carry Forward

Encashment Eligibility

Policy Activation
```

------------------------------------------------------------------------

# Appendix B --- Domain Event Catalogue

  Event                   Producer                  Purpose
  ----------------------- ------------------------- ------------------------------
  LeaveRequested          Leave Request Aggregate   Request submitted
  LeaveApproved           Leave Request Aggregate   Approval completed
  LeaveRejected           Leave Request Aggregate   Approval denied
  LeaveCancelled          Leave Request Aggregate   Leave cancelled
  LeaveBalanceUpdated     Leave Ledger              Balance changed
  LeaveAccrualCompleted   Accrual Process           Accrual completed
  LeavePolicyActivated    Policy Aggregate          Policy activated
  LeaveAdjusted           Adjustment Workflow       Balance correction completed

------------------------------------------------------------------------

# Appendix C --- Command Catalogue

  Command                 Purpose
  ----------------------- -----------------------
  SubmitLeaveRequest      Create leave request
  ApproveLeaveRequest     Approve request
  RejectLeaveRequest      Reject request
  CancelLeaveRequest      Cancel leave
  CreateLeaveAdjustment   Correct leave history

------------------------------------------------------------------------

# Appendix D --- Query Catalogue

  Query                 Purpose
  --------------------- ----------------------------
  GetLeaveBalance       Retrieve balance
  GetLeaveHistory       Retrieve transactions
  GetLeaveEligibility   Evaluate eligibility
  GetLeavePolicy        Retrieve applicable policy

------------------------------------------------------------------------

# Appendix E --- State Reference

## Leave Request States

``` text
Draft

Submitted

Validated

Pending Approval

Approved

Reserved

Consumed

Closed

Rejected

Cancelled
```

------------------------------------------------------------------------

## Policy States

``` text
Draft

Under Review

Approved

Scheduled

Active

Superseded

Archived
```

------------------------------------------------------------------------

# Appendix F --- Boundary Reference

## E05 Owns

``` text
Leave Decisions

Leave Policies

Leave Requests

Leave Ledger

Leave Balance Projection

Leave Events
```

------------------------------------------------------------------------

## External Ownership

``` text
Employee Data

Organization Structure

Attendance Facts

Payroll Calculation

Documents

Notifications
```

------------------------------------------------------------------------

# Appendix G --- Final Architecture Statement

The Leave Management domain provides SutraERP with a secure, scalable,
and configurable enterprise capability for managing employee leave
lifecycle operations.

The architecture is based on:

``` text
Clear Domain Ownership

+

Ledger-Based Transaction Integrity

+

Configurable Policies

+

Event Driven Integration

+

Enterprise Security

+

Controlled Extensibility
```

The completed L2-E05 architecture is designed to support institutional
diversity while preserving consistency, auditability, and long-term
maintainability.


