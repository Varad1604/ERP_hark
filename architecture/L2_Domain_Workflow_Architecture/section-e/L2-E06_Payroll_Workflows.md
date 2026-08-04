# L2-E06 — Payroll Workflows

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose; §24.1 Payroll Domain Purpose; Part 1 --- Metadata + Purpose + Scope |
| Scope | §2 Scope; §2.1 In Scope; §2.2 Out of Scope |
| Actors | Actors |
| Entities | §6.1 Aggregate Overview; §7 Aggregate Design; §7.2 Payroll Result Aggregate |
| Business Rules | §10 Business Rules; §10.1 Payroll Run Business Rules; Final Business Rule and Event Statement |
| Workflow | §8 Domain Workflows; §8.1 Payroll Workflow Overview; §8.2 Workflow 1 --- Payroll Run Creation |
| State Machine | §13.1 Payroll Run State Machine |
| Contracts | §12 Service Contracts; §12.4 Integration Contracts; §16 Service Contract Governance |
| Events | §11 Domain Events; §12 Event Flow Model; §13 Event Consumers |
| Dependencies | §3 Upstream Dependencies; §3.1 Upstream Dependency Model; §3.2 Workforce Domain Dependency |
| Locked Decisions | §5.6 Ownership Decision Principle; §17 Decision Ownership Model; §17.1 Decision Authority Matrix |

---

# Part 1 --- Metadata + Purpose + Scope

------------------------------------------------------------------------

# Document Metadata

  -----------------------------------------------------------------------
  Attribute                           Value
  ----------------------------------- -----------------------------------
  Document Name                       L2-E06 --- Payroll Workflows
  Document Owner                      Varad

  Domain Classification               Financial Transaction Processing
                                      Domain

  Document Level                      Level 2 --- Domain Workflow
                                      Architecture

  ERP Section                         Section E --- People & Workforce

  Domain Owner                        Payroll Domain

  Architecture Style                  Domain-Driven Design + Event-Driven
                                      Architecture

  Primary Responsibility              Payroll calculation, execution
                                      lifecycle, financial result
                                      generation

  Source of Truth                     Payroll transactional records and
                                      finalized payroll snapshots

  Security Classification             Sensitive Financial Data

  Status                              Draft

  Version                             1.0
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 1. Purpose

The Payroll Management domain provides SutraERP with an enterprise
capability for executing employee compensation processing through
controlled payroll workflows.

The domain transforms governed workforce and compensation facts into
validated financial outcomes through:

-   payroll calculation
-   earnings processing
-   deduction processing
-   statutory execution
-   approval workflows
-   payroll finalization
-   financial output generation
-   payslip generation
-   payroll audit preservation

Payroll exists as an independent financial execution domain.

The core responsibility is:

> Given approved business facts and applicable payroll rules, determine
> the accurate financial compensation outcome for each employee.

------------------------------------------------------------------------

# 1.1 Core Objectives

## Accuracy

Payroll must apply correct:

-   compensation structures
-   deduction rules
-   compliance rules
-   effective dates
-   calculation versions

## Reproducibility

Every finalized payroll result must be explainable through:

-   input snapshots
-   rule versions
-   calculation versions
-   approval history

## Auditability

Important payroll actions must preserve:

-   actor identity
-   timestamp
-   business reason
-   state changes

## Security

Payroll data is sensitive financial information protected through:

-   least privilege access
-   organization scope isolation
-   controlled visibility

## Scalability

Payroll must support:

-   multiple institutions
-   large employee populations
-   configurable payroll policies
-   multiple payroll cycles

------------------------------------------------------------------------

# 2. Scope

The Payroll domain covers responsibilities required to execute the
payroll lifecycle from payroll run creation until payroll closure.

------------------------------------------------------------------------

# 2.1 In Scope

## Payroll Run Management

Includes:

-   payroll run creation
-   payroll period management
-   payroll type handling
-   calculation lifecycle
-   approval lifecycle
-   finalization lifecycle
-   closure lifecycle

Supported payroll run types:

-   Regular Payroll
-   Bonus Payroll
-   Arrears Payroll
-   Final Settlement Payroll
-   Off-Cycle Payroll
-   Adjustment Payroll

------------------------------------------------------------------------

## Payroll Calculation Execution

Payroll owns:

-   earnings calculation execution
-   deduction calculation execution
-   net pay calculation
-   validation
-   calculation snapshots
-   payroll result generation

Payroll executes approved rules and does not own originating business
facts.

------------------------------------------------------------------------

## Payroll Adjustments

Includes:

-   retro pay
-   arrears
-   reversals
-   correction transactions
-   controlled recalculation

Direct modification of finalized payroll is prohibited.

------------------------------------------------------------------------

## Payslip Generation

Payroll owns:

-   payslip generation
-   payslip metadata
-   payslip lifecycle events

Document storage remains with Document Platform.

------------------------------------------------------------------------

## Payroll Audit History

Maintains audit records for:

-   calculations
-   approvals
-   adjustments
-   finalization
-   configuration usage

------------------------------------------------------------------------

# 2.2 Out of Scope

Payroll does not own:

## Employee Master Management

Owned by Employee/Workforce Domain.

## Attendance Management

Owned by Attendance Domain.

## Leave Management

Owned by Leave Management Domain.

## Compensation Policy Definition

Owned by Compensation/HR Policy Domain.

## Payment Execution

Owned by Finance/Treasury Domain.

## Accounting Ledger

Owned by Finance Domain.

## Document Storage

Owned by Document Platform.

------------------------------------------------------------------------

# 2.3 Scope Boundary Principle

``` text
Payroll owns calculation execution.

Payroll does not own the facts used for calculation.
```

``` mermaid
flowchart LR

Facts["Governed Business Facts"]

Payroll["Payroll Execution Domain"]

Result["Payroll Financial Result"]

Consumers["Finance / Employee / Analytics / Compliance"]

Facts --> Payroll

Payroll --> Result

Result --> Consumers
```

------------------------------------------------------------------------

# 2.4 Final Scope Statement

L2-E06 Payroll Workflows covers the complete lifecycle of payroll
execution, from payroll run creation through calculation, approval,
finalization, financial output generation, and closure.

The domain ensures payroll outcomes are:

-   accurate
-   auditable
-   reproducible
-   secure
-   scalable


# 3. Upstream Dependencies

The Payroll domain does not create the business facts required for
payroll calculation.

Payroll consumes governed facts from authoritative upstream domains.

The fundamental principle:

``` text
Upstream Domains Own Facts

Payroll Owns Financial Calculation
```

------------------------------------------------------------------------

# 3.1 Upstream Dependency Model

``` mermaid
flowchart LR

Workforce["Workforce Domain"]

Attendance["Attendance Domain"]

Leave["Leave Management Domain"]

Compensation["Compensation Domain"]

Compliance["Compliance Configuration"]

Calendar["Enterprise Calendar"]


Payroll["Payroll Domain"]


Workforce --> Payroll

Attendance --> Payroll

Leave --> Payroll

Compensation --> Payroll

Compliance --> Payroll

Calendar --> Payroll
```

------------------------------------------------------------------------

# 3.2 Workforce Domain Dependency

## Dependency Purpose

Payroll requires authoritative workforce information to determine who
can participate in payroll processing.

------------------------------------------------------------------------

## Workforce Provides

-   Employee identity reference
-   Employment status
-   Engagement type
-   Effective employment dates
-   Organizational assignment
-   Workforce category

------------------------------------------------------------------------

## Payroll Uses Workforce Data For

-   Payroll eligibility evaluation
-   Employee payroll inclusion
-   Employee grouping
-   Payroll scope determination

------------------------------------------------------------------------

## Payroll Does Not Own

-   Employee creation
-   Employment lifecycle
-   Employee master data

------------------------------------------------------------------------

## Integration Contract

``` text
Workforce Fact

↓

Payroll Eligibility Evaluation
```

------------------------------------------------------------------------

# 3.3 Attendance Domain Dependency

## Dependency Purpose

Attendance provides finalized work participation facts required for
payroll calculation.

------------------------------------------------------------------------

## Attendance Provides

-   Finalized attendance records
-   Payable days
-   Loss of pay information
-   Attendance adjustment outcomes

------------------------------------------------------------------------

## Payroll Uses Attendance Data For

-   Salary impact calculation
-   Attendance-based deductions
-   Payable day calculation

------------------------------------------------------------------------

## Payroll Does Not Own

-   Attendance capture
-   Attendance correction
-   Attendance approval

------------------------------------------------------------------------

## Integration Principle

``` text
Attendance decides attendance truth.

Payroll calculates financial impact.
```

------------------------------------------------------------------------

# 3.4 Leave Management Domain Dependency

## Dependency Purpose

Leave Management provides approved leave outcomes that affect payroll
processing.

------------------------------------------------------------------------

## Leave Provides

-   Approved leave records
-   Leave impact information
-   Leave-related payable adjustments

------------------------------------------------------------------------

## Payroll Uses Leave Data For

-   Payable calculation
-   Leave-related deductions
-   Leave encashment processing where applicable

------------------------------------------------------------------------

## Payroll Does Not Own

-   Leave policy
-   Leave entitlement
-   Leave balance
-   Leave approval

------------------------------------------------------------------------

# 3.5 Compensation Domain Dependency

## Dependency Purpose

Compensation provides approved compensation structures and policies.

------------------------------------------------------------------------

## Compensation Provides

-   Salary structure definitions
-   Compensation component definitions
-   Benefit eligibility
-   Effective dates
-   Compensation versions

------------------------------------------------------------------------

## Payroll Uses Compensation Data For

-   Earnings calculation
-   Allowance calculation
-   Benefit processing

------------------------------------------------------------------------

## Ownership Boundary

``` text
Compensation Defines

Payroll Executes
```

------------------------------------------------------------------------

# 3.6 Compliance Configuration Dependency

## Dependency Purpose

Payroll requires versioned statutory and compliance rules.

------------------------------------------------------------------------

## Compliance Provides

-   Rule definitions
-   Effective dates
-   Jurisdiction applicability
-   Regulatory parameters

------------------------------------------------------------------------

## Payroll Owns

-   Rule execution
-   Calculation application
-   Compliance output generation

------------------------------------------------------------------------

# 3.7 Enterprise Calendar Dependency

## Dependency Purpose

Payroll requires governed time periods.

------------------------------------------------------------------------

## Calendar Provides

-   Payroll periods
-   Effective dates
-   Processing windows

------------------------------------------------------------------------

## Payroll Uses Calendar For

-   Payroll scheduling
-   Period locking
-   Processing control

------------------------------------------------------------------------

# 4. Downstream Dependencies

Payroll produces financial outcomes consumed by downstream domains.

------------------------------------------------------------------------

# 4.1 Downstream Dependency Model

``` mermaid
flowchart LR

Payroll["Payroll Domain"]

Finance["Finance Domain"]

EmployeePortal["Employee Portal"]

Notification["Notification Domain"]

Analytics["Analytics Platform"]

Compliance["Compliance Systems"]

Documents["Document Platform"]


Payroll --> Finance

Payroll --> EmployeePortal

Payroll --> Notification

Payroll --> Analytics

Payroll --> Compliance

Payroll --> Documents
```

------------------------------------------------------------------------

# 4.2 Finance Domain Dependency

## Dependency Purpose

Finance consumes finalized payroll financial outputs.

------------------------------------------------------------------------

## Payroll Provides

-   Payroll totals
-   Employee payable amounts
-   Deduction summaries
-   Employer contributions
-   Accounting-ready outputs

------------------------------------------------------------------------

## Finance Owns

-   Payment execution
-   Accounting journals
-   Financial statements
-   Ledger reconciliation

------------------------------------------------------------------------

## Boundary Rule

``` text
Payroll Calculates Money.

Finance Moves and Accounts Money.
```

------------------------------------------------------------------------

# 4.3 Employee Portal Dependency

## Dependency Purpose

Employees access payroll information through employee-facing channels.

------------------------------------------------------------------------

## Payroll Provides

-   Payslip availability
-   Payroll result status
-   Employee payroll information

------------------------------------------------------------------------

## Employee Portal Owns

-   User experience
-   Notification presentation
-   Self-service interaction

------------------------------------------------------------------------

# 4.4 Notification Domain Dependency

## Dependency Purpose

Payroll publishes events requiring communication.

------------------------------------------------------------------------

## Payroll Events May Trigger

-   Payslip available notification
-   Payroll completion notification
-   Adjustment notification

------------------------------------------------------------------------

## Notification Owns

-   Delivery channels
-   Templates
-   Communication tracking

------------------------------------------------------------------------

# 4.5 Analytics Platform Dependency

## Dependency Purpose

Analytics consumes payroll facts for institutional insights.

------------------------------------------------------------------------

## Payroll Provides

-   Payroll events
-   Reporting datasets
-   Aggregated financial facts

------------------------------------------------------------------------

## Analytics Owns

-   Dashboards
-   Trends
-   Forecasting
-   Business intelligence models

------------------------------------------------------------------------

# 4.6 Compliance Systems Dependency

## Dependency Purpose

External compliance systems consume payroll outputs.

------------------------------------------------------------------------

## Payroll Provides

-   Compliance reports
-   Calculated statutory values
-   Submission data

------------------------------------------------------------------------

## External Systems Own

-   Government submission channels
-   External verification

------------------------------------------------------------------------

# 4.7 Document Platform Dependency

## Dependency Purpose

Payslip documents require secure document lifecycle management.

------------------------------------------------------------------------

## Payroll Provides

-   Payslip generation metadata
-   Document creation requests

------------------------------------------------------------------------

## Document Platform Owns

-   Storage
-   Versioning
-   Access lifecycle

------------------------------------------------------------------------

# 5. Dependency Governance Principles

## Principle 1 --- No Shared Database Ownership

Domains communicate through:

-   APIs
-   Events
-   Read contracts

------------------------------------------------------------------------

## Principle 2 --- Source Domain Owns Facts

Examples:

``` text
Attendance owns attendance facts

Leave owns leave facts

Compensation owns compensation facts

Payroll owns payroll results
```

------------------------------------------------------------------------

## Principle 3 --- Payroll Consumes, Calculates, Publishes

``` mermaid
flowchart LR

Facts["Authoritative Domain Facts"]

Payroll["Payroll Calculation"]

Results["Payroll Outcomes"]

Consumers["Enterprise Consumers"]


Facts --> Payroll

Payroll --> Results

Results --> Consumers
```

------------------------------------------------------------------------

# Final Dependency Summary

Payroll sits at the center of financial execution while remaining
bounded from upstream and downstream ownership.

The final dependency model is:

``` text
Governed Facts

↓

Payroll Execution

↓

Financial Outcomes

↓

Enterprise Consumers
```

This preserves:

-   ownership clarity
-   audit integrity
-   scalability
-   maintainability


# 5. Domain Ownership Boundaries

The Payroll domain follows strict bounded-context ownership.

The fundamental rule:

``` text
A domain owns decisions within its capability boundary.

A domain consumes facts owned by other authoritative domains.
```

Payroll owns financial execution decisions but does not become the owner
of every business fact involved in payroll processing.

------------------------------------------------------------------------

# 5.1 Payroll Domain Ownership Model

``` mermaid
flowchart TD

subgraph Payroll["L2-E06 Payroll Domain"]

Run["Payroll Run Management"]

Calc["Payroll Calculation"]

Result["Payroll Results"]

Adjustment["Payroll Adjustments"]

Payslip["Payslip Generation"]

Audit["Payroll Audit History"]

end


Workforce["Workforce Domain"]

Attendance["Attendance Domain"]

Leave["Leave Management Domain"]

Compensation["Compensation Domain"]

Finance["Finance Domain"]


Workforce --> Payroll

Attendance --> Payroll

Leave --> Payroll

Compensation --> Payroll

Payroll --> Finance
```

------------------------------------------------------------------------

# 5.2 Capabilities Owned by Payroll

## Payroll Run Lifecycle Ownership

Payroll owns:

-   payroll run creation
-   payroll period execution
-   payroll processing state
-   validation state
-   approval state
-   finalization state
-   closure state

Payroll does not own enterprise calendar creation.

------------------------------------------------------------------------

## Payroll Calculation Ownership

Payroll owns:

-   calculation execution
-   earnings computation
-   deduction execution
-   net pay calculation
-   calculation validation

Payroll does not own the source policies used for calculation.

------------------------------------------------------------------------

## Payroll Result Ownership

Payroll owns the authoritative payroll outcome.

A Payroll Result contains:

-   employee reference
-   earnings
-   deductions
-   net pay
-   applied rule versions
-   calculation snapshot references
-   processing metadata

------------------------------------------------------------------------

## Payroll Adjustment Ownership

Payroll owns controlled correction workflows.

Includes:

-   arrears
-   retro pay
-   reversals
-   adjustment calculations
-   special payroll processing

------------------------------------------------------------------------

## Payslip Ownership

Payroll owns:

-   payslip generation
-   payslip content generation
-   payslip availability event

Document Platform owns:

-   physical storage
-   document lifecycle
-   document access controls

------------------------------------------------------------------------

# 5.3 External Domain Ownership Boundaries

------------------------------------------------------------------------

# Workforce Domain Boundary

## Workforce Owns

-   employee identity
-   employment relationship
-   workforce category
-   employment status
-   effective employment dates

## Payroll Owns

-   payroll eligibility evaluation
-   payroll processing decisions

Boundary:

``` text
Workforce determines employment facts.

Payroll determines payroll execution.
```

------------------------------------------------------------------------

# Attendance Domain Boundary

## Attendance Owns

-   attendance capture
-   attendance validation
-   attendance corrections
-   attendance finalization

## Payroll Owns

-   financial impact calculation

Boundary:

``` text
Attendance determines worked time.

Payroll determines compensation impact.
```

------------------------------------------------------------------------

# Leave Management Domain Boundary

## Leave Owns

-   leave policy
-   leave eligibility
-   leave approval
-   leave balance
-   leave ledger

## Payroll Owns

-   financial impact of approved leave outcomes

Boundary:

``` text
Leave determines leave facts.

Payroll determines monetary impact.
```

------------------------------------------------------------------------

# Compensation Domain Boundary

## Compensation Owns

-   salary structure definition
-   compensation policy
-   benefit rules
-   compensation eligibility

## Payroll Owns

-   applying approved compensation rules during execution

Boundary:

``` text
Compensation defines.

Payroll executes.
```

------------------------------------------------------------------------

# Finance Domain Boundary

## Finance Owns

-   payment execution
-   accounting journals
-   financial ledger
-   treasury operations

## Payroll Owns

-   calculated payable outcomes

Boundary:

``` text
Payroll calculates money.

Finance moves and accounts money.
```

------------------------------------------------------------------------

# 5.4 Ownership Matrix

  Capability            Owner
  --------------------- -------------------------
  Employee Identity     Workforce Domain
  Employment Status     Workforce Domain
  Attendance Facts      Attendance Domain
  Leave Facts           Leave Management Domain
  Salary Policy         Compensation Domain
  Payroll Calculation   Payroll Domain
  Payroll Result        Payroll Domain
  Payroll Adjustment    Payroll Domain
  Payslip Generation    Payroll Domain
  Payslip Storage       Document Platform
  Payment Execution     Finance Domain
  Accounting Posting    Finance Domain
  Payroll Analytics     Analytics Platform

------------------------------------------------------------------------

# 5.5 Forbidden Ownership Patterns

The following patterns are prohibited.

------------------------------------------------------------------------

## Payroll Owning Employee Master

Reason:

Creates duplicate workforce truth.

------------------------------------------------------------------------

## Payroll Calculating Attendance

Reason:

Attendance facts belong to Attendance domain.

------------------------------------------------------------------------

## Payroll Managing Leave Balance

Reason:

Leave lifecycle belongs to Leave Management.

------------------------------------------------------------------------

## Finance Editing Payroll Results

Reason:

Financial accounting must consume payroll outcomes, not modify them.

------------------------------------------------------------------------

## HR Directly Editing Final Payroll

Reason:

Final payroll is a controlled financial record.

------------------------------------------------------------------------

# 5.6 Ownership Decision Principle

The final ownership model:

``` mermaid
flowchart LR

Facts["Business Facts"]

Payroll["Payroll Execution"]

Financial["Financial Outcomes"]


Facts --> Payroll

Payroll --> Financial
```

Where:

Business Facts are owned by their source domains.

Payroll owns transformation from facts into financial outcomes.

------------------------------------------------------------------------

# Final Domain Ownership Statement

The Payroll domain is the authoritative owner of payroll execution,
calculation, payroll results, adjustments, and payroll audit history.

It intentionally avoids ownership of employee, attendance, leave,
compensation policy, payment execution, and accounting responsibilities.

This separation preserves:

-   architectural clarity
-   financial integrity
-   auditability
-   scalability
-   long-term maintainability


# 6. Domain Model

The Payroll domain is a financial execution domain that transforms
governed business facts into controlled payroll outcomes.

Core principle:

``` text
Business Facts → Payroll Execution → Financial Outcome
```

Payroll does not own employee, attendance, leave, or compensation truth.

------------------------------------------------------------------------

# 6.1 Aggregate Overview

The Payroll domain contains four primary aggregates:

  -----------------------------------------------------------------------
  Aggregate                           Responsibility
  ----------------------------------- -----------------------------------
  Payroll Run Aggregate               Controls payroll execution
                                      lifecycle

  Payroll Result Aggregate            Stores employee payroll outcomes

  Payroll Adjustment Aggregate        Handles corrections and retroactive
                                      changes

  -----------------------------------------------------------------------

``` mermaid
flowchart TD

Config["Payroll Configuration Aggregate"]

Run["Payroll Run Aggregate"]

Result["Payroll Result Aggregate"]

Adjustment["Payroll Adjustment Aggregate"]

Config --> Run

Run --> Result

Result --> Adjustment
```

------------------------------------------------------------------------

# 7. Aggregate Design

## 7.1 Payroll Run Aggregate

### Purpose

Controls a complete payroll execution cycle.

### Aggregate Root

`PayrollRun`

### Owns

-   Payroll period
-   Payroll type
-   Processing state
-   Validation state
-   Approval state
-   Finalization state

### Does Not Own

-   Employee master
-   Attendance records
-   Leave balances
-   Compensation policy

------------------------------------------------------------------------

## Payroll Run Lifecycle

``` mermaid
stateDiagram-v2

[*] --> Created

Created --> Calculating

Calculating --> Calculated

Calculated --> Validating

Validating --> PendingApproval

PendingApproval --> Approved

Approved --> Finalized

Finalized --> Posted

Posted --> Closed

Calculating --> Failed

Validating --> Failed

Failed --> Reprocessing
```

------------------------------------------------------------------------

## Payroll Run Invariants

-   Approval is mandatory before finalization.
-   Finalized payroll cannot be directly modified.
-   Closed payroll cannot return to active processing.
-   Every run belongs to one payroll period.

------------------------------------------------------------------------

# 7.2 Payroll Result Aggregate

### Purpose

Represents employee-level payroll financial outcome.

### Aggregate Root

`PayrollResult`

### Owns

• Earnings
• Deductions
• Net Pay
• Calculation Snapshot
• Applied Compensation Version Reference
• Applied Compliance Rule Version Reference
• Applied Rule Snapshot Reference
• Execution Metadata

------------------------------------------------------------------------

## Payroll Result Model

``` mermaid
flowchart TD

Result["Payroll Result"]

Earnings["Earnings"]

Deductions["Deductions"]

Taxes["Statutory Deductions"]

Benefits["Benefits"]

NetPay["Net Pay"]

Result --> Earnings

Result --> Deductions

Deductions --> Taxes

Deductions --> Benefits

Result --> NetPay
```

------------------------------------------------------------------------

## Payroll Result Invariants

-   Net pay must be derived from earnings and deductions.
-   Every result belongs to exactly one payroll run.
-   Every result maintains calculation references.
-   Finalized results are immutable.

------------------------------------------------------------------------

# 7.3 Payroll Adjustment Aggregate

### Purpose

Handles controlled changes after payroll calculation.

Examples:

-   Retro pay
-   Arrears
-   Corrections
-   Reversals
-   Off-cycle adjustments

------------------------------------------------------------------------

## Adjustment Lifecycle

``` mermaid
stateDiagram-v2

[*] --> Requested

Requested --> Validated

Validated --> PendingApproval

PendingApproval --> Approved

Approved --> Calculated

Calculated --> Applied

Applied --> Closed

PendingApproval --> Rejected
```

------------------------------------------------------------------------

## Adjustment Rules

Allowed:

-   Create adjustment request
-   Validate impact
-   Approve adjustment
-   Apply financial correction

Forbidden:

-   Editing finalized payroll directly
-   Removing historical payroll results

------------------------------------------------------------------------



## Configuration Lifecycle

``` mermaid
stateDiagram-v2

[*] --> Draft

Draft --> Review

Review --> Approved

Approved --> Active

Active --> Superseded

Superseded --> Archived
```

------------------------------------------------------------------------

# 8. Historical Integrity Model

A finalized payroll result must always be reproducible.

Required references:

-   Employee snapshot
-   Attendance snapshot
-   Leave snapshot
-   Compensation version
-   Tax rule version
-   Calculation version
-   Approval history
-   Finalization timestamp

------------------------------------------------------------------------

## Immutable Payroll Principle

``` mermaid
flowchart LR

Calculated["Calculated Payroll"]

Approved["Approved Payroll"]

Finalized["Finalized Payroll"]

Adjustment["Adjustment Workflow"]

Calculated --> Approved

Approved --> Finalized

Finalized --> Adjustment
```

------------------------------------------------------------------------

# 9. Complete Payroll Lifecycle

``` mermaid
flowchart LR

Create["Create Payroll Run"]

Calculate["Calculate Payroll"]

Validate["Validate Results"]

Approve["Approve Payroll"]

Finalize["Finalize Payroll"]

Post["Finance Posting"]

Close["Close Payroll"]

Archive["Archive History"]

Create --> Calculate

Calculate --> Validate

Validate --> Approve

Approve --> Finalize

Finalize --> Post

Post --> Close

Close --> Archive
```

------------------------------------------------------------------------

# Final Architecture Statement

Payroll is modeled as independent financial aggregates with explicit
ownership boundaries.

The aggregate design ensures:

-   scalable payroll execution
-   immutable financial records
-   controlled corrections
-   versioned calculation logic
-   audit-ready history


# 8. Domain Workflows

The Payroll domain executes controlled financial processing workflows.

The workflows are designed around the principle:

``` text
Governed Inputs → Payroll Processing → Validated Financial Outcome
```

Payroll workflows preserve:

-   financial correctness
-   approval control
-   auditability
-   historical integrity

------------------------------------------------------------------------

# 8.1 Payroll Workflow Overview

``` mermaid
flowchart LR

Create["Create Payroll Run"]

Input["Collect Governed Inputs"]

Calculate["Calculate Payroll"]

Validate["Validate Payroll"]

Approve["Approve Payroll"]

Finalize["Finalize Payroll"]

Post["Finance Posting"]

Payslip["Generate Payslips"]

Close["Close Payroll"]


Create --> Input
Input --> Calculate
Calculate --> Validate
Validate --> Approve
Approve --> Finalize
Finalize --> Post
Finalize --> Payslip
Post --> Close
Payslip --> Close
```

------------------------------------------------------------------------

# 8.2 Workflow 1 --- Payroll Run Creation

## Purpose

Creates a controlled payroll execution instance.

## Trigger

Authorized payroll user initiates payroll processing.

## Actors

-   Payroll Administrator
-   Payroll System

## Preconditions

-   Payroll period exists
-   Payroll configuration is active
-   User has authorization

## Workflow

``` mermaid
flowchart LR

Request["Create Payroll Run Request"]

Check["Validate Configuration"]

Create["Create Payroll Run"]

Event["Publish PayrollRunCreated"]


Request --> Check
Check --> Create
Create --> Event
```

## Output

Created Payroll Run Aggregate.

------------------------------------------------------------------------

# 8.3 Workflow 2 --- Payroll Input Collection

## Purpose

Collects approved facts from upstream domains.

Sources:

-   Workforce
-   Attendance
-   Leave
-   Compensation
-   Compliance Rules

## Rule

Payroll consumes facts but does not modify source data.

``` mermaid
flowchart LR

Workforce["Workforce Facts"]

Attendance["Attendance Facts"]

Leave["Leave Facts"]

Compensation["Compensation Facts"]

Payroll["Payroll Engine"]


Workforce --> Payroll

Attendance --> Payroll

Leave --> Payroll

Compensation --> Payroll
```

------------------------------------------------------------------------

# 8.4 Workflow 3 --- Payroll Calculation

## Purpose

Calculates employee payroll outcomes.

Processing:

-   earnings calculation
-   deduction calculation
-   statutory calculation
-   net pay calculation

``` mermaid
flowchart LR

Inputs["Payroll Inputs"]

Engine["Calculation Engine"]

Snapshot["Calculation Snapshot"]

Result["Payroll Result"]


Inputs --> Engine

Engine --> Snapshot

Snapshot --> Result
```

------------------------------------------------------------------------

# 8.5 Workflow 4 --- Payroll Validation

## Purpose

Ensures payroll correctness before approval.

Validation includes:

-   missing employee data
-   invalid rules
-   calculation failures
-   abnormal values
-   reconciliation checks

``` mermaid
flowchart LR

Calculated["Calculated Payroll"]

Validation["Validation Engine"]

Approved["Validation Passed"]

Exception["Validation Exception"]


Calculated --> Validation

Validation --> Approved

Validation --> Exception
```

------------------------------------------------------------------------

# 8.6 Workflow 5 --- Payroll Approval

## Purpose

Provides financial authorization before finalization.

Actors:

-   Payroll Approver
-   Authorized Finance Reviewer (if configured)

``` mermaid
flowchart LR

Validation["Validated Payroll"]

Review["Approval Review"]

Approved["Approved"]

Rejected["Rejected"]


Validation --> Review

Review --> Approved

Review --> Rejected
```

------------------------------------------------------------------------

# 8.7 Workflow 6 --- Payroll Finalization

## Purpose

Locks payroll results as authoritative financial records.

Rules:

-   Finalized payroll cannot be edited
-   Corrections require adjustment workflow

``` mermaid
flowchart LR

Approved["Approved Payroll"]

Finalize["Finalize Payroll"]

Locked["Immutable Payroll Result"]

Adjustment["Adjustment Workflow"]


Approved --> Finalize

Finalize --> Locked

Locked --> Adjustment
```

------------------------------------------------------------------------

# 8.8 Workflow 7 --- Finance Posting

## Purpose

Transfers payroll financial outcomes to Finance.

Payroll provides:

-   payable amounts
-   deductions
-   employer contributions

Finance owns:

-   accounting
-   payment execution

``` mermaid
flowchart LR

Payroll["Finalized Payroll"]

Output["Financial Output"]

Finance["Finance Processing"]


Payroll --> Output

Output --> Finance
```

------------------------------------------------------------------------

# 8.9 Workflow 8 --- Payslip Generation

## Purpose

Creates employee payroll documents.

Flow:

``` mermaid
flowchart LR

Finalized["Finalized Payroll"]

Generate["Generate Payslip"]

Document["Document Platform"]

Employee["Employee Portal"]


Finalized --> Generate

Generate --> Document

Document --> Employee
```

------------------------------------------------------------------------

# 8.10 Workflow 9 --- Payroll Adjustment

## Purpose

Handles controlled post-payroll corrections.

Examples:

-   incorrect deduction
-   missed earning
-   retroactive change

``` mermaid
flowchart LR

Issue["Correction Request"]

Review["Impact Analysis"]

Approval["Approval"]

Adjustment["Adjustment Calculation"]

Apply["Apply Adjustment"]


Issue --> Review

Review --> Approval

Approval --> Adjustment

Adjustment --> Apply
```

------------------------------------------------------------------------

# 8.11 Workflow 10 --- Retro Pay Processing

## Purpose

Processes salary changes effective from previous periods.

Example:

Salary revision effective April discovered in June.

Process:

``` mermaid
flowchart LR

Change["Compensation Change"]

Identify["Identify Impact Period"]

Calculate["Calculate Difference"]

Create["Create Arrear Transaction"]

Pay["Include in Payroll"]


Change --> Identify

Identify --> Calculate

Calculate --> Create

Create --> Pay
```

------------------------------------------------------------------------

# 8.12 Workflow 11 --- Final Settlement Payroll

## Purpose

Processes employee exit compensation.

Includes:

-   pending salary
-   deductions
-   recoveries
-   leave settlement

Uses:

`FINAL_SETTLEMENT` payroll run type.

------------------------------------------------------------------------

# 8.13 Workflow 12 --- Off-Cycle Payroll

## Purpose

Handles exceptional payroll execution outside normal cycles.

Examples:

-   emergency payments
-   special incentives
-   urgent corrections

Rules:

-   requires authorization
-   uses payroll engine
-   maintains audit trail

``` mermaid
flowchart LR

Request["Off-Cycle Request"]

Approval["Approval"]

Run["Off-Cycle Payroll"]

Payment["Finance Processing"]


Request --> Approval

Approval --> Run

Run --> Payment
```

------------------------------------------------------------------------

# 8.14 Workflow Governance Principles

## Principle 1

Finalized payroll is immutable.

## Principle 2

Corrections happen through adjustments.

## Principle 3

Payroll never bypasses approval controls.

## Principle 4

Every financial action is auditable.

## Principle 5

Source domains own business facts.

------------------------------------------------------------------------

# Final Workflow Statement

The Payroll domain provides a controlled end-to-end workflow from
payroll creation to financial closure while maintaining strict ownership
boundaries and financial integrity.

# 9. Workflow Specifications

This section defines the detailed operational specifications for Payroll
workflows.

Each workflow specification defines:

-   Purpose
-   Trigger
-   Actors
-   Preconditions
-   Inputs
-   Processing steps
-   Decision points
-   Events
-   Outputs
-   Failure handling
-   Audit requirements

------------------------------------------------------------------------

# WF-E06-001 --- Payroll Run Creation Workflow

## Purpose

Creates a controlled payroll execution instance.

------------------------------------------------------------------------

## Trigger

Authorized payroll user initiates payroll processing for a defined
payroll period.

------------------------------------------------------------------------

## Actors

  Actor                   Responsibility
  ----------------------- ---------------------------
  Payroll Administrator   Initiates payroll run
  Payroll System          Validates and creates run

------------------------------------------------------------------------

## Preconditions

-   Payroll period exists
-   Payroll configuration is active
-   User authorization exists
-   Organization scope is valid

------------------------------------------------------------------------

## Inputs

-   Payroll period
-   Payroll type
-   Organization scope
-   Processing configuration

------------------------------------------------------------------------

## Processing Steps

``` mermaid
flowchart LR

Request["Create Payroll Run"]

Validate["Validate Period and Configuration"]

Create["Create Payroll Run Aggregate"]

Event["Publish PayrollRunCreated"]


Request --> Validate

Validate --> Create

Create --> Event
```

------------------------------------------------------------------------

## Decision Points

  Decision                    Outcome
  --------------------------- -----------------------
  Configuration available?    Continue / Reject
  Period already processed?   Prevent duplicate run
  User authorized?            Allow / Deny

------------------------------------------------------------------------

## Output

Created Payroll Run.

------------------------------------------------------------------------

## Failure Handling

  Failure                 Recovery
  ----------------------- ------------------------------
  Invalid period          Correct period configuration
  Duplicate payroll run   Existing run review
  Authorization failure   Request permission

------------------------------------------------------------------------

## Audit Requirements

Capture:

-   creator
-   timestamp
-   payroll period
-   payroll type

------------------------------------------------------------------------

# WF-E06-002 --- Payroll Calculation Workflow

## Purpose

Calculates employee payroll outcomes using governed facts and approved
rules.

------------------------------------------------------------------------

## Trigger

Payroll run enters calculation state.

------------------------------------------------------------------------

## Actors

-   Payroll Engine
-   Payroll Administrator

------------------------------------------------------------------------

## Preconditions

-   Payroll run created
-   Input facts available
-   Rules active

------------------------------------------------------------------------

## Processing Steps

``` mermaid
flowchart LR

Facts["Governed Facts"]

Engine["Calculation Engine"]

Snapshot["Calculation Snapshot"]

Result["Payroll Results"]


Facts --> Engine

Engine --> Snapshot

Snapshot --> Result
```

------------------------------------------------------------------------

## Calculation Sequence

1.  Load employee eligibility.
2.  Load compensation configuration.
3.  Load attendance impact.
4.  Load leave impact.
5.  Apply earning rules.
6.  Apply deduction rules.
7.  Apply compliance rules.
8.  Generate payroll results.

------------------------------------------------------------------------

## Decision Points

  Decision                Action
  ----------------------- --------------------
  Missing input data      Exception
  Invalid rule version    Stop calculation
  Calculation completed   Move to validation

------------------------------------------------------------------------

## Events

Published:

`PayrollCalculationCompleted`

------------------------------------------------------------------------

## Failure Handling

Calculation failures:

-   isolate failed employees
-   preserve error details
-   allow controlled retry

------------------------------------------------------------------------

# WF-E06-003 --- Payroll Validation Workflow

## Purpose

Ensures payroll correctness before approval.

------------------------------------------------------------------------

## Validation Categories

## Data Validation

Checks:

-   employee information
-   required inputs
-   configuration availability

------------------------------------------------------------------------

## Calculation Validation

Checks:

-   calculation completeness
-   deduction consistency
-   net pay accuracy

------------------------------------------------------------------------

## Compliance Validation

Checks:

-   statutory requirements
-   applicable rules

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart LR

Payroll["Calculated Payroll"]

Validation["Validation Engine"]

Passed["Validation Passed"]

Exception["Validation Exception"]


Payroll --> Validation

Validation --> Passed

Validation --> Exception
```

------------------------------------------------------------------------

## Output

Validated payroll ready for approval.

------------------------------------------------------------------------

# WF-E06-004 --- Payroll Approval Workflow

## Purpose

Provides controlled financial authorization.

------------------------------------------------------------------------

## Actors

-   Payroll Approver
-   Finance Reviewer (optional)

------------------------------------------------------------------------

## Preconditions

-   Validation completed
-   No blocking exceptions

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart LR

Validated["Validated Payroll"]

Review["Approval Review"]

Approve["Approved"]

Reject["Rejected"]


Validated --> Review

Review --> Approve

Review --> Reject
```

------------------------------------------------------------------------

## Approval Decisions

  Condition               Result
  ----------------------- ---------
  Acceptable              Approve
  Exception unresolved    Reject
  Missing authorization   Block

------------------------------------------------------------------------

## Audit Capture

Store:

-   approver identity
-   approval timestamp
-   decision
-   comments

------------------------------------------------------------------------

# WF-E06-005 --- Payroll Finalization Workflow

## Purpose

Converts approved payroll into immutable financial records.

------------------------------------------------------------------------

## Preconditions

-   Approval completed

------------------------------------------------------------------------

## Processing

``` mermaid
flowchart LR

Approved["Approved Payroll"]

Finalize["Finalize"]

Immutable["Locked Payroll Result"]


Approved --> Finalize

Finalize --> Immutable
```

------------------------------------------------------------------------

## Rules

After finalization:

Allowed:

-   View
-   Report
-   Audit
-   Adjustment request

Not allowed:

-   Direct edit
-   Delete
-   Recalculate in place

------------------------------------------------------------------------

# WF-E06-006 --- Finance Posting Workflow

## Purpose

Transfers payroll financial outputs to Finance.

------------------------------------------------------------------------

## Processing

``` mermaid
flowchart LR

Payroll["Finalized Payroll"]

Output["Financial Output"]

Finance["Finance Domain"]


Payroll --> Output

Output --> Finance
```

------------------------------------------------------------------------

## Finance Receives

-   payroll totals
-   payable amounts
-   deductions
-   contributions

------------------------------------------------------------------------

# WF-E06-007 --- Payslip Generation Workflow

## Purpose

Creates employee payroll documents.

------------------------------------------------------------------------

## Processing

``` mermaid
flowchart LR

Result["Finalized Payroll Result"]

Generate["Generate Payslip"]

Store["Document Platform"]

Access["Employee Portal"]


Result --> Generate

Generate --> Store

Store --> Access
```

------------------------------------------------------------------------

## Rules

-   Payslip represents finalized payroll state.
-   Modification requires regeneration.
-   Previous versions remain auditable.

------------------------------------------------------------------------

# WF-E06-008 --- Payroll Adjustment Workflow

## Purpose

Handles controlled payroll corrections.

------------------------------------------------------------------------

## Trigger

Correction requirement identified.

------------------------------------------------------------------------

## Processing

``` mermaid
flowchart LR

Request["Adjustment Request"]

Analyze["Impact Analysis"]

Approve["Approval"]

Calculate["Adjustment Calculation"]

Apply["Apply Adjustment"]


Request --> Analyze

Analyze --> Approve

Approve --> Calculate

Calculate --> Apply
```

------------------------------------------------------------------------

## Supported Adjustments

-   Retro pay
-   Arrears
-   Missing earnings
-   Incorrect deductions
-   Reversals

------------------------------------------------------------------------

# WF-E06-009 --- Retro Pay Workflow

## Purpose

Processes historical compensation changes.

------------------------------------------------------------------------

## Processing

``` mermaid
flowchart LR

Change["Compensation Change"]

Identify["Identify Previous Periods"]

Calculate["Calculate Difference"]

Arrear["Create Arrear"]

Include["Include Payroll"]


Change --> Identify

Identify --> Calculate

Calculate --> Arrear

Arrear --> Include
```

------------------------------------------------------------------------

# WF-E06-010 --- Final Settlement Workflow

## Purpose

Processes employee exit payroll.

------------------------------------------------------------------------

## Includes

-   pending salary
-   deductions
-   recoveries
-   settlements

------------------------------------------------------------------------

## Rule

Final settlement uses separate payroll run type.

------------------------------------------------------------------------

# WF-E06-011 --- Off-Cycle Payroll Workflow

## Purpose

Processes exceptional payroll execution.

------------------------------------------------------------------------

## Examples

-   emergency payments
-   special incentive payments
-   approved corrections

------------------------------------------------------------------------

## Processing

``` mermaid
flowchart LR

Request["Off-Cycle Request"]

Approval["Approval"]

Run["Off-Cycle Payroll"]

Payment["Finance Processing"]


Request --> Approval

Approval --> Run

Run --> Payment
```

------------------------------------------------------------------------

# Workflow Specification Summary

  Workflow               Primary Outcome
  ---------------------- -----------------------------
  Payroll Run Creation   Controlled payroll instance
  Calculation            Payroll results
  Validation             Error-free payroll
  Approval               Financial authorization
  Finalization           Immutable records
  Finance Posting        Financial integration
  Payslip Generation     Employee document
  Adjustment             Controlled correction
  Retro Pay              Historical correction
  Final Settlement       Exit processing
  Off-Cycle Payroll      Exceptional processing

------------------------------------------------------------------------

# Final Workflow Specification Principle

Every Payroll workflow must maintain:

-   authorization
-   auditability
-   financial integrity
-   domain ownership boundaries
-   reproducibility

# 10. Business Rules

Payroll business rules define the financial execution constraints that
ensure payroll processing remains accurate, controlled, auditable, and
compliant.

The governing principle:

``` text
Source Domains Define Facts and Policies.

Payroll Executes Approved Financial Processing Rules.
```

------------------------------------------------------------------------

# 10.1 Payroll Run Business Rules

## BR-PAY-001 --- Unique Payroll Run

A payroll run must be uniquely identified by:

-   organization scope
-   payroll period
-   payroll type

Duplicate active payroll runs for the same scope and period are
prohibited.

------------------------------------------------------------------------

## BR-PAY-002 --- Valid Payroll Period Required

A payroll run cannot be created unless:

-   payroll period exists
-   period is open
-   applicable configuration exists

------------------------------------------------------------------------

## BR-PAY-003 --- Controlled Payroll Lifecycle

Payroll runs must follow:

``` text
Created
→ Calculating
→ Validating
→ Approved
→ Finalized
→ Posted
→ Closed
```

Skipping lifecycle stages is prohibited.

------------------------------------------------------------------------

# 10.2 Payroll Calculation Rules

## BR-PAY-004 --- Rule Version Locking

Payroll shall persist the exact Compensation Version ID and Compliance Rule Version ID used during execution. Payroll shall not create, approve, or modify these versions. Ownership remains with the originating domains.

Every payroll calculation must execute against a specific rule version.

A calculation must preserve:

-   rule version
-   configuration version
-   execution timestamp

------------------------------------------------------------------------

## BR-PAY-005 --- Calculation Snapshot Requirement

Every payroll result must maintain a snapshot reference containing the
inputs used during calculation.

Required references:

-   employee snapshot
-   attendance snapshot
-   leave snapshot
-   compensation snapshot
-   compliance rule snapshot

------------------------------------------------------------------------

## BR-PAY-006 --- Net Pay Integrity

Payroll result validation must ensure:

``` text
Net Pay = Total Earnings - Total Deductions
```

------------------------------------------------------------------------

# 10.3 Payroll Eligibility Rules

## BR-PAY-007 --- Employment Eligibility

An employee may participate in payroll only when workforce eligibility
criteria are satisfied.

Payroll evaluates:

-   employment status
-   effective dates
-   organization assignment

------------------------------------------------------------------------

## BR-PAY-008 --- Historical Employee Processing

Employees leaving during a payroll period must be processed according
to:

-   effective employment dates
-   settlement rules
-   applicable compensation rules

------------------------------------------------------------------------

# 10.4 Attendance Impact Rules

## BR-PAY-009 --- Attendance Source Authority

Attendance facts must come from the Attendance domain.

Payroll cannot modify attendance records.

------------------------------------------------------------------------

## BR-PAY-010 --- Finalized Attendance Dependency

Attendance-based payroll calculation requires finalized attendance
information.

------------------------------------------------------------------------

# 10.5 Leave Impact Rules

## BR-PAY-011 --- Leave Source Authority

Leave facts must come from Leave Management.

Payroll consumes:

-   approved leave outcomes
-   payable impact information

------------------------------------------------------------------------

## BR-PAY-012 --- Leave Correction Handling

Changes after payroll calculation require controlled adjustment
workflows.

------------------------------------------------------------------------

# 10.6 Compensation Rules

## BR-PAY-013 --- Compensation Policy Ownership

Compensation defines salary structures.

Payroll applies approved structures during calculation.

------------------------------------------------------------------------

## BR-PAY-014 --- Effective Date Selection

Payroll must select the compensation version applicable for the payroll
period.

------------------------------------------------------------------------

# 10.7 Deduction Rules

## BR-PAY-015 --- Deduction Priority

Deduction execution must follow configured priority ordering.

Examples:

1.  Statutory deductions
2.  Mandatory deductions
3.  Voluntary deductions

------------------------------------------------------------------------

## BR-PAY-016 --- Deduction Validation

Payroll must validate:

-   deduction eligibility
-   deduction limits
-   applicable rules

------------------------------------------------------------------------

# 10.8 Approval Rules

## BR-PAY-017 --- Approval Required Before Finalization

Payroll cannot be finalized without authorized approval.

------------------------------------------------------------------------

## BR-PAY-018 --- Approval Audit

Approval records must capture:

-   approver identity
-   timestamp
-   decision
-   comments

------------------------------------------------------------------------

# 10.9 Finalization Rules

## BR-PAY-019 --- Immutable Final Payroll

After finalization:

Allowed:

-   view
-   reporting
-   audit
-   adjustment creation

Not allowed:

-   direct editing
-   deletion
-   recalculation in place

------------------------------------------------------------------------

## BR-PAY-020 --- Correction Through Adjustment

All finalized payroll corrections must use:

-   adjustment workflow
-   approval workflow
-   audit history

------------------------------------------------------------------------

# 10.10 Adjustment Rules

## BR-PAY-021 --- Adjustment Reason Required

Every adjustment requires:

-   business reason
-   requester
-   approval information

------------------------------------------------------------------------

## BR-PAY-022 --- Retro Pay Processing

Retroactive changes must identify:

-   impacted periods
-   difference amount
-   correction reason

------------------------------------------------------------------------

# 11. Domain Events

Payroll publishes business events representing completed business facts.

The principle:

``` text
Events Describe What Happened.

Events Do Not Describe Database Changes.
```

------------------------------------------------------------------------

# 11.1 PayrollRunCreated

## Meaning

A payroll execution instance has been created.

## Payload

``` text
Payroll Run ID

Payroll Period

Payroll Type

Organization Scope

Created By

Creation Timestamp
```

------------------------------------------------------------------------

# 11.2 PayrollCalculationStarted

## Meaning

Payroll calculation execution has started.

Payload:

``` text
Payroll Run ID

Calculation Version

Execution Timestamp
```

------------------------------------------------------------------------

# 11.3 PayrollCalculationCompleted

## Meaning

Payroll calculation completed successfully.

Payload:

``` text
Payroll Run ID

Processed Employee Count

Calculation Summary

Completion Timestamp
```

------------------------------------------------------------------------

# 11.4 PayrollValidationCompleted

## Meaning

Payroll validation completed.

Payload:

``` text
Payroll Run ID

Validation Status

Exception Count

Validation Timestamp
```

------------------------------------------------------------------------

# 11.5 PayrollApproved

## Meaning

Payroll received authorized approval.

Payload:

``` text
Payroll Run ID

Approver Identity

Approval Decision

Approval Timestamp
```

------------------------------------------------------------------------

# 11.6 PayrollFinalized

## Meaning

Payroll became an immutable financial record.

Payload:

``` text
Payroll Run ID

Finalization Timestamp

Financial Summary

Finalization Actor
```

------------------------------------------------------------------------

# 11.7 PayslipGenerated

## Meaning

Employee payslip artifact was generated.

Payload:

``` text
Employee Reference

Payroll Run ID

Document Reference

Generation Timestamp
```

------------------------------------------------------------------------

# 11.8 PayrollAdjustmentCreated

## Meaning

A controlled correction workflow was initiated.

Payload:

``` text
Adjustment ID

Adjustment Type

Reason

Affected Payroll Period
```

------------------------------------------------------------------------

# 11.9 PayrollClosed

## Meaning

Payroll lifecycle completed.

Payload:

``` text
Payroll Run ID

Closure Timestamp

Final Status
```

------------------------------------------------------------------------

# 12. Event Flow Model

``` mermaid
flowchart LR

Created["PayrollRunCreated"]

Calculated["PayrollCalculationCompleted"]

Validated["PayrollValidationCompleted"]

Approved["PayrollApproved"]

Finalized["PayrollFinalized"]

Payslip["PayslipGenerated"]

Closed["PayrollClosed"]


Created --> Calculated

Calculated --> Validated

Validated --> Approved

Approved --> Finalized

Finalized --> Payslip

Payslip --> Closed
```

------------------------------------------------------------------------

# 13. Event Consumers

  Event                      Consumer
  -------------------------- --------------------
  PayrollFinalized           Finance Domain
  PayslipGenerated           Employee Portal
  PayrollClosed              Analytics Platform
  PayrollAdjustmentCreated   Payroll Audit
  Compliance Outputs         Compliance Systems

------------------------------------------------------------------------

# 14. Event Governance Rules

## Event Versioning

Events must support backward compatibility.

------------------------------------------------------------------------

## Idempotency

Consumers must safely process duplicate events.

------------------------------------------------------------------------

## Event Audit

Published events must maintain:

-   event identifier
-   timestamp
-   producer
-   processing status

------------------------------------------------------------------------

# Final Business Rule and Event Statement

Payroll business rules ensure financial correctness.

Payroll domain events provide controlled communication of payroll
outcomes across SutraERP while maintaining:

-   ownership boundaries
-   auditability
-   consistency
-   scalability

# 12. Service Contracts

Service contracts define the controlled interaction model of the Payroll
domain.

The principle:

``` text
External Systems Request Business Capabilities.

They Do Not Manipulate Payroll Data Directly.
```

Payroll services expose:

-   commands
-   queries
-   domain operations
-   integration contracts

------------------------------------------------------------------------

# 12.1 Service Architecture Model

``` mermaid
flowchart LR

Client["Enterprise Consumers"]

API["Payroll Service Layer"]

Domain["Payroll Domain Logic"]

Aggregate["Payroll Aggregates"]

Events["Domain Events"]


Client --> API

API --> Domain

Domain --> Aggregate

Aggregate --> Events
```

------------------------------------------------------------------------

# 12.2 Payroll Command Services

## Create Payroll Run

### Purpose

Creates a new payroll execution instance.

Command:

``` text
CreatePayrollRun
```

Input:

-   payroll period
-   payroll type
-   organization scope
-   processing configuration

Output:

-   Payroll Run ID
-   Initial status

Rules:

-   duplicate payroll runs prohibited
-   valid configuration required

------------------------------------------------------------------------

## Start Payroll Calculation

### Purpose

Initiates payroll calculation.

Command:

``` text
StartPayrollCalculation
```

Input:

-   Payroll Run ID

Output:

-   Calculation execution status

Rules:

-   payroll run must be in Calculating state
-   required inputs must exist

------------------------------------------------------------------------

## Validate Payroll

### Purpose

Executes payroll validation.

Command:

``` text
ValidatePayrollRun
```

Input:

-   Payroll Run ID

Output:

-   Validation result
-   Exception summary

------------------------------------------------------------------------

## Approve Payroll

### Purpose

Authorizes payroll finalization.

Command:

``` text
ApprovePayrollRun
```

Input:

-   Payroll Run ID
-   Approver identity
-   Approval decision

Rules:

-   only authorized approvers
-   validation must pass

------------------------------------------------------------------------

## Finalize Payroll

### Purpose

Locks payroll as immutable.

Command:

``` text
FinalizePayrollRun
```

Rules:

-   approval required
-   finalized payroll cannot be edited

------------------------------------------------------------------------

## Create Payroll Adjustment

### Purpose

Creates controlled correction workflow.

Command:

``` text
CreatePayrollAdjustment
```

Input:

-   affected payroll result
-   adjustment reason
-   adjustment type

------------------------------------------------------------------------

# 12.3 Payroll Query Services

## Get Payroll Run Status

Returns:

-   lifecycle state
-   processing progress
-   exceptions

------------------------------------------------------------------------

## Get Payroll Result

Returns:

-   earnings
-   deductions
-   net pay
-   calculation metadata

------------------------------------------------------------------------

## Get Payslip

Returns:

-   payslip reference
-   document status

------------------------------------------------------------------------

## Get Payroll Audit History

Returns:

-   lifecycle changes
-   approvals
-   adjustments

------------------------------------------------------------------------

# 12.4 Integration Contracts

## Finance Integration Contract

Payroll publishes:

-   finalized payroll totals
-   payable amounts
-   deduction summaries

Finance owns:

-   payment
-   accounting

------------------------------------------------------------------------

## Notification Contract

Payroll publishes:

``` text
PayslipGenerated
PayrollFinalized
```

Notification owns delivery.

------------------------------------------------------------------------

# 13. State Transition Matrix

------------------------------------------------------------------------

# 13.1 Payroll Run State Machine

``` mermaid
stateDiagram-v2

[*] --> Created

Created --> Calculating

Calculating --> Calculated

Calculated --> Validating

Validating --> PendingApproval

PendingApproval --> Approved

Approved --> Finalized

Finalized --> Posted

Posted --> Closed

Calculating --> Failed

Validating --> Failed

Failed --> Reprocessing
```

------------------------------------------------------------------------

# 13.2 State Transition Rules

  Current State     Allowed Transition   Condition
  ----------------- -------------------- --------------------------
  Created           Calculating          Inputs available
  Calculating       Calculated           Calculation completed
  Calculated        Validating           Results generated
  Validating        PendingApproval      Validation passed
  PendingApproval   Approved             Authorized approval
  Approved          Finalized            Approval complete
  Finalized         Posted               Finance output accepted
  Posted            Closed               Reconciliation completed

------------------------------------------------------------------------

# 13.3 Restricted Transitions

  Transition               Reason
  ------------------------ ----------------------------------
  Closed → Calculating     Closed payroll cannot reopen
  Finalized → Editing      Financial integrity violation
  Approved → Calculating   Requires controlled reprocessing
  Posted → Modification    Accounting inconsistency

------------------------------------------------------------------------

# 14. Payroll Result State Model

``` mermaid
stateDiagram-v2

[*] --> Calculated

Calculated --> Validated

Validated --> Approved

Approved --> Finalized

Finalized --> AdjustmentRequired

AdjustmentRequired --> Adjusted

Adjusted --> Finalized
```

------------------------------------------------------------------------

# 15. Adjustment State Model

``` mermaid
stateDiagram-v2

[*] --> Requested

Requested --> UnderReview

UnderReview --> Approved

Approved --> Calculated

Calculated --> Applied

Applied --> Closed

UnderReview --> Rejected
```

------------------------------------------------------------------------

# 16. Service Contract Governance

All Payroll services must enforce:

## Authorization

Every command requires permission validation.

## Idempotency

Repeated requests must not create duplicate financial outcomes.

## Auditability

Every state-changing operation must record:

-   actor
-   timestamp
-   reason
-   previous state
-   new state

## Version Compatibility

Integration contracts must support controlled evolution.

------------------------------------------------------------------------

# Final Service Contract Principle

Payroll services expose business capabilities, not database operations.

The domain boundary remains:

``` text
Commands Change Payroll State.

Queries Read Payroll State.

Events Communicate Payroll Outcomes.
```


# 17. Decision Ownership Model

Payroll decisions must have explicit ownership.

The principle:

``` text
The Domain That Owns The Rule Owns The Decision.
```

Payroll executes payroll decisions.

It does not override upstream business ownership.

------------------------------------------------------------------------

# 17.1 Decision Authority Matrix

  -----------------------------------------------------------------------
  Decision Area     Decision Owner    Supporting Domain Payroll
                                                        Responsibility
  ----------------- ----------------- ----------------- -----------------
  Employee payroll  Workforce Domain  Payroll           Consume
  eligibility                                           eligibility
                                                        result

  Attendance impact Attendance Domain Payroll           Apply financial
                                                        impact

  Leave impact      Leave Domain      Payroll           Apply approved
                                                        impact

  Salary structure  Compensation      Payroll           Execute
                    Domain                              calculation

  Statutory rules   Compliance Domain Payroll           Apply active
                                                        rules

  Payroll           Payroll Domain    All input domains Execute
  calculation                                           calculation

  Payroll approval  Authorized        Finance           Maintain workflow
                    Payroll Approver  (optional)        

  Payroll           Payroll Domain    Approver          Lock results
  finalization                                          

  Payment execution Finance Domain    Payroll           Provide payable
                                                        output

  Accounting        Finance Domain    Payroll           Provide financial
  posting                                               data
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 17.2 Payroll Processing Decision Matrix

  -----------------------------------------------------------------------
  Decision          Options           Decision          Outcome
                                      Authority         
  ----------------- ----------------- ----------------- -----------------
  Create payroll    Create / Reject   Payroll           Run created or
  run                                 Administrator     rejected

  Include employee  Include / Exclude Payroll Engine    Employee
                                                        processed or
                                                        skipped

  Calculation valid Pass / Fail       Validation Engine Continue or
                                                        exception

  Payroll approval  Approve / Reject  Approver          Finalization
                                                        allowed or
                                                        blocked

  Finalization      Lock / Prevent    Payroll System    Immutable record
                                                        created

  Adjustment        Create / Ignore   Payroll           Correction
  required                            Controller        workflow
                                                        initiated
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 17.3 Exception Decision Matrix

  -----------------------------------------------------------------------
  Situation         Decision          Owner             Action
  ----------------- ----------------- ----------------- -----------------
  Missing           Block calculation Payroll           Wait for
  attendance                                            attendance
                                                        correction

  Missing           Block calculation Compensation +    Resolve rule
  compensation                        Payroll           issue

  Invalid deduction Exception review  Payroll           Correct deduction
                                                        configuration

  Approval rejected Recalculate if    Payroll           Restart
                    required                            controlled flow

  Payment failure   Finance recovery  Finance           Retry settlement
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 18. Failure Matrix

Payroll failures are categorized by:

-   severity
-   ownership
-   recovery strategy
-   financial impact

------------------------------------------------------------------------

# 18.1 Payroll Run Creation Failures

  -----------------------------------------------------------------------
  Failure           Cause             Impact            Recovery
  ----------------- ----------------- ----------------- -----------------
  Duplicate payroll Existing active   Duplicate         Review existing
  run               run               processing risk   run

  Invalid payroll   Configuration     Run creation      Correct period
  period            issue             blocked           

  Unauthorized user Permission        Operation denied  Request
                    failure                             authorization

  Missing           Rule unavailable  Cannot start      Activate
  configuration                       payroll           configuration
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 18.2 Calculation Failures

  -----------------------------------------------------------------------
  Failure           Cause             Impact            Recovery
  ----------------- ----------------- ----------------- -----------------
  Missing employee  Upstream          Employee          Resolve source
  input             incomplete data   calculation       data
                                      blocked           

  Rule execution    Configuration     Incorrect         Fix rule version
  error             defect            calculation risk  

  Calculation       Processing        Partial           Resume controlled
  timeout           overload          completion        processing

  Engine failure    Technical issue   Payroll stopped   Retry execution
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 18.3 Validation Failures

  -----------------------------------------------------------------------
  Failure           Cause             Severity          Recovery
  ----------------- ----------------- ----------------- -----------------
  Net pay mismatch  Calculation       Critical          Recalculate
                    inconsistency                       

  Invalid deduction Rule issue        High              Correct deduction
  amount                                                rule

  Missing mandatory Source issue      High              Update source
  data                                                  domain

  Compliance        Regulatory issue  Critical          Resolve
  validation                                            compliance rule
  failure                                               
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 18.4 Approval Failures

  -----------------------------------------------------------------------
  Failure           Cause             Impact            Recovery
  ----------------- ----------------- ----------------- -----------------
  Approval timeout  Approver          Payroll delayed   Escalation
                    unavailable                         workflow

  Rejection         Business          Finalization      Correct and
                    disagreement      blocked           resubmit

  Unauthorized      Security          Approval invalid  Audit
  approval          violation                           investigation
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 18.5 Finalization Failures

  -----------------------------------------------------------------------
  Failure           Cause             Impact            Recovery
  ----------------- ----------------- ----------------- -----------------
  Lock failure      System issue      Payroll not       Retry transaction
                                      finalized         

  Event publishing  Integration issue Consumer delay    Retry event
  failure                                               delivery

  Snapshot missing  Data integrity    Cannot finalize   Rebuild snapshot
                    issue                               
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 18.6 Finance Integration Failures

  -----------------------------------------------------------------------
  Failure           Cause             Impact            Recovery
  ----------------- ----------------- ----------------- -----------------
  Posting rejected  Finance           Payment delayed   Finance
                    validation issue                    correction

  Duplicate posting Retry issue       Accounting risk   Idempotency
  attempt                                               handling

  Finance           System outage     Posting delayed   Retry queue
  unavailable                                           
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 18.7 Payslip Generation Failures

  ------------------------------------------------------------------------
  Failure           Cause              Impact            Recovery
  ----------------- ------------------ ----------------- -----------------
  Document          Template/storage   Employee cannot   Regenerate
  generation        issue              view payslip      
  failure                                                

  Document storage  Platform issue     Payslip           Retry storage
  failure                              unavailable       

  Access failure    Permission issue   Employee          Fix access policy
                                       visibility issue  
  ------------------------------------------------------------------------

------------------------------------------------------------------------

# 19. Failure Recovery Architecture

``` mermaid
flowchart LR

Failure["Payroll Failure"]

Detect["Failure Detection"]

Classify["Failure Classification"]

Recover["Recovery Strategy"]

Retry["Retry"]

Escalate["Human Escalation"]

Resolve["Resolved"]


Failure --> Detect

Detect --> Classify

Classify --> Recover

Recover --> Retry

Recover --> Escalate

Retry --> Resolve

Escalate --> Resolve
```

------------------------------------------------------------------------

# 19.1 Recovery Principles

## Retryable Failures

Examples:

-   temporary service outage
-   network failure
-   event delivery failure

Action:

Automatic retry.

------------------------------------------------------------------------

## Business Failures

Examples:

-   missing approval
-   invalid configuration
-   incorrect data

Action:

Human correction required.

------------------------------------------------------------------------

## Critical Financial Failures

Examples:

-   incorrect final payroll
-   duplicate payment risk
-   invalid statutory calculation

Action:

Immediate escalation and controlled recovery.

------------------------------------------------------------------------

# 20. Failure Handling Rules

## Rule 1 --- Never Corrupt Finalized Payroll

Finalized payroll failures require:

-   adjustment workflow
-   audit record
-   controlled correction

------------------------------------------------------------------------

## Rule 2 --- Preserve Failure Context

Every failure must capture:

-   failure ID
-   workflow ID
-   timestamp
-   affected payroll run
-   technical details
-   business impact

------------------------------------------------------------------------

## Rule 3 --- No Silent Failure

All failures must become:

-   events
-   audit records
-   operational alerts

------------------------------------------------------------------------

# 21. Operational Failure Events

  Event                       Purpose
  --------------------------- ----------------------------------
  PayrollCalculationFailed    Calculation failure notification
  PayrollValidationFailed     Validation failure notification
  PayrollApprovalRejected     Approval failure tracking
  PayrollFinalizationFailed   Finalization recovery
  PayrollPostingFailed        Finance integration recovery

------------------------------------------------------------------------

# Final Decision and Failure Architecture Statement

Payroll maintains enterprise reliability through:

-   explicit decision ownership
-   controlled state transitions
-   recoverable failures
-   immutable financial records
-   complete auditability

The system prevents silent financial corruption while allowing
controlled recovery from operational failures.

# 15. Validation Rules

Validation rules ensure that Payroll processing produces accurate,
compliant, and financially reliable outcomes.

Payroll validation is not limited to data checking.

The validation framework protects:

-   data integrity
-   calculation integrity
-   financial integrity
-   compliance integrity
-   workflow integrity
-   authorization integrity

Core principle:

``` text
Payroll cannot proceed when financial correctness cannot be guaranteed.
```

------------------------------------------------------------------------

# 15.1 Validation Architecture

The Payroll Validation Engine evaluates payroll at multiple stages.

``` mermaid
flowchart LR

Inputs["Payroll Inputs"]

DataValidation["Data Validation"]

CalculationValidation["Calculation Validation"]

ComplianceValidation["Compliance Validation"]

FinancialValidation["Financial Validation"]

WorkflowValidation["Workflow Validation"]

Decision["Validation Decision"]


Inputs --> DataValidation

DataValidation --> CalculationValidation

CalculationValidation --> ComplianceValidation

ComplianceValidation --> FinancialValidation

FinancialValidation --> WorkflowValidation

WorkflowValidation --> Decision
```

------------------------------------------------------------------------

# 15.2 Validation Execution Stages

## Stage 1 --- Input Validation

Purpose:

Ensure all required payroll inputs are available.

Checks:

-   employee reference exists
-   employment status available
-   attendance data available
-   leave impact available
-   compensation configuration available
-   payroll rules available

------------------------------------------------------------------------

## Stage 2 --- Calculation Validation

Purpose:

Ensure payroll calculations are mathematically and logically correct.

Checks:

-   earnings calculation completed
-   deductions calculated
-   net pay generated
-   calculation snapshot created
-   rule versions recorded

------------------------------------------------------------------------

## Stage 3 --- Compliance Validation

Purpose:

Ensure statutory processing requirements are satisfied.

Checks:

-   applicable compliance rules exist
-   rule version is active
-   jurisdiction mapping exists
-   effective dates are valid

------------------------------------------------------------------------

## Stage 4 --- Financial Validation

Purpose:

Protect financial correctness.

Checks:

-   payroll totals reconcile
-   duplicate payment risks detected
-   deduction totals balanced
-   rounding rules applied

------------------------------------------------------------------------

## Stage 5 --- Workflow Validation

Purpose:

Ensure payroll lifecycle correctness.

Checks:

-   correct state transition
-   required approval completed
-   authorized actor performed action

------------------------------------------------------------------------

# 15.3 Validation Severity Model

  Severity      Meaning                 Payroll Impact
  ------------- ----------------------- -------------------------
  BLOCKER       Critical issue          Processing stopped
  ERROR         Correction required     Cannot finalize
  WARNING       Review required         Processing may continue
  INFORMATION   Informational message   No restriction

------------------------------------------------------------------------

# 15.4 Validation Rule Matrix

  ----------------------------------------------------------------------------------------
  Rule ID       Validation     Owner             Stage         Severity    Recovery
                Rule                                                       
  ------------- -------------- ----------------- ------------- ----------- ---------------
  VAL-PAY-001   Employee       Workforce         Input         BLOCKER     Correct
                reference must                                             employee data
                exist                                                      

  VAL-PAY-002   Attendance     Attendance        Input         BLOCKER     Complete
                must be                                                    attendance
                finalized                                                  finalization

  VAL-PAY-003   Compensation   Compensation      Input         BLOCKER     Activate
                version must                                               compensation
                exist                                                      rule

  VAL-PAY-004   Calculation    Payroll           Calculation   ERROR       Recalculate
                snapshot                                                   payroll
                required                                                   

  VAL-PAY-005   Net pay must   Payroll           Financial     BLOCKER     Correct
                reconcile                                                  calculation

  VAL-PAY-006   Compliance     Compliance        Compliance    BLOCKER     Update rule
                rule must be                                               configuration
                active                                                     

  VAL-PAY-007   Approval must  Payroll           Workflow      BLOCKER     Complete
                exist before                                               approval
                finalization                                               

  VAL-PAY-008   Duplicate      Payroll/Finance   Financial     ERROR       Review
                payment                                                    duplicate risk
                detection                                                  
  ----------------------------------------------------------------------------------------

------------------------------------------------------------------------

# 15.5 Validation Failure Handling

``` mermaid
flowchart LR

Failure["Validation Failure"]

Classify["Classify Severity"]

Block["Block Processing"]

Review["Human Review"]

Resolve["Correction"]

Revalidate["Revalidation"]

Continue["Continue Payroll"]


Failure --> Classify

Classify --> Block

Classify --> Review

Block --> Resolve

Review --> Resolve

Resolve --> Revalidate

Revalidate --> Continue
```

------------------------------------------------------------------------

# 15.6 Validation Ownership Principles

  Validation Area                   Owner
  --------------------------------- ---------------------
  Employee data correctness         Workforce Domain
  Attendance correctness            Attendance Domain
  Leave correctness                 Leave Domain
  Compensation correctness          Compensation Domain
  Payroll calculation correctness   Payroll Domain
  Compliance correctness            Compliance Domain
  Payment correctness               Finance Domain

Payroll validates applicability but does not replace source ownership.

------------------------------------------------------------------------

# 16. Authorization Model

Payroll authorization protects sensitive financial operations.

Authorization follows:

``` text
Identity

↓

Role

↓

Permission

↓

Organization Scope

↓

Payroll Context

↓

Allowed Action
```

------------------------------------------------------------------------

# 16.1 Authorization Architecture

``` mermaid
flowchart LR

Identity["User Identity"]

Role["Assigned Role"]

Permission["Permission"]

Scope["Organization Scope"]

Context["Payroll State Context"]

Decision["Authorization Decision"]


Identity --> Role

Role --> Permission

Permission --> Scope

Scope --> Context

Context --> Decision
```

------------------------------------------------------------------------

# 16.2 Authorization Layers

## Layer 1 --- Authentication

Determines:

-   who is requesting access
-   identity validity
-   session security

------------------------------------------------------------------------

## Layer 2 --- Role Based Access Control (RBAC)

Determines:

-   available capabilities
-   allowed operations

Example roles:

-   Payroll Administrator
-   Payroll Operator
-   Payroll Approver
-   Finance Reviewer
-   Employee

------------------------------------------------------------------------

## Layer 3 --- Attribute Based Access Control (ABAC)

Determines:

-   organization scope
-   institution access
-   department access
-   payroll group access

Example:

A Payroll Officer may process only:

``` text
Institution A Payroll
```

not:

``` text
All Institutions Payroll
```

------------------------------------------------------------------------

## Layer 4 --- State Based Authorization

Actions depend on payroll state.

Example:

A user cannot approve:

``` text
Created Payroll
```

Approval is allowed only when:

``` text
Pending Approval Payroll
```

------------------------------------------------------------------------

# 16.3 Role Permission Matrix

  ------------------------------------------------------------------------------------
  Role            Create Run Calculate   Validate   Approve    Finalize     View
                                                                            Payslip
  --------------- ---------- ----------- ---------- ---------- ------------ ----------
  Payroll         Yes        Yes         Yes        No         No           Yes
  Administrator                                                             

  Payroll         No         Yes         Yes        No         No           Yes
  Operator                                                                  

  Payroll         No         No          No         Yes        Controlled   Yes
  Approver                                                                  

  Finance         No         No          No         Review     No           Summary
  Reviewer                                                                  

  Employee        No         No          No         No         No           Own
                                                                            Payslip
  ------------------------------------------------------------------------------------

------------------------------------------------------------------------

# 16.4 Separation of Duties

Payroll enforces:

## Rule 1 --- Creator and Final Approver Separation

The person creating payroll should not be the sole final approver.

------------------------------------------------------------------------

## Rule 2 --- Calculator and Approver Separation

The person executing calculation should not independently approve final
payroll.

------------------------------------------------------------------------

## Rule 3 --- Payroll and Finance Separation

Payroll creates financial results.

Finance executes payment.

------------------------------------------------------------------------

# 16.5 Authorization Decision Flow

``` mermaid
flowchart LR

Request["Action Request"]

Identity["Identity Verification"]

Role["Role Check"]

Scope["Scope Check"]

State["Payroll State Check"]

Allow["Allow Action"]

Deny["Deny Action"]


Request --> Identity

Identity --> Role

Role --> Scope

Scope --> State

State --> Allow

State --> Deny
```

------------------------------------------------------------------------

# 16.6 Security Authorization Controls

Mandatory controls:

-   least privilege access
-   permission auditing
-   privilege escalation prevention
-   organization isolation
-   sensitive data protection
-   approval traceability

------------------------------------------------------------------------

# Final Validation and Authorization Statement

Payroll validation ensures financial correctness before execution
continues.

Payroll authorization ensures only qualified and authorized actors can
perform sensitive financial operations.

Together they protect:

-   payroll accuracy
-   financial integrity
-   compliance
-   auditability
-   enterprise security

# 17. Audit Requirements

Payroll is a sensitive financial domain where every significant action
must be traceable.

The audit principle:

``` text
Every financial decision must have an accountable actor,
timestamp, reason, and historical record.
```

Auditability is required for:

-   financial review
-   compliance verification
-   dispute resolution
-   operational investigation
-   regulatory requirements

------------------------------------------------------------------------

# 17.1 Audit Architecture

``` mermaid
flowchart LR

Action["Payroll Action"]

Actor["Actor Identity"]

Context["Business Context"]

Audit["Audit Record"]

Storage["Audit Repository"]


Action --> Audit

Actor --> Audit

Context --> Audit

Audit --> Storage
```

------------------------------------------------------------------------

# 17.2 Audit Scope

The Payroll domain must audit:

## Payroll Lifecycle Actions

Including:

-   payroll creation
-   calculation start
-   calculation completion
-   validation completion
-   approval
-   rejection
-   finalization
-   closure

------------------------------------------------------------------------

## Configuration Actions

Including:

-   rule creation
-   rule modification
-   rule activation
-   rule retirement

------------------------------------------------------------------------

## Adjustment Actions

Including:

-   adjustment creation
-   approval
-   calculation
-   application
-   rejection

------------------------------------------------------------------------

## Security Actions

Including:

-   login attempts
-   permission changes
-   privileged operations
-   unauthorized access attempts

------------------------------------------------------------------------

# 17.3 Audit Record Structure

Every audit record should contain:

  Field                Purpose
  -------------------- --------------------------------
  Audit ID             Unique audit identifier
  Event Type           Action performed
  Entity Reference     Affected payroll object
  Actor ID             User/service performing action
  Timestamp            When action occurred
  Previous State       Before value/state
  New State            After value/state
  Reason               Business justification
  Organization Scope   Tenant/institution context
  Correlation ID       Trace across systems

------------------------------------------------------------------------

# 17.4 Immutable Audit Principle

Audit records must not be modified after creation.

``` mermaid
flowchart LR

Action["Payroll Action"]

Create["Create Audit Record"]

Immutable["Immutable Audit History"]

Review["Audit Review"]


Action --> Create

Create --> Immutable

Immutable --> Review
```

------------------------------------------------------------------------

# 17.5 Payroll Audit Events

  Event                   Audit Requirement
  ----------------------- ---------------------------------------
  Payroll Created         Creator and configuration captured
  Calculation Completed   Engine version and timestamp captured
  Approval Completed      Approver and decision captured
  Payroll Finalized       Finalization authority captured
  Adjustment Applied      Reason and approval captured
  Configuration Changed   Old and new versions captured

------------------------------------------------------------------------

# 17.6 Audit Retention

Payroll audit records should support:

-   long-term retention
-   regulatory review
-   historical reconstruction
-   financial investigation

Retention policy must follow institutional and regulatory requirements.

------------------------------------------------------------------------

# 18. Security Considerations

Payroll contains highly sensitive financial information.

Security objectives:

``` text
Confidentiality

+

Integrity

+

Availability

+

Traceability
```

------------------------------------------------------------------------

# 18.1 Data Classification

Payroll data categories:

  Data Type               Classification
  ----------------------- -------------------
  Salary Information      Highly Sensitive
  Tax Information         Highly Sensitive
  Deduction Information   Sensitive
  Payslip Documents       Sensitive
  Payroll Reports         Controlled Access

------------------------------------------------------------------------

# 18.2 Data Protection Controls

Mandatory controls:

## Encryption

Sensitive payroll information must be protected:

-   during transmission
-   during storage
-   during backup

------------------------------------------------------------------------

## Access Control

Access must follow:

-   least privilege
-   role boundaries
-   organization scope
-   payroll state restrictions

------------------------------------------------------------------------

## Data Masking

Sensitive values should be masked where full visibility is unnecessary.

Examples:

-   payroll dashboards
-   analytics views
-   operational reports

------------------------------------------------------------------------

# 18.3 Security Architecture

``` mermaid
flowchart LR

User["User"]

Auth["Authentication"]

Authorization["Authorization"]

Payroll["Payroll Services"]

Data["Sensitive Payroll Data"]

Audit["Audit Logging"]


User --> Auth

Auth --> Authorization

Authorization --> Payroll

Payroll --> Data

Payroll --> Audit
```

------------------------------------------------------------------------

# 18.4 Tenant Isolation

In multi-institution deployments:

Each institution must have isolated:

-   payroll data
-   payroll configuration
-   user access scope
-   audit history

Cross-tenant payroll visibility is prohibited unless explicitly
authorized.

------------------------------------------------------------------------

# 18.5 Privileged Access Controls

Privileged payroll operations require:

-   elevated permission
-   additional audit logging
-   approval where applicable

Examples:

-   reopening payroll
-   modifying configuration
-   accessing sensitive payroll reports

------------------------------------------------------------------------

# 18.6 Security Monitoring

Monitor:

-   failed authorization attempts
-   unusual payroll access
-   bulk data extraction
-   configuration changes
-   privileged operations

------------------------------------------------------------------------

# 18.7 Document Security

Payslips and payroll documents require:

-   controlled access
-   secure storage
-   authorization validation
-   download tracking

------------------------------------------------------------------------

# 18.8 Security Failure Handling

Security-related failures must:

-   deny unauthorized access
-   record audit event
-   notify security monitoring systems where required

------------------------------------------------------------------------

# Final Audit and Security Statement

The Payroll domain maintains enterprise trust through:

-   immutable audit history
-   controlled financial operations
-   strict authorization
-   sensitive data protection
-   tenant isolation
-   security monitoring

Payroll security ensures that financial records remain confidential,
accurate, and accountable throughout their lifecycle.

# 21. Architecture Visual Models

Architecture Visual Models provide enterprise-level views of the Payroll
domain.

The purpose of AVM is different from workflow diagrams.

Workflow diagrams explain execution.

AVMs explain:

-   system boundaries
-   domain relationships
-   runtime architecture
-   integration patterns
-   scalability model
-   security model

------------------------------------------------------------------------

# AVM-01 --- Payroll Domain Context Model

``` mermaid
flowchart LR

Workforce["Workforce Domain"]

Attendance["Attendance Domain"]

Leave["Leave Management Domain"]

Compensation["Compensation Domain"]

Compliance["Compliance Domain"]


Payroll["Payroll Domain"]


Finance["Finance Domain"]

Portal["Employee Portal"]

Analytics["Analytics Platform"]

Notification["Notification Domain"]

Documents["Document Platform"]


Workforce --> Payroll

Attendance --> Payroll

Leave --> Payroll

Compensation --> Payroll

Compliance --> Payroll

Payroll --> Finance

Payroll --> Portal

Payroll --> Analytics

Payroll --> Notification

Payroll --> Documents
```

Purpose:

Defines Payroll as an independent financial execution domain.

------------------------------------------------------------------------

# AVM-02 --- Payroll Bounded Context Model

``` mermaid
flowchart TD

Payroll["Payroll Bounded Context"]

Run["Payroll Run Management"]

Engine["Calculation Engine"]

Validation["Validation Engine"]

Approval["Approval Workflow"]

Adjustment["Adjustment Engine"]

Payslip["Payslip Service"]

Audit["Audit Service"]


Payroll --> Run

Payroll --> Engine

Payroll --> Validation

Payroll --> Approval

Payroll --> Adjustment

Payroll --> Payslip

Payroll --> Audit
```

------------------------------------------------------------------------

# AVM-03 --- Aggregate Architecture Model

``` mermaid
flowchart LR

Configuration["Payroll Configuration Aggregate"]

Run["Payroll Run Aggregate"]

Result["Payroll Result Aggregate"]

Adjustment["Payroll Adjustment Aggregate"]


Configuration --> Run

Run --> Result

Result --> Adjustment
```

------------------------------------------------------------------------

# AVM-04 --- Payroll Processing Pipeline

``` mermaid
flowchart LR

Input["Governed Business Facts"]

Queue["Processing Queue"]

Workers["Calculation Workers"]

Validation["Validation Engine"]

Approval["Approval Workflow"]

Finalize["Finalization"]

Finance["Finance Integration"]

Payslip["Payslip Generation"]


Input --> Queue

Queue --> Workers

Workers --> Validation

Validation --> Approval

Approval --> Finalize

Finalize --> Finance

Finalize --> Payslip
```

------------------------------------------------------------------------

# AVM-05 --- Event Driven Architecture

``` mermaid
flowchart LR

Payroll["Payroll Domain"]

Events["Payroll Events"]

Finance["Finance"]

Portal["Employee Portal"]

Analytics["Analytics"]

Notification["Notification"]

Audit["Audit Platform"]


Payroll --> Events

Events --> Finance

Events --> Portal

Events --> Analytics

Events --> Notification

Events --> Audit
```

------------------------------------------------------------------------

# AVM-06 --- Multi-Tenant Payroll Architecture

``` mermaid
flowchart TD

Platform["SutraERP Payroll Platform"]

Engine["Shared Payroll Engine"]

TenantA["Institution A Configuration"]

TenantB["Institution B Configuration"]

TenantC["Institution C Configuration"]


Platform --> Engine

Engine --> TenantA

Engine --> TenantB

Engine --> TenantC
```

Isolation requirements:

-   tenant data isolation
-   configuration isolation
-   authorization isolation
-   audit isolation

------------------------------------------------------------------------

# AVM-07 --- Security Architecture Model

``` mermaid
flowchart LR

User["User"]

Identity["Identity Provider"]

Authorization["Authorization Layer"]

Payroll["Payroll Services"]

Data["Sensitive Payroll Data"]

Audit["Audit Logging"]


User --> Identity

Identity --> Authorization

Authorization --> Payroll

Payroll --> Data

Payroll --> Audit
```

------------------------------------------------------------------------

# AVM-08 --- Failure Recovery Architecture

``` mermaid
flowchart LR

Failure["Failure Detection"]

Classification["Failure Classification"]

Retry["Automatic Retry"]

Escalation["Human Escalation"]

Recovery["Recovery"]

Audit["Audit Record"]


Failure --> Classification

Classification --> Retry

Classification --> Escalation

Retry --> Recovery

Escalation --> Recovery

Recovery --> Audit
```

------------------------------------------------------------------------

# AVM-09 --- Payroll Lifecycle Architecture

``` mermaid
stateDiagram-v2

[*] --> Created

Created --> Calculating

Calculating --> Calculated

Calculated --> Validating

Validating --> PendingApproval

PendingApproval --> Approved

Approved --> Finalized

Finalized --> Posted

Posted --> Closed
```

------------------------------------------------------------------------

# AVM-10 --- Runtime Deployment Model

``` mermaid
flowchart LR

Client["ERP Clients"]

API["Payroll API Layer"]

Workflow["Workflow Engine"]

Queue["Message Queue"]

Workers["Calculation Workers"]

Database["Payroll Database"]

Events["Event Bus"]

Documents["Document Service"]


Client --> API

API --> Workflow

Workflow --> Queue

Queue --> Workers

Workers --> Database

Database --> Events

Events --> Documents
```

------------------------------------------------------------------------

# AVM Governance Principles

All Payroll architecture visual models follow:

1.  Domain ownership boundaries remain explicit.
2.  Source domains own facts.
3.  Payroll owns financial execution.
4.  Finance owns settlement and accounting.
5.  Finalized payroll remains immutable.
6.  Events communicate business outcomes.
7.  Security boundaries apply across all layers.

------------------------------------------------------------------------

# Final AVM Statement

The Payroll architecture uses bounded domains, event-driven integration,
scalable processing, and controlled security boundaries to support
enterprise payroll execution.


# 22. Architecture Decision Records

Architecture Decision Records document the major technical and business
architecture decisions that define the Payroll domain.

The purpose of ADRs is to preserve:

-   architectural intent
-   decision rationale
-   rejected alternatives
-   long-term maintainability

------------------------------------------------------------------------

# ADR-E06-001 --- Payroll as Independent Financial Domain

## Status

Accepted

------------------------------------------------------------------------

## Context

Payroll interacts with workforce, attendance, leave, compensation,
compliance, and finance domains.

However, payroll has unique responsibilities:

-   financial calculation
-   payroll lifecycle management
-   financial record generation
-   payroll audit history

------------------------------------------------------------------------

## Decision

Payroll will exist as an independent bounded context.

Payroll owns:

-   payroll execution
-   payroll calculation
-   payroll results
-   payroll adjustments
-   payroll history

------------------------------------------------------------------------

## Alternatives Considered

### Alternative 1 --- Payroll inside Workforce Domain

Rejected.

Reason:

Workforce owns employment facts, not financial execution.

------------------------------------------------------------------------

### Alternative 2 --- Payroll inside Finance Domain

Rejected.

Reason:

Finance owns accounting and settlement, not payroll calculation
lifecycle.

------------------------------------------------------------------------

## Consequences

Positive:

-   clear ownership
-   better scalability
-   independent evolution

Negative:

-   requires integration contracts

------------------------------------------------------------------------

# ADR-E06-002 --- Governed Facts Input Model

## Status

Accepted

------------------------------------------------------------------------

## Context

Payroll depends on information owned by multiple domains.

Duplicating those facts creates inconsistency.

------------------------------------------------------------------------

## Decision

Payroll consumes authoritative facts from source domains.

Ownership:

``` text
Workforce → Employee Facts

Attendance → Attendance Facts

Leave → Leave Facts

Compensation → Salary Policy

Payroll → Financial Outcome
```

------------------------------------------------------------------------

## Alternatives Considered

### Duplicate Source Data Inside Payroll

Rejected.

Reason:

Creates multiple sources of truth.

------------------------------------------------------------------------

## Consequences

Positive:

-   consistent enterprise data ownership
-   reduced duplication

Negative:

-   requires reliable integrations

------------------------------------------------------------------------

# ADR-E06-003 --- Versioned Payroll Calculation Engine

## Status

Accepted

------------------------------------------------------------------------

## Context

Payroll rules change due to:

-   policy changes
-   compliance changes
-   institutional requirements

Historical payroll must remain reproducible.

------------------------------------------------------------------------

## Decision

All payroll calculations execute against versioned rules.

Every result stores:

-   rule version
-   configuration version
-   calculation timestamp

------------------------------------------------------------------------

## Alternatives Considered

### Hardcoded Calculation Logic

Rejected.

Reason:

Cannot support frequent regulatory changes.

------------------------------------------------------------------------

### Unversioned Configuration

Rejected.

Reason:

Historical reconstruction becomes impossible.

------------------------------------------------------------------------

## Consequences

Positive:

-   reproducible payroll
-   safer rule evolution

Negative:

-   increased configuration governance

------------------------------------------------------------------------

# ADR-E06-004 --- Immutable Payroll Finalization

## Status

Accepted

------------------------------------------------------------------------

## Context

Payroll records represent financial truth after approval.

Direct modification creates audit and compliance risks.

------------------------------------------------------------------------

## Decision

Finalized payroll is immutable.

Corrections must use adjustment workflows.

------------------------------------------------------------------------

## Alternatives Considered

### Direct Editing After Finalization

Rejected.

Reason:

Breaks audit integrity.

------------------------------------------------------------------------

## Consequences

Positive:

-   financial trust
-   clear history

Negative:

-   requires adjustment processes

------------------------------------------------------------------------

# ADR-E06-005 --- Adjustment-Based Correction Model

## Status

Accepted

------------------------------------------------------------------------

## Context

Errors may be discovered after payroll completion.

------------------------------------------------------------------------

## Decision

All corrections use controlled adjustment workflows.

Examples:

-   retro pay
-   arrears
-   reversals
-   missing earnings

------------------------------------------------------------------------

## Alternatives Considered

### Reopen Existing Payroll

Rejected.

Reason:

Destroys historical integrity.

------------------------------------------------------------------------

## Consequences

Positive:

-   traceable corrections
-   complete audit history

------------------------------------------------------------------------

# ADR-E06-006 --- Payroll and Finance Boundary

## Status

Accepted

------------------------------------------------------------------------

## Context

Payroll calculates financial outcomes.

Finance manages financial settlement.

------------------------------------------------------------------------

## Decision

Boundary:

``` text
Payroll Calculates Money.

Finance Accounts and Pays Money.
```

------------------------------------------------------------------------

## Alternatives Considered

### Payroll Executes Payments

Rejected.

Reason:

Violates financial separation of duties.

------------------------------------------------------------------------

## Consequences

Positive:

-   cleaner accounting model
-   better controls

------------------------------------------------------------------------

# ADR-E06-007 --- Shared Multi-Tenant Payroll Engine

## Status

Accepted

------------------------------------------------------------------------

## Context

SutraERP supports multiple institutions.

A separate payroll implementation per tenant creates duplication.

------------------------------------------------------------------------

## Decision

Use a shared payroll engine with tenant-specific configuration.

Tenant isolation applies to:

-   data
-   configuration
-   permissions
-   audit history

------------------------------------------------------------------------

## Alternatives Considered

### Separate Payroll Codebase Per Tenant

Rejected.

Reason:

High maintenance cost and inconsistent evolution.

------------------------------------------------------------------------

## Consequences

Positive:

-   scalable platform
-   consistent capabilities

Negative:

-   requires strong isolation architecture

------------------------------------------------------------------------

# ADR-E06-008 --- Event Driven Integration Model

## Status

Accepted

------------------------------------------------------------------------

## Context

Payroll outcomes are consumed by multiple enterprise systems.

------------------------------------------------------------------------

## Decision

Payroll publishes domain events.

Examples:

-   PayrollFinalized
-   PayslipGenerated
-   PayrollClosed
-   PayrollAdjustmentCreated

------------------------------------------------------------------------

## Alternatives Considered

### Shared Database Integration

Rejected.

Reason:

Creates tight coupling and ownership violations.

------------------------------------------------------------------------

## Consequences

Positive:

-   loose coupling
-   scalable integrations

Negative:

-   requires event governance

------------------------------------------------------------------------

# ADR-E06-009 --- Batch Processing and Scalability Architecture

## Status

Accepted

------------------------------------------------------------------------

## Context

Payroll may process very large employee populations.

A synchronous single-process model does not scale.

------------------------------------------------------------------------

## Decision

Payroll processing uses:

-   queues
-   worker execution
-   partitioned processing
-   asynchronous workloads

------------------------------------------------------------------------

## Processing Model

``` text
Payroll Run

↓

Processing Queue

↓

Calculation Workers

↓

Validation

↓

Results
```

------------------------------------------------------------------------

## Alternatives Considered

### Single Synchronous Payroll Execution

Rejected.

Reason:

Poor scalability and failure isolation.

------------------------------------------------------------------------

## Consequences

Positive:

-   horizontal scalability
-   fault isolation

Negative:

-   operational complexity

------------------------------------------------------------------------

# ADR-E06-010 --- Security and Audit First Design

## Status

Accepted

------------------------------------------------------------------------

## Context

Payroll contains highly sensitive financial information.

------------------------------------------------------------------------

## Decision

Security and audit are mandatory architecture capabilities.

Required:

-   immutable audit trail
-   least privilege access
-   tenant isolation
-   privileged operation tracking
-   sensitive data protection

------------------------------------------------------------------------

## Alternatives Considered

### Generic Application Security Only

Rejected.

Reason:

Payroll requires financial-grade controls.

------------------------------------------------------------------------

## Consequences

Positive:

-   regulatory readiness
-   stronger trust model

Negative:

-   additional implementation requirements

------------------------------------------------------------------------

# ADR Summary Matrix

  ADR           Decision
  ------------- ----------------------------------------
  ADR-E06-001   Payroll is independent bounded context
  ADR-E06-002   Payroll consumes governed facts
  ADR-E06-003   Calculation engine is versioned
  ADR-E06-004   Finalized payroll is immutable
  ADR-E06-005   Corrections use adjustments
  ADR-E06-006   Payroll and Finance are separated
  ADR-E06-007   Shared multi-tenant engine
  ADR-E06-008   Event-driven integration
  ADR-E06-009   Scalable batch processing
  ADR-E06-010   Security and audit first design

------------------------------------------------------------------------

# Final ADR Statement

The Payroll architecture decisions establish a secure, scalable,
auditable, and enterprise-ready financial processing capability.

These decisions preserve:

-   domain ownership
-   financial integrity
-   historical accuracy
-   scalability
-   future extensibility


# 23. Glossary

This glossary defines canonical terminology used within the Payroll
domain.

The objective is to prevent ambiguity across:

-   Product teams
-   Engineering teams
-   Finance teams
-   HR teams
-   Compliance teams
-   Operations teams

------------------------------------------------------------------------

# 23.1 Payroll Core Terms

## Payroll Run

A controlled execution instance used to process payroll for a defined:

-   payroll period
-   organization scope
-   payroll type

A Payroll Run owns the execution lifecycle.

------------------------------------------------------------------------

## Payroll Period

A defined time range for which payroll calculations are performed.

Examples:

-   monthly payroll cycle
-   special payroll cycle
-   final settlement cycle

------------------------------------------------------------------------

## Payroll Result

The employee-level financial outcome produced after payroll calculation.

Contains:

-   earnings
-   deductions
-   net pay
-   calculation references
-   applied rule versions

------------------------------------------------------------------------

## Payroll Finalization

The lifecycle stage where payroll becomes an immutable financial record.

After finalization:

-   direct editing is prohibited
-   corrections require adjustments
-   audit history is preserved

------------------------------------------------------------------------

## Payroll Adjustment

A controlled correction mechanism used after payroll processing.

Examples:

-   retro pay
-   arrears
-   missed earnings
-   incorrect deductions
-   reversals

------------------------------------------------------------------------

## Retro Pay

A payroll correction generated due to a historical change that affects
previous payroll periods.

Example:

A salary revision applied after previous payroll processing.

------------------------------------------------------------------------

## Arrears

Accumulated unpaid financial differences from previous payroll periods.

------------------------------------------------------------------------

## Off-Cycle Payroll

A payroll execution performed outside the standard payroll schedule.

Examples:

-   emergency payment
-   special incentive
-   approved correction

------------------------------------------------------------------------

# 23.2 Domain Architecture Terms

## Aggregate

A Domain Driven Design consistency boundary containing related entities
and enforcing business rules.

------------------------------------------------------------------------

## Aggregate Root

The primary entity responsible for controlling changes within an
aggregate.

------------------------------------------------------------------------

## Domain Event

A business fact published after a meaningful domain state change.

Examples:

-   PayrollFinalized
-   PayslipGenerated

------------------------------------------------------------------------

## Bounded Context

A clearly defined domain boundary where a specific model and ownership
responsibility applies.

------------------------------------------------------------------------

## Source of Truth

The authoritative owner of a specific business fact.

Examples:

-   Workforce owns employee facts.
-   Attendance owns attendance facts.
-   Payroll owns payroll outcomes.

------------------------------------------------------------------------

# 23.3 Financial Terms

## Gross Earnings

Total employee earnings before deductions.

------------------------------------------------------------------------

## Deduction

An amount removed from gross earnings according to applicable rules.

------------------------------------------------------------------------

## Net Pay

The final payable amount after applying deductions.

Formula:

``` text
Net Pay = Gross Earnings - Total Deductions
```

------------------------------------------------------------------------

## Statutory Deduction

A legally required payroll deduction.

Examples:

-   taxes
-   mandated contributions

------------------------------------------------------------------------

## Employer Contribution

A financial contribution made by the employer separately from employee
deductions.

------------------------------------------------------------------------

# 23.4 Governance Terms

## Payroll Snapshot

A preserved representation of payroll inputs and calculation context
used for reproducibility.

Includes references to:

-   employee state
-   attendance state
-   leave state
-   compensation version
-   rule versions

------------------------------------------------------------------------

## Audit Trail

An immutable historical record of actions, decisions, and changes.

------------------------------------------------------------------------

## Separation of Duties

A security and governance principle where conflicting responsibilities
are assigned to different actors.

Example:

``` text
Payroll Creator ≠ Final Approver
```

------------------------------------------------------------------------

## Immutable Record

A record that cannot be directly modified after reaching a controlled
final state.

------------------------------------------------------------------------

# 24. Domain Summary

------------------------------------------------------------------------

# 24.1 Payroll Domain Purpose

The Payroll domain transforms governed enterprise business facts into
validated financial outcomes.

The domain ensures:

-   accurate payroll calculation
-   controlled execution
-   financial integrity
-   compliance readiness
-   auditability

------------------------------------------------------------------------

# 24.2 Payroll Domain Responsibilities

Payroll owns:

  Capability               Ownership
  ------------------------ -----------
  Payroll Run Management   Payroll
  Payroll Calculation      Payroll
  Payroll Results          Payroll
  Payroll Adjustments      Payroll
  Payslip Generation       Payroll
  Payroll Audit History    Payroll

------------------------------------------------------------------------

# 24.3 Payroll Domain Non-Responsibilities

Payroll does not own:

  Capability             Owner
  ---------------------- -------------------------
  Employee Master Data   Workforce Domain
  Attendance Facts       Attendance Domain
  Leave Facts            Leave Management Domain
  Compensation Policy    Compensation Domain
  Payment Execution      Finance Domain
  Accounting Ledger      Finance Domain

------------------------------------------------------------------------

# 24.4 Enterprise Data Flow Summary

``` mermaid
flowchart LR

Workforce["Workforce Facts"]

Attendance["Attendance Facts"]

Leave["Leave Facts"]

Compensation["Compensation Rules"]

Payroll["Payroll Execution"]

Finance["Finance Settlement"]

Analytics["Analytics"]

Employee["Employee Portal"]


Workforce --> Payroll

Attendance --> Payroll

Leave --> Payroll

Compensation --> Payroll

Payroll --> Finance

Payroll --> Analytics

Payroll --> Employee
```

------------------------------------------------------------------------

# 24.5 Capability Summary

  Capability               Authoritative Owner
  ------------------------ ---------------------
  Employee Identity        Workforce
  Attendance Information   Attendance
  Leave Information        Leave Management
  Salary Policy            Compensation
  Payroll Calculation      Payroll
  Payroll Results          Payroll
  Payment Processing       Finance
  Accounting               Finance
  Reporting Insights       Analytics

------------------------------------------------------------------------

# 24.6 Final Domain Statement

The Payroll domain provides SutraERP with an enterprise-grade payroll
processing capability.

It maintains strict boundaries:

``` text
Source Domains Own Facts.

Payroll Owns Financial Transformation.

Finance Owns Settlement.

Consumers Receive Payroll Outcomes.
```

The architecture enables:

-   scalability
-   security
-   auditability
-   financial correctness
-   future extensibility

while preserving clear ownership across the enterprise platform.


# 25. Document Completion & Governance

This section defines the governance model for maintaining the Payroll
architecture document throughout its lifecycle.

The objective is to ensure:

-   architectural consistency
-   controlled evolution
-   ownership clarity
-   continuous alignment with SutraERP architecture principles

------------------------------------------------------------------------

# 25.1 Document Status

  Attribute            Value
  -------------------- --------------------------------------
  Document             L2-E06 --- Payroll Workflows
  Architecture Level   Level 2 Domain Workflow Architecture
  Domain               Enterprise Payroll
  Status               Draft
  Lifecycle            Controlled Evolution
  Ownership            Payroll Domain Architecture Owner

------------------------------------------------------------------------

# 25.2 Document Ownership

## Primary Owner

Payroll Domain Architecture Owner

Responsibilities:

-   maintain domain correctness
-   approve architectural changes
-   ensure workflow alignment

------------------------------------------------------------------------

## Supporting Owners

  Area                       Owner
  -------------------------- -------------------------
  Workforce Integration      Workforce Domain
  Attendance Integration     Attendance Domain
  Leave Integration          Leave Management Domain
  Compensation Integration   Compensation Domain
  Financial Integration      Finance Domain
  Security Controls          Platform Security
  Analytics Integration      Analytics Platform

------------------------------------------------------------------------

# 25.3 Change Governance

All changes must follow controlled architecture review.

Change categories:

  Change Type          Review Requirement
  -------------------- --------------------------------
  Workflow change      Domain review
  Aggregate change     Architecture review
  Ownership change     Enterprise architecture review
  Integration change   Contract review
  Security change      Security review

------------------------------------------------------------------------

# 25.4 Version Management

Every document revision must maintain:

-   version number
-   change summary
-   author
-   approval record
-   review date

------------------------------------------------------------------------

# 25.5 Architecture Compliance Review

Periodic review should validate:

## Domain Boundaries

-   Payroll ownership remains clear
-   No upstream ownership duplication
-   No Finance responsibility leakage

------------------------------------------------------------------------

## Workflow Integrity

-   Lifecycle states remain valid
-   Approval controls remain active
-   Finalization remains immutable

------------------------------------------------------------------------

## Security Integrity

-   Authorization remains enforced
-   Audit remains complete
-   Tenant isolation remains protected

------------------------------------------------------------------------

# 25.6 Completion Checklist

  Area                           Status
  ------------------------------ ----------
  Purpose Defined                Complete
  Scope Defined                  Complete
  Dependencies Defined           Complete
  Ownership Boundaries Defined   Complete
  Domain Model Defined           Complete
  Aggregates Defined             Complete
  Workflows Defined              Complete
  Business Rules Defined         Complete
  Events Defined                 Complete
  Service Contracts Defined      Complete
  State Models Defined           Complete
  Decision Matrix Defined        Complete
  Failure Matrix Defined         Complete
  Validation Rules Defined       Complete
  Authorization Defined          Complete
  Audit Defined                  Complete
  Security Defined               Complete
  Scalability Defined            Complete
  Extension Points Defined       Complete
  AVM Defined                    Complete
  ADR Defined                    Complete
  Glossary Defined               Complete

------------------------------------------------------------------------

# 26. Appendices

------------------------------------------------------------------------

# Appendix A --- Payroll Lifecycle Summary

``` mermaid
stateDiagram-v2

[*] --> Created

Created --> Calculating

Calculating --> Calculated

Calculated --> Validating

Validating --> PendingApproval

PendingApproval --> Approved

Approved --> Finalized

Finalized --> Posted

Posted --> Closed
```

------------------------------------------------------------------------

# Appendix B --- Core Ownership Summary

``` text
Workforce
    Owns employee facts

Attendance
    Owns attendance facts

Leave Management
    Owns leave facts

Compensation
    Owns salary policy

Payroll
    Owns payroll execution and outcomes

Finance
    Owns settlement and accounting
```

------------------------------------------------------------------------

# Appendix C --- Core Architecture Principles

## Principle 1

Source domains own business facts.

------------------------------------------------------------------------

## Principle 2

Payroll owns financial transformation.

------------------------------------------------------------------------

## Principle 3

Finance owns payment and accounting.

------------------------------------------------------------------------

## Principle 4

Finalized payroll is immutable.

------------------------------------------------------------------------

## Principle 5

Corrections happen through controlled adjustments.

------------------------------------------------------------------------

## Principle 6

All financial actions are auditable.

------------------------------------------------------------------------

# Appendix D --- Key Domain Events

  Event                         Purpose
  ----------------------------- -----------------------------
  PayrollRunCreated             Payroll execution initiated
  PayrollCalculationCompleted   Calculation completed
  PayrollValidationCompleted    Validation completed
  PayrollApproved               Authorization completed
  PayrollFinalized              Financial record locked
  PayslipGenerated              Employee document available
  PayrollAdjustmentCreated      Correction initiated
  PayrollClosed                 Payroll lifecycle completed

------------------------------------------------------------------------

# Appendix E --- Final Architecture Statement

The L2-E06 Payroll Workflow Architecture defines an enterprise-grade
payroll capability for SutraERP.

The architecture provides:

-   controlled payroll execution
-   scalable processing
-   financial integrity
-   security
-   auditability
-   clear domain ownership
-   future extensibility

The final architectural boundary remains:

``` text
Enterprise Facts

        ↓

Payroll Financial Transformation

        ↓

Finance Settlement + Enterprise Consumers
```

This document is considered complete when all implementation designs,
APIs, workflows, and extensions continue to comply with the decisions
defined in this architecture.


